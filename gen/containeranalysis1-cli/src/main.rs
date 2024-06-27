// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_containeranalysis1::{api, Error, oauth2, client::chrono, FieldMask};


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
    hub: api::ContainerAnalysis<S>,
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
    async fn _projects_locations_notes_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_notes_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_notes_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_notes_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_notes_occurrences_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_notes_occurrences_list(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_occurrences_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_occurrences_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_occurrences_get_notes(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_occurrences_get_notes(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_occurrences_get_vulnerability_summary(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_occurrences_get_vulnerability_summary(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
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
                                                                           v.extend(["filter"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _projects_locations_occurrences_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_occurrences_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_resources_export_sbom(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::ExportSBOMRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_resources_export_sbom(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_notes_batch_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::BatchCreateNotesRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().notes_batch_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_notes_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "attestation.hint.human-readable-name" => Some(("attestation.hint.humanReadableName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.builder-version" => Some(("build.builderVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "compliance.cis-benchmark.profile-level" => Some(("compliance.cisBenchmark.profileLevel", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "compliance.cis-benchmark.severity" => Some(("compliance.cisBenchmark.severity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "compliance.description" => Some(("compliance.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "compliance.impact" => Some(("compliance.impact", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "compliance.rationale" => Some(("compliance.rationale", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "compliance.remediation" => Some(("compliance.remediation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "compliance.scan-instructions" => Some(("compliance.scanInstructions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "compliance.title" => Some(("compliance.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment.resource-uri" => Some(("deployment.resourceUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "discovery.analysis-kind" => Some(("discovery.analysisKind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.hint.human-readable-name" => Some(("dsseAttestation.hint.humanReadableName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "expiration-time" => Some(("expirationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image.fingerprint.v1-name" => Some(("image.fingerprint.v1Name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image.fingerprint.v2-blob" => Some(("image.fingerprint.v2Blob", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "image.fingerprint.v2-name" => Some(("image.fingerprint.v2Name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image.resource-url" => Some(("image.resourceUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "long-description" => Some(("longDescription", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.architecture" => Some(("package.architecture", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.cpe-uri" => Some(("package.cpeUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.description" => Some(("package.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.license.comments" => Some(("package.license.comments", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.license.expression" => Some(("package.license.expression", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.maintainer" => Some(("package.maintainer", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.name" => Some(("package.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.package-type" => Some(("package.packageType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.url" => Some(("package.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.version.epoch" => Some(("package.version.epoch", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "package.version.full-name" => Some(("package.version.fullName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.version.inclusive" => Some(("package.version.inclusive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "package.version.kind" => Some(("package.version.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.version.name" => Some(("package.version.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.version.revision" => Some(("package.version.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "related-note-names" => Some(("relatedNoteNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "sbom-reference.format" => Some(("sbomReference.format", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sbom-reference.version" => Some(("sbomReference.version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "short-description" => Some(("shortDescription", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.package" => Some(("upgrade.package", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.version.epoch" => Some(("upgrade.version.epoch", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "upgrade.version.full-name" => Some(("upgrade.version.fullName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.version.inclusive" => Some(("upgrade.version.inclusive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "upgrade.version.kind" => Some(("upgrade.version.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.version.name" => Some(("upgrade.version.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.version.revision" => Some(("upgrade.version.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.windows-update.description" => Some(("upgrade.windowsUpdate.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.windows-update.identity.revision" => Some(("upgrade.windowsUpdate.identity.revision", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "upgrade.windows-update.identity.update-id" => Some(("upgrade.windowsUpdate.identity.updateId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.windows-update.kb-article-ids" => Some(("upgrade.windowsUpdate.kbArticleIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "upgrade.windows-update.last-published-timestamp" => Some(("upgrade.windowsUpdate.lastPublishedTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.windows-update.support-url" => Some(("upgrade.windowsUpdate.supportUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.windows-update.title" => Some(("upgrade.windowsUpdate.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-score" => Some(("vulnerability.cvssScore", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.attack-complexity" => Some(("vulnerability.cvssV2.attackComplexity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.attack-vector" => Some(("vulnerability.cvssV2.attackVector", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.authentication" => Some(("vulnerability.cvssV2.authentication", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.availability-impact" => Some(("vulnerability.cvssV2.availabilityImpact", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.base-score" => Some(("vulnerability.cvssV2.baseScore", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.confidentiality-impact" => Some(("vulnerability.cvssV2.confidentialityImpact", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.exploitability-score" => Some(("vulnerability.cvssV2.exploitabilityScore", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.impact-score" => Some(("vulnerability.cvssV2.impactScore", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.integrity-impact" => Some(("vulnerability.cvssV2.integrityImpact", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.privileges-required" => Some(("vulnerability.cvssV2.privilegesRequired", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.scope" => Some(("vulnerability.cvssV2.scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.user-interaction" => Some(("vulnerability.cvssV2.userInteraction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.attack-complexity" => Some(("vulnerability.cvssV3.attackComplexity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.attack-vector" => Some(("vulnerability.cvssV3.attackVector", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.availability-impact" => Some(("vulnerability.cvssV3.availabilityImpact", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.base-score" => Some(("vulnerability.cvssV3.baseScore", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.confidentiality-impact" => Some(("vulnerability.cvssV3.confidentialityImpact", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.exploitability-score" => Some(("vulnerability.cvssV3.exploitabilityScore", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.impact-score" => Some(("vulnerability.cvssV3.impactScore", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.integrity-impact" => Some(("vulnerability.cvssV3.integrityImpact", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.privileges-required" => Some(("vulnerability.cvssV3.privilegesRequired", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.scope" => Some(("vulnerability.cvssV3.scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.user-interaction" => Some(("vulnerability.cvssV3.userInteraction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-version" => Some(("vulnerability.cvssVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.severity" => Some(("vulnerability.severity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.source-update-time" => Some(("vulnerability.sourceUpdateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability-assessment.assessment.cve" => Some(("vulnerabilityAssessment.assessment.cve", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability-assessment.assessment.impacts" => Some(("vulnerabilityAssessment.assessment.impacts", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "vulnerability-assessment.assessment.justification.details" => Some(("vulnerabilityAssessment.assessment.justification.details", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability-assessment.assessment.justification.justification-type" => Some(("vulnerabilityAssessment.assessment.justification.justificationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability-assessment.assessment.long-description" => Some(("vulnerabilityAssessment.assessment.longDescription", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability-assessment.assessment.short-description" => Some(("vulnerabilityAssessment.assessment.shortDescription", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability-assessment.assessment.state" => Some(("vulnerabilityAssessment.assessment.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability-assessment.assessment.vulnerability-id" => Some(("vulnerabilityAssessment.assessment.vulnerabilityId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability-assessment.language-code" => Some(("vulnerabilityAssessment.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability-assessment.long-description" => Some(("vulnerabilityAssessment.longDescription", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability-assessment.product.generic-uri" => Some(("vulnerabilityAssessment.product.genericUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability-assessment.product.id" => Some(("vulnerabilityAssessment.product.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability-assessment.product.name" => Some(("vulnerabilityAssessment.product.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability-assessment.publisher.issuing-authority" => Some(("vulnerabilityAssessment.publisher.issuingAuthority", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability-assessment.publisher.name" => Some(("vulnerabilityAssessment.publisher.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability-assessment.publisher.publisher-namespace" => Some(("vulnerabilityAssessment.publisher.publisherNamespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability-assessment.short-description" => Some(("vulnerabilityAssessment.shortDescription", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability-assessment.title" => Some(("vulnerabilityAssessment.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["analysis-kind", "architecture", "assessment", "attack-complexity", "attack-vector", "attestation", "authentication", "availability-impact", "base-score", "build", "builder-version", "cis-benchmark", "comments", "compliance", "confidentiality-impact", "cpe-uri", "create-time", "cve", "cvss-score", "cvss-v2", "cvss-v3", "cvss-version", "deployment", "description", "details", "discovery", "dsse-attestation", "epoch", "expiration-time", "exploitability-score", "expression", "fingerprint", "format", "full-name", "generic-uri", "hint", "human-readable-name", "id", "identity", "image", "impact", "impact-score", "impacts", "inclusive", "integrity-impact", "issuing-authority", "justification", "justification-type", "kb-article-ids", "kind", "language-code", "last-published-timestamp", "license", "long-description", "maintainer", "name", "package", "package-type", "privileges-required", "product", "profile-level", "publisher", "publisher-namespace", "rationale", "related-note-names", "remediation", "resource-uri", "resource-url", "revision", "sbom-reference", "scan-instructions", "scope", "severity", "short-description", "source-update-time", "state", "support-url", "title", "update-id", "update-time", "upgrade", "url", "user-interaction", "v1-name", "v2-blob", "v2-name", "version", "vulnerability", "vulnerability-assessment", "vulnerability-id", "windows-update"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Note = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().notes_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "note-id" => {
                    call = call.note_id(value.unwrap_or(""));
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
                                                                           v.extend(["note-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _projects_notes_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().notes_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_notes_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().notes_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_notes_get_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.projects().notes_get_iam_policy(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_notes_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().notes_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_notes_occurrences_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().notes_occurrences_list(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_notes_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "attestation.hint.human-readable-name" => Some(("attestation.hint.humanReadableName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.builder-version" => Some(("build.builderVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "compliance.cis-benchmark.profile-level" => Some(("compliance.cisBenchmark.profileLevel", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "compliance.cis-benchmark.severity" => Some(("compliance.cisBenchmark.severity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "compliance.description" => Some(("compliance.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "compliance.impact" => Some(("compliance.impact", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "compliance.rationale" => Some(("compliance.rationale", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "compliance.remediation" => Some(("compliance.remediation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "compliance.scan-instructions" => Some(("compliance.scanInstructions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "compliance.title" => Some(("compliance.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment.resource-uri" => Some(("deployment.resourceUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "discovery.analysis-kind" => Some(("discovery.analysisKind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.hint.human-readable-name" => Some(("dsseAttestation.hint.humanReadableName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "expiration-time" => Some(("expirationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image.fingerprint.v1-name" => Some(("image.fingerprint.v1Name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image.fingerprint.v2-blob" => Some(("image.fingerprint.v2Blob", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "image.fingerprint.v2-name" => Some(("image.fingerprint.v2Name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image.resource-url" => Some(("image.resourceUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "long-description" => Some(("longDescription", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.architecture" => Some(("package.architecture", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.cpe-uri" => Some(("package.cpeUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.description" => Some(("package.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.license.comments" => Some(("package.license.comments", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.license.expression" => Some(("package.license.expression", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.maintainer" => Some(("package.maintainer", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.name" => Some(("package.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.package-type" => Some(("package.packageType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.url" => Some(("package.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.version.epoch" => Some(("package.version.epoch", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "package.version.full-name" => Some(("package.version.fullName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.version.inclusive" => Some(("package.version.inclusive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "package.version.kind" => Some(("package.version.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.version.name" => Some(("package.version.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.version.revision" => Some(("package.version.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "related-note-names" => Some(("relatedNoteNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "sbom-reference.format" => Some(("sbomReference.format", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sbom-reference.version" => Some(("sbomReference.version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "short-description" => Some(("shortDescription", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.package" => Some(("upgrade.package", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.version.epoch" => Some(("upgrade.version.epoch", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "upgrade.version.full-name" => Some(("upgrade.version.fullName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.version.inclusive" => Some(("upgrade.version.inclusive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "upgrade.version.kind" => Some(("upgrade.version.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.version.name" => Some(("upgrade.version.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.version.revision" => Some(("upgrade.version.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.windows-update.description" => Some(("upgrade.windowsUpdate.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.windows-update.identity.revision" => Some(("upgrade.windowsUpdate.identity.revision", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "upgrade.windows-update.identity.update-id" => Some(("upgrade.windowsUpdate.identity.updateId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.windows-update.kb-article-ids" => Some(("upgrade.windowsUpdate.kbArticleIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "upgrade.windows-update.last-published-timestamp" => Some(("upgrade.windowsUpdate.lastPublishedTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.windows-update.support-url" => Some(("upgrade.windowsUpdate.supportUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.windows-update.title" => Some(("upgrade.windowsUpdate.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-score" => Some(("vulnerability.cvssScore", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.attack-complexity" => Some(("vulnerability.cvssV2.attackComplexity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.attack-vector" => Some(("vulnerability.cvssV2.attackVector", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.authentication" => Some(("vulnerability.cvssV2.authentication", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.availability-impact" => Some(("vulnerability.cvssV2.availabilityImpact", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.base-score" => Some(("vulnerability.cvssV2.baseScore", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.confidentiality-impact" => Some(("vulnerability.cvssV2.confidentialityImpact", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.exploitability-score" => Some(("vulnerability.cvssV2.exploitabilityScore", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.impact-score" => Some(("vulnerability.cvssV2.impactScore", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.integrity-impact" => Some(("vulnerability.cvssV2.integrityImpact", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.privileges-required" => Some(("vulnerability.cvssV2.privilegesRequired", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.scope" => Some(("vulnerability.cvssV2.scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.user-interaction" => Some(("vulnerability.cvssV2.userInteraction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.attack-complexity" => Some(("vulnerability.cvssV3.attackComplexity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.attack-vector" => Some(("vulnerability.cvssV3.attackVector", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.availability-impact" => Some(("vulnerability.cvssV3.availabilityImpact", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.base-score" => Some(("vulnerability.cvssV3.baseScore", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.confidentiality-impact" => Some(("vulnerability.cvssV3.confidentialityImpact", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.exploitability-score" => Some(("vulnerability.cvssV3.exploitabilityScore", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.impact-score" => Some(("vulnerability.cvssV3.impactScore", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.integrity-impact" => Some(("vulnerability.cvssV3.integrityImpact", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.privileges-required" => Some(("vulnerability.cvssV3.privilegesRequired", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.scope" => Some(("vulnerability.cvssV3.scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v3.user-interaction" => Some(("vulnerability.cvssV3.userInteraction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-version" => Some(("vulnerability.cvssVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.severity" => Some(("vulnerability.severity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.source-update-time" => Some(("vulnerability.sourceUpdateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability-assessment.assessment.cve" => Some(("vulnerabilityAssessment.assessment.cve", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability-assessment.assessment.impacts" => Some(("vulnerabilityAssessment.assessment.impacts", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "vulnerability-assessment.assessment.justification.details" => Some(("vulnerabilityAssessment.assessment.justification.details", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability-assessment.assessment.justification.justification-type" => Some(("vulnerabilityAssessment.assessment.justification.justificationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability-assessment.assessment.long-description" => Some(("vulnerabilityAssessment.assessment.longDescription", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability-assessment.assessment.short-description" => Some(("vulnerabilityAssessment.assessment.shortDescription", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability-assessment.assessment.state" => Some(("vulnerabilityAssessment.assessment.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability-assessment.assessment.vulnerability-id" => Some(("vulnerabilityAssessment.assessment.vulnerabilityId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability-assessment.language-code" => Some(("vulnerabilityAssessment.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability-assessment.long-description" => Some(("vulnerabilityAssessment.longDescription", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability-assessment.product.generic-uri" => Some(("vulnerabilityAssessment.product.genericUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability-assessment.product.id" => Some(("vulnerabilityAssessment.product.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability-assessment.product.name" => Some(("vulnerabilityAssessment.product.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability-assessment.publisher.issuing-authority" => Some(("vulnerabilityAssessment.publisher.issuingAuthority", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability-assessment.publisher.name" => Some(("vulnerabilityAssessment.publisher.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability-assessment.publisher.publisher-namespace" => Some(("vulnerabilityAssessment.publisher.publisherNamespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability-assessment.short-description" => Some(("vulnerabilityAssessment.shortDescription", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability-assessment.title" => Some(("vulnerabilityAssessment.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["analysis-kind", "architecture", "assessment", "attack-complexity", "attack-vector", "attestation", "authentication", "availability-impact", "base-score", "build", "builder-version", "cis-benchmark", "comments", "compliance", "confidentiality-impact", "cpe-uri", "create-time", "cve", "cvss-score", "cvss-v2", "cvss-v3", "cvss-version", "deployment", "description", "details", "discovery", "dsse-attestation", "epoch", "expiration-time", "exploitability-score", "expression", "fingerprint", "format", "full-name", "generic-uri", "hint", "human-readable-name", "id", "identity", "image", "impact", "impact-score", "impacts", "inclusive", "integrity-impact", "issuing-authority", "justification", "justification-type", "kb-article-ids", "kind", "language-code", "last-published-timestamp", "license", "long-description", "maintainer", "name", "package", "package-type", "privileges-required", "product", "profile-level", "publisher", "publisher-namespace", "rationale", "related-note-names", "remediation", "resource-uri", "resource-url", "revision", "sbom-reference", "scan-instructions", "scope", "severity", "short-description", "source-update-time", "state", "support-url", "title", "update-id", "update-time", "upgrade", "url", "user-interaction", "v1-name", "v2-blob", "v2-name", "version", "vulnerability", "vulnerability-assessment", "vulnerability-id", "windows-update"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Note = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().notes_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_notes_set_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["etag", "policy", "version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SetIamPolicyRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().notes_set_iam_policy(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_notes_test_iam_permissions(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.projects().notes_test_iam_permissions(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_occurrences_batch_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::BatchCreateOccurrencesRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().occurrences_batch_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_occurrences_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "attestation.serialized-payload" => Some(("attestation.serializedPayload", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.in-toto-slsa-provenance-v1.-type" => Some(("build.inTotoSlsaProvenanceV1._type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.in-toto-slsa-provenance-v1.predicate.build-definition.build-type" => Some(("build.inTotoSlsaProvenanceV1.predicate.buildDefinition.buildType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.in-toto-slsa-provenance-v1.predicate.run-details.builder.id" => Some(("build.inTotoSlsaProvenanceV1.predicate.runDetails.builder.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.in-toto-slsa-provenance-v1.predicate.run-details.builder.version" => Some(("build.inTotoSlsaProvenanceV1.predicate.runDetails.builder.version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "build.in-toto-slsa-provenance-v1.predicate.run-details.metadata.finished-on" => Some(("build.inTotoSlsaProvenanceV1.predicate.runDetails.metadata.finishedOn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.in-toto-slsa-provenance-v1.predicate.run-details.metadata.invocation-id" => Some(("build.inTotoSlsaProvenanceV1.predicate.runDetails.metadata.invocationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.in-toto-slsa-provenance-v1.predicate.run-details.metadata.started-on" => Some(("build.inTotoSlsaProvenanceV1.predicate.runDetails.metadata.startedOn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.in-toto-slsa-provenance-v1.predicate-type" => Some(("build.inTotoSlsaProvenanceV1.predicateType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-provenance.builder-config.id" => Some(("build.intotoProvenance.builderConfig.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-provenance.materials" => Some(("build.intotoProvenance.materials", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.intoto-provenance.metadata.build-finished-on" => Some(("build.intotoProvenance.metadata.buildFinishedOn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-provenance.metadata.build-invocation-id" => Some(("build.intotoProvenance.metadata.buildInvocationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-provenance.metadata.build-started-on" => Some(("build.intotoProvenance.metadata.buildStartedOn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-provenance.metadata.completeness.arguments" => Some(("build.intotoProvenance.metadata.completeness.arguments", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.intoto-provenance.metadata.completeness.environment" => Some(("build.intotoProvenance.metadata.completeness.environment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.intoto-provenance.metadata.completeness.materials" => Some(("build.intotoProvenance.metadata.completeness.materials", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.intoto-provenance.metadata.reproducible" => Some(("build.intotoProvenance.metadata.reproducible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.intoto-provenance.recipe.defined-in-material" => Some(("build.intotoProvenance.recipe.definedInMaterial", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-provenance.recipe.entry-point" => Some(("build.intotoProvenance.recipe.entryPoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-provenance.recipe.type" => Some(("build.intotoProvenance.recipe.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.-type" => Some(("build.intotoStatement._type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.predicate-type" => Some(("build.intotoStatement.predicateType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.provenance.builder-config.id" => Some(("build.intotoStatement.provenance.builderConfig.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.provenance.materials" => Some(("build.intotoStatement.provenance.materials", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.intoto-statement.provenance.metadata.build-finished-on" => Some(("build.intotoStatement.provenance.metadata.buildFinishedOn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.provenance.metadata.build-invocation-id" => Some(("build.intotoStatement.provenance.metadata.buildInvocationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.provenance.metadata.build-started-on" => Some(("build.intotoStatement.provenance.metadata.buildStartedOn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.provenance.metadata.completeness.arguments" => Some(("build.intotoStatement.provenance.metadata.completeness.arguments", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.intoto-statement.provenance.metadata.completeness.environment" => Some(("build.intotoStatement.provenance.metadata.completeness.environment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.intoto-statement.provenance.metadata.completeness.materials" => Some(("build.intotoStatement.provenance.metadata.completeness.materials", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.intoto-statement.provenance.metadata.reproducible" => Some(("build.intotoStatement.provenance.metadata.reproducible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.intoto-statement.provenance.recipe.defined-in-material" => Some(("build.intotoStatement.provenance.recipe.definedInMaterial", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.provenance.recipe.entry-point" => Some(("build.intotoStatement.provenance.recipe.entryPoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.provenance.recipe.type" => Some(("build.intotoStatement.provenance.recipe.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance.builder.id" => Some(("build.intotoStatement.slsaProvenance.builder.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance.metadata.build-finished-on" => Some(("build.intotoStatement.slsaProvenance.metadata.buildFinishedOn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance.metadata.build-invocation-id" => Some(("build.intotoStatement.slsaProvenance.metadata.buildInvocationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance.metadata.build-started-on" => Some(("build.intotoStatement.slsaProvenance.metadata.buildStartedOn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance.metadata.completeness.arguments" => Some(("build.intotoStatement.slsaProvenance.metadata.completeness.arguments", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance.metadata.completeness.environment" => Some(("build.intotoStatement.slsaProvenance.metadata.completeness.environment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance.metadata.completeness.materials" => Some(("build.intotoStatement.slsaProvenance.metadata.completeness.materials", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance.metadata.reproducible" => Some(("build.intotoStatement.slsaProvenance.metadata.reproducible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance.recipe.defined-in-material" => Some(("build.intotoStatement.slsaProvenance.recipe.definedInMaterial", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance.recipe.entry-point" => Some(("build.intotoStatement.slsaProvenance.recipe.entryPoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance.recipe.type" => Some(("build.intotoStatement.slsaProvenance.recipe.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance-zero-two.build-type" => Some(("build.intotoStatement.slsaProvenanceZeroTwo.buildType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance-zero-two.builder.id" => Some(("build.intotoStatement.slsaProvenanceZeroTwo.builder.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance-zero-two.invocation.config-source.digest" => Some(("build.intotoStatement.slsaProvenanceZeroTwo.invocation.configSource.digest", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "build.intoto-statement.slsa-provenance-zero-two.invocation.config-source.entry-point" => Some(("build.intotoStatement.slsaProvenanceZeroTwo.invocation.configSource.entryPoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance-zero-two.invocation.config-source.uri" => Some(("build.intotoStatement.slsaProvenanceZeroTwo.invocation.configSource.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance-zero-two.metadata.build-finished-on" => Some(("build.intotoStatement.slsaProvenanceZeroTwo.metadata.buildFinishedOn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance-zero-two.metadata.build-invocation-id" => Some(("build.intotoStatement.slsaProvenanceZeroTwo.metadata.buildInvocationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance-zero-two.metadata.build-started-on" => Some(("build.intotoStatement.slsaProvenanceZeroTwo.metadata.buildStartedOn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance-zero-two.metadata.completeness.environment" => Some(("build.intotoStatement.slsaProvenanceZeroTwo.metadata.completeness.environment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance-zero-two.metadata.completeness.materials" => Some(("build.intotoStatement.slsaProvenanceZeroTwo.metadata.completeness.materials", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance-zero-two.metadata.completeness.parameters" => Some(("build.intotoStatement.slsaProvenanceZeroTwo.metadata.completeness.parameters", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance-zero-two.metadata.reproducible" => Some(("build.intotoStatement.slsaProvenanceZeroTwo.metadata.reproducible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.provenance.build-options" => Some(("build.provenance.buildOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "build.provenance.builder-version" => Some(("build.provenance.builderVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.create-time" => Some(("build.provenance.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.creator" => Some(("build.provenance.creator", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.end-time" => Some(("build.provenance.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.id" => Some(("build.provenance.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.logs-uri" => Some(("build.provenance.logsUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.project-id" => Some(("build.provenance.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.artifact-storage-source-uri" => Some(("build.provenance.sourceProvenance.artifactStorageSourceUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.cloud-repo.alias-context.kind" => Some(("build.provenance.sourceProvenance.context.cloudRepo.aliasContext.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.cloud-repo.alias-context.name" => Some(("build.provenance.sourceProvenance.context.cloudRepo.aliasContext.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.cloud-repo.repo-id.project-repo-id.project-id" => Some(("build.provenance.sourceProvenance.context.cloudRepo.repoId.projectRepoId.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.cloud-repo.repo-id.project-repo-id.repo-name" => Some(("build.provenance.sourceProvenance.context.cloudRepo.repoId.projectRepoId.repoName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.cloud-repo.repo-id.uid" => Some(("build.provenance.sourceProvenance.context.cloudRepo.repoId.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.cloud-repo.revision-id" => Some(("build.provenance.sourceProvenance.context.cloudRepo.revisionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.gerrit.alias-context.kind" => Some(("build.provenance.sourceProvenance.context.gerrit.aliasContext.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.gerrit.alias-context.name" => Some(("build.provenance.sourceProvenance.context.gerrit.aliasContext.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.gerrit.gerrit-project" => Some(("build.provenance.sourceProvenance.context.gerrit.gerritProject", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.gerrit.host-uri" => Some(("build.provenance.sourceProvenance.context.gerrit.hostUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.gerrit.revision-id" => Some(("build.provenance.sourceProvenance.context.gerrit.revisionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.git.revision-id" => Some(("build.provenance.sourceProvenance.context.git.revisionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.git.url" => Some(("build.provenance.sourceProvenance.context.git.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.labels" => Some(("build.provenance.sourceProvenance.context.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "build.provenance.start-time" => Some(("build.provenance.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.trigger-id" => Some(("build.provenance.triggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance-bytes" => Some(("build.provenanceBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "compliance.non-compliance-reason" => Some(("compliance.nonComplianceReason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "compliance.version.benchmark-document" => Some(("compliance.version.benchmarkDocument", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "compliance.version.cpe-uri" => Some(("compliance.version.cpeUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "compliance.version.version" => Some(("compliance.version.version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment.address" => Some(("deployment.address", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment.config" => Some(("deployment.config", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment.deploy-time" => Some(("deployment.deployTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment.platform" => Some(("deployment.platform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment.resource-uri" => Some(("deployment.resourceUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "deployment.undeploy-time" => Some(("deployment.undeployTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment.user-email" => Some(("deployment.userEmail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery.analysis-completed.analysis-type" => Some(("discovery.analysisCompleted.analysisType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "discovery.analysis-status" => Some(("discovery.analysisStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery.analysis-status-error.code" => Some(("discovery.analysisStatusError.code", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "discovery.analysis-status-error.message" => Some(("discovery.analysisStatusError.message", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery.archive-time" => Some(("discovery.archiveTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery.continuous-analysis" => Some(("discovery.continuousAnalysis", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery.cpe" => Some(("discovery.cpe", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery.last-scan-time" => Some(("discovery.lastScanTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery.sbom-status.error" => Some(("discovery.sbomStatus.error", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery.sbom-status.sbom-state" => Some(("discovery.sbomStatus.sbomState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery.vulnerability-attestation.error" => Some(("discovery.vulnerabilityAttestation.error", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery.vulnerability-attestation.last-attempt-time" => Some(("discovery.vulnerabilityAttestation.lastAttemptTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery.vulnerability-attestation.state" => Some(("discovery.vulnerabilityAttestation.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.envelope.payload" => Some(("dsseAttestation.envelope.payload", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.envelope.payload-type" => Some(("dsseAttestation.envelope.payloadType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.-type" => Some(("dsseAttestation.statement._type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.predicate-type" => Some(("dsseAttestation.statement.predicateType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.provenance.builder-config.id" => Some(("dsseAttestation.statement.provenance.builderConfig.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.provenance.materials" => Some(("dsseAttestation.statement.provenance.materials", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "dsse-attestation.statement.provenance.metadata.build-finished-on" => Some(("dsseAttestation.statement.provenance.metadata.buildFinishedOn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.provenance.metadata.build-invocation-id" => Some(("dsseAttestation.statement.provenance.metadata.buildInvocationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.provenance.metadata.build-started-on" => Some(("dsseAttestation.statement.provenance.metadata.buildStartedOn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.provenance.metadata.completeness.arguments" => Some(("dsseAttestation.statement.provenance.metadata.completeness.arguments", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.provenance.metadata.completeness.environment" => Some(("dsseAttestation.statement.provenance.metadata.completeness.environment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.provenance.metadata.completeness.materials" => Some(("dsseAttestation.statement.provenance.metadata.completeness.materials", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.provenance.metadata.reproducible" => Some(("dsseAttestation.statement.provenance.metadata.reproducible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.provenance.recipe.defined-in-material" => Some(("dsseAttestation.statement.provenance.recipe.definedInMaterial", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.provenance.recipe.entry-point" => Some(("dsseAttestation.statement.provenance.recipe.entryPoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.provenance.recipe.type" => Some(("dsseAttestation.statement.provenance.recipe.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance.builder.id" => Some(("dsseAttestation.statement.slsaProvenance.builder.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance.metadata.build-finished-on" => Some(("dsseAttestation.statement.slsaProvenance.metadata.buildFinishedOn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance.metadata.build-invocation-id" => Some(("dsseAttestation.statement.slsaProvenance.metadata.buildInvocationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance.metadata.build-started-on" => Some(("dsseAttestation.statement.slsaProvenance.metadata.buildStartedOn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance.metadata.completeness.arguments" => Some(("dsseAttestation.statement.slsaProvenance.metadata.completeness.arguments", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance.metadata.completeness.environment" => Some(("dsseAttestation.statement.slsaProvenance.metadata.completeness.environment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance.metadata.completeness.materials" => Some(("dsseAttestation.statement.slsaProvenance.metadata.completeness.materials", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance.metadata.reproducible" => Some(("dsseAttestation.statement.slsaProvenance.metadata.reproducible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance.recipe.defined-in-material" => Some(("dsseAttestation.statement.slsaProvenance.recipe.definedInMaterial", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance.recipe.entry-point" => Some(("dsseAttestation.statement.slsaProvenance.recipe.entryPoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance.recipe.type" => Some(("dsseAttestation.statement.slsaProvenance.recipe.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance-zero-two.build-type" => Some(("dsseAttestation.statement.slsaProvenanceZeroTwo.buildType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance-zero-two.builder.id" => Some(("dsseAttestation.statement.slsaProvenanceZeroTwo.builder.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance-zero-two.invocation.config-source.digest" => Some(("dsseAttestation.statement.slsaProvenanceZeroTwo.invocation.configSource.digest", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "dsse-attestation.statement.slsa-provenance-zero-two.invocation.config-source.entry-point" => Some(("dsseAttestation.statement.slsaProvenanceZeroTwo.invocation.configSource.entryPoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance-zero-two.invocation.config-source.uri" => Some(("dsseAttestation.statement.slsaProvenanceZeroTwo.invocation.configSource.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance-zero-two.metadata.build-finished-on" => Some(("dsseAttestation.statement.slsaProvenanceZeroTwo.metadata.buildFinishedOn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance-zero-two.metadata.build-invocation-id" => Some(("dsseAttestation.statement.slsaProvenanceZeroTwo.metadata.buildInvocationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance-zero-two.metadata.build-started-on" => Some(("dsseAttestation.statement.slsaProvenanceZeroTwo.metadata.buildStartedOn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance-zero-two.metadata.completeness.environment" => Some(("dsseAttestation.statement.slsaProvenanceZeroTwo.metadata.completeness.environment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance-zero-two.metadata.completeness.materials" => Some(("dsseAttestation.statement.slsaProvenanceZeroTwo.metadata.completeness.materials", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance-zero-two.metadata.completeness.parameters" => Some(("dsseAttestation.statement.slsaProvenanceZeroTwo.metadata.completeness.parameters", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance-zero-two.metadata.reproducible" => Some(("dsseAttestation.statement.slsaProvenanceZeroTwo.metadata.reproducible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "envelope.payload" => Some(("envelope.payload", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "envelope.payload-type" => Some(("envelope.payloadType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image.base-resource-url" => Some(("image.baseResourceUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image.distance" => Some(("image.distance", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image.fingerprint.v1-name" => Some(("image.fingerprint.v1Name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image.fingerprint.v2-blob" => Some(("image.fingerprint.v2Blob", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "image.fingerprint.v2-name" => Some(("image.fingerprint.v2Name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "note-name" => Some(("noteName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.architecture" => Some(("package.architecture", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.cpe-uri" => Some(("package.cpeUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.license.comments" => Some(("package.license.comments", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.license.expression" => Some(("package.license.expression", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.name" => Some(("package.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.package-type" => Some(("package.packageType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.version.epoch" => Some(("package.version.epoch", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "package.version.full-name" => Some(("package.version.fullName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.version.inclusive" => Some(("package.version.inclusive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "package.version.kind" => Some(("package.version.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.version.name" => Some(("package.version.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.version.revision" => Some(("package.version.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "remediation" => Some(("remediation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "resource-uri" => Some(("resourceUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sbom-reference.payload.-type" => Some(("sbomReference.payload._type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sbom-reference.payload.predicate.digest" => Some(("sbomReference.payload.predicate.digest", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "sbom-reference.payload.predicate.location" => Some(("sbomReference.payload.predicate.location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sbom-reference.payload.predicate.mime-type" => Some(("sbomReference.payload.predicate.mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sbom-reference.payload.predicate.referrer-id" => Some(("sbomReference.payload.predicate.referrerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sbom-reference.payload.predicate-type" => Some(("sbomReference.payload.predicateType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sbom-reference.payload-type" => Some(("sbomReference.payloadType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.distribution.classification" => Some(("upgrade.distribution.classification", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.distribution.cpe-uri" => Some(("upgrade.distribution.cpeUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.distribution.cve" => Some(("upgrade.distribution.cve", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "upgrade.distribution.severity" => Some(("upgrade.distribution.severity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.package" => Some(("upgrade.package", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.parsed-version.epoch" => Some(("upgrade.parsedVersion.epoch", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "upgrade.parsed-version.full-name" => Some(("upgrade.parsedVersion.fullName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.parsed-version.inclusive" => Some(("upgrade.parsedVersion.inclusive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "upgrade.parsed-version.kind" => Some(("upgrade.parsedVersion.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.parsed-version.name" => Some(("upgrade.parsedVersion.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.parsed-version.revision" => Some(("upgrade.parsedVersion.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.windows-update.description" => Some(("upgrade.windowsUpdate.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.windows-update.identity.revision" => Some(("upgrade.windowsUpdate.identity.revision", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "upgrade.windows-update.identity.update-id" => Some(("upgrade.windowsUpdate.identity.updateId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.windows-update.kb-article-ids" => Some(("upgrade.windowsUpdate.kbArticleIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "upgrade.windows-update.last-published-timestamp" => Some(("upgrade.windowsUpdate.lastPublishedTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.windows-update.support-url" => Some(("upgrade.windowsUpdate.supportUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.windows-update.title" => Some(("upgrade.windowsUpdate.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-score" => Some(("vulnerability.cvssScore", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.attack-complexity" => Some(("vulnerability.cvssV2.attackComplexity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.attack-vector" => Some(("vulnerability.cvssV2.attackVector", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.authentication" => Some(("vulnerability.cvssV2.authentication", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.availability-impact" => Some(("vulnerability.cvssV2.availabilityImpact", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.base-score" => Some(("vulnerability.cvssV2.baseScore", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.confidentiality-impact" => Some(("vulnerability.cvssV2.confidentialityImpact", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.exploitability-score" => Some(("vulnerability.cvssV2.exploitabilityScore", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.impact-score" => Some(("vulnerability.cvssV2.impactScore", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.integrity-impact" => Some(("vulnerability.cvssV2.integrityImpact", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.privileges-required" => Some(("vulnerability.cvssV2.privilegesRequired", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.scope" => Some(("vulnerability.cvssV2.scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.user-interaction" => Some(("vulnerability.cvssV2.userInteraction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-version" => Some(("vulnerability.cvssVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvssv3.attack-complexity" => Some(("vulnerability.cvssv3.attackComplexity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvssv3.attack-vector" => Some(("vulnerability.cvssv3.attackVector", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvssv3.authentication" => Some(("vulnerability.cvssv3.authentication", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvssv3.availability-impact" => Some(("vulnerability.cvssv3.availabilityImpact", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvssv3.base-score" => Some(("vulnerability.cvssv3.baseScore", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "vulnerability.cvssv3.confidentiality-impact" => Some(("vulnerability.cvssv3.confidentialityImpact", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvssv3.exploitability-score" => Some(("vulnerability.cvssv3.exploitabilityScore", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "vulnerability.cvssv3.impact-score" => Some(("vulnerability.cvssv3.impactScore", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "vulnerability.cvssv3.integrity-impact" => Some(("vulnerability.cvssv3.integrityImpact", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvssv3.privileges-required" => Some(("vulnerability.cvssv3.privilegesRequired", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvssv3.scope" => Some(("vulnerability.cvssv3.scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvssv3.user-interaction" => Some(("vulnerability.cvssv3.userInteraction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.effective-severity" => Some(("vulnerability.effectiveSeverity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.extra-details" => Some(("vulnerability.extraDetails", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.fix-available" => Some(("vulnerability.fixAvailable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "vulnerability.long-description" => Some(("vulnerability.longDescription", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.severity" => Some(("vulnerability.severity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.short-description" => Some(("vulnerability.shortDescription", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.type" => Some(("vulnerability.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.vex-assessment.cve" => Some(("vulnerability.vexAssessment.cve", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.vex-assessment.impacts" => Some(("vulnerability.vexAssessment.impacts", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "vulnerability.vex-assessment.justification.details" => Some(("vulnerability.vexAssessment.justification.details", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.vex-assessment.justification.justification-type" => Some(("vulnerability.vexAssessment.justification.justificationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.vex-assessment.note-name" => Some(("vulnerability.vexAssessment.noteName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.vex-assessment.state" => Some(("vulnerability.vexAssessment.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.vex-assessment.vulnerability-id" => Some(("vulnerability.vexAssessment.vulnerabilityId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["-type", "address", "alias-context", "analysis-completed", "analysis-status", "analysis-status-error", "analysis-type", "architecture", "archive-time", "arguments", "artifact-storage-source-uri", "attack-complexity", "attack-vector", "attestation", "authentication", "availability-impact", "base-resource-url", "base-score", "benchmark-document", "build", "build-definition", "build-finished-on", "build-invocation-id", "build-options", "build-started-on", "build-type", "builder", "builder-config", "builder-version", "classification", "cloud-repo", "code", "comments", "completeness", "compliance", "confidentiality-impact", "config", "config-source", "context", "continuous-analysis", "cpe", "cpe-uri", "create-time", "creator", "cve", "cvss-score", "cvss-v2", "cvss-version", "cvssv3", "defined-in-material", "deploy-time", "deployment", "description", "details", "digest", "discovery", "distance", "distribution", "dsse-attestation", "effective-severity", "end-time", "entry-point", "envelope", "environment", "epoch", "error", "exploitability-score", "expression", "extra-details", "fingerprint", "finished-on", "fix-available", "full-name", "gerrit", "gerrit-project", "git", "host-uri", "id", "identity", "image", "impact-score", "impacts", "in-toto-slsa-provenance-v1", "inclusive", "integrity-impact", "intoto-provenance", "intoto-statement", "invocation", "invocation-id", "justification", "justification-type", "kb-article-ids", "kind", "labels", "last-attempt-time", "last-published-timestamp", "last-scan-time", "license", "location", "logs-uri", "long-description", "materials", "message", "metadata", "mime-type", "name", "non-compliance-reason", "note-name", "package", "package-type", "parameters", "parsed-version", "payload", "payload-type", "platform", "predicate", "predicate-type", "privileges-required", "project-id", "project-repo-id", "provenance", "provenance-bytes", "recipe", "referrer-id", "remediation", "repo-id", "repo-name", "reproducible", "resource-uri", "revision", "revision-id", "run-details", "sbom-reference", "sbom-state", "sbom-status", "scope", "serialized-payload", "severity", "short-description", "slsa-provenance", "slsa-provenance-zero-two", "source-provenance", "start-time", "started-on", "state", "statement", "support-url", "title", "trigger-id", "type", "uid", "undeploy-time", "update-id", "update-time", "upgrade", "uri", "url", "user-email", "user-interaction", "v1-name", "v2-blob", "v2-name", "version", "vex-assessment", "vulnerability", "vulnerability-attestation", "vulnerability-id", "windows-update"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Occurrence = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().occurrences_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_occurrences_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().occurrences_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_occurrences_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().occurrences_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_occurrences_get_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.projects().occurrences_get_iam_policy(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_occurrences_get_notes(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().occurrences_get_notes(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_occurrences_get_vulnerability_summary(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().occurrences_get_vulnerability_summary(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
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
                                                                           v.extend(["filter"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _projects_occurrences_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().occurrences_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_occurrences_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "attestation.serialized-payload" => Some(("attestation.serializedPayload", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.in-toto-slsa-provenance-v1.-type" => Some(("build.inTotoSlsaProvenanceV1._type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.in-toto-slsa-provenance-v1.predicate.build-definition.build-type" => Some(("build.inTotoSlsaProvenanceV1.predicate.buildDefinition.buildType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.in-toto-slsa-provenance-v1.predicate.run-details.builder.id" => Some(("build.inTotoSlsaProvenanceV1.predicate.runDetails.builder.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.in-toto-slsa-provenance-v1.predicate.run-details.builder.version" => Some(("build.inTotoSlsaProvenanceV1.predicate.runDetails.builder.version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "build.in-toto-slsa-provenance-v1.predicate.run-details.metadata.finished-on" => Some(("build.inTotoSlsaProvenanceV1.predicate.runDetails.metadata.finishedOn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.in-toto-slsa-provenance-v1.predicate.run-details.metadata.invocation-id" => Some(("build.inTotoSlsaProvenanceV1.predicate.runDetails.metadata.invocationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.in-toto-slsa-provenance-v1.predicate.run-details.metadata.started-on" => Some(("build.inTotoSlsaProvenanceV1.predicate.runDetails.metadata.startedOn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.in-toto-slsa-provenance-v1.predicate-type" => Some(("build.inTotoSlsaProvenanceV1.predicateType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-provenance.builder-config.id" => Some(("build.intotoProvenance.builderConfig.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-provenance.materials" => Some(("build.intotoProvenance.materials", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.intoto-provenance.metadata.build-finished-on" => Some(("build.intotoProvenance.metadata.buildFinishedOn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-provenance.metadata.build-invocation-id" => Some(("build.intotoProvenance.metadata.buildInvocationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-provenance.metadata.build-started-on" => Some(("build.intotoProvenance.metadata.buildStartedOn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-provenance.metadata.completeness.arguments" => Some(("build.intotoProvenance.metadata.completeness.arguments", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.intoto-provenance.metadata.completeness.environment" => Some(("build.intotoProvenance.metadata.completeness.environment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.intoto-provenance.metadata.completeness.materials" => Some(("build.intotoProvenance.metadata.completeness.materials", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.intoto-provenance.metadata.reproducible" => Some(("build.intotoProvenance.metadata.reproducible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.intoto-provenance.recipe.defined-in-material" => Some(("build.intotoProvenance.recipe.definedInMaterial", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-provenance.recipe.entry-point" => Some(("build.intotoProvenance.recipe.entryPoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-provenance.recipe.type" => Some(("build.intotoProvenance.recipe.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.-type" => Some(("build.intotoStatement._type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.predicate-type" => Some(("build.intotoStatement.predicateType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.provenance.builder-config.id" => Some(("build.intotoStatement.provenance.builderConfig.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.provenance.materials" => Some(("build.intotoStatement.provenance.materials", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "build.intoto-statement.provenance.metadata.build-finished-on" => Some(("build.intotoStatement.provenance.metadata.buildFinishedOn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.provenance.metadata.build-invocation-id" => Some(("build.intotoStatement.provenance.metadata.buildInvocationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.provenance.metadata.build-started-on" => Some(("build.intotoStatement.provenance.metadata.buildStartedOn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.provenance.metadata.completeness.arguments" => Some(("build.intotoStatement.provenance.metadata.completeness.arguments", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.intoto-statement.provenance.metadata.completeness.environment" => Some(("build.intotoStatement.provenance.metadata.completeness.environment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.intoto-statement.provenance.metadata.completeness.materials" => Some(("build.intotoStatement.provenance.metadata.completeness.materials", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.intoto-statement.provenance.metadata.reproducible" => Some(("build.intotoStatement.provenance.metadata.reproducible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.intoto-statement.provenance.recipe.defined-in-material" => Some(("build.intotoStatement.provenance.recipe.definedInMaterial", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.provenance.recipe.entry-point" => Some(("build.intotoStatement.provenance.recipe.entryPoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.provenance.recipe.type" => Some(("build.intotoStatement.provenance.recipe.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance.builder.id" => Some(("build.intotoStatement.slsaProvenance.builder.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance.metadata.build-finished-on" => Some(("build.intotoStatement.slsaProvenance.metadata.buildFinishedOn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance.metadata.build-invocation-id" => Some(("build.intotoStatement.slsaProvenance.metadata.buildInvocationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance.metadata.build-started-on" => Some(("build.intotoStatement.slsaProvenance.metadata.buildStartedOn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance.metadata.completeness.arguments" => Some(("build.intotoStatement.slsaProvenance.metadata.completeness.arguments", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance.metadata.completeness.environment" => Some(("build.intotoStatement.slsaProvenance.metadata.completeness.environment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance.metadata.completeness.materials" => Some(("build.intotoStatement.slsaProvenance.metadata.completeness.materials", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance.metadata.reproducible" => Some(("build.intotoStatement.slsaProvenance.metadata.reproducible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance.recipe.defined-in-material" => Some(("build.intotoStatement.slsaProvenance.recipe.definedInMaterial", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance.recipe.entry-point" => Some(("build.intotoStatement.slsaProvenance.recipe.entryPoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance.recipe.type" => Some(("build.intotoStatement.slsaProvenance.recipe.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance-zero-two.build-type" => Some(("build.intotoStatement.slsaProvenanceZeroTwo.buildType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance-zero-two.builder.id" => Some(("build.intotoStatement.slsaProvenanceZeroTwo.builder.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance-zero-two.invocation.config-source.digest" => Some(("build.intotoStatement.slsaProvenanceZeroTwo.invocation.configSource.digest", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "build.intoto-statement.slsa-provenance-zero-two.invocation.config-source.entry-point" => Some(("build.intotoStatement.slsaProvenanceZeroTwo.invocation.configSource.entryPoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance-zero-two.invocation.config-source.uri" => Some(("build.intotoStatement.slsaProvenanceZeroTwo.invocation.configSource.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance-zero-two.metadata.build-finished-on" => Some(("build.intotoStatement.slsaProvenanceZeroTwo.metadata.buildFinishedOn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance-zero-two.metadata.build-invocation-id" => Some(("build.intotoStatement.slsaProvenanceZeroTwo.metadata.buildInvocationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance-zero-two.metadata.build-started-on" => Some(("build.intotoStatement.slsaProvenanceZeroTwo.metadata.buildStartedOn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance-zero-two.metadata.completeness.environment" => Some(("build.intotoStatement.slsaProvenanceZeroTwo.metadata.completeness.environment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance-zero-two.metadata.completeness.materials" => Some(("build.intotoStatement.slsaProvenanceZeroTwo.metadata.completeness.materials", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance-zero-two.metadata.completeness.parameters" => Some(("build.intotoStatement.slsaProvenanceZeroTwo.metadata.completeness.parameters", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.intoto-statement.slsa-provenance-zero-two.metadata.reproducible" => Some(("build.intotoStatement.slsaProvenanceZeroTwo.metadata.reproducible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "build.provenance.build-options" => Some(("build.provenance.buildOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "build.provenance.builder-version" => Some(("build.provenance.builderVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.create-time" => Some(("build.provenance.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.creator" => Some(("build.provenance.creator", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.end-time" => Some(("build.provenance.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.id" => Some(("build.provenance.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.logs-uri" => Some(("build.provenance.logsUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.project-id" => Some(("build.provenance.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.artifact-storage-source-uri" => Some(("build.provenance.sourceProvenance.artifactStorageSourceUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.cloud-repo.alias-context.kind" => Some(("build.provenance.sourceProvenance.context.cloudRepo.aliasContext.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.cloud-repo.alias-context.name" => Some(("build.provenance.sourceProvenance.context.cloudRepo.aliasContext.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.cloud-repo.repo-id.project-repo-id.project-id" => Some(("build.provenance.sourceProvenance.context.cloudRepo.repoId.projectRepoId.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.cloud-repo.repo-id.project-repo-id.repo-name" => Some(("build.provenance.sourceProvenance.context.cloudRepo.repoId.projectRepoId.repoName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.cloud-repo.repo-id.uid" => Some(("build.provenance.sourceProvenance.context.cloudRepo.repoId.uid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.cloud-repo.revision-id" => Some(("build.provenance.sourceProvenance.context.cloudRepo.revisionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.gerrit.alias-context.kind" => Some(("build.provenance.sourceProvenance.context.gerrit.aliasContext.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.gerrit.alias-context.name" => Some(("build.provenance.sourceProvenance.context.gerrit.aliasContext.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.gerrit.gerrit-project" => Some(("build.provenance.sourceProvenance.context.gerrit.gerritProject", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.gerrit.host-uri" => Some(("build.provenance.sourceProvenance.context.gerrit.hostUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.gerrit.revision-id" => Some(("build.provenance.sourceProvenance.context.gerrit.revisionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.git.revision-id" => Some(("build.provenance.sourceProvenance.context.git.revisionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.git.url" => Some(("build.provenance.sourceProvenance.context.git.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.source-provenance.context.labels" => Some(("build.provenance.sourceProvenance.context.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "build.provenance.start-time" => Some(("build.provenance.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance.trigger-id" => Some(("build.provenance.triggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "build.provenance-bytes" => Some(("build.provenanceBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "compliance.non-compliance-reason" => Some(("compliance.nonComplianceReason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "compliance.version.benchmark-document" => Some(("compliance.version.benchmarkDocument", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "compliance.version.cpe-uri" => Some(("compliance.version.cpeUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "compliance.version.version" => Some(("compliance.version.version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment.address" => Some(("deployment.address", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment.config" => Some(("deployment.config", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment.deploy-time" => Some(("deployment.deployTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment.platform" => Some(("deployment.platform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment.resource-uri" => Some(("deployment.resourceUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "deployment.undeploy-time" => Some(("deployment.undeployTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment.user-email" => Some(("deployment.userEmail", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery.analysis-completed.analysis-type" => Some(("discovery.analysisCompleted.analysisType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "discovery.analysis-status" => Some(("discovery.analysisStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery.analysis-status-error.code" => Some(("discovery.analysisStatusError.code", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "discovery.analysis-status-error.message" => Some(("discovery.analysisStatusError.message", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery.archive-time" => Some(("discovery.archiveTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery.continuous-analysis" => Some(("discovery.continuousAnalysis", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery.cpe" => Some(("discovery.cpe", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery.last-scan-time" => Some(("discovery.lastScanTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery.sbom-status.error" => Some(("discovery.sbomStatus.error", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery.sbom-status.sbom-state" => Some(("discovery.sbomStatus.sbomState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery.vulnerability-attestation.error" => Some(("discovery.vulnerabilityAttestation.error", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery.vulnerability-attestation.last-attempt-time" => Some(("discovery.vulnerabilityAttestation.lastAttemptTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery.vulnerability-attestation.state" => Some(("discovery.vulnerabilityAttestation.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.envelope.payload" => Some(("dsseAttestation.envelope.payload", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.envelope.payload-type" => Some(("dsseAttestation.envelope.payloadType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.-type" => Some(("dsseAttestation.statement._type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.predicate-type" => Some(("dsseAttestation.statement.predicateType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.provenance.builder-config.id" => Some(("dsseAttestation.statement.provenance.builderConfig.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.provenance.materials" => Some(("dsseAttestation.statement.provenance.materials", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "dsse-attestation.statement.provenance.metadata.build-finished-on" => Some(("dsseAttestation.statement.provenance.metadata.buildFinishedOn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.provenance.metadata.build-invocation-id" => Some(("dsseAttestation.statement.provenance.metadata.buildInvocationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.provenance.metadata.build-started-on" => Some(("dsseAttestation.statement.provenance.metadata.buildStartedOn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.provenance.metadata.completeness.arguments" => Some(("dsseAttestation.statement.provenance.metadata.completeness.arguments", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.provenance.metadata.completeness.environment" => Some(("dsseAttestation.statement.provenance.metadata.completeness.environment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.provenance.metadata.completeness.materials" => Some(("dsseAttestation.statement.provenance.metadata.completeness.materials", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.provenance.metadata.reproducible" => Some(("dsseAttestation.statement.provenance.metadata.reproducible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.provenance.recipe.defined-in-material" => Some(("dsseAttestation.statement.provenance.recipe.definedInMaterial", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.provenance.recipe.entry-point" => Some(("dsseAttestation.statement.provenance.recipe.entryPoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.provenance.recipe.type" => Some(("dsseAttestation.statement.provenance.recipe.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance.builder.id" => Some(("dsseAttestation.statement.slsaProvenance.builder.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance.metadata.build-finished-on" => Some(("dsseAttestation.statement.slsaProvenance.metadata.buildFinishedOn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance.metadata.build-invocation-id" => Some(("dsseAttestation.statement.slsaProvenance.metadata.buildInvocationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance.metadata.build-started-on" => Some(("dsseAttestation.statement.slsaProvenance.metadata.buildStartedOn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance.metadata.completeness.arguments" => Some(("dsseAttestation.statement.slsaProvenance.metadata.completeness.arguments", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance.metadata.completeness.environment" => Some(("dsseAttestation.statement.slsaProvenance.metadata.completeness.environment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance.metadata.completeness.materials" => Some(("dsseAttestation.statement.slsaProvenance.metadata.completeness.materials", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance.metadata.reproducible" => Some(("dsseAttestation.statement.slsaProvenance.metadata.reproducible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance.recipe.defined-in-material" => Some(("dsseAttestation.statement.slsaProvenance.recipe.definedInMaterial", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance.recipe.entry-point" => Some(("dsseAttestation.statement.slsaProvenance.recipe.entryPoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance.recipe.type" => Some(("dsseAttestation.statement.slsaProvenance.recipe.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance-zero-two.build-type" => Some(("dsseAttestation.statement.slsaProvenanceZeroTwo.buildType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance-zero-two.builder.id" => Some(("dsseAttestation.statement.slsaProvenanceZeroTwo.builder.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance-zero-two.invocation.config-source.digest" => Some(("dsseAttestation.statement.slsaProvenanceZeroTwo.invocation.configSource.digest", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "dsse-attestation.statement.slsa-provenance-zero-two.invocation.config-source.entry-point" => Some(("dsseAttestation.statement.slsaProvenanceZeroTwo.invocation.configSource.entryPoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance-zero-two.invocation.config-source.uri" => Some(("dsseAttestation.statement.slsaProvenanceZeroTwo.invocation.configSource.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance-zero-two.metadata.build-finished-on" => Some(("dsseAttestation.statement.slsaProvenanceZeroTwo.metadata.buildFinishedOn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance-zero-two.metadata.build-invocation-id" => Some(("dsseAttestation.statement.slsaProvenanceZeroTwo.metadata.buildInvocationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance-zero-two.metadata.build-started-on" => Some(("dsseAttestation.statement.slsaProvenanceZeroTwo.metadata.buildStartedOn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance-zero-two.metadata.completeness.environment" => Some(("dsseAttestation.statement.slsaProvenanceZeroTwo.metadata.completeness.environment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance-zero-two.metadata.completeness.materials" => Some(("dsseAttestation.statement.slsaProvenanceZeroTwo.metadata.completeness.materials", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance-zero-two.metadata.completeness.parameters" => Some(("dsseAttestation.statement.slsaProvenanceZeroTwo.metadata.completeness.parameters", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "dsse-attestation.statement.slsa-provenance-zero-two.metadata.reproducible" => Some(("dsseAttestation.statement.slsaProvenanceZeroTwo.metadata.reproducible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "envelope.payload" => Some(("envelope.payload", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "envelope.payload-type" => Some(("envelope.payloadType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image.base-resource-url" => Some(("image.baseResourceUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image.distance" => Some(("image.distance", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "image.fingerprint.v1-name" => Some(("image.fingerprint.v1Name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image.fingerprint.v2-blob" => Some(("image.fingerprint.v2Blob", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "image.fingerprint.v2-name" => Some(("image.fingerprint.v2Name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "note-name" => Some(("noteName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.architecture" => Some(("package.architecture", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.cpe-uri" => Some(("package.cpeUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.license.comments" => Some(("package.license.comments", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.license.expression" => Some(("package.license.expression", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.name" => Some(("package.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.package-type" => Some(("package.packageType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.version.epoch" => Some(("package.version.epoch", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "package.version.full-name" => Some(("package.version.fullName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.version.inclusive" => Some(("package.version.inclusive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "package.version.kind" => Some(("package.version.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.version.name" => Some(("package.version.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "package.version.revision" => Some(("package.version.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "remediation" => Some(("remediation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "resource-uri" => Some(("resourceUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sbom-reference.payload.-type" => Some(("sbomReference.payload._type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sbom-reference.payload.predicate.digest" => Some(("sbomReference.payload.predicate.digest", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "sbom-reference.payload.predicate.location" => Some(("sbomReference.payload.predicate.location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sbom-reference.payload.predicate.mime-type" => Some(("sbomReference.payload.predicate.mimeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sbom-reference.payload.predicate.referrer-id" => Some(("sbomReference.payload.predicate.referrerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sbom-reference.payload.predicate-type" => Some(("sbomReference.payload.predicateType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sbom-reference.payload-type" => Some(("sbomReference.payloadType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.distribution.classification" => Some(("upgrade.distribution.classification", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.distribution.cpe-uri" => Some(("upgrade.distribution.cpeUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.distribution.cve" => Some(("upgrade.distribution.cve", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "upgrade.distribution.severity" => Some(("upgrade.distribution.severity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.package" => Some(("upgrade.package", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.parsed-version.epoch" => Some(("upgrade.parsedVersion.epoch", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "upgrade.parsed-version.full-name" => Some(("upgrade.parsedVersion.fullName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.parsed-version.inclusive" => Some(("upgrade.parsedVersion.inclusive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "upgrade.parsed-version.kind" => Some(("upgrade.parsedVersion.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.parsed-version.name" => Some(("upgrade.parsedVersion.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.parsed-version.revision" => Some(("upgrade.parsedVersion.revision", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.windows-update.description" => Some(("upgrade.windowsUpdate.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.windows-update.identity.revision" => Some(("upgrade.windowsUpdate.identity.revision", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "upgrade.windows-update.identity.update-id" => Some(("upgrade.windowsUpdate.identity.updateId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.windows-update.kb-article-ids" => Some(("upgrade.windowsUpdate.kbArticleIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "upgrade.windows-update.last-published-timestamp" => Some(("upgrade.windowsUpdate.lastPublishedTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.windows-update.support-url" => Some(("upgrade.windowsUpdate.supportUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade.windows-update.title" => Some(("upgrade.windowsUpdate.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-score" => Some(("vulnerability.cvssScore", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.attack-complexity" => Some(("vulnerability.cvssV2.attackComplexity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.attack-vector" => Some(("vulnerability.cvssV2.attackVector", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.authentication" => Some(("vulnerability.cvssV2.authentication", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.availability-impact" => Some(("vulnerability.cvssV2.availabilityImpact", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.base-score" => Some(("vulnerability.cvssV2.baseScore", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.confidentiality-impact" => Some(("vulnerability.cvssV2.confidentialityImpact", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.exploitability-score" => Some(("vulnerability.cvssV2.exploitabilityScore", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.impact-score" => Some(("vulnerability.cvssV2.impactScore", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.integrity-impact" => Some(("vulnerability.cvssV2.integrityImpact", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.privileges-required" => Some(("vulnerability.cvssV2.privilegesRequired", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.scope" => Some(("vulnerability.cvssV2.scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-v2.user-interaction" => Some(("vulnerability.cvssV2.userInteraction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvss-version" => Some(("vulnerability.cvssVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvssv3.attack-complexity" => Some(("vulnerability.cvssv3.attackComplexity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvssv3.attack-vector" => Some(("vulnerability.cvssv3.attackVector", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvssv3.authentication" => Some(("vulnerability.cvssv3.authentication", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvssv3.availability-impact" => Some(("vulnerability.cvssv3.availabilityImpact", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvssv3.base-score" => Some(("vulnerability.cvssv3.baseScore", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "vulnerability.cvssv3.confidentiality-impact" => Some(("vulnerability.cvssv3.confidentialityImpact", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvssv3.exploitability-score" => Some(("vulnerability.cvssv3.exploitabilityScore", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "vulnerability.cvssv3.impact-score" => Some(("vulnerability.cvssv3.impactScore", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "vulnerability.cvssv3.integrity-impact" => Some(("vulnerability.cvssv3.integrityImpact", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvssv3.privileges-required" => Some(("vulnerability.cvssv3.privilegesRequired", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvssv3.scope" => Some(("vulnerability.cvssv3.scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.cvssv3.user-interaction" => Some(("vulnerability.cvssv3.userInteraction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.effective-severity" => Some(("vulnerability.effectiveSeverity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.extra-details" => Some(("vulnerability.extraDetails", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.fix-available" => Some(("vulnerability.fixAvailable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "vulnerability.long-description" => Some(("vulnerability.longDescription", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.severity" => Some(("vulnerability.severity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.short-description" => Some(("vulnerability.shortDescription", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.type" => Some(("vulnerability.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.vex-assessment.cve" => Some(("vulnerability.vexAssessment.cve", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.vex-assessment.impacts" => Some(("vulnerability.vexAssessment.impacts", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "vulnerability.vex-assessment.justification.details" => Some(("vulnerability.vexAssessment.justification.details", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.vex-assessment.justification.justification-type" => Some(("vulnerability.vexAssessment.justification.justificationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.vex-assessment.note-name" => Some(("vulnerability.vexAssessment.noteName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.vex-assessment.state" => Some(("vulnerability.vexAssessment.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vulnerability.vex-assessment.vulnerability-id" => Some(("vulnerability.vexAssessment.vulnerabilityId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["-type", "address", "alias-context", "analysis-completed", "analysis-status", "analysis-status-error", "analysis-type", "architecture", "archive-time", "arguments", "artifact-storage-source-uri", "attack-complexity", "attack-vector", "attestation", "authentication", "availability-impact", "base-resource-url", "base-score", "benchmark-document", "build", "build-definition", "build-finished-on", "build-invocation-id", "build-options", "build-started-on", "build-type", "builder", "builder-config", "builder-version", "classification", "cloud-repo", "code", "comments", "completeness", "compliance", "confidentiality-impact", "config", "config-source", "context", "continuous-analysis", "cpe", "cpe-uri", "create-time", "creator", "cve", "cvss-score", "cvss-v2", "cvss-version", "cvssv3", "defined-in-material", "deploy-time", "deployment", "description", "details", "digest", "discovery", "distance", "distribution", "dsse-attestation", "effective-severity", "end-time", "entry-point", "envelope", "environment", "epoch", "error", "exploitability-score", "expression", "extra-details", "fingerprint", "finished-on", "fix-available", "full-name", "gerrit", "gerrit-project", "git", "host-uri", "id", "identity", "image", "impact-score", "impacts", "in-toto-slsa-provenance-v1", "inclusive", "integrity-impact", "intoto-provenance", "intoto-statement", "invocation", "invocation-id", "justification", "justification-type", "kb-article-ids", "kind", "labels", "last-attempt-time", "last-published-timestamp", "last-scan-time", "license", "location", "logs-uri", "long-description", "materials", "message", "metadata", "mime-type", "name", "non-compliance-reason", "note-name", "package", "package-type", "parameters", "parsed-version", "payload", "payload-type", "platform", "predicate", "predicate-type", "privileges-required", "project-id", "project-repo-id", "provenance", "provenance-bytes", "recipe", "referrer-id", "remediation", "repo-id", "repo-name", "reproducible", "resource-uri", "revision", "revision-id", "run-details", "sbom-reference", "sbom-state", "sbom-status", "scope", "serialized-payload", "severity", "short-description", "slsa-provenance", "slsa-provenance-zero-two", "source-provenance", "start-time", "started-on", "state", "statement", "support-url", "title", "trigger-id", "type", "uid", "undeploy-time", "update-id", "update-time", "upgrade", "uri", "url", "user-email", "user-interaction", "v1-name", "v2-blob", "v2-name", "version", "vex-assessment", "vulnerability", "vulnerability-attestation", "vulnerability-id", "windows-update"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Occurrence = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().occurrences_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_occurrences_set_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["etag", "policy", "version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SetIamPolicyRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().occurrences_set_iam_policy(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_occurrences_test_iam_permissions(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.projects().occurrences_test_iam_permissions(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_resources_export_sbom(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::ExportSBOMRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().resources_export_sbom(request, opt.value_of("name").unwrap_or(""));
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
                    ("locations-notes-get", Some(opt)) => {
                        call_result = self._projects_locations_notes_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-notes-list", Some(opt)) => {
                        call_result = self._projects_locations_notes_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-notes-occurrences-list", Some(opt)) => {
                        call_result = self._projects_locations_notes_occurrences_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-occurrences-get", Some(opt)) => {
                        call_result = self._projects_locations_occurrences_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-occurrences-get-notes", Some(opt)) => {
                        call_result = self._projects_locations_occurrences_get_notes(opt, dry_run, &mut err).await;
                    },
                    ("locations-occurrences-get-vulnerability-summary", Some(opt)) => {
                        call_result = self._projects_locations_occurrences_get_vulnerability_summary(opt, dry_run, &mut err).await;
                    },
                    ("locations-occurrences-list", Some(opt)) => {
                        call_result = self._projects_locations_occurrences_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-resources-export-sbom", Some(opt)) => {
                        call_result = self._projects_locations_resources_export_sbom(opt, dry_run, &mut err).await;
                    },
                    ("notes-batch-create", Some(opt)) => {
                        call_result = self._projects_notes_batch_create(opt, dry_run, &mut err).await;
                    },
                    ("notes-create", Some(opt)) => {
                        call_result = self._projects_notes_create(opt, dry_run, &mut err).await;
                    },
                    ("notes-delete", Some(opt)) => {
                        call_result = self._projects_notes_delete(opt, dry_run, &mut err).await;
                    },
                    ("notes-get", Some(opt)) => {
                        call_result = self._projects_notes_get(opt, dry_run, &mut err).await;
                    },
                    ("notes-get-iam-policy", Some(opt)) => {
                        call_result = self._projects_notes_get_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("notes-list", Some(opt)) => {
                        call_result = self._projects_notes_list(opt, dry_run, &mut err).await;
                    },
                    ("notes-occurrences-list", Some(opt)) => {
                        call_result = self._projects_notes_occurrences_list(opt, dry_run, &mut err).await;
                    },
                    ("notes-patch", Some(opt)) => {
                        call_result = self._projects_notes_patch(opt, dry_run, &mut err).await;
                    },
                    ("notes-set-iam-policy", Some(opt)) => {
                        call_result = self._projects_notes_set_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("notes-test-iam-permissions", Some(opt)) => {
                        call_result = self._projects_notes_test_iam_permissions(opt, dry_run, &mut err).await;
                    },
                    ("occurrences-batch-create", Some(opt)) => {
                        call_result = self._projects_occurrences_batch_create(opt, dry_run, &mut err).await;
                    },
                    ("occurrences-create", Some(opt)) => {
                        call_result = self._projects_occurrences_create(opt, dry_run, &mut err).await;
                    },
                    ("occurrences-delete", Some(opt)) => {
                        call_result = self._projects_occurrences_delete(opt, dry_run, &mut err).await;
                    },
                    ("occurrences-get", Some(opt)) => {
                        call_result = self._projects_occurrences_get(opt, dry_run, &mut err).await;
                    },
                    ("occurrences-get-iam-policy", Some(opt)) => {
                        call_result = self._projects_occurrences_get_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("occurrences-get-notes", Some(opt)) => {
                        call_result = self._projects_occurrences_get_notes(opt, dry_run, &mut err).await;
                    },
                    ("occurrences-get-vulnerability-summary", Some(opt)) => {
                        call_result = self._projects_occurrences_get_vulnerability_summary(opt, dry_run, &mut err).await;
                    },
                    ("occurrences-list", Some(opt)) => {
                        call_result = self._projects_occurrences_list(opt, dry_run, &mut err).await;
                    },
                    ("occurrences-patch", Some(opt)) => {
                        call_result = self._projects_occurrences_patch(opt, dry_run, &mut err).await;
                    },
                    ("occurrences-set-iam-policy", Some(opt)) => {
                        call_result = self._projects_occurrences_set_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("occurrences-test-iam-permissions", Some(opt)) => {
                        call_result = self._projects_occurrences_test_iam_permissions(opt, dry_run, &mut err).await;
                    },
                    ("resources-export-sbom", Some(opt)) => {
                        call_result = self._projects_resources_export_sbom(opt, dry_run, &mut err).await;
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

            match client::application_secret_from_directory(&config_dir, "containeranalysis1-secret.json",
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
        ).persist_tokens_to_disk(format!("{}/containeranalysis1", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::ContainerAnalysis::new(client, auth),
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
        ("projects", "methods: 'locations-notes-get', 'locations-notes-list', 'locations-notes-occurrences-list', 'locations-occurrences-get', 'locations-occurrences-get-notes', 'locations-occurrences-get-vulnerability-summary', 'locations-occurrences-list', 'locations-resources-export-sbom', 'notes-batch-create', 'notes-create', 'notes-delete', 'notes-get', 'notes-get-iam-policy', 'notes-list', 'notes-occurrences-list', 'notes-patch', 'notes-set-iam-policy', 'notes-test-iam-permissions', 'occurrences-batch-create', 'occurrences-create', 'occurrences-delete', 'occurrences-get', 'occurrences-get-iam-policy', 'occurrences-get-notes', 'occurrences-get-vulnerability-summary', 'occurrences-list', 'occurrences-patch', 'occurrences-set-iam-policy', 'occurrences-test-iam-permissions' and 'resources-export-sbom'", vec![
            ("locations-notes-get",
                    Some(r##"Gets the specified note."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_cli/projects_locations-notes-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the note in the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`."##),
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
            ("locations-notes-list",
                    Some(r##"Lists notes for the specified project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_cli/projects_locations-notes-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the project to list notes for in the form of `projects/[PROJECT_ID]`."##),
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
            ("locations-notes-occurrences-list",
                    Some(r##"Lists occurrences referencing the specified note. Provider projects can use this method to get all occurrences across consumer projects referencing the specified note."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_cli/projects_locations-notes-occurrences-list",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the note to list occurrences for in the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`."##),
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
            ("locations-occurrences-get",
                    Some(r##"Gets the specified occurrence."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_cli/projects_locations-occurrences-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the occurrence in the form of `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]`."##),
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
            ("locations-occurrences-get-notes",
                    Some(r##"Gets the note attached to the specified occurrence. Consumer projects can use this method to get a note that belongs to a provider project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_cli/projects_locations-occurrences-get-notes",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the occurrence in the form of `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]`."##),
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
            ("locations-occurrences-get-vulnerability-summary",
                    Some(r##"Gets a summary of the number and severity of occurrences."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_cli/projects_locations-occurrences-get-vulnerability-summary",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the project to get a vulnerability summary for in the form of `projects/[PROJECT_ID]`."##),
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
            ("locations-occurrences-list",
                    Some(r##"Lists occurrences for the specified project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_cli/projects_locations-occurrences-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the project to list occurrences for in the form of `projects/[PROJECT_ID]`."##),
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
            ("locations-resources-export-sbom",
                    Some(r##"Generates an SBOM for the given resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_cli/projects_locations-resources-export-sbom",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the resource in the form of `projects/[PROJECT_ID]/resources/[RESOURCE_URL]`."##),
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
            ("notes-batch-create",
                    Some(r##"Creates new notes in batch."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_cli/projects_notes-batch-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the project in the form of `projects/[PROJECT_ID]`, under which the notes are to be created."##),
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
            ("notes-create",
                    Some(r##"Creates a new note."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_cli/projects_notes-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the project in the form of `projects/[PROJECT_ID]`, under which the note is to be created."##),
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
            ("notes-delete",
                    Some(r##"Deletes the specified note."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_cli/projects_notes-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the note in the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`."##),
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
            ("notes-get",
                    Some(r##"Gets the specified note."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_cli/projects_notes-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the note in the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`."##),
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
            ("notes-get-iam-policy",
                    Some(r##"Gets the access control policy for a note or an occurrence resource. Requires `containeranalysis.notes.setIamPolicy` or `containeranalysis.occurrences.setIamPolicy` permission if the resource is a note or occurrence, respectively. The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for occurrences."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_cli/projects_notes-get-iam-policy",
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
            ("notes-list",
                    Some(r##"Lists notes for the specified project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_cli/projects_notes-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the project to list notes for in the form of `projects/[PROJECT_ID]`."##),
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
            ("notes-occurrences-list",
                    Some(r##"Lists occurrences referencing the specified note. Provider projects can use this method to get all occurrences across consumer projects referencing the specified note."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_cli/projects_notes-occurrences-list",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the note to list occurrences for in the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`."##),
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
            ("notes-patch",
                    Some(r##"Updates the specified note."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_cli/projects_notes-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the note in the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`."##),
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
            ("notes-set-iam-policy",
                    Some(r##"Sets the access control policy on the specified note or occurrence. Requires `containeranalysis.notes.setIamPolicy` or `containeranalysis.occurrences.setIamPolicy` permission if the resource is a note or an occurrence, respectively. The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for occurrences."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_cli/projects_notes-set-iam-policy",
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
            ("notes-test-iam-permissions",
                    Some(r##"Returns the permissions that a caller has on the specified note or occurrence. Requires list permission on the project (for example, `containeranalysis.notes.list`). The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for occurrences."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_cli/projects_notes-test-iam-permissions",
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
            ("occurrences-batch-create",
                    Some(r##"Creates new occurrences in batch."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_cli/projects_occurrences-batch-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the project in the form of `projects/[PROJECT_ID]`, under which the occurrences are to be created."##),
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
            ("occurrences-create",
                    Some(r##"Creates a new occurrence."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_cli/projects_occurrences-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the project in the form of `projects/[PROJECT_ID]`, under which the occurrence is to be created."##),
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
            ("occurrences-delete",
                    Some(r##"Deletes the specified occurrence. For example, use this method to delete an occurrence when the occurrence is no longer applicable for the given resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_cli/projects_occurrences-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the occurrence in the form of `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]`."##),
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
            ("occurrences-get",
                    Some(r##"Gets the specified occurrence."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_cli/projects_occurrences-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the occurrence in the form of `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]`."##),
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
            ("occurrences-get-iam-policy",
                    Some(r##"Gets the access control policy for a note or an occurrence resource. Requires `containeranalysis.notes.setIamPolicy` or `containeranalysis.occurrences.setIamPolicy` permission if the resource is a note or occurrence, respectively. The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for occurrences."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_cli/projects_occurrences-get-iam-policy",
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
            ("occurrences-get-notes",
                    Some(r##"Gets the note attached to the specified occurrence. Consumer projects can use this method to get a note that belongs to a provider project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_cli/projects_occurrences-get-notes",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the occurrence in the form of `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]`."##),
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
            ("occurrences-get-vulnerability-summary",
                    Some(r##"Gets a summary of the number and severity of occurrences."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_cli/projects_occurrences-get-vulnerability-summary",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the project to get a vulnerability summary for in the form of `projects/[PROJECT_ID]`."##),
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
            ("occurrences-list",
                    Some(r##"Lists occurrences for the specified project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_cli/projects_occurrences-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the project to list occurrences for in the form of `projects/[PROJECT_ID]`."##),
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
            ("occurrences-patch",
                    Some(r##"Updates the specified occurrence."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_cli/projects_occurrences-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the occurrence in the form of `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]`."##),
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
            ("occurrences-set-iam-policy",
                    Some(r##"Sets the access control policy on the specified note or occurrence. Requires `containeranalysis.notes.setIamPolicy` or `containeranalysis.occurrences.setIamPolicy` permission if the resource is a note or an occurrence, respectively. The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for occurrences."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_cli/projects_occurrences-set-iam-policy",
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
            ("occurrences-test-iam-permissions",
                    Some(r##"Returns the permissions that a caller has on the specified note or occurrence. Requires list permission on the project (for example, `containeranalysis.notes.list`). The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for occurrences."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_cli/projects_occurrences-test-iam-permissions",
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
            ("resources-export-sbom",
                    Some(r##"Generates an SBOM for the given resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_containeranalysis1_cli/projects_resources-export-sbom",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the resource in the form of `projects/[PROJECT_ID]/resources/[RESOURCE_URL]`."##),
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
    
    let mut app = App::new("containeranalysis1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+20240625")
           .about("This API is a prerequisite for leveraging Artifact Analysis scanning capabilities in both Artifact Registry and with Advanced Vulnerability Insights (runtime scanning) in GKE. In addition, the Container Analysis API is an implementation of the Grafeas API, which enables storing, querying, and retrieval of critical metadata about all of your software artifacts.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_containeranalysis1_cli")
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
