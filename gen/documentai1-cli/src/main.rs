// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_documentai1::{api, Error, oauth2, client::chrono, FieldMask};


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
    hub: api::Document<S>,
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
    async fn _operations_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.operations().delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_fetch_processor_types(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_fetch_processor_types(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_processor_types_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_processor_types_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_processor_types_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_processor_types_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_processors_batch_process(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "document-output-config.gcs-output-config.field-mask" => Some(("documentOutputConfig.gcsOutputConfig.fieldMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "document-output-config.gcs-output-config.gcs-uri" => Some(("documentOutputConfig.gcsOutputConfig.gcsUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "document-output-config.gcs-output-config.sharding-config.pages-overlap" => Some(("documentOutputConfig.gcsOutputConfig.shardingConfig.pagesOverlap", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "document-output-config.gcs-output-config.sharding-config.pages-per-shard" => Some(("documentOutputConfig.gcsOutputConfig.shardingConfig.pagesPerShard", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "input-documents.gcs-prefix.gcs-uri-prefix" => Some(("inputDocuments.gcsPrefix.gcsUriPrefix", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "process-options.from-end" => Some(("processOptions.fromEnd", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "process-options.from-start" => Some(("processOptions.fromStart", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "process-options.individual-page-selector.pages" => Some(("processOptions.individualPageSelector.pages", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Vec })),
                    "process-options.layout-config.chunking-config.chunk-size" => Some(("processOptions.layoutConfig.chunkingConfig.chunkSize", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "process-options.layout-config.chunking-config.include-ancestor-headings" => Some(("processOptions.layoutConfig.chunkingConfig.includeAncestorHeadings", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.ocr-config.advanced-ocr-options" => Some(("processOptions.ocrConfig.advancedOcrOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "process-options.ocr-config.compute-style-info" => Some(("processOptions.ocrConfig.computeStyleInfo", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.ocr-config.disable-character-boxes-detection" => Some(("processOptions.ocrConfig.disableCharacterBoxesDetection", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.ocr-config.enable-image-quality-scores" => Some(("processOptions.ocrConfig.enableImageQualityScores", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.ocr-config.enable-native-pdf-parsing" => Some(("processOptions.ocrConfig.enableNativePdfParsing", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.ocr-config.enable-symbol" => Some(("processOptions.ocrConfig.enableSymbol", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.ocr-config.hints.language-hints" => Some(("processOptions.ocrConfig.hints.languageHints", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "process-options.ocr-config.premium-features.compute-style-info" => Some(("processOptions.ocrConfig.premiumFeatures.computeStyleInfo", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.ocr-config.premium-features.enable-math-ocr" => Some(("processOptions.ocrConfig.premiumFeatures.enableMathOcr", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.ocr-config.premium-features.enable-selection-mark-detection" => Some(("processOptions.ocrConfig.premiumFeatures.enableSelectionMarkDetection", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.schema-override.description" => Some(("processOptions.schemaOverride.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "process-options.schema-override.display-name" => Some(("processOptions.schemaOverride.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "process-options.schema-override.metadata.document-allow-multiple-labels" => Some(("processOptions.schemaOverride.metadata.documentAllowMultipleLabels", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.schema-override.metadata.document-splitter" => Some(("processOptions.schemaOverride.metadata.documentSplitter", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.schema-override.metadata.prefixed-naming-on-properties" => Some(("processOptions.schemaOverride.metadata.prefixedNamingOnProperties", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.schema-override.metadata.skip-naming-validation" => Some(("processOptions.schemaOverride.metadata.skipNamingValidation", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "skip-human-review" => Some(("skipHumanReview", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["advanced-ocr-options", "chunk-size", "chunking-config", "compute-style-info", "description", "disable-character-boxes-detection", "display-name", "document-allow-multiple-labels", "document-output-config", "document-splitter", "enable-image-quality-scores", "enable-math-ocr", "enable-native-pdf-parsing", "enable-selection-mark-detection", "enable-symbol", "field-mask", "from-end", "from-start", "gcs-output-config", "gcs-prefix", "gcs-uri", "gcs-uri-prefix", "hints", "include-ancestor-headings", "individual-page-selector", "input-documents", "labels", "language-hints", "layout-config", "metadata", "ocr-config", "pages", "pages-overlap", "pages-per-shard", "prefixed-naming-on-properties", "premium-features", "process-options", "schema-override", "sharding-config", "skip-human-review", "skip-naming-validation"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDocumentaiV1BatchProcessRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_processors_batch_process(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_processors_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "default-processor-version" => Some(("defaultProcessorVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kms-key-name" => Some(("kmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "process-endpoint" => Some(("processEndpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "satisfies-pzi" => Some(("satisfiesPzi", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "satisfies-pzs" => Some(("satisfiesPzs", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["create-time", "default-processor-version", "display-name", "kms-key-name", "name", "process-endpoint", "satisfies-pzi", "satisfies-pzs", "state", "type"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDocumentaiV1Processor = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_processors_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_processors_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_processors_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_processors_disable(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::GoogleCloudDocumentaiV1DisableProcessorRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_processors_disable(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_processors_enable(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::GoogleCloudDocumentaiV1EnableProcessorRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_processors_enable(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_processors_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_processors_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_processors_human_review_config_review_document(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "document-schema.description" => Some(("documentSchema.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "document-schema.display-name" => Some(("documentSchema.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "document-schema.metadata.document-allow-multiple-labels" => Some(("documentSchema.metadata.documentAllowMultipleLabels", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "document-schema.metadata.document-splitter" => Some(("documentSchema.metadata.documentSplitter", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "document-schema.metadata.prefixed-naming-on-properties" => Some(("documentSchema.metadata.prefixedNamingOnProperties", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "document-schema.metadata.skip-naming-validation" => Some(("documentSchema.metadata.skipNamingValidation", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "enable-schema-validation" => Some(("enableSchemaValidation", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inline-document.content" => Some(("inlineDocument.content", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inline-document.error.code" => Some(("inlineDocument.error.code", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inline-document.error.message" => Some(("inlineDocument.error.message", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inline-document.mime-type" => Some(("inlineDocument.mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inline-document.shard-info.shard-count" => Some(("inlineDocument.shardInfo.shardCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inline-document.shard-info.shard-index" => Some(("inlineDocument.shardInfo.shardIndex", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inline-document.shard-info.text-offset" => Some(("inlineDocument.shardInfo.textOffset", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inline-document.text" => Some(("inlineDocument.text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inline-document.uri" => Some(("inlineDocument.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "priority" => Some(("priority", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["code", "content", "description", "display-name", "document-allow-multiple-labels", "document-schema", "document-splitter", "enable-schema-validation", "error", "inline-document", "message", "metadata", "mime-type", "prefixed-naming-on-properties", "priority", "shard-count", "shard-index", "shard-info", "skip-naming-validation", "text", "text-offset", "uri"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDocumentaiV1ReviewDocumentRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_processors_human_review_config_review_document(request, opt.value_of("human-review-config").unwrap_or(""));
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

    async fn _projects_locations_processors_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_processors_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_processors_process(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "field-mask" => Some(("fieldMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gcs-document.gcs-uri" => Some(("gcsDocument.gcsUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gcs-document.mime-type" => Some(("gcsDocument.mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inline-document.content" => Some(("inlineDocument.content", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inline-document.error.code" => Some(("inlineDocument.error.code", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inline-document.error.message" => Some(("inlineDocument.error.message", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inline-document.mime-type" => Some(("inlineDocument.mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inline-document.shard-info.shard-count" => Some(("inlineDocument.shardInfo.shardCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inline-document.shard-info.shard-index" => Some(("inlineDocument.shardInfo.shardIndex", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inline-document.shard-info.text-offset" => Some(("inlineDocument.shardInfo.textOffset", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inline-document.text" => Some(("inlineDocument.text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inline-document.uri" => Some(("inlineDocument.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "process-options.from-end" => Some(("processOptions.fromEnd", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "process-options.from-start" => Some(("processOptions.fromStart", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "process-options.individual-page-selector.pages" => Some(("processOptions.individualPageSelector.pages", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Vec })),
                    "process-options.layout-config.chunking-config.chunk-size" => Some(("processOptions.layoutConfig.chunkingConfig.chunkSize", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "process-options.layout-config.chunking-config.include-ancestor-headings" => Some(("processOptions.layoutConfig.chunkingConfig.includeAncestorHeadings", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.ocr-config.advanced-ocr-options" => Some(("processOptions.ocrConfig.advancedOcrOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "process-options.ocr-config.compute-style-info" => Some(("processOptions.ocrConfig.computeStyleInfo", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.ocr-config.disable-character-boxes-detection" => Some(("processOptions.ocrConfig.disableCharacterBoxesDetection", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.ocr-config.enable-image-quality-scores" => Some(("processOptions.ocrConfig.enableImageQualityScores", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.ocr-config.enable-native-pdf-parsing" => Some(("processOptions.ocrConfig.enableNativePdfParsing", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.ocr-config.enable-symbol" => Some(("processOptions.ocrConfig.enableSymbol", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.ocr-config.hints.language-hints" => Some(("processOptions.ocrConfig.hints.languageHints", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "process-options.ocr-config.premium-features.compute-style-info" => Some(("processOptions.ocrConfig.premiumFeatures.computeStyleInfo", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.ocr-config.premium-features.enable-math-ocr" => Some(("processOptions.ocrConfig.premiumFeatures.enableMathOcr", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.ocr-config.premium-features.enable-selection-mark-detection" => Some(("processOptions.ocrConfig.premiumFeatures.enableSelectionMarkDetection", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.schema-override.description" => Some(("processOptions.schemaOverride.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "process-options.schema-override.display-name" => Some(("processOptions.schemaOverride.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "process-options.schema-override.metadata.document-allow-multiple-labels" => Some(("processOptions.schemaOverride.metadata.documentAllowMultipleLabels", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.schema-override.metadata.document-splitter" => Some(("processOptions.schemaOverride.metadata.documentSplitter", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.schema-override.metadata.prefixed-naming-on-properties" => Some(("processOptions.schemaOverride.metadata.prefixedNamingOnProperties", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.schema-override.metadata.skip-naming-validation" => Some(("processOptions.schemaOverride.metadata.skipNamingValidation", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "raw-document.content" => Some(("rawDocument.content", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "raw-document.display-name" => Some(("rawDocument.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "raw-document.mime-type" => Some(("rawDocument.mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "skip-human-review" => Some(("skipHumanReview", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["advanced-ocr-options", "chunk-size", "chunking-config", "code", "compute-style-info", "content", "description", "disable-character-boxes-detection", "display-name", "document-allow-multiple-labels", "document-splitter", "enable-image-quality-scores", "enable-math-ocr", "enable-native-pdf-parsing", "enable-selection-mark-detection", "enable-symbol", "error", "field-mask", "from-end", "from-start", "gcs-document", "gcs-uri", "hints", "include-ancestor-headings", "individual-page-selector", "inline-document", "labels", "language-hints", "layout-config", "message", "metadata", "mime-type", "ocr-config", "pages", "prefixed-naming-on-properties", "premium-features", "process-options", "raw-document", "schema-override", "shard-count", "shard-index", "shard-info", "skip-human-review", "skip-naming-validation", "text", "text-offset", "uri"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDocumentaiV1ProcessRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_processors_process(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_processors_processor_versions_batch_process(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "document-output-config.gcs-output-config.field-mask" => Some(("documentOutputConfig.gcsOutputConfig.fieldMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "document-output-config.gcs-output-config.gcs-uri" => Some(("documentOutputConfig.gcsOutputConfig.gcsUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "document-output-config.gcs-output-config.sharding-config.pages-overlap" => Some(("documentOutputConfig.gcsOutputConfig.shardingConfig.pagesOverlap", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "document-output-config.gcs-output-config.sharding-config.pages-per-shard" => Some(("documentOutputConfig.gcsOutputConfig.shardingConfig.pagesPerShard", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "input-documents.gcs-prefix.gcs-uri-prefix" => Some(("inputDocuments.gcsPrefix.gcsUriPrefix", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "process-options.from-end" => Some(("processOptions.fromEnd", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "process-options.from-start" => Some(("processOptions.fromStart", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "process-options.individual-page-selector.pages" => Some(("processOptions.individualPageSelector.pages", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Vec })),
                    "process-options.layout-config.chunking-config.chunk-size" => Some(("processOptions.layoutConfig.chunkingConfig.chunkSize", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "process-options.layout-config.chunking-config.include-ancestor-headings" => Some(("processOptions.layoutConfig.chunkingConfig.includeAncestorHeadings", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.ocr-config.advanced-ocr-options" => Some(("processOptions.ocrConfig.advancedOcrOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "process-options.ocr-config.compute-style-info" => Some(("processOptions.ocrConfig.computeStyleInfo", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.ocr-config.disable-character-boxes-detection" => Some(("processOptions.ocrConfig.disableCharacterBoxesDetection", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.ocr-config.enable-image-quality-scores" => Some(("processOptions.ocrConfig.enableImageQualityScores", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.ocr-config.enable-native-pdf-parsing" => Some(("processOptions.ocrConfig.enableNativePdfParsing", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.ocr-config.enable-symbol" => Some(("processOptions.ocrConfig.enableSymbol", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.ocr-config.hints.language-hints" => Some(("processOptions.ocrConfig.hints.languageHints", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "process-options.ocr-config.premium-features.compute-style-info" => Some(("processOptions.ocrConfig.premiumFeatures.computeStyleInfo", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.ocr-config.premium-features.enable-math-ocr" => Some(("processOptions.ocrConfig.premiumFeatures.enableMathOcr", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.ocr-config.premium-features.enable-selection-mark-detection" => Some(("processOptions.ocrConfig.premiumFeatures.enableSelectionMarkDetection", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.schema-override.description" => Some(("processOptions.schemaOverride.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "process-options.schema-override.display-name" => Some(("processOptions.schemaOverride.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "process-options.schema-override.metadata.document-allow-multiple-labels" => Some(("processOptions.schemaOverride.metadata.documentAllowMultipleLabels", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.schema-override.metadata.document-splitter" => Some(("processOptions.schemaOverride.metadata.documentSplitter", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.schema-override.metadata.prefixed-naming-on-properties" => Some(("processOptions.schemaOverride.metadata.prefixedNamingOnProperties", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.schema-override.metadata.skip-naming-validation" => Some(("processOptions.schemaOverride.metadata.skipNamingValidation", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "skip-human-review" => Some(("skipHumanReview", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["advanced-ocr-options", "chunk-size", "chunking-config", "compute-style-info", "description", "disable-character-boxes-detection", "display-name", "document-allow-multiple-labels", "document-output-config", "document-splitter", "enable-image-quality-scores", "enable-math-ocr", "enable-native-pdf-parsing", "enable-selection-mark-detection", "enable-symbol", "field-mask", "from-end", "from-start", "gcs-output-config", "gcs-prefix", "gcs-uri", "gcs-uri-prefix", "hints", "include-ancestor-headings", "individual-page-selector", "input-documents", "labels", "language-hints", "layout-config", "metadata", "ocr-config", "pages", "pages-overlap", "pages-per-shard", "prefixed-naming-on-properties", "premium-features", "process-options", "schema-override", "sharding-config", "skip-human-review", "skip-naming-validation"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDocumentaiV1BatchProcessRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_processors_processor_versions_batch_process(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_processors_processor_versions_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_processors_processor_versions_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_processors_processor_versions_deploy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::GoogleCloudDocumentaiV1DeployProcessorVersionRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_processors_processor_versions_deploy(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_processors_processor_versions_evaluate_processor_version(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "evaluation-documents.gcs-prefix.gcs-uri-prefix" => Some(("evaluationDocuments.gcsPrefix.gcsUriPrefix", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["evaluation-documents", "gcs-prefix", "gcs-uri-prefix"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDocumentaiV1EvaluateProcessorVersionRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_processors_processor_versions_evaluate_processor_version(request, opt.value_of("processor-version").unwrap_or(""));
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

    async fn _projects_locations_processors_processor_versions_evaluations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_processors_processor_versions_evaluations_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_processors_processor_versions_evaluations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_processors_processor_versions_evaluations_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_processors_processor_versions_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_processors_processor_versions_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_processors_processor_versions_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_processors_processor_versions_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_processors_processor_versions_process(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "field-mask" => Some(("fieldMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gcs-document.gcs-uri" => Some(("gcsDocument.gcsUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gcs-document.mime-type" => Some(("gcsDocument.mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inline-document.content" => Some(("inlineDocument.content", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inline-document.error.code" => Some(("inlineDocument.error.code", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inline-document.error.message" => Some(("inlineDocument.error.message", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inline-document.mime-type" => Some(("inlineDocument.mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inline-document.shard-info.shard-count" => Some(("inlineDocument.shardInfo.shardCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inline-document.shard-info.shard-index" => Some(("inlineDocument.shardInfo.shardIndex", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inline-document.shard-info.text-offset" => Some(("inlineDocument.shardInfo.textOffset", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inline-document.text" => Some(("inlineDocument.text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inline-document.uri" => Some(("inlineDocument.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "process-options.from-end" => Some(("processOptions.fromEnd", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "process-options.from-start" => Some(("processOptions.fromStart", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "process-options.individual-page-selector.pages" => Some(("processOptions.individualPageSelector.pages", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Vec })),
                    "process-options.layout-config.chunking-config.chunk-size" => Some(("processOptions.layoutConfig.chunkingConfig.chunkSize", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "process-options.layout-config.chunking-config.include-ancestor-headings" => Some(("processOptions.layoutConfig.chunkingConfig.includeAncestorHeadings", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.ocr-config.advanced-ocr-options" => Some(("processOptions.ocrConfig.advancedOcrOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "process-options.ocr-config.compute-style-info" => Some(("processOptions.ocrConfig.computeStyleInfo", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.ocr-config.disable-character-boxes-detection" => Some(("processOptions.ocrConfig.disableCharacterBoxesDetection", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.ocr-config.enable-image-quality-scores" => Some(("processOptions.ocrConfig.enableImageQualityScores", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.ocr-config.enable-native-pdf-parsing" => Some(("processOptions.ocrConfig.enableNativePdfParsing", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.ocr-config.enable-symbol" => Some(("processOptions.ocrConfig.enableSymbol", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.ocr-config.hints.language-hints" => Some(("processOptions.ocrConfig.hints.languageHints", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "process-options.ocr-config.premium-features.compute-style-info" => Some(("processOptions.ocrConfig.premiumFeatures.computeStyleInfo", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.ocr-config.premium-features.enable-math-ocr" => Some(("processOptions.ocrConfig.premiumFeatures.enableMathOcr", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.ocr-config.premium-features.enable-selection-mark-detection" => Some(("processOptions.ocrConfig.premiumFeatures.enableSelectionMarkDetection", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.schema-override.description" => Some(("processOptions.schemaOverride.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "process-options.schema-override.display-name" => Some(("processOptions.schemaOverride.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "process-options.schema-override.metadata.document-allow-multiple-labels" => Some(("processOptions.schemaOverride.metadata.documentAllowMultipleLabels", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.schema-override.metadata.document-splitter" => Some(("processOptions.schemaOverride.metadata.documentSplitter", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.schema-override.metadata.prefixed-naming-on-properties" => Some(("processOptions.schemaOverride.metadata.prefixedNamingOnProperties", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "process-options.schema-override.metadata.skip-naming-validation" => Some(("processOptions.schemaOverride.metadata.skipNamingValidation", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "raw-document.content" => Some(("rawDocument.content", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "raw-document.display-name" => Some(("rawDocument.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "raw-document.mime-type" => Some(("rawDocument.mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "skip-human-review" => Some(("skipHumanReview", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["advanced-ocr-options", "chunk-size", "chunking-config", "code", "compute-style-info", "content", "description", "disable-character-boxes-detection", "display-name", "document-allow-multiple-labels", "document-splitter", "enable-image-quality-scores", "enable-math-ocr", "enable-native-pdf-parsing", "enable-selection-mark-detection", "enable-symbol", "error", "field-mask", "from-end", "from-start", "gcs-document", "gcs-uri", "hints", "include-ancestor-headings", "individual-page-selector", "inline-document", "labels", "language-hints", "layout-config", "message", "metadata", "mime-type", "ocr-config", "pages", "prefixed-naming-on-properties", "premium-features", "process-options", "raw-document", "schema-override", "shard-count", "shard-index", "shard-info", "skip-human-review", "skip-naming-validation", "text", "text-offset", "uri"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDocumentaiV1ProcessRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_processors_processor_versions_process(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_processors_processor_versions_train(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "base-processor-version" => Some(("baseProcessorVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "custom-document-extraction-options.training-method" => Some(("customDocumentExtractionOptions.trainingMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "document-schema.description" => Some(("documentSchema.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "document-schema.display-name" => Some(("documentSchema.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "document-schema.metadata.document-allow-multiple-labels" => Some(("documentSchema.metadata.documentAllowMultipleLabels", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "document-schema.metadata.document-splitter" => Some(("documentSchema.metadata.documentSplitter", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "document-schema.metadata.prefixed-naming-on-properties" => Some(("documentSchema.metadata.prefixedNamingOnProperties", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "document-schema.metadata.skip-naming-validation" => Some(("documentSchema.metadata.skipNamingValidation", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "foundation-model-tuning-options.learning-rate-multiplier" => Some(("foundationModelTuningOptions.learningRateMultiplier", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "foundation-model-tuning-options.train-steps" => Some(("foundationModelTuningOptions.trainSteps", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "input-data.test-documents.gcs-prefix.gcs-uri-prefix" => Some(("inputData.testDocuments.gcsPrefix.gcsUriPrefix", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "input-data.training-documents.gcs-prefix.gcs-uri-prefix" => Some(("inputData.trainingDocuments.gcsPrefix.gcsUriPrefix", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processor-version.create-time" => Some(("processorVersion.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processor-version.deprecation-info.deprecation-time" => Some(("processorVersion.deprecationInfo.deprecationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processor-version.deprecation-info.replacement-processor-version" => Some(("processorVersion.deprecationInfo.replacementProcessorVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processor-version.display-name" => Some(("processorVersion.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processor-version.document-schema.description" => Some(("processorVersion.documentSchema.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processor-version.document-schema.display-name" => Some(("processorVersion.documentSchema.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processor-version.document-schema.metadata.document-allow-multiple-labels" => Some(("processorVersion.documentSchema.metadata.documentAllowMultipleLabels", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "processor-version.document-schema.metadata.document-splitter" => Some(("processorVersion.documentSchema.metadata.documentSplitter", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "processor-version.document-schema.metadata.prefixed-naming-on-properties" => Some(("processorVersion.documentSchema.metadata.prefixedNamingOnProperties", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "processor-version.document-schema.metadata.skip-naming-validation" => Some(("processorVersion.documentSchema.metadata.skipNamingValidation", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "processor-version.google-managed" => Some(("processorVersion.googleManaged", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "processor-version.kms-key-name" => Some(("processorVersion.kmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processor-version.kms-key-version-name" => Some(("processorVersion.kmsKeyVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processor-version.latest-evaluation.aggregate-metrics.f1-score" => Some(("processorVersion.latestEvaluation.aggregateMetrics.f1Score", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "processor-version.latest-evaluation.aggregate-metrics.false-negatives-count" => Some(("processorVersion.latestEvaluation.aggregateMetrics.falseNegativesCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "processor-version.latest-evaluation.aggregate-metrics.false-positives-count" => Some(("processorVersion.latestEvaluation.aggregateMetrics.falsePositivesCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "processor-version.latest-evaluation.aggregate-metrics.ground-truth-document-count" => Some(("processorVersion.latestEvaluation.aggregateMetrics.groundTruthDocumentCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "processor-version.latest-evaluation.aggregate-metrics.ground-truth-occurrences-count" => Some(("processorVersion.latestEvaluation.aggregateMetrics.groundTruthOccurrencesCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "processor-version.latest-evaluation.aggregate-metrics.precision" => Some(("processorVersion.latestEvaluation.aggregateMetrics.precision", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "processor-version.latest-evaluation.aggregate-metrics.predicted-document-count" => Some(("processorVersion.latestEvaluation.aggregateMetrics.predictedDocumentCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "processor-version.latest-evaluation.aggregate-metrics.predicted-occurrences-count" => Some(("processorVersion.latestEvaluation.aggregateMetrics.predictedOccurrencesCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "processor-version.latest-evaluation.aggregate-metrics.recall" => Some(("processorVersion.latestEvaluation.aggregateMetrics.recall", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "processor-version.latest-evaluation.aggregate-metrics.total-documents-count" => Some(("processorVersion.latestEvaluation.aggregateMetrics.totalDocumentsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "processor-version.latest-evaluation.aggregate-metrics.true-positives-count" => Some(("processorVersion.latestEvaluation.aggregateMetrics.truePositivesCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "processor-version.latest-evaluation.aggregate-metrics-exact.f1-score" => Some(("processorVersion.latestEvaluation.aggregateMetricsExact.f1Score", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "processor-version.latest-evaluation.aggregate-metrics-exact.false-negatives-count" => Some(("processorVersion.latestEvaluation.aggregateMetricsExact.falseNegativesCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "processor-version.latest-evaluation.aggregate-metrics-exact.false-positives-count" => Some(("processorVersion.latestEvaluation.aggregateMetricsExact.falsePositivesCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "processor-version.latest-evaluation.aggregate-metrics-exact.ground-truth-document-count" => Some(("processorVersion.latestEvaluation.aggregateMetricsExact.groundTruthDocumentCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "processor-version.latest-evaluation.aggregate-metrics-exact.ground-truth-occurrences-count" => Some(("processorVersion.latestEvaluation.aggregateMetricsExact.groundTruthOccurrencesCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "processor-version.latest-evaluation.aggregate-metrics-exact.precision" => Some(("processorVersion.latestEvaluation.aggregateMetricsExact.precision", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "processor-version.latest-evaluation.aggregate-metrics-exact.predicted-document-count" => Some(("processorVersion.latestEvaluation.aggregateMetricsExact.predictedDocumentCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "processor-version.latest-evaluation.aggregate-metrics-exact.predicted-occurrences-count" => Some(("processorVersion.latestEvaluation.aggregateMetricsExact.predictedOccurrencesCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "processor-version.latest-evaluation.aggregate-metrics-exact.recall" => Some(("processorVersion.latestEvaluation.aggregateMetricsExact.recall", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "processor-version.latest-evaluation.aggregate-metrics-exact.total-documents-count" => Some(("processorVersion.latestEvaluation.aggregateMetricsExact.totalDocumentsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "processor-version.latest-evaluation.aggregate-metrics-exact.true-positives-count" => Some(("processorVersion.latestEvaluation.aggregateMetricsExact.truePositivesCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "processor-version.latest-evaluation.evaluation" => Some(("processorVersion.latestEvaluation.evaluation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processor-version.latest-evaluation.operation" => Some(("processorVersion.latestEvaluation.operation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processor-version.model-type" => Some(("processorVersion.modelType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processor-version.name" => Some(("processorVersion.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "processor-version.satisfies-pzi" => Some(("processorVersion.satisfiesPzi", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "processor-version.satisfies-pzs" => Some(("processorVersion.satisfiesPzs", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "processor-version.state" => Some(("processorVersion.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["aggregate-metrics", "aggregate-metrics-exact", "base-processor-version", "create-time", "custom-document-extraction-options", "deprecation-info", "deprecation-time", "description", "display-name", "document-allow-multiple-labels", "document-schema", "document-splitter", "evaluation", "f1-score", "false-negatives-count", "false-positives-count", "foundation-model-tuning-options", "gcs-prefix", "gcs-uri-prefix", "google-managed", "ground-truth-document-count", "ground-truth-occurrences-count", "input-data", "kms-key-name", "kms-key-version-name", "latest-evaluation", "learning-rate-multiplier", "metadata", "model-type", "name", "operation", "precision", "predicted-document-count", "predicted-occurrences-count", "prefixed-naming-on-properties", "processor-version", "recall", "replacement-processor-version", "satisfies-pzi", "satisfies-pzs", "skip-naming-validation", "state", "test-documents", "total-documents-count", "train-steps", "training-documents", "training-method", "true-positives-count"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDocumentaiV1TrainProcessorVersionRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_processors_processor_versions_train(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_processors_processor_versions_undeploy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::GoogleCloudDocumentaiV1UndeployProcessorVersionRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_processors_processor_versions_undeploy(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_processors_set_default_processor_version(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "default-processor-version" => Some(("defaultProcessorVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["default-processor-version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleCloudDocumentaiV1SetDefaultProcessorVersionRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_processors_set_default_processor_version(request, opt.value_of("processor").unwrap_or(""));
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

    async fn _doit(&self, dry_run: bool) -> Result<Result<(), DoitError>, Option<InvalidOptionsError>> {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Result<(), DoitError> = Ok(());
        let mut err_opt: Option<InvalidOptionsError> = None;
        match self.opt.subcommand() {
            ("operations", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._operations_delete(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("operations".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("projects", Some(opt)) => {
                match opt.subcommand() {
                    ("locations-fetch-processor-types", Some(opt)) => {
                        call_result = self._projects_locations_fetch_processor_types(opt, dry_run, &mut err).await;
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
                    ("locations-operations-list", Some(opt)) => {
                        call_result = self._projects_locations_operations_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-processor-types-get", Some(opt)) => {
                        call_result = self._projects_locations_processor_types_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-processor-types-list", Some(opt)) => {
                        call_result = self._projects_locations_processor_types_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-processors-batch-process", Some(opt)) => {
                        call_result = self._projects_locations_processors_batch_process(opt, dry_run, &mut err).await;
                    },
                    ("locations-processors-create", Some(opt)) => {
                        call_result = self._projects_locations_processors_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-processors-delete", Some(opt)) => {
                        call_result = self._projects_locations_processors_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-processors-disable", Some(opt)) => {
                        call_result = self._projects_locations_processors_disable(opt, dry_run, &mut err).await;
                    },
                    ("locations-processors-enable", Some(opt)) => {
                        call_result = self._projects_locations_processors_enable(opt, dry_run, &mut err).await;
                    },
                    ("locations-processors-get", Some(opt)) => {
                        call_result = self._projects_locations_processors_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-processors-human-review-config-review-document", Some(opt)) => {
                        call_result = self._projects_locations_processors_human_review_config_review_document(opt, dry_run, &mut err).await;
                    },
                    ("locations-processors-list", Some(opt)) => {
                        call_result = self._projects_locations_processors_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-processors-process", Some(opt)) => {
                        call_result = self._projects_locations_processors_process(opt, dry_run, &mut err).await;
                    },
                    ("locations-processors-processor-versions-batch-process", Some(opt)) => {
                        call_result = self._projects_locations_processors_processor_versions_batch_process(opt, dry_run, &mut err).await;
                    },
                    ("locations-processors-processor-versions-delete", Some(opt)) => {
                        call_result = self._projects_locations_processors_processor_versions_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-processors-processor-versions-deploy", Some(opt)) => {
                        call_result = self._projects_locations_processors_processor_versions_deploy(opt, dry_run, &mut err).await;
                    },
                    ("locations-processors-processor-versions-evaluate-processor-version", Some(opt)) => {
                        call_result = self._projects_locations_processors_processor_versions_evaluate_processor_version(opt, dry_run, &mut err).await;
                    },
                    ("locations-processors-processor-versions-evaluations-get", Some(opt)) => {
                        call_result = self._projects_locations_processors_processor_versions_evaluations_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-processors-processor-versions-evaluations-list", Some(opt)) => {
                        call_result = self._projects_locations_processors_processor_versions_evaluations_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-processors-processor-versions-get", Some(opt)) => {
                        call_result = self._projects_locations_processors_processor_versions_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-processors-processor-versions-list", Some(opt)) => {
                        call_result = self._projects_locations_processors_processor_versions_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-processors-processor-versions-process", Some(opt)) => {
                        call_result = self._projects_locations_processors_processor_versions_process(opt, dry_run, &mut err).await;
                    },
                    ("locations-processors-processor-versions-train", Some(opt)) => {
                        call_result = self._projects_locations_processors_processor_versions_train(opt, dry_run, &mut err).await;
                    },
                    ("locations-processors-processor-versions-undeploy", Some(opt)) => {
                        call_result = self._projects_locations_processors_processor_versions_undeploy(opt, dry_run, &mut err).await;
                    },
                    ("locations-processors-set-default-processor-version", Some(opt)) => {
                        call_result = self._projects_locations_processors_set_default_processor_version(opt, dry_run, &mut err).await;
                    },
                    ("operations-get", Some(opt)) => {
                        call_result = self._projects_operations_get(opt, dry_run, &mut err).await;
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

            match client::application_secret_from_directory(&config_dir, "documentai1-secret.json",
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
        ).persist_tokens_to_disk(format!("{}/documentai1", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::Document::new(client, auth),
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
        ("operations", "methods: 'delete'", vec![
            ("delete",
                    Some(r##"Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_documentai1_cli/operations_delete",
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
            ]),
        
        ("projects", "methods: 'locations-fetch-processor-types', 'locations-get', 'locations-list', 'locations-operations-cancel', 'locations-operations-get', 'locations-operations-list', 'locations-processor-types-get', 'locations-processor-types-list', 'locations-processors-batch-process', 'locations-processors-create', 'locations-processors-delete', 'locations-processors-disable', 'locations-processors-enable', 'locations-processors-get', 'locations-processors-human-review-config-review-document', 'locations-processors-list', 'locations-processors-process', 'locations-processors-processor-versions-batch-process', 'locations-processors-processor-versions-delete', 'locations-processors-processor-versions-deploy', 'locations-processors-processor-versions-evaluate-processor-version', 'locations-processors-processor-versions-evaluations-get', 'locations-processors-processor-versions-evaluations-list', 'locations-processors-processor-versions-get', 'locations-processors-processor-versions-list', 'locations-processors-processor-versions-process', 'locations-processors-processor-versions-train', 'locations-processors-processor-versions-undeploy', 'locations-processors-set-default-processor-version' and 'operations-get'", vec![
            ("locations-fetch-processor-types",
                    Some(r##"Fetches processor types. Note that we don't use ListProcessorTypes here, because it isn't paginated."##),
                    "Details at http://byron.github.io/google-apis-rs/google_documentai1_cli/projects_locations-fetch-processor-types",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The location of processor types to list. Format: `projects/{project}/locations/{location}`."##),
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
            ("locations-get",
                    Some(r##"Gets information about a location."##),
                    "Details at http://byron.github.io/google-apis-rs/google_documentai1_cli/projects_locations-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Resource name for the location."##),
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
                    "Details at http://byron.github.io/google-apis-rs/google_documentai1_cli/projects_locations-list",
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
            ("locations-operations-cancel",
                    Some(r##"Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_documentai1_cli/projects_locations-operations-cancel",
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
                    "Details at http://byron.github.io/google-apis-rs/google_documentai1_cli/projects_locations-operations-get",
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
                    "Details at http://byron.github.io/google-apis-rs/google_documentai1_cli/projects_locations-operations-list",
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
            ("locations-processor-types-get",
                    Some(r##"Gets a processor type detail."##),
                    "Details at http://byron.github.io/google-apis-rs/google_documentai1_cli/projects_locations-processor-types-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The processor type resource name."##),
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
            ("locations-processor-types-list",
                    Some(r##"Lists the processor types that exist."##),
                    "Details at http://byron.github.io/google-apis-rs/google_documentai1_cli/projects_locations-processor-types-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The location of processor types to list. Format: `projects/{project}/locations/{location}`."##),
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
            ("locations-processors-batch-process",
                    Some(r##"LRO endpoint to batch process many documents. The output is written to Cloud Storage as JSON in the [Document] format."##),
                    "Details at http://byron.github.io/google-apis-rs/google_documentai1_cli/projects_locations-processors-batch-process",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name of Processor or ProcessorVersion. Format: `projects/{project}/locations/{location}/processors/{processor}`, or `projects/{project}/locations/{location}/processors/{processor}/processorVersions/{processorVersion}`"##),
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
            ("locations-processors-create",
                    Some(r##"Creates a processor from the ProcessorType provided. The processor will be at `ENABLED` state by default after its creation. Note that this method requires the `documentai.processors.create` permission on the project, which is highly privileged. A user or service account with this permission can create new processors that can interact with any gcs bucket in your project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_documentai1_cli/projects_locations-processors-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent (project and location) under which to create the processor. Format: `projects/{project}/locations/{location}`"##),
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
            ("locations-processors-delete",
                    Some(r##"Deletes the processor, unloads all deployed model artifacts if it was enabled and then deletes all artifacts associated with this processor."##),
                    "Details at http://byron.github.io/google-apis-rs/google_documentai1_cli/projects_locations-processors-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The processor resource name to be deleted."##),
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
            ("locations-processors-disable",
                    Some(r##"Disables a processor"##),
                    "Details at http://byron.github.io/google-apis-rs/google_documentai1_cli/projects_locations-processors-disable",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The processor resource name to be disabled."##),
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
            ("locations-processors-enable",
                    Some(r##"Enables a processor"##),
                    "Details at http://byron.github.io/google-apis-rs/google_documentai1_cli/projects_locations-processors-enable",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The processor resource name to be enabled."##),
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
            ("locations-processors-get",
                    Some(r##"Gets a processor detail."##),
                    "Details at http://byron.github.io/google-apis-rs/google_documentai1_cli/projects_locations-processors-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The processor resource name."##),
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
            ("locations-processors-human-review-config-review-document",
                    Some(r##"Send a document for Human Review. The input document should be processed by the specified processor."##),
                    "Details at http://byron.github.io/google-apis-rs/google_documentai1_cli/projects_locations-processors-human-review-config-review-document",
                  vec![
                    (Some(r##"human-review-config"##),
                     None,
                     Some(r##"Required. The resource name of the HumanReviewConfig that the document will be reviewed with."##),
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
            ("locations-processors-list",
                    Some(r##"Lists all processors which belong to this project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_documentai1_cli/projects_locations-processors-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent (project and location) which owns this collection of Processors. Format: `projects/{project}/locations/{location}`"##),
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
            ("locations-processors-process",
                    Some(r##"Processes a single document."##),
                    "Details at http://byron.github.io/google-apis-rs/google_documentai1_cli/projects_locations-processors-process",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name of the Processor or ProcessorVersion to use for processing. If a Processor is specified, the server will use its default version. Format: `projects/{project}/locations/{location}/processors/{processor}`, or `projects/{project}/locations/{location}/processors/{processor}/processorVersions/{processorVersion}`"##),
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
            ("locations-processors-processor-versions-batch-process",
                    Some(r##"LRO endpoint to batch process many documents. The output is written to Cloud Storage as JSON in the [Document] format."##),
                    "Details at http://byron.github.io/google-apis-rs/google_documentai1_cli/projects_locations-processors-processor-versions-batch-process",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name of Processor or ProcessorVersion. Format: `projects/{project}/locations/{location}/processors/{processor}`, or `projects/{project}/locations/{location}/processors/{processor}/processorVersions/{processorVersion}`"##),
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
            ("locations-processors-processor-versions-delete",
                    Some(r##"Deletes the processor version, all artifacts under the processor version will be deleted."##),
                    "Details at http://byron.github.io/google-apis-rs/google_documentai1_cli/projects_locations-processors-processor-versions-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The processor version resource name to be deleted."##),
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
            ("locations-processors-processor-versions-deploy",
                    Some(r##"Deploys the processor version."##),
                    "Details at http://byron.github.io/google-apis-rs/google_documentai1_cli/projects_locations-processors-processor-versions-deploy",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The processor version resource name to be deployed."##),
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
            ("locations-processors-processor-versions-evaluate-processor-version",
                    Some(r##"Evaluates a ProcessorVersion against annotated documents, producing an Evaluation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_documentai1_cli/projects_locations-processors-processor-versions-evaluate-processor-version",
                  vec![
                    (Some(r##"processor-version"##),
                     None,
                     Some(r##"Required. The resource name of the ProcessorVersion to evaluate. `projects/{project}/locations/{location}/processors/{processor}/processorVersions/{processorVersion}`"##),
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
            ("locations-processors-processor-versions-evaluations-get",
                    Some(r##"Retrieves a specific evaluation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_documentai1_cli/projects_locations-processors-processor-versions-evaluations-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name of the Evaluation to get. `projects/{project}/locations/{location}/processors/{processor}/processorVersions/{processorVersion}/evaluations/{evaluation}`"##),
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
            ("locations-processors-processor-versions-evaluations-list",
                    Some(r##"Retrieves a set of evaluations for a given processor version."##),
                    "Details at http://byron.github.io/google-apis-rs/google_documentai1_cli/projects_locations-processors-processor-versions-evaluations-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The resource name of the ProcessorVersion to list evaluations for. `projects/{project}/locations/{location}/processors/{processor}/processorVersions/{processorVersion}`"##),
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
            ("locations-processors-processor-versions-get",
                    Some(r##"Gets a processor version detail."##),
                    "Details at http://byron.github.io/google-apis-rs/google_documentai1_cli/projects_locations-processors-processor-versions-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The processor resource name."##),
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
            ("locations-processors-processor-versions-list",
                    Some(r##"Lists all versions of a processor."##),
                    "Details at http://byron.github.io/google-apis-rs/google_documentai1_cli/projects_locations-processors-processor-versions-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent (project, location and processor) to list all versions. Format: `projects/{project}/locations/{location}/processors/{processor}`"##),
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
            ("locations-processors-processor-versions-process",
                    Some(r##"Processes a single document."##),
                    "Details at http://byron.github.io/google-apis-rs/google_documentai1_cli/projects_locations-processors-processor-versions-process",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name of the Processor or ProcessorVersion to use for processing. If a Processor is specified, the server will use its default version. Format: `projects/{project}/locations/{location}/processors/{processor}`, or `projects/{project}/locations/{location}/processors/{processor}/processorVersions/{processorVersion}`"##),
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
            ("locations-processors-processor-versions-train",
                    Some(r##"Trains a new processor version. Operation metadata is returned as TrainProcessorVersionMetadata."##),
                    "Details at http://byron.github.io/google-apis-rs/google_documentai1_cli/projects_locations-processors-processor-versions-train",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent (project, location and processor) to create the new version for. Format: `projects/{project}/locations/{location}/processors/{processor}`."##),
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
            ("locations-processors-processor-versions-undeploy",
                    Some(r##"Undeploys the processor version."##),
                    "Details at http://byron.github.io/google-apis-rs/google_documentai1_cli/projects_locations-processors-processor-versions-undeploy",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The processor version resource name to be undeployed."##),
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
            ("locations-processors-set-default-processor-version",
                    Some(r##"Set the default (active) version of a Processor that will be used in ProcessDocument and BatchProcessDocuments."##),
                    "Details at http://byron.github.io/google-apis-rs/google_documentai1_cli/projects_locations-processors-set-default-processor-version",
                  vec![
                    (Some(r##"processor"##),
                     None,
                     Some(r##"Required. The resource name of the Processor to change default version."##),
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
                    "Details at http://byron.github.io/google-apis-rs/google_documentai1_cli/projects_operations-get",
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
        
    ];
    
    let mut app = App::new("documentai1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+20240619")
           .about("Service to parse structured information from unstructured or semi-structured documents using state-of-the-art Google AI such as natural language, computer vision, translation, and AutoML.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_documentai1_cli")
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
