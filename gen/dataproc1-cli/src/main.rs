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
extern crate google_dataproc1 as api;

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
    hub: api::Dataproc<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n> Engine<'n> {
    fn _projects_regions_clusters_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "status.state" => Some(("status.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.state-start-time" => Some(("status.stateStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.substate" => Some(("status.substate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.detail" => Some(("status.detail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster-uuid" => Some(("clusterUuid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster-name" => Some(("clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metrics.yarn-metrics" => Some(("metrics.yarnMetrics", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metrics.hdfs-metrics" => Some(("metrics.hdfsMetrics", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "config.software-config.image-version" => Some(("config.softwareConfig.imageVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.software-config.properties" => Some(("config.softwareConfig.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "config.config-bucket" => Some(("config.configBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.gce-cluster-config.internal-ip-only" => Some(("config.gceClusterConfig.internalIpOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.gce-cluster-config.network-uri" => Some(("config.gceClusterConfig.networkUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.gce-cluster-config.tags" => Some(("config.gceClusterConfig.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.gce-cluster-config.service-account" => Some(("config.gceClusterConfig.serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.gce-cluster-config.zone-uri" => Some(("config.gceClusterConfig.zoneUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.gce-cluster-config.subnetwork-uri" => Some(("config.gceClusterConfig.subnetworkUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.gce-cluster-config.service-account-scopes" => Some(("config.gceClusterConfig.serviceAccountScopes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.gce-cluster-config.metadata" => Some(("config.gceClusterConfig.metadata", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "config.worker-config.num-instances" => Some(("config.workerConfig.numInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.worker-config.machine-type-uri" => Some(("config.workerConfig.machineTypeUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.worker-config.instance-names" => Some(("config.workerConfig.instanceNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.worker-config.image-uri" => Some(("config.workerConfig.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.worker-config.managed-group-config.instance-template-name" => Some(("config.workerConfig.managedGroupConfig.instanceTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.worker-config.managed-group-config.instance-group-manager-name" => Some(("config.workerConfig.managedGroupConfig.instanceGroupManagerName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.worker-config.is-preemptible" => Some(("config.workerConfig.isPreemptible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.worker-config.disk-config.num-local-ssds" => Some(("config.workerConfig.diskConfig.numLocalSsds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.worker-config.disk-config.boot-disk-size-gb" => Some(("config.workerConfig.diskConfig.bootDiskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.num-instances" => Some(("config.secondaryWorkerConfig.numInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.machine-type-uri" => Some(("config.secondaryWorkerConfig.machineTypeUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.instance-names" => Some(("config.secondaryWorkerConfig.instanceNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.secondary-worker-config.image-uri" => Some(("config.secondaryWorkerConfig.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.managed-group-config.instance-template-name" => Some(("config.secondaryWorkerConfig.managedGroupConfig.instanceTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.managed-group-config.instance-group-manager-name" => Some(("config.secondaryWorkerConfig.managedGroupConfig.instanceGroupManagerName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.is-preemptible" => Some(("config.secondaryWorkerConfig.isPreemptible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.disk-config.num-local-ssds" => Some(("config.secondaryWorkerConfig.diskConfig.numLocalSsds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.disk-config.boot-disk-size-gb" => Some(("config.secondaryWorkerConfig.diskConfig.bootDiskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.master-config.num-instances" => Some(("config.masterConfig.numInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.master-config.machine-type-uri" => Some(("config.masterConfig.machineTypeUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.master-config.instance-names" => Some(("config.masterConfig.instanceNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.master-config.image-uri" => Some(("config.masterConfig.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.master-config.managed-group-config.instance-template-name" => Some(("config.masterConfig.managedGroupConfig.instanceTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.master-config.managed-group-config.instance-group-manager-name" => Some(("config.masterConfig.managedGroupConfig.instanceGroupManagerName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.master-config.is-preemptible" => Some(("config.masterConfig.isPreemptible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.master-config.disk-config.num-local-ssds" => Some(("config.masterConfig.diskConfig.numLocalSsds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.master-config.disk-config.boot-disk-size-gb" => Some(("config.masterConfig.diskConfig.bootDiskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["boot-disk-size-gb", "cluster-name", "cluster-uuid", "config", "config-bucket", "detail", "disk-config", "gce-cluster-config", "hdfs-metrics", "image-uri", "image-version", "instance-group-manager-name", "instance-names", "instance-template-name", "internal-ip-only", "is-preemptible", "labels", "machine-type-uri", "managed-group-config", "master-config", "metadata", "metrics", "network-uri", "num-instances", "num-local-ssds", "project-id", "properties", "secondary-worker-config", "service-account", "service-account-scopes", "software-config", "state", "state-start-time", "status", "subnetwork-uri", "substate", "tags", "worker-config", "yarn-metrics", "zone-uri"]);
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
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
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

    fn _projects_regions_clusters_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().regions_clusters_delete(opt.value_of("project-id").unwrap_or(""), opt.value_of("region").unwrap_or(""), opt.value_of("cluster-name").unwrap_or(""));
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

    fn _projects_regions_clusters_diagnose(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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

    fn _projects_regions_clusters_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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

    fn _projects_regions_clusters_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().regions_clusters_list(opt.value_of("project-id").unwrap_or(""), opt.value_of("region").unwrap_or(""));
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

    fn _projects_regions_clusters_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "status.state" => Some(("status.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.state-start-time" => Some(("status.stateStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.substate" => Some(("status.substate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.detail" => Some(("status.detail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster-uuid" => Some(("clusterUuid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster-name" => Some(("clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metrics.yarn-metrics" => Some(("metrics.yarnMetrics", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "metrics.hdfs-metrics" => Some(("metrics.hdfsMetrics", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "config.software-config.image-version" => Some(("config.softwareConfig.imageVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.software-config.properties" => Some(("config.softwareConfig.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "config.config-bucket" => Some(("config.configBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.gce-cluster-config.internal-ip-only" => Some(("config.gceClusterConfig.internalIpOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.gce-cluster-config.network-uri" => Some(("config.gceClusterConfig.networkUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.gce-cluster-config.tags" => Some(("config.gceClusterConfig.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.gce-cluster-config.service-account" => Some(("config.gceClusterConfig.serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.gce-cluster-config.zone-uri" => Some(("config.gceClusterConfig.zoneUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.gce-cluster-config.subnetwork-uri" => Some(("config.gceClusterConfig.subnetworkUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.gce-cluster-config.service-account-scopes" => Some(("config.gceClusterConfig.serviceAccountScopes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.gce-cluster-config.metadata" => Some(("config.gceClusterConfig.metadata", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "config.worker-config.num-instances" => Some(("config.workerConfig.numInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.worker-config.machine-type-uri" => Some(("config.workerConfig.machineTypeUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.worker-config.instance-names" => Some(("config.workerConfig.instanceNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.worker-config.image-uri" => Some(("config.workerConfig.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.worker-config.managed-group-config.instance-template-name" => Some(("config.workerConfig.managedGroupConfig.instanceTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.worker-config.managed-group-config.instance-group-manager-name" => Some(("config.workerConfig.managedGroupConfig.instanceGroupManagerName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.worker-config.is-preemptible" => Some(("config.workerConfig.isPreemptible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.worker-config.disk-config.num-local-ssds" => Some(("config.workerConfig.diskConfig.numLocalSsds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.worker-config.disk-config.boot-disk-size-gb" => Some(("config.workerConfig.diskConfig.bootDiskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.num-instances" => Some(("config.secondaryWorkerConfig.numInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.machine-type-uri" => Some(("config.secondaryWorkerConfig.machineTypeUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.instance-names" => Some(("config.secondaryWorkerConfig.instanceNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.secondary-worker-config.image-uri" => Some(("config.secondaryWorkerConfig.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.managed-group-config.instance-template-name" => Some(("config.secondaryWorkerConfig.managedGroupConfig.instanceTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.managed-group-config.instance-group-manager-name" => Some(("config.secondaryWorkerConfig.managedGroupConfig.instanceGroupManagerName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.is-preemptible" => Some(("config.secondaryWorkerConfig.isPreemptible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.disk-config.num-local-ssds" => Some(("config.secondaryWorkerConfig.diskConfig.numLocalSsds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.secondary-worker-config.disk-config.boot-disk-size-gb" => Some(("config.secondaryWorkerConfig.diskConfig.bootDiskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.master-config.num-instances" => Some(("config.masterConfig.numInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.master-config.machine-type-uri" => Some(("config.masterConfig.machineTypeUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.master-config.instance-names" => Some(("config.masterConfig.instanceNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.master-config.image-uri" => Some(("config.masterConfig.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.master-config.managed-group-config.instance-template-name" => Some(("config.masterConfig.managedGroupConfig.instanceTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.master-config.managed-group-config.instance-group-manager-name" => Some(("config.masterConfig.managedGroupConfig.instanceGroupManagerName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.master-config.is-preemptible" => Some(("config.masterConfig.isPreemptible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.master-config.disk-config.num-local-ssds" => Some(("config.masterConfig.diskConfig.numLocalSsds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.master-config.disk-config.boot-disk-size-gb" => Some(("config.masterConfig.diskConfig.bootDiskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["boot-disk-size-gb", "cluster-name", "cluster-uuid", "config", "config-bucket", "detail", "disk-config", "gce-cluster-config", "hdfs-metrics", "image-uri", "image-version", "instance-group-manager-name", "instance-names", "instance-template-name", "internal-ip-only", "is-preemptible", "labels", "machine-type-uri", "managed-group-config", "master-config", "metadata", "metrics", "network-uri", "num-instances", "num-local-ssds", "project-id", "properties", "secondary-worker-config", "service-account", "service-account-scopes", "software-config", "state", "state-start-time", "status", "subnetwork-uri", "substate", "tags", "worker-config", "yarn-metrics", "zone-uri"]);
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
                    call = call.update_mask(value.unwrap_or(""));
                },
                "graceful-decommission-timeout" => {
                    call = call.graceful_decommission_timeout(value.unwrap_or(""));
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
                                                                           v.extend(["update-mask", "graceful-decommission-timeout"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    fn _projects_regions_jobs_cancel(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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

    fn _projects_regions_jobs_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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

    fn _projects_regions_jobs_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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

    fn _projects_regions_jobs_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().regions_jobs_list(opt.value_of("project-id").unwrap_or(""), opt.value_of("region").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
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
                                                                           v.extend(["filter", "page-token", "cluster-name", "job-state-matcher", "page-size"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    fn _projects_regions_jobs_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "status.state" => Some(("status.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.state-start-time" => Some(("status.stateStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.substate" => Some(("status.substate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.details" => Some(("status.details", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spark-sql-job.query-file-uri" => Some(("sparkSqlJob.queryFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spark-sql-job.script-variables" => Some(("sparkSqlJob.scriptVariables", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "spark-sql-job.logging-config.driver-log-levels" => Some(("sparkSqlJob.loggingConfig.driverLogLevels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "spark-sql-job.jar-file-uris" => Some(("sparkSqlJob.jarFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spark-sql-job.query-list.queries" => Some(("sparkSqlJob.queryList.queries", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spark-sql-job.properties" => Some(("sparkSqlJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "pig-job.query-file-uri" => Some(("pigJob.queryFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pig-job.script-variables" => Some(("pigJob.scriptVariables", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "pig-job.logging-config.driver-log-levels" => Some(("pigJob.loggingConfig.driverLogLevels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "pig-job.jar-file-uris" => Some(("pigJob.jarFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "pig-job.query-list.queries" => Some(("pigJob.queryList.queries", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "pig-job.continue-on-failure" => Some(("pigJob.continueOnFailure", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "pig-job.properties" => Some(("pigJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "placement.cluster-name" => Some(("placement.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "placement.cluster-uuid" => Some(("placement.clusterUuid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "reference.project-id" => Some(("reference.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "reference.job-id" => Some(("reference.jobId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hadoop-job.args" => Some(("hadoopJob.args", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "hadoop-job.logging-config.driver-log-levels" => Some(("hadoopJob.loggingConfig.driverLogLevels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "hadoop-job.jar-file-uris" => Some(("hadoopJob.jarFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "hadoop-job.file-uris" => Some(("hadoopJob.fileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "hadoop-job.main-class" => Some(("hadoopJob.mainClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hadoop-job.archive-uris" => Some(("hadoopJob.archiveUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "hadoop-job.main-jar-file-uri" => Some(("hadoopJob.mainJarFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hadoop-job.properties" => Some(("hadoopJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "driver-output-resource-uri" => Some(("driverOutputResourceUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "driver-control-files-uri" => Some(("driverControlFilesUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spark-job.args" => Some(("sparkJob.args", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spark-job.logging-config.driver-log-levels" => Some(("sparkJob.loggingConfig.driverLogLevels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "spark-job.jar-file-uris" => Some(("sparkJob.jarFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spark-job.file-uris" => Some(("sparkJob.fileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spark-job.main-class" => Some(("sparkJob.mainClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spark-job.archive-uris" => Some(("sparkJob.archiveUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spark-job.main-jar-file-uri" => Some(("sparkJob.mainJarFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spark-job.properties" => Some(("sparkJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "scheduling.max-failures-per-hour" => Some(("scheduling.maxFailuresPerHour", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "pyspark-job.main-python-file-uri" => Some(("pysparkJob.mainPythonFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pyspark-job.args" => Some(("pysparkJob.args", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "pyspark-job.logging-config.driver-log-levels" => Some(("pysparkJob.loggingConfig.driverLogLevels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "pyspark-job.jar-file-uris" => Some(("pysparkJob.jarFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "pyspark-job.file-uris" => Some(("pysparkJob.fileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "pyspark-job.archive-uris" => Some(("pysparkJob.archiveUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "pyspark-job.python-file-uris" => Some(("pysparkJob.pythonFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "pyspark-job.properties" => Some(("pysparkJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "hive-job.query-file-uri" => Some(("hiveJob.queryFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hive-job.script-variables" => Some(("hiveJob.scriptVariables", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "hive-job.jar-file-uris" => Some(("hiveJob.jarFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "hive-job.query-list.queries" => Some(("hiveJob.queryList.queries", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "hive-job.continue-on-failure" => Some(("hiveJob.continueOnFailure", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "hive-job.properties" => Some(("hiveJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["archive-uris", "args", "cluster-name", "cluster-uuid", "continue-on-failure", "details", "driver-control-files-uri", "driver-log-levels", "driver-output-resource-uri", "file-uris", "hadoop-job", "hive-job", "jar-file-uris", "job-id", "labels", "logging-config", "main-class", "main-jar-file-uri", "main-python-file-uri", "max-failures-per-hour", "pig-job", "placement", "project-id", "properties", "pyspark-job", "python-file-uris", "queries", "query-file-uri", "query-list", "reference", "scheduling", "script-variables", "spark-job", "spark-sql-job", "state", "state-start-time", "status", "substate"]);
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

    fn _projects_regions_jobs_submit(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "job.status.state" => Some(("job.status.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.status.state-start-time" => Some(("job.status.stateStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.status.substate" => Some(("job.status.substate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.status.details" => Some(("job.status.details", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.spark-sql-job.query-file-uri" => Some(("job.sparkSqlJob.queryFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.spark-sql-job.script-variables" => Some(("job.sparkSqlJob.scriptVariables", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.spark-sql-job.logging-config.driver-log-levels" => Some(("job.sparkSqlJob.loggingConfig.driverLogLevels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.spark-sql-job.jar-file-uris" => Some(("job.sparkSqlJob.jarFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.spark-sql-job.query-list.queries" => Some(("job.sparkSqlJob.queryList.queries", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.spark-sql-job.properties" => Some(("job.sparkSqlJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.pig-job.query-file-uri" => Some(("job.pigJob.queryFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.pig-job.script-variables" => Some(("job.pigJob.scriptVariables", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.pig-job.logging-config.driver-log-levels" => Some(("job.pigJob.loggingConfig.driverLogLevels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.pig-job.jar-file-uris" => Some(("job.pigJob.jarFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.pig-job.query-list.queries" => Some(("job.pigJob.queryList.queries", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.pig-job.continue-on-failure" => Some(("job.pigJob.continueOnFailure", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job.pig-job.properties" => Some(("job.pigJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.placement.cluster-name" => Some(("job.placement.clusterName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.placement.cluster-uuid" => Some(("job.placement.clusterUuid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.reference.project-id" => Some(("job.reference.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.reference.job-id" => Some(("job.reference.jobId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.hadoop-job.args" => Some(("job.hadoopJob.args", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.hadoop-job.logging-config.driver-log-levels" => Some(("job.hadoopJob.loggingConfig.driverLogLevels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.hadoop-job.jar-file-uris" => Some(("job.hadoopJob.jarFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.hadoop-job.file-uris" => Some(("job.hadoopJob.fileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.hadoop-job.main-class" => Some(("job.hadoopJob.mainClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.hadoop-job.archive-uris" => Some(("job.hadoopJob.archiveUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.hadoop-job.main-jar-file-uri" => Some(("job.hadoopJob.mainJarFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.hadoop-job.properties" => Some(("job.hadoopJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.labels" => Some(("job.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.driver-output-resource-uri" => Some(("job.driverOutputResourceUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.driver-control-files-uri" => Some(("job.driverControlFilesUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.spark-job.args" => Some(("job.sparkJob.args", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.spark-job.logging-config.driver-log-levels" => Some(("job.sparkJob.loggingConfig.driverLogLevels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.spark-job.jar-file-uris" => Some(("job.sparkJob.jarFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.spark-job.file-uris" => Some(("job.sparkJob.fileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.spark-job.main-class" => Some(("job.sparkJob.mainClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.spark-job.archive-uris" => Some(("job.sparkJob.archiveUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.spark-job.main-jar-file-uri" => Some(("job.sparkJob.mainJarFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.spark-job.properties" => Some(("job.sparkJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.scheduling.max-failures-per-hour" => Some(("job.scheduling.maxFailuresPerHour", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job.pyspark-job.main-python-file-uri" => Some(("job.pysparkJob.mainPythonFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.pyspark-job.args" => Some(("job.pysparkJob.args", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.pyspark-job.logging-config.driver-log-levels" => Some(("job.pysparkJob.loggingConfig.driverLogLevels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.pyspark-job.jar-file-uris" => Some(("job.pysparkJob.jarFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.pyspark-job.file-uris" => Some(("job.pysparkJob.fileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.pyspark-job.archive-uris" => Some(("job.pysparkJob.archiveUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.pyspark-job.python-file-uris" => Some(("job.pysparkJob.pythonFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.pyspark-job.properties" => Some(("job.pysparkJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.hive-job.query-file-uri" => Some(("job.hiveJob.queryFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.hive-job.script-variables" => Some(("job.hiveJob.scriptVariables", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job.hive-job.jar-file-uris" => Some(("job.hiveJob.jarFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.hive-job.query-list.queries" => Some(("job.hiveJob.queryList.queries", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.hive-job.continue-on-failure" => Some(("job.hiveJob.continueOnFailure", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job.hive-job.properties" => Some(("job.hiveJob.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["archive-uris", "args", "cluster-name", "cluster-uuid", "continue-on-failure", "details", "driver-control-files-uri", "driver-log-levels", "driver-output-resource-uri", "file-uris", "hadoop-job", "hive-job", "jar-file-uris", "job", "job-id", "labels", "logging-config", "main-class", "main-jar-file-uri", "main-python-file-uri", "max-failures-per-hour", "pig-job", "placement", "project-id", "properties", "pyspark-job", "python-file-uris", "queries", "query-file-uri", "query-list", "reference", "scheduling", "script-variables", "spark-job", "spark-sql-job", "state", "state-start-time", "status", "substate"]);
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

    fn _projects_regions_operations_cancel(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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

    fn _projects_regions_operations_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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

    fn _projects_regions_operations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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

    fn _projects_regions_operations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().regions_operations_list(opt.value_of("name").unwrap_or(""));
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

    fn _doit(&self, dry_run: bool) -> Result<Result<(), DoitError>, Option<InvalidOptionsError>> {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Result<(), DoitError> = Ok(());
        let mut err_opt: Option<InvalidOptionsError> = None;
        match self.opt.subcommand() {
            ("projects", Some(opt)) => {
                match opt.subcommand() {
                    ("regions-clusters-create", Some(opt)) => {
                        call_result = self._projects_regions_clusters_create(opt, dry_run, &mut err);
                    },
                    ("regions-clusters-delete", Some(opt)) => {
                        call_result = self._projects_regions_clusters_delete(opt, dry_run, &mut err);
                    },
                    ("regions-clusters-diagnose", Some(opt)) => {
                        call_result = self._projects_regions_clusters_diagnose(opt, dry_run, &mut err);
                    },
                    ("regions-clusters-get", Some(opt)) => {
                        call_result = self._projects_regions_clusters_get(opt, dry_run, &mut err);
                    },
                    ("regions-clusters-list", Some(opt)) => {
                        call_result = self._projects_regions_clusters_list(opt, dry_run, &mut err);
                    },
                    ("regions-clusters-patch", Some(opt)) => {
                        call_result = self._projects_regions_clusters_patch(opt, dry_run, &mut err);
                    },
                    ("regions-jobs-cancel", Some(opt)) => {
                        call_result = self._projects_regions_jobs_cancel(opt, dry_run, &mut err);
                    },
                    ("regions-jobs-delete", Some(opt)) => {
                        call_result = self._projects_regions_jobs_delete(opt, dry_run, &mut err);
                    },
                    ("regions-jobs-get", Some(opt)) => {
                        call_result = self._projects_regions_jobs_get(opt, dry_run, &mut err);
                    },
                    ("regions-jobs-list", Some(opt)) => {
                        call_result = self._projects_regions_jobs_list(opt, dry_run, &mut err);
                    },
                    ("regions-jobs-patch", Some(opt)) => {
                        call_result = self._projects_regions_jobs_patch(opt, dry_run, &mut err);
                    },
                    ("regions-jobs-submit", Some(opt)) => {
                        call_result = self._projects_regions_jobs_submit(opt, dry_run, &mut err);
                    },
                    ("regions-operations-cancel", Some(opt)) => {
                        call_result = self._projects_regions_operations_cancel(opt, dry_run, &mut err);
                    },
                    ("regions-operations-delete", Some(opt)) => {
                        call_result = self._projects_regions_operations_delete(opt, dry_run, &mut err);
                    },
                    ("regions-operations-get", Some(opt)) => {
                        call_result = self._projects_regions_operations_get(opt, dry_run, &mut err);
                    },
                    ("regions-operations-list", Some(opt)) => {
                        call_result = self._projects_regions_operations_list(opt, dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "dataproc1-secret.json",
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
                                          program_name: "dataproc1",
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
            hub: api::Dataproc::new(client, auth),
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
        ("projects", "methods: 'regions-clusters-create', 'regions-clusters-delete', 'regions-clusters-diagnose', 'regions-clusters-get', 'regions-clusters-list', 'regions-clusters-patch', 'regions-jobs-cancel', 'regions-jobs-delete', 'regions-jobs-get', 'regions-jobs-list', 'regions-jobs-patch', 'regions-jobs-submit', 'regions-operations-cancel', 'regions-operations-delete', 'regions-operations-get' and 'regions-operations-list'", vec![
            ("regions-clusters-create",
                    Some(r##"Creates a cluster in a project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-clusters-create",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. The ID of the Google Cloud Platform project that the cluster belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"region"##),
                     None,
                     Some(r##"Required. The Cloud Dataproc region in which to handle the request."##),
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
                    Some(r##"Deletes a cluster in a project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-clusters-delete",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. The ID of the Google Cloud Platform project that the cluster belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"region"##),
                     None,
                     Some(r##"Required. The Cloud Dataproc region in which to handle the request."##),
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
                    Some(r##"Gets cluster diagnostic information. After the operation completes, the Operation.response field contains DiagnoseClusterOutputLocation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-clusters-diagnose",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. The ID of the Google Cloud Platform project that the cluster belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"region"##),
                     None,
                     Some(r##"Required. The Cloud Dataproc region in which to handle the request."##),
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
                     Some(r##"Required. The Cloud Dataproc region in which to handle the request."##),
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
            ("regions-clusters-list",
                    Some(r##"Lists all regions/{region}/clusters in a project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-clusters-list",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. The ID of the Google Cloud Platform project that the cluster belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"region"##),
                     None,
                     Some(r##"Required. The Cloud Dataproc region in which to handle the request."##),
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
            ("regions-clusters-patch",
                    Some(r##"Updates a cluster in a project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-clusters-patch",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. The ID of the Google Cloud Platform project the cluster belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"region"##),
                     None,
                     Some(r##"Required. The Cloud Dataproc region in which to handle the request."##),
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
            ("regions-jobs-cancel",
                    Some(r##"Starts a job cancellation request. To access the job resource after cancellation, call regions/{region}/jobs.list or regions/{region}/jobs.get."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dataproc1_cli/projects_regions-jobs-cancel",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. The ID of the Google Cloud Platform project that the job belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"region"##),
                     None,
                     Some(r##"Required. The Cloud Dataproc region in which to handle the request."##),
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
                     Some(r##"Required. The Cloud Dataproc region in which to handle the request."##),
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
                     Some(r##"Required. The Cloud Dataproc region in which to handle the request."##),
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
                     Some(r##"Required. The Cloud Dataproc region in which to handle the request."##),
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
                     Some(r##"Required. The Cloud Dataproc region in which to handle the request."##),
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
                     Some(r##"Required. The Cloud Dataproc region in which to handle the request."##),
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
            ("regions-operations-list",
                    Some(r##"Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns UNIMPLEMENTED.NOTE: the name binding allows API services to override the binding to use different resource name schemes, such as users/*/operations. To override the binding, API services can add a binding such as "/v1/{name=users/*}/operations" to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id."##),
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
            ]),
        
    ];
    
    let mut app = App::new("dataproc1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("1.0.6+20171207")
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