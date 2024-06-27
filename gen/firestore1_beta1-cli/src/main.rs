// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_firestore1_beta1::{api, Error, oauth2, client::chrono, FieldMask};


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
    hub: api::Firestore<S>,
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
    async fn _projects_databases_documents_batch_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "documents" => Some(("documents", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "mask.field-paths" => Some(("mask.fieldPaths", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "new-transaction.read-only.read-time" => Some(("newTransaction.readOnly.readTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "new-transaction.read-write.retry-transaction" => Some(("newTransaction.readWrite.retryTransaction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "read-time" => Some(("readTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transaction" => Some(("transaction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["documents", "field-paths", "mask", "new-transaction", "read-only", "read-time", "read-write", "retry-transaction", "transaction"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::BatchGetDocumentsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().databases_documents_batch_get(request, opt.value_of("database").unwrap_or(""));
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

    async fn _projects_databases_documents_batch_write(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["labels"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::BatchWriteRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().databases_documents_batch_write(request, opt.value_of("database").unwrap_or(""));
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

    async fn _projects_databases_documents_begin_transaction(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "options.read-only.read-time" => Some(("options.readOnly.readTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "options.read-write.retry-transaction" => Some(("options.readWrite.retryTransaction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["options", "read-only", "read-time", "read-write", "retry-transaction"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::BeginTransactionRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().databases_documents_begin_transaction(request, opt.value_of("database").unwrap_or(""));
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

    async fn _projects_databases_documents_commit(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "transaction" => Some(("transaction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["transaction"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CommitRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().databases_documents_commit(request, opt.value_of("database").unwrap_or(""));
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

    async fn _projects_databases_documents_create_document(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["create-time", "name", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Document = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().databases_documents_create_document(request, opt.value_of("parent").unwrap_or(""), opt.value_of("collection-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "mask-field-paths" => {
                    call = call.add_mask_field_paths(value.unwrap_or(""));
                },
                "document-id" => {
                    call = call.document_id(value.unwrap_or(""));
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
                                                                           v.extend(["document-id", "mask-field-paths"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_databases_documents_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().databases_documents_delete(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "current-document-update-time" => {
                    call = call.current_document_update_time(        value.map(|v| arg_from_str(v, err, "current-document-update-time", "google-datetime")).unwrap_or(chrono::Utc::now()));
                },
                "current-document-exists" => {
                    call = call.current_document_exists(        value.map(|v| arg_from_str(v, err, "current-document-exists", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["current-document-exists", "current-document-update-time"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_databases_documents_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().databases_documents_get(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "transaction" => {
                    call = call.transaction(        value.map(|v| arg_from_str(v, err, "transaction", "byte")).unwrap_or(b"hello world"));
                },
                "read-time" => {
                    call = call.read_time(        value.map(|v| arg_from_str(v, err, "read-time", "google-datetime")).unwrap_or(chrono::Utc::now()));
                },
                "mask-field-paths" => {
                    call = call.add_mask_field_paths(value.unwrap_or(""));
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
                                                                           v.extend(["mask-field-paths", "read-time", "transaction"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_databases_documents_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().databases_documents_list(opt.value_of("parent").unwrap_or(""), opt.value_of("collection-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "transaction" => {
                    call = call.transaction(        value.map(|v| arg_from_str(v, err, "transaction", "byte")).unwrap_or(b"hello world"));
                },
                "show-missing" => {
                    call = call.show_missing(        value.map(|v| arg_from_str(v, err, "show-missing", "boolean")).unwrap_or(false));
                },
                "read-time" => {
                    call = call.read_time(        value.map(|v| arg_from_str(v, err, "read-time", "google-datetime")).unwrap_or(chrono::Utc::now()));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "mask-field-paths" => {
                    call = call.add_mask_field_paths(value.unwrap_or(""));
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
                                                                           v.extend(["mask-field-paths", "order-by", "page-size", "page-token", "read-time", "show-missing", "transaction"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_databases_documents_list_collection_ids(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "page-size" => Some(("pageSize", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "page-token" => Some(("pageToken", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "read-time" => Some(("readTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["page-size", "page-token", "read-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ListCollectionIdsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().databases_documents_list_collection_ids(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_databases_documents_list_documents(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().databases_documents_list_documents(opt.value_of("parent").unwrap_or(""), opt.value_of("collection-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "transaction" => {
                    call = call.transaction(        value.map(|v| arg_from_str(v, err, "transaction", "byte")).unwrap_or(b"hello world"));
                },
                "show-missing" => {
                    call = call.show_missing(        value.map(|v| arg_from_str(v, err, "show-missing", "boolean")).unwrap_or(false));
                },
                "read-time" => {
                    call = call.read_time(        value.map(|v| arg_from_str(v, err, "read-time", "google-datetime")).unwrap_or(chrono::Utc::now()));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "mask-field-paths" => {
                    call = call.add_mask_field_paths(value.unwrap_or(""));
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
                                                                           v.extend(["mask-field-paths", "order-by", "page-size", "page-token", "read-time", "show-missing", "transaction"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_databases_documents_listen(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "add-target.documents.documents" => Some(("addTarget.documents.documents", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "add-target.expected-count" => Some(("addTarget.expectedCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "add-target.once" => Some(("addTarget.once", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "add-target.query.parent" => Some(("addTarget.query.parent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "add-target.query.structured-query.end-at.before" => Some(("addTarget.query.structuredQuery.endAt.before", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "add-target.query.structured-query.find-nearest.distance-measure" => Some(("addTarget.query.structuredQuery.findNearest.distanceMeasure", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "add-target.query.structured-query.find-nearest.limit" => Some(("addTarget.query.structuredQuery.findNearest.limit", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "add-target.query.structured-query.find-nearest.query-vector.boolean-value" => Some(("addTarget.query.structuredQuery.findNearest.queryVector.booleanValue", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "add-target.query.structured-query.find-nearest.query-vector.bytes-value" => Some(("addTarget.query.structuredQuery.findNearest.queryVector.bytesValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "add-target.query.structured-query.find-nearest.query-vector.double-value" => Some(("addTarget.query.structuredQuery.findNearest.queryVector.doubleValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "add-target.query.structured-query.find-nearest.query-vector.geo-point-value.latitude" => Some(("addTarget.query.structuredQuery.findNearest.queryVector.geoPointValue.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "add-target.query.structured-query.find-nearest.query-vector.geo-point-value.longitude" => Some(("addTarget.query.structuredQuery.findNearest.queryVector.geoPointValue.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "add-target.query.structured-query.find-nearest.query-vector.integer-value" => Some(("addTarget.query.structuredQuery.findNearest.queryVector.integerValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "add-target.query.structured-query.find-nearest.query-vector.null-value" => Some(("addTarget.query.structuredQuery.findNearest.queryVector.nullValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "add-target.query.structured-query.find-nearest.query-vector.reference-value" => Some(("addTarget.query.structuredQuery.findNearest.queryVector.referenceValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "add-target.query.structured-query.find-nearest.query-vector.string-value" => Some(("addTarget.query.structuredQuery.findNearest.queryVector.stringValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "add-target.query.structured-query.find-nearest.query-vector.timestamp-value" => Some(("addTarget.query.structuredQuery.findNearest.queryVector.timestampValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "add-target.query.structured-query.find-nearest.vector-field.field-path" => Some(("addTarget.query.structuredQuery.findNearest.vectorField.fieldPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "add-target.query.structured-query.limit" => Some(("addTarget.query.structuredQuery.limit", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "add-target.query.structured-query.offset" => Some(("addTarget.query.structuredQuery.offset", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "add-target.query.structured-query.start-at.before" => Some(("addTarget.query.structuredQuery.startAt.before", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "add-target.query.structured-query.where.composite-filter.op" => Some(("addTarget.query.structuredQuery.where.compositeFilter.op", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "add-target.query.structured-query.where.field-filter.field.field-path" => Some(("addTarget.query.structuredQuery.where.fieldFilter.field.fieldPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "add-target.query.structured-query.where.field-filter.op" => Some(("addTarget.query.structuredQuery.where.fieldFilter.op", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "add-target.query.structured-query.where.field-filter.value.boolean-value" => Some(("addTarget.query.structuredQuery.where.fieldFilter.value.booleanValue", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "add-target.query.structured-query.where.field-filter.value.bytes-value" => Some(("addTarget.query.structuredQuery.where.fieldFilter.value.bytesValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "add-target.query.structured-query.where.field-filter.value.double-value" => Some(("addTarget.query.structuredQuery.where.fieldFilter.value.doubleValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "add-target.query.structured-query.where.field-filter.value.geo-point-value.latitude" => Some(("addTarget.query.structuredQuery.where.fieldFilter.value.geoPointValue.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "add-target.query.structured-query.where.field-filter.value.geo-point-value.longitude" => Some(("addTarget.query.structuredQuery.where.fieldFilter.value.geoPointValue.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "add-target.query.structured-query.where.field-filter.value.integer-value" => Some(("addTarget.query.structuredQuery.where.fieldFilter.value.integerValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "add-target.query.structured-query.where.field-filter.value.null-value" => Some(("addTarget.query.structuredQuery.where.fieldFilter.value.nullValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "add-target.query.structured-query.where.field-filter.value.reference-value" => Some(("addTarget.query.structuredQuery.where.fieldFilter.value.referenceValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "add-target.query.structured-query.where.field-filter.value.string-value" => Some(("addTarget.query.structuredQuery.where.fieldFilter.value.stringValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "add-target.query.structured-query.where.field-filter.value.timestamp-value" => Some(("addTarget.query.structuredQuery.where.fieldFilter.value.timestampValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "add-target.query.structured-query.where.unary-filter.field.field-path" => Some(("addTarget.query.structuredQuery.where.unaryFilter.field.fieldPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "add-target.query.structured-query.where.unary-filter.op" => Some(("addTarget.query.structuredQuery.where.unaryFilter.op", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "add-target.read-time" => Some(("addTarget.readTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "add-target.resume-token" => Some(("addTarget.resumeToken", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "add-target.target-id" => Some(("addTarget.targetId", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "remove-target" => Some(("removeTarget", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["add-target", "before", "boolean-value", "bytes-value", "composite-filter", "distance-measure", "documents", "double-value", "end-at", "expected-count", "field", "field-filter", "field-path", "find-nearest", "geo-point-value", "integer-value", "labels", "latitude", "limit", "longitude", "null-value", "offset", "once", "op", "parent", "query", "query-vector", "read-time", "reference-value", "remove-target", "resume-token", "start-at", "string-value", "structured-query", "target-id", "timestamp-value", "unary-filter", "value", "vector-field", "where"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ListenRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().databases_documents_listen(request, opt.value_of("database").unwrap_or(""));
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

    async fn _projects_databases_documents_partition_query(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "page-size" => Some(("pageSize", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "page-token" => Some(("pageToken", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "partition-count" => Some(("partitionCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "read-time" => Some(("readTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-query.end-at.before" => Some(("structuredQuery.endAt.before", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "structured-query.find-nearest.distance-measure" => Some(("structuredQuery.findNearest.distanceMeasure", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-query.find-nearest.limit" => Some(("structuredQuery.findNearest.limit", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "structured-query.find-nearest.query-vector.boolean-value" => Some(("structuredQuery.findNearest.queryVector.booleanValue", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "structured-query.find-nearest.query-vector.bytes-value" => Some(("structuredQuery.findNearest.queryVector.bytesValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-query.find-nearest.query-vector.double-value" => Some(("structuredQuery.findNearest.queryVector.doubleValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "structured-query.find-nearest.query-vector.geo-point-value.latitude" => Some(("structuredQuery.findNearest.queryVector.geoPointValue.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "structured-query.find-nearest.query-vector.geo-point-value.longitude" => Some(("structuredQuery.findNearest.queryVector.geoPointValue.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "structured-query.find-nearest.query-vector.integer-value" => Some(("structuredQuery.findNearest.queryVector.integerValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-query.find-nearest.query-vector.null-value" => Some(("structuredQuery.findNearest.queryVector.nullValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-query.find-nearest.query-vector.reference-value" => Some(("structuredQuery.findNearest.queryVector.referenceValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-query.find-nearest.query-vector.string-value" => Some(("structuredQuery.findNearest.queryVector.stringValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-query.find-nearest.query-vector.timestamp-value" => Some(("structuredQuery.findNearest.queryVector.timestampValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-query.find-nearest.vector-field.field-path" => Some(("structuredQuery.findNearest.vectorField.fieldPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-query.limit" => Some(("structuredQuery.limit", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "structured-query.offset" => Some(("structuredQuery.offset", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "structured-query.start-at.before" => Some(("structuredQuery.startAt.before", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "structured-query.where.composite-filter.op" => Some(("structuredQuery.where.compositeFilter.op", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-query.where.field-filter.field.field-path" => Some(("structuredQuery.where.fieldFilter.field.fieldPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-query.where.field-filter.op" => Some(("structuredQuery.where.fieldFilter.op", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-query.where.field-filter.value.boolean-value" => Some(("structuredQuery.where.fieldFilter.value.booleanValue", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "structured-query.where.field-filter.value.bytes-value" => Some(("structuredQuery.where.fieldFilter.value.bytesValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-query.where.field-filter.value.double-value" => Some(("structuredQuery.where.fieldFilter.value.doubleValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "structured-query.where.field-filter.value.geo-point-value.latitude" => Some(("structuredQuery.where.fieldFilter.value.geoPointValue.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "structured-query.where.field-filter.value.geo-point-value.longitude" => Some(("structuredQuery.where.fieldFilter.value.geoPointValue.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "structured-query.where.field-filter.value.integer-value" => Some(("structuredQuery.where.fieldFilter.value.integerValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-query.where.field-filter.value.null-value" => Some(("structuredQuery.where.fieldFilter.value.nullValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-query.where.field-filter.value.reference-value" => Some(("structuredQuery.where.fieldFilter.value.referenceValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-query.where.field-filter.value.string-value" => Some(("structuredQuery.where.fieldFilter.value.stringValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-query.where.field-filter.value.timestamp-value" => Some(("structuredQuery.where.fieldFilter.value.timestampValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-query.where.unary-filter.field.field-path" => Some(("structuredQuery.where.unaryFilter.field.fieldPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-query.where.unary-filter.op" => Some(("structuredQuery.where.unaryFilter.op", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["before", "boolean-value", "bytes-value", "composite-filter", "distance-measure", "double-value", "end-at", "field", "field-filter", "field-path", "find-nearest", "geo-point-value", "integer-value", "latitude", "limit", "longitude", "null-value", "offset", "op", "page-size", "page-token", "partition-count", "query-vector", "read-time", "reference-value", "start-at", "string-value", "structured-query", "timestamp-value", "unary-filter", "value", "vector-field", "where"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::PartitionQueryRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().databases_documents_partition_query(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_databases_documents_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["create-time", "name", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Document = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().databases_documents_patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask-field-paths" => {
                    call = call.add_update_mask_field_paths(value.unwrap_or(""));
                },
                "mask-field-paths" => {
                    call = call.add_mask_field_paths(value.unwrap_or(""));
                },
                "current-document-update-time" => {
                    call = call.current_document_update_time(        value.map(|v| arg_from_str(v, err, "current-document-update-time", "google-datetime")).unwrap_or(chrono::Utc::now()));
                },
                "current-document-exists" => {
                    call = call.current_document_exists(        value.map(|v| arg_from_str(v, err, "current-document-exists", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["current-document-exists", "current-document-update-time", "mask-field-paths", "update-mask-field-paths"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_databases_documents_rollback(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "transaction" => Some(("transaction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["transaction"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::RollbackRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().databases_documents_rollback(request, opt.value_of("database").unwrap_or(""));
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

    async fn _projects_databases_documents_run_aggregation_query(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "explain-options.analyze" => Some(("explainOptions.analyze", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "new-transaction.read-only.read-time" => Some(("newTransaction.readOnly.readTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "new-transaction.read-write.retry-transaction" => Some(("newTransaction.readWrite.retryTransaction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "read-time" => Some(("readTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-aggregation-query.structured-query.end-at.before" => Some(("structuredAggregationQuery.structuredQuery.endAt.before", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "structured-aggregation-query.structured-query.find-nearest.distance-measure" => Some(("structuredAggregationQuery.structuredQuery.findNearest.distanceMeasure", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-aggregation-query.structured-query.find-nearest.limit" => Some(("structuredAggregationQuery.structuredQuery.findNearest.limit", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "structured-aggregation-query.structured-query.find-nearest.query-vector.boolean-value" => Some(("structuredAggregationQuery.structuredQuery.findNearest.queryVector.booleanValue", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "structured-aggregation-query.structured-query.find-nearest.query-vector.bytes-value" => Some(("structuredAggregationQuery.structuredQuery.findNearest.queryVector.bytesValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-aggregation-query.structured-query.find-nearest.query-vector.double-value" => Some(("structuredAggregationQuery.structuredQuery.findNearest.queryVector.doubleValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "structured-aggregation-query.structured-query.find-nearest.query-vector.geo-point-value.latitude" => Some(("structuredAggregationQuery.structuredQuery.findNearest.queryVector.geoPointValue.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "structured-aggregation-query.structured-query.find-nearest.query-vector.geo-point-value.longitude" => Some(("structuredAggregationQuery.structuredQuery.findNearest.queryVector.geoPointValue.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "structured-aggregation-query.structured-query.find-nearest.query-vector.integer-value" => Some(("structuredAggregationQuery.structuredQuery.findNearest.queryVector.integerValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-aggregation-query.structured-query.find-nearest.query-vector.null-value" => Some(("structuredAggregationQuery.structuredQuery.findNearest.queryVector.nullValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-aggregation-query.structured-query.find-nearest.query-vector.reference-value" => Some(("structuredAggregationQuery.structuredQuery.findNearest.queryVector.referenceValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-aggregation-query.structured-query.find-nearest.query-vector.string-value" => Some(("structuredAggregationQuery.structuredQuery.findNearest.queryVector.stringValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-aggregation-query.structured-query.find-nearest.query-vector.timestamp-value" => Some(("structuredAggregationQuery.structuredQuery.findNearest.queryVector.timestampValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-aggregation-query.structured-query.find-nearest.vector-field.field-path" => Some(("structuredAggregationQuery.structuredQuery.findNearest.vectorField.fieldPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-aggregation-query.structured-query.limit" => Some(("structuredAggregationQuery.structuredQuery.limit", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "structured-aggregation-query.structured-query.offset" => Some(("structuredAggregationQuery.structuredQuery.offset", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "structured-aggregation-query.structured-query.start-at.before" => Some(("structuredAggregationQuery.structuredQuery.startAt.before", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "structured-aggregation-query.structured-query.where.composite-filter.op" => Some(("structuredAggregationQuery.structuredQuery.where.compositeFilter.op", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-aggregation-query.structured-query.where.field-filter.field.field-path" => Some(("structuredAggregationQuery.structuredQuery.where.fieldFilter.field.fieldPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-aggregation-query.structured-query.where.field-filter.op" => Some(("structuredAggregationQuery.structuredQuery.where.fieldFilter.op", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-aggregation-query.structured-query.where.field-filter.value.boolean-value" => Some(("structuredAggregationQuery.structuredQuery.where.fieldFilter.value.booleanValue", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "structured-aggregation-query.structured-query.where.field-filter.value.bytes-value" => Some(("structuredAggregationQuery.structuredQuery.where.fieldFilter.value.bytesValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-aggregation-query.structured-query.where.field-filter.value.double-value" => Some(("structuredAggregationQuery.structuredQuery.where.fieldFilter.value.doubleValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "structured-aggregation-query.structured-query.where.field-filter.value.geo-point-value.latitude" => Some(("structuredAggregationQuery.structuredQuery.where.fieldFilter.value.geoPointValue.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "structured-aggregation-query.structured-query.where.field-filter.value.geo-point-value.longitude" => Some(("structuredAggregationQuery.structuredQuery.where.fieldFilter.value.geoPointValue.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "structured-aggregation-query.structured-query.where.field-filter.value.integer-value" => Some(("structuredAggregationQuery.structuredQuery.where.fieldFilter.value.integerValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-aggregation-query.structured-query.where.field-filter.value.null-value" => Some(("structuredAggregationQuery.structuredQuery.where.fieldFilter.value.nullValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-aggregation-query.structured-query.where.field-filter.value.reference-value" => Some(("structuredAggregationQuery.structuredQuery.where.fieldFilter.value.referenceValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-aggregation-query.structured-query.where.field-filter.value.string-value" => Some(("structuredAggregationQuery.structuredQuery.where.fieldFilter.value.stringValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-aggregation-query.structured-query.where.field-filter.value.timestamp-value" => Some(("structuredAggregationQuery.structuredQuery.where.fieldFilter.value.timestampValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-aggregation-query.structured-query.where.unary-filter.field.field-path" => Some(("structuredAggregationQuery.structuredQuery.where.unaryFilter.field.fieldPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-aggregation-query.structured-query.where.unary-filter.op" => Some(("structuredAggregationQuery.structuredQuery.where.unaryFilter.op", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transaction" => Some(("transaction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["analyze", "before", "boolean-value", "bytes-value", "composite-filter", "distance-measure", "double-value", "end-at", "explain-options", "field", "field-filter", "field-path", "find-nearest", "geo-point-value", "integer-value", "latitude", "limit", "longitude", "new-transaction", "null-value", "offset", "op", "query-vector", "read-only", "read-time", "read-write", "reference-value", "retry-transaction", "start-at", "string-value", "structured-aggregation-query", "structured-query", "timestamp-value", "transaction", "unary-filter", "value", "vector-field", "where"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::RunAggregationQueryRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().databases_documents_run_aggregation_query(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_databases_documents_run_query(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "explain-options.analyze" => Some(("explainOptions.analyze", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "new-transaction.read-only.read-time" => Some(("newTransaction.readOnly.readTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "new-transaction.read-write.retry-transaction" => Some(("newTransaction.readWrite.retryTransaction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "read-time" => Some(("readTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-query.end-at.before" => Some(("structuredQuery.endAt.before", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "structured-query.find-nearest.distance-measure" => Some(("structuredQuery.findNearest.distanceMeasure", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-query.find-nearest.limit" => Some(("structuredQuery.findNearest.limit", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "structured-query.find-nearest.query-vector.boolean-value" => Some(("structuredQuery.findNearest.queryVector.booleanValue", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "structured-query.find-nearest.query-vector.bytes-value" => Some(("structuredQuery.findNearest.queryVector.bytesValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-query.find-nearest.query-vector.double-value" => Some(("structuredQuery.findNearest.queryVector.doubleValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "structured-query.find-nearest.query-vector.geo-point-value.latitude" => Some(("structuredQuery.findNearest.queryVector.geoPointValue.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "structured-query.find-nearest.query-vector.geo-point-value.longitude" => Some(("structuredQuery.findNearest.queryVector.geoPointValue.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "structured-query.find-nearest.query-vector.integer-value" => Some(("structuredQuery.findNearest.queryVector.integerValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-query.find-nearest.query-vector.null-value" => Some(("structuredQuery.findNearest.queryVector.nullValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-query.find-nearest.query-vector.reference-value" => Some(("structuredQuery.findNearest.queryVector.referenceValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-query.find-nearest.query-vector.string-value" => Some(("structuredQuery.findNearest.queryVector.stringValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-query.find-nearest.query-vector.timestamp-value" => Some(("structuredQuery.findNearest.queryVector.timestampValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-query.find-nearest.vector-field.field-path" => Some(("structuredQuery.findNearest.vectorField.fieldPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-query.limit" => Some(("structuredQuery.limit", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "structured-query.offset" => Some(("structuredQuery.offset", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "structured-query.start-at.before" => Some(("structuredQuery.startAt.before", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "structured-query.where.composite-filter.op" => Some(("structuredQuery.where.compositeFilter.op", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-query.where.field-filter.field.field-path" => Some(("structuredQuery.where.fieldFilter.field.fieldPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-query.where.field-filter.op" => Some(("structuredQuery.where.fieldFilter.op", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-query.where.field-filter.value.boolean-value" => Some(("structuredQuery.where.fieldFilter.value.booleanValue", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "structured-query.where.field-filter.value.bytes-value" => Some(("structuredQuery.where.fieldFilter.value.bytesValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-query.where.field-filter.value.double-value" => Some(("structuredQuery.where.fieldFilter.value.doubleValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "structured-query.where.field-filter.value.geo-point-value.latitude" => Some(("structuredQuery.where.fieldFilter.value.geoPointValue.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "structured-query.where.field-filter.value.geo-point-value.longitude" => Some(("structuredQuery.where.fieldFilter.value.geoPointValue.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "structured-query.where.field-filter.value.integer-value" => Some(("structuredQuery.where.fieldFilter.value.integerValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-query.where.field-filter.value.null-value" => Some(("structuredQuery.where.fieldFilter.value.nullValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-query.where.field-filter.value.reference-value" => Some(("structuredQuery.where.fieldFilter.value.referenceValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-query.where.field-filter.value.string-value" => Some(("structuredQuery.where.fieldFilter.value.stringValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-query.where.field-filter.value.timestamp-value" => Some(("structuredQuery.where.fieldFilter.value.timestampValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-query.where.unary-filter.field.field-path" => Some(("structuredQuery.where.unaryFilter.field.fieldPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "structured-query.where.unary-filter.op" => Some(("structuredQuery.where.unaryFilter.op", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "transaction" => Some(("transaction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["analyze", "before", "boolean-value", "bytes-value", "composite-filter", "distance-measure", "double-value", "end-at", "explain-options", "field", "field-filter", "field-path", "find-nearest", "geo-point-value", "integer-value", "latitude", "limit", "longitude", "new-transaction", "null-value", "offset", "op", "query-vector", "read-only", "read-time", "read-write", "reference-value", "retry-transaction", "start-at", "string-value", "structured-query", "timestamp-value", "transaction", "unary-filter", "value", "vector-field", "where"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::RunQueryRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().databases_documents_run_query(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_databases_documents_write(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "stream-id" => Some(("streamId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "stream-token" => Some(("streamToken", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["labels", "stream-id", "stream-token"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::WriteRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().databases_documents_write(request, opt.value_of("database").unwrap_or(""));
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

    async fn _projects_databases_export_documents(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "collection-ids" => Some(("collectionIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "output-uri-prefix" => Some(("outputUriPrefix", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["collection-ids", "output-uri-prefix"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleFirestoreAdminV1beta1ExportDocumentsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().databases_export_documents(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_databases_import_documents(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "collection-ids" => Some(("collectionIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "input-uri-prefix" => Some(("inputUriPrefix", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["collection-ids", "input-uri-prefix"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleFirestoreAdminV1beta1ImportDocumentsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().databases_import_documents(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_databases_indexes_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "collection-id" => Some(("collectionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["collection-id", "name", "state"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GoogleFirestoreAdminV1beta1Index = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().databases_indexes_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_databases_indexes_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().databases_indexes_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_databases_indexes_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().databases_indexes_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_databases_indexes_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().databases_indexes_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _doit(&self, dry_run: bool) -> Result<Result<(), DoitError>, Option<InvalidOptionsError>> {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Result<(), DoitError> = Ok(());
        let mut err_opt: Option<InvalidOptionsError> = None;
        match self.opt.subcommand() {
            ("projects", Some(opt)) => {
                match opt.subcommand() {
                    ("databases-documents-batch-get", Some(opt)) => {
                        call_result = self._projects_databases_documents_batch_get(opt, dry_run, &mut err).await;
                    },
                    ("databases-documents-batch-write", Some(opt)) => {
                        call_result = self._projects_databases_documents_batch_write(opt, dry_run, &mut err).await;
                    },
                    ("databases-documents-begin-transaction", Some(opt)) => {
                        call_result = self._projects_databases_documents_begin_transaction(opt, dry_run, &mut err).await;
                    },
                    ("databases-documents-commit", Some(opt)) => {
                        call_result = self._projects_databases_documents_commit(opt, dry_run, &mut err).await;
                    },
                    ("databases-documents-create-document", Some(opt)) => {
                        call_result = self._projects_databases_documents_create_document(opt, dry_run, &mut err).await;
                    },
                    ("databases-documents-delete", Some(opt)) => {
                        call_result = self._projects_databases_documents_delete(opt, dry_run, &mut err).await;
                    },
                    ("databases-documents-get", Some(opt)) => {
                        call_result = self._projects_databases_documents_get(opt, dry_run, &mut err).await;
                    },
                    ("databases-documents-list", Some(opt)) => {
                        call_result = self._projects_databases_documents_list(opt, dry_run, &mut err).await;
                    },
                    ("databases-documents-list-collection-ids", Some(opt)) => {
                        call_result = self._projects_databases_documents_list_collection_ids(opt, dry_run, &mut err).await;
                    },
                    ("databases-documents-list-documents", Some(opt)) => {
                        call_result = self._projects_databases_documents_list_documents(opt, dry_run, &mut err).await;
                    },
                    ("databases-documents-listen", Some(opt)) => {
                        call_result = self._projects_databases_documents_listen(opt, dry_run, &mut err).await;
                    },
                    ("databases-documents-partition-query", Some(opt)) => {
                        call_result = self._projects_databases_documents_partition_query(opt, dry_run, &mut err).await;
                    },
                    ("databases-documents-patch", Some(opt)) => {
                        call_result = self._projects_databases_documents_patch(opt, dry_run, &mut err).await;
                    },
                    ("databases-documents-rollback", Some(opt)) => {
                        call_result = self._projects_databases_documents_rollback(opt, dry_run, &mut err).await;
                    },
                    ("databases-documents-run-aggregation-query", Some(opt)) => {
                        call_result = self._projects_databases_documents_run_aggregation_query(opt, dry_run, &mut err).await;
                    },
                    ("databases-documents-run-query", Some(opt)) => {
                        call_result = self._projects_databases_documents_run_query(opt, dry_run, &mut err).await;
                    },
                    ("databases-documents-write", Some(opt)) => {
                        call_result = self._projects_databases_documents_write(opt, dry_run, &mut err).await;
                    },
                    ("databases-export-documents", Some(opt)) => {
                        call_result = self._projects_databases_export_documents(opt, dry_run, &mut err).await;
                    },
                    ("databases-import-documents", Some(opt)) => {
                        call_result = self._projects_databases_import_documents(opt, dry_run, &mut err).await;
                    },
                    ("databases-indexes-create", Some(opt)) => {
                        call_result = self._projects_databases_indexes_create(opt, dry_run, &mut err).await;
                    },
                    ("databases-indexes-delete", Some(opt)) => {
                        call_result = self._projects_databases_indexes_delete(opt, dry_run, &mut err).await;
                    },
                    ("databases-indexes-get", Some(opt)) => {
                        call_result = self._projects_databases_indexes_get(opt, dry_run, &mut err).await;
                    },
                    ("databases-indexes-list", Some(opt)) => {
                        call_result = self._projects_databases_indexes_list(opt, dry_run, &mut err).await;
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

            match client::application_secret_from_directory(&config_dir, "firestore1-beta1-secret.json",
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
        ).persist_tokens_to_disk(format!("{}/firestore1-beta1", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::Firestore::new(client, auth),
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
        ("projects", "methods: 'databases-documents-batch-get', 'databases-documents-batch-write', 'databases-documents-begin-transaction', 'databases-documents-commit', 'databases-documents-create-document', 'databases-documents-delete', 'databases-documents-get', 'databases-documents-list', 'databases-documents-list-collection-ids', 'databases-documents-list-documents', 'databases-documents-listen', 'databases-documents-partition-query', 'databases-documents-patch', 'databases-documents-rollback', 'databases-documents-run-aggregation-query', 'databases-documents-run-query', 'databases-documents-write', 'databases-export-documents', 'databases-import-documents', 'databases-indexes-create', 'databases-indexes-delete', 'databases-indexes-get' and 'databases-indexes-list'", vec![
            ("databases-documents-batch-get",
                    Some(r##"Gets multiple documents. Documents returned by this method are not guaranteed to be returned in the same order that they were requested."##),
                    "Details at http://byron.github.io/google-apis-rs/google_firestore1_beta1_cli/projects_databases-documents-batch-get",
                  vec![
                    (Some(r##"database"##),
                     None,
                     Some(r##"Required. The database name. In the format: `projects/{project_id}/databases/{database_id}`."##),
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
            ("databases-documents-batch-write",
                    Some(r##"Applies a batch of write operations. The BatchWrite method does not apply the write operations atomically and can apply them out of order. Method does not allow more than one write per document. Each write succeeds or fails independently. See the BatchWriteResponse for the success status of each write. If you require an atomically applied set of writes, use Commit instead."##),
                    "Details at http://byron.github.io/google-apis-rs/google_firestore1_beta1_cli/projects_databases-documents-batch-write",
                  vec![
                    (Some(r##"database"##),
                     None,
                     Some(r##"Required. The database name. In the format: `projects/{project_id}/databases/{database_id}`."##),
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
            ("databases-documents-begin-transaction",
                    Some(r##"Starts a new transaction."##),
                    "Details at http://byron.github.io/google-apis-rs/google_firestore1_beta1_cli/projects_databases-documents-begin-transaction",
                  vec![
                    (Some(r##"database"##),
                     None,
                     Some(r##"Required. The database name. In the format: `projects/{project_id}/databases/{database_id}`."##),
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
            ("databases-documents-commit",
                    Some(r##"Commits a transaction, while optionally updating documents."##),
                    "Details at http://byron.github.io/google-apis-rs/google_firestore1_beta1_cli/projects_databases-documents-commit",
                  vec![
                    (Some(r##"database"##),
                     None,
                     Some(r##"Required. The database name. In the format: `projects/{project_id}/databases/{database_id}`."##),
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
            ("databases-documents-create-document",
                    Some(r##"Creates a new document."##),
                    "Details at http://byron.github.io/google-apis-rs/google_firestore1_beta1_cli/projects_databases-documents-create-document",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource. For example: `projects/{project_id}/databases/{database_id}/documents` or `projects/{project_id}/databases/{database_id}/documents/chatrooms/{chatroom_id}`"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"collection-id"##),
                     None,
                     Some(r##"Required. The collection ID, relative to `parent`, to list. For example: `chatrooms`."##),
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
            ("databases-documents-delete",
                    Some(r##"Deletes a document."##),
                    "Details at http://byron.github.io/google-apis-rs/google_firestore1_beta1_cli/projects_databases-documents-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name of the Document to delete. In the format: `projects/{project_id}/databases/{database_id}/documents/{document_path}`."##),
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
            ("databases-documents-get",
                    Some(r##"Gets a single document."##),
                    "Details at http://byron.github.io/google-apis-rs/google_firestore1_beta1_cli/projects_databases-documents-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name of the Document to get. In the format: `projects/{project_id}/databases/{database_id}/documents/{document_path}`."##),
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
            ("databases-documents-list",
                    Some(r##"Lists documents."##),
                    "Details at http://byron.github.io/google-apis-rs/google_firestore1_beta1_cli/projects_databases-documents-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource name. In the format: `projects/{project_id}/databases/{database_id}/documents` or `projects/{project_id}/databases/{database_id}/documents/{document_path}`. For example: `projects/my-project/databases/my-database/documents` or `projects/my-project/databases/my-database/documents/chatrooms/my-chatroom`"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"collection-id"##),
                     None,
                     Some(r##"Optional. The collection ID, relative to `parent`, to list. For example: `chatrooms` or `messages`. This is optional, and when not provided, Firestore will list documents from all collections under the provided `parent`."##),
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
            ("databases-documents-list-collection-ids",
                    Some(r##"Lists all the collection IDs underneath a document."##),
                    "Details at http://byron.github.io/google-apis-rs/google_firestore1_beta1_cli/projects_databases-documents-list-collection-ids",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent document. In the format: `projects/{project_id}/databases/{database_id}/documents/{document_path}`. For example: `projects/my-project/databases/my-database/documents/chatrooms/my-chatroom`"##),
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
            ("databases-documents-list-documents",
                    Some(r##"Lists documents."##),
                    "Details at http://byron.github.io/google-apis-rs/google_firestore1_beta1_cli/projects_databases-documents-list-documents",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource name. In the format: `projects/{project_id}/databases/{database_id}/documents` or `projects/{project_id}/databases/{database_id}/documents/{document_path}`. For example: `projects/my-project/databases/my-database/documents` or `projects/my-project/databases/my-database/documents/chatrooms/my-chatroom`"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"collection-id"##),
                     None,
                     Some(r##"Optional. The collection ID, relative to `parent`, to list. For example: `chatrooms` or `messages`. This is optional, and when not provided, Firestore will list documents from all collections under the provided `parent`."##),
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
            ("databases-documents-listen",
                    Some(r##"Listens to changes. This method is only available via gRPC or WebChannel (not REST)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_firestore1_beta1_cli/projects_databases-documents-listen",
                  vec![
                    (Some(r##"database"##),
                     None,
                     Some(r##"Required. The database name. In the format: `projects/{project_id}/databases/{database_id}`."##),
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
            ("databases-documents-partition-query",
                    Some(r##"Partitions a query by returning partition cursors that can be used to run the query in parallel. The returned partition cursors are split points that can be used by RunQuery as starting/end points for the query results."##),
                    "Details at http://byron.github.io/google-apis-rs/google_firestore1_beta1_cli/projects_databases-documents-partition-query",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource name. In the format: `projects/{project_id}/databases/{database_id}/documents`. Document resource names are not supported; only database resource names can be specified."##),
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
            ("databases-documents-patch",
                    Some(r##"Updates or inserts a document."##),
                    "Details at http://byron.github.io/google-apis-rs/google_firestore1_beta1_cli/projects_databases-documents-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The resource name of the document, for example `projects/{project_id}/databases/{database_id}/documents/{document_path}`."##),
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
            ("databases-documents-rollback",
                    Some(r##"Rolls back a transaction."##),
                    "Details at http://byron.github.io/google-apis-rs/google_firestore1_beta1_cli/projects_databases-documents-rollback",
                  vec![
                    (Some(r##"database"##),
                     None,
                     Some(r##"Required. The database name. In the format: `projects/{project_id}/databases/{database_id}`."##),
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
            ("databases-documents-run-aggregation-query",
                    Some(r##"Runs an aggregation query. Rather than producing Document results like Firestore.RunQuery, this API allows running an aggregation to produce a series of AggregationResult server-side. High-Level Example: ``` -- Return the number of documents in table given a filter. SELECT COUNT(*) FROM ( SELECT * FROM k where a = true ); ```"##),
                    "Details at http://byron.github.io/google-apis-rs/google_firestore1_beta1_cli/projects_databases-documents-run-aggregation-query",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource name. In the format: `projects/{project_id}/databases/{database_id}/documents` or `projects/{project_id}/databases/{database_id}/documents/{document_path}`. For example: `projects/my-project/databases/my-database/documents` or `projects/my-project/databases/my-database/documents/chatrooms/my-chatroom`"##),
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
            ("databases-documents-run-query",
                    Some(r##"Runs a query."##),
                    "Details at http://byron.github.io/google-apis-rs/google_firestore1_beta1_cli/projects_databases-documents-run-query",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource name. In the format: `projects/{project_id}/databases/{database_id}/documents` or `projects/{project_id}/databases/{database_id}/documents/{document_path}`. For example: `projects/my-project/databases/my-database/documents` or `projects/my-project/databases/my-database/documents/chatrooms/my-chatroom`"##),
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
            ("databases-documents-write",
                    Some(r##"Streams batches of document updates and deletes, in order. This method is only available via gRPC or WebChannel (not REST)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_firestore1_beta1_cli/projects_databases-documents-write",
                  vec![
                    (Some(r##"database"##),
                     None,
                     Some(r##"Required. The database name. In the format: `projects/{project_id}/databases/{database_id}`. This is only required in the first message."##),
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
            ("databases-export-documents",
                    Some(r##"Exports a copy of all or a subset of documents from Google Cloud Firestore to another storage system, such as Google Cloud Storage. Recent updates to documents may not be reflected in the export. The export occurs in the background and its progress can be monitored and managed via the Operation resource that is created. The output of an export may only be used once the associated operation is done. If an export operation is cancelled before completion it may leave partial data behind in Google Cloud Storage."##),
                    "Details at http://byron.github.io/google-apis-rs/google_firestore1_beta1_cli/projects_databases-export-documents",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Database to export. Should be of the form: `projects/{project_id}/databases/{database_id}`."##),
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
            ("databases-import-documents",
                    Some(r##"Imports documents into Google Cloud Firestore. Existing documents with the same name are overwritten. The import occurs in the background and its progress can be monitored and managed via the Operation resource that is created. If an ImportDocuments operation is cancelled, it is possible that a subset of the data has already been imported to Cloud Firestore."##),
                    "Details at http://byron.github.io/google-apis-rs/google_firestore1_beta1_cli/projects_databases-import-documents",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Database to import into. Should be of the form: `projects/{project_id}/databases/{database_id}`."##),
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
            ("databases-indexes-create",
                    Some(r##"Creates the specified index. A newly created index's initial state is `CREATING`. On completion of the returned google.longrunning.Operation, the state will be `READY`. If the index already exists, the call will return an `ALREADY_EXISTS` status. During creation, the process could result in an error, in which case the index will move to the `ERROR` state. The process can be recovered by fixing the data that caused the error, removing the index with delete, then re-creating the index with create. Indexes with a single field cannot be created."##),
                    "Details at http://byron.github.io/google-apis-rs/google_firestore1_beta1_cli/projects_databases-indexes-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The name of the database this index will apply to. For example: `projects/{project_id}/databases/{database_id}`"##),
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
            ("databases-indexes-delete",
                    Some(r##"Deletes an index."##),
                    "Details at http://byron.github.io/google-apis-rs/google_firestore1_beta1_cli/projects_databases-indexes-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The index name. For example: `projects/{project_id}/databases/{database_id}/indexes/{index_id}`"##),
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
            ("databases-indexes-get",
                    Some(r##"Gets an index."##),
                    "Details at http://byron.github.io/google-apis-rs/google_firestore1_beta1_cli/projects_databases-indexes-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the index. For example: `projects/{project_id}/databases/{database_id}/indexes/{index_id}`"##),
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
            ("databases-indexes-list",
                    Some(r##"Lists the indexes that match the specified filters."##),
                    "Details at http://byron.github.io/google-apis-rs/google_firestore1_beta1_cli/projects_databases-indexes-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The database name. For example: `projects/{project_id}/databases/{database_id}`"##),
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
    
    let mut app = App::new("firestore1-beta1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+20240617")
           .about("Accesses the NoSQL document database built for automatic scaling, high performance, and ease of application development. ")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_firestore1_beta1_cli")
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
