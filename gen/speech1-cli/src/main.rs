// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_speech1::{api, Error, oauth2, client::chrono, FieldMask};


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
    hub: api::Speech<S>,
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
    async fn _operations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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

    async fn _operations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.operations().list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "name" => {
                    call = call.name(value.unwrap_or(""));
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
                                                                           v.extend(["filter", "name", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _projects_locations_custom_classes_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "custom-class.annotations" => Some(("customClass.annotations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "custom-class.custom-class-id" => Some(("customClass.customClassId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "custom-class.delete-time" => Some(("customClass.deleteTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "custom-class.display-name" => Some(("customClass.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "custom-class.etag" => Some(("customClass.etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "custom-class.expire-time" => Some(("customClass.expireTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "custom-class.kms-key-name" => Some(("customClass.kmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "custom-class.kms-key-version-name" => Some(("customClass.kmsKeyVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "custom-class.name" => Some(("customClass.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "custom-class.reconciling" => Some(("customClass.reconciling", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "custom-class.state" => Some(("customClass.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "custom-class.uid" => Some(("customClass.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "custom-class-id" => Some(("customClassId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["annotations", "custom-class", "custom-class-id", "delete-time", "display-name", "etag", "expire-time", "kms-key-name", "kms-key-version-name", "name", "reconciling", "state", "uid"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CreateCustomClassRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_custom_classes_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_custom_classes_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_custom_classes_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_custom_classes_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_custom_classes_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_custom_classes_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_custom_classes_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_custom_classes_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "annotations" => Some(("annotations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "custom-class-id" => Some(("customClassId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "delete-time" => Some(("deleteTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "expire-time" => Some(("expireTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kms-key-name" => Some(("kmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kms-key-version-name" => Some(("kmsKeyVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "reconciling" => Some(("reconciling", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uid" => Some(("uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["annotations", "custom-class-id", "delete-time", "display-name", "etag", "expire-time", "kms-key-name", "kms-key-version-name", "name", "reconciling", "state", "uid"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CustomClass = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_custom_classes_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_phrase_sets_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "phrase-set.annotations" => Some(("phraseSet.annotations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "phrase-set.boost" => Some(("phraseSet.boost", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "phrase-set.delete-time" => Some(("phraseSet.deleteTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "phrase-set.display-name" => Some(("phraseSet.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "phrase-set.etag" => Some(("phraseSet.etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "phrase-set.expire-time" => Some(("phraseSet.expireTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "phrase-set.kms-key-name" => Some(("phraseSet.kmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "phrase-set.kms-key-version-name" => Some(("phraseSet.kmsKeyVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "phrase-set.name" => Some(("phraseSet.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "phrase-set.reconciling" => Some(("phraseSet.reconciling", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "phrase-set.state" => Some(("phraseSet.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "phrase-set.uid" => Some(("phraseSet.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "phrase-set-id" => Some(("phraseSetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["annotations", "boost", "delete-time", "display-name", "etag", "expire-time", "kms-key-name", "kms-key-version-name", "name", "phrase-set", "phrase-set-id", "reconciling", "state", "uid"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CreatePhraseSetRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_phrase_sets_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_phrase_sets_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_phrase_sets_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_phrase_sets_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_phrase_sets_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_phrase_sets_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_phrase_sets_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_phrase_sets_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "annotations" => Some(("annotations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "boost" => Some(("boost", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "delete-time" => Some(("deleteTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "expire-time" => Some(("expireTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kms-key-name" => Some(("kmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kms-key-version-name" => Some(("kmsKeyVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "reconciling" => Some(("reconciling", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uid" => Some(("uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["annotations", "boost", "delete-time", "display-name", "etag", "expire-time", "kms-key-name", "kms-key-version-name", "name", "reconciling", "state", "uid"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::PhraseSet = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_phrase_sets_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _speech_longrunningrecognize(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "audio.content" => Some(("audio.content", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "audio.uri" => Some(("audio.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.adaptation.abnf-grammar.abnf-strings" => Some(("config.adaptation.abnfGrammar.abnfStrings", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.adaptation.phrase-set-references" => Some(("config.adaptation.phraseSetReferences", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.alternative-language-codes" => Some(("config.alternativeLanguageCodes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.audio-channel-count" => Some(("config.audioChannelCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.diarization-config.enable-speaker-diarization" => Some(("config.diarizationConfig.enableSpeakerDiarization", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.diarization-config.max-speaker-count" => Some(("config.diarizationConfig.maxSpeakerCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.diarization-config.min-speaker-count" => Some(("config.diarizationConfig.minSpeakerCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.diarization-config.speaker-tag" => Some(("config.diarizationConfig.speakerTag", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.enable-automatic-punctuation" => Some(("config.enableAutomaticPunctuation", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.enable-separate-recognition-per-channel" => Some(("config.enableSeparateRecognitionPerChannel", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.enable-spoken-emojis" => Some(("config.enableSpokenEmojis", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.enable-spoken-punctuation" => Some(("config.enableSpokenPunctuation", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.enable-word-confidence" => Some(("config.enableWordConfidence", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.enable-word-time-offsets" => Some(("config.enableWordTimeOffsets", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.encoding" => Some(("config.encoding", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.language-code" => Some(("config.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.max-alternatives" => Some(("config.maxAlternatives", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.metadata.audio-topic" => Some(("config.metadata.audioTopic", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.metadata.industry-naics-code-of-audio" => Some(("config.metadata.industryNaicsCodeOfAudio", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.metadata.interaction-type" => Some(("config.metadata.interactionType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.metadata.microphone-distance" => Some(("config.metadata.microphoneDistance", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.metadata.original-media-type" => Some(("config.metadata.originalMediaType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.metadata.original-mime-type" => Some(("config.metadata.originalMimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.metadata.recording-device-name" => Some(("config.metadata.recordingDeviceName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.metadata.recording-device-type" => Some(("config.metadata.recordingDeviceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.model" => Some(("config.model", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.profanity-filter" => Some(("config.profanityFilter", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.sample-rate-hertz" => Some(("config.sampleRateHertz", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.use-enhanced" => Some(("config.useEnhanced", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "output-config.gcs-uri" => Some(("outputConfig.gcsUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["abnf-grammar", "abnf-strings", "adaptation", "alternative-language-codes", "audio", "audio-channel-count", "audio-topic", "config", "content", "diarization-config", "enable-automatic-punctuation", "enable-separate-recognition-per-channel", "enable-speaker-diarization", "enable-spoken-emojis", "enable-spoken-punctuation", "enable-word-confidence", "enable-word-time-offsets", "encoding", "gcs-uri", "industry-naics-code-of-audio", "interaction-type", "language-code", "max-alternatives", "max-speaker-count", "metadata", "microphone-distance", "min-speaker-count", "model", "original-media-type", "original-mime-type", "output-config", "phrase-set-references", "profanity-filter", "recording-device-name", "recording-device-type", "sample-rate-hertz", "speaker-tag", "uri", "use-enhanced"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::LongRunningRecognizeRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.speech().longrunningrecognize(request);
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

    async fn _speech_recognize(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "audio.content" => Some(("audio.content", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "audio.uri" => Some(("audio.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.adaptation.abnf-grammar.abnf-strings" => Some(("config.adaptation.abnfGrammar.abnfStrings", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.adaptation.phrase-set-references" => Some(("config.adaptation.phraseSetReferences", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.alternative-language-codes" => Some(("config.alternativeLanguageCodes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.audio-channel-count" => Some(("config.audioChannelCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.diarization-config.enable-speaker-diarization" => Some(("config.diarizationConfig.enableSpeakerDiarization", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.diarization-config.max-speaker-count" => Some(("config.diarizationConfig.maxSpeakerCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.diarization-config.min-speaker-count" => Some(("config.diarizationConfig.minSpeakerCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.diarization-config.speaker-tag" => Some(("config.diarizationConfig.speakerTag", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.enable-automatic-punctuation" => Some(("config.enableAutomaticPunctuation", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.enable-separate-recognition-per-channel" => Some(("config.enableSeparateRecognitionPerChannel", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.enable-spoken-emojis" => Some(("config.enableSpokenEmojis", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.enable-spoken-punctuation" => Some(("config.enableSpokenPunctuation", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.enable-word-confidence" => Some(("config.enableWordConfidence", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.enable-word-time-offsets" => Some(("config.enableWordTimeOffsets", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.encoding" => Some(("config.encoding", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.language-code" => Some(("config.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.max-alternatives" => Some(("config.maxAlternatives", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.metadata.audio-topic" => Some(("config.metadata.audioTopic", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.metadata.industry-naics-code-of-audio" => Some(("config.metadata.industryNaicsCodeOfAudio", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.metadata.interaction-type" => Some(("config.metadata.interactionType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.metadata.microphone-distance" => Some(("config.metadata.microphoneDistance", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.metadata.original-media-type" => Some(("config.metadata.originalMediaType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.metadata.original-mime-type" => Some(("config.metadata.originalMimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.metadata.recording-device-name" => Some(("config.metadata.recordingDeviceName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.metadata.recording-device-type" => Some(("config.metadata.recordingDeviceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.model" => Some(("config.model", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.profanity-filter" => Some(("config.profanityFilter", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.sample-rate-hertz" => Some(("config.sampleRateHertz", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.use-enhanced" => Some(("config.useEnhanced", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["abnf-grammar", "abnf-strings", "adaptation", "alternative-language-codes", "audio", "audio-channel-count", "audio-topic", "config", "content", "diarization-config", "enable-automatic-punctuation", "enable-separate-recognition-per-channel", "enable-speaker-diarization", "enable-spoken-emojis", "enable-spoken-punctuation", "enable-word-confidence", "enable-word-time-offsets", "encoding", "industry-naics-code-of-audio", "interaction-type", "language-code", "max-alternatives", "max-speaker-count", "metadata", "microphone-distance", "min-speaker-count", "model", "original-media-type", "original-mime-type", "phrase-set-references", "profanity-filter", "recording-device-name", "recording-device-type", "sample-rate-hertz", "speaker-tag", "uri", "use-enhanced"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::RecognizeRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.speech().recognize(request);
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
                    ("get", Some(opt)) => {
                        call_result = self._operations_get(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._operations_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("operations".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("projects", Some(opt)) => {
                match opt.subcommand() {
                    ("locations-custom-classes-create", Some(opt)) => {
                        call_result = self._projects_locations_custom_classes_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-custom-classes-delete", Some(opt)) => {
                        call_result = self._projects_locations_custom_classes_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-custom-classes-get", Some(opt)) => {
                        call_result = self._projects_locations_custom_classes_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-custom-classes-list", Some(opt)) => {
                        call_result = self._projects_locations_custom_classes_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-custom-classes-patch", Some(opt)) => {
                        call_result = self._projects_locations_custom_classes_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-phrase-sets-create", Some(opt)) => {
                        call_result = self._projects_locations_phrase_sets_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-phrase-sets-delete", Some(opt)) => {
                        call_result = self._projects_locations_phrase_sets_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-phrase-sets-get", Some(opt)) => {
                        call_result = self._projects_locations_phrase_sets_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-phrase-sets-list", Some(opt)) => {
                        call_result = self._projects_locations_phrase_sets_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-phrase-sets-patch", Some(opt)) => {
                        call_result = self._projects_locations_phrase_sets_patch(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("projects".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("speech", Some(opt)) => {
                match opt.subcommand() {
                    ("longrunningrecognize", Some(opt)) => {
                        call_result = self._speech_longrunningrecognize(opt, dry_run, &mut err).await;
                    },
                    ("recognize", Some(opt)) => {
                        call_result = self._speech_recognize(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("speech".to_string()));
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

            match client::application_secret_from_directory(&config_dir, "speech1-secret.json",
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
        ).persist_tokens_to_disk(format!("{}/speech1", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::Speech::new(client, auth),
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
        ("operations", "methods: 'get' and 'list'", vec![
            ("get",
                    Some(r##"Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_speech1_cli/operations_get",
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
                    Some(r##"Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_speech1_cli/operations_list",
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
        
        ("projects", "methods: 'locations-custom-classes-create', 'locations-custom-classes-delete', 'locations-custom-classes-get', 'locations-custom-classes-list', 'locations-custom-classes-patch', 'locations-phrase-sets-create', 'locations-phrase-sets-delete', 'locations-phrase-sets-get', 'locations-phrase-sets-list' and 'locations-phrase-sets-patch'", vec![
            ("locations-custom-classes-create",
                    Some(r##"Create a custom class."##),
                    "Details at http://byron.github.io/google-apis-rs/google_speech1_cli/projects_locations-custom-classes-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource where this custom class will be created. Format: `projects/{project}/locations/{location}/customClasses` Speech-to-Text supports three locations: `global`, `us` (US North America), and `eu` (Europe). If you are calling the `speech.googleapis.com` endpoint, use the `global` location. To specify a region, use a [regional endpoint](https://cloud.google.com/speech-to-text/docs/endpoints) with matching `us` or `eu` location value."##),
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
            ("locations-custom-classes-delete",
                    Some(r##"Delete a custom class."##),
                    "Details at http://byron.github.io/google-apis-rs/google_speech1_cli/projects_locations-custom-classes-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the custom class to delete. Format: `projects/{project}/locations/{location}/customClasses/{custom_class}` Speech-to-Text supports three locations: `global`, `us` (US North America), and `eu` (Europe). If you are calling the `speech.googleapis.com` endpoint, use the `global` location. To specify a region, use a [regional endpoint](https://cloud.google.com/speech-to-text/docs/endpoints) with matching `us` or `eu` location value."##),
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
            ("locations-custom-classes-get",
                    Some(r##"Get a custom class."##),
                    "Details at http://byron.github.io/google-apis-rs/google_speech1_cli/projects_locations-custom-classes-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the custom class to retrieve. Format: `projects/{project}/locations/{location}/customClasses/{custom_class}`"##),
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
            ("locations-custom-classes-list",
                    Some(r##"List custom classes."##),
                    "Details at http://byron.github.io/google-apis-rs/google_speech1_cli/projects_locations-custom-classes-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent, which owns this collection of custom classes. Format: `projects/{project}/locations/{location}/customClasses` Speech-to-Text supports three locations: `global`, `us` (US North America), and `eu` (Europe). If you are calling the `speech.googleapis.com` endpoint, use the `global` location. To specify a region, use a [regional endpoint](https://cloud.google.com/speech-to-text/docs/endpoints) with matching `us` or `eu` location value."##),
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
            ("locations-custom-classes-patch",
                    Some(r##"Update a custom class."##),
                    "Details at http://byron.github.io/google-apis-rs/google_speech1_cli/projects_locations-custom-classes-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The resource name of the custom class."##),
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
            ("locations-phrase-sets-create",
                    Some(r##"Create a set of phrase hints. Each item in the set can be a single word or a multi-word phrase. The items in the PhraseSet are favored by the recognition model when you send a call that includes the PhraseSet."##),
                    "Details at http://byron.github.io/google-apis-rs/google_speech1_cli/projects_locations-phrase-sets-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource where this phrase set will be created. Format: `projects/{project}/locations/{location}` Speech-to-Text supports three locations: `global`, `us` (US North America), and `eu` (Europe). If you are calling the `speech.googleapis.com` endpoint, use the `global` location. To specify a region, use a [regional endpoint](https://cloud.google.com/speech-to-text/docs/endpoints) with matching `us` or `eu` location value."##),
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
            ("locations-phrase-sets-delete",
                    Some(r##"Delete a phrase set."##),
                    "Details at http://byron.github.io/google-apis-rs/google_speech1_cli/projects_locations-phrase-sets-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the phrase set to delete. Format: `projects/{project}/locations/{location}/phraseSets/{phrase_set}`"##),
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
            ("locations-phrase-sets-get",
                    Some(r##"Get a phrase set."##),
                    "Details at http://byron.github.io/google-apis-rs/google_speech1_cli/projects_locations-phrase-sets-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the phrase set to retrieve. Format: `projects/{project}/locations/{location}/phraseSets/{phrase_set}` Speech-to-Text supports three locations: `global`, `us` (US North America), and `eu` (Europe). If you are calling the `speech.googleapis.com` endpoint, use the `global` location. To specify a region, use a [regional endpoint](https://cloud.google.com/speech-to-text/docs/endpoints) with matching `us` or `eu` location value."##),
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
            ("locations-phrase-sets-list",
                    Some(r##"List phrase sets."##),
                    "Details at http://byron.github.io/google-apis-rs/google_speech1_cli/projects_locations-phrase-sets-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent, which owns this collection of phrase set. Format: `projects/{project}/locations/{location}` Speech-to-Text supports three locations: `global`, `us` (US North America), and `eu` (Europe). If you are calling the `speech.googleapis.com` endpoint, use the `global` location. To specify a region, use a [regional endpoint](https://cloud.google.com/speech-to-text/docs/endpoints) with matching `us` or `eu` location value."##),
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
            ("locations-phrase-sets-patch",
                    Some(r##"Update a phrase set."##),
                    "Details at http://byron.github.io/google-apis-rs/google_speech1_cli/projects_locations-phrase-sets-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The resource name of the phrase set."##),
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
        
        ("speech", "methods: 'longrunningrecognize' and 'recognize'", vec![
            ("longrunningrecognize",
                    Some(r##"Performs asynchronous speech recognition: receive results via the google.longrunning.Operations interface. Returns either an `Operation.error` or an `Operation.response` which contains a `LongRunningRecognizeResponse` message. For more information on asynchronous speech recognition, see the [how-to](https://cloud.google.com/speech-to-text/docs/async-recognize)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_speech1_cli/speech_longrunningrecognize",
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
            ("recognize",
                    Some(r##"Performs synchronous speech recognition: receive results after all audio has been sent and processed."##),
                    "Details at http://byron.github.io/google-apis-rs/google_speech1_cli/speech_recognize",
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
            ]),
        
    ];
    
    let mut app = App::new("speech1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+20240625")
           .about("Converts audio to text by applying powerful neural network models.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_speech1_cli")
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
