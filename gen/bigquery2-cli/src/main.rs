// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_bigquery2::{api, Error, oauth2, client::chrono, FieldMask};


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
    hub: api::Bigquery<S>,
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
    async fn _datasets_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.datasets().delete(opt.value_of("project-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "delete-contents" => {
                    call = call.delete_contents(        value.map(|v| arg_from_str(v, err, "delete-contents", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["delete-contents"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _datasets_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.datasets().get(opt.value_of("project-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "dataset-view" => {
                    call = call.dataset_view(value.unwrap_or(""));
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
                                                                           v.extend(["dataset-view"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _datasets_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "dataset-reference.dataset-id" => Some(("datasetReference.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dataset-reference.project-id" => Some(("datasetReference.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-collation" => Some(("defaultCollation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-encryption-configuration.kms-key-name" => Some(("defaultEncryptionConfiguration.kmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-partition-expiration-ms" => Some(("defaultPartitionExpirationMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-rounding-mode" => Some(("defaultRoundingMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-table-expiration-ms" => Some(("defaultTableExpirationMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-dataset-reference.connection" => Some(("externalDatasetReference.connection", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-dataset-reference.external-source" => Some(("externalDatasetReference.externalSource", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "friendly-name" => Some(("friendlyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "is-case-insensitive" => Some(("isCaseInsensitive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "last-modified-time" => Some(("lastModifiedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "linked-dataset-source.source-dataset.dataset-id" => Some(("linkedDatasetSource.sourceDataset.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "linked-dataset-source.source-dataset.project-id" => Some(("linkedDatasetSource.sourceDataset.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location" => Some(("location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "max-time-travel-hours" => Some(("maxTimeTravelHours", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "satisfies-pzi" => Some(("satisfiesPzi", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "satisfies-pzs" => Some(("satisfiesPzs", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "storage-billing-model" => Some(("storageBillingModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["connection", "creation-time", "dataset-id", "dataset-reference", "default-collation", "default-encryption-configuration", "default-partition-expiration-ms", "default-rounding-mode", "default-table-expiration-ms", "description", "etag", "external-dataset-reference", "external-source", "friendly-name", "id", "is-case-insensitive", "kind", "kms-key-name", "labels", "last-modified-time", "linked-dataset-source", "location", "max-time-travel-hours", "project-id", "satisfies-pzi", "satisfies-pzs", "self-link", "source-dataset", "storage-billing-model", "type"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Dataset = json::value::from_value(object).unwrap();
        let mut call = self.hub.datasets().insert(request, opt.value_of("project-id").unwrap_or(""));
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

    async fn _datasets_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.datasets().list(opt.value_of("project-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "uint32")).unwrap_or(0));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                "all" => {
                    call = call.all(        value.map(|v| arg_from_str(v, err, "all", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["all", "filter", "max-results", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _datasets_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "dataset-reference.dataset-id" => Some(("datasetReference.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dataset-reference.project-id" => Some(("datasetReference.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-collation" => Some(("defaultCollation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-encryption-configuration.kms-key-name" => Some(("defaultEncryptionConfiguration.kmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-partition-expiration-ms" => Some(("defaultPartitionExpirationMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-rounding-mode" => Some(("defaultRoundingMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-table-expiration-ms" => Some(("defaultTableExpirationMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-dataset-reference.connection" => Some(("externalDatasetReference.connection", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-dataset-reference.external-source" => Some(("externalDatasetReference.externalSource", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "friendly-name" => Some(("friendlyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "is-case-insensitive" => Some(("isCaseInsensitive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "last-modified-time" => Some(("lastModifiedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "linked-dataset-source.source-dataset.dataset-id" => Some(("linkedDatasetSource.sourceDataset.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "linked-dataset-source.source-dataset.project-id" => Some(("linkedDatasetSource.sourceDataset.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location" => Some(("location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "max-time-travel-hours" => Some(("maxTimeTravelHours", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "satisfies-pzi" => Some(("satisfiesPzi", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "satisfies-pzs" => Some(("satisfiesPzs", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "storage-billing-model" => Some(("storageBillingModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["connection", "creation-time", "dataset-id", "dataset-reference", "default-collation", "default-encryption-configuration", "default-partition-expiration-ms", "default-rounding-mode", "default-table-expiration-ms", "description", "etag", "external-dataset-reference", "external-source", "friendly-name", "id", "is-case-insensitive", "kind", "kms-key-name", "labels", "last-modified-time", "linked-dataset-source", "location", "max-time-travel-hours", "project-id", "satisfies-pzi", "satisfies-pzs", "self-link", "source-dataset", "storage-billing-model", "type"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Dataset = json::value::from_value(object).unwrap();
        let mut call = self.hub.datasets().patch(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""));
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

    async fn _datasets_undelete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "deletion-time" => Some(("deletionTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["deletion-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::UndeleteDatasetRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.datasets().undelete(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""));
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

    async fn _datasets_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "dataset-reference.dataset-id" => Some(("datasetReference.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dataset-reference.project-id" => Some(("datasetReference.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-collation" => Some(("defaultCollation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-encryption-configuration.kms-key-name" => Some(("defaultEncryptionConfiguration.kmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-partition-expiration-ms" => Some(("defaultPartitionExpirationMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-rounding-mode" => Some(("defaultRoundingMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-table-expiration-ms" => Some(("defaultTableExpirationMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-dataset-reference.connection" => Some(("externalDatasetReference.connection", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-dataset-reference.external-source" => Some(("externalDatasetReference.externalSource", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "friendly-name" => Some(("friendlyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "is-case-insensitive" => Some(("isCaseInsensitive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "last-modified-time" => Some(("lastModifiedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "linked-dataset-source.source-dataset.dataset-id" => Some(("linkedDatasetSource.sourceDataset.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "linked-dataset-source.source-dataset.project-id" => Some(("linkedDatasetSource.sourceDataset.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location" => Some(("location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "max-time-travel-hours" => Some(("maxTimeTravelHours", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "satisfies-pzi" => Some(("satisfiesPzi", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "satisfies-pzs" => Some(("satisfiesPzs", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "storage-billing-model" => Some(("storageBillingModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["connection", "creation-time", "dataset-id", "dataset-reference", "default-collation", "default-encryption-configuration", "default-partition-expiration-ms", "default-rounding-mode", "default-table-expiration-ms", "description", "etag", "external-dataset-reference", "external-source", "friendly-name", "id", "is-case-insensitive", "kind", "kms-key-name", "labels", "last-modified-time", "linked-dataset-source", "location", "max-time-travel-hours", "project-id", "satisfies-pzi", "satisfies-pzs", "self-link", "source-dataset", "storage-billing-model", "type"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Dataset = json::value::from_value(object).unwrap();
        let mut call = self.hub.datasets().update(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""));
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

    async fn _jobs_cancel(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.jobs().cancel(opt.value_of("project-id").unwrap_or(""), opt.value_of("job-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "location" => {
                    call = call.location(value.unwrap_or(""));
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
                                                                           v.extend(["location"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _jobs_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.jobs().delete(opt.value_of("project-id").unwrap_or(""), opt.value_of("job-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "location" => {
                    call = call.location(value.unwrap_or(""));
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
                                                                           v.extend(["location"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _jobs_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.jobs().get(opt.value_of("project-id").unwrap_or(""), opt.value_of("job-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "location" => {
                    call = call.location(value.unwrap_or(""));
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
                                                                           v.extend(["location"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _jobs_get_query_results(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.jobs().get_query_results(opt.value_of("project-id").unwrap_or(""), opt.value_of("job-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "timeout-ms" => {
                    call = call.timeout_ms(        value.map(|v| arg_from_str(v, err, "timeout-ms", "uint32")).unwrap_or(0));
                },
                "start-index" => {
                    call = call.start_index(        value.map(|v| arg_from_str(v, err, "start-index", "uint64")).unwrap_or(0));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "uint32")).unwrap_or(0));
                },
                "location" => {
                    call = call.location(value.unwrap_or(""));
                },
                "format-options-use-int64-timestamp" => {
                    call = call.format_options_use_int64_timestamp(        value.map(|v| arg_from_str(v, err, "format-options-use-int64-timestamp", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["format-options-use-int64-timestamp", "location", "max-results", "page-token", "start-index", "timeout-ms"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _jobs_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "configuration.copy.create-disposition" => Some(("configuration.copy.createDisposition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.copy.destination-encryption-configuration.kms-key-name" => Some(("configuration.copy.destinationEncryptionConfiguration.kmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.copy.destination-expiration-time" => Some(("configuration.copy.destinationExpirationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.copy.destination-table.dataset-id" => Some(("configuration.copy.destinationTable.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.copy.destination-table.project-id" => Some(("configuration.copy.destinationTable.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.copy.destination-table.table-id" => Some(("configuration.copy.destinationTable.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.copy.operation-type" => Some(("configuration.copy.operationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.copy.source-table.dataset-id" => Some(("configuration.copy.sourceTable.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.copy.source-table.project-id" => Some(("configuration.copy.sourceTable.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.copy.source-table.table-id" => Some(("configuration.copy.sourceTable.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.copy.write-disposition" => Some(("configuration.copy.writeDisposition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.dry-run" => Some(("configuration.dryRun", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "configuration.extract.compression" => Some(("configuration.extract.compression", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.extract.destination-format" => Some(("configuration.extract.destinationFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.extract.destination-uri" => Some(("configuration.extract.destinationUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.extract.destination-uris" => Some(("configuration.extract.destinationUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "configuration.extract.field-delimiter" => Some(("configuration.extract.fieldDelimiter", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.extract.model-extract-options.trial-id" => Some(("configuration.extract.modelExtractOptions.trialId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.extract.print-header" => Some(("configuration.extract.printHeader", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "configuration.extract.source-model.dataset-id" => Some(("configuration.extract.sourceModel.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.extract.source-model.model-id" => Some(("configuration.extract.sourceModel.modelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.extract.source-model.project-id" => Some(("configuration.extract.sourceModel.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.extract.source-table.dataset-id" => Some(("configuration.extract.sourceTable.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.extract.source-table.project-id" => Some(("configuration.extract.sourceTable.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.extract.source-table.table-id" => Some(("configuration.extract.sourceTable.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.extract.use-avro-logical-types" => Some(("configuration.extract.useAvroLogicalTypes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "configuration.job-timeout-ms" => Some(("configuration.jobTimeoutMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.job-type" => Some(("configuration.jobType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.labels" => Some(("configuration.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "configuration.load.allow-jagged-rows" => Some(("configuration.load.allowJaggedRows", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "configuration.load.allow-quoted-newlines" => Some(("configuration.load.allowQuotedNewlines", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "configuration.load.autodetect" => Some(("configuration.load.autodetect", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "configuration.load.clustering.fields" => Some(("configuration.load.clustering.fields", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "configuration.load.copy-files-only" => Some(("configuration.load.copyFilesOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "configuration.load.create-disposition" => Some(("configuration.load.createDisposition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.load.create-session" => Some(("configuration.load.createSession", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "configuration.load.decimal-target-types" => Some(("configuration.load.decimalTargetTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "configuration.load.destination-encryption-configuration.kms-key-name" => Some(("configuration.load.destinationEncryptionConfiguration.kmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.load.destination-table.dataset-id" => Some(("configuration.load.destinationTable.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.load.destination-table.project-id" => Some(("configuration.load.destinationTable.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.load.destination-table.table-id" => Some(("configuration.load.destinationTable.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.load.destination-table-properties.description" => Some(("configuration.load.destinationTableProperties.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.load.destination-table-properties.expiration-time" => Some(("configuration.load.destinationTableProperties.expirationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.load.destination-table-properties.friendly-name" => Some(("configuration.load.destinationTableProperties.friendlyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.load.destination-table-properties.labels" => Some(("configuration.load.destinationTableProperties.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "configuration.load.encoding" => Some(("configuration.load.encoding", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.load.field-delimiter" => Some(("configuration.load.fieldDelimiter", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.load.file-set-spec-type" => Some(("configuration.load.fileSetSpecType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.load.hive-partitioning-options.fields" => Some(("configuration.load.hivePartitioningOptions.fields", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "configuration.load.hive-partitioning-options.mode" => Some(("configuration.load.hivePartitioningOptions.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.load.hive-partitioning-options.require-partition-filter" => Some(("configuration.load.hivePartitioningOptions.requirePartitionFilter", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "configuration.load.hive-partitioning-options.source-uri-prefix" => Some(("configuration.load.hivePartitioningOptions.sourceUriPrefix", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.load.ignore-unknown-values" => Some(("configuration.load.ignoreUnknownValues", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "configuration.load.json-extension" => Some(("configuration.load.jsonExtension", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.load.max-bad-records" => Some(("configuration.load.maxBadRecords", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "configuration.load.null-marker" => Some(("configuration.load.nullMarker", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.load.parquet-options.enable-list-inference" => Some(("configuration.load.parquetOptions.enableListInference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "configuration.load.parquet-options.enum-as-string" => Some(("configuration.load.parquetOptions.enumAsString", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "configuration.load.preserve-ascii-control-characters" => Some(("configuration.load.preserveAsciiControlCharacters", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "configuration.load.projection-fields" => Some(("configuration.load.projectionFields", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "configuration.load.quote" => Some(("configuration.load.quote", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.load.range-partitioning.field" => Some(("configuration.load.rangePartitioning.field", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.load.range-partitioning.range.end" => Some(("configuration.load.rangePartitioning.range.end", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.load.range-partitioning.range.interval" => Some(("configuration.load.rangePartitioning.range.interval", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.load.range-partitioning.range.start" => Some(("configuration.load.rangePartitioning.range.start", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.load.reference-file-schema-uri" => Some(("configuration.load.referenceFileSchemaUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.load.schema-inline" => Some(("configuration.load.schemaInline", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.load.schema-inline-format" => Some(("configuration.load.schemaInlineFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.load.schema-update-options" => Some(("configuration.load.schemaUpdateOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "configuration.load.skip-leading-rows" => Some(("configuration.load.skipLeadingRows", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "configuration.load.source-format" => Some(("configuration.load.sourceFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.load.source-uris" => Some(("configuration.load.sourceUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "configuration.load.time-partitioning.expiration-ms" => Some(("configuration.load.timePartitioning.expirationMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.load.time-partitioning.field" => Some(("configuration.load.timePartitioning.field", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.load.time-partitioning.require-partition-filter" => Some(("configuration.load.timePartitioning.requirePartitionFilter", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "configuration.load.time-partitioning.type" => Some(("configuration.load.timePartitioning.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.load.use-avro-logical-types" => Some(("configuration.load.useAvroLogicalTypes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "configuration.load.write-disposition" => Some(("configuration.load.writeDisposition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.query.allow-large-results" => Some(("configuration.query.allowLargeResults", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "configuration.query.clustering.fields" => Some(("configuration.query.clustering.fields", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "configuration.query.continuous" => Some(("configuration.query.continuous", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "configuration.query.create-disposition" => Some(("configuration.query.createDisposition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.query.create-session" => Some(("configuration.query.createSession", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "configuration.query.default-dataset.dataset-id" => Some(("configuration.query.defaultDataset.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.query.default-dataset.project-id" => Some(("configuration.query.defaultDataset.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.query.destination-encryption-configuration.kms-key-name" => Some(("configuration.query.destinationEncryptionConfiguration.kmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.query.destination-table.dataset-id" => Some(("configuration.query.destinationTable.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.query.destination-table.project-id" => Some(("configuration.query.destinationTable.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.query.destination-table.table-id" => Some(("configuration.query.destinationTable.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.query.flatten-results" => Some(("configuration.query.flattenResults", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "configuration.query.maximum-billing-tier" => Some(("configuration.query.maximumBillingTier", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "configuration.query.maximum-bytes-billed" => Some(("configuration.query.maximumBytesBilled", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.query.parameter-mode" => Some(("configuration.query.parameterMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.query.preserve-nulls" => Some(("configuration.query.preserveNulls", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "configuration.query.priority" => Some(("configuration.query.priority", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.query.query" => Some(("configuration.query.query", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.query.range-partitioning.field" => Some(("configuration.query.rangePartitioning.field", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.query.range-partitioning.range.end" => Some(("configuration.query.rangePartitioning.range.end", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.query.range-partitioning.range.interval" => Some(("configuration.query.rangePartitioning.range.interval", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.query.range-partitioning.range.start" => Some(("configuration.query.rangePartitioning.range.start", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.query.schema-update-options" => Some(("configuration.query.schemaUpdateOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "configuration.query.script-options.key-result-statement" => Some(("configuration.query.scriptOptions.keyResultStatement", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.query.script-options.statement-byte-budget" => Some(("configuration.query.scriptOptions.statementByteBudget", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.query.script-options.statement-timeout-ms" => Some(("configuration.query.scriptOptions.statementTimeoutMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.query.time-partitioning.expiration-ms" => Some(("configuration.query.timePartitioning.expirationMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.query.time-partitioning.field" => Some(("configuration.query.timePartitioning.field", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.query.time-partitioning.require-partition-filter" => Some(("configuration.query.timePartitioning.requirePartitionFilter", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "configuration.query.time-partitioning.type" => Some(("configuration.query.timePartitioning.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.query.use-legacy-sql" => Some(("configuration.query.useLegacySql", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "configuration.query.use-query-cache" => Some(("configuration.query.useQueryCache", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "configuration.query.write-disposition" => Some(("configuration.query.writeDisposition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-creation-reason.code" => Some(("jobCreationReason.code", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-reference.job-id" => Some(("jobReference.jobId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-reference.location" => Some(("jobReference.location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-reference.project-id" => Some(("jobReference.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "principal-subject" => Some(("principal_subject", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.completion-ratio" => Some(("statistics.completionRatio", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "statistics.copy.copied-logical-bytes" => Some(("statistics.copy.copiedLogicalBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.copy.copied-rows" => Some(("statistics.copy.copiedRows", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.creation-time" => Some(("statistics.creationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.data-masking-statistics.data-masking-applied" => Some(("statistics.dataMaskingStatistics.dataMaskingApplied", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "statistics.end-time" => Some(("statistics.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.extract.destination-uri-file-counts" => Some(("statistics.extract.destinationUriFileCounts", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "statistics.extract.input-bytes" => Some(("statistics.extract.inputBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.final-execution-duration-ms" => Some(("statistics.finalExecutionDurationMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.load.bad-records" => Some(("statistics.load.badRecords", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.load.input-file-bytes" => Some(("statistics.load.inputFileBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.load.input-files" => Some(("statistics.load.inputFiles", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.load.output-bytes" => Some(("statistics.load.outputBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.load.output-rows" => Some(("statistics.load.outputRows", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.num-child-jobs" => Some(("statistics.numChildJobs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.parent-job-id" => Some(("statistics.parentJobId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.bi-engine-statistics.acceleration-mode" => Some(("statistics.query.biEngineStatistics.accelerationMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.bi-engine-statistics.bi-engine-mode" => Some(("statistics.query.biEngineStatistics.biEngineMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.billing-tier" => Some(("statistics.query.billingTier", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "statistics.query.cache-hit" => Some(("statistics.query.cacheHit", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "statistics.query.dcl-target-dataset.dataset-id" => Some(("statistics.query.dclTargetDataset.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.dcl-target-dataset.project-id" => Some(("statistics.query.dclTargetDataset.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.dcl-target-table.dataset-id" => Some(("statistics.query.dclTargetTable.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.dcl-target-table.project-id" => Some(("statistics.query.dclTargetTable.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.dcl-target-table.table-id" => Some(("statistics.query.dclTargetTable.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.dcl-target-view.dataset-id" => Some(("statistics.query.dclTargetView.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.dcl-target-view.project-id" => Some(("statistics.query.dclTargetView.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.dcl-target-view.table-id" => Some(("statistics.query.dclTargetView.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.ddl-affected-row-access-policy-count" => Some(("statistics.query.ddlAffectedRowAccessPolicyCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "statistics.query.ddl-destination-table.dataset-id" => Some(("statistics.query.ddlDestinationTable.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.ddl-destination-table.project-id" => Some(("statistics.query.ddlDestinationTable.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.ddl-destination-table.table-id" => Some(("statistics.query.ddlDestinationTable.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.ddl-operation-performed" => Some(("statistics.query.ddlOperationPerformed", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.ddl-target-dataset.dataset-id" => Some(("statistics.query.ddlTargetDataset.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.ddl-target-dataset.project-id" => Some(("statistics.query.ddlTargetDataset.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.ddl-target-routine.dataset-id" => Some(("statistics.query.ddlTargetRoutine.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.ddl-target-routine.project-id" => Some(("statistics.query.ddlTargetRoutine.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.ddl-target-routine.routine-id" => Some(("statistics.query.ddlTargetRoutine.routineId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.ddl-target-row-access-policy.dataset-id" => Some(("statistics.query.ddlTargetRowAccessPolicy.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.ddl-target-row-access-policy.policy-id" => Some(("statistics.query.ddlTargetRowAccessPolicy.policyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.ddl-target-row-access-policy.project-id" => Some(("statistics.query.ddlTargetRowAccessPolicy.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.ddl-target-row-access-policy.table-id" => Some(("statistics.query.ddlTargetRowAccessPolicy.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.ddl-target-table.dataset-id" => Some(("statistics.query.ddlTargetTable.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.ddl-target-table.project-id" => Some(("statistics.query.ddlTargetTable.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.ddl-target-table.table-id" => Some(("statistics.query.ddlTargetTable.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.dml-stats.deleted-row-count" => Some(("statistics.query.dmlStats.deletedRowCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "statistics.query.dml-stats.inserted-row-count" => Some(("statistics.query.dmlStats.insertedRowCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "statistics.query.dml-stats.updated-row-count" => Some(("statistics.query.dmlStats.updatedRowCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "statistics.query.estimated-bytes-processed" => Some(("statistics.query.estimatedBytesProcessed", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.export-data-statistics.file-count" => Some(("statistics.query.exportDataStatistics.fileCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "statistics.query.export-data-statistics.row-count" => Some(("statistics.query.exportDataStatistics.rowCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "statistics.query.load-query-statistics.bad-records" => Some(("statistics.query.loadQueryStatistics.badRecords", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.load-query-statistics.bytes-transferred" => Some(("statistics.query.loadQueryStatistics.bytesTransferred", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.load-query-statistics.input-file-bytes" => Some(("statistics.query.loadQueryStatistics.inputFileBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.load-query-statistics.input-files" => Some(("statistics.query.loadQueryStatistics.inputFiles", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.load-query-statistics.output-bytes" => Some(("statistics.query.loadQueryStatistics.outputBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.load-query-statistics.output-rows" => Some(("statistics.query.loadQueryStatistics.outputRows", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.ml-statistics.max-iterations" => Some(("statistics.query.mlStatistics.maxIterations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.ml-statistics.model-type" => Some(("statistics.query.mlStatistics.modelType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.ml-statistics.training-type" => Some(("statistics.query.mlStatistics.trainingType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.model-training.current-iteration" => Some(("statistics.query.modelTraining.currentIteration", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "statistics.query.model-training.expected-total-iterations" => Some(("statistics.query.modelTraining.expectedTotalIterations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.model-training-current-iteration" => Some(("statistics.query.modelTrainingCurrentIteration", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "statistics.query.model-training-expected-total-iteration" => Some(("statistics.query.modelTrainingExpectedTotalIteration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.num-dml-affected-rows" => Some(("statistics.query.numDmlAffectedRows", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.performance-insights.avg-previous-execution-ms" => Some(("statistics.query.performanceInsights.avgPreviousExecutionMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.search-statistics.index-usage-mode" => Some(("statistics.query.searchStatistics.indexUsageMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.spark-statistics.endpoints" => Some(("statistics.query.sparkStatistics.endpoints", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "statistics.query.spark-statistics.gcs-staging-bucket" => Some(("statistics.query.sparkStatistics.gcsStagingBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.spark-statistics.kms-key-name" => Some(("statistics.query.sparkStatistics.kmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.spark-statistics.logging-info.project-id" => Some(("statistics.query.sparkStatistics.loggingInfo.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.spark-statistics.logging-info.resource-type" => Some(("statistics.query.sparkStatistics.loggingInfo.resourceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.spark-statistics.spark-job-id" => Some(("statistics.query.sparkStatistics.sparkJobId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.spark-statistics.spark-job-location" => Some(("statistics.query.sparkStatistics.sparkJobLocation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.statement-type" => Some(("statistics.query.statementType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.total-bytes-billed" => Some(("statistics.query.totalBytesBilled", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.total-bytes-processed" => Some(("statistics.query.totalBytesProcessed", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.total-bytes-processed-accuracy" => Some(("statistics.query.totalBytesProcessedAccuracy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.total-partitions-processed" => Some(("statistics.query.totalPartitionsProcessed", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.total-slot-ms" => Some(("statistics.query.totalSlotMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.transferred-bytes" => Some(("statistics.query.transferredBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.vector-search-statistics.index-usage-mode" => Some(("statistics.query.vectorSearchStatistics.indexUsageMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.quota-deferments" => Some(("statistics.quotaDeferments", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "statistics.reservation-id" => Some(("statistics.reservation_id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.row-level-security-statistics.row-level-security-applied" => Some(("statistics.rowLevelSecurityStatistics.rowLevelSecurityApplied", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "statistics.script-statistics.evaluation-kind" => Some(("statistics.scriptStatistics.evaluationKind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.session-info.session-id" => Some(("statistics.sessionInfo.sessionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.start-time" => Some(("statistics.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.total-bytes-processed" => Some(("statistics.totalBytesProcessed", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.total-slot-ms" => Some(("statistics.totalSlotMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.transaction-info.transaction-id" => Some(("statistics.transactionInfo.transactionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.error-result.debug-info" => Some(("status.errorResult.debugInfo", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.error-result.location" => Some(("status.errorResult.location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.error-result.message" => Some(("status.errorResult.message", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.error-result.reason" => Some(("status.errorResult.reason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.state" => Some(("status.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-email" => Some(("user_email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["acceleration-mode", "allow-jagged-rows", "allow-large-results", "allow-quoted-newlines", "autodetect", "avg-previous-execution-ms", "bad-records", "bi-engine-mode", "bi-engine-statistics", "billing-tier", "bytes-transferred", "cache-hit", "clustering", "code", "completion-ratio", "compression", "configuration", "continuous", "copied-logical-bytes", "copied-rows", "copy", "copy-files-only", "create-disposition", "create-session", "creation-time", "current-iteration", "data-masking-applied", "data-masking-statistics", "dataset-id", "dcl-target-dataset", "dcl-target-table", "dcl-target-view", "ddl-affected-row-access-policy-count", "ddl-destination-table", "ddl-operation-performed", "ddl-target-dataset", "ddl-target-routine", "ddl-target-row-access-policy", "ddl-target-table", "debug-info", "decimal-target-types", "default-dataset", "deleted-row-count", "description", "destination-encryption-configuration", "destination-expiration-time", "destination-format", "destination-table", "destination-table-properties", "destination-uri", "destination-uri-file-counts", "destination-uris", "dml-stats", "dry-run", "enable-list-inference", "encoding", "end", "end-time", "endpoints", "enum-as-string", "error-result", "estimated-bytes-processed", "etag", "evaluation-kind", "expected-total-iterations", "expiration-ms", "expiration-time", "export-data-statistics", "extract", "field", "field-delimiter", "fields", "file-count", "file-set-spec-type", "final-execution-duration-ms", "flatten-results", "friendly-name", "gcs-staging-bucket", "hive-partitioning-options", "id", "ignore-unknown-values", "index-usage-mode", "input-bytes", "input-file-bytes", "input-files", "inserted-row-count", "interval", "job-creation-reason", "job-id", "job-reference", "job-timeout-ms", "job-type", "json-extension", "key-result-statement", "kind", "kms-key-name", "labels", "load", "load-query-statistics", "location", "logging-info", "max-bad-records", "max-iterations", "maximum-billing-tier", "maximum-bytes-billed", "message", "ml-statistics", "mode", "model-extract-options", "model-id", "model-training", "model-training-current-iteration", "model-training-expected-total-iteration", "model-type", "null-marker", "num-child-jobs", "num-dml-affected-rows", "operation-type", "output-bytes", "output-rows", "parameter-mode", "parent-job-id", "parquet-options", "performance-insights", "policy-id", "preserve-ascii-control-characters", "preserve-nulls", "principal-subject", "print-header", "priority", "project-id", "projection-fields", "query", "quota-deferments", "quote", "range", "range-partitioning", "reason", "reference-file-schema-uri", "require-partition-filter", "reservation-id", "resource-type", "routine-id", "row-count", "row-level-security-applied", "row-level-security-statistics", "schema-inline", "schema-inline-format", "schema-update-options", "script-options", "script-statistics", "search-statistics", "self-link", "session-id", "session-info", "skip-leading-rows", "source-format", "source-model", "source-table", "source-uri-prefix", "source-uris", "spark-job-id", "spark-job-location", "spark-statistics", "start", "start-time", "state", "statement-byte-budget", "statement-timeout-ms", "statement-type", "statistics", "status", "table-id", "time-partitioning", "total-bytes-billed", "total-bytes-processed", "total-bytes-processed-accuracy", "total-partitions-processed", "total-slot-ms", "training-type", "transaction-id", "transaction-info", "transferred-bytes", "trial-id", "type", "updated-row-count", "use-avro-logical-types", "use-legacy-sql", "use-query-cache", "user-email", "vector-search-statistics", "write-disposition"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Job = json::value::from_value(object).unwrap();
        let mut call = self.hub.jobs().insert(request, opt.value_of("project-id").unwrap_or(""));
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
        let vals = opt.values_of("mode").unwrap().collect::<Vec<&str>>();
        let protocol = calltype_from_str(vals[0], ["simple"].iter().map(|&v| v.to_string()).collect(), err);
        let mut input_file = input_file_from_opts(vals[1], err);
        let mime_type = input_mime_from_opts(opt.value_of("mime").unwrap_or("application/octet-stream"), err);
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Upload(UploadProtocol::Simple) => call.upload(input_file.unwrap(), mime_type.unwrap()).await,
                CallType::Standard => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _jobs_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.jobs().list(opt.value_of("project-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "state-filter" => {
                    call = call.add_state_filter(value.unwrap_or(""));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "parent-job-id" => {
                    call = call.parent_job_id(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "min-creation-time" => {
                    call = call.min_creation_time(        value.map(|v| arg_from_str(v, err, "min-creation-time", "uint64")).unwrap_or(0));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "uint32")).unwrap_or(0));
                },
                "max-creation-time" => {
                    call = call.max_creation_time(        value.map(|v| arg_from_str(v, err, "max-creation-time", "uint64")).unwrap_or(0));
                },
                "all-users" => {
                    call = call.all_users(        value.map(|v| arg_from_str(v, err, "all-users", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["all-users", "max-creation-time", "max-results", "min-creation-time", "page-token", "parent-job-id", "projection", "state-filter"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _jobs_query(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "continuous" => Some(("continuous", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "create-session" => Some(("createSession", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "default-dataset.dataset-id" => Some(("defaultDataset.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-dataset.project-id" => Some(("defaultDataset.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dry-run" => Some(("dryRun", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "format-options.use-int64-timestamp" => Some(("formatOptions.useInt64Timestamp", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job-creation-mode" => Some(("jobCreationMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "location" => Some(("location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "max-results" => Some(("maxResults", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "maximum-bytes-billed" => Some(("maximumBytesBilled", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parameter-mode" => Some(("parameterMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preserve-nulls" => Some(("preserveNulls", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "query" => Some(("query", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-id" => Some(("requestId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "timeout-ms" => Some(("timeoutMs", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "use-legacy-sql" => Some(("useLegacySql", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "use-query-cache" => Some(("useQueryCache", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["continuous", "create-session", "dataset-id", "default-dataset", "dry-run", "format-options", "job-creation-mode", "kind", "labels", "location", "max-results", "maximum-bytes-billed", "parameter-mode", "preserve-nulls", "project-id", "query", "request-id", "timeout-ms", "use-int64-timestamp", "use-legacy-sql", "use-query-cache"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::QueryRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.jobs().query(request, opt.value_of("project-id").unwrap_or(""));
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

    async fn _models_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.models().delete(opt.value_of("project-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""), opt.value_of("model-id").unwrap_or(""));
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
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _models_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.models().get(opt.value_of("project-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""), opt.value_of("model-id").unwrap_or(""));
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

    async fn _models_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.models().list(opt.value_of("project-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "uint32")).unwrap_or(0));
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
                                                                           v.extend(["max-results", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _models_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "best-trial-id" => Some(("bestTrialId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creation-time" => Some(("creationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-trial-id" => Some(("defaultTrialId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "encryption-configuration.kms-key-name" => Some(("encryptionConfiguration.kmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "expiration-time" => Some(("expirationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "friendly-name" => Some(("friendlyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hparam-search-spaces.activation-fn.candidates" => Some(("hparamSearchSpaces.activationFn.candidates", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "hparam-search-spaces.batch-size.candidates.candidates" => Some(("hparamSearchSpaces.batchSize.candidates.candidates", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "hparam-search-spaces.batch-size.range.max" => Some(("hparamSearchSpaces.batchSize.range.max", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hparam-search-spaces.batch-size.range.min" => Some(("hparamSearchSpaces.batchSize.range.min", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hparam-search-spaces.booster-type.candidates" => Some(("hparamSearchSpaces.boosterType.candidates", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "hparam-search-spaces.colsample-bylevel.candidates.candidates" => Some(("hparamSearchSpaces.colsampleBylevel.candidates.candidates", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Vec })),
                    "hparam-search-spaces.colsample-bylevel.range.max" => Some(("hparamSearchSpaces.colsampleBylevel.range.max", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "hparam-search-spaces.colsample-bylevel.range.min" => Some(("hparamSearchSpaces.colsampleBylevel.range.min", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "hparam-search-spaces.colsample-bynode.candidates.candidates" => Some(("hparamSearchSpaces.colsampleBynode.candidates.candidates", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Vec })),
                    "hparam-search-spaces.colsample-bynode.range.max" => Some(("hparamSearchSpaces.colsampleBynode.range.max", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "hparam-search-spaces.colsample-bynode.range.min" => Some(("hparamSearchSpaces.colsampleBynode.range.min", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "hparam-search-spaces.colsample-bytree.candidates.candidates" => Some(("hparamSearchSpaces.colsampleBytree.candidates.candidates", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Vec })),
                    "hparam-search-spaces.colsample-bytree.range.max" => Some(("hparamSearchSpaces.colsampleBytree.range.max", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "hparam-search-spaces.colsample-bytree.range.min" => Some(("hparamSearchSpaces.colsampleBytree.range.min", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "hparam-search-spaces.dart-normalize-type.candidates" => Some(("hparamSearchSpaces.dartNormalizeType.candidates", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "hparam-search-spaces.dropout.candidates.candidates" => Some(("hparamSearchSpaces.dropout.candidates.candidates", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Vec })),
                    "hparam-search-spaces.dropout.range.max" => Some(("hparamSearchSpaces.dropout.range.max", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "hparam-search-spaces.dropout.range.min" => Some(("hparamSearchSpaces.dropout.range.min", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "hparam-search-spaces.l1-reg.candidates.candidates" => Some(("hparamSearchSpaces.l1Reg.candidates.candidates", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Vec })),
                    "hparam-search-spaces.l1-reg.range.max" => Some(("hparamSearchSpaces.l1Reg.range.max", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "hparam-search-spaces.l1-reg.range.min" => Some(("hparamSearchSpaces.l1Reg.range.min", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "hparam-search-spaces.l2-reg.candidates.candidates" => Some(("hparamSearchSpaces.l2Reg.candidates.candidates", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Vec })),
                    "hparam-search-spaces.l2-reg.range.max" => Some(("hparamSearchSpaces.l2Reg.range.max", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "hparam-search-spaces.l2-reg.range.min" => Some(("hparamSearchSpaces.l2Reg.range.min", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "hparam-search-spaces.learn-rate.candidates.candidates" => Some(("hparamSearchSpaces.learnRate.candidates.candidates", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Vec })),
                    "hparam-search-spaces.learn-rate.range.max" => Some(("hparamSearchSpaces.learnRate.range.max", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "hparam-search-spaces.learn-rate.range.min" => Some(("hparamSearchSpaces.learnRate.range.min", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "hparam-search-spaces.max-tree-depth.candidates.candidates" => Some(("hparamSearchSpaces.maxTreeDepth.candidates.candidates", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "hparam-search-spaces.max-tree-depth.range.max" => Some(("hparamSearchSpaces.maxTreeDepth.range.max", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hparam-search-spaces.max-tree-depth.range.min" => Some(("hparamSearchSpaces.maxTreeDepth.range.min", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hparam-search-spaces.min-split-loss.candidates.candidates" => Some(("hparamSearchSpaces.minSplitLoss.candidates.candidates", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Vec })),
                    "hparam-search-spaces.min-split-loss.range.max" => Some(("hparamSearchSpaces.minSplitLoss.range.max", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "hparam-search-spaces.min-split-loss.range.min" => Some(("hparamSearchSpaces.minSplitLoss.range.min", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "hparam-search-spaces.min-tree-child-weight.candidates.candidates" => Some(("hparamSearchSpaces.minTreeChildWeight.candidates.candidates", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "hparam-search-spaces.min-tree-child-weight.range.max" => Some(("hparamSearchSpaces.minTreeChildWeight.range.max", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hparam-search-spaces.min-tree-child-weight.range.min" => Some(("hparamSearchSpaces.minTreeChildWeight.range.min", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hparam-search-spaces.num-clusters.candidates.candidates" => Some(("hparamSearchSpaces.numClusters.candidates.candidates", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "hparam-search-spaces.num-clusters.range.max" => Some(("hparamSearchSpaces.numClusters.range.max", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hparam-search-spaces.num-clusters.range.min" => Some(("hparamSearchSpaces.numClusters.range.min", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hparam-search-spaces.num-factors.candidates.candidates" => Some(("hparamSearchSpaces.numFactors.candidates.candidates", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "hparam-search-spaces.num-factors.range.max" => Some(("hparamSearchSpaces.numFactors.range.max", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hparam-search-spaces.num-factors.range.min" => Some(("hparamSearchSpaces.numFactors.range.min", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hparam-search-spaces.num-parallel-tree.candidates.candidates" => Some(("hparamSearchSpaces.numParallelTree.candidates.candidates", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "hparam-search-spaces.num-parallel-tree.range.max" => Some(("hparamSearchSpaces.numParallelTree.range.max", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hparam-search-spaces.num-parallel-tree.range.min" => Some(("hparamSearchSpaces.numParallelTree.range.min", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hparam-search-spaces.optimizer.candidates" => Some(("hparamSearchSpaces.optimizer.candidates", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "hparam-search-spaces.subsample.candidates.candidates" => Some(("hparamSearchSpaces.subsample.candidates.candidates", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Vec })),
                    "hparam-search-spaces.subsample.range.max" => Some(("hparamSearchSpaces.subsample.range.max", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "hparam-search-spaces.subsample.range.min" => Some(("hparamSearchSpaces.subsample.range.min", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "hparam-search-spaces.tree-method.candidates" => Some(("hparamSearchSpaces.treeMethod.candidates", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "hparam-search-spaces.wals-alpha.candidates.candidates" => Some(("hparamSearchSpaces.walsAlpha.candidates.candidates", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Vec })),
                    "hparam-search-spaces.wals-alpha.range.max" => Some(("hparamSearchSpaces.walsAlpha.range.max", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "hparam-search-spaces.wals-alpha.range.min" => Some(("hparamSearchSpaces.walsAlpha.range.min", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "last-modified-time" => Some(("lastModifiedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location" => Some(("location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "model-reference.dataset-id" => Some(("modelReference.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "model-reference.model-id" => Some(("modelReference.modelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "model-reference.project-id" => Some(("modelReference.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "model-type" => Some(("modelType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "optimal-trial-ids" => Some(("optimalTrialIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "remote-model-info.connection" => Some(("remoteModelInfo.connection", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "remote-model-info.endpoint" => Some(("remoteModelInfo.endpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "remote-model-info.max-batching-rows" => Some(("remoteModelInfo.maxBatchingRows", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "remote-model-info.remote-model-version" => Some(("remoteModelInfo.remoteModelVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "remote-model-info.remote-service-type" => Some(("remoteModelInfo.remoteServiceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "remote-model-info.speech-recognizer" => Some(("remoteModelInfo.speechRecognizer", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["activation-fn", "batch-size", "best-trial-id", "booster-type", "candidates", "colsample-bylevel", "colsample-bynode", "colsample-bytree", "connection", "creation-time", "dart-normalize-type", "dataset-id", "default-trial-id", "description", "dropout", "encryption-configuration", "endpoint", "etag", "expiration-time", "friendly-name", "hparam-search-spaces", "kms-key-name", "l1-reg", "l2-reg", "labels", "last-modified-time", "learn-rate", "location", "max", "max-batching-rows", "max-tree-depth", "min", "min-split-loss", "min-tree-child-weight", "model-id", "model-reference", "model-type", "num-clusters", "num-factors", "num-parallel-tree", "optimal-trial-ids", "optimizer", "project-id", "range", "remote-model-info", "remote-model-version", "remote-service-type", "speech-recognizer", "subsample", "tree-method", "wals-alpha"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Model = json::value::from_value(object).unwrap();
        let mut call = self.hub.models().patch(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""), opt.value_of("model-id").unwrap_or(""));
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

    async fn _projects_get_service_account(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().get_service_account(opt.value_of("project-id").unwrap_or(""));
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

    async fn _projects_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "uint32")).unwrap_or(0));
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
                                                                           v.extend(["max-results", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _routines_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.routines().delete(opt.value_of("project-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""), opt.value_of("routine-id").unwrap_or(""));
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
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _routines_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.routines().get(opt.value_of("project-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""), opt.value_of("routine-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "read-mask" => {
                    call = call.read_mask(        value.map(|v| arg_from_str(v, err, "read-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
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
                                                                           v.extend(["read-mask"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _routines_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "data-governance-type" => Some(("dataGovernanceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "definition-body" => Some(("definitionBody", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "determinism-level" => Some(("determinismLevel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "imported-libraries" => Some(("importedLibraries", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "language" => Some(("language", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modified-time" => Some(("lastModifiedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "remote-function-options.connection" => Some(("remoteFunctionOptions.connection", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "remote-function-options.endpoint" => Some(("remoteFunctionOptions.endpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "remote-function-options.max-batching-rows" => Some(("remoteFunctionOptions.maxBatchingRows", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "remote-function-options.user-defined-context" => Some(("remoteFunctionOptions.userDefinedContext", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "return-type.type-kind" => Some(("returnType.typeKind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "routine-reference.dataset-id" => Some(("routineReference.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "routine-reference.project-id" => Some(("routineReference.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "routine-reference.routine-id" => Some(("routineReference.routineId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "routine-type" => Some(("routineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "security-mode" => Some(("securityMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spark-options.archive-uris" => Some(("sparkOptions.archiveUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spark-options.connection" => Some(("sparkOptions.connection", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spark-options.container-image" => Some(("sparkOptions.containerImage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spark-options.file-uris" => Some(("sparkOptions.fileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spark-options.jar-uris" => Some(("sparkOptions.jarUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spark-options.main-class" => Some(("sparkOptions.mainClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spark-options.main-file-uri" => Some(("sparkOptions.mainFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spark-options.properties" => Some(("sparkOptions.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "spark-options.py-file-uris" => Some(("sparkOptions.pyFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spark-options.runtime-version" => Some(("sparkOptions.runtimeVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "strict-mode" => Some(("strictMode", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["archive-uris", "connection", "container-image", "creation-time", "data-governance-type", "dataset-id", "definition-body", "description", "determinism-level", "endpoint", "etag", "file-uris", "imported-libraries", "jar-uris", "language", "last-modified-time", "main-class", "main-file-uri", "max-batching-rows", "project-id", "properties", "py-file-uris", "remote-function-options", "return-type", "routine-id", "routine-reference", "routine-type", "runtime-version", "security-mode", "spark-options", "strict-mode", "type-kind", "user-defined-context"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Routine = json::value::from_value(object).unwrap();
        let mut call = self.hub.routines().insert(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""));
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

    async fn _routines_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.routines().list(opt.value_of("project-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "read-mask" => {
                    call = call.read_mask(        value.map(|v| arg_from_str(v, err, "read-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "uint32")).unwrap_or(0));
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
                                                                           v.extend(["filter", "max-results", "page-token", "read-mask"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _routines_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "data-governance-type" => Some(("dataGovernanceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "definition-body" => Some(("definitionBody", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "determinism-level" => Some(("determinismLevel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "imported-libraries" => Some(("importedLibraries", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "language" => Some(("language", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modified-time" => Some(("lastModifiedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "remote-function-options.connection" => Some(("remoteFunctionOptions.connection", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "remote-function-options.endpoint" => Some(("remoteFunctionOptions.endpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "remote-function-options.max-batching-rows" => Some(("remoteFunctionOptions.maxBatchingRows", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "remote-function-options.user-defined-context" => Some(("remoteFunctionOptions.userDefinedContext", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "return-type.type-kind" => Some(("returnType.typeKind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "routine-reference.dataset-id" => Some(("routineReference.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "routine-reference.project-id" => Some(("routineReference.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "routine-reference.routine-id" => Some(("routineReference.routineId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "routine-type" => Some(("routineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "security-mode" => Some(("securityMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spark-options.archive-uris" => Some(("sparkOptions.archiveUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spark-options.connection" => Some(("sparkOptions.connection", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spark-options.container-image" => Some(("sparkOptions.containerImage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spark-options.file-uris" => Some(("sparkOptions.fileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spark-options.jar-uris" => Some(("sparkOptions.jarUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spark-options.main-class" => Some(("sparkOptions.mainClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spark-options.main-file-uri" => Some(("sparkOptions.mainFileUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "spark-options.properties" => Some(("sparkOptions.properties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "spark-options.py-file-uris" => Some(("sparkOptions.pyFileUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "spark-options.runtime-version" => Some(("sparkOptions.runtimeVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "strict-mode" => Some(("strictMode", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["archive-uris", "connection", "container-image", "creation-time", "data-governance-type", "dataset-id", "definition-body", "description", "determinism-level", "endpoint", "etag", "file-uris", "imported-libraries", "jar-uris", "language", "last-modified-time", "main-class", "main-file-uri", "max-batching-rows", "project-id", "properties", "py-file-uris", "remote-function-options", "return-type", "routine-id", "routine-reference", "routine-type", "runtime-version", "security-mode", "spark-options", "strict-mode", "type-kind", "user-defined-context"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Routine = json::value::from_value(object).unwrap();
        let mut call = self.hub.routines().update(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""), opt.value_of("routine-id").unwrap_or(""));
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

    async fn _row_access_policies_get_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.row_access_policies().get_iam_policy(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _row_access_policies_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.row_access_policies().list(opt.value_of("project-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""), opt.value_of("table-id").unwrap_or(""));
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

    async fn _row_access_policies_test_iam_permissions(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.row_access_policies().test_iam_permissions(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _tabledata_insert_all(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "ignore-unknown-values" => Some(("ignoreUnknownValues", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "skip-invalid-rows" => Some(("skipInvalidRows", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "template-suffix" => Some(("templateSuffix", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trace-id" => Some(("traceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["ignore-unknown-values", "kind", "skip-invalid-rows", "template-suffix", "trace-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::TableDataInsertAllRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.tabledata().insert_all(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""), opt.value_of("table-id").unwrap_or(""));
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

    async fn _tabledata_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.tabledata().list(opt.value_of("project-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""), opt.value_of("table-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(        value.map(|v| arg_from_str(v, err, "start-index", "uint64")).unwrap_or(0));
                },
                "selected-fields" => {
                    call = call.selected_fields(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "uint32")).unwrap_or(0));
                },
                "format-options-use-int64-timestamp" => {
                    call = call.format_options_use_int64_timestamp(        value.map(|v| arg_from_str(v, err, "format-options-use-int64-timestamp", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["format-options-use-int64-timestamp", "max-results", "page-token", "selected-fields", "start-index"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _tables_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.tables().delete(opt.value_of("project-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""), opt.value_of("table-id").unwrap_or(""));
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
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _tables_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.tables().get(opt.value_of("project-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""), opt.value_of("table-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "view" => {
                    call = call.view(value.unwrap_or(""));
                },
                "selected-fields" => {
                    call = call.selected_fields(value.unwrap_or(""));
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
                                                                           v.extend(["selected-fields", "view"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _tables_get_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.tables().get_iam_policy(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _tables_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "biglake-configuration.connection-id" => Some(("biglakeConfiguration.connectionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "biglake-configuration.file-format" => Some(("biglakeConfiguration.fileFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "biglake-configuration.storage-uri" => Some(("biglakeConfiguration.storageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "biglake-configuration.table-format" => Some(("biglakeConfiguration.tableFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "clone-definition.base-table-reference.dataset-id" => Some(("cloneDefinition.baseTableReference.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "clone-definition.base-table-reference.project-id" => Some(("cloneDefinition.baseTableReference.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "clone-definition.base-table-reference.table-id" => Some(("cloneDefinition.baseTableReference.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "clone-definition.clone-time" => Some(("cloneDefinition.cloneTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "clustering.fields" => Some(("clustering.fields", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "creation-time" => Some(("creationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-collation" => Some(("defaultCollation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-rounding-mode" => Some(("defaultRoundingMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "encryption-configuration.kms-key-name" => Some(("encryptionConfiguration.kmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "expiration-time" => Some(("expirationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.autodetect" => Some(("externalDataConfiguration.autodetect", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.avro-options.use-avro-logical-types" => Some(("externalDataConfiguration.avroOptions.useAvroLogicalTypes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.bigtable-options.ignore-unspecified-column-families" => Some(("externalDataConfiguration.bigtableOptions.ignoreUnspecifiedColumnFamilies", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.bigtable-options.output-column-families-as-json" => Some(("externalDataConfiguration.bigtableOptions.outputColumnFamiliesAsJson", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.bigtable-options.read-rowkey-as-string" => Some(("externalDataConfiguration.bigtableOptions.readRowkeyAsString", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.compression" => Some(("externalDataConfiguration.compression", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.connection-id" => Some(("externalDataConfiguration.connectionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.allow-jagged-rows" => Some(("externalDataConfiguration.csvOptions.allowJaggedRows", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.allow-quoted-newlines" => Some(("externalDataConfiguration.csvOptions.allowQuotedNewlines", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.encoding" => Some(("externalDataConfiguration.csvOptions.encoding", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.field-delimiter" => Some(("externalDataConfiguration.csvOptions.fieldDelimiter", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.null-marker" => Some(("externalDataConfiguration.csvOptions.nullMarker", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.preserve-ascii-control-characters" => Some(("externalDataConfiguration.csvOptions.preserveAsciiControlCharacters", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.quote" => Some(("externalDataConfiguration.csvOptions.quote", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.skip-leading-rows" => Some(("externalDataConfiguration.csvOptions.skipLeadingRows", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.decimal-target-types" => Some(("externalDataConfiguration.decimalTargetTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "external-data-configuration.file-set-spec-type" => Some(("externalDataConfiguration.fileSetSpecType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.google-sheets-options.range" => Some(("externalDataConfiguration.googleSheetsOptions.range", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.google-sheets-options.skip-leading-rows" => Some(("externalDataConfiguration.googleSheetsOptions.skipLeadingRows", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.hive-partitioning-options.fields" => Some(("externalDataConfiguration.hivePartitioningOptions.fields", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "external-data-configuration.hive-partitioning-options.mode" => Some(("externalDataConfiguration.hivePartitioningOptions.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.hive-partitioning-options.require-partition-filter" => Some(("externalDataConfiguration.hivePartitioningOptions.requirePartitionFilter", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.hive-partitioning-options.source-uri-prefix" => Some(("externalDataConfiguration.hivePartitioningOptions.sourceUriPrefix", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.ignore-unknown-values" => Some(("externalDataConfiguration.ignoreUnknownValues", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.json-extension" => Some(("externalDataConfiguration.jsonExtension", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.json-options.encoding" => Some(("externalDataConfiguration.jsonOptions.encoding", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.max-bad-records" => Some(("externalDataConfiguration.maxBadRecords", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "external-data-configuration.metadata-cache-mode" => Some(("externalDataConfiguration.metadataCacheMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.object-metadata" => Some(("externalDataConfiguration.objectMetadata", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.parquet-options.enable-list-inference" => Some(("externalDataConfiguration.parquetOptions.enableListInference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.parquet-options.enum-as-string" => Some(("externalDataConfiguration.parquetOptions.enumAsString", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.reference-file-schema-uri" => Some(("externalDataConfiguration.referenceFileSchemaUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.source-format" => Some(("externalDataConfiguration.sourceFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.source-uris" => Some(("externalDataConfiguration.sourceUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "friendly-name" => Some(("friendlyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "last-modified-time" => Some(("lastModifiedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location" => Some(("location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "materialized-view.allow-non-incremental-definition" => Some(("materializedView.allowNonIncrementalDefinition", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "materialized-view.enable-refresh" => Some(("materializedView.enableRefresh", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "materialized-view.last-refresh-time" => Some(("materializedView.lastRefreshTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "materialized-view.max-staleness" => Some(("materializedView.maxStaleness", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "materialized-view.query" => Some(("materializedView.query", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "materialized-view.refresh-interval-ms" => Some(("materializedView.refreshIntervalMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "materialized-view-status.last-refresh-status.debug-info" => Some(("materializedViewStatus.lastRefreshStatus.debugInfo", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "materialized-view-status.last-refresh-status.location" => Some(("materializedViewStatus.lastRefreshStatus.location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "materialized-view-status.last-refresh-status.message" => Some(("materializedViewStatus.lastRefreshStatus.message", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "materialized-view-status.last-refresh-status.reason" => Some(("materializedViewStatus.lastRefreshStatus.reason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "materialized-view-status.refresh-watermark" => Some(("materializedViewStatus.refreshWatermark", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "max-staleness" => Some(("maxStaleness", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "model.model-options.labels" => Some(("model.modelOptions.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "model.model-options.loss-type" => Some(("model.modelOptions.lossType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "model.model-options.model-type" => Some(("model.modelOptions.modelType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-active-logical-bytes" => Some(("numActiveLogicalBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-active-physical-bytes" => Some(("numActivePhysicalBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-bytes" => Some(("numBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-long-term-bytes" => Some(("numLongTermBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-long-term-logical-bytes" => Some(("numLongTermLogicalBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-long-term-physical-bytes" => Some(("numLongTermPhysicalBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-partitions" => Some(("numPartitions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-physical-bytes" => Some(("numPhysicalBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-rows" => Some(("numRows", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-time-travel-physical-bytes" => Some(("numTimeTravelPhysicalBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-total-logical-bytes" => Some(("numTotalLogicalBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-total-physical-bytes" => Some(("numTotalPhysicalBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "range-partitioning.field" => Some(("rangePartitioning.field", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "range-partitioning.range.end" => Some(("rangePartitioning.range.end", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "range-partitioning.range.interval" => Some(("rangePartitioning.range.interval", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "range-partitioning.range.start" => Some(("rangePartitioning.range.start", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "require-partition-filter" => Some(("requirePartitionFilter", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "resource-tags" => Some(("resourceTags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snapshot-definition.base-table-reference.dataset-id" => Some(("snapshotDefinition.baseTableReference.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snapshot-definition.base-table-reference.project-id" => Some(("snapshotDefinition.baseTableReference.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snapshot-definition.base-table-reference.table-id" => Some(("snapshotDefinition.baseTableReference.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snapshot-definition.snapshot-time" => Some(("snapshotDefinition.snapshotTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "streaming-buffer.estimated-bytes" => Some(("streamingBuffer.estimatedBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "streaming-buffer.estimated-rows" => Some(("streamingBuffer.estimatedRows", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "streaming-buffer.oldest-entry-time" => Some(("streamingBuffer.oldestEntryTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-constraints.primary-key.columns" => Some(("tableConstraints.primaryKey.columns", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "table-reference.dataset-id" => Some(("tableReference.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-reference.project-id" => Some(("tableReference.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-reference.table-id" => Some(("tableReference.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-replication-info.replicated-source-last-refresh-time" => Some(("tableReplicationInfo.replicatedSourceLastRefreshTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-replication-info.replication-error.debug-info" => Some(("tableReplicationInfo.replicationError.debugInfo", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-replication-info.replication-error.location" => Some(("tableReplicationInfo.replicationError.location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-replication-info.replication-error.message" => Some(("tableReplicationInfo.replicationError.message", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-replication-info.replication-error.reason" => Some(("tableReplicationInfo.replicationError.reason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-replication-info.replication-interval-ms" => Some(("tableReplicationInfo.replicationIntervalMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-replication-info.replication-status" => Some(("tableReplicationInfo.replicationStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-replication-info.source-table.dataset-id" => Some(("tableReplicationInfo.sourceTable.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-replication-info.source-table.project-id" => Some(("tableReplicationInfo.sourceTable.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-replication-info.source-table.table-id" => Some(("tableReplicationInfo.sourceTable.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-partitioning.expiration-ms" => Some(("timePartitioning.expirationMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-partitioning.field" => Some(("timePartitioning.field", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-partitioning.require-partition-filter" => Some(("timePartitioning.requirePartitionFilter", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "time-partitioning.type" => Some(("timePartitioning.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "view.privacy-policy.aggregation-threshold-policy.privacy-unit-columns" => Some(("view.privacyPolicy.aggregationThresholdPolicy.privacyUnitColumns", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "view.privacy-policy.aggregation-threshold-policy.threshold" => Some(("view.privacyPolicy.aggregationThresholdPolicy.threshold", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "view.query" => Some(("view.query", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "view.use-explicit-column-names" => Some(("view.useExplicitColumnNames", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "view.use-legacy-sql" => Some(("view.useLegacySql", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["aggregation-threshold-policy", "allow-jagged-rows", "allow-non-incremental-definition", "allow-quoted-newlines", "autodetect", "avro-options", "base-table-reference", "biglake-configuration", "bigtable-options", "clone-definition", "clone-time", "clustering", "columns", "compression", "connection-id", "creation-time", "csv-options", "dataset-id", "debug-info", "decimal-target-types", "default-collation", "default-rounding-mode", "description", "enable-list-inference", "enable-refresh", "encoding", "encryption-configuration", "end", "enum-as-string", "estimated-bytes", "estimated-rows", "etag", "expiration-ms", "expiration-time", "external-data-configuration", "field", "field-delimiter", "fields", "file-format", "file-set-spec-type", "friendly-name", "google-sheets-options", "hive-partitioning-options", "id", "ignore-unknown-values", "ignore-unspecified-column-families", "interval", "json-extension", "json-options", "kind", "kms-key-name", "labels", "last-modified-time", "last-refresh-status", "last-refresh-time", "location", "loss-type", "materialized-view", "materialized-view-status", "max-bad-records", "max-staleness", "message", "metadata-cache-mode", "mode", "model", "model-options", "model-type", "null-marker", "num-active-logical-bytes", "num-active-physical-bytes", "num-bytes", "num-long-term-bytes", "num-long-term-logical-bytes", "num-long-term-physical-bytes", "num-partitions", "num-physical-bytes", "num-rows", "num-time-travel-physical-bytes", "num-total-logical-bytes", "num-total-physical-bytes", "object-metadata", "oldest-entry-time", "output-column-families-as-json", "parquet-options", "preserve-ascii-control-characters", "primary-key", "privacy-policy", "privacy-unit-columns", "project-id", "query", "quote", "range", "range-partitioning", "read-rowkey-as-string", "reason", "reference-file-schema-uri", "refresh-interval-ms", "refresh-watermark", "replicated-source-last-refresh-time", "replication-error", "replication-interval-ms", "replication-status", "require-partition-filter", "resource-tags", "self-link", "skip-leading-rows", "snapshot-definition", "snapshot-time", "source-format", "source-table", "source-uri-prefix", "source-uris", "start", "storage-uri", "streaming-buffer", "table-constraints", "table-format", "table-id", "table-reference", "table-replication-info", "threshold", "time-partitioning", "type", "use-avro-logical-types", "use-explicit-column-names", "use-legacy-sql", "view"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Table = json::value::from_value(object).unwrap();
        let mut call = self.hub.tables().insert(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""));
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

    async fn _tables_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.tables().list(opt.value_of("project-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "uint32")).unwrap_or(0));
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
                                                                           v.extend(["max-results", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _tables_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "biglake-configuration.connection-id" => Some(("biglakeConfiguration.connectionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "biglake-configuration.file-format" => Some(("biglakeConfiguration.fileFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "biglake-configuration.storage-uri" => Some(("biglakeConfiguration.storageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "biglake-configuration.table-format" => Some(("biglakeConfiguration.tableFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "clone-definition.base-table-reference.dataset-id" => Some(("cloneDefinition.baseTableReference.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "clone-definition.base-table-reference.project-id" => Some(("cloneDefinition.baseTableReference.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "clone-definition.base-table-reference.table-id" => Some(("cloneDefinition.baseTableReference.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "clone-definition.clone-time" => Some(("cloneDefinition.cloneTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "clustering.fields" => Some(("clustering.fields", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "creation-time" => Some(("creationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-collation" => Some(("defaultCollation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-rounding-mode" => Some(("defaultRoundingMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "encryption-configuration.kms-key-name" => Some(("encryptionConfiguration.kmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "expiration-time" => Some(("expirationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.autodetect" => Some(("externalDataConfiguration.autodetect", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.avro-options.use-avro-logical-types" => Some(("externalDataConfiguration.avroOptions.useAvroLogicalTypes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.bigtable-options.ignore-unspecified-column-families" => Some(("externalDataConfiguration.bigtableOptions.ignoreUnspecifiedColumnFamilies", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.bigtable-options.output-column-families-as-json" => Some(("externalDataConfiguration.bigtableOptions.outputColumnFamiliesAsJson", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.bigtable-options.read-rowkey-as-string" => Some(("externalDataConfiguration.bigtableOptions.readRowkeyAsString", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.compression" => Some(("externalDataConfiguration.compression", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.connection-id" => Some(("externalDataConfiguration.connectionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.allow-jagged-rows" => Some(("externalDataConfiguration.csvOptions.allowJaggedRows", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.allow-quoted-newlines" => Some(("externalDataConfiguration.csvOptions.allowQuotedNewlines", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.encoding" => Some(("externalDataConfiguration.csvOptions.encoding", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.field-delimiter" => Some(("externalDataConfiguration.csvOptions.fieldDelimiter", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.null-marker" => Some(("externalDataConfiguration.csvOptions.nullMarker", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.preserve-ascii-control-characters" => Some(("externalDataConfiguration.csvOptions.preserveAsciiControlCharacters", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.quote" => Some(("externalDataConfiguration.csvOptions.quote", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.skip-leading-rows" => Some(("externalDataConfiguration.csvOptions.skipLeadingRows", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.decimal-target-types" => Some(("externalDataConfiguration.decimalTargetTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "external-data-configuration.file-set-spec-type" => Some(("externalDataConfiguration.fileSetSpecType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.google-sheets-options.range" => Some(("externalDataConfiguration.googleSheetsOptions.range", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.google-sheets-options.skip-leading-rows" => Some(("externalDataConfiguration.googleSheetsOptions.skipLeadingRows", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.hive-partitioning-options.fields" => Some(("externalDataConfiguration.hivePartitioningOptions.fields", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "external-data-configuration.hive-partitioning-options.mode" => Some(("externalDataConfiguration.hivePartitioningOptions.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.hive-partitioning-options.require-partition-filter" => Some(("externalDataConfiguration.hivePartitioningOptions.requirePartitionFilter", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.hive-partitioning-options.source-uri-prefix" => Some(("externalDataConfiguration.hivePartitioningOptions.sourceUriPrefix", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.ignore-unknown-values" => Some(("externalDataConfiguration.ignoreUnknownValues", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.json-extension" => Some(("externalDataConfiguration.jsonExtension", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.json-options.encoding" => Some(("externalDataConfiguration.jsonOptions.encoding", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.max-bad-records" => Some(("externalDataConfiguration.maxBadRecords", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "external-data-configuration.metadata-cache-mode" => Some(("externalDataConfiguration.metadataCacheMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.object-metadata" => Some(("externalDataConfiguration.objectMetadata", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.parquet-options.enable-list-inference" => Some(("externalDataConfiguration.parquetOptions.enableListInference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.parquet-options.enum-as-string" => Some(("externalDataConfiguration.parquetOptions.enumAsString", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.reference-file-schema-uri" => Some(("externalDataConfiguration.referenceFileSchemaUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.source-format" => Some(("externalDataConfiguration.sourceFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.source-uris" => Some(("externalDataConfiguration.sourceUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "friendly-name" => Some(("friendlyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "last-modified-time" => Some(("lastModifiedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location" => Some(("location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "materialized-view.allow-non-incremental-definition" => Some(("materializedView.allowNonIncrementalDefinition", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "materialized-view.enable-refresh" => Some(("materializedView.enableRefresh", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "materialized-view.last-refresh-time" => Some(("materializedView.lastRefreshTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "materialized-view.max-staleness" => Some(("materializedView.maxStaleness", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "materialized-view.query" => Some(("materializedView.query", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "materialized-view.refresh-interval-ms" => Some(("materializedView.refreshIntervalMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "materialized-view-status.last-refresh-status.debug-info" => Some(("materializedViewStatus.lastRefreshStatus.debugInfo", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "materialized-view-status.last-refresh-status.location" => Some(("materializedViewStatus.lastRefreshStatus.location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "materialized-view-status.last-refresh-status.message" => Some(("materializedViewStatus.lastRefreshStatus.message", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "materialized-view-status.last-refresh-status.reason" => Some(("materializedViewStatus.lastRefreshStatus.reason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "materialized-view-status.refresh-watermark" => Some(("materializedViewStatus.refreshWatermark", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "max-staleness" => Some(("maxStaleness", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "model.model-options.labels" => Some(("model.modelOptions.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "model.model-options.loss-type" => Some(("model.modelOptions.lossType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "model.model-options.model-type" => Some(("model.modelOptions.modelType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-active-logical-bytes" => Some(("numActiveLogicalBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-active-physical-bytes" => Some(("numActivePhysicalBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-bytes" => Some(("numBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-long-term-bytes" => Some(("numLongTermBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-long-term-logical-bytes" => Some(("numLongTermLogicalBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-long-term-physical-bytes" => Some(("numLongTermPhysicalBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-partitions" => Some(("numPartitions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-physical-bytes" => Some(("numPhysicalBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-rows" => Some(("numRows", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-time-travel-physical-bytes" => Some(("numTimeTravelPhysicalBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-total-logical-bytes" => Some(("numTotalLogicalBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-total-physical-bytes" => Some(("numTotalPhysicalBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "range-partitioning.field" => Some(("rangePartitioning.field", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "range-partitioning.range.end" => Some(("rangePartitioning.range.end", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "range-partitioning.range.interval" => Some(("rangePartitioning.range.interval", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "range-partitioning.range.start" => Some(("rangePartitioning.range.start", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "require-partition-filter" => Some(("requirePartitionFilter", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "resource-tags" => Some(("resourceTags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snapshot-definition.base-table-reference.dataset-id" => Some(("snapshotDefinition.baseTableReference.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snapshot-definition.base-table-reference.project-id" => Some(("snapshotDefinition.baseTableReference.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snapshot-definition.base-table-reference.table-id" => Some(("snapshotDefinition.baseTableReference.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snapshot-definition.snapshot-time" => Some(("snapshotDefinition.snapshotTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "streaming-buffer.estimated-bytes" => Some(("streamingBuffer.estimatedBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "streaming-buffer.estimated-rows" => Some(("streamingBuffer.estimatedRows", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "streaming-buffer.oldest-entry-time" => Some(("streamingBuffer.oldestEntryTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-constraints.primary-key.columns" => Some(("tableConstraints.primaryKey.columns", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "table-reference.dataset-id" => Some(("tableReference.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-reference.project-id" => Some(("tableReference.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-reference.table-id" => Some(("tableReference.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-replication-info.replicated-source-last-refresh-time" => Some(("tableReplicationInfo.replicatedSourceLastRefreshTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-replication-info.replication-error.debug-info" => Some(("tableReplicationInfo.replicationError.debugInfo", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-replication-info.replication-error.location" => Some(("tableReplicationInfo.replicationError.location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-replication-info.replication-error.message" => Some(("tableReplicationInfo.replicationError.message", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-replication-info.replication-error.reason" => Some(("tableReplicationInfo.replicationError.reason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-replication-info.replication-interval-ms" => Some(("tableReplicationInfo.replicationIntervalMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-replication-info.replication-status" => Some(("tableReplicationInfo.replicationStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-replication-info.source-table.dataset-id" => Some(("tableReplicationInfo.sourceTable.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-replication-info.source-table.project-id" => Some(("tableReplicationInfo.sourceTable.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-replication-info.source-table.table-id" => Some(("tableReplicationInfo.sourceTable.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-partitioning.expiration-ms" => Some(("timePartitioning.expirationMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-partitioning.field" => Some(("timePartitioning.field", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-partitioning.require-partition-filter" => Some(("timePartitioning.requirePartitionFilter", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "time-partitioning.type" => Some(("timePartitioning.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "view.privacy-policy.aggregation-threshold-policy.privacy-unit-columns" => Some(("view.privacyPolicy.aggregationThresholdPolicy.privacyUnitColumns", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "view.privacy-policy.aggregation-threshold-policy.threshold" => Some(("view.privacyPolicy.aggregationThresholdPolicy.threshold", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "view.query" => Some(("view.query", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "view.use-explicit-column-names" => Some(("view.useExplicitColumnNames", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "view.use-legacy-sql" => Some(("view.useLegacySql", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["aggregation-threshold-policy", "allow-jagged-rows", "allow-non-incremental-definition", "allow-quoted-newlines", "autodetect", "avro-options", "base-table-reference", "biglake-configuration", "bigtable-options", "clone-definition", "clone-time", "clustering", "columns", "compression", "connection-id", "creation-time", "csv-options", "dataset-id", "debug-info", "decimal-target-types", "default-collation", "default-rounding-mode", "description", "enable-list-inference", "enable-refresh", "encoding", "encryption-configuration", "end", "enum-as-string", "estimated-bytes", "estimated-rows", "etag", "expiration-ms", "expiration-time", "external-data-configuration", "field", "field-delimiter", "fields", "file-format", "file-set-spec-type", "friendly-name", "google-sheets-options", "hive-partitioning-options", "id", "ignore-unknown-values", "ignore-unspecified-column-families", "interval", "json-extension", "json-options", "kind", "kms-key-name", "labels", "last-modified-time", "last-refresh-status", "last-refresh-time", "location", "loss-type", "materialized-view", "materialized-view-status", "max-bad-records", "max-staleness", "message", "metadata-cache-mode", "mode", "model", "model-options", "model-type", "null-marker", "num-active-logical-bytes", "num-active-physical-bytes", "num-bytes", "num-long-term-bytes", "num-long-term-logical-bytes", "num-long-term-physical-bytes", "num-partitions", "num-physical-bytes", "num-rows", "num-time-travel-physical-bytes", "num-total-logical-bytes", "num-total-physical-bytes", "object-metadata", "oldest-entry-time", "output-column-families-as-json", "parquet-options", "preserve-ascii-control-characters", "primary-key", "privacy-policy", "privacy-unit-columns", "project-id", "query", "quote", "range", "range-partitioning", "read-rowkey-as-string", "reason", "reference-file-schema-uri", "refresh-interval-ms", "refresh-watermark", "replicated-source-last-refresh-time", "replication-error", "replication-interval-ms", "replication-status", "require-partition-filter", "resource-tags", "self-link", "skip-leading-rows", "snapshot-definition", "snapshot-time", "source-format", "source-table", "source-uri-prefix", "source-uris", "start", "storage-uri", "streaming-buffer", "table-constraints", "table-format", "table-id", "table-reference", "table-replication-info", "threshold", "time-partitioning", "type", "use-avro-logical-types", "use-explicit-column-names", "use-legacy-sql", "view"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Table = json::value::from_value(object).unwrap();
        let mut call = self.hub.tables().patch(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""), opt.value_of("table-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "autodetect-schema" => {
                    call = call.autodetect_schema(        value.map(|v| arg_from_str(v, err, "autodetect-schema", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["autodetect-schema"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _tables_set_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.tables().set_iam_policy(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _tables_test_iam_permissions(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.tables().test_iam_permissions(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _tables_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "biglake-configuration.connection-id" => Some(("biglakeConfiguration.connectionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "biglake-configuration.file-format" => Some(("biglakeConfiguration.fileFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "biglake-configuration.storage-uri" => Some(("biglakeConfiguration.storageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "biglake-configuration.table-format" => Some(("biglakeConfiguration.tableFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "clone-definition.base-table-reference.dataset-id" => Some(("cloneDefinition.baseTableReference.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "clone-definition.base-table-reference.project-id" => Some(("cloneDefinition.baseTableReference.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "clone-definition.base-table-reference.table-id" => Some(("cloneDefinition.baseTableReference.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "clone-definition.clone-time" => Some(("cloneDefinition.cloneTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "clustering.fields" => Some(("clustering.fields", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "creation-time" => Some(("creationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-collation" => Some(("defaultCollation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-rounding-mode" => Some(("defaultRoundingMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "encryption-configuration.kms-key-name" => Some(("encryptionConfiguration.kmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "expiration-time" => Some(("expirationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.autodetect" => Some(("externalDataConfiguration.autodetect", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.avro-options.use-avro-logical-types" => Some(("externalDataConfiguration.avroOptions.useAvroLogicalTypes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.bigtable-options.ignore-unspecified-column-families" => Some(("externalDataConfiguration.bigtableOptions.ignoreUnspecifiedColumnFamilies", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.bigtable-options.output-column-families-as-json" => Some(("externalDataConfiguration.bigtableOptions.outputColumnFamiliesAsJson", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.bigtable-options.read-rowkey-as-string" => Some(("externalDataConfiguration.bigtableOptions.readRowkeyAsString", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.compression" => Some(("externalDataConfiguration.compression", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.connection-id" => Some(("externalDataConfiguration.connectionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.allow-jagged-rows" => Some(("externalDataConfiguration.csvOptions.allowJaggedRows", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.allow-quoted-newlines" => Some(("externalDataConfiguration.csvOptions.allowQuotedNewlines", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.encoding" => Some(("externalDataConfiguration.csvOptions.encoding", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.field-delimiter" => Some(("externalDataConfiguration.csvOptions.fieldDelimiter", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.null-marker" => Some(("externalDataConfiguration.csvOptions.nullMarker", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.preserve-ascii-control-characters" => Some(("externalDataConfiguration.csvOptions.preserveAsciiControlCharacters", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.quote" => Some(("externalDataConfiguration.csvOptions.quote", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.skip-leading-rows" => Some(("externalDataConfiguration.csvOptions.skipLeadingRows", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.decimal-target-types" => Some(("externalDataConfiguration.decimalTargetTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "external-data-configuration.file-set-spec-type" => Some(("externalDataConfiguration.fileSetSpecType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.google-sheets-options.range" => Some(("externalDataConfiguration.googleSheetsOptions.range", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.google-sheets-options.skip-leading-rows" => Some(("externalDataConfiguration.googleSheetsOptions.skipLeadingRows", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.hive-partitioning-options.fields" => Some(("externalDataConfiguration.hivePartitioningOptions.fields", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "external-data-configuration.hive-partitioning-options.mode" => Some(("externalDataConfiguration.hivePartitioningOptions.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.hive-partitioning-options.require-partition-filter" => Some(("externalDataConfiguration.hivePartitioningOptions.requirePartitionFilter", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.hive-partitioning-options.source-uri-prefix" => Some(("externalDataConfiguration.hivePartitioningOptions.sourceUriPrefix", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.ignore-unknown-values" => Some(("externalDataConfiguration.ignoreUnknownValues", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.json-extension" => Some(("externalDataConfiguration.jsonExtension", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.json-options.encoding" => Some(("externalDataConfiguration.jsonOptions.encoding", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.max-bad-records" => Some(("externalDataConfiguration.maxBadRecords", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "external-data-configuration.metadata-cache-mode" => Some(("externalDataConfiguration.metadataCacheMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.object-metadata" => Some(("externalDataConfiguration.objectMetadata", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.parquet-options.enable-list-inference" => Some(("externalDataConfiguration.parquetOptions.enableListInference", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.parquet-options.enum-as-string" => Some(("externalDataConfiguration.parquetOptions.enumAsString", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.reference-file-schema-uri" => Some(("externalDataConfiguration.referenceFileSchemaUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.source-format" => Some(("externalDataConfiguration.sourceFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.source-uris" => Some(("externalDataConfiguration.sourceUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "friendly-name" => Some(("friendlyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "last-modified-time" => Some(("lastModifiedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location" => Some(("location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "materialized-view.allow-non-incremental-definition" => Some(("materializedView.allowNonIncrementalDefinition", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "materialized-view.enable-refresh" => Some(("materializedView.enableRefresh", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "materialized-view.last-refresh-time" => Some(("materializedView.lastRefreshTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "materialized-view.max-staleness" => Some(("materializedView.maxStaleness", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "materialized-view.query" => Some(("materializedView.query", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "materialized-view.refresh-interval-ms" => Some(("materializedView.refreshIntervalMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "materialized-view-status.last-refresh-status.debug-info" => Some(("materializedViewStatus.lastRefreshStatus.debugInfo", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "materialized-view-status.last-refresh-status.location" => Some(("materializedViewStatus.lastRefreshStatus.location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "materialized-view-status.last-refresh-status.message" => Some(("materializedViewStatus.lastRefreshStatus.message", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "materialized-view-status.last-refresh-status.reason" => Some(("materializedViewStatus.lastRefreshStatus.reason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "materialized-view-status.refresh-watermark" => Some(("materializedViewStatus.refreshWatermark", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "max-staleness" => Some(("maxStaleness", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "model.model-options.labels" => Some(("model.modelOptions.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "model.model-options.loss-type" => Some(("model.modelOptions.lossType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "model.model-options.model-type" => Some(("model.modelOptions.modelType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-active-logical-bytes" => Some(("numActiveLogicalBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-active-physical-bytes" => Some(("numActivePhysicalBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-bytes" => Some(("numBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-long-term-bytes" => Some(("numLongTermBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-long-term-logical-bytes" => Some(("numLongTermLogicalBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-long-term-physical-bytes" => Some(("numLongTermPhysicalBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-partitions" => Some(("numPartitions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-physical-bytes" => Some(("numPhysicalBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-rows" => Some(("numRows", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-time-travel-physical-bytes" => Some(("numTimeTravelPhysicalBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-total-logical-bytes" => Some(("numTotalLogicalBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-total-physical-bytes" => Some(("numTotalPhysicalBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "range-partitioning.field" => Some(("rangePartitioning.field", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "range-partitioning.range.end" => Some(("rangePartitioning.range.end", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "range-partitioning.range.interval" => Some(("rangePartitioning.range.interval", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "range-partitioning.range.start" => Some(("rangePartitioning.range.start", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "require-partition-filter" => Some(("requirePartitionFilter", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "resource-tags" => Some(("resourceTags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snapshot-definition.base-table-reference.dataset-id" => Some(("snapshotDefinition.baseTableReference.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snapshot-definition.base-table-reference.project-id" => Some(("snapshotDefinition.baseTableReference.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snapshot-definition.base-table-reference.table-id" => Some(("snapshotDefinition.baseTableReference.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snapshot-definition.snapshot-time" => Some(("snapshotDefinition.snapshotTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "streaming-buffer.estimated-bytes" => Some(("streamingBuffer.estimatedBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "streaming-buffer.estimated-rows" => Some(("streamingBuffer.estimatedRows", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "streaming-buffer.oldest-entry-time" => Some(("streamingBuffer.oldestEntryTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-constraints.primary-key.columns" => Some(("tableConstraints.primaryKey.columns", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "table-reference.dataset-id" => Some(("tableReference.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-reference.project-id" => Some(("tableReference.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-reference.table-id" => Some(("tableReference.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-replication-info.replicated-source-last-refresh-time" => Some(("tableReplicationInfo.replicatedSourceLastRefreshTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-replication-info.replication-error.debug-info" => Some(("tableReplicationInfo.replicationError.debugInfo", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-replication-info.replication-error.location" => Some(("tableReplicationInfo.replicationError.location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-replication-info.replication-error.message" => Some(("tableReplicationInfo.replicationError.message", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-replication-info.replication-error.reason" => Some(("tableReplicationInfo.replicationError.reason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-replication-info.replication-interval-ms" => Some(("tableReplicationInfo.replicationIntervalMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-replication-info.replication-status" => Some(("tableReplicationInfo.replicationStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-replication-info.source-table.dataset-id" => Some(("tableReplicationInfo.sourceTable.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-replication-info.source-table.project-id" => Some(("tableReplicationInfo.sourceTable.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-replication-info.source-table.table-id" => Some(("tableReplicationInfo.sourceTable.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-partitioning.expiration-ms" => Some(("timePartitioning.expirationMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-partitioning.field" => Some(("timePartitioning.field", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-partitioning.require-partition-filter" => Some(("timePartitioning.requirePartitionFilter", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "time-partitioning.type" => Some(("timePartitioning.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "view.privacy-policy.aggregation-threshold-policy.privacy-unit-columns" => Some(("view.privacyPolicy.aggregationThresholdPolicy.privacyUnitColumns", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "view.privacy-policy.aggregation-threshold-policy.threshold" => Some(("view.privacyPolicy.aggregationThresholdPolicy.threshold", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "view.query" => Some(("view.query", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "view.use-explicit-column-names" => Some(("view.useExplicitColumnNames", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "view.use-legacy-sql" => Some(("view.useLegacySql", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["aggregation-threshold-policy", "allow-jagged-rows", "allow-non-incremental-definition", "allow-quoted-newlines", "autodetect", "avro-options", "base-table-reference", "biglake-configuration", "bigtable-options", "clone-definition", "clone-time", "clustering", "columns", "compression", "connection-id", "creation-time", "csv-options", "dataset-id", "debug-info", "decimal-target-types", "default-collation", "default-rounding-mode", "description", "enable-list-inference", "enable-refresh", "encoding", "encryption-configuration", "end", "enum-as-string", "estimated-bytes", "estimated-rows", "etag", "expiration-ms", "expiration-time", "external-data-configuration", "field", "field-delimiter", "fields", "file-format", "file-set-spec-type", "friendly-name", "google-sheets-options", "hive-partitioning-options", "id", "ignore-unknown-values", "ignore-unspecified-column-families", "interval", "json-extension", "json-options", "kind", "kms-key-name", "labels", "last-modified-time", "last-refresh-status", "last-refresh-time", "location", "loss-type", "materialized-view", "materialized-view-status", "max-bad-records", "max-staleness", "message", "metadata-cache-mode", "mode", "model", "model-options", "model-type", "null-marker", "num-active-logical-bytes", "num-active-physical-bytes", "num-bytes", "num-long-term-bytes", "num-long-term-logical-bytes", "num-long-term-physical-bytes", "num-partitions", "num-physical-bytes", "num-rows", "num-time-travel-physical-bytes", "num-total-logical-bytes", "num-total-physical-bytes", "object-metadata", "oldest-entry-time", "output-column-families-as-json", "parquet-options", "preserve-ascii-control-characters", "primary-key", "privacy-policy", "privacy-unit-columns", "project-id", "query", "quote", "range", "range-partitioning", "read-rowkey-as-string", "reason", "reference-file-schema-uri", "refresh-interval-ms", "refresh-watermark", "replicated-source-last-refresh-time", "replication-error", "replication-interval-ms", "replication-status", "require-partition-filter", "resource-tags", "self-link", "skip-leading-rows", "snapshot-definition", "snapshot-time", "source-format", "source-table", "source-uri-prefix", "source-uris", "start", "storage-uri", "streaming-buffer", "table-constraints", "table-format", "table-id", "table-reference", "table-replication-info", "threshold", "time-partitioning", "type", "use-avro-logical-types", "use-explicit-column-names", "use-legacy-sql", "view"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Table = json::value::from_value(object).unwrap();
        let mut call = self.hub.tables().update(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""), opt.value_of("table-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "autodetect-schema" => {
                    call = call.autodetect_schema(        value.map(|v| arg_from_str(v, err, "autodetect-schema", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["autodetect-schema"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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
            ("datasets", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._datasets_delete(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._datasets_get(opt, dry_run, &mut err).await;
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._datasets_insert(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._datasets_list(opt, dry_run, &mut err).await;
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._datasets_patch(opt, dry_run, &mut err).await;
                    },
                    ("undelete", Some(opt)) => {
                        call_result = self._datasets_undelete(opt, dry_run, &mut err).await;
                    },
                    ("update", Some(opt)) => {
                        call_result = self._datasets_update(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("datasets".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("jobs", Some(opt)) => {
                match opt.subcommand() {
                    ("cancel", Some(opt)) => {
                        call_result = self._jobs_cancel(opt, dry_run, &mut err).await;
                    },
                    ("delete", Some(opt)) => {
                        call_result = self._jobs_delete(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._jobs_get(opt, dry_run, &mut err).await;
                    },
                    ("get-query-results", Some(opt)) => {
                        call_result = self._jobs_get_query_results(opt, dry_run, &mut err).await;
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._jobs_insert(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._jobs_list(opt, dry_run, &mut err).await;
                    },
                    ("query", Some(opt)) => {
                        call_result = self._jobs_query(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("jobs".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("models", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._models_delete(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._models_get(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._models_list(opt, dry_run, &mut err).await;
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._models_patch(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("models".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("projects", Some(opt)) => {
                match opt.subcommand() {
                    ("get-service-account", Some(opt)) => {
                        call_result = self._projects_get_service_account(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._projects_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("projects".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("routines", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._routines_delete(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._routines_get(opt, dry_run, &mut err).await;
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._routines_insert(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._routines_list(opt, dry_run, &mut err).await;
                    },
                    ("update", Some(opt)) => {
                        call_result = self._routines_update(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("routines".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("row-access-policies", Some(opt)) => {
                match opt.subcommand() {
                    ("get-iam-policy", Some(opt)) => {
                        call_result = self._row_access_policies_get_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._row_access_policies_list(opt, dry_run, &mut err).await;
                    },
                    ("test-iam-permissions", Some(opt)) => {
                        call_result = self._row_access_policies_test_iam_permissions(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("row-access-policies".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("tabledata", Some(opt)) => {
                match opt.subcommand() {
                    ("insert-all", Some(opt)) => {
                        call_result = self._tabledata_insert_all(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._tabledata_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("tabledata".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("tables", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._tables_delete(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._tables_get(opt, dry_run, &mut err).await;
                    },
                    ("get-iam-policy", Some(opt)) => {
                        call_result = self._tables_get_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._tables_insert(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._tables_list(opt, dry_run, &mut err).await;
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._tables_patch(opt, dry_run, &mut err).await;
                    },
                    ("set-iam-policy", Some(opt)) => {
                        call_result = self._tables_set_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("test-iam-permissions", Some(opt)) => {
                        call_result = self._tables_test_iam_permissions(opt, dry_run, &mut err).await;
                    },
                    ("update", Some(opt)) => {
                        call_result = self._tables_update(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("tables".to_string()));
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

            match client::application_secret_from_directory(&config_dir, "bigquery2-secret.json",
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
        ).persist_tokens_to_disk(format!("{}/bigquery2", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::Bigquery::new(client, auth),
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
    let upload_value_names = ["mode", "file"];
    let arg_data = [
        ("datasets", "methods: 'delete', 'get', 'insert', 'list', 'patch', 'undelete' and 'update'", vec![
            ("delete",
                    Some(r##"Deletes the dataset specified by the datasetId value. Before you can delete a dataset, you must delete all its tables, either manually or by specifying deleteContents. Immediately after deletion, you can create another dataset with the same name."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/datasets_delete",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. Project ID of the dataset being deleted"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Required. Dataset ID of dataset being deleted"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",
                    Some(r##"Returns the dataset specified by datasetID."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/datasets_get",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. Project ID of the requested dataset"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Required. Dataset ID of the requested dataset"##),
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
            ("insert",
                    Some(r##"Creates a new empty dataset."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/datasets_insert",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. Project ID of the new dataset"##),
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
            ("list",
                    Some(r##"Lists all datasets in the specified project to which the user has been granted the READER dataset role."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/datasets_list",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. Project ID of the datasets to be listed"##),
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
                    Some(r##"Updates information in an existing dataset. The update method replaces the entire dataset resource, whereas the patch method only replaces fields that are provided in the submitted dataset resource. This method supports RFC5789 patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/datasets_patch",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. Project ID of the dataset being updated"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Required. Dataset ID of the dataset being updated"##),
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
            ("undelete",
                    Some(r##"Undeletes a dataset which is within time travel window based on datasetId. If a time is specified, the dataset version deleted at that time is undeleted, else the last live version is undeleted."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/datasets_undelete",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. Project ID of the dataset to be undeleted"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Required. Dataset ID of dataset being deleted"##),
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
            ("update",
                    Some(r##"Updates information in an existing dataset. The update method replaces the entire dataset resource, whereas the patch method only replaces fields that are provided in the submitted dataset resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/datasets_update",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. Project ID of the dataset being updated"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Required. Dataset ID of the dataset being updated"##),
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
        
        ("jobs", "methods: 'cancel', 'delete', 'get', 'get-query-results', 'insert', 'list' and 'query'", vec![
            ("cancel",
                    Some(r##"Requests that a job be cancelled. This call will return immediately, and the client will need to poll for the job status to see if the cancel completed successfully. Cancelled jobs may still incur costs."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/jobs_cancel",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. Project ID of the job to cancel"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"job-id"##),
                     None,
                     Some(r##"Required. Job ID of the job to cancel"##),
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
            ("delete",
                    Some(r##"Requests the deletion of the metadata of a job. This call returns when the job's metadata is deleted."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/jobs_delete",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. Project ID of the job for which metadata is to be deleted."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"job-id"##),
                     None,
                     Some(r##"Required. Job ID of the job for which metadata is to be deleted. If this is a parent job which has child jobs, the metadata from all child jobs will be deleted as well. Direct deletion of the metadata of child jobs is not allowed."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",
                    Some(r##"Returns information about a specific job. Job information is available for a six month period after creation. Requires that you're the person who ran the job, or have the Is Owner project role."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/jobs_get",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. Project ID of the requested job."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"job-id"##),
                     None,
                     Some(r##"Required. Job ID of the requested job."##),
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
            ("get-query-results",
                    Some(r##"RPC to get the results of a query job."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/jobs_get-query-results",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. Project ID of the query job."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"job-id"##),
                     None,
                     Some(r##"Required. Job ID of the query job."##),
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
            ("insert",
                    Some(r##"Starts a new asynchronous job. This API has two different kinds of endpoint URIs, as this method supports a variety of use cases. * The *Metadata* URI is used for most interactions, as it accepts the job configuration directly. * The *Upload* URI is ONLY for the case when you're sending both a load job configuration and a data stream together. In this case, the Upload URI accepts the job configuration and the data as two distinct multipart MIME parts."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/jobs_insert",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Project ID of project that will be billed for the job."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"mode"##),
                     Some(r##"u"##),
                     Some(r##"Specify the upload protocol (simple) and the file to upload"##),
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
            ("list",
                    Some(r##"Lists all jobs that you started in the specified project. Job information is available for a six month period after creation. The job list is sorted in reverse chronological order, by job creation time. Requires the Can View project role, or the Is Owner project role if you set the allUsers property."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/jobs_list",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Project ID of the jobs to list."##),
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
            ("query",
                    Some(r##"Runs a BigQuery SQL query synchronously and returns query results if the query completes within a specified timeout."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/jobs_query",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. Project ID of the query request."##),
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
        
        ("models", "methods: 'delete', 'get', 'list' and 'patch'", vec![
            ("delete",
                    Some(r##"Deletes the model specified by modelId from the dataset."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/models_delete",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. Project ID of the model to delete."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Required. Dataset ID of the model to delete."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"model-id"##),
                     None,
                     Some(r##"Required. Model ID of the model to delete."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",
                    Some(r##"Gets the specified model resource by model ID."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/models_get",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. Project ID of the requested model."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Required. Dataset ID of the requested model."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"model-id"##),
                     None,
                     Some(r##"Required. Model ID of the requested model."##),
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
                    Some(r##"Lists all models in the specified dataset. Requires the READER dataset role. After retrieving the list of models, you can get information about a particular model by calling the models.get method."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/models_list",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. Project ID of the models to list."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Required. Dataset ID of the models to list."##),
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
                    Some(r##"Patch specific fields in the specified model."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/models_patch",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. Project ID of the model to patch."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Required. Dataset ID of the model to patch."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"model-id"##),
                     None,
                     Some(r##"Required. Model ID of the model to patch."##),
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
        
        ("projects", "methods: 'get-service-account' and 'list'", vec![
            ("get-service-account",
                    Some(r##"RPC to get the service account for a project used for interactions with Google Cloud KMS"##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/projects_get-service-account",
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
            ("list",
                    Some(r##"RPC to list projects to which the user has been granted any project role. Users of this method are encouraged to consider the [Resource Manager](https://cloud.google.com/resource-manager/docs/) API, which provides the underlying data for this method and has more capabilities."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/projects_list",
                  vec![
                    (Some(r##"v"##),
                     Some(r##"p"##),
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
        
        ("routines", "methods: 'delete', 'get', 'insert', 'list' and 'update'", vec![
            ("delete",
                    Some(r##"Deletes the routine specified by routineId from the dataset."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/routines_delete",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. Project ID of the routine to delete"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Required. Dataset ID of the routine to delete"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"routine-id"##),
                     None,
                     Some(r##"Required. Routine ID of the routine to delete"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",
                    Some(r##"Gets the specified routine resource by routine ID."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/routines_get",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. Project ID of the requested routine"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Required. Dataset ID of the requested routine"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"routine-id"##),
                     None,
                     Some(r##"Required. Routine ID of the requested routine"##),
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
            ("insert",
                    Some(r##"Creates a new routine in the dataset."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/routines_insert",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. Project ID of the new routine"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Required. Dataset ID of the new routine"##),
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
            ("list",
                    Some(r##"Lists all routines in the specified dataset. Requires the READER dataset role."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/routines_list",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. Project ID of the routines to list"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Required. Dataset ID of the routines to list"##),
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
                    Some(r##"Updates information in an existing routine. The update method replaces the entire Routine resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/routines_update",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. Project ID of the routine to update"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Required. Dataset ID of the routine to update"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"routine-id"##),
                     None,
                     Some(r##"Required. Routine ID of the routine to update"##),
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
        
        ("row-access-policies", "methods: 'get-iam-policy', 'list' and 'test-iam-permissions'", vec![
            ("get-iam-policy",
                    Some(r##"Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/row-access-policies_get-iam-policy",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
            ("list",
                    Some(r##"Lists all row access policies on the specified table."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/row-access-policies_list",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. Project ID of the row access policies to list."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Required. Dataset ID of row access policies to list."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Required. Table ID of the table to list row access policies."##),
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
            ("test-iam-permissions",
                    Some(r##"Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/row-access-policies_test-iam-permissions",
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
        
        ("tabledata", "methods: 'insert-all' and 'list'", vec![
            ("insert-all",
                    Some(r##"Streams data into BigQuery one record at a time without needing to run a load job."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/tabledata_insert-all",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. Project ID of the destination."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Required. Dataset ID of the destination."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Required. Table ID of the destination."##),
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
            ("list",
                    Some(r##"List the content of a table in rows."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/tabledata_list",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. Project id of the table to list."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Required. Dataset id of the table to list."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Required. Table id of the table to list."##),
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
        
        ("tables", "methods: 'delete', 'get', 'get-iam-policy', 'insert', 'list', 'patch', 'set-iam-policy', 'test-iam-permissions' and 'update'", vec![
            ("delete",
                    Some(r##"Deletes the table specified by tableId from the dataset. If the table contains data, all the data will be deleted."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/tables_delete",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. Project ID of the table to delete"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Required. Dataset ID of the table to delete"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Required. Table ID of the table to delete"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",
                    Some(r##"Gets the specified table resource by table ID. This method does not return the data in the table, it only returns the table resource, which describes the structure of this table."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/tables_get",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. Project ID of the requested table"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Required. Dataset ID of the requested table"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Required. Table ID of the requested table"##),
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
            ("get-iam-policy",
                    Some(r##"Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/tables_get-iam-policy",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
            ("insert",
                    Some(r##"Creates a new, empty table in the dataset."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/tables_insert",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. Project ID of the new table"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Required. Dataset ID of the new table"##),
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
            ("list",
                    Some(r##"Lists all tables in the specified dataset. Requires the READER dataset role."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/tables_list",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. Project ID of the tables to list"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Required. Dataset ID of the tables to list"##),
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
                    Some(r##"Updates information in an existing table. The update method replaces the entire table resource, whereas the patch method only replaces fields that are provided in the submitted table resource. This method supports RFC5789 patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/tables_patch",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. Project ID of the table to update"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Required. Dataset ID of the table to update"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Required. Table ID of the table to update"##),
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
            ("set-iam-policy",
                    Some(r##"Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/tables_set-iam-policy",
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
            ("test-iam-permissions",
                    Some(r##"Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/tables_test-iam-permissions",
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
            ("update",
                    Some(r##"Updates information in an existing table. The update method replaces the entire Table resource, whereas the patch method only replaces fields that are provided in the submitted Table resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/tables_update",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Required. Project ID of the table to update"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Required. Dataset ID of the table to update"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Required. Table ID of the table to update"##),
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
    
    let mut app = App::new("bigquery2")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.3+20240214")
           .about("A data platform for customers to create, manage, share and query data.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_bigquery2_cli")
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
                       if arg_name_str == "mode" {
                           arg = arg.number_of_values(2);
                           arg = arg.value_names(&upload_value_names);
           
                           scmd = scmd.arg(Arg::with_name("mime")
                                               .short("m")
                                               .requires("mode")
                                               .required(false)
                                               .help("The file's mime time, like 'application/octet-stream'")
                                               .takes_value(true));
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
