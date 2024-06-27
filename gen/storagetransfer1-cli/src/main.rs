// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_storagetransfer1::{api, Error, oauth2, client::chrono, FieldMask};


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
    hub: api::Storagetransfer<S>,
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
    async fn _google_service_accounts_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.google_service_accounts().get(opt.value_of("project-id").unwrap_or(""));
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

    async fn _projects_agent_pools_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "bandwidth-limit.limit-mbps" => Some(("bandwidthLimit.limitMbps", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["bandwidth-limit", "display-name", "limit-mbps", "name", "state"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::AgentPool = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().agent_pools_create(request, opt.value_of("project-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "agent-pool-id" => {
                    call = call.agent_pool_id(value.unwrap_or(""));
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
                                                                           v.extend(["agent-pool-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _projects_agent_pools_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().agent_pools_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_agent_pools_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().agent_pools_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_agent_pools_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().agent_pools_list(opt.value_of("project-id").unwrap_or(""));
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

    async fn _projects_agent_pools_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "bandwidth-limit.limit-mbps" => Some(("bandwidthLimit.limitMbps", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["bandwidth-limit", "display-name", "limit-mbps", "name", "state"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::AgentPool = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().agent_pools_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _transfer_jobs_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "creation-time" => Some(("creationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deletion-time" => Some(("deletionTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event-stream.event-stream-expiration-time" => Some(("eventStream.eventStreamExpirationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event-stream.event-stream-start-time" => Some(("eventStream.eventStreamStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event-stream.name" => Some(("eventStream.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modification-time" => Some(("lastModificationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "latest-operation-name" => Some(("latestOperationName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "logging-config.enable-onprem-gcs-transfer-logs" => Some(("loggingConfig.enableOnpremGcsTransferLogs", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "logging-config.log-action-states" => Some(("loggingConfig.logActionStates", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "logging-config.log-actions" => Some(("loggingConfig.logActions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "notification-config.event-types" => Some(("notificationConfig.eventTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "notification-config.payload-format" => Some(("notificationConfig.payloadFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "notification-config.pubsub-topic" => Some(("notificationConfig.pubsubTopic", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replication-spec.gcs-data-sink.bucket-name" => Some(("replicationSpec.gcsDataSink.bucketName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replication-spec.gcs-data-sink.managed-folder-transfer-enabled" => Some(("replicationSpec.gcsDataSink.managedFolderTransferEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "replication-spec.gcs-data-sink.path" => Some(("replicationSpec.gcsDataSink.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replication-spec.gcs-data-source.bucket-name" => Some(("replicationSpec.gcsDataSource.bucketName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replication-spec.gcs-data-source.managed-folder-transfer-enabled" => Some(("replicationSpec.gcsDataSource.managedFolderTransferEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "replication-spec.gcs-data-source.path" => Some(("replicationSpec.gcsDataSource.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replication-spec.object-conditions.exclude-prefixes" => Some(("replicationSpec.objectConditions.excludePrefixes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "replication-spec.object-conditions.include-prefixes" => Some(("replicationSpec.objectConditions.includePrefixes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "replication-spec.object-conditions.last-modified-before" => Some(("replicationSpec.objectConditions.lastModifiedBefore", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replication-spec.object-conditions.last-modified-since" => Some(("replicationSpec.objectConditions.lastModifiedSince", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replication-spec.object-conditions.max-time-elapsed-since-last-modification" => Some(("replicationSpec.objectConditions.maxTimeElapsedSinceLastModification", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replication-spec.object-conditions.min-time-elapsed-since-last-modification" => Some(("replicationSpec.objectConditions.minTimeElapsedSinceLastModification", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replication-spec.transfer-options.delete-objects-from-source-after-transfer" => Some(("replicationSpec.transferOptions.deleteObjectsFromSourceAfterTransfer", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "replication-spec.transfer-options.delete-objects-unique-in-sink" => Some(("replicationSpec.transferOptions.deleteObjectsUniqueInSink", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "replication-spec.transfer-options.metadata-options.acl" => Some(("replicationSpec.transferOptions.metadataOptions.acl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replication-spec.transfer-options.metadata-options.gid" => Some(("replicationSpec.transferOptions.metadataOptions.gid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replication-spec.transfer-options.metadata-options.kms-key" => Some(("replicationSpec.transferOptions.metadataOptions.kmsKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replication-spec.transfer-options.metadata-options.mode" => Some(("replicationSpec.transferOptions.metadataOptions.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replication-spec.transfer-options.metadata-options.storage-class" => Some(("replicationSpec.transferOptions.metadataOptions.storageClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replication-spec.transfer-options.metadata-options.symlink" => Some(("replicationSpec.transferOptions.metadataOptions.symlink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replication-spec.transfer-options.metadata-options.temporary-hold" => Some(("replicationSpec.transferOptions.metadataOptions.temporaryHold", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replication-spec.transfer-options.metadata-options.time-created" => Some(("replicationSpec.transferOptions.metadataOptions.timeCreated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replication-spec.transfer-options.metadata-options.uid" => Some(("replicationSpec.transferOptions.metadataOptions.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replication-spec.transfer-options.overwrite-objects-already-existing-in-sink" => Some(("replicationSpec.transferOptions.overwriteObjectsAlreadyExistingInSink", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "replication-spec.transfer-options.overwrite-when" => Some(("replicationSpec.transferOptions.overwriteWhen", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "schedule.end-time-of-day.hours" => Some(("schedule.endTimeOfDay.hours", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "schedule.end-time-of-day.minutes" => Some(("schedule.endTimeOfDay.minutes", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "schedule.end-time-of-day.nanos" => Some(("schedule.endTimeOfDay.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "schedule.end-time-of-day.seconds" => Some(("schedule.endTimeOfDay.seconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "schedule.repeat-interval" => Some(("schedule.repeatInterval", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "schedule.schedule-end-date.day" => Some(("schedule.scheduleEndDate.day", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "schedule.schedule-end-date.month" => Some(("schedule.scheduleEndDate.month", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "schedule.schedule-end-date.year" => Some(("schedule.scheduleEndDate.year", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "schedule.schedule-start-date.day" => Some(("schedule.scheduleStartDate.day", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "schedule.schedule-start-date.month" => Some(("schedule.scheduleStartDate.month", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "schedule.schedule-start-date.year" => Some(("schedule.scheduleStartDate.year", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "schedule.start-time-of-day.hours" => Some(("schedule.startTimeOfDay.hours", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "schedule.start-time-of-day.minutes" => Some(("schedule.startTimeOfDay.minutes", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "schedule.start-time-of-day.nanos" => Some(("schedule.startTimeOfDay.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "schedule.start-time-of-day.seconds" => Some(("schedule.startTimeOfDay.seconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "status" => Some(("status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.aws-s3-compatible-data-source.bucket-name" => Some(("transferSpec.awsS3CompatibleDataSource.bucketName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.aws-s3-compatible-data-source.endpoint" => Some(("transferSpec.awsS3CompatibleDataSource.endpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.aws-s3-compatible-data-source.path" => Some(("transferSpec.awsS3CompatibleDataSource.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.aws-s3-compatible-data-source.region" => Some(("transferSpec.awsS3CompatibleDataSource.region", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.aws-s3-compatible-data-source.s3-metadata.auth-method" => Some(("transferSpec.awsS3CompatibleDataSource.s3Metadata.authMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.aws-s3-compatible-data-source.s3-metadata.list-api" => Some(("transferSpec.awsS3CompatibleDataSource.s3Metadata.listApi", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.aws-s3-compatible-data-source.s3-metadata.protocol" => Some(("transferSpec.awsS3CompatibleDataSource.s3Metadata.protocol", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.aws-s3-compatible-data-source.s3-metadata.request-model" => Some(("transferSpec.awsS3CompatibleDataSource.s3Metadata.requestModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.aws-s3-data-source.aws-access-key.access-key-id" => Some(("transferSpec.awsS3DataSource.awsAccessKey.accessKeyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.aws-s3-data-source.aws-access-key.secret-access-key" => Some(("transferSpec.awsS3DataSource.awsAccessKey.secretAccessKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.aws-s3-data-source.bucket-name" => Some(("transferSpec.awsS3DataSource.bucketName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.aws-s3-data-source.cloudfront-domain" => Some(("transferSpec.awsS3DataSource.cloudfrontDomain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.aws-s3-data-source.credentials-secret" => Some(("transferSpec.awsS3DataSource.credentialsSecret", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.aws-s3-data-source.managed-private-network" => Some(("transferSpec.awsS3DataSource.managedPrivateNetwork", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "transfer-spec.aws-s3-data-source.path" => Some(("transferSpec.awsS3DataSource.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.aws-s3-data-source.role-arn" => Some(("transferSpec.awsS3DataSource.roleArn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.azure-blob-storage-data-source.azure-credentials.sas-token" => Some(("transferSpec.azureBlobStorageDataSource.azureCredentials.sasToken", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.azure-blob-storage-data-source.container" => Some(("transferSpec.azureBlobStorageDataSource.container", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.azure-blob-storage-data-source.credentials-secret" => Some(("transferSpec.azureBlobStorageDataSource.credentialsSecret", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.azure-blob-storage-data-source.path" => Some(("transferSpec.azureBlobStorageDataSource.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.azure-blob-storage-data-source.storage-account" => Some(("transferSpec.azureBlobStorageDataSource.storageAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.gcs-data-sink.bucket-name" => Some(("transferSpec.gcsDataSink.bucketName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.gcs-data-sink.managed-folder-transfer-enabled" => Some(("transferSpec.gcsDataSink.managedFolderTransferEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "transfer-spec.gcs-data-sink.path" => Some(("transferSpec.gcsDataSink.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.gcs-data-source.bucket-name" => Some(("transferSpec.gcsDataSource.bucketName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.gcs-data-source.managed-folder-transfer-enabled" => Some(("transferSpec.gcsDataSource.managedFolderTransferEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "transfer-spec.gcs-data-source.path" => Some(("transferSpec.gcsDataSource.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.gcs-intermediate-data-location.bucket-name" => Some(("transferSpec.gcsIntermediateDataLocation.bucketName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.gcs-intermediate-data-location.managed-folder-transfer-enabled" => Some(("transferSpec.gcsIntermediateDataLocation.managedFolderTransferEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "transfer-spec.gcs-intermediate-data-location.path" => Some(("transferSpec.gcsIntermediateDataLocation.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.hdfs-data-source.path" => Some(("transferSpec.hdfsDataSource.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.http-data-source.list-url" => Some(("transferSpec.httpDataSource.listUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.object-conditions.exclude-prefixes" => Some(("transferSpec.objectConditions.excludePrefixes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "transfer-spec.object-conditions.include-prefixes" => Some(("transferSpec.objectConditions.includePrefixes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "transfer-spec.object-conditions.last-modified-before" => Some(("transferSpec.objectConditions.lastModifiedBefore", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.object-conditions.last-modified-since" => Some(("transferSpec.objectConditions.lastModifiedSince", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.object-conditions.max-time-elapsed-since-last-modification" => Some(("transferSpec.objectConditions.maxTimeElapsedSinceLastModification", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.object-conditions.min-time-elapsed-since-last-modification" => Some(("transferSpec.objectConditions.minTimeElapsedSinceLastModification", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.posix-data-sink.root-directory" => Some(("transferSpec.posixDataSink.rootDirectory", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.posix-data-source.root-directory" => Some(("transferSpec.posixDataSource.rootDirectory", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.sink-agent-pool-name" => Some(("transferSpec.sinkAgentPoolName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.source-agent-pool-name" => Some(("transferSpec.sourceAgentPoolName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.transfer-manifest.location" => Some(("transferSpec.transferManifest.location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.transfer-options.delete-objects-from-source-after-transfer" => Some(("transferSpec.transferOptions.deleteObjectsFromSourceAfterTransfer", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "transfer-spec.transfer-options.delete-objects-unique-in-sink" => Some(("transferSpec.transferOptions.deleteObjectsUniqueInSink", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "transfer-spec.transfer-options.metadata-options.acl" => Some(("transferSpec.transferOptions.metadataOptions.acl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.transfer-options.metadata-options.gid" => Some(("transferSpec.transferOptions.metadataOptions.gid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.transfer-options.metadata-options.kms-key" => Some(("transferSpec.transferOptions.metadataOptions.kmsKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.transfer-options.metadata-options.mode" => Some(("transferSpec.transferOptions.metadataOptions.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.transfer-options.metadata-options.storage-class" => Some(("transferSpec.transferOptions.metadataOptions.storageClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.transfer-options.metadata-options.symlink" => Some(("transferSpec.transferOptions.metadataOptions.symlink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.transfer-options.metadata-options.temporary-hold" => Some(("transferSpec.transferOptions.metadataOptions.temporaryHold", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.transfer-options.metadata-options.time-created" => Some(("transferSpec.transferOptions.metadataOptions.timeCreated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.transfer-options.metadata-options.uid" => Some(("transferSpec.transferOptions.metadataOptions.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-spec.transfer-options.overwrite-objects-already-existing-in-sink" => Some(("transferSpec.transferOptions.overwriteObjectsAlreadyExistingInSink", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "transfer-spec.transfer-options.overwrite-when" => Some(("transferSpec.transferOptions.overwriteWhen", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["access-key-id", "acl", "auth-method", "aws-access-key", "aws-s3-compatible-data-source", "aws-s3-data-source", "azure-blob-storage-data-source", "azure-credentials", "bucket-name", "cloudfront-domain", "container", "creation-time", "credentials-secret", "day", "delete-objects-from-source-after-transfer", "delete-objects-unique-in-sink", "deletion-time", "description", "enable-onprem-gcs-transfer-logs", "end-time-of-day", "endpoint", "event-stream", "event-stream-expiration-time", "event-stream-start-time", "event-types", "exclude-prefixes", "gcs-data-sink", "gcs-data-source", "gcs-intermediate-data-location", "gid", "hdfs-data-source", "hours", "http-data-source", "include-prefixes", "kms-key", "last-modification-time", "last-modified-before", "last-modified-since", "latest-operation-name", "list-api", "list-url", "location", "log-action-states", "log-actions", "logging-config", "managed-folder-transfer-enabled", "managed-private-network", "max-time-elapsed-since-last-modification", "metadata-options", "min-time-elapsed-since-last-modification", "minutes", "mode", "month", "name", "nanos", "notification-config", "object-conditions", "overwrite-objects-already-existing-in-sink", "overwrite-when", "path", "payload-format", "posix-data-sink", "posix-data-source", "project-id", "protocol", "pubsub-topic", "region", "repeat-interval", "replication-spec", "request-model", "role-arn", "root-directory", "s3-metadata", "sas-token", "schedule", "schedule-end-date", "schedule-start-date", "seconds", "secret-access-key", "sink-agent-pool-name", "source-agent-pool-name", "start-time-of-day", "status", "storage-account", "storage-class", "symlink", "temporary-hold", "time-created", "transfer-manifest", "transfer-options", "transfer-spec", "uid", "year"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::TransferJob = json::value::from_value(object).unwrap();
        let mut call = self.hub.transfer_jobs().create(request);
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

    async fn _transfer_jobs_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.transfer_jobs().delete(opt.value_of("job-name").unwrap_or(""), opt.value_of("project-id").unwrap_or(""));
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

    async fn _transfer_jobs_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.transfer_jobs().get(opt.value_of("job-name").unwrap_or(""), opt.value_of("project-id").unwrap_or(""));
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

    async fn _transfer_jobs_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.transfer_jobs().list(opt.value_of("filter").unwrap_or(""));
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

    async fn _transfer_jobs_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "transfer-job.creation-time" => Some(("transferJob.creationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.deletion-time" => Some(("transferJob.deletionTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.description" => Some(("transferJob.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.event-stream.event-stream-expiration-time" => Some(("transferJob.eventStream.eventStreamExpirationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.event-stream.event-stream-start-time" => Some(("transferJob.eventStream.eventStreamStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.event-stream.name" => Some(("transferJob.eventStream.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.last-modification-time" => Some(("transferJob.lastModificationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.latest-operation-name" => Some(("transferJob.latestOperationName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.logging-config.enable-onprem-gcs-transfer-logs" => Some(("transferJob.loggingConfig.enableOnpremGcsTransferLogs", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "transfer-job.logging-config.log-action-states" => Some(("transferJob.loggingConfig.logActionStates", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "transfer-job.logging-config.log-actions" => Some(("transferJob.loggingConfig.logActions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "transfer-job.name" => Some(("transferJob.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.notification-config.event-types" => Some(("transferJob.notificationConfig.eventTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "transfer-job.notification-config.payload-format" => Some(("transferJob.notificationConfig.payloadFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.notification-config.pubsub-topic" => Some(("transferJob.notificationConfig.pubsubTopic", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.project-id" => Some(("transferJob.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.replication-spec.gcs-data-sink.bucket-name" => Some(("transferJob.replicationSpec.gcsDataSink.bucketName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.replication-spec.gcs-data-sink.managed-folder-transfer-enabled" => Some(("transferJob.replicationSpec.gcsDataSink.managedFolderTransferEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "transfer-job.replication-spec.gcs-data-sink.path" => Some(("transferJob.replicationSpec.gcsDataSink.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.replication-spec.gcs-data-source.bucket-name" => Some(("transferJob.replicationSpec.gcsDataSource.bucketName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.replication-spec.gcs-data-source.managed-folder-transfer-enabled" => Some(("transferJob.replicationSpec.gcsDataSource.managedFolderTransferEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "transfer-job.replication-spec.gcs-data-source.path" => Some(("transferJob.replicationSpec.gcsDataSource.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.replication-spec.object-conditions.exclude-prefixes" => Some(("transferJob.replicationSpec.objectConditions.excludePrefixes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "transfer-job.replication-spec.object-conditions.include-prefixes" => Some(("transferJob.replicationSpec.objectConditions.includePrefixes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "transfer-job.replication-spec.object-conditions.last-modified-before" => Some(("transferJob.replicationSpec.objectConditions.lastModifiedBefore", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.replication-spec.object-conditions.last-modified-since" => Some(("transferJob.replicationSpec.objectConditions.lastModifiedSince", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.replication-spec.object-conditions.max-time-elapsed-since-last-modification" => Some(("transferJob.replicationSpec.objectConditions.maxTimeElapsedSinceLastModification", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.replication-spec.object-conditions.min-time-elapsed-since-last-modification" => Some(("transferJob.replicationSpec.objectConditions.minTimeElapsedSinceLastModification", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.replication-spec.transfer-options.delete-objects-from-source-after-transfer" => Some(("transferJob.replicationSpec.transferOptions.deleteObjectsFromSourceAfterTransfer", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "transfer-job.replication-spec.transfer-options.delete-objects-unique-in-sink" => Some(("transferJob.replicationSpec.transferOptions.deleteObjectsUniqueInSink", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "transfer-job.replication-spec.transfer-options.metadata-options.acl" => Some(("transferJob.replicationSpec.transferOptions.metadataOptions.acl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.replication-spec.transfer-options.metadata-options.gid" => Some(("transferJob.replicationSpec.transferOptions.metadataOptions.gid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.replication-spec.transfer-options.metadata-options.kms-key" => Some(("transferJob.replicationSpec.transferOptions.metadataOptions.kmsKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.replication-spec.transfer-options.metadata-options.mode" => Some(("transferJob.replicationSpec.transferOptions.metadataOptions.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.replication-spec.transfer-options.metadata-options.storage-class" => Some(("transferJob.replicationSpec.transferOptions.metadataOptions.storageClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.replication-spec.transfer-options.metadata-options.symlink" => Some(("transferJob.replicationSpec.transferOptions.metadataOptions.symlink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.replication-spec.transfer-options.metadata-options.temporary-hold" => Some(("transferJob.replicationSpec.transferOptions.metadataOptions.temporaryHold", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.replication-spec.transfer-options.metadata-options.time-created" => Some(("transferJob.replicationSpec.transferOptions.metadataOptions.timeCreated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.replication-spec.transfer-options.metadata-options.uid" => Some(("transferJob.replicationSpec.transferOptions.metadataOptions.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.replication-spec.transfer-options.overwrite-objects-already-existing-in-sink" => Some(("transferJob.replicationSpec.transferOptions.overwriteObjectsAlreadyExistingInSink", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "transfer-job.replication-spec.transfer-options.overwrite-when" => Some(("transferJob.replicationSpec.transferOptions.overwriteWhen", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.schedule.end-time-of-day.hours" => Some(("transferJob.schedule.endTimeOfDay.hours", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "transfer-job.schedule.end-time-of-day.minutes" => Some(("transferJob.schedule.endTimeOfDay.minutes", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "transfer-job.schedule.end-time-of-day.nanos" => Some(("transferJob.schedule.endTimeOfDay.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "transfer-job.schedule.end-time-of-day.seconds" => Some(("transferJob.schedule.endTimeOfDay.seconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "transfer-job.schedule.repeat-interval" => Some(("transferJob.schedule.repeatInterval", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.schedule.schedule-end-date.day" => Some(("transferJob.schedule.scheduleEndDate.day", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "transfer-job.schedule.schedule-end-date.month" => Some(("transferJob.schedule.scheduleEndDate.month", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "transfer-job.schedule.schedule-end-date.year" => Some(("transferJob.schedule.scheduleEndDate.year", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "transfer-job.schedule.schedule-start-date.day" => Some(("transferJob.schedule.scheduleStartDate.day", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "transfer-job.schedule.schedule-start-date.month" => Some(("transferJob.schedule.scheduleStartDate.month", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "transfer-job.schedule.schedule-start-date.year" => Some(("transferJob.schedule.scheduleStartDate.year", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "transfer-job.schedule.start-time-of-day.hours" => Some(("transferJob.schedule.startTimeOfDay.hours", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "transfer-job.schedule.start-time-of-day.minutes" => Some(("transferJob.schedule.startTimeOfDay.minutes", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "transfer-job.schedule.start-time-of-day.nanos" => Some(("transferJob.schedule.startTimeOfDay.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "transfer-job.schedule.start-time-of-day.seconds" => Some(("transferJob.schedule.startTimeOfDay.seconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "transfer-job.status" => Some(("transferJob.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.aws-s3-compatible-data-source.bucket-name" => Some(("transferJob.transferSpec.awsS3CompatibleDataSource.bucketName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.aws-s3-compatible-data-source.endpoint" => Some(("transferJob.transferSpec.awsS3CompatibleDataSource.endpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.aws-s3-compatible-data-source.path" => Some(("transferJob.transferSpec.awsS3CompatibleDataSource.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.aws-s3-compatible-data-source.region" => Some(("transferJob.transferSpec.awsS3CompatibleDataSource.region", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.aws-s3-compatible-data-source.s3-metadata.auth-method" => Some(("transferJob.transferSpec.awsS3CompatibleDataSource.s3Metadata.authMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.aws-s3-compatible-data-source.s3-metadata.list-api" => Some(("transferJob.transferSpec.awsS3CompatibleDataSource.s3Metadata.listApi", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.aws-s3-compatible-data-source.s3-metadata.protocol" => Some(("transferJob.transferSpec.awsS3CompatibleDataSource.s3Metadata.protocol", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.aws-s3-compatible-data-source.s3-metadata.request-model" => Some(("transferJob.transferSpec.awsS3CompatibleDataSource.s3Metadata.requestModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.aws-s3-data-source.aws-access-key.access-key-id" => Some(("transferJob.transferSpec.awsS3DataSource.awsAccessKey.accessKeyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.aws-s3-data-source.aws-access-key.secret-access-key" => Some(("transferJob.transferSpec.awsS3DataSource.awsAccessKey.secretAccessKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.aws-s3-data-source.bucket-name" => Some(("transferJob.transferSpec.awsS3DataSource.bucketName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.aws-s3-data-source.cloudfront-domain" => Some(("transferJob.transferSpec.awsS3DataSource.cloudfrontDomain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.aws-s3-data-source.credentials-secret" => Some(("transferJob.transferSpec.awsS3DataSource.credentialsSecret", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.aws-s3-data-source.managed-private-network" => Some(("transferJob.transferSpec.awsS3DataSource.managedPrivateNetwork", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.aws-s3-data-source.path" => Some(("transferJob.transferSpec.awsS3DataSource.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.aws-s3-data-source.role-arn" => Some(("transferJob.transferSpec.awsS3DataSource.roleArn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.azure-blob-storage-data-source.azure-credentials.sas-token" => Some(("transferJob.transferSpec.azureBlobStorageDataSource.azureCredentials.sasToken", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.azure-blob-storage-data-source.container" => Some(("transferJob.transferSpec.azureBlobStorageDataSource.container", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.azure-blob-storage-data-source.credentials-secret" => Some(("transferJob.transferSpec.azureBlobStorageDataSource.credentialsSecret", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.azure-blob-storage-data-source.path" => Some(("transferJob.transferSpec.azureBlobStorageDataSource.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.azure-blob-storage-data-source.storage-account" => Some(("transferJob.transferSpec.azureBlobStorageDataSource.storageAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.gcs-data-sink.bucket-name" => Some(("transferJob.transferSpec.gcsDataSink.bucketName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.gcs-data-sink.managed-folder-transfer-enabled" => Some(("transferJob.transferSpec.gcsDataSink.managedFolderTransferEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.gcs-data-sink.path" => Some(("transferJob.transferSpec.gcsDataSink.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.gcs-data-source.bucket-name" => Some(("transferJob.transferSpec.gcsDataSource.bucketName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.gcs-data-source.managed-folder-transfer-enabled" => Some(("transferJob.transferSpec.gcsDataSource.managedFolderTransferEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.gcs-data-source.path" => Some(("transferJob.transferSpec.gcsDataSource.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.gcs-intermediate-data-location.bucket-name" => Some(("transferJob.transferSpec.gcsIntermediateDataLocation.bucketName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.gcs-intermediate-data-location.managed-folder-transfer-enabled" => Some(("transferJob.transferSpec.gcsIntermediateDataLocation.managedFolderTransferEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.gcs-intermediate-data-location.path" => Some(("transferJob.transferSpec.gcsIntermediateDataLocation.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.hdfs-data-source.path" => Some(("transferJob.transferSpec.hdfsDataSource.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.http-data-source.list-url" => Some(("transferJob.transferSpec.httpDataSource.listUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.object-conditions.exclude-prefixes" => Some(("transferJob.transferSpec.objectConditions.excludePrefixes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "transfer-job.transfer-spec.object-conditions.include-prefixes" => Some(("transferJob.transferSpec.objectConditions.includePrefixes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "transfer-job.transfer-spec.object-conditions.last-modified-before" => Some(("transferJob.transferSpec.objectConditions.lastModifiedBefore", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.object-conditions.last-modified-since" => Some(("transferJob.transferSpec.objectConditions.lastModifiedSince", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.object-conditions.max-time-elapsed-since-last-modification" => Some(("transferJob.transferSpec.objectConditions.maxTimeElapsedSinceLastModification", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.object-conditions.min-time-elapsed-since-last-modification" => Some(("transferJob.transferSpec.objectConditions.minTimeElapsedSinceLastModification", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.posix-data-sink.root-directory" => Some(("transferJob.transferSpec.posixDataSink.rootDirectory", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.posix-data-source.root-directory" => Some(("transferJob.transferSpec.posixDataSource.rootDirectory", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.sink-agent-pool-name" => Some(("transferJob.transferSpec.sinkAgentPoolName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.source-agent-pool-name" => Some(("transferJob.transferSpec.sourceAgentPoolName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.transfer-manifest.location" => Some(("transferJob.transferSpec.transferManifest.location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.transfer-options.delete-objects-from-source-after-transfer" => Some(("transferJob.transferSpec.transferOptions.deleteObjectsFromSourceAfterTransfer", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.transfer-options.delete-objects-unique-in-sink" => Some(("transferJob.transferSpec.transferOptions.deleteObjectsUniqueInSink", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.transfer-options.metadata-options.acl" => Some(("transferJob.transferSpec.transferOptions.metadataOptions.acl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.transfer-options.metadata-options.gid" => Some(("transferJob.transferSpec.transferOptions.metadataOptions.gid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.transfer-options.metadata-options.kms-key" => Some(("transferJob.transferSpec.transferOptions.metadataOptions.kmsKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.transfer-options.metadata-options.mode" => Some(("transferJob.transferSpec.transferOptions.metadataOptions.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.transfer-options.metadata-options.storage-class" => Some(("transferJob.transferSpec.transferOptions.metadataOptions.storageClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.transfer-options.metadata-options.symlink" => Some(("transferJob.transferSpec.transferOptions.metadataOptions.symlink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.transfer-options.metadata-options.temporary-hold" => Some(("transferJob.transferSpec.transferOptions.metadataOptions.temporaryHold", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.transfer-options.metadata-options.time-created" => Some(("transferJob.transferSpec.transferOptions.metadataOptions.timeCreated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.transfer-options.metadata-options.uid" => Some(("transferJob.transferSpec.transferOptions.metadataOptions.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.transfer-options.overwrite-objects-already-existing-in-sink" => Some(("transferJob.transferSpec.transferOptions.overwriteObjectsAlreadyExistingInSink", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "transfer-job.transfer-spec.transfer-options.overwrite-when" => Some(("transferJob.transferSpec.transferOptions.overwriteWhen", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-transfer-job-field-mask" => Some(("updateTransferJobFieldMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["access-key-id", "acl", "auth-method", "aws-access-key", "aws-s3-compatible-data-source", "aws-s3-data-source", "azure-blob-storage-data-source", "azure-credentials", "bucket-name", "cloudfront-domain", "container", "creation-time", "credentials-secret", "day", "delete-objects-from-source-after-transfer", "delete-objects-unique-in-sink", "deletion-time", "description", "enable-onprem-gcs-transfer-logs", "end-time-of-day", "endpoint", "event-stream", "event-stream-expiration-time", "event-stream-start-time", "event-types", "exclude-prefixes", "gcs-data-sink", "gcs-data-source", "gcs-intermediate-data-location", "gid", "hdfs-data-source", "hours", "http-data-source", "include-prefixes", "kms-key", "last-modification-time", "last-modified-before", "last-modified-since", "latest-operation-name", "list-api", "list-url", "location", "log-action-states", "log-actions", "logging-config", "managed-folder-transfer-enabled", "managed-private-network", "max-time-elapsed-since-last-modification", "metadata-options", "min-time-elapsed-since-last-modification", "minutes", "mode", "month", "name", "nanos", "notification-config", "object-conditions", "overwrite-objects-already-existing-in-sink", "overwrite-when", "path", "payload-format", "posix-data-sink", "posix-data-source", "project-id", "protocol", "pubsub-topic", "region", "repeat-interval", "replication-spec", "request-model", "role-arn", "root-directory", "s3-metadata", "sas-token", "schedule", "schedule-end-date", "schedule-start-date", "seconds", "secret-access-key", "sink-agent-pool-name", "source-agent-pool-name", "start-time-of-day", "status", "storage-account", "storage-class", "symlink", "temporary-hold", "time-created", "transfer-job", "transfer-manifest", "transfer-options", "transfer-spec", "uid", "update-transfer-job-field-mask", "year"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::UpdateTransferJobRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.transfer_jobs().patch(request, opt.value_of("job-name").unwrap_or(""));
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

    async fn _transfer_jobs_run(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["project-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::RunTransferJobRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.transfer_jobs().run(request, opt.value_of("job-name").unwrap_or(""));
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

    async fn _transfer_operations_cancel(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.transfer_operations().cancel(request, opt.value_of("name").unwrap_or(""));
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

    async fn _transfer_operations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.transfer_operations().get(opt.value_of("name").unwrap_or(""));
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

    async fn _transfer_operations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.transfer_operations().list(opt.value_of("name").unwrap_or(""), opt.value_of("filter").unwrap_or(""));
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

    async fn _transfer_operations_pause(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::PauseTransferOperationRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.transfer_operations().pause(request, opt.value_of("name").unwrap_or(""));
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

    async fn _transfer_operations_resume(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::ResumeTransferOperationRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.transfer_operations().resume(request, opt.value_of("name").unwrap_or(""));
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
            ("google-service-accounts", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._google_service_accounts_get(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("google-service-accounts".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("projects", Some(opt)) => {
                match opt.subcommand() {
                    ("agent-pools-create", Some(opt)) => {
                        call_result = self._projects_agent_pools_create(opt, dry_run, &mut err).await;
                    },
                    ("agent-pools-delete", Some(opt)) => {
                        call_result = self._projects_agent_pools_delete(opt, dry_run, &mut err).await;
                    },
                    ("agent-pools-get", Some(opt)) => {
                        call_result = self._projects_agent_pools_get(opt, dry_run, &mut err).await;
                    },
                    ("agent-pools-list", Some(opt)) => {
                        call_result = self._projects_agent_pools_list(opt, dry_run, &mut err).await;
                    },
                    ("agent-pools-patch", Some(opt)) => {
                        call_result = self._projects_agent_pools_patch(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("projects".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("transfer-jobs", Some(opt)) => {
                match opt.subcommand() {
                    ("create", Some(opt)) => {
                        call_result = self._transfer_jobs_create(opt, dry_run, &mut err).await;
                    },
                    ("delete", Some(opt)) => {
                        call_result = self._transfer_jobs_delete(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._transfer_jobs_get(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._transfer_jobs_list(opt, dry_run, &mut err).await;
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._transfer_jobs_patch(opt, dry_run, &mut err).await;
                    },
                    ("run", Some(opt)) => {
                        call_result = self._transfer_jobs_run(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("transfer-jobs".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("transfer-operations", Some(opt)) => {
                match opt.subcommand() {
                    ("cancel", Some(opt)) => {
                        call_result = self._transfer_operations_cancel(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._transfer_operations_get(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._transfer_operations_list(opt, dry_run, &mut err).await;
                    },
                    ("pause", Some(opt)) => {
                        call_result = self._transfer_operations_pause(opt, dry_run, &mut err).await;
                    },
                    ("resume", Some(opt)) => {
                        call_result = self._transfer_operations_resume(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("transfer-operations".to_string()));
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

            match client::application_secret_from_directory(&config_dir, "storagetransfer1-secret.json",
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
        ).persist_tokens_to_disk(format!("{}/storagetransfer1", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::Storagetransfer::new(client, auth),
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
        ("google-service-accounts", "methods: 'get'", vec![
            ("get",
                    Some(r##"Returns the Google service account that is used by Storage Transfer Service to access buckets in the project where transfers run or in other projects. Each Google service account is associated with one Google Cloud project. Users should add this service account to the Google Cloud Storage bucket ACLs to grant access to Storage Transfer Service. This service account is created and owned by Storage Transfer Service and can only be used by Storage Transfer Service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storagetransfer1_cli/google-service-accounts_get",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. The ID of the Google Cloud project that the Google service account is associated with."##),
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
        
        ("projects", "methods: 'agent-pools-create', 'agent-pools-delete', 'agent-pools-get', 'agent-pools-list' and 'agent-pools-patch'", vec![
            ("agent-pools-create",
                    Some(r##"Creates an agent pool resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storagetransfer1_cli/projects_agent-pools-create",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. The ID of the Google Cloud project that owns the agent pool."##),
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
            ("agent-pools-delete",
                    Some(r##"Deletes an agent pool."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storagetransfer1_cli/projects_agent-pools-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the agent pool to delete."##),
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
            ("agent-pools-get",
                    Some(r##"Gets an agent pool."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storagetransfer1_cli/projects_agent-pools-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the agent pool to get."##),
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
            ("agent-pools-list",
                    Some(r##"Lists agent pools."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storagetransfer1_cli/projects_agent-pools-list",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. The ID of the Google Cloud project that owns the job."##),
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
            ("agent-pools-patch",
                    Some(r##"Updates an existing agent pool resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storagetransfer1_cli/projects_agent-pools-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Specifies a unique string that identifies the agent pool. Format: `projects/{project_id}/agentPools/{agent_pool_id}`"##),
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
        
        ("transfer-jobs", "methods: 'create', 'delete', 'get', 'list', 'patch' and 'run'", vec![
            ("create",
                    Some(r##"Creates a transfer job that runs periodically."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storagetransfer1_cli/transfer-jobs_create",
                  vec![
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
            ("delete",
                    Some(r##"Deletes a transfer job. Deleting a transfer job sets its status to DELETED."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storagetransfer1_cli/transfer-jobs_delete",
                  vec![
                    (Some(r##"job-name"##),
                     None,
                     Some(r##"Required. The job to delete."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. The ID of the Google Cloud project that owns the job."##),
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
            ("get",
                    Some(r##"Gets a transfer job."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storagetransfer1_cli/transfer-jobs_get",
                  vec![
                    (Some(r##"job-name"##),
                     None,
                     Some(r##"Required. The job to get."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. The ID of the Google Cloud project that owns the job."##),
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
            ("list",
                    Some(r##"Lists transfer jobs."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storagetransfer1_cli/transfer-jobs_list",
                  vec![
                    (Some(r##"filter"##),
                     None,
                     Some(r##"Required. A list of query parameters specified as JSON text in the form of: `{"projectId":"my_project_id", "jobNames":["jobid1","jobid2",...], "jobStatuses":["status1","status2",...]}` Since `jobNames` and `jobStatuses` support multiple values, their values must be specified with array notation. `projectId` is required. `jobNames` and `jobStatuses` are optional. The valid values for `jobStatuses` are case-insensitive: ENABLED, DISABLED, and DELETED."##),
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
            ("patch",
                    Some(r##"Updates a transfer job. Updating a job's transfer spec does not affect transfer operations that are running already. **Note:** The job's status field can be modified using this RPC (for example, to set a job's status to DELETED, DISABLED, or ENABLED)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storagetransfer1_cli/transfer-jobs_patch",
                  vec![
                    (Some(r##"job-name"##),
                     None,
                     Some(r##"Required. The name of job to update."##),
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
            ("run",
                    Some(r##"Starts a new operation for the specified transfer job. A `TransferJob` has a maximum of one active `TransferOperation`. If this method is called while a `TransferOperation` is active, an error is returned."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storagetransfer1_cli/transfer-jobs_run",
                  vec![
                    (Some(r##"job-name"##),
                     None,
                     Some(r##"Required. The name of the transfer job."##),
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
        
        ("transfer-operations", "methods: 'cancel', 'get', 'list', 'pause' and 'resume'", vec![
            ("cancel",
                    Some(r##"Cancels a transfer. Use the transferOperations.get method to check if the cancellation succeeded or if the operation completed despite the `cancel` request. When you cancel an operation, the currently running transfer is interrupted. For recurring transfer jobs, the next instance of the transfer job will still run. For example, if your job is configured to run every day at 1pm and you cancel Monday's operation at 1:05pm, Monday's transfer will stop. However, a transfer job will still be attempted on Tuesday. This applies only to currently running operations. If an operation is not currently running, `cancel` does nothing. *Caution:* Canceling a transfer job can leave your data in an unknown state. We recommend that you restore the state at both the destination and the source after the `cancel` request completes so that your data is in a consistent state. When you cancel a job, the next job computes a delta of files and may repair any inconsistent state. For instance, if you run a job every day, and today's job found 10 new files and transferred five files before you canceled the job, tomorrow's transfer operation will compute a new delta with the five files that were not copied today plus any new files discovered tomorrow."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storagetransfer1_cli/transfer-operations_cancel",
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
                    Some(r##"Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storagetransfer1_cli/transfer-operations_get",
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
            ("list",
                    Some(r##"Lists transfer operations. Operations are ordered by their creation time in reverse chronological order."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storagetransfer1_cli/transfer-operations_list",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the type being listed; must be `transferOperations`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"filter"##),
                     None,
                     Some(r##"Required. A list of query parameters specified as JSON text in the form of: `{"projectId":"my_project_id", "jobNames":["jobid1","jobid2",...], "jobNamePattern": "job_name_pattern", "operationNames":["opid1","opid2",...], "operationNamePattern": "operation_name_pattern", "minCreationTime": "min_creation_time", "maxCreationTime": "max_creation_time", "transferStatuses":["status1","status2",...]}` Since `jobNames`, `operationNames`, and `transferStatuses` support multiple values, they must be specified with array notation. `projectId` is the only argument that is required. If specified, `jobNamePattern` and `operationNamePattern` must match the full job or operation name respectively. '*' is a wildcard matching 0 or more characters. `minCreationTime` and `maxCreationTime` should be timestamps encoded as a string in the [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format. The valid values for `transferStatuses` are case-insensitive: IN_PROGRESS, PAUSED, SUCCESS, FAILED, and ABORTED."##),
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
            ("pause",
                    Some(r##"Pauses a transfer operation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storagetransfer1_cli/transfer-operations_pause",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the transfer operation."##),
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
            ("resume",
                    Some(r##"Resumes a transfer operation that is paused."##),
                    "Details at http://byron.github.io/google-apis-rs/google_storagetransfer1_cli/transfer-operations_resume",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the transfer operation."##),
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
    
    let mut app = App::new("storagetransfer1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+20240622")
           .about("Transfers data from external data sources to a Google Cloud Storage bucket or between Google Cloud Storage buckets. ")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_storagetransfer1_cli")
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
