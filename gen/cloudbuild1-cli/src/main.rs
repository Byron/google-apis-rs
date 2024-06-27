// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_cloudbuild1::{api, Error, oauth2, client::chrono, FieldMask};


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
    hub: api::CloudBuild<S>,
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
    async fn _github_dot_com_webhook_receive(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "content-type" => Some(("contentType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "data" => Some(("data", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["content-type", "data"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::HttpBody = json::value::from_value(object).unwrap();
        let mut call = self.hub.github_dot_com_webhook().receive(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "webhook-key" => {
                    call = call.webhook_key(value.unwrap_or(""));
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
                                                                           v.extend(["webhook-key"].iter().map(|v|*v));
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

    async fn _locations_regional_webhook(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "content-type" => Some(("contentType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "data" => Some(("data", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["content-type", "data"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::HttpBody = json::value::from_value(object).unwrap();
        let mut call = self.hub.locations().regional_webhook(request, opt.value_of("location").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "webhook-key" => {
                    call = call.webhook_key(value.unwrap_or(""));
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
                                                                           v.extend(["webhook-key"].iter().map(|v|*v));
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

    async fn _methods_webhook(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "content-type" => Some(("contentType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "data" => Some(("data", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["content-type", "data"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::HttpBody = json::value::from_value(object).unwrap();
        let mut call = self.hub.methods().webhook(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "webhook-key" => {
                    call = call.webhook_key(value.unwrap_or(""));
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
                                                                           v.extend(["webhook-key"].iter().map(|v|*v));
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

    async fn _operations_cancel(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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

    async fn _projects_builds_approve(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "approval-result.approval-time" => Some(("approvalResult.approvalTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "approval-result.approver-account" => Some(("approvalResult.approverAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "approval-result.comment" => Some(("approvalResult.comment", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "approval-result.decision" => Some(("approvalResult.decision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "approval-result.url" => Some(("approvalResult.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["approval-result", "approval-time", "approver-account", "comment", "decision", "url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ApproveBuildRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().builds_approve(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_builds_cancel(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["id", "name", "project-id"]);
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

    async fn _projects_builds_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "approval.config.approval-required" => Some(("approval.config.approvalRequired", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "approval.result.approval-time" => Some(("approval.result.approvalTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "approval.result.approver-account" => Some(("approval.result.approverAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "approval.result.comment" => Some(("approval.result.comment", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "approval.result.decision" => Some(("approval.result.decision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "approval.result.url" => Some(("approval.result.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "approval.state" => Some(("approval.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "artifacts.images" => Some(("artifacts.images", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "artifacts.objects.location" => Some(("artifacts.objects.location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "artifacts.objects.paths" => Some(("artifacts.objects.paths", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "artifacts.objects.timing.end-time" => Some(("artifacts.objects.timing.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "artifacts.objects.timing.start-time" => Some(("artifacts.objects.timing.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build-trigger-id" => Some(("buildTriggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "failure-info.detail" => Some(("failureInfo.detail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "failure-info.type" => Some(("failureInfo.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "finish-time" => Some(("finishTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-config.http.proxy-secret-version-name" => Some(("gitConfig.http.proxySecretVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-config.http.proxy-ssl-ca-info.bucket" => Some(("gitConfig.http.proxySslCaInfo.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-config.http.proxy-ssl-ca-info.generation" => Some(("gitConfig.http.proxySslCaInfo.generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-config.http.proxy-ssl-ca-info.object" => Some(("gitConfig.http.proxySslCaInfo.object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "images" => Some(("images", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "log-url" => Some(("logUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "logs-bucket" => Some(("logsBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "options.automap-substitutions" => Some(("options.automapSubstitutions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "options.default-logs-bucket-behavior" => Some(("options.defaultLogsBucketBehavior", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "options.disk-size-gb" => Some(("options.diskSizeGb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "options.dynamic-substitutions" => Some(("options.dynamicSubstitutions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "options.env" => Some(("options.env", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "options.log-streaming-option" => Some(("options.logStreamingOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "options.logging" => Some(("options.logging", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "options.machine-type" => Some(("options.machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "options.pool.name" => Some(("options.pool.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "options.requested-verify-option" => Some(("options.requestedVerifyOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "options.secret-env" => Some(("options.secretEnv", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "options.source-provenance-hash" => Some(("options.sourceProvenanceHash", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "options.substitution-option" => Some(("options.substitutionOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "options.worker-pool" => Some(("options.workerPool", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "queue-ttl" => Some(("queueTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "results.artifact-manifest" => Some(("results.artifactManifest", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "results.artifact-timing.end-time" => Some(("results.artifactTiming.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "results.artifact-timing.start-time" => Some(("results.artifactTiming.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "results.build-step-images" => Some(("results.buildStepImages", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "results.build-step-outputs" => Some(("results.buildStepOutputs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "results.num-artifacts" => Some(("results.numArtifacts", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-account" => Some(("serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.connected-repository.dir" => Some(("source.connectedRepository.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.connected-repository.repository" => Some(("source.connectedRepository.repository", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.connected-repository.revision" => Some(("source.connectedRepository.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.developer-connect-config.dir" => Some(("source.developerConnectConfig.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.developer-connect-config.git-repository-link" => Some(("source.developerConnectConfig.gitRepositoryLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.developer-connect-config.revision" => Some(("source.developerConnectConfig.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.git-source.dir" => Some(("source.gitSource.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.git-source.revision" => Some(("source.gitSource.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.git-source.url" => Some(("source.gitSource.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.repo-source.branch-name" => Some(("source.repoSource.branchName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.repo-source.commit-sha" => Some(("source.repoSource.commitSha", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.repo-source.dir" => Some(("source.repoSource.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.repo-source.invert-regex" => Some(("source.repoSource.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "source.repo-source.project-id" => Some(("source.repoSource.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.repo-source.repo-name" => Some(("source.repoSource.repoName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.repo-source.substitutions" => Some(("source.repoSource.substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "source.repo-source.tag-name" => Some(("source.repoSource.tagName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.storage-source.bucket" => Some(("source.storageSource.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.storage-source.generation" => Some(("source.storageSource.generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.storage-source.object" => Some(("source.storageSource.object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.storage-source.source-fetcher" => Some(("source.storageSource.sourceFetcher", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.storage-source-manifest.bucket" => Some(("source.storageSourceManifest.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.storage-source-manifest.generation" => Some(("source.storageSourceManifest.generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.storage-source-manifest.object" => Some(("source.storageSourceManifest.object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-connected-repository.dir" => Some(("sourceProvenance.resolvedConnectedRepository.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-connected-repository.repository" => Some(("sourceProvenance.resolvedConnectedRepository.repository", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-connected-repository.revision" => Some(("sourceProvenance.resolvedConnectedRepository.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-git-source.dir" => Some(("sourceProvenance.resolvedGitSource.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-git-source.revision" => Some(("sourceProvenance.resolvedGitSource.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-git-source.url" => Some(("sourceProvenance.resolvedGitSource.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-repo-source.branch-name" => Some(("sourceProvenance.resolvedRepoSource.branchName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-repo-source.commit-sha" => Some(("sourceProvenance.resolvedRepoSource.commitSha", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-repo-source.dir" => Some(("sourceProvenance.resolvedRepoSource.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-repo-source.invert-regex" => Some(("sourceProvenance.resolvedRepoSource.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-repo-source.project-id" => Some(("sourceProvenance.resolvedRepoSource.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-repo-source.repo-name" => Some(("sourceProvenance.resolvedRepoSource.repoName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-repo-source.substitutions" => Some(("sourceProvenance.resolvedRepoSource.substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "source-provenance.resolved-repo-source.tag-name" => Some(("sourceProvenance.resolvedRepoSource.tagName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-storage-source.bucket" => Some(("sourceProvenance.resolvedStorageSource.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-storage-source.generation" => Some(("sourceProvenance.resolvedStorageSource.generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-storage-source.object" => Some(("sourceProvenance.resolvedStorageSource.object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-storage-source.source-fetcher" => Some(("sourceProvenance.resolvedStorageSource.sourceFetcher", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-storage-source-manifest.bucket" => Some(("sourceProvenance.resolvedStorageSourceManifest.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-storage-source-manifest.generation" => Some(("sourceProvenance.resolvedStorageSourceManifest.generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-storage-source-manifest.object" => Some(("sourceProvenance.resolvedStorageSourceManifest.object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "start-time" => Some(("startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status" => Some(("status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status-detail" => Some(("statusDetail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "substitutions" => Some(("substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "tags" => Some(("tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "timeout" => Some(("timeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["approval", "approval-required", "approval-time", "approver-account", "artifact-manifest", "artifact-timing", "artifacts", "automap-substitutions", "branch-name", "bucket", "build-step-images", "build-step-outputs", "build-trigger-id", "comment", "commit-sha", "config", "connected-repository", "create-time", "decision", "default-logs-bucket-behavior", "detail", "developer-connect-config", "dir", "disk-size-gb", "dynamic-substitutions", "end-time", "env", "failure-info", "finish-time", "generation", "git-config", "git-repository-link", "git-source", "http", "id", "images", "invert-regex", "location", "log-streaming-option", "log-url", "logging", "logs-bucket", "machine-type", "name", "num-artifacts", "object", "objects", "options", "paths", "pool", "project-id", "proxy-secret-version-name", "proxy-ssl-ca-info", "queue-ttl", "repo-name", "repo-source", "repository", "requested-verify-option", "resolved-connected-repository", "resolved-git-source", "resolved-repo-source", "resolved-storage-source", "resolved-storage-source-manifest", "result", "results", "revision", "secret-env", "service-account", "source", "source-fetcher", "source-provenance", "source-provenance-hash", "start-time", "state", "status", "status-detail", "storage-source", "storage-source-manifest", "substitution-option", "substitutions", "tag-name", "tags", "timeout", "timing", "type", "url", "worker-pool"]);
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
                "parent" => {
                    call = call.parent(value.unwrap_or(""));
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
                                                                           v.extend(["parent"].iter().map(|v|*v));
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

    async fn _projects_builds_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().builds_get(opt.value_of("project-id").unwrap_or(""), opt.value_of("id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "name" => {
                    call = call.name(value.unwrap_or(""));
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
                                                                           v.extend(["name"].iter().map(|v|*v));
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

    async fn _projects_builds_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().builds_list(opt.value_of("project-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "parent" => {
                    call = call.parent(value.unwrap_or(""));
                },
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
                                                                           v.extend(["filter", "page-size", "page-token", "parent"].iter().map(|v|*v));
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

    async fn _projects_builds_retry(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["id", "name", "project-id"]);
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

    async fn _projects_github_enterprise_configs_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "app-id" => Some(("appId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "host-url" => Some(("hostUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "peered-network" => Some(("peeredNetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.oauth-client-id-name" => Some(("secrets.oauthClientIdName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.oauth-client-id-version-name" => Some(("secrets.oauthClientIdVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.oauth-secret-name" => Some(("secrets.oauthSecretName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.oauth-secret-version-name" => Some(("secrets.oauthSecretVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.private-key-name" => Some(("secrets.privateKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.private-key-version-name" => Some(("secrets.privateKeyVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.webhook-secret-name" => Some(("secrets.webhookSecretName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.webhook-secret-version-name" => Some(("secrets.webhookSecretVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ssl-ca" => Some(("sslCa", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "webhook-key" => Some(("webhookKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["app-id", "create-time", "display-name", "host-url", "name", "oauth-client-id-name", "oauth-client-id-version-name", "oauth-secret-name", "oauth-secret-version-name", "peered-network", "private-key-name", "private-key-version-name", "secrets", "ssl-ca", "webhook-key", "webhook-secret-name", "webhook-secret-version-name"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GitHubEnterpriseConfig = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().github_enterprise_configs_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
                },
                "ghe-config-id" => {
                    call = call.ghe_config_id(value.unwrap_or(""));
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
                                                                           v.extend(["ghe-config-id", "project-id"].iter().map(|v|*v));
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

    async fn _projects_github_enterprise_configs_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().github_enterprise_configs_delete(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
                },
                "config-id" => {
                    call = call.config_id(value.unwrap_or(""));
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
                                                                           v.extend(["config-id", "project-id"].iter().map(|v|*v));
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

    async fn _projects_github_enterprise_configs_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().github_enterprise_configs_get(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
                },
                "config-id" => {
                    call = call.config_id(value.unwrap_or(""));
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
                                                                           v.extend(["config-id", "project-id"].iter().map(|v|*v));
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

    async fn _projects_github_enterprise_configs_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().github_enterprise_configs_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
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
                                                                           v.extend(["project-id"].iter().map(|v|*v));
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

    async fn _projects_github_enterprise_configs_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "app-id" => Some(("appId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "host-url" => Some(("hostUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "peered-network" => Some(("peeredNetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.oauth-client-id-name" => Some(("secrets.oauthClientIdName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.oauth-client-id-version-name" => Some(("secrets.oauthClientIdVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.oauth-secret-name" => Some(("secrets.oauthSecretName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.oauth-secret-version-name" => Some(("secrets.oauthSecretVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.private-key-name" => Some(("secrets.privateKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.private-key-version-name" => Some(("secrets.privateKeyVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.webhook-secret-name" => Some(("secrets.webhookSecretName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.webhook-secret-version-name" => Some(("secrets.webhookSecretVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ssl-ca" => Some(("sslCa", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "webhook-key" => Some(("webhookKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["app-id", "create-time", "display-name", "host-url", "name", "oauth-client-id-name", "oauth-client-id-version-name", "oauth-secret-name", "oauth-secret-version-name", "peered-network", "private-key-name", "private-key-version-name", "secrets", "ssl-ca", "webhook-key", "webhook-secret-name", "webhook-secret-version-name"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GitHubEnterpriseConfig = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().github_enterprise_configs_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_bitbucket_server_configs_connected_repositories_batch_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::BatchCreateBitbucketServerConnectedRepositoriesRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_bitbucket_server_configs_connected_repositories_batch_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_bitbucket_server_configs_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "api-key" => Some(("apiKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "host-uri" => Some(("hostUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "peered-network" => Some(("peeredNetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "peered-network-ip-range" => Some(("peeredNetworkIpRange", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.admin-access-token-version-name" => Some(("secrets.adminAccessTokenVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.read-access-token-version-name" => Some(("secrets.readAccessTokenVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.webhook-secret-version-name" => Some(("secrets.webhookSecretVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ssl-ca" => Some(("sslCa", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "username" => Some(("username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "webhook-key" => Some(("webhookKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["admin-access-token-version-name", "api-key", "create-time", "host-uri", "name", "peered-network", "peered-network-ip-range", "read-access-token-version-name", "secrets", "ssl-ca", "username", "webhook-key", "webhook-secret-version-name"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::BitbucketServerConfig = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_bitbucket_server_configs_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "bitbucket-server-config-id" => {
                    call = call.bitbucket_server_config_id(value.unwrap_or(""));
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
                                                                           v.extend(["bitbucket-server-config-id"].iter().map(|v|*v));
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

    async fn _projects_locations_bitbucket_server_configs_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_bitbucket_server_configs_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_bitbucket_server_configs_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_bitbucket_server_configs_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_bitbucket_server_configs_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_bitbucket_server_configs_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_bitbucket_server_configs_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "api-key" => Some(("apiKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "host-uri" => Some(("hostUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "peered-network" => Some(("peeredNetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "peered-network-ip-range" => Some(("peeredNetworkIpRange", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.admin-access-token-version-name" => Some(("secrets.adminAccessTokenVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.read-access-token-version-name" => Some(("secrets.readAccessTokenVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.webhook-secret-version-name" => Some(("secrets.webhookSecretVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ssl-ca" => Some(("sslCa", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "username" => Some(("username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "webhook-key" => Some(("webhookKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["admin-access-token-version-name", "api-key", "create-time", "host-uri", "name", "peered-network", "peered-network-ip-range", "read-access-token-version-name", "secrets", "ssl-ca", "username", "webhook-key", "webhook-secret-version-name"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::BitbucketServerConfig = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_bitbucket_server_configs_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_bitbucket_server_configs_remove_bitbucket_server_connected_repository(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "connected-repository.project-key" => Some(("connectedRepository.projectKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "connected-repository.repo-slug" => Some(("connectedRepository.repoSlug", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "connected-repository.webhook-id" => Some(("connectedRepository.webhookId", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["connected-repository", "project-key", "repo-slug", "webhook-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::RemoveBitbucketServerConnectedRepositoryRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_bitbucket_server_configs_remove_bitbucket_server_connected_repository(request, opt.value_of("config").unwrap_or(""));
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

    async fn _projects_locations_bitbucket_server_configs_repos_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_bitbucket_server_configs_repos_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_builds_approve(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "approval-result.approval-time" => Some(("approvalResult.approvalTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "approval-result.approver-account" => Some(("approvalResult.approverAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "approval-result.comment" => Some(("approvalResult.comment", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "approval-result.decision" => Some(("approvalResult.decision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "approval-result.url" => Some(("approvalResult.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["approval-result", "approval-time", "approver-account", "comment", "decision", "url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ApproveBuildRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_builds_approve(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_builds_cancel(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["id", "name", "project-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CancelBuildRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_builds_cancel(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_builds_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "approval.config.approval-required" => Some(("approval.config.approvalRequired", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "approval.result.approval-time" => Some(("approval.result.approvalTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "approval.result.approver-account" => Some(("approval.result.approverAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "approval.result.comment" => Some(("approval.result.comment", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "approval.result.decision" => Some(("approval.result.decision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "approval.result.url" => Some(("approval.result.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "approval.state" => Some(("approval.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "artifacts.images" => Some(("artifacts.images", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "artifacts.objects.location" => Some(("artifacts.objects.location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "artifacts.objects.paths" => Some(("artifacts.objects.paths", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "artifacts.objects.timing.end-time" => Some(("artifacts.objects.timing.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "artifacts.objects.timing.start-time" => Some(("artifacts.objects.timing.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build-trigger-id" => Some(("buildTriggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "failure-info.detail" => Some(("failureInfo.detail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "failure-info.type" => Some(("failureInfo.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "finish-time" => Some(("finishTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-config.http.proxy-secret-version-name" => Some(("gitConfig.http.proxySecretVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-config.http.proxy-ssl-ca-info.bucket" => Some(("gitConfig.http.proxySslCaInfo.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-config.http.proxy-ssl-ca-info.generation" => Some(("gitConfig.http.proxySslCaInfo.generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-config.http.proxy-ssl-ca-info.object" => Some(("gitConfig.http.proxySslCaInfo.object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "images" => Some(("images", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "log-url" => Some(("logUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "logs-bucket" => Some(("logsBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "options.automap-substitutions" => Some(("options.automapSubstitutions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "options.default-logs-bucket-behavior" => Some(("options.defaultLogsBucketBehavior", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "options.disk-size-gb" => Some(("options.diskSizeGb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "options.dynamic-substitutions" => Some(("options.dynamicSubstitutions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "options.env" => Some(("options.env", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "options.log-streaming-option" => Some(("options.logStreamingOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "options.logging" => Some(("options.logging", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "options.machine-type" => Some(("options.machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "options.pool.name" => Some(("options.pool.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "options.requested-verify-option" => Some(("options.requestedVerifyOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "options.secret-env" => Some(("options.secretEnv", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "options.source-provenance-hash" => Some(("options.sourceProvenanceHash", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "options.substitution-option" => Some(("options.substitutionOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "options.worker-pool" => Some(("options.workerPool", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "queue-ttl" => Some(("queueTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "results.artifact-manifest" => Some(("results.artifactManifest", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "results.artifact-timing.end-time" => Some(("results.artifactTiming.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "results.artifact-timing.start-time" => Some(("results.artifactTiming.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "results.build-step-images" => Some(("results.buildStepImages", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "results.build-step-outputs" => Some(("results.buildStepOutputs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "results.num-artifacts" => Some(("results.numArtifacts", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-account" => Some(("serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.connected-repository.dir" => Some(("source.connectedRepository.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.connected-repository.repository" => Some(("source.connectedRepository.repository", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.connected-repository.revision" => Some(("source.connectedRepository.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.developer-connect-config.dir" => Some(("source.developerConnectConfig.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.developer-connect-config.git-repository-link" => Some(("source.developerConnectConfig.gitRepositoryLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.developer-connect-config.revision" => Some(("source.developerConnectConfig.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.git-source.dir" => Some(("source.gitSource.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.git-source.revision" => Some(("source.gitSource.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.git-source.url" => Some(("source.gitSource.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.repo-source.branch-name" => Some(("source.repoSource.branchName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.repo-source.commit-sha" => Some(("source.repoSource.commitSha", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.repo-source.dir" => Some(("source.repoSource.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.repo-source.invert-regex" => Some(("source.repoSource.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "source.repo-source.project-id" => Some(("source.repoSource.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.repo-source.repo-name" => Some(("source.repoSource.repoName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.repo-source.substitutions" => Some(("source.repoSource.substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "source.repo-source.tag-name" => Some(("source.repoSource.tagName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.storage-source.bucket" => Some(("source.storageSource.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.storage-source.generation" => Some(("source.storageSource.generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.storage-source.object" => Some(("source.storageSource.object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.storage-source.source-fetcher" => Some(("source.storageSource.sourceFetcher", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.storage-source-manifest.bucket" => Some(("source.storageSourceManifest.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.storage-source-manifest.generation" => Some(("source.storageSourceManifest.generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.storage-source-manifest.object" => Some(("source.storageSourceManifest.object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-connected-repository.dir" => Some(("sourceProvenance.resolvedConnectedRepository.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-connected-repository.repository" => Some(("sourceProvenance.resolvedConnectedRepository.repository", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-connected-repository.revision" => Some(("sourceProvenance.resolvedConnectedRepository.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-git-source.dir" => Some(("sourceProvenance.resolvedGitSource.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-git-source.revision" => Some(("sourceProvenance.resolvedGitSource.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-git-source.url" => Some(("sourceProvenance.resolvedGitSource.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-repo-source.branch-name" => Some(("sourceProvenance.resolvedRepoSource.branchName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-repo-source.commit-sha" => Some(("sourceProvenance.resolvedRepoSource.commitSha", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-repo-source.dir" => Some(("sourceProvenance.resolvedRepoSource.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-repo-source.invert-regex" => Some(("sourceProvenance.resolvedRepoSource.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-repo-source.project-id" => Some(("sourceProvenance.resolvedRepoSource.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-repo-source.repo-name" => Some(("sourceProvenance.resolvedRepoSource.repoName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-repo-source.substitutions" => Some(("sourceProvenance.resolvedRepoSource.substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "source-provenance.resolved-repo-source.tag-name" => Some(("sourceProvenance.resolvedRepoSource.tagName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-storage-source.bucket" => Some(("sourceProvenance.resolvedStorageSource.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-storage-source.generation" => Some(("sourceProvenance.resolvedStorageSource.generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-storage-source.object" => Some(("sourceProvenance.resolvedStorageSource.object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-storage-source.source-fetcher" => Some(("sourceProvenance.resolvedStorageSource.sourceFetcher", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-storage-source-manifest.bucket" => Some(("sourceProvenance.resolvedStorageSourceManifest.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-storage-source-manifest.generation" => Some(("sourceProvenance.resolvedStorageSourceManifest.generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-provenance.resolved-storage-source-manifest.object" => Some(("sourceProvenance.resolvedStorageSourceManifest.object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "start-time" => Some(("startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status" => Some(("status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status-detail" => Some(("statusDetail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "substitutions" => Some(("substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "tags" => Some(("tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "timeout" => Some(("timeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["approval", "approval-required", "approval-time", "approver-account", "artifact-manifest", "artifact-timing", "artifacts", "automap-substitutions", "branch-name", "bucket", "build-step-images", "build-step-outputs", "build-trigger-id", "comment", "commit-sha", "config", "connected-repository", "create-time", "decision", "default-logs-bucket-behavior", "detail", "developer-connect-config", "dir", "disk-size-gb", "dynamic-substitutions", "end-time", "env", "failure-info", "finish-time", "generation", "git-config", "git-repository-link", "git-source", "http", "id", "images", "invert-regex", "location", "log-streaming-option", "log-url", "logging", "logs-bucket", "machine-type", "name", "num-artifacts", "object", "objects", "options", "paths", "pool", "project-id", "proxy-secret-version-name", "proxy-ssl-ca-info", "queue-ttl", "repo-name", "repo-source", "repository", "requested-verify-option", "resolved-connected-repository", "resolved-git-source", "resolved-repo-source", "resolved-storage-source", "resolved-storage-source-manifest", "result", "results", "revision", "secret-env", "service-account", "source", "source-fetcher", "source-provenance", "source-provenance-hash", "start-time", "state", "status", "status-detail", "storage-source", "storage-source-manifest", "substitution-option", "substitutions", "tag-name", "tags", "timeout", "timing", "type", "url", "worker-pool"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Build = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_builds_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
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
                                                                           v.extend(["project-id"].iter().map(|v|*v));
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

    async fn _projects_locations_builds_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_builds_get(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
                },
                "id" => {
                    call = call.id(value.unwrap_or(""));
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
                                                                           v.extend(["id", "project-id"].iter().map(|v|*v));
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

    async fn _projects_locations_builds_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_builds_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
                },
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
                                                                           v.extend(["filter", "page-size", "page-token", "project-id"].iter().map(|v|*v));
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

    async fn _projects_locations_builds_retry(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["id", "name", "project-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::RetryBuildRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_builds_retry(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_get_default_service_account(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_get_default_service_account(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_git_lab_configs_connected_repositories_batch_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::BatchCreateGitLabConnectedRepositoriesRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_git_lab_configs_connected_repositories_batch_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_git_lab_configs_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "enterprise-config.host-uri" => Some(("enterpriseConfig.hostUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "enterprise-config.service-directory-config.service" => Some(("enterpriseConfig.serviceDirectoryConfig.service", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "enterprise-config.ssl-ca" => Some(("enterpriseConfig.sslCa", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.api-access-token-version" => Some(("secrets.apiAccessTokenVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.api-key-version" => Some(("secrets.apiKeyVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.read-access-token-version" => Some(("secrets.readAccessTokenVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.webhook-secret-version" => Some(("secrets.webhookSecretVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "username" => Some(("username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "webhook-key" => Some(("webhookKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["api-access-token-version", "api-key-version", "create-time", "enterprise-config", "host-uri", "name", "read-access-token-version", "secrets", "service", "service-directory-config", "ssl-ca", "username", "webhook-key", "webhook-secret-version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GitLabConfig = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_git_lab_configs_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "gitlab-config-id" => {
                    call = call.gitlab_config_id(value.unwrap_or(""));
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
                                                                           v.extend(["gitlab-config-id"].iter().map(|v|*v));
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

    async fn _projects_locations_git_lab_configs_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_git_lab_configs_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_git_lab_configs_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_git_lab_configs_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_git_lab_configs_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_git_lab_configs_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_git_lab_configs_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "enterprise-config.host-uri" => Some(("enterpriseConfig.hostUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "enterprise-config.service-directory-config.service" => Some(("enterpriseConfig.serviceDirectoryConfig.service", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "enterprise-config.ssl-ca" => Some(("enterpriseConfig.sslCa", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.api-access-token-version" => Some(("secrets.apiAccessTokenVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.api-key-version" => Some(("secrets.apiKeyVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.read-access-token-version" => Some(("secrets.readAccessTokenVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.webhook-secret-version" => Some(("secrets.webhookSecretVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "username" => Some(("username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "webhook-key" => Some(("webhookKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["api-access-token-version", "api-key-version", "create-time", "enterprise-config", "host-uri", "name", "read-access-token-version", "secrets", "service", "service-directory-config", "ssl-ca", "username", "webhook-key", "webhook-secret-version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GitLabConfig = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_git_lab_configs_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_git_lab_configs_remove_git_lab_connected_repository(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "connected-repository.id" => Some(("connectedRepository.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "connected-repository.webhook-id" => Some(("connectedRepository.webhookId", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["connected-repository", "id", "webhook-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::RemoveGitLabConnectedRepositoryRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_git_lab_configs_remove_git_lab_connected_repository(request, opt.value_of("config").unwrap_or(""));
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

    async fn _projects_locations_git_lab_configs_repos_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_git_lab_configs_repos_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_github_enterprise_configs_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "app-id" => Some(("appId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "host-url" => Some(("hostUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "peered-network" => Some(("peeredNetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.oauth-client-id-name" => Some(("secrets.oauthClientIdName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.oauth-client-id-version-name" => Some(("secrets.oauthClientIdVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.oauth-secret-name" => Some(("secrets.oauthSecretName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.oauth-secret-version-name" => Some(("secrets.oauthSecretVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.private-key-name" => Some(("secrets.privateKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.private-key-version-name" => Some(("secrets.privateKeyVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.webhook-secret-name" => Some(("secrets.webhookSecretName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.webhook-secret-version-name" => Some(("secrets.webhookSecretVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ssl-ca" => Some(("sslCa", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "webhook-key" => Some(("webhookKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["app-id", "create-time", "display-name", "host-url", "name", "oauth-client-id-name", "oauth-client-id-version-name", "oauth-secret-name", "oauth-secret-version-name", "peered-network", "private-key-name", "private-key-version-name", "secrets", "ssl-ca", "webhook-key", "webhook-secret-name", "webhook-secret-version-name"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GitHubEnterpriseConfig = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_github_enterprise_configs_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
                },
                "ghe-config-id" => {
                    call = call.ghe_config_id(value.unwrap_or(""));
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
                                                                           v.extend(["ghe-config-id", "project-id"].iter().map(|v|*v));
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

    async fn _projects_locations_github_enterprise_configs_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_github_enterprise_configs_delete(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
                },
                "config-id" => {
                    call = call.config_id(value.unwrap_or(""));
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
                                                                           v.extend(["config-id", "project-id"].iter().map(|v|*v));
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

    async fn _projects_locations_github_enterprise_configs_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_github_enterprise_configs_get(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
                },
                "config-id" => {
                    call = call.config_id(value.unwrap_or(""));
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
                                                                           v.extend(["config-id", "project-id"].iter().map(|v|*v));
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

    async fn _projects_locations_github_enterprise_configs_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_github_enterprise_configs_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
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
                                                                           v.extend(["project-id"].iter().map(|v|*v));
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

    async fn _projects_locations_github_enterprise_configs_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "app-id" => Some(("appId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "host-url" => Some(("hostUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "peered-network" => Some(("peeredNetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.oauth-client-id-name" => Some(("secrets.oauthClientIdName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.oauth-client-id-version-name" => Some(("secrets.oauthClientIdVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.oauth-secret-name" => Some(("secrets.oauthSecretName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.oauth-secret-version-name" => Some(("secrets.oauthSecretVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.private-key-name" => Some(("secrets.privateKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.private-key-version-name" => Some(("secrets.privateKeyVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.webhook-secret-name" => Some(("secrets.webhookSecretName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secrets.webhook-secret-version-name" => Some(("secrets.webhookSecretVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ssl-ca" => Some(("sslCa", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "webhook-key" => Some(("webhookKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["app-id", "create-time", "display-name", "host-url", "name", "oauth-client-id-name", "oauth-client-id-version-name", "oauth-secret-name", "oauth-secret-version-name", "peered-network", "private-key-name", "private-key-version-name", "secrets", "ssl-ca", "webhook-key", "webhook-secret-name", "webhook-secret-version-name"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GitHubEnterpriseConfig = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_github_enterprise_configs_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_operations_cancel(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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

    async fn _projects_locations_triggers_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "approval-config.approval-required" => Some(("approvalConfig.approvalRequired", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "autodetect" => Some(("autodetect", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.api-key" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.apiKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.create-time" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.host-uri" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.hostUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.name" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.peered-network" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.peeredNetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.peered-network-ip-range" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.peeredNetworkIpRange", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.secrets.admin-access-token-version-name" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.secrets.adminAccessTokenVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.secrets.read-access-token-version-name" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.secrets.readAccessTokenVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.secrets.webhook-secret-version-name" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.secrets.webhookSecretVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.ssl-ca" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.sslCa", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.username" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.webhook-key" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.webhookKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config-resource" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfigResource", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.project-key" => Some(("bitbucketServerTriggerConfig.projectKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.pull-request.branch" => Some(("bitbucketServerTriggerConfig.pullRequest.branch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.pull-request.comment-control" => Some(("bitbucketServerTriggerConfig.pullRequest.commentControl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.pull-request.invert-regex" => Some(("bitbucketServerTriggerConfig.pullRequest.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.push.branch" => Some(("bitbucketServerTriggerConfig.push.branch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.push.invert-regex" => Some(("bitbucketServerTriggerConfig.push.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.push.tag" => Some(("bitbucketServerTriggerConfig.push.tag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.repo-slug" => Some(("bitbucketServerTriggerConfig.repoSlug", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.approval.config.approval-required" => Some(("build.approval.config.approvalRequired", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.approval.result.approval-time" => Some(("build.approval.result.approvalTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.approval.result.approver-account" => Some(("build.approval.result.approverAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.approval.result.comment" => Some(("build.approval.result.comment", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.approval.result.decision" => Some(("build.approval.result.decision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.approval.result.url" => Some(("build.approval.result.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.approval.state" => Some(("build.approval.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.artifacts.images" => Some(("build.artifacts.images", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.artifacts.objects.location" => Some(("build.artifacts.objects.location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.artifacts.objects.paths" => Some(("build.artifacts.objects.paths", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.artifacts.objects.timing.end-time" => Some(("build.artifacts.objects.timing.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.artifacts.objects.timing.start-time" => Some(("build.artifacts.objects.timing.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.build-trigger-id" => Some(("build.buildTriggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.create-time" => Some(("build.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.failure-info.detail" => Some(("build.failureInfo.detail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.failure-info.type" => Some(("build.failureInfo.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.finish-time" => Some(("build.finishTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.git-config.http.proxy-secret-version-name" => Some(("build.gitConfig.http.proxySecretVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.git-config.http.proxy-ssl-ca-info.bucket" => Some(("build.gitConfig.http.proxySslCaInfo.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.git-config.http.proxy-ssl-ca-info.generation" => Some(("build.gitConfig.http.proxySslCaInfo.generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.git-config.http.proxy-ssl-ca-info.object" => Some(("build.gitConfig.http.proxySslCaInfo.object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.id" => Some(("build.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.images" => Some(("build.images", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.log-url" => Some(("build.logUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.logs-bucket" => Some(("build.logsBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.name" => Some(("build.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.automap-substitutions" => Some(("build.options.automapSubstitutions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.options.default-logs-bucket-behavior" => Some(("build.options.defaultLogsBucketBehavior", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.disk-size-gb" => Some(("build.options.diskSizeGb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.dynamic-substitutions" => Some(("build.options.dynamicSubstitutions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.options.env" => Some(("build.options.env", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.options.log-streaming-option" => Some(("build.options.logStreamingOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.logging" => Some(("build.options.logging", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.machine-type" => Some(("build.options.machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.pool.name" => Some(("build.options.pool.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.requested-verify-option" => Some(("build.options.requestedVerifyOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.secret-env" => Some(("build.options.secretEnv", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.options.source-provenance-hash" => Some(("build.options.sourceProvenanceHash", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.options.substitution-option" => Some(("build.options.substitutionOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.worker-pool" => Some(("build.options.workerPool", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.project-id" => Some(("build.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.queue-ttl" => Some(("build.queueTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.results.artifact-manifest" => Some(("build.results.artifactManifest", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.results.artifact-timing.end-time" => Some(("build.results.artifactTiming.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.results.artifact-timing.start-time" => Some(("build.results.artifactTiming.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.results.build-step-images" => Some(("build.results.buildStepImages", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.results.build-step-outputs" => Some(("build.results.buildStepOutputs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.results.num-artifacts" => Some(("build.results.numArtifacts", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.service-account" => Some(("build.serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.connected-repository.dir" => Some(("build.source.connectedRepository.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.connected-repository.repository" => Some(("build.source.connectedRepository.repository", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.connected-repository.revision" => Some(("build.source.connectedRepository.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.developer-connect-config.dir" => Some(("build.source.developerConnectConfig.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.developer-connect-config.git-repository-link" => Some(("build.source.developerConnectConfig.gitRepositoryLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.developer-connect-config.revision" => Some(("build.source.developerConnectConfig.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.git-source.dir" => Some(("build.source.gitSource.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.git-source.revision" => Some(("build.source.gitSource.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.git-source.url" => Some(("build.source.gitSource.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.repo-source.branch-name" => Some(("build.source.repoSource.branchName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.repo-source.commit-sha" => Some(("build.source.repoSource.commitSha", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.repo-source.dir" => Some(("build.source.repoSource.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.repo-source.invert-regex" => Some(("build.source.repoSource.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.source.repo-source.project-id" => Some(("build.source.repoSource.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.repo-source.repo-name" => Some(("build.source.repoSource.repoName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.repo-source.substitutions" => Some(("build.source.repoSource.substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "build.source.repo-source.tag-name" => Some(("build.source.repoSource.tagName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.storage-source.bucket" => Some(("build.source.storageSource.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.storage-source.generation" => Some(("build.source.storageSource.generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.storage-source.object" => Some(("build.source.storageSource.object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.storage-source.source-fetcher" => Some(("build.source.storageSource.sourceFetcher", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.storage-source-manifest.bucket" => Some(("build.source.storageSourceManifest.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.storage-source-manifest.generation" => Some(("build.source.storageSourceManifest.generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.storage-source-manifest.object" => Some(("build.source.storageSourceManifest.object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-connected-repository.dir" => Some(("build.sourceProvenance.resolvedConnectedRepository.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-connected-repository.repository" => Some(("build.sourceProvenance.resolvedConnectedRepository.repository", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-connected-repository.revision" => Some(("build.sourceProvenance.resolvedConnectedRepository.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-git-source.dir" => Some(("build.sourceProvenance.resolvedGitSource.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-git-source.revision" => Some(("build.sourceProvenance.resolvedGitSource.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-git-source.url" => Some(("build.sourceProvenance.resolvedGitSource.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.branch-name" => Some(("build.sourceProvenance.resolvedRepoSource.branchName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.commit-sha" => Some(("build.sourceProvenance.resolvedRepoSource.commitSha", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.dir" => Some(("build.sourceProvenance.resolvedRepoSource.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.invert-regex" => Some(("build.sourceProvenance.resolvedRepoSource.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.project-id" => Some(("build.sourceProvenance.resolvedRepoSource.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.repo-name" => Some(("build.sourceProvenance.resolvedRepoSource.repoName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.substitutions" => Some(("build.sourceProvenance.resolvedRepoSource.substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "build.source-provenance.resolved-repo-source.tag-name" => Some(("build.sourceProvenance.resolvedRepoSource.tagName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-storage-source.bucket" => Some(("build.sourceProvenance.resolvedStorageSource.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-storage-source.generation" => Some(("build.sourceProvenance.resolvedStorageSource.generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-storage-source.object" => Some(("build.sourceProvenance.resolvedStorageSource.object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-storage-source.source-fetcher" => Some(("build.sourceProvenance.resolvedStorageSource.sourceFetcher", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-storage-source-manifest.bucket" => Some(("build.sourceProvenance.resolvedStorageSourceManifest.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-storage-source-manifest.generation" => Some(("build.sourceProvenance.resolvedStorageSourceManifest.generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-storage-source-manifest.object" => Some(("build.sourceProvenance.resolvedStorageSourceManifest.object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.start-time" => Some(("build.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.status" => Some(("build.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.status-detail" => Some(("build.statusDetail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.substitutions" => Some(("build.substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "build.tags" => Some(("build.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.timeout" => Some(("build.timeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "disabled" => Some(("disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "event-type" => Some(("eventType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "filename" => Some(("filename", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "filter" => Some(("filter", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-file-source.bitbucket-server-config" => Some(("gitFileSource.bitbucketServerConfig", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-file-source.github-enterprise-config" => Some(("gitFileSource.githubEnterpriseConfig", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-file-source.path" => Some(("gitFileSource.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-file-source.repo-type" => Some(("gitFileSource.repoType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-file-source.repository" => Some(("gitFileSource.repository", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-file-source.revision" => Some(("gitFileSource.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-file-source.uri" => Some(("gitFileSource.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.enterprise-config-resource-name" => Some(("github.enterpriseConfigResourceName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.installation-id" => Some(("github.installationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.name" => Some(("github.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.owner" => Some(("github.owner", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.pull-request.branch" => Some(("github.pullRequest.branch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.pull-request.comment-control" => Some(("github.pullRequest.commentControl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.pull-request.invert-regex" => Some(("github.pullRequest.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "github.push.branch" => Some(("github.push.branch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.push.invert-regex" => Some(("github.push.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "github.push.tag" => Some(("github.push.tag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.create-time" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.enterprise-config.host-uri" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.enterpriseConfig.hostUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.enterprise-config.service-directory-config.service" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.enterpriseConfig.serviceDirectoryConfig.service", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.enterprise-config.ssl-ca" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.enterpriseConfig.sslCa", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.name" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.secrets.api-access-token-version" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.secrets.apiAccessTokenVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.secrets.api-key-version" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.secrets.apiKeyVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.secrets.read-access-token-version" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.secrets.readAccessTokenVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.secrets.webhook-secret-version" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.secrets.webhookSecretVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.username" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.webhook-key" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.webhookKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config-resource" => Some(("gitlabEnterpriseEventsConfig.gitlabConfigResource", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.project-namespace" => Some(("gitlabEnterpriseEventsConfig.projectNamespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.pull-request.branch" => Some(("gitlabEnterpriseEventsConfig.pullRequest.branch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.pull-request.comment-control" => Some(("gitlabEnterpriseEventsConfig.pullRequest.commentControl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.pull-request.invert-regex" => Some(("gitlabEnterpriseEventsConfig.pullRequest.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.push.branch" => Some(("gitlabEnterpriseEventsConfig.push.branch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.push.invert-regex" => Some(("gitlabEnterpriseEventsConfig.push.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.push.tag" => Some(("gitlabEnterpriseEventsConfig.push.tag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ignored-files" => Some(("ignoredFiles", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "include-build-logs" => Some(("includeBuildLogs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "included-files" => Some(("includedFiles", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pubsub-config.service-account-email" => Some(("pubsubConfig.serviceAccountEmail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pubsub-config.state" => Some(("pubsubConfig.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pubsub-config.subscription" => Some(("pubsubConfig.subscription", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pubsub-config.topic" => Some(("pubsubConfig.topic", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "repository-event-config.pull-request.branch" => Some(("repositoryEventConfig.pullRequest.branch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "repository-event-config.pull-request.comment-control" => Some(("repositoryEventConfig.pullRequest.commentControl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "repository-event-config.pull-request.invert-regex" => Some(("repositoryEventConfig.pullRequest.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "repository-event-config.push.branch" => Some(("repositoryEventConfig.push.branch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "repository-event-config.push.invert-regex" => Some(("repositoryEventConfig.push.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "repository-event-config.push.tag" => Some(("repositoryEventConfig.push.tag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "repository-event-config.repository" => Some(("repositoryEventConfig.repository", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "repository-event-config.repository-type" => Some(("repositoryEventConfig.repositoryType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "resource-name" => Some(("resourceName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-account" => Some(("serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-to-build.bitbucket-server-config" => Some(("sourceToBuild.bitbucketServerConfig", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-to-build.github-enterprise-config" => Some(("sourceToBuild.githubEnterpriseConfig", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-to-build.ref" => Some(("sourceToBuild.ref", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-to-build.repo-type" => Some(("sourceToBuild.repoType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-to-build.repository" => Some(("sourceToBuild.repository", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-to-build.uri" => Some(("sourceToBuild.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "substitutions" => Some(("substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "tags" => Some(("tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "trigger-template.branch-name" => Some(("triggerTemplate.branchName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger-template.commit-sha" => Some(("triggerTemplate.commitSha", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger-template.dir" => Some(("triggerTemplate.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger-template.invert-regex" => Some(("triggerTemplate.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "trigger-template.project-id" => Some(("triggerTemplate.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger-template.repo-name" => Some(("triggerTemplate.repoName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger-template.substitutions" => Some(("triggerTemplate.substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "trigger-template.tag-name" => Some(("triggerTemplate.tagName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "webhook-config.secret" => Some(("webhookConfig.secret", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "webhook-config.state" => Some(("webhookConfig.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["admin-access-token-version-name", "api-access-token-version", "api-key", "api-key-version", "approval", "approval-config", "approval-required", "approval-time", "approver-account", "artifact-manifest", "artifact-timing", "artifacts", "autodetect", "automap-substitutions", "bitbucket-server-config", "bitbucket-server-config-resource", "bitbucket-server-trigger-config", "branch", "branch-name", "bucket", "build", "build-step-images", "build-step-outputs", "build-trigger-id", "comment", "comment-control", "commit-sha", "config", "connected-repository", "create-time", "decision", "default-logs-bucket-behavior", "description", "detail", "developer-connect-config", "dir", "disabled", "disk-size-gb", "dynamic-substitutions", "end-time", "enterprise-config", "enterprise-config-resource-name", "env", "event-type", "failure-info", "filename", "filter", "finish-time", "generation", "git-config", "git-file-source", "git-repository-link", "git-source", "github", "github-enterprise-config", "gitlab-config", "gitlab-config-resource", "gitlab-enterprise-events-config", "host-uri", "http", "id", "ignored-files", "images", "include-build-logs", "included-files", "installation-id", "invert-regex", "location", "log-streaming-option", "log-url", "logging", "logs-bucket", "machine-type", "name", "num-artifacts", "object", "objects", "options", "owner", "path", "paths", "peered-network", "peered-network-ip-range", "pool", "project-id", "project-key", "project-namespace", "proxy-secret-version-name", "proxy-ssl-ca-info", "pubsub-config", "pull-request", "push", "queue-ttl", "read-access-token-version", "read-access-token-version-name", "ref", "repo-name", "repo-slug", "repo-source", "repo-type", "repository", "repository-event-config", "repository-type", "requested-verify-option", "resolved-connected-repository", "resolved-git-source", "resolved-repo-source", "resolved-storage-source", "resolved-storage-source-manifest", "resource-name", "result", "results", "revision", "secret", "secret-env", "secrets", "service", "service-account", "service-account-email", "service-directory-config", "source", "source-fetcher", "source-provenance", "source-provenance-hash", "source-to-build", "ssl-ca", "start-time", "state", "status", "status-detail", "storage-source", "storage-source-manifest", "subscription", "substitution-option", "substitutions", "tag", "tag-name", "tags", "timeout", "timing", "topic", "trigger-template", "type", "uri", "url", "username", "webhook-config", "webhook-key", "webhook-secret-version", "webhook-secret-version-name", "worker-pool"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::BuildTrigger = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_triggers_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
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
                                                                           v.extend(["project-id"].iter().map(|v|*v));
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

    async fn _projects_locations_triggers_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_triggers_delete(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "trigger-id" => {
                    call = call.trigger_id(value.unwrap_or(""));
                },
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
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
                                                                           v.extend(["project-id", "trigger-id"].iter().map(|v|*v));
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

    async fn _projects_locations_triggers_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_triggers_get(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "trigger-id" => {
                    call = call.trigger_id(value.unwrap_or(""));
                },
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
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
                                                                           v.extend(["project-id", "trigger-id"].iter().map(|v|*v));
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

    async fn _projects_locations_triggers_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_triggers_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
                },
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
                                                                           v.extend(["page-size", "page-token", "project-id"].iter().map(|v|*v));
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

    async fn _projects_locations_triggers_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "approval-config.approval-required" => Some(("approvalConfig.approvalRequired", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "autodetect" => Some(("autodetect", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.api-key" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.apiKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.create-time" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.host-uri" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.hostUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.name" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.peered-network" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.peeredNetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.peered-network-ip-range" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.peeredNetworkIpRange", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.secrets.admin-access-token-version-name" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.secrets.adminAccessTokenVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.secrets.read-access-token-version-name" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.secrets.readAccessTokenVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.secrets.webhook-secret-version-name" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.secrets.webhookSecretVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.ssl-ca" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.sslCa", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.username" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.webhook-key" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.webhookKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config-resource" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfigResource", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.project-key" => Some(("bitbucketServerTriggerConfig.projectKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.pull-request.branch" => Some(("bitbucketServerTriggerConfig.pullRequest.branch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.pull-request.comment-control" => Some(("bitbucketServerTriggerConfig.pullRequest.commentControl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.pull-request.invert-regex" => Some(("bitbucketServerTriggerConfig.pullRequest.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.push.branch" => Some(("bitbucketServerTriggerConfig.push.branch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.push.invert-regex" => Some(("bitbucketServerTriggerConfig.push.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.push.tag" => Some(("bitbucketServerTriggerConfig.push.tag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.repo-slug" => Some(("bitbucketServerTriggerConfig.repoSlug", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.approval.config.approval-required" => Some(("build.approval.config.approvalRequired", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.approval.result.approval-time" => Some(("build.approval.result.approvalTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.approval.result.approver-account" => Some(("build.approval.result.approverAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.approval.result.comment" => Some(("build.approval.result.comment", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.approval.result.decision" => Some(("build.approval.result.decision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.approval.result.url" => Some(("build.approval.result.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.approval.state" => Some(("build.approval.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.artifacts.images" => Some(("build.artifacts.images", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.artifacts.objects.location" => Some(("build.artifacts.objects.location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.artifacts.objects.paths" => Some(("build.artifacts.objects.paths", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.artifacts.objects.timing.end-time" => Some(("build.artifacts.objects.timing.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.artifacts.objects.timing.start-time" => Some(("build.artifacts.objects.timing.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.build-trigger-id" => Some(("build.buildTriggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.create-time" => Some(("build.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.failure-info.detail" => Some(("build.failureInfo.detail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.failure-info.type" => Some(("build.failureInfo.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.finish-time" => Some(("build.finishTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.git-config.http.proxy-secret-version-name" => Some(("build.gitConfig.http.proxySecretVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.git-config.http.proxy-ssl-ca-info.bucket" => Some(("build.gitConfig.http.proxySslCaInfo.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.git-config.http.proxy-ssl-ca-info.generation" => Some(("build.gitConfig.http.proxySslCaInfo.generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.git-config.http.proxy-ssl-ca-info.object" => Some(("build.gitConfig.http.proxySslCaInfo.object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.id" => Some(("build.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.images" => Some(("build.images", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.log-url" => Some(("build.logUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.logs-bucket" => Some(("build.logsBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.name" => Some(("build.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.automap-substitutions" => Some(("build.options.automapSubstitutions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.options.default-logs-bucket-behavior" => Some(("build.options.defaultLogsBucketBehavior", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.disk-size-gb" => Some(("build.options.diskSizeGb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.dynamic-substitutions" => Some(("build.options.dynamicSubstitutions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.options.env" => Some(("build.options.env", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.options.log-streaming-option" => Some(("build.options.logStreamingOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.logging" => Some(("build.options.logging", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.machine-type" => Some(("build.options.machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.pool.name" => Some(("build.options.pool.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.requested-verify-option" => Some(("build.options.requestedVerifyOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.secret-env" => Some(("build.options.secretEnv", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.options.source-provenance-hash" => Some(("build.options.sourceProvenanceHash", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.options.substitution-option" => Some(("build.options.substitutionOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.worker-pool" => Some(("build.options.workerPool", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.project-id" => Some(("build.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.queue-ttl" => Some(("build.queueTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.results.artifact-manifest" => Some(("build.results.artifactManifest", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.results.artifact-timing.end-time" => Some(("build.results.artifactTiming.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.results.artifact-timing.start-time" => Some(("build.results.artifactTiming.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.results.build-step-images" => Some(("build.results.buildStepImages", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.results.build-step-outputs" => Some(("build.results.buildStepOutputs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.results.num-artifacts" => Some(("build.results.numArtifacts", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.service-account" => Some(("build.serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.connected-repository.dir" => Some(("build.source.connectedRepository.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.connected-repository.repository" => Some(("build.source.connectedRepository.repository", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.connected-repository.revision" => Some(("build.source.connectedRepository.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.developer-connect-config.dir" => Some(("build.source.developerConnectConfig.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.developer-connect-config.git-repository-link" => Some(("build.source.developerConnectConfig.gitRepositoryLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.developer-connect-config.revision" => Some(("build.source.developerConnectConfig.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.git-source.dir" => Some(("build.source.gitSource.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.git-source.revision" => Some(("build.source.gitSource.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.git-source.url" => Some(("build.source.gitSource.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.repo-source.branch-name" => Some(("build.source.repoSource.branchName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.repo-source.commit-sha" => Some(("build.source.repoSource.commitSha", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.repo-source.dir" => Some(("build.source.repoSource.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.repo-source.invert-regex" => Some(("build.source.repoSource.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.source.repo-source.project-id" => Some(("build.source.repoSource.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.repo-source.repo-name" => Some(("build.source.repoSource.repoName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.repo-source.substitutions" => Some(("build.source.repoSource.substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "build.source.repo-source.tag-name" => Some(("build.source.repoSource.tagName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.storage-source.bucket" => Some(("build.source.storageSource.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.storage-source.generation" => Some(("build.source.storageSource.generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.storage-source.object" => Some(("build.source.storageSource.object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.storage-source.source-fetcher" => Some(("build.source.storageSource.sourceFetcher", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.storage-source-manifest.bucket" => Some(("build.source.storageSourceManifest.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.storage-source-manifest.generation" => Some(("build.source.storageSourceManifest.generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.storage-source-manifest.object" => Some(("build.source.storageSourceManifest.object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-connected-repository.dir" => Some(("build.sourceProvenance.resolvedConnectedRepository.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-connected-repository.repository" => Some(("build.sourceProvenance.resolvedConnectedRepository.repository", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-connected-repository.revision" => Some(("build.sourceProvenance.resolvedConnectedRepository.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-git-source.dir" => Some(("build.sourceProvenance.resolvedGitSource.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-git-source.revision" => Some(("build.sourceProvenance.resolvedGitSource.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-git-source.url" => Some(("build.sourceProvenance.resolvedGitSource.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.branch-name" => Some(("build.sourceProvenance.resolvedRepoSource.branchName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.commit-sha" => Some(("build.sourceProvenance.resolvedRepoSource.commitSha", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.dir" => Some(("build.sourceProvenance.resolvedRepoSource.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.invert-regex" => Some(("build.sourceProvenance.resolvedRepoSource.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.project-id" => Some(("build.sourceProvenance.resolvedRepoSource.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.repo-name" => Some(("build.sourceProvenance.resolvedRepoSource.repoName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.substitutions" => Some(("build.sourceProvenance.resolvedRepoSource.substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "build.source-provenance.resolved-repo-source.tag-name" => Some(("build.sourceProvenance.resolvedRepoSource.tagName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-storage-source.bucket" => Some(("build.sourceProvenance.resolvedStorageSource.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-storage-source.generation" => Some(("build.sourceProvenance.resolvedStorageSource.generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-storage-source.object" => Some(("build.sourceProvenance.resolvedStorageSource.object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-storage-source.source-fetcher" => Some(("build.sourceProvenance.resolvedStorageSource.sourceFetcher", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-storage-source-manifest.bucket" => Some(("build.sourceProvenance.resolvedStorageSourceManifest.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-storage-source-manifest.generation" => Some(("build.sourceProvenance.resolvedStorageSourceManifest.generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-storage-source-manifest.object" => Some(("build.sourceProvenance.resolvedStorageSourceManifest.object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.start-time" => Some(("build.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.status" => Some(("build.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.status-detail" => Some(("build.statusDetail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.substitutions" => Some(("build.substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "build.tags" => Some(("build.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.timeout" => Some(("build.timeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "disabled" => Some(("disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "event-type" => Some(("eventType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "filename" => Some(("filename", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "filter" => Some(("filter", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-file-source.bitbucket-server-config" => Some(("gitFileSource.bitbucketServerConfig", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-file-source.github-enterprise-config" => Some(("gitFileSource.githubEnterpriseConfig", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-file-source.path" => Some(("gitFileSource.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-file-source.repo-type" => Some(("gitFileSource.repoType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-file-source.repository" => Some(("gitFileSource.repository", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-file-source.revision" => Some(("gitFileSource.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-file-source.uri" => Some(("gitFileSource.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.enterprise-config-resource-name" => Some(("github.enterpriseConfigResourceName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.installation-id" => Some(("github.installationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.name" => Some(("github.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.owner" => Some(("github.owner", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.pull-request.branch" => Some(("github.pullRequest.branch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.pull-request.comment-control" => Some(("github.pullRequest.commentControl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.pull-request.invert-regex" => Some(("github.pullRequest.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "github.push.branch" => Some(("github.push.branch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.push.invert-regex" => Some(("github.push.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "github.push.tag" => Some(("github.push.tag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.create-time" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.enterprise-config.host-uri" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.enterpriseConfig.hostUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.enterprise-config.service-directory-config.service" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.enterpriseConfig.serviceDirectoryConfig.service", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.enterprise-config.ssl-ca" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.enterpriseConfig.sslCa", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.name" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.secrets.api-access-token-version" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.secrets.apiAccessTokenVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.secrets.api-key-version" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.secrets.apiKeyVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.secrets.read-access-token-version" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.secrets.readAccessTokenVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.secrets.webhook-secret-version" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.secrets.webhookSecretVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.username" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.webhook-key" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.webhookKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config-resource" => Some(("gitlabEnterpriseEventsConfig.gitlabConfigResource", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.project-namespace" => Some(("gitlabEnterpriseEventsConfig.projectNamespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.pull-request.branch" => Some(("gitlabEnterpriseEventsConfig.pullRequest.branch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.pull-request.comment-control" => Some(("gitlabEnterpriseEventsConfig.pullRequest.commentControl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.pull-request.invert-regex" => Some(("gitlabEnterpriseEventsConfig.pullRequest.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.push.branch" => Some(("gitlabEnterpriseEventsConfig.push.branch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.push.invert-regex" => Some(("gitlabEnterpriseEventsConfig.push.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.push.tag" => Some(("gitlabEnterpriseEventsConfig.push.tag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ignored-files" => Some(("ignoredFiles", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "include-build-logs" => Some(("includeBuildLogs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "included-files" => Some(("includedFiles", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pubsub-config.service-account-email" => Some(("pubsubConfig.serviceAccountEmail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pubsub-config.state" => Some(("pubsubConfig.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pubsub-config.subscription" => Some(("pubsubConfig.subscription", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pubsub-config.topic" => Some(("pubsubConfig.topic", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "repository-event-config.pull-request.branch" => Some(("repositoryEventConfig.pullRequest.branch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "repository-event-config.pull-request.comment-control" => Some(("repositoryEventConfig.pullRequest.commentControl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "repository-event-config.pull-request.invert-regex" => Some(("repositoryEventConfig.pullRequest.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "repository-event-config.push.branch" => Some(("repositoryEventConfig.push.branch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "repository-event-config.push.invert-regex" => Some(("repositoryEventConfig.push.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "repository-event-config.push.tag" => Some(("repositoryEventConfig.push.tag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "repository-event-config.repository" => Some(("repositoryEventConfig.repository", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "repository-event-config.repository-type" => Some(("repositoryEventConfig.repositoryType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "resource-name" => Some(("resourceName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-account" => Some(("serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-to-build.bitbucket-server-config" => Some(("sourceToBuild.bitbucketServerConfig", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-to-build.github-enterprise-config" => Some(("sourceToBuild.githubEnterpriseConfig", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-to-build.ref" => Some(("sourceToBuild.ref", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-to-build.repo-type" => Some(("sourceToBuild.repoType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-to-build.repository" => Some(("sourceToBuild.repository", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-to-build.uri" => Some(("sourceToBuild.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "substitutions" => Some(("substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "tags" => Some(("tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "trigger-template.branch-name" => Some(("triggerTemplate.branchName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger-template.commit-sha" => Some(("triggerTemplate.commitSha", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger-template.dir" => Some(("triggerTemplate.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger-template.invert-regex" => Some(("triggerTemplate.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "trigger-template.project-id" => Some(("triggerTemplate.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger-template.repo-name" => Some(("triggerTemplate.repoName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger-template.substitutions" => Some(("triggerTemplate.substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "trigger-template.tag-name" => Some(("triggerTemplate.tagName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "webhook-config.secret" => Some(("webhookConfig.secret", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "webhook-config.state" => Some(("webhookConfig.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["admin-access-token-version-name", "api-access-token-version", "api-key", "api-key-version", "approval", "approval-config", "approval-required", "approval-time", "approver-account", "artifact-manifest", "artifact-timing", "artifacts", "autodetect", "automap-substitutions", "bitbucket-server-config", "bitbucket-server-config-resource", "bitbucket-server-trigger-config", "branch", "branch-name", "bucket", "build", "build-step-images", "build-step-outputs", "build-trigger-id", "comment", "comment-control", "commit-sha", "config", "connected-repository", "create-time", "decision", "default-logs-bucket-behavior", "description", "detail", "developer-connect-config", "dir", "disabled", "disk-size-gb", "dynamic-substitutions", "end-time", "enterprise-config", "enterprise-config-resource-name", "env", "event-type", "failure-info", "filename", "filter", "finish-time", "generation", "git-config", "git-file-source", "git-repository-link", "git-source", "github", "github-enterprise-config", "gitlab-config", "gitlab-config-resource", "gitlab-enterprise-events-config", "host-uri", "http", "id", "ignored-files", "images", "include-build-logs", "included-files", "installation-id", "invert-regex", "location", "log-streaming-option", "log-url", "logging", "logs-bucket", "machine-type", "name", "num-artifacts", "object", "objects", "options", "owner", "path", "paths", "peered-network", "peered-network-ip-range", "pool", "project-id", "project-key", "project-namespace", "proxy-secret-version-name", "proxy-ssl-ca-info", "pubsub-config", "pull-request", "push", "queue-ttl", "read-access-token-version", "read-access-token-version-name", "ref", "repo-name", "repo-slug", "repo-source", "repo-type", "repository", "repository-event-config", "repository-type", "requested-verify-option", "resolved-connected-repository", "resolved-git-source", "resolved-repo-source", "resolved-storage-source", "resolved-storage-source-manifest", "resource-name", "result", "results", "revision", "secret", "secret-env", "secrets", "service", "service-account", "service-account-email", "service-directory-config", "source", "source-fetcher", "source-provenance", "source-provenance-hash", "source-to-build", "ssl-ca", "start-time", "state", "status", "status-detail", "storage-source", "storage-source-manifest", "subscription", "substitution-option", "substitutions", "tag", "tag-name", "tags", "timeout", "timing", "topic", "trigger-template", "type", "uri", "url", "username", "webhook-config", "webhook-key", "webhook-secret-version", "webhook-secret-version-name", "worker-pool"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::BuildTrigger = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_triggers_patch(request, opt.value_of("resource-name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
                },
                "trigger-id" => {
                    call = call.trigger_id(value.unwrap_or(""));
                },
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
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
                                                                           v.extend(["project-id", "trigger-id", "update-mask"].iter().map(|v|*v));
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

    async fn _projects_locations_triggers_run(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "source.branch-name" => Some(("source.branchName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.commit-sha" => Some(("source.commitSha", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.dir" => Some(("source.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.invert-regex" => Some(("source.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "source.project-id" => Some(("source.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.repo-name" => Some(("source.repoName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source.substitutions" => Some(("source.substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "source.tag-name" => Some(("source.tagName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger-id" => Some(("triggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["branch-name", "commit-sha", "dir", "invert-regex", "project-id", "repo-name", "source", "substitutions", "tag-name", "trigger-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::RunBuildTriggerRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_triggers_run(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_triggers_webhook(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "content-type" => Some(("contentType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "data" => Some(("data", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["content-type", "data"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::HttpBody = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_triggers_webhook(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "trigger" => {
                    call = call.trigger(value.unwrap_or(""));
                },
                "secret" => {
                    call = call.secret(value.unwrap_or(""));
                },
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
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
                                                                           v.extend(["project-id", "secret", "trigger"].iter().map(|v|*v));
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

    async fn _projects_locations_worker_pools_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "delete-time" => Some(("deleteTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "private-pool-v1-config.network-config.egress-option" => Some(("privatePoolV1Config.networkConfig.egressOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "private-pool-v1-config.network-config.peered-network" => Some(("privatePoolV1Config.networkConfig.peeredNetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "private-pool-v1-config.network-config.peered-network-ip-range" => Some(("privatePoolV1Config.networkConfig.peeredNetworkIpRange", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "private-pool-v1-config.worker-config.disk-size-gb" => Some(("privatePoolV1Config.workerConfig.diskSizeGb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "private-pool-v1-config.worker-config.machine-type" => Some(("privatePoolV1Config.workerConfig.machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uid" => Some(("uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["annotations", "create-time", "delete-time", "disk-size-gb", "display-name", "egress-option", "etag", "machine-type", "name", "network-config", "peered-network", "peered-network-ip-range", "private-pool-v1-config", "state", "uid", "update-time", "worker-config"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::WorkerPool = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_worker_pools_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "worker-pool-id" => {
                    call = call.worker_pool_id(value.unwrap_or(""));
                },
                "validate-only" => {
                    call = call.validate_only(        value.map(|v| arg_from_str(v, err, "validate-only", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["validate-only", "worker-pool-id"].iter().map(|v|*v));
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

    async fn _projects_locations_worker_pools_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_worker_pools_delete(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "validate-only" => {
                    call = call.validate_only(        value.map(|v| arg_from_str(v, err, "validate-only", "boolean")).unwrap_or(false));
                },
                "etag" => {
                    call = call.etag(value.unwrap_or(""));
                },
                "allow-missing" => {
                    call = call.allow_missing(        value.map(|v| arg_from_str(v, err, "allow-missing", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["allow-missing", "etag", "validate-only"].iter().map(|v|*v));
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

    async fn _projects_locations_worker_pools_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_worker_pools_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_worker_pools_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_worker_pools_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_worker_pools_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "delete-time" => Some(("deleteTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "private-pool-v1-config.network-config.egress-option" => Some(("privatePoolV1Config.networkConfig.egressOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "private-pool-v1-config.network-config.peered-network" => Some(("privatePoolV1Config.networkConfig.peeredNetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "private-pool-v1-config.network-config.peered-network-ip-range" => Some(("privatePoolV1Config.networkConfig.peeredNetworkIpRange", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "private-pool-v1-config.worker-config.disk-size-gb" => Some(("privatePoolV1Config.workerConfig.diskSizeGb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "private-pool-v1-config.worker-config.machine-type" => Some(("privatePoolV1Config.workerConfig.machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uid" => Some(("uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["annotations", "create-time", "delete-time", "disk-size-gb", "display-name", "egress-option", "etag", "machine-type", "name", "network-config", "peered-network", "peered-network-ip-range", "private-pool-v1-config", "state", "uid", "update-time", "worker-config"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::WorkerPool = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_worker_pools_patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "validate-only" => {
                    call = call.validate_only(        value.map(|v| arg_from_str(v, err, "validate-only", "boolean")).unwrap_or(false));
                },
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
                                                                           v.extend(["update-mask", "validate-only"].iter().map(|v|*v));
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

    async fn _projects_triggers_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "approval-config.approval-required" => Some(("approvalConfig.approvalRequired", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "autodetect" => Some(("autodetect", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.api-key" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.apiKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.create-time" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.host-uri" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.hostUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.name" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.peered-network" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.peeredNetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.peered-network-ip-range" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.peeredNetworkIpRange", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.secrets.admin-access-token-version-name" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.secrets.adminAccessTokenVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.secrets.read-access-token-version-name" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.secrets.readAccessTokenVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.secrets.webhook-secret-version-name" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.secrets.webhookSecretVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.ssl-ca" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.sslCa", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.username" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.webhook-key" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.webhookKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config-resource" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfigResource", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.project-key" => Some(("bitbucketServerTriggerConfig.projectKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.pull-request.branch" => Some(("bitbucketServerTriggerConfig.pullRequest.branch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.pull-request.comment-control" => Some(("bitbucketServerTriggerConfig.pullRequest.commentControl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.pull-request.invert-regex" => Some(("bitbucketServerTriggerConfig.pullRequest.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.push.branch" => Some(("bitbucketServerTriggerConfig.push.branch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.push.invert-regex" => Some(("bitbucketServerTriggerConfig.push.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.push.tag" => Some(("bitbucketServerTriggerConfig.push.tag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.repo-slug" => Some(("bitbucketServerTriggerConfig.repoSlug", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.approval.config.approval-required" => Some(("build.approval.config.approvalRequired", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.approval.result.approval-time" => Some(("build.approval.result.approvalTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.approval.result.approver-account" => Some(("build.approval.result.approverAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.approval.result.comment" => Some(("build.approval.result.comment", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.approval.result.decision" => Some(("build.approval.result.decision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.approval.result.url" => Some(("build.approval.result.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.approval.state" => Some(("build.approval.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.artifacts.images" => Some(("build.artifacts.images", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.artifacts.objects.location" => Some(("build.artifacts.objects.location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.artifacts.objects.paths" => Some(("build.artifacts.objects.paths", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.artifacts.objects.timing.end-time" => Some(("build.artifacts.objects.timing.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.artifacts.objects.timing.start-time" => Some(("build.artifacts.objects.timing.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.build-trigger-id" => Some(("build.buildTriggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.create-time" => Some(("build.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.failure-info.detail" => Some(("build.failureInfo.detail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.failure-info.type" => Some(("build.failureInfo.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.finish-time" => Some(("build.finishTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.git-config.http.proxy-secret-version-name" => Some(("build.gitConfig.http.proxySecretVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.git-config.http.proxy-ssl-ca-info.bucket" => Some(("build.gitConfig.http.proxySslCaInfo.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.git-config.http.proxy-ssl-ca-info.generation" => Some(("build.gitConfig.http.proxySslCaInfo.generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.git-config.http.proxy-ssl-ca-info.object" => Some(("build.gitConfig.http.proxySslCaInfo.object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.id" => Some(("build.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.images" => Some(("build.images", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.log-url" => Some(("build.logUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.logs-bucket" => Some(("build.logsBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.name" => Some(("build.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.automap-substitutions" => Some(("build.options.automapSubstitutions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.options.default-logs-bucket-behavior" => Some(("build.options.defaultLogsBucketBehavior", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.disk-size-gb" => Some(("build.options.diskSizeGb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.dynamic-substitutions" => Some(("build.options.dynamicSubstitutions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.options.env" => Some(("build.options.env", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.options.log-streaming-option" => Some(("build.options.logStreamingOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.logging" => Some(("build.options.logging", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.machine-type" => Some(("build.options.machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.pool.name" => Some(("build.options.pool.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.requested-verify-option" => Some(("build.options.requestedVerifyOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.secret-env" => Some(("build.options.secretEnv", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.options.source-provenance-hash" => Some(("build.options.sourceProvenanceHash", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.options.substitution-option" => Some(("build.options.substitutionOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.worker-pool" => Some(("build.options.workerPool", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.project-id" => Some(("build.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.queue-ttl" => Some(("build.queueTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.results.artifact-manifest" => Some(("build.results.artifactManifest", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.results.artifact-timing.end-time" => Some(("build.results.artifactTiming.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.results.artifact-timing.start-time" => Some(("build.results.artifactTiming.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.results.build-step-images" => Some(("build.results.buildStepImages", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.results.build-step-outputs" => Some(("build.results.buildStepOutputs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.results.num-artifacts" => Some(("build.results.numArtifacts", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.service-account" => Some(("build.serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.connected-repository.dir" => Some(("build.source.connectedRepository.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.connected-repository.repository" => Some(("build.source.connectedRepository.repository", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.connected-repository.revision" => Some(("build.source.connectedRepository.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.developer-connect-config.dir" => Some(("build.source.developerConnectConfig.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.developer-connect-config.git-repository-link" => Some(("build.source.developerConnectConfig.gitRepositoryLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.developer-connect-config.revision" => Some(("build.source.developerConnectConfig.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.git-source.dir" => Some(("build.source.gitSource.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.git-source.revision" => Some(("build.source.gitSource.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.git-source.url" => Some(("build.source.gitSource.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.repo-source.branch-name" => Some(("build.source.repoSource.branchName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.repo-source.commit-sha" => Some(("build.source.repoSource.commitSha", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.repo-source.dir" => Some(("build.source.repoSource.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.repo-source.invert-regex" => Some(("build.source.repoSource.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.source.repo-source.project-id" => Some(("build.source.repoSource.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.repo-source.repo-name" => Some(("build.source.repoSource.repoName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.repo-source.substitutions" => Some(("build.source.repoSource.substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "build.source.repo-source.tag-name" => Some(("build.source.repoSource.tagName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.storage-source.bucket" => Some(("build.source.storageSource.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.storage-source.generation" => Some(("build.source.storageSource.generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.storage-source.object" => Some(("build.source.storageSource.object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.storage-source.source-fetcher" => Some(("build.source.storageSource.sourceFetcher", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.storage-source-manifest.bucket" => Some(("build.source.storageSourceManifest.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.storage-source-manifest.generation" => Some(("build.source.storageSourceManifest.generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.storage-source-manifest.object" => Some(("build.source.storageSourceManifest.object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-connected-repository.dir" => Some(("build.sourceProvenance.resolvedConnectedRepository.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-connected-repository.repository" => Some(("build.sourceProvenance.resolvedConnectedRepository.repository", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-connected-repository.revision" => Some(("build.sourceProvenance.resolvedConnectedRepository.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-git-source.dir" => Some(("build.sourceProvenance.resolvedGitSource.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-git-source.revision" => Some(("build.sourceProvenance.resolvedGitSource.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-git-source.url" => Some(("build.sourceProvenance.resolvedGitSource.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.branch-name" => Some(("build.sourceProvenance.resolvedRepoSource.branchName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.commit-sha" => Some(("build.sourceProvenance.resolvedRepoSource.commitSha", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.dir" => Some(("build.sourceProvenance.resolvedRepoSource.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.invert-regex" => Some(("build.sourceProvenance.resolvedRepoSource.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.project-id" => Some(("build.sourceProvenance.resolvedRepoSource.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.repo-name" => Some(("build.sourceProvenance.resolvedRepoSource.repoName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.substitutions" => Some(("build.sourceProvenance.resolvedRepoSource.substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "build.source-provenance.resolved-repo-source.tag-name" => Some(("build.sourceProvenance.resolvedRepoSource.tagName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-storage-source.bucket" => Some(("build.sourceProvenance.resolvedStorageSource.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-storage-source.generation" => Some(("build.sourceProvenance.resolvedStorageSource.generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-storage-source.object" => Some(("build.sourceProvenance.resolvedStorageSource.object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-storage-source.source-fetcher" => Some(("build.sourceProvenance.resolvedStorageSource.sourceFetcher", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-storage-source-manifest.bucket" => Some(("build.sourceProvenance.resolvedStorageSourceManifest.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-storage-source-manifest.generation" => Some(("build.sourceProvenance.resolvedStorageSourceManifest.generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-storage-source-manifest.object" => Some(("build.sourceProvenance.resolvedStorageSourceManifest.object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.start-time" => Some(("build.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.status" => Some(("build.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.status-detail" => Some(("build.statusDetail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.substitutions" => Some(("build.substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "build.tags" => Some(("build.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.timeout" => Some(("build.timeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "disabled" => Some(("disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "event-type" => Some(("eventType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "filename" => Some(("filename", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "filter" => Some(("filter", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-file-source.bitbucket-server-config" => Some(("gitFileSource.bitbucketServerConfig", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-file-source.github-enterprise-config" => Some(("gitFileSource.githubEnterpriseConfig", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-file-source.path" => Some(("gitFileSource.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-file-source.repo-type" => Some(("gitFileSource.repoType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-file-source.repository" => Some(("gitFileSource.repository", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-file-source.revision" => Some(("gitFileSource.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-file-source.uri" => Some(("gitFileSource.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.enterprise-config-resource-name" => Some(("github.enterpriseConfigResourceName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.installation-id" => Some(("github.installationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.name" => Some(("github.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.owner" => Some(("github.owner", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.pull-request.branch" => Some(("github.pullRequest.branch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.pull-request.comment-control" => Some(("github.pullRequest.commentControl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.pull-request.invert-regex" => Some(("github.pullRequest.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "github.push.branch" => Some(("github.push.branch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.push.invert-regex" => Some(("github.push.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "github.push.tag" => Some(("github.push.tag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.create-time" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.enterprise-config.host-uri" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.enterpriseConfig.hostUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.enterprise-config.service-directory-config.service" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.enterpriseConfig.serviceDirectoryConfig.service", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.enterprise-config.ssl-ca" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.enterpriseConfig.sslCa", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.name" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.secrets.api-access-token-version" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.secrets.apiAccessTokenVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.secrets.api-key-version" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.secrets.apiKeyVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.secrets.read-access-token-version" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.secrets.readAccessTokenVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.secrets.webhook-secret-version" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.secrets.webhookSecretVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.username" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.webhook-key" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.webhookKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config-resource" => Some(("gitlabEnterpriseEventsConfig.gitlabConfigResource", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.project-namespace" => Some(("gitlabEnterpriseEventsConfig.projectNamespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.pull-request.branch" => Some(("gitlabEnterpriseEventsConfig.pullRequest.branch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.pull-request.comment-control" => Some(("gitlabEnterpriseEventsConfig.pullRequest.commentControl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.pull-request.invert-regex" => Some(("gitlabEnterpriseEventsConfig.pullRequest.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.push.branch" => Some(("gitlabEnterpriseEventsConfig.push.branch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.push.invert-regex" => Some(("gitlabEnterpriseEventsConfig.push.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.push.tag" => Some(("gitlabEnterpriseEventsConfig.push.tag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ignored-files" => Some(("ignoredFiles", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "include-build-logs" => Some(("includeBuildLogs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "included-files" => Some(("includedFiles", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pubsub-config.service-account-email" => Some(("pubsubConfig.serviceAccountEmail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pubsub-config.state" => Some(("pubsubConfig.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pubsub-config.subscription" => Some(("pubsubConfig.subscription", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pubsub-config.topic" => Some(("pubsubConfig.topic", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "repository-event-config.pull-request.branch" => Some(("repositoryEventConfig.pullRequest.branch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "repository-event-config.pull-request.comment-control" => Some(("repositoryEventConfig.pullRequest.commentControl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "repository-event-config.pull-request.invert-regex" => Some(("repositoryEventConfig.pullRequest.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "repository-event-config.push.branch" => Some(("repositoryEventConfig.push.branch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "repository-event-config.push.invert-regex" => Some(("repositoryEventConfig.push.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "repository-event-config.push.tag" => Some(("repositoryEventConfig.push.tag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "repository-event-config.repository" => Some(("repositoryEventConfig.repository", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "repository-event-config.repository-type" => Some(("repositoryEventConfig.repositoryType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "resource-name" => Some(("resourceName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-account" => Some(("serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-to-build.bitbucket-server-config" => Some(("sourceToBuild.bitbucketServerConfig", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-to-build.github-enterprise-config" => Some(("sourceToBuild.githubEnterpriseConfig", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-to-build.ref" => Some(("sourceToBuild.ref", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-to-build.repo-type" => Some(("sourceToBuild.repoType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-to-build.repository" => Some(("sourceToBuild.repository", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-to-build.uri" => Some(("sourceToBuild.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "substitutions" => Some(("substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "tags" => Some(("tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "trigger-template.branch-name" => Some(("triggerTemplate.branchName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger-template.commit-sha" => Some(("triggerTemplate.commitSha", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger-template.dir" => Some(("triggerTemplate.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger-template.invert-regex" => Some(("triggerTemplate.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "trigger-template.project-id" => Some(("triggerTemplate.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger-template.repo-name" => Some(("triggerTemplate.repoName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger-template.substitutions" => Some(("triggerTemplate.substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "trigger-template.tag-name" => Some(("triggerTemplate.tagName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "webhook-config.secret" => Some(("webhookConfig.secret", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "webhook-config.state" => Some(("webhookConfig.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["admin-access-token-version-name", "api-access-token-version", "api-key", "api-key-version", "approval", "approval-config", "approval-required", "approval-time", "approver-account", "artifact-manifest", "artifact-timing", "artifacts", "autodetect", "automap-substitutions", "bitbucket-server-config", "bitbucket-server-config-resource", "bitbucket-server-trigger-config", "branch", "branch-name", "bucket", "build", "build-step-images", "build-step-outputs", "build-trigger-id", "comment", "comment-control", "commit-sha", "config", "connected-repository", "create-time", "decision", "default-logs-bucket-behavior", "description", "detail", "developer-connect-config", "dir", "disabled", "disk-size-gb", "dynamic-substitutions", "end-time", "enterprise-config", "enterprise-config-resource-name", "env", "event-type", "failure-info", "filename", "filter", "finish-time", "generation", "git-config", "git-file-source", "git-repository-link", "git-source", "github", "github-enterprise-config", "gitlab-config", "gitlab-config-resource", "gitlab-enterprise-events-config", "host-uri", "http", "id", "ignored-files", "images", "include-build-logs", "included-files", "installation-id", "invert-regex", "location", "log-streaming-option", "log-url", "logging", "logs-bucket", "machine-type", "name", "num-artifacts", "object", "objects", "options", "owner", "path", "paths", "peered-network", "peered-network-ip-range", "pool", "project-id", "project-key", "project-namespace", "proxy-secret-version-name", "proxy-ssl-ca-info", "pubsub-config", "pull-request", "push", "queue-ttl", "read-access-token-version", "read-access-token-version-name", "ref", "repo-name", "repo-slug", "repo-source", "repo-type", "repository", "repository-event-config", "repository-type", "requested-verify-option", "resolved-connected-repository", "resolved-git-source", "resolved-repo-source", "resolved-storage-source", "resolved-storage-source-manifest", "resource-name", "result", "results", "revision", "secret", "secret-env", "secrets", "service", "service-account", "service-account-email", "service-directory-config", "source", "source-fetcher", "source-provenance", "source-provenance-hash", "source-to-build", "ssl-ca", "start-time", "state", "status", "status-detail", "storage-source", "storage-source-manifest", "subscription", "substitution-option", "substitutions", "tag", "tag-name", "tags", "timeout", "timing", "topic", "trigger-template", "type", "uri", "url", "username", "webhook-config", "webhook-key", "webhook-secret-version", "webhook-secret-version-name", "worker-pool"]);
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
                "parent" => {
                    call = call.parent(value.unwrap_or(""));
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
                                                                           v.extend(["parent"].iter().map(|v|*v));
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

    async fn _projects_triggers_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().triggers_delete(opt.value_of("project-id").unwrap_or(""), opt.value_of("trigger-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "name" => {
                    call = call.name(value.unwrap_or(""));
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
                                                                           v.extend(["name"].iter().map(|v|*v));
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

    async fn _projects_triggers_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().triggers_get(opt.value_of("project-id").unwrap_or(""), opt.value_of("trigger-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "name" => {
                    call = call.name(value.unwrap_or(""));
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
                                                                           v.extend(["name"].iter().map(|v|*v));
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

    async fn _projects_triggers_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().triggers_list(opt.value_of("project-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "parent" => {
                    call = call.parent(value.unwrap_or(""));
                },
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
                                                                           v.extend(["page-size", "page-token", "parent"].iter().map(|v|*v));
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

    async fn _projects_triggers_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "approval-config.approval-required" => Some(("approvalConfig.approvalRequired", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "autodetect" => Some(("autodetect", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.api-key" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.apiKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.create-time" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.host-uri" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.hostUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.name" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.peered-network" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.peeredNetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.peered-network-ip-range" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.peeredNetworkIpRange", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.secrets.admin-access-token-version-name" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.secrets.adminAccessTokenVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.secrets.read-access-token-version-name" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.secrets.readAccessTokenVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.secrets.webhook-secret-version-name" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.secrets.webhookSecretVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.ssl-ca" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.sslCa", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.username" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config.webhook-key" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfig.webhookKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.bitbucket-server-config-resource" => Some(("bitbucketServerTriggerConfig.bitbucketServerConfigResource", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.project-key" => Some(("bitbucketServerTriggerConfig.projectKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.pull-request.branch" => Some(("bitbucketServerTriggerConfig.pullRequest.branch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.pull-request.comment-control" => Some(("bitbucketServerTriggerConfig.pullRequest.commentControl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.pull-request.invert-regex" => Some(("bitbucketServerTriggerConfig.pullRequest.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.push.branch" => Some(("bitbucketServerTriggerConfig.push.branch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.push.invert-regex" => Some(("bitbucketServerTriggerConfig.push.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.push.tag" => Some(("bitbucketServerTriggerConfig.push.tag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bitbucket-server-trigger-config.repo-slug" => Some(("bitbucketServerTriggerConfig.repoSlug", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.approval.config.approval-required" => Some(("build.approval.config.approvalRequired", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.approval.result.approval-time" => Some(("build.approval.result.approvalTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.approval.result.approver-account" => Some(("build.approval.result.approverAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.approval.result.comment" => Some(("build.approval.result.comment", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.approval.result.decision" => Some(("build.approval.result.decision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.approval.result.url" => Some(("build.approval.result.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.approval.state" => Some(("build.approval.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.artifacts.images" => Some(("build.artifacts.images", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.artifacts.objects.location" => Some(("build.artifacts.objects.location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.artifacts.objects.paths" => Some(("build.artifacts.objects.paths", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.artifacts.objects.timing.end-time" => Some(("build.artifacts.objects.timing.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.artifacts.objects.timing.start-time" => Some(("build.artifacts.objects.timing.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.build-trigger-id" => Some(("build.buildTriggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.create-time" => Some(("build.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.failure-info.detail" => Some(("build.failureInfo.detail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.failure-info.type" => Some(("build.failureInfo.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.finish-time" => Some(("build.finishTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.git-config.http.proxy-secret-version-name" => Some(("build.gitConfig.http.proxySecretVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.git-config.http.proxy-ssl-ca-info.bucket" => Some(("build.gitConfig.http.proxySslCaInfo.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.git-config.http.proxy-ssl-ca-info.generation" => Some(("build.gitConfig.http.proxySslCaInfo.generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.git-config.http.proxy-ssl-ca-info.object" => Some(("build.gitConfig.http.proxySslCaInfo.object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.id" => Some(("build.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.images" => Some(("build.images", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.log-url" => Some(("build.logUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.logs-bucket" => Some(("build.logsBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.name" => Some(("build.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.automap-substitutions" => Some(("build.options.automapSubstitutions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.options.default-logs-bucket-behavior" => Some(("build.options.defaultLogsBucketBehavior", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.disk-size-gb" => Some(("build.options.diskSizeGb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.dynamic-substitutions" => Some(("build.options.dynamicSubstitutions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.options.env" => Some(("build.options.env", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.options.log-streaming-option" => Some(("build.options.logStreamingOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.logging" => Some(("build.options.logging", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.machine-type" => Some(("build.options.machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.pool.name" => Some(("build.options.pool.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.requested-verify-option" => Some(("build.options.requestedVerifyOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.secret-env" => Some(("build.options.secretEnv", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.options.source-provenance-hash" => Some(("build.options.sourceProvenanceHash", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.options.substitution-option" => Some(("build.options.substitutionOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.options.worker-pool" => Some(("build.options.workerPool", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.project-id" => Some(("build.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.queue-ttl" => Some(("build.queueTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.results.artifact-manifest" => Some(("build.results.artifactManifest", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.results.artifact-timing.end-time" => Some(("build.results.artifactTiming.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.results.artifact-timing.start-time" => Some(("build.results.artifactTiming.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.results.build-step-images" => Some(("build.results.buildStepImages", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.results.build-step-outputs" => Some(("build.results.buildStepOutputs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.results.num-artifacts" => Some(("build.results.numArtifacts", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.service-account" => Some(("build.serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.connected-repository.dir" => Some(("build.source.connectedRepository.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.connected-repository.repository" => Some(("build.source.connectedRepository.repository", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.connected-repository.revision" => Some(("build.source.connectedRepository.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.developer-connect-config.dir" => Some(("build.source.developerConnectConfig.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.developer-connect-config.git-repository-link" => Some(("build.source.developerConnectConfig.gitRepositoryLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.developer-connect-config.revision" => Some(("build.source.developerConnectConfig.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.git-source.dir" => Some(("build.source.gitSource.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.git-source.revision" => Some(("build.source.gitSource.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.git-source.url" => Some(("build.source.gitSource.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.repo-source.branch-name" => Some(("build.source.repoSource.branchName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.repo-source.commit-sha" => Some(("build.source.repoSource.commitSha", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.repo-source.dir" => Some(("build.source.repoSource.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.repo-source.invert-regex" => Some(("build.source.repoSource.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.source.repo-source.project-id" => Some(("build.source.repoSource.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.repo-source.repo-name" => Some(("build.source.repoSource.repoName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.repo-source.substitutions" => Some(("build.source.repoSource.substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "build.source.repo-source.tag-name" => Some(("build.source.repoSource.tagName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.storage-source.bucket" => Some(("build.source.storageSource.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.storage-source.generation" => Some(("build.source.storageSource.generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.storage-source.object" => Some(("build.source.storageSource.object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.storage-source.source-fetcher" => Some(("build.source.storageSource.sourceFetcher", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.storage-source-manifest.bucket" => Some(("build.source.storageSourceManifest.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.storage-source-manifest.generation" => Some(("build.source.storageSourceManifest.generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source.storage-source-manifest.object" => Some(("build.source.storageSourceManifest.object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-connected-repository.dir" => Some(("build.sourceProvenance.resolvedConnectedRepository.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-connected-repository.repository" => Some(("build.sourceProvenance.resolvedConnectedRepository.repository", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-connected-repository.revision" => Some(("build.sourceProvenance.resolvedConnectedRepository.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-git-source.dir" => Some(("build.sourceProvenance.resolvedGitSource.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-git-source.revision" => Some(("build.sourceProvenance.resolvedGitSource.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-git-source.url" => Some(("build.sourceProvenance.resolvedGitSource.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.branch-name" => Some(("build.sourceProvenance.resolvedRepoSource.branchName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.commit-sha" => Some(("build.sourceProvenance.resolvedRepoSource.commitSha", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.dir" => Some(("build.sourceProvenance.resolvedRepoSource.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.invert-regex" => Some(("build.sourceProvenance.resolvedRepoSource.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.project-id" => Some(("build.sourceProvenance.resolvedRepoSource.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.repo-name" => Some(("build.sourceProvenance.resolvedRepoSource.repoName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-repo-source.substitutions" => Some(("build.sourceProvenance.resolvedRepoSource.substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "build.source-provenance.resolved-repo-source.tag-name" => Some(("build.sourceProvenance.resolvedRepoSource.tagName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-storage-source.bucket" => Some(("build.sourceProvenance.resolvedStorageSource.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-storage-source.generation" => Some(("build.sourceProvenance.resolvedStorageSource.generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-storage-source.object" => Some(("build.sourceProvenance.resolvedStorageSource.object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-storage-source.source-fetcher" => Some(("build.sourceProvenance.resolvedStorageSource.sourceFetcher", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-storage-source-manifest.bucket" => Some(("build.sourceProvenance.resolvedStorageSourceManifest.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-storage-source-manifest.generation" => Some(("build.sourceProvenance.resolvedStorageSourceManifest.generation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.source-provenance.resolved-storage-source-manifest.object" => Some(("build.sourceProvenance.resolvedStorageSourceManifest.object", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.start-time" => Some(("build.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.status" => Some(("build.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.status-detail" => Some(("build.statusDetail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.substitutions" => Some(("build.substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "build.tags" => Some(("build.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.timeout" => Some(("build.timeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "disabled" => Some(("disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "event-type" => Some(("eventType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "filename" => Some(("filename", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "filter" => Some(("filter", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-file-source.bitbucket-server-config" => Some(("gitFileSource.bitbucketServerConfig", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-file-source.github-enterprise-config" => Some(("gitFileSource.githubEnterpriseConfig", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-file-source.path" => Some(("gitFileSource.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-file-source.repo-type" => Some(("gitFileSource.repoType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-file-source.repository" => Some(("gitFileSource.repository", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-file-source.revision" => Some(("gitFileSource.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "git-file-source.uri" => Some(("gitFileSource.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.enterprise-config-resource-name" => Some(("github.enterpriseConfigResourceName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.installation-id" => Some(("github.installationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.name" => Some(("github.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.owner" => Some(("github.owner", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.pull-request.branch" => Some(("github.pullRequest.branch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.pull-request.comment-control" => Some(("github.pullRequest.commentControl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.pull-request.invert-regex" => Some(("github.pullRequest.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "github.push.branch" => Some(("github.push.branch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "github.push.invert-regex" => Some(("github.push.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "github.push.tag" => Some(("github.push.tag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.create-time" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.enterprise-config.host-uri" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.enterpriseConfig.hostUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.enterprise-config.service-directory-config.service" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.enterpriseConfig.serviceDirectoryConfig.service", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.enterprise-config.ssl-ca" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.enterpriseConfig.sslCa", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.name" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.secrets.api-access-token-version" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.secrets.apiAccessTokenVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.secrets.api-key-version" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.secrets.apiKeyVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.secrets.read-access-token-version" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.secrets.readAccessTokenVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.secrets.webhook-secret-version" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.secrets.webhookSecretVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.username" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config.webhook-key" => Some(("gitlabEnterpriseEventsConfig.gitlabConfig.webhookKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.gitlab-config-resource" => Some(("gitlabEnterpriseEventsConfig.gitlabConfigResource", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.project-namespace" => Some(("gitlabEnterpriseEventsConfig.projectNamespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.pull-request.branch" => Some(("gitlabEnterpriseEventsConfig.pullRequest.branch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.pull-request.comment-control" => Some(("gitlabEnterpriseEventsConfig.pullRequest.commentControl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.pull-request.invert-regex" => Some(("gitlabEnterpriseEventsConfig.pullRequest.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.push.branch" => Some(("gitlabEnterpriseEventsConfig.push.branch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.push.invert-regex" => Some(("gitlabEnterpriseEventsConfig.push.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "gitlab-enterprise-events-config.push.tag" => Some(("gitlabEnterpriseEventsConfig.push.tag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ignored-files" => Some(("ignoredFiles", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "include-build-logs" => Some(("includeBuildLogs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "included-files" => Some(("includedFiles", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pubsub-config.service-account-email" => Some(("pubsubConfig.serviceAccountEmail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pubsub-config.state" => Some(("pubsubConfig.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pubsub-config.subscription" => Some(("pubsubConfig.subscription", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pubsub-config.topic" => Some(("pubsubConfig.topic", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "repository-event-config.pull-request.branch" => Some(("repositoryEventConfig.pullRequest.branch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "repository-event-config.pull-request.comment-control" => Some(("repositoryEventConfig.pullRequest.commentControl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "repository-event-config.pull-request.invert-regex" => Some(("repositoryEventConfig.pullRequest.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "repository-event-config.push.branch" => Some(("repositoryEventConfig.push.branch", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "repository-event-config.push.invert-regex" => Some(("repositoryEventConfig.push.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "repository-event-config.push.tag" => Some(("repositoryEventConfig.push.tag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "repository-event-config.repository" => Some(("repositoryEventConfig.repository", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "repository-event-config.repository-type" => Some(("repositoryEventConfig.repositoryType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "resource-name" => Some(("resourceName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-account" => Some(("serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-to-build.bitbucket-server-config" => Some(("sourceToBuild.bitbucketServerConfig", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-to-build.github-enterprise-config" => Some(("sourceToBuild.githubEnterpriseConfig", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-to-build.ref" => Some(("sourceToBuild.ref", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-to-build.repo-type" => Some(("sourceToBuild.repoType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-to-build.repository" => Some(("sourceToBuild.repository", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-to-build.uri" => Some(("sourceToBuild.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "substitutions" => Some(("substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "tags" => Some(("tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "trigger-template.branch-name" => Some(("triggerTemplate.branchName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger-template.commit-sha" => Some(("triggerTemplate.commitSha", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger-template.dir" => Some(("triggerTemplate.dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger-template.invert-regex" => Some(("triggerTemplate.invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "trigger-template.project-id" => Some(("triggerTemplate.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger-template.repo-name" => Some(("triggerTemplate.repoName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger-template.substitutions" => Some(("triggerTemplate.substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "trigger-template.tag-name" => Some(("triggerTemplate.tagName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "webhook-config.secret" => Some(("webhookConfig.secret", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "webhook-config.state" => Some(("webhookConfig.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["admin-access-token-version-name", "api-access-token-version", "api-key", "api-key-version", "approval", "approval-config", "approval-required", "approval-time", "approver-account", "artifact-manifest", "artifact-timing", "artifacts", "autodetect", "automap-substitutions", "bitbucket-server-config", "bitbucket-server-config-resource", "bitbucket-server-trigger-config", "branch", "branch-name", "bucket", "build", "build-step-images", "build-step-outputs", "build-trigger-id", "comment", "comment-control", "commit-sha", "config", "connected-repository", "create-time", "decision", "default-logs-bucket-behavior", "description", "detail", "developer-connect-config", "dir", "disabled", "disk-size-gb", "dynamic-substitutions", "end-time", "enterprise-config", "enterprise-config-resource-name", "env", "event-type", "failure-info", "filename", "filter", "finish-time", "generation", "git-config", "git-file-source", "git-repository-link", "git-source", "github", "github-enterprise-config", "gitlab-config", "gitlab-config-resource", "gitlab-enterprise-events-config", "host-uri", "http", "id", "ignored-files", "images", "include-build-logs", "included-files", "installation-id", "invert-regex", "location", "log-streaming-option", "log-url", "logging", "logs-bucket", "machine-type", "name", "num-artifacts", "object", "objects", "options", "owner", "path", "paths", "peered-network", "peered-network-ip-range", "pool", "project-id", "project-key", "project-namespace", "proxy-secret-version-name", "proxy-ssl-ca-info", "pubsub-config", "pull-request", "push", "queue-ttl", "read-access-token-version", "read-access-token-version-name", "ref", "repo-name", "repo-slug", "repo-source", "repo-type", "repository", "repository-event-config", "repository-type", "requested-verify-option", "resolved-connected-repository", "resolved-git-source", "resolved-repo-source", "resolved-storage-source", "resolved-storage-source-manifest", "resource-name", "result", "results", "revision", "secret", "secret-env", "secrets", "service", "service-account", "service-account-email", "service-directory-config", "source", "source-fetcher", "source-provenance", "source-provenance-hash", "source-to-build", "ssl-ca", "start-time", "state", "status", "status-detail", "storage-source", "storage-source-manifest", "subscription", "substitution-option", "substitutions", "tag", "tag-name", "tags", "timeout", "timing", "topic", "trigger-template", "type", "uri", "url", "username", "webhook-config", "webhook-key", "webhook-secret-version", "webhook-secret-version-name", "worker-pool"]);
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

    async fn _projects_triggers_run(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "branch-name" => Some(("branchName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "commit-sha" => Some(("commitSha", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dir" => Some(("dir", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "invert-regex" => Some(("invertRegex", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "repo-name" => Some(("repoName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "substitutions" => Some(("substitutions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "tag-name" => Some(("tagName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                "name" => {
                    call = call.name(value.unwrap_or(""));
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
                                                                           v.extend(["name"].iter().map(|v|*v));
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

    async fn _projects_triggers_webhook(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "content-type" => Some(("contentType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "data" => Some(("data", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["content-type", "data"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::HttpBody = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().triggers_webhook(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("trigger").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "secret" => {
                    call = call.secret(value.unwrap_or(""));
                },
                "name" => {
                    call = call.name(value.unwrap_or(""));
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
                                                                           v.extend(["name", "secret"].iter().map(|v|*v));
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
            ("github-dot-com-webhook", Some(opt)) => {
                match opt.subcommand() {
                    ("receive", Some(opt)) => {
                        call_result = self._github_dot_com_webhook_receive(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("github-dot-com-webhook".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("locations", Some(opt)) => {
                match opt.subcommand() {
                    ("regional-webhook", Some(opt)) => {
                        call_result = self._locations_regional_webhook(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("locations".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("methods", Some(opt)) => {
                match opt.subcommand() {
                    ("webhook", Some(opt)) => {
                        call_result = self._methods_webhook(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("methods".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("operations", Some(opt)) => {
                match opt.subcommand() {
                    ("cancel", Some(opt)) => {
                        call_result = self._operations_cancel(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._operations_get(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("operations".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("projects", Some(opt)) => {
                match opt.subcommand() {
                    ("builds-approve", Some(opt)) => {
                        call_result = self._projects_builds_approve(opt, dry_run, &mut err).await;
                    },
                    ("builds-cancel", Some(opt)) => {
                        call_result = self._projects_builds_cancel(opt, dry_run, &mut err).await;
                    },
                    ("builds-create", Some(opt)) => {
                        call_result = self._projects_builds_create(opt, dry_run, &mut err).await;
                    },
                    ("builds-get", Some(opt)) => {
                        call_result = self._projects_builds_get(opt, dry_run, &mut err).await;
                    },
                    ("builds-list", Some(opt)) => {
                        call_result = self._projects_builds_list(opt, dry_run, &mut err).await;
                    },
                    ("builds-retry", Some(opt)) => {
                        call_result = self._projects_builds_retry(opt, dry_run, &mut err).await;
                    },
                    ("github-enterprise-configs-create", Some(opt)) => {
                        call_result = self._projects_github_enterprise_configs_create(opt, dry_run, &mut err).await;
                    },
                    ("github-enterprise-configs-delete", Some(opt)) => {
                        call_result = self._projects_github_enterprise_configs_delete(opt, dry_run, &mut err).await;
                    },
                    ("github-enterprise-configs-get", Some(opt)) => {
                        call_result = self._projects_github_enterprise_configs_get(opt, dry_run, &mut err).await;
                    },
                    ("github-enterprise-configs-list", Some(opt)) => {
                        call_result = self._projects_github_enterprise_configs_list(opt, dry_run, &mut err).await;
                    },
                    ("github-enterprise-configs-patch", Some(opt)) => {
                        call_result = self._projects_github_enterprise_configs_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-bitbucket-server-configs-connected-repositories-batch-create", Some(opt)) => {
                        call_result = self._projects_locations_bitbucket_server_configs_connected_repositories_batch_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-bitbucket-server-configs-create", Some(opt)) => {
                        call_result = self._projects_locations_bitbucket_server_configs_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-bitbucket-server-configs-delete", Some(opt)) => {
                        call_result = self._projects_locations_bitbucket_server_configs_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-bitbucket-server-configs-get", Some(opt)) => {
                        call_result = self._projects_locations_bitbucket_server_configs_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-bitbucket-server-configs-list", Some(opt)) => {
                        call_result = self._projects_locations_bitbucket_server_configs_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-bitbucket-server-configs-patch", Some(opt)) => {
                        call_result = self._projects_locations_bitbucket_server_configs_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-bitbucket-server-configs-remove-bitbucket-server-connected-repository", Some(opt)) => {
                        call_result = self._projects_locations_bitbucket_server_configs_remove_bitbucket_server_connected_repository(opt, dry_run, &mut err).await;
                    },
                    ("locations-bitbucket-server-configs-repos-list", Some(opt)) => {
                        call_result = self._projects_locations_bitbucket_server_configs_repos_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-builds-approve", Some(opt)) => {
                        call_result = self._projects_locations_builds_approve(opt, dry_run, &mut err).await;
                    },
                    ("locations-builds-cancel", Some(opt)) => {
                        call_result = self._projects_locations_builds_cancel(opt, dry_run, &mut err).await;
                    },
                    ("locations-builds-create", Some(opt)) => {
                        call_result = self._projects_locations_builds_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-builds-get", Some(opt)) => {
                        call_result = self._projects_locations_builds_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-builds-list", Some(opt)) => {
                        call_result = self._projects_locations_builds_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-builds-retry", Some(opt)) => {
                        call_result = self._projects_locations_builds_retry(opt, dry_run, &mut err).await;
                    },
                    ("locations-get-default-service-account", Some(opt)) => {
                        call_result = self._projects_locations_get_default_service_account(opt, dry_run, &mut err).await;
                    },
                    ("locations-git-lab-configs-connected-repositories-batch-create", Some(opt)) => {
                        call_result = self._projects_locations_git_lab_configs_connected_repositories_batch_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-git-lab-configs-create", Some(opt)) => {
                        call_result = self._projects_locations_git_lab_configs_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-git-lab-configs-delete", Some(opt)) => {
                        call_result = self._projects_locations_git_lab_configs_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-git-lab-configs-get", Some(opt)) => {
                        call_result = self._projects_locations_git_lab_configs_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-git-lab-configs-list", Some(opt)) => {
                        call_result = self._projects_locations_git_lab_configs_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-git-lab-configs-patch", Some(opt)) => {
                        call_result = self._projects_locations_git_lab_configs_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-git-lab-configs-remove-git-lab-connected-repository", Some(opt)) => {
                        call_result = self._projects_locations_git_lab_configs_remove_git_lab_connected_repository(opt, dry_run, &mut err).await;
                    },
                    ("locations-git-lab-configs-repos-list", Some(opt)) => {
                        call_result = self._projects_locations_git_lab_configs_repos_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-github-enterprise-configs-create", Some(opt)) => {
                        call_result = self._projects_locations_github_enterprise_configs_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-github-enterprise-configs-delete", Some(opt)) => {
                        call_result = self._projects_locations_github_enterprise_configs_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-github-enterprise-configs-get", Some(opt)) => {
                        call_result = self._projects_locations_github_enterprise_configs_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-github-enterprise-configs-list", Some(opt)) => {
                        call_result = self._projects_locations_github_enterprise_configs_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-github-enterprise-configs-patch", Some(opt)) => {
                        call_result = self._projects_locations_github_enterprise_configs_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-operations-cancel", Some(opt)) => {
                        call_result = self._projects_locations_operations_cancel(opt, dry_run, &mut err).await;
                    },
                    ("locations-operations-get", Some(opt)) => {
                        call_result = self._projects_locations_operations_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-triggers-create", Some(opt)) => {
                        call_result = self._projects_locations_triggers_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-triggers-delete", Some(opt)) => {
                        call_result = self._projects_locations_triggers_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-triggers-get", Some(opt)) => {
                        call_result = self._projects_locations_triggers_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-triggers-list", Some(opt)) => {
                        call_result = self._projects_locations_triggers_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-triggers-patch", Some(opt)) => {
                        call_result = self._projects_locations_triggers_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-triggers-run", Some(opt)) => {
                        call_result = self._projects_locations_triggers_run(opt, dry_run, &mut err).await;
                    },
                    ("locations-triggers-webhook", Some(opt)) => {
                        call_result = self._projects_locations_triggers_webhook(opt, dry_run, &mut err).await;
                    },
                    ("locations-worker-pools-create", Some(opt)) => {
                        call_result = self._projects_locations_worker_pools_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-worker-pools-delete", Some(opt)) => {
                        call_result = self._projects_locations_worker_pools_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-worker-pools-get", Some(opt)) => {
                        call_result = self._projects_locations_worker_pools_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-worker-pools-list", Some(opt)) => {
                        call_result = self._projects_locations_worker_pools_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-worker-pools-patch", Some(opt)) => {
                        call_result = self._projects_locations_worker_pools_patch(opt, dry_run, &mut err).await;
                    },
                    ("triggers-create", Some(opt)) => {
                        call_result = self._projects_triggers_create(opt, dry_run, &mut err).await;
                    },
                    ("triggers-delete", Some(opt)) => {
                        call_result = self._projects_triggers_delete(opt, dry_run, &mut err).await;
                    },
                    ("triggers-get", Some(opt)) => {
                        call_result = self._projects_triggers_get(opt, dry_run, &mut err).await;
                    },
                    ("triggers-list", Some(opt)) => {
                        call_result = self._projects_triggers_list(opt, dry_run, &mut err).await;
                    },
                    ("triggers-patch", Some(opt)) => {
                        call_result = self._projects_triggers_patch(opt, dry_run, &mut err).await;
                    },
                    ("triggers-run", Some(opt)) => {
                        call_result = self._projects_triggers_run(opt, dry_run, &mut err).await;
                    },
                    ("triggers-webhook", Some(opt)) => {
                        call_result = self._projects_triggers_webhook(opt, dry_run, &mut err).await;
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

            match client::application_secret_from_directory(&config_dir, "cloudbuild1-secret.json",
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
        ).persist_tokens_to_disk(format!("{}/cloudbuild1", config_dir)).build().await.unwrap();

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
        ("github-dot-com-webhook", "methods: 'receive'", vec![
            ("receive",
                    Some(r##"ReceiveGitHubDotComWebhook is called when the API receives a github.com webhook."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/github-dot-com-webhook_receive",
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
        
        ("locations", "methods: 'regional-webhook'", vec![
            ("regional-webhook",
                    Some(r##"ReceiveRegionalWebhook is called when the API receives a regional GitHub webhook."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/locations_regional-webhook",
                  vec![
                    (Some(r##"location"##),
                     None,
                     Some(r##"Required. The location where the webhook should be sent."##),
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
        
        ("methods", "methods: 'webhook'", vec![
            ("webhook",
                    Some(r##"ReceiveWebhook is called when the API receives a GitHub webhook."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/methods_webhook",
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
        
        ("operations", "methods: 'cancel' and 'get'", vec![
            ("cancel",
                    Some(r##"Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`."##),
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
                    Some(r##"Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."##),
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
        
        ("projects", "methods: 'builds-approve', 'builds-cancel', 'builds-create', 'builds-get', 'builds-list', 'builds-retry', 'github-enterprise-configs-create', 'github-enterprise-configs-delete', 'github-enterprise-configs-get', 'github-enterprise-configs-list', 'github-enterprise-configs-patch', 'locations-bitbucket-server-configs-connected-repositories-batch-create', 'locations-bitbucket-server-configs-create', 'locations-bitbucket-server-configs-delete', 'locations-bitbucket-server-configs-get', 'locations-bitbucket-server-configs-list', 'locations-bitbucket-server-configs-patch', 'locations-bitbucket-server-configs-remove-bitbucket-server-connected-repository', 'locations-bitbucket-server-configs-repos-list', 'locations-builds-approve', 'locations-builds-cancel', 'locations-builds-create', 'locations-builds-get', 'locations-builds-list', 'locations-builds-retry', 'locations-get-default-service-account', 'locations-git-lab-configs-connected-repositories-batch-create', 'locations-git-lab-configs-create', 'locations-git-lab-configs-delete', 'locations-git-lab-configs-get', 'locations-git-lab-configs-list', 'locations-git-lab-configs-patch', 'locations-git-lab-configs-remove-git-lab-connected-repository', 'locations-git-lab-configs-repos-list', 'locations-github-enterprise-configs-create', 'locations-github-enterprise-configs-delete', 'locations-github-enterprise-configs-get', 'locations-github-enterprise-configs-list', 'locations-github-enterprise-configs-patch', 'locations-operations-cancel', 'locations-operations-get', 'locations-triggers-create', 'locations-triggers-delete', 'locations-triggers-get', 'locations-triggers-list', 'locations-triggers-patch', 'locations-triggers-run', 'locations-triggers-webhook', 'locations-worker-pools-create', 'locations-worker-pools-delete', 'locations-worker-pools-get', 'locations-worker-pools-list', 'locations-worker-pools-patch', 'triggers-create', 'triggers-delete', 'triggers-get', 'triggers-list', 'triggers-patch', 'triggers-run' and 'triggers-webhook'", vec![
            ("builds-approve",
                    Some(r##"Approves or rejects a pending build. If approved, the returned LRO will be analogous to the LRO returned from a CreateBuild call. If rejected, the returned LRO will be immediately done."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_builds-approve",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Name of the target build. For example: "projects/{$project_id}/builds/{$build_id}""##),
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
                    Some(r##"Starts a build with the specified configuration. This method returns a long-running `Operation`, which includes the build ID. Pass the build ID to `GetBuild` to determine the build status (such as `SUCCESS` or `FAILURE`)."##),
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
                    Some(r##"Returns information about a previously requested build. The `Build` that is returned includes its status (such as `SUCCESS`, `FAILURE`, or `WORKING`), and timing information."##),
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
                    Some(r##"Lists previously requested builds. Previously requested builds may still be in-progress, or may have finished successfully or unsuccessfully."##),
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
                    Some(r##"Creates a new build based on the specified build. This method creates a new build using the original build request, which may or may not result in an identical build. For triggered builds: * Triggered builds resolve to a precise revision; therefore a retry of a triggered build will result in a build that uses the same revision. For non-triggered builds that specify `RepoSource`: * If the original build built from the tip of a branch, the retried build will build from the tip of that branch, which may not be the same revision as the original build. * If the original build specified a commit sha or revision ID, the retried build will use the identical source. For builds that specify `StorageSource`: * If the original build pulled source from Cloud Storage without specifying the generation of the object, the new build will use the current object, which may be different from the original build source. * If the original build pulled source from Cloud Storage and specified the generation of the object, the new build will attempt to use the same object, which may or may not be available depending on the bucket's lifecycle management settings."##),
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
            ("github-enterprise-configs-create",
                    Some(r##"Create an association between a GCP project and a GitHub Enterprise server."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_github-enterprise-configs-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Name of the parent project. For example: projects/{$project_number} or projects/{$project_id}"##),
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
            ("github-enterprise-configs-delete",
                    Some(r##"Delete an association between a GCP project and a GitHub Enterprise server."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_github-enterprise-configs-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"This field should contain the name of the enterprise config resource. For example: "projects/{$project_id}/locations/{$location_id}/githubEnterpriseConfigs/{$config_id}""##),
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
            ("github-enterprise-configs-get",
                    Some(r##"Retrieve a GitHubEnterpriseConfig."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_github-enterprise-configs-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"This field should contain the name of the enterprise config resource. For example: "projects/{$project_id}/locations/{$location_id}/githubEnterpriseConfigs/{$config_id}""##),
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
            ("github-enterprise-configs-list",
                    Some(r##"List all GitHubEnterpriseConfigs for a given project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_github-enterprise-configs-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Name of the parent project. For example: projects/{$project_number} or projects/{$project_id}"##),
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
            ("github-enterprise-configs-patch",
                    Some(r##"Update an association between a GCP project and a GitHub Enterprise server."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_github-enterprise-configs-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The full resource name for the GitHubEnterpriseConfig For example: "projects/{$project_id}/locations/{$location_id}/githubEnterpriseConfigs/{$config_id}""##),
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
            ("locations-bitbucket-server-configs-connected-repositories-batch-create",
                    Some(r##"Batch connecting Bitbucket Server repositories to Cloud Build."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-bitbucket-server-configs-connected-repositories-batch-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The name of the `BitbucketServerConfig` that added connected repository. Format: `projects/{project}/locations/{location}/bitbucketServerConfigs/{config}`"##),
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
            ("locations-bitbucket-server-configs-create",
                    Some(r##"Creates a new `BitbucketServerConfig`. This API is experimental."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-bitbucket-server-configs-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Name of the parent resource."##),
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
            ("locations-bitbucket-server-configs-delete",
                    Some(r##"Delete a `BitbucketServerConfig`. This API is experimental."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-bitbucket-server-configs-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The config resource name."##),
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
            ("locations-bitbucket-server-configs-get",
                    Some(r##"Retrieve a `BitbucketServerConfig`. This API is experimental."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-bitbucket-server-configs-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The config resource name."##),
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
            ("locations-bitbucket-server-configs-list",
                    Some(r##"List all `BitbucketServerConfigs` for a given project. This API is experimental."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-bitbucket-server-configs-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Name of the parent resource."##),
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
            ("locations-bitbucket-server-configs-patch",
                    Some(r##"Updates an existing `BitbucketServerConfig`. This API is experimental."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-bitbucket-server-configs-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The resource name for the config."##),
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
            ("locations-bitbucket-server-configs-remove-bitbucket-server-connected-repository",
                    Some(r##"Remove a Bitbucket Server repository from a given BitbucketServerConfig's connected repositories. This API is experimental."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-bitbucket-server-configs-remove-bitbucket-server-connected-repository",
                  vec![
                    (Some(r##"config"##),
                     None,
                     Some(r##"Required. The name of the `BitbucketServerConfig` to remove a connected repository. Format: `projects/{project}/locations/{location}/bitbucketServerConfigs/{config}`"##),
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
            ("locations-bitbucket-server-configs-repos-list",
                    Some(r##"List all repositories for a given `BitbucketServerConfig`. This API is experimental."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-bitbucket-server-configs-repos-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Name of the parent resource."##),
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
            ("locations-builds-approve",
                    Some(r##"Approves or rejects a pending build. If approved, the returned LRO will be analogous to the LRO returned from a CreateBuild call. If rejected, the returned LRO will be immediately done."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-builds-approve",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Name of the target build. For example: "projects/{$project_id}/builds/{$build_id}""##),
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
            ("locations-builds-cancel",
                    Some(r##"Cancels a build in progress."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-builds-cancel",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the `Build` to cancel. Format: `projects/{project}/locations/{location}/builds/{build}`"##),
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
            ("locations-builds-create",
                    Some(r##"Starts a build with the specified configuration. This method returns a long-running `Operation`, which includes the build ID. Pass the build ID to `GetBuild` to determine the build status (such as `SUCCESS` or `FAILURE`)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-builds-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The parent resource where this build will be created. Format: `projects/{project}/locations/{location}`"##),
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
            ("locations-builds-get",
                    Some(r##"Returns information about a previously requested build. The `Build` that is returned includes its status (such as `SUCCESS`, `FAILURE`, or `WORKING`), and timing information."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-builds-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the `Build` to retrieve. Format: `projects/{project}/locations/{location}/builds/{build}`"##),
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
            ("locations-builds-list",
                    Some(r##"Lists previously requested builds. Previously requested builds may still be in-progress, or may have finished successfully or unsuccessfully."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-builds-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The parent of the collection of `Builds`. Format: `projects/{project}/locations/{location}`"##),
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
            ("locations-builds-retry",
                    Some(r##"Creates a new build based on the specified build. This method creates a new build using the original build request, which may or may not result in an identical build. For triggered builds: * Triggered builds resolve to a precise revision; therefore a retry of a triggered build will result in a build that uses the same revision. For non-triggered builds that specify `RepoSource`: * If the original build built from the tip of a branch, the retried build will build from the tip of that branch, which may not be the same revision as the original build. * If the original build specified a commit sha or revision ID, the retried build will use the identical source. For builds that specify `StorageSource`: * If the original build pulled source from Cloud Storage without specifying the generation of the object, the new build will use the current object, which may be different from the original build source. * If the original build pulled source from Cloud Storage and specified the generation of the object, the new build will attempt to use the same object, which may or may not be available depending on the bucket's lifecycle management settings."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-builds-retry",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the `Build` to retry. Format: `projects/{project}/locations/{location}/builds/{build}`"##),
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
            ("locations-get-default-service-account",
                    Some(r##"Returns the `DefaultServiceAccount` used by the project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-get-default-service-account",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the `DefaultServiceAccount` to retrieve. Format: `projects/{project}/locations/{location}/defaultServiceAccount`"##),
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
            ("locations-git-lab-configs-connected-repositories-batch-create",
                    Some(r##"Batch connecting GitLab repositories to Cloud Build. This API is experimental."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-git-lab-configs-connected-repositories-batch-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The name of the `GitLabConfig` that adds connected repositories. Format: `projects/{project}/locations/{location}/gitLabConfigs/{config}`"##),
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
            ("locations-git-lab-configs-create",
                    Some(r##"Creates a new `GitLabConfig`. This API is experimental"##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-git-lab-configs-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Name of the parent resource."##),
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
            ("locations-git-lab-configs-delete",
                    Some(r##"Delete a `GitLabConfig`. This API is experimental"##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-git-lab-configs-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The config resource name."##),
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
            ("locations-git-lab-configs-get",
                    Some(r##"Retrieves a `GitLabConfig`. This API is experimental"##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-git-lab-configs-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The config resource name."##),
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
            ("locations-git-lab-configs-list",
                    Some(r##"List all `GitLabConfigs` for a given project. This API is experimental"##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-git-lab-configs-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Name of the parent resource"##),
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
            ("locations-git-lab-configs-patch",
                    Some(r##"Updates an existing `GitLabConfig`. This API is experimental"##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-git-lab-configs-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The resource name for the config."##),
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
            ("locations-git-lab-configs-remove-git-lab-connected-repository",
                    Some(r##"Remove a GitLab repository from a given GitLabConfig's connected repositories. This API is experimental."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-git-lab-configs-remove-git-lab-connected-repository",
                  vec![
                    (Some(r##"config"##),
                     None,
                     Some(r##"Required. The name of the `GitLabConfig` to remove a connected repository. Format: `projects/{project}/locations/{location}/gitLabConfigs/{config}`"##),
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
            ("locations-git-lab-configs-repos-list",
                    Some(r##"List all repositories for a given `GitLabConfig`. This API is experimental"##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-git-lab-configs-repos-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Name of the parent resource."##),
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
            ("locations-github-enterprise-configs-create",
                    Some(r##"Create an association between a GCP project and a GitHub Enterprise server."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-github-enterprise-configs-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Name of the parent project. For example: projects/{$project_number} or projects/{$project_id}"##),
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
            ("locations-github-enterprise-configs-delete",
                    Some(r##"Delete an association between a GCP project and a GitHub Enterprise server."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-github-enterprise-configs-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"This field should contain the name of the enterprise config resource. For example: "projects/{$project_id}/locations/{$location_id}/githubEnterpriseConfigs/{$config_id}""##),
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
            ("locations-github-enterprise-configs-get",
                    Some(r##"Retrieve a GitHubEnterpriseConfig."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-github-enterprise-configs-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"This field should contain the name of the enterprise config resource. For example: "projects/{$project_id}/locations/{$location_id}/githubEnterpriseConfigs/{$config_id}""##),
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
            ("locations-github-enterprise-configs-list",
                    Some(r##"List all GitHubEnterpriseConfigs for a given project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-github-enterprise-configs-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Name of the parent project. For example: projects/{$project_number} or projects/{$project_id}"##),
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
            ("locations-github-enterprise-configs-patch",
                    Some(r##"Update an association between a GCP project and a GitHub Enterprise server."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-github-enterprise-configs-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The full resource name for the GitHubEnterpriseConfig For example: "projects/{$project_id}/locations/{$location_id}/githubEnterpriseConfigs/{$config_id}""##),
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
                    Some(r##"Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`."##),
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
                    Some(r##"Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."##),
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
            ("locations-triggers-create",
                    Some(r##"Creates a new `BuildTrigger`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-triggers-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The parent resource where this trigger will be created. Format: `projects/{project}/locations/{location}`"##),
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
            ("locations-triggers-delete",
                    Some(r##"Deletes a `BuildTrigger` by its project ID and trigger ID."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-triggers-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the `Trigger` to delete. Format: `projects/{project}/locations/{location}/triggers/{trigger}`"##),
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
            ("locations-triggers-get",
                    Some(r##"Returns information about a `BuildTrigger`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-triggers-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the `Trigger` to retrieve. Format: `projects/{project}/locations/{location}/triggers/{trigger}`"##),
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
            ("locations-triggers-list",
                    Some(r##"Lists existing `BuildTrigger`s."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-triggers-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The parent of the collection of `Triggers`. Format: `projects/{project}/locations/{location}`"##),
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
            ("locations-triggers-patch",
                    Some(r##"Updates a `BuildTrigger` by its project ID and trigger ID."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-triggers-patch",
                  vec![
                    (Some(r##"resource-name"##),
                     None,
                     Some(r##"The `Trigger` name with format: `projects/{project}/locations/{location}/triggers/{trigger}`, where {trigger} is a unique identifier generated by the service."##),
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
            ("locations-triggers-run",
                    Some(r##"Runs a `BuildTrigger` at a particular source revision. To run a regional or global trigger, use the POST request that includes the location endpoint in the path (ex. v1/projects/{projectId}/locations/{region}/triggers/{triggerId}:run). The POST request that does not include the location endpoint in the path can only be used when running global triggers."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-triggers-run",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the `Trigger` to run. Format: `projects/{project}/locations/{location}/triggers/{trigger}`"##),
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
            ("locations-triggers-webhook",
                    Some(r##"ReceiveTriggerWebhook [Experimental] is called when the API receives a webhook request targeted at a specific trigger."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-triggers-webhook",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the `ReceiveTriggerWebhook` to retrieve. Format: `projects/{project}/locations/{location}/triggers/{trigger}`"##),
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
            ("locations-worker-pools-create",
                    Some(r##"Creates a `WorkerPool`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-worker-pools-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource where this worker pool will be created. Format: `projects/{project}/locations/{location}`."##),
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
            ("locations-worker-pools-delete",
                    Some(r##"Deletes a `WorkerPool`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-worker-pools-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the `WorkerPool` to delete. Format: `projects/{project}/locations/{location}/workerPools/{workerPool}`."##),
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
            ("locations-worker-pools-get",
                    Some(r##"Returns details of a `WorkerPool`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-worker-pools-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the `WorkerPool` to retrieve. Format: `projects/{project}/locations/{location}/workerPools/{workerPool}`."##),
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
            ("locations-worker-pools-list",
                    Some(r##"Lists `WorkerPool`s."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-worker-pools-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent of the collection of `WorkerPools`. Format: `projects/{project}/locations/{location}`."##),
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
            ("locations-worker-pools-patch",
                    Some(r##"Updates a `WorkerPool`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_locations-worker-pools-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Output only. The resource name of the `WorkerPool`, with format `projects/{project}/locations/{location}/workerPools/{worker_pool}`. The value of `{worker_pool}` is provided by `worker_pool_id` in `CreateWorkerPool` request and the value of `{location}` is determined by the endpoint accessed."##),
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
            ("triggers-create",
                    Some(r##"Creates a new `BuildTrigger`."##),
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
                    Some(r##"Deletes a `BuildTrigger` by its project ID and trigger ID."##),
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
                    Some(r##"Returns information about a `BuildTrigger`."##),
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
                    Some(r##"Lists existing `BuildTrigger`s."##),
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
                    Some(r##"Updates a `BuildTrigger` by its project ID and trigger ID."##),
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
                    Some(r##"Runs a `BuildTrigger` at a particular source revision. To run a regional or global trigger, use the POST request that includes the location endpoint in the path (ex. v1/projects/{projectId}/locations/{region}/triggers/{triggerId}:run). The POST request that does not include the location endpoint in the path can only be used when running global triggers."##),
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
            ("triggers-webhook",
                    Some(r##"ReceiveTriggerWebhook [Experimental] is called when the API receives a webhook request targeted at a specific trigger."##),
                    "Details at http://byron.github.io/google-apis-rs/google_cloudbuild1_cli/projects_triggers-webhook",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Project in which the specified trigger lives"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"trigger"##),
                     None,
                     Some(r##"Name of the trigger to run the payload against"##),
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
           .version("5.0.5+20240618")
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
