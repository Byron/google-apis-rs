// DO NOT EDIT !
// This file was generated automatically from 'src/mako/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;
extern crate yup_oauth2 as oauth2;
extern crate yup_hyper_mock as mock;
extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate mime;
extern crate strsim;
extern crate google_bigquery2 as api;

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
    hub: api::Bigquery<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n> Engine<'n> {
    fn _datasets_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.datasets().delete(opt.value_of("project-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "delete-contents" => {
                    call = call.delete_contents(arg_from_str(value.unwrap_or("false"), err, "delete-contents", "boolean"));
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _datasets_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.datasets().get(opt.value_of("project-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""));
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _datasets_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dataset-reference.project-id" => Some(("datasetReference.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dataset-reference.dataset-id" => Some(("datasetReference.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "creation-time" => Some(("creationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-table-expiration-ms" => Some(("defaultTableExpirationMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location" => Some(("location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "friendly-name" => Some(("friendlyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modified-time" => Some(("lastModifiedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["creation-time", "dataset-id", "dataset-reference", "default-table-expiration-ms", "description", "etag", "friendly-name", "id", "kind", "labels", "last-modified-time", "location", "project-id", "self-link"]);
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _datasets_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.datasets().list(opt.value_of("project-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                "all" => {
                    call = call.all(arg_from_str(value.unwrap_or("false"), err, "all", "boolean"));
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
                                                                           v.extend(["filter", "page-token", "all", "max-results"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _datasets_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dataset-reference.project-id" => Some(("datasetReference.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dataset-reference.dataset-id" => Some(("datasetReference.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "creation-time" => Some(("creationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-table-expiration-ms" => Some(("defaultTableExpirationMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location" => Some(("location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "friendly-name" => Some(("friendlyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modified-time" => Some(("lastModifiedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["creation-time", "dataset-id", "dataset-reference", "default-table-expiration-ms", "description", "etag", "friendly-name", "id", "kind", "labels", "last-modified-time", "location", "project-id", "self-link"]);
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _datasets_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dataset-reference.project-id" => Some(("datasetReference.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dataset-reference.dataset-id" => Some(("datasetReference.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "creation-time" => Some(("creationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-table-expiration-ms" => Some(("defaultTableExpirationMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location" => Some(("location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "friendly-name" => Some(("friendlyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modified-time" => Some(("lastModifiedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["creation-time", "dataset-id", "dataset-reference", "default-table-expiration-ms", "description", "etag", "friendly-name", "id", "kind", "labels", "last-modified-time", "location", "project-id", "self-link"]);
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _jobs_cancel(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.jobs().cancel(opt.value_of("project-id").unwrap_or(""), opt.value_of("job-id").unwrap_or(""));
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _jobs_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.jobs().get(opt.value_of("project-id").unwrap_or(""), opt.value_of("job-id").unwrap_or(""));
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _jobs_get_query_results(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.jobs().get_query_results(opt.value_of("project-id").unwrap_or(""), opt.value_of("job-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "timeout-ms" => {
                    call = call.timeout_ms(arg_from_str(value.unwrap_or("-0"), err, "timeout-ms", "integer"));
                },
                "start-index" => {
                    call = call.start_index(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
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
                                                                           v.extend(["timeout-ms", "page-token", "start-index", "max-results"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _jobs_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "status.error-result.debug-info" => Some(("status.errorResult.debugInfo", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.error-result.message" => Some(("status.errorResult.message", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.error-result.reason" => Some(("status.errorResult.reason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status.error-result.location" => Some(("status.errorResult.location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.load.output-rows" => Some(("statistics.load.outputRows", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.load.input-files" => Some(("statistics.load.inputFiles", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.load.input-file-bytes" => Some(("statistics.load.inputFileBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.load.output-bytes" => Some(("statistics.load.outputBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.creation-time" => Some(("statistics.creationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.total-bytes-processed" => Some(("statistics.totalBytesProcessed", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.start-time" => Some(("statistics.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.cache-hit" => Some(("statistics.query.cacheHit", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "statistics.query.billing-tier" => Some(("statistics.query.billingTier", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "statistics.query.total-bytes-billed" => Some(("statistics.query.totalBytesBilled", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.num-dml-affected-rows" => Some(("statistics.query.numDmlAffectedRows", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.query.total-bytes-processed" => Some(("statistics.query.totalBytesProcessed", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.end-time" => Some(("statistics.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "statistics.extract.destination-uri-file-counts" => Some(("statistics.extract.destinationUriFileCounts", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Vec })),
                    "job-reference.project-id" => Some(("jobReference.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-reference.job-id" => Some(("jobReference.jobId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-email" => Some(("user_email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.load.skip-leading-rows" => Some(("configuration.load.skipLeadingRows", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "configuration.load.encoding" => Some(("configuration.load.encoding", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.load.ignore-unknown-values" => Some(("configuration.load.ignoreUnknownValues", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "configuration.load.quote" => Some(("configuration.load.quote", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.load.autodetect" => Some(("configuration.load.autodetect", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "configuration.load.destination-table.project-id" => Some(("configuration.load.destinationTable.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.load.destination-table.table-id" => Some(("configuration.load.destinationTable.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.load.destination-table.dataset-id" => Some(("configuration.load.destinationTable.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.load.max-bad-records" => Some(("configuration.load.maxBadRecords", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "configuration.load.schema-update-options" => Some(("configuration.load.schemaUpdateOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "configuration.load.allow-jagged-rows" => Some(("configuration.load.allowJaggedRows", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "configuration.load.write-disposition" => Some(("configuration.load.writeDisposition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.load.source-uris" => Some(("configuration.load.sourceUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "configuration.load.field-delimiter" => Some(("configuration.load.fieldDelimiter", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.load.create-disposition" => Some(("configuration.load.createDisposition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.load.schema-inline-format" => Some(("configuration.load.schemaInlineFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.load.schema-inline" => Some(("configuration.load.schemaInline", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.load.allow-quoted-newlines" => Some(("configuration.load.allowQuotedNewlines", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "configuration.load.projection-fields" => Some(("configuration.load.projectionFields", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "configuration.load.source-format" => Some(("configuration.load.sourceFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.extract.destination-uri" => Some(("configuration.extract.destinationUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.extract.compression" => Some(("configuration.extract.compression", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.extract.field-delimiter" => Some(("configuration.extract.fieldDelimiter", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.extract.destination-format" => Some(("configuration.extract.destinationFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.extract.print-header" => Some(("configuration.extract.printHeader", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "configuration.extract.destination-uris" => Some(("configuration.extract.destinationUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "configuration.extract.source-table.project-id" => Some(("configuration.extract.sourceTable.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.extract.source-table.table-id" => Some(("configuration.extract.sourceTable.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.extract.source-table.dataset-id" => Some(("configuration.extract.sourceTable.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.copy.create-disposition" => Some(("configuration.copy.createDisposition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.copy.write-disposition" => Some(("configuration.copy.writeDisposition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.copy.destination-table.project-id" => Some(("configuration.copy.destinationTable.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.copy.destination-table.table-id" => Some(("configuration.copy.destinationTable.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.copy.destination-table.dataset-id" => Some(("configuration.copy.destinationTable.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.copy.source-table.project-id" => Some(("configuration.copy.sourceTable.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.copy.source-table.table-id" => Some(("configuration.copy.sourceTable.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.copy.source-table.dataset-id" => Some(("configuration.copy.sourceTable.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.dry-run" => Some(("configuration.dryRun", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "configuration.query.flatten-results" => Some(("configuration.query.flattenResults", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "configuration.query.maximum-billing-tier" => Some(("configuration.query.maximumBillingTier", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "configuration.query.use-query-cache" => Some(("configuration.query.useQueryCache", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "configuration.query.default-dataset.project-id" => Some(("configuration.query.defaultDataset.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.query.default-dataset.dataset-id" => Some(("configuration.query.defaultDataset.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.query.use-legacy-sql" => Some(("configuration.query.useLegacySql", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "configuration.query.destination-table.project-id" => Some(("configuration.query.destinationTable.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.query.destination-table.table-id" => Some(("configuration.query.destinationTable.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.query.destination-table.dataset-id" => Some(("configuration.query.destinationTable.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.query.schema-update-options" => Some(("configuration.query.schemaUpdateOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "configuration.query.priority" => Some(("configuration.query.priority", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.query.write-disposition" => Some(("configuration.query.writeDisposition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.query.allow-large-results" => Some(("configuration.query.allowLargeResults", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "configuration.query.create-disposition" => Some(("configuration.query.createDisposition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.query.query" => Some(("configuration.query.query", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.query.preserve-nulls" => Some(("configuration.query.preserveNulls", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "configuration.query.maximum-bytes-billed" => Some(("configuration.query.maximumBytesBilled", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "configuration.query.parameter-mode" => Some(("configuration.query.parameterMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["allow-jagged-rows", "allow-large-results", "allow-quoted-newlines", "autodetect", "billing-tier", "cache-hit", "compression", "configuration", "copy", "create-disposition", "creation-time", "dataset-id", "debug-info", "default-dataset", "destination-format", "destination-table", "destination-uri", "destination-uri-file-counts", "destination-uris", "dry-run", "encoding", "end-time", "error-result", "etag", "extract", "field-delimiter", "flatten-results", "id", "ignore-unknown-values", "input-file-bytes", "input-files", "job-id", "job-reference", "kind", "load", "location", "max-bad-records", "maximum-billing-tier", "maximum-bytes-billed", "message", "num-dml-affected-rows", "output-bytes", "output-rows", "parameter-mode", "preserve-nulls", "print-header", "priority", "project-id", "projection-fields", "query", "quote", "reason", "schema-inline", "schema-inline-format", "schema-update-options", "self-link", "skip-leading-rows", "source-format", "source-table", "source-uris", "start-time", "state", "statistics", "status", "table-id", "total-bytes-billed", "total-bytes-processed", "use-legacy-sql", "use-query-cache", "user-email", "write-disposition"]);
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
        let protocol = calltype_from_str(vals[0], ["simple", "resumable"].iter().map(|&v| v.to_string()).collect(), err);
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
                CallType::Upload(UploadProtocol::Simple) => call.upload(input_file.unwrap(), mime_type.unwrap()),
                CallType::Upload(UploadProtocol::Resumable) => call.upload_resumable(input_file.unwrap(), mime_type.unwrap()),
                CallType::Standard => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _jobs_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "all-users" => {
                    call = call.all_users(arg_from_str(value.unwrap_or("false"), err, "all-users", "boolean"));
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
                                                                           v.extend(["page-token", "all-users", "max-results", "projection", "state-filter"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _jobs_query(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "timeout-ms" => Some(("timeoutMs", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dry-run" => Some(("dryRun", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "parameter-mode" => Some(("parameterMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "use-query-cache" => Some(("useQueryCache", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "default-dataset.project-id" => Some(("defaultDataset.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-dataset.dataset-id" => Some(("defaultDataset.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "use-legacy-sql" => Some(("useLegacySql", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "max-results" => Some(("maxResults", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "query" => Some(("query", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preserve-nulls" => Some(("preserveNulls", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["dataset-id", "default-dataset", "dry-run", "kind", "max-results", "parameter-mode", "preserve-nulls", "project-id", "query", "timeout-ms", "use-legacy-sql", "use-query-cache"]);
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
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
                                                                           v.extend(["page-token", "max-results"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _tabledata_insert_all(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "template-suffix" => Some(("templateSuffix", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "skip-invalid-rows" => Some(("skipInvalidRows", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["ignore-unknown-values", "kind", "skip-invalid-rows", "template-suffix"]);
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _tabledata_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.tabledata().list(opt.value_of("project-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""), opt.value_of("table-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
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
                                                                           v.extend(["page-token", "start-index", "max-results"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _tables_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _tables_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.tables().get(opt.value_of("project-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""), opt.value_of("table-id").unwrap_or(""));
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _tables_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "expiration-time" => Some(("expirationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "streaming-buffer.estimated-bytes" => Some(("streamingBuffer.estimatedBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "streaming-buffer.estimated-rows" => Some(("streamingBuffer.estimatedRows", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "streaming-buffer.oldest-entry-time" => Some(("streamingBuffer.oldestEntryTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creation-time" => Some(("creationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-reference.project-id" => Some(("tableReference.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-reference.table-id" => Some(("tableReference.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-reference.dataset-id" => Some(("tableReference.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-rows" => Some(("numRows", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-bytes" => Some(("numBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.compression" => Some(("externalDataConfiguration.compression", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.encoding" => Some(("externalDataConfiguration.csvOptions.encoding", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.quote" => Some(("externalDataConfiguration.csvOptions.quote", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.allow-jagged-rows" => Some(("externalDataConfiguration.csvOptions.allowJaggedRows", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.skip-leading-rows" => Some(("externalDataConfiguration.csvOptions.skipLeadingRows", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.field-delimiter" => Some(("externalDataConfiguration.csvOptions.fieldDelimiter", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.allow-quoted-newlines" => Some(("externalDataConfiguration.csvOptions.allowQuotedNewlines", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.autodetect" => Some(("externalDataConfiguration.autodetect", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.max-bad-records" => Some(("externalDataConfiguration.maxBadRecords", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "external-data-configuration.ignore-unknown-values" => Some(("externalDataConfiguration.ignoreUnknownValues", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.source-uris" => Some(("externalDataConfiguration.sourceUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "external-data-configuration.bigtable-options.read-rowkey-as-string" => Some(("externalDataConfiguration.bigtableOptions.readRowkeyAsString", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.bigtable-options.ignore-unspecified-column-families" => Some(("externalDataConfiguration.bigtableOptions.ignoreUnspecifiedColumnFamilies", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.source-format" => Some(("externalDataConfiguration.sourceFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.google-sheets-options.skip-leading-rows" => Some(("externalDataConfiguration.googleSheetsOptions.skipLeadingRows", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location" => Some(("location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "friendly-name" => Some(("friendlyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-partitioning.type" => Some(("timePartitioning.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-partitioning.expiration-ms" => Some(("timePartitioning.expirationMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modified-time" => Some(("lastModifiedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "view.query" => Some(("view.query", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "view.use-legacy-sql" => Some(("view.useLegacySql", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-long-term-bytes" => Some(("numLongTermBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["allow-jagged-rows", "allow-quoted-newlines", "autodetect", "bigtable-options", "compression", "creation-time", "csv-options", "dataset-id", "description", "encoding", "estimated-bytes", "estimated-rows", "etag", "expiration-ms", "expiration-time", "external-data-configuration", "field-delimiter", "friendly-name", "google-sheets-options", "id", "ignore-unknown-values", "ignore-unspecified-column-families", "kind", "last-modified-time", "location", "max-bad-records", "num-bytes", "num-long-term-bytes", "num-rows", "oldest-entry-time", "project-id", "query", "quote", "read-rowkey-as-string", "self-link", "skip-leading-rows", "source-format", "source-uris", "streaming-buffer", "table-id", "table-reference", "time-partitioning", "type", "use-legacy-sql", "view"]);
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
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _tables_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.tables().list(opt.value_of("project-id").unwrap_or(""), opt.value_of("dataset-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
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
                                                                           v.extend(["page-token", "max-results"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _tables_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "expiration-time" => Some(("expirationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "streaming-buffer.estimated-bytes" => Some(("streamingBuffer.estimatedBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "streaming-buffer.estimated-rows" => Some(("streamingBuffer.estimatedRows", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "streaming-buffer.oldest-entry-time" => Some(("streamingBuffer.oldestEntryTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creation-time" => Some(("creationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-reference.project-id" => Some(("tableReference.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-reference.table-id" => Some(("tableReference.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-reference.dataset-id" => Some(("tableReference.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-rows" => Some(("numRows", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-bytes" => Some(("numBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.compression" => Some(("externalDataConfiguration.compression", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.encoding" => Some(("externalDataConfiguration.csvOptions.encoding", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.quote" => Some(("externalDataConfiguration.csvOptions.quote", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.allow-jagged-rows" => Some(("externalDataConfiguration.csvOptions.allowJaggedRows", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.skip-leading-rows" => Some(("externalDataConfiguration.csvOptions.skipLeadingRows", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.field-delimiter" => Some(("externalDataConfiguration.csvOptions.fieldDelimiter", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.allow-quoted-newlines" => Some(("externalDataConfiguration.csvOptions.allowQuotedNewlines", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.autodetect" => Some(("externalDataConfiguration.autodetect", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.max-bad-records" => Some(("externalDataConfiguration.maxBadRecords", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "external-data-configuration.ignore-unknown-values" => Some(("externalDataConfiguration.ignoreUnknownValues", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.source-uris" => Some(("externalDataConfiguration.sourceUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "external-data-configuration.bigtable-options.read-rowkey-as-string" => Some(("externalDataConfiguration.bigtableOptions.readRowkeyAsString", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.bigtable-options.ignore-unspecified-column-families" => Some(("externalDataConfiguration.bigtableOptions.ignoreUnspecifiedColumnFamilies", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.source-format" => Some(("externalDataConfiguration.sourceFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.google-sheets-options.skip-leading-rows" => Some(("externalDataConfiguration.googleSheetsOptions.skipLeadingRows", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location" => Some(("location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "friendly-name" => Some(("friendlyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-partitioning.type" => Some(("timePartitioning.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-partitioning.expiration-ms" => Some(("timePartitioning.expirationMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modified-time" => Some(("lastModifiedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "view.query" => Some(("view.query", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "view.use-legacy-sql" => Some(("view.useLegacySql", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-long-term-bytes" => Some(("numLongTermBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["allow-jagged-rows", "allow-quoted-newlines", "autodetect", "bigtable-options", "compression", "creation-time", "csv-options", "dataset-id", "description", "encoding", "estimated-bytes", "estimated-rows", "etag", "expiration-ms", "expiration-time", "external-data-configuration", "field-delimiter", "friendly-name", "google-sheets-options", "id", "ignore-unknown-values", "ignore-unspecified-column-families", "kind", "last-modified-time", "location", "max-bad-records", "num-bytes", "num-long-term-bytes", "num-rows", "oldest-entry-time", "project-id", "query", "quote", "read-rowkey-as-string", "self-link", "skip-leading-rows", "source-format", "source-uris", "streaming-buffer", "table-id", "table-reference", "time-partitioning", "type", "use-legacy-sql", "view"]);
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
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _tables_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "expiration-time" => Some(("expirationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "streaming-buffer.estimated-bytes" => Some(("streamingBuffer.estimatedBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "streaming-buffer.estimated-rows" => Some(("streamingBuffer.estimatedRows", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "streaming-buffer.oldest-entry-time" => Some(("streamingBuffer.oldestEntryTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creation-time" => Some(("creationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-reference.project-id" => Some(("tableReference.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-reference.table-id" => Some(("tableReference.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "table-reference.dataset-id" => Some(("tableReference.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-rows" => Some(("numRows", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-bytes" => Some(("numBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.compression" => Some(("externalDataConfiguration.compression", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.encoding" => Some(("externalDataConfiguration.csvOptions.encoding", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.quote" => Some(("externalDataConfiguration.csvOptions.quote", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.allow-jagged-rows" => Some(("externalDataConfiguration.csvOptions.allowJaggedRows", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.skip-leading-rows" => Some(("externalDataConfiguration.csvOptions.skipLeadingRows", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.field-delimiter" => Some(("externalDataConfiguration.csvOptions.fieldDelimiter", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.csv-options.allow-quoted-newlines" => Some(("externalDataConfiguration.csvOptions.allowQuotedNewlines", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.autodetect" => Some(("externalDataConfiguration.autodetect", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.max-bad-records" => Some(("externalDataConfiguration.maxBadRecords", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "external-data-configuration.ignore-unknown-values" => Some(("externalDataConfiguration.ignoreUnknownValues", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.source-uris" => Some(("externalDataConfiguration.sourceUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "external-data-configuration.bigtable-options.read-rowkey-as-string" => Some(("externalDataConfiguration.bigtableOptions.readRowkeyAsString", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.bigtable-options.ignore-unspecified-column-families" => Some(("externalDataConfiguration.bigtableOptions.ignoreUnspecifiedColumnFamilies", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "external-data-configuration.source-format" => Some(("externalDataConfiguration.sourceFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-data-configuration.google-sheets-options.skip-leading-rows" => Some(("externalDataConfiguration.googleSheetsOptions.skipLeadingRows", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location" => Some(("location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "friendly-name" => Some(("friendlyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-partitioning.type" => Some(("timePartitioning.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-partitioning.expiration-ms" => Some(("timePartitioning.expirationMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-modified-time" => Some(("lastModifiedTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "view.query" => Some(("view.query", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "view.use-legacy-sql" => Some(("view.useLegacySql", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-long-term-bytes" => Some(("numLongTermBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["allow-jagged-rows", "allow-quoted-newlines", "autodetect", "bigtable-options", "compression", "creation-time", "csv-options", "dataset-id", "description", "encoding", "estimated-bytes", "estimated-rows", "etag", "expiration-ms", "expiration-time", "external-data-configuration", "field-delimiter", "friendly-name", "google-sheets-options", "id", "ignore-unknown-values", "ignore-unspecified-column-families", "kind", "last-modified-time", "location", "max-bad-records", "num-bytes", "num-long-term-bytes", "num-rows", "oldest-entry-time", "project-id", "query", "quote", "read-rowkey-as-string", "self-link", "skip-leading-rows", "source-format", "source-uris", "streaming-buffer", "table-id", "table-reference", "time-partitioning", "type", "use-legacy-sql", "view"]);
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
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
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
                    let mut value = json::value::to_value(&output_schema);
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
            ("datasets", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._datasets_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._datasets_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._datasets_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._datasets_list(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._datasets_patch(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._datasets_update(opt, dry_run, &mut err);
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
                        call_result = self._jobs_cancel(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._jobs_get(opt, dry_run, &mut err);
                    },
                    ("get-query-results", Some(opt)) => {
                        call_result = self._jobs_get_query_results(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._jobs_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._jobs_list(opt, dry_run, &mut err);
                    },
                    ("query", Some(opt)) => {
                        call_result = self._jobs_query(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("jobs".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("projects", Some(opt)) => {
                match opt.subcommand() {
                    ("list", Some(opt)) => {
                        call_result = self._projects_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("projects".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("tabledata", Some(opt)) => {
                match opt.subcommand() {
                    ("insert-all", Some(opt)) => {
                        call_result = self._tabledata_insert_all(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._tabledata_list(opt, dry_run, &mut err);
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
                        call_result = self._tables_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._tables_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._tables_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._tables_list(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._tables_patch(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._tables_update(opt, dry_run, &mut err);
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
    fn new(opt: ArgMatches<'n>) -> Result<Engine<'n>, InvalidOptionsError> {
        let (config_dir, secret) = {
            let config_dir = match cmn::assure_config_dir_exists(opt.value_of("folder").unwrap_or("~/.google-service-cli")) {
                Err(e) => return Err(InvalidOptionsError::single(e, 3)),
                Ok(p) => p,
            };

            match cmn::application_secret_from_directory(&config_dir, "bigquery2-secret.json",
                                                         "{\"installed\":{\"auth_uri\":\"https://accounts.google.com/o/oauth2/auth\",\"client_secret\":\"hCsslbCUyfehWMmbkG8vTYxG\",\"token_uri\":\"https://accounts.google.com/o/oauth2/token\",\"client_email\":\"\",\"redirect_uris\":[\"urn:ietf:wg:oauth:2.0:oob\",\"oob\"],\"client_x509_cert_url\":\"\",\"client_id\":\"620010449518-9ngf7o4dhs0dka470npqvor6dc5lqb9b.apps.googleusercontent.com\",\"auth_provider_x509_cert_url\":\"https://www.googleapis.com/oauth2/v1/certs\"}}") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let auth = Authenticator::new(  &secret, DefaultAuthenticatorDelegate,
                                        if opt.is_present("debug-auth") {
                                            hyper::Client::with_connector(mock::TeeConnector {
                                                    connector: hyper::net::HttpsConnector::<hyper::net::Openssl>::default()
                                                })
                                        } else {
                                            hyper::Client::new()
                                        },
                                        JsonTokenStorage {
                                          program_name: "bigquery2",
                                          db_dir: config_dir.clone(),
                                        }, Some(FlowType::InstalledRedirect(54324)));

        let client =
            if opt.is_present("debug") {
                hyper::Client::with_connector(mock::TeeConnector {
                        connector: hyper::net::HttpsConnector::<hyper::net::Openssl>::default()
                    })
            } else {
                hyper::Client::new()
            };
        let engine = Engine {
            opt: opt,
            hub: api::Bigquery::new(client, auth),
            gp: vec!["alt", "fields", "key", "oauth-token", "pretty-print", "quota-user", "user-ip"],
            gpm: vec![
                    ("oauth-token", "oauth_token"),
                    ("pretty-print", "prettyPrint"),
                    ("quota-user", "quotaUser"),
                    ("user-ip", "userIp"),
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
    let upload_value_names = ["mode", "file"];
    let arg_data = [
        ("datasets", "methods: 'delete', 'get', 'insert', 'list', 'patch' and 'update'", vec![
            ("delete",
                    Some(r##"Deletes the dataset specified by the datasetId value. Before you can delete a dataset, you must delete all its tables, either manually or by specifying deleteContents. Immediately after deletion, you can create another dataset with the same name."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/datasets_delete",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Project ID of the dataset being deleted"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Dataset ID of dataset being deleted"##),
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
                     Some(r##"Project ID of the requested dataset"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Dataset ID of the requested dataset"##),
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
                     Some(r##"Project ID of the new dataset"##),
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
                    Some(r##"Lists all datasets in the specified project to which you have been granted the READER dataset role."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/datasets_list",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Project ID of the datasets to be listed"##),
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
                    Some(r##"Updates information in an existing dataset. The update method replaces the entire dataset resource, whereas the patch method only replaces fields that are provided in the submitted dataset resource. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/datasets_patch",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Project ID of the dataset being updated"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Dataset ID of the dataset being updated"##),
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
                     Some(r##"Project ID of the dataset being updated"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Dataset ID of the dataset being updated"##),
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
        
        ("jobs", "methods: 'cancel', 'get', 'get-query-results', 'insert', 'list' and 'query'", vec![
            ("cancel",
                    Some(r##"Requests that a job be cancelled. This call will return immediately, and the client will need to poll for the job status to see if the cancel completed successfully. Cancelled jobs may still incur costs."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/jobs_cancel",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"[Required] Project ID of the job to cancel"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"job-id"##),
                     None,
                     Some(r##"[Required] Job ID of the job to cancel"##),
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
                    Some(r##"Returns information about a specific job. Job information is available for a six month period after creation. Requires that you're the person who ran the job, or have the Is Owner project role."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/jobs_get",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"[Required] Project ID of the requested job"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"job-id"##),
                     None,
                     Some(r##"[Required] Job ID of the requested job"##),
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
                    Some(r##"Retrieves the results of a query job."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/jobs_get-query-results",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"[Required] Project ID of the query job"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"job-id"##),
                     None,
                     Some(r##"[Required] Job ID of the query job"##),
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
                    Some(r##"Starts a new asynchronous job. Requires the Can View project role."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/jobs_insert",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Project ID of the project that will be billed for the job"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"mode"##),
                     Some(r##"u"##),
                     Some(r##"Specify the upload protocol (simple|resumable) and the file to upload"##),
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
                     Some(r##"Project ID of the jobs to list"##),
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
                     Some(r##"Project ID of the project billed for the query"##),
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
        
        ("projects", "methods: 'list'", vec![
            ("list",
                    Some(r##"Lists all projects to which you have been granted any project role."##),
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
        
        ("tabledata", "methods: 'insert-all' and 'list'", vec![
            ("insert-all",
                    Some(r##"Streams data into BigQuery one record at a time without needing to run a load job. Requires the WRITER dataset role."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/tabledata_insert-all",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Project ID of the destination table."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Dataset ID of the destination table."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Table ID of the destination table."##),
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
                    Some(r##"Retrieves table data from a specified set of rows. Requires the READER dataset role."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/tabledata_list",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Project ID of the table to read"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Dataset ID of the table to read"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Table ID of the table to read"##),
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
        
        ("tables", "methods: 'delete', 'get', 'insert', 'list', 'patch' and 'update'", vec![
            ("delete",
                    Some(r##"Deletes the table specified by tableId from the dataset. If the table contains data, all the data will be deleted."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/tables_delete",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Project ID of the table to delete"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Dataset ID of the table to delete"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Table ID of the table to delete"##),
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
                     Some(r##"Project ID of the requested table"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Dataset ID of the requested table"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Table ID of the requested table"##),
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
                    Some(r##"Creates a new, empty table in the dataset."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/tables_insert",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Project ID of the new table"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Dataset ID of the new table"##),
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
                     Some(r##"Project ID of the tables to list"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Dataset ID of the tables to list"##),
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
                    Some(r##"Updates information in an existing table. The update method replaces the entire table resource, whereas the patch method only replaces fields that are provided in the submitted table resource. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/tables_patch",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Project ID of the table to update"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Dataset ID of the table to update"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Table ID of the table to update"##),
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
                    Some(r##"Updates information in an existing table. The update method replaces the entire table resource, whereas the patch method only replaces fields that are provided in the submitted table resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_bigquery2_cli/tables_update",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Project ID of the table to update"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Dataset ID of the table to update"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Table ID of the table to update"##),
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
           .version("1.0.0+20160903")
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