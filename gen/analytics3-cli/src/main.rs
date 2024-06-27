// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_analytics3::{api, Error, oauth2, client::chrono, FieldMask};


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
    hub: api::Analytics<S>,
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
    async fn _data_ga_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.data().ga_get(opt.value_of("ids").unwrap_or(""), opt.value_of("start-date").unwrap_or(""), opt.value_of("end-date").unwrap_or(""), opt.value_of("metrics").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(        value.map(|v| arg_from_str(v, err, "start-index", "int32")).unwrap_or(-0));
                },
                "sort" => {
                    call = call.sort(value.unwrap_or(""));
                },
                "segment" => {
                    call = call.segment(value.unwrap_or(""));
                },
                "sampling-level" => {
                    call = call.sampling_level(value.unwrap_or(""));
                },
                "output" => {
                    call = call.output(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "int32")).unwrap_or(-0));
                },
                "include-empty-rows" => {
                    call = call.include_empty_rows(        value.map(|v| arg_from_str(v, err, "include-empty-rows", "boolean")).unwrap_or(false));
                },
                "filters" => {
                    call = call.filters(value.unwrap_or(""));
                },
                "dimensions" => {
                    call = call.dimensions(value.unwrap_or(""));
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
                                                                           v.extend(["dimensions", "filters", "include-empty-rows", "max-results", "output", "sampling-level", "segment", "sort", "start-index"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _data_mcf_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.data().mcf_get(opt.value_of("ids").unwrap_or(""), opt.value_of("start-date").unwrap_or(""), opt.value_of("end-date").unwrap_or(""), opt.value_of("metrics").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(        value.map(|v| arg_from_str(v, err, "start-index", "int32")).unwrap_or(-0));
                },
                "sort" => {
                    call = call.sort(value.unwrap_or(""));
                },
                "sampling-level" => {
                    call = call.sampling_level(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "int32")).unwrap_or(-0));
                },
                "filters" => {
                    call = call.filters(value.unwrap_or(""));
                },
                "dimensions" => {
                    call = call.dimensions(value.unwrap_or(""));
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
                                                                           v.extend(["dimensions", "filters", "max-results", "sampling-level", "sort", "start-index"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _data_realtime_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.data().realtime_get(opt.value_of("ids").unwrap_or(""), opt.value_of("metrics").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "sort" => {
                    call = call.sort(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "int32")).unwrap_or(-0));
                },
                "filters" => {
                    call = call.filters(value.unwrap_or(""));
                },
                "dimensions" => {
                    call = call.dimensions(value.unwrap_or(""));
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
                                                                           v.extend(["dimensions", "filters", "max-results", "sort"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _management_account_summaries_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().account_summaries_list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(        value.map(|v| arg_from_str(v, err, "start-index", "int32")).unwrap_or(-0));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["max-results", "start-index"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _management_account_user_links_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().account_user_links_delete(opt.value_of("account-id").unwrap_or(""), opt.value_of("link-id").unwrap_or(""));
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

    async fn _management_account_user_links_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "entity.account-ref.href" => Some(("entity.accountRef.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.account-ref.id" => Some(("entity.accountRef.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.account-ref.kind" => Some(("entity.accountRef.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.account-ref.name" => Some(("entity.accountRef.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.account-id" => Some(("entity.profileRef.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.href" => Some(("entity.profileRef.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.id" => Some(("entity.profileRef.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.internal-web-property-id" => Some(("entity.profileRef.internalWebPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.kind" => Some(("entity.profileRef.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.name" => Some(("entity.profileRef.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.web-property-id" => Some(("entity.profileRef.webPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.account-id" => Some(("entity.webPropertyRef.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.href" => Some(("entity.webPropertyRef.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.id" => Some(("entity.webPropertyRef.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.internal-web-property-id" => Some(("entity.webPropertyRef.internalWebPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.kind" => Some(("entity.webPropertyRef.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.name" => Some(("entity.webPropertyRef.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "permissions.effective" => Some(("permissions.effective", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "permissions.local" => Some(("permissions.local", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-ref.email" => Some(("userRef.email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-ref.id" => Some(("userRef.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-ref.kind" => Some(("userRef.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "account-ref", "effective", "email", "entity", "href", "id", "internal-web-property-id", "kind", "local", "name", "permissions", "profile-ref", "self-link", "user-ref", "web-property-id", "web-property-ref"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::EntityUserLink = json::value::from_value(object).unwrap();
        let mut call = self.hub.management().account_user_links_insert(request, opt.value_of("account-id").unwrap_or(""));
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

    async fn _management_account_user_links_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().account_user_links_list(opt.value_of("account-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(        value.map(|v| arg_from_str(v, err, "start-index", "int32")).unwrap_or(-0));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["max-results", "start-index"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _management_account_user_links_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "entity.account-ref.href" => Some(("entity.accountRef.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.account-ref.id" => Some(("entity.accountRef.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.account-ref.kind" => Some(("entity.accountRef.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.account-ref.name" => Some(("entity.accountRef.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.account-id" => Some(("entity.profileRef.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.href" => Some(("entity.profileRef.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.id" => Some(("entity.profileRef.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.internal-web-property-id" => Some(("entity.profileRef.internalWebPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.kind" => Some(("entity.profileRef.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.name" => Some(("entity.profileRef.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.web-property-id" => Some(("entity.profileRef.webPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.account-id" => Some(("entity.webPropertyRef.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.href" => Some(("entity.webPropertyRef.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.id" => Some(("entity.webPropertyRef.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.internal-web-property-id" => Some(("entity.webPropertyRef.internalWebPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.kind" => Some(("entity.webPropertyRef.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.name" => Some(("entity.webPropertyRef.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "permissions.effective" => Some(("permissions.effective", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "permissions.local" => Some(("permissions.local", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-ref.email" => Some(("userRef.email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-ref.id" => Some(("userRef.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-ref.kind" => Some(("userRef.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "account-ref", "effective", "email", "entity", "href", "id", "internal-web-property-id", "kind", "local", "name", "permissions", "profile-ref", "self-link", "user-ref", "web-property-id", "web-property-ref"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::EntityUserLink = json::value::from_value(object).unwrap();
        let mut call = self.hub.management().account_user_links_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("link-id").unwrap_or(""));
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

    async fn _management_accounts_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().accounts_list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(        value.map(|v| arg_from_str(v, err, "start-index", "int32")).unwrap_or(-0));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["max-results", "start-index"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _management_client_id_hash_client_id(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "client-id" => Some(("clientId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-property-id" => Some(("webPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["client-id", "kind", "web-property-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::HashClientIdRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.management().client_id_hash_client_id(request);
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

    async fn _management_custom_data_sources_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().custom_data_sources_list(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(        value.map(|v| arg_from_str(v, err, "start-index", "int32")).unwrap_or(-0));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["max-results", "start-index"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _management_custom_dimensions_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().custom_dimensions_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("custom-dimension-id").unwrap_or(""));
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

    async fn _management_custom_dimensions_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "account-id" => Some(("accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active" => Some(("active", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "created" => Some(("created", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "index" => Some(("index", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.href" => Some(("parentLink.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.type" => Some(("parentLink.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "scope" => Some(("scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "updated" => Some(("updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-property-id" => Some(("webPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "active", "created", "href", "id", "index", "kind", "name", "parent-link", "scope", "self-link", "type", "updated", "web-property-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CustomDimension = json::value::from_value(object).unwrap();
        let mut call = self.hub.management().custom_dimensions_insert(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""));
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

    async fn _management_custom_dimensions_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().custom_dimensions_list(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(        value.map(|v| arg_from_str(v, err, "start-index", "int32")).unwrap_or(-0));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["max-results", "start-index"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _management_custom_dimensions_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "account-id" => Some(("accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active" => Some(("active", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "created" => Some(("created", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "index" => Some(("index", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.href" => Some(("parentLink.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.type" => Some(("parentLink.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "scope" => Some(("scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "updated" => Some(("updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-property-id" => Some(("webPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "active", "created", "href", "id", "index", "kind", "name", "parent-link", "scope", "self-link", "type", "updated", "web-property-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CustomDimension = json::value::from_value(object).unwrap();
        let mut call = self.hub.management().custom_dimensions_patch(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("custom-dimension-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "ignore-custom-data-source-links" => {
                    call = call.ignore_custom_data_source_links(        value.map(|v| arg_from_str(v, err, "ignore-custom-data-source-links", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["ignore-custom-data-source-links"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _management_custom_dimensions_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "account-id" => Some(("accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active" => Some(("active", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "created" => Some(("created", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "index" => Some(("index", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.href" => Some(("parentLink.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.type" => Some(("parentLink.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "scope" => Some(("scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "updated" => Some(("updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-property-id" => Some(("webPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "active", "created", "href", "id", "index", "kind", "name", "parent-link", "scope", "self-link", "type", "updated", "web-property-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CustomDimension = json::value::from_value(object).unwrap();
        let mut call = self.hub.management().custom_dimensions_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("custom-dimension-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "ignore-custom-data-source-links" => {
                    call = call.ignore_custom_data_source_links(        value.map(|v| arg_from_str(v, err, "ignore-custom-data-source-links", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["ignore-custom-data-source-links"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _management_custom_metrics_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().custom_metrics_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("custom-metric-id").unwrap_or(""));
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

    async fn _management_custom_metrics_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "account-id" => Some(("accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active" => Some(("active", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "created" => Some(("created", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "index" => Some(("index", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "max-value" => Some(("max_value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "min-value" => Some(("min_value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.href" => Some(("parentLink.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.type" => Some(("parentLink.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "scope" => Some(("scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "updated" => Some(("updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-property-id" => Some(("webPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "active", "created", "href", "id", "index", "kind", "max-value", "min-value", "name", "parent-link", "scope", "self-link", "type", "updated", "web-property-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CustomMetric = json::value::from_value(object).unwrap();
        let mut call = self.hub.management().custom_metrics_insert(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""));
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

    async fn _management_custom_metrics_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().custom_metrics_list(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(        value.map(|v| arg_from_str(v, err, "start-index", "int32")).unwrap_or(-0));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["max-results", "start-index"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _management_custom_metrics_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "account-id" => Some(("accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active" => Some(("active", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "created" => Some(("created", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "index" => Some(("index", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "max-value" => Some(("max_value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "min-value" => Some(("min_value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.href" => Some(("parentLink.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.type" => Some(("parentLink.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "scope" => Some(("scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "updated" => Some(("updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-property-id" => Some(("webPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "active", "created", "href", "id", "index", "kind", "max-value", "min-value", "name", "parent-link", "scope", "self-link", "type", "updated", "web-property-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CustomMetric = json::value::from_value(object).unwrap();
        let mut call = self.hub.management().custom_metrics_patch(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("custom-metric-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "ignore-custom-data-source-links" => {
                    call = call.ignore_custom_data_source_links(        value.map(|v| arg_from_str(v, err, "ignore-custom-data-source-links", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["ignore-custom-data-source-links"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _management_custom_metrics_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "account-id" => Some(("accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active" => Some(("active", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "created" => Some(("created", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "index" => Some(("index", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "max-value" => Some(("max_value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "min-value" => Some(("min_value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.href" => Some(("parentLink.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.type" => Some(("parentLink.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "scope" => Some(("scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "updated" => Some(("updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-property-id" => Some(("webPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "active", "created", "href", "id", "index", "kind", "max-value", "min-value", "name", "parent-link", "scope", "self-link", "type", "updated", "web-property-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CustomMetric = json::value::from_value(object).unwrap();
        let mut call = self.hub.management().custom_metrics_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("custom-metric-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "ignore-custom-data-source-links" => {
                    call = call.ignore_custom_data_source_links(        value.map(|v| arg_from_str(v, err, "ignore-custom-data-source-links", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["ignore-custom-data-source-links"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _management_experiments_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().experiments_delete(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""), opt.value_of("experiment-id").unwrap_or(""));
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

    async fn _management_experiments_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().experiments_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""), opt.value_of("experiment-id").unwrap_or(""));
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

    async fn _management_experiments_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "account-id" => Some(("accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created" => Some(("created", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "editable-in-ga-ui" => Some(("editableInGaUi", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "end-time" => Some(("endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "equal-weighting" => Some(("equalWeighting", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "internal-web-property-id" => Some(("internalWebPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "minimum-experiment-length-in-days" => Some(("minimumExperimentLengthInDays", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "objective-metric" => Some(("objectiveMetric", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "optimization-type" => Some(("optimizationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.href" => Some(("parentLink.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.type" => Some(("parentLink.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile-id" => Some(("profileId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "reason-experiment-ended" => Some(("reasonExperimentEnded", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rewrite-variation-urls-as-original" => Some(("rewriteVariationUrlsAsOriginal", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "serving-framework" => Some(("servingFramework", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet" => Some(("snippet", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "start-time" => Some(("startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status" => Some(("status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "traffic-coverage" => Some(("trafficCoverage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "updated" => Some(("updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-property-id" => Some(("webPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "winner-confidence-level" => Some(("winnerConfidenceLevel", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "winner-found" => Some(("winnerFound", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "created", "description", "editable-in-ga-ui", "end-time", "equal-weighting", "href", "id", "internal-web-property-id", "kind", "minimum-experiment-length-in-days", "name", "objective-metric", "optimization-type", "parent-link", "profile-id", "reason-experiment-ended", "rewrite-variation-urls-as-original", "self-link", "serving-framework", "snippet", "start-time", "status", "traffic-coverage", "type", "updated", "web-property-id", "winner-confidence-level", "winner-found"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Experiment = json::value::from_value(object).unwrap();
        let mut call = self.hub.management().experiments_insert(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""));
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

    async fn _management_experiments_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().experiments_list(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(        value.map(|v| arg_from_str(v, err, "start-index", "int32")).unwrap_or(-0));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["max-results", "start-index"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _management_experiments_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "account-id" => Some(("accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created" => Some(("created", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "editable-in-ga-ui" => Some(("editableInGaUi", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "end-time" => Some(("endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "equal-weighting" => Some(("equalWeighting", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "internal-web-property-id" => Some(("internalWebPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "minimum-experiment-length-in-days" => Some(("minimumExperimentLengthInDays", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "objective-metric" => Some(("objectiveMetric", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "optimization-type" => Some(("optimizationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.href" => Some(("parentLink.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.type" => Some(("parentLink.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile-id" => Some(("profileId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "reason-experiment-ended" => Some(("reasonExperimentEnded", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rewrite-variation-urls-as-original" => Some(("rewriteVariationUrlsAsOriginal", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "serving-framework" => Some(("servingFramework", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet" => Some(("snippet", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "start-time" => Some(("startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status" => Some(("status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "traffic-coverage" => Some(("trafficCoverage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "updated" => Some(("updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-property-id" => Some(("webPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "winner-confidence-level" => Some(("winnerConfidenceLevel", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "winner-found" => Some(("winnerFound", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "created", "description", "editable-in-ga-ui", "end-time", "equal-weighting", "href", "id", "internal-web-property-id", "kind", "minimum-experiment-length-in-days", "name", "objective-metric", "optimization-type", "parent-link", "profile-id", "reason-experiment-ended", "rewrite-variation-urls-as-original", "self-link", "serving-framework", "snippet", "start-time", "status", "traffic-coverage", "type", "updated", "web-property-id", "winner-confidence-level", "winner-found"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Experiment = json::value::from_value(object).unwrap();
        let mut call = self.hub.management().experiments_patch(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""), opt.value_of("experiment-id").unwrap_or(""));
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

    async fn _management_experiments_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "account-id" => Some(("accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created" => Some(("created", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "editable-in-ga-ui" => Some(("editableInGaUi", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "end-time" => Some(("endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "equal-weighting" => Some(("equalWeighting", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "internal-web-property-id" => Some(("internalWebPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "minimum-experiment-length-in-days" => Some(("minimumExperimentLengthInDays", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "objective-metric" => Some(("objectiveMetric", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "optimization-type" => Some(("optimizationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.href" => Some(("parentLink.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.type" => Some(("parentLink.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile-id" => Some(("profileId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "reason-experiment-ended" => Some(("reasonExperimentEnded", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rewrite-variation-urls-as-original" => Some(("rewriteVariationUrlsAsOriginal", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "serving-framework" => Some(("servingFramework", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "snippet" => Some(("snippet", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "start-time" => Some(("startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status" => Some(("status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "traffic-coverage" => Some(("trafficCoverage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "updated" => Some(("updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-property-id" => Some(("webPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "winner-confidence-level" => Some(("winnerConfidenceLevel", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "winner-found" => Some(("winnerFound", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "created", "description", "editable-in-ga-ui", "end-time", "equal-weighting", "href", "id", "internal-web-property-id", "kind", "minimum-experiment-length-in-days", "name", "objective-metric", "optimization-type", "parent-link", "profile-id", "reason-experiment-ended", "rewrite-variation-urls-as-original", "self-link", "serving-framework", "snippet", "start-time", "status", "traffic-coverage", "type", "updated", "web-property-id", "winner-confidence-level", "winner-found"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Experiment = json::value::from_value(object).unwrap();
        let mut call = self.hub.management().experiments_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""), opt.value_of("experiment-id").unwrap_or(""));
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

    async fn _management_filters_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().filters_delete(opt.value_of("account-id").unwrap_or(""), opt.value_of("filter-id").unwrap_or(""));
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

    async fn _management_filters_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().filters_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("filter-id").unwrap_or(""));
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

    async fn _management_filters_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "account-id" => Some(("accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-details.case-sensitive" => Some(("advancedDetails.caseSensitive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "advanced-details.extract-a" => Some(("advancedDetails.extractA", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-details.extract-b" => Some(("advancedDetails.extractB", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-details.field-a" => Some(("advancedDetails.fieldA", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-details.field-a-index" => Some(("advancedDetails.fieldAIndex", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "advanced-details.field-a-required" => Some(("advancedDetails.fieldARequired", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "advanced-details.field-b" => Some(("advancedDetails.fieldB", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-details.field-b-index" => Some(("advancedDetails.fieldBIndex", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "advanced-details.field-b-required" => Some(("advancedDetails.fieldBRequired", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "advanced-details.output-constructor" => Some(("advancedDetails.outputConstructor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-details.output-to-field" => Some(("advancedDetails.outputToField", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-details.output-to-field-index" => Some(("advancedDetails.outputToFieldIndex", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "advanced-details.override-output-field" => Some(("advancedDetails.overrideOutputField", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "created" => Some(("created", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "exclude-details.case-sensitive" => Some(("excludeDetails.caseSensitive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "exclude-details.expression-value" => Some(("excludeDetails.expressionValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "exclude-details.field" => Some(("excludeDetails.field", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "exclude-details.field-index" => Some(("excludeDetails.fieldIndex", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "exclude-details.kind" => Some(("excludeDetails.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "exclude-details.match-type" => Some(("excludeDetails.matchType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "include-details.case-sensitive" => Some(("includeDetails.caseSensitive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "include-details.expression-value" => Some(("includeDetails.expressionValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "include-details.field" => Some(("includeDetails.field", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "include-details.field-index" => Some(("includeDetails.fieldIndex", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "include-details.kind" => Some(("includeDetails.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "include-details.match-type" => Some(("includeDetails.matchType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lowercase-details.field" => Some(("lowercaseDetails.field", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lowercase-details.field-index" => Some(("lowercaseDetails.fieldIndex", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.href" => Some(("parentLink.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.type" => Some(("parentLink.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "search-and-replace-details.case-sensitive" => Some(("searchAndReplaceDetails.caseSensitive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "search-and-replace-details.field" => Some(("searchAndReplaceDetails.field", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "search-and-replace-details.field-index" => Some(("searchAndReplaceDetails.fieldIndex", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "search-and-replace-details.replace-string" => Some(("searchAndReplaceDetails.replaceString", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "search-and-replace-details.search-string" => Some(("searchAndReplaceDetails.searchString", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "updated" => Some(("updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uppercase-details.field" => Some(("uppercaseDetails.field", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uppercase-details.field-index" => Some(("uppercaseDetails.fieldIndex", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "advanced-details", "case-sensitive", "created", "exclude-details", "expression-value", "extract-a", "extract-b", "field", "field-a", "field-a-index", "field-a-required", "field-b", "field-b-index", "field-b-required", "field-index", "href", "id", "include-details", "kind", "lowercase-details", "match-type", "name", "output-constructor", "output-to-field", "output-to-field-index", "override-output-field", "parent-link", "replace-string", "search-and-replace-details", "search-string", "self-link", "type", "updated", "uppercase-details"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Filter = json::value::from_value(object).unwrap();
        let mut call = self.hub.management().filters_insert(request, opt.value_of("account-id").unwrap_or(""));
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

    async fn _management_filters_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().filters_list(opt.value_of("account-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(        value.map(|v| arg_from_str(v, err, "start-index", "int32")).unwrap_or(-0));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["max-results", "start-index"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _management_filters_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "account-id" => Some(("accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-details.case-sensitive" => Some(("advancedDetails.caseSensitive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "advanced-details.extract-a" => Some(("advancedDetails.extractA", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-details.extract-b" => Some(("advancedDetails.extractB", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-details.field-a" => Some(("advancedDetails.fieldA", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-details.field-a-index" => Some(("advancedDetails.fieldAIndex", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "advanced-details.field-a-required" => Some(("advancedDetails.fieldARequired", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "advanced-details.field-b" => Some(("advancedDetails.fieldB", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-details.field-b-index" => Some(("advancedDetails.fieldBIndex", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "advanced-details.field-b-required" => Some(("advancedDetails.fieldBRequired", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "advanced-details.output-constructor" => Some(("advancedDetails.outputConstructor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-details.output-to-field" => Some(("advancedDetails.outputToField", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-details.output-to-field-index" => Some(("advancedDetails.outputToFieldIndex", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "advanced-details.override-output-field" => Some(("advancedDetails.overrideOutputField", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "created" => Some(("created", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "exclude-details.case-sensitive" => Some(("excludeDetails.caseSensitive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "exclude-details.expression-value" => Some(("excludeDetails.expressionValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "exclude-details.field" => Some(("excludeDetails.field", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "exclude-details.field-index" => Some(("excludeDetails.fieldIndex", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "exclude-details.kind" => Some(("excludeDetails.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "exclude-details.match-type" => Some(("excludeDetails.matchType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "include-details.case-sensitive" => Some(("includeDetails.caseSensitive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "include-details.expression-value" => Some(("includeDetails.expressionValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "include-details.field" => Some(("includeDetails.field", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "include-details.field-index" => Some(("includeDetails.fieldIndex", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "include-details.kind" => Some(("includeDetails.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "include-details.match-type" => Some(("includeDetails.matchType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lowercase-details.field" => Some(("lowercaseDetails.field", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lowercase-details.field-index" => Some(("lowercaseDetails.fieldIndex", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.href" => Some(("parentLink.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.type" => Some(("parentLink.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "search-and-replace-details.case-sensitive" => Some(("searchAndReplaceDetails.caseSensitive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "search-and-replace-details.field" => Some(("searchAndReplaceDetails.field", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "search-and-replace-details.field-index" => Some(("searchAndReplaceDetails.fieldIndex", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "search-and-replace-details.replace-string" => Some(("searchAndReplaceDetails.replaceString", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "search-and-replace-details.search-string" => Some(("searchAndReplaceDetails.searchString", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "updated" => Some(("updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uppercase-details.field" => Some(("uppercaseDetails.field", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uppercase-details.field-index" => Some(("uppercaseDetails.fieldIndex", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "advanced-details", "case-sensitive", "created", "exclude-details", "expression-value", "extract-a", "extract-b", "field", "field-a", "field-a-index", "field-a-required", "field-b", "field-b-index", "field-b-required", "field-index", "href", "id", "include-details", "kind", "lowercase-details", "match-type", "name", "output-constructor", "output-to-field", "output-to-field-index", "override-output-field", "parent-link", "replace-string", "search-and-replace-details", "search-string", "self-link", "type", "updated", "uppercase-details"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Filter = json::value::from_value(object).unwrap();
        let mut call = self.hub.management().filters_patch(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("filter-id").unwrap_or(""));
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

    async fn _management_filters_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "account-id" => Some(("accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-details.case-sensitive" => Some(("advancedDetails.caseSensitive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "advanced-details.extract-a" => Some(("advancedDetails.extractA", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-details.extract-b" => Some(("advancedDetails.extractB", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-details.field-a" => Some(("advancedDetails.fieldA", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-details.field-a-index" => Some(("advancedDetails.fieldAIndex", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "advanced-details.field-a-required" => Some(("advancedDetails.fieldARequired", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "advanced-details.field-b" => Some(("advancedDetails.fieldB", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-details.field-b-index" => Some(("advancedDetails.fieldBIndex", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "advanced-details.field-b-required" => Some(("advancedDetails.fieldBRequired", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "advanced-details.output-constructor" => Some(("advancedDetails.outputConstructor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-details.output-to-field" => Some(("advancedDetails.outputToField", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advanced-details.output-to-field-index" => Some(("advancedDetails.outputToFieldIndex", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "advanced-details.override-output-field" => Some(("advancedDetails.overrideOutputField", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "created" => Some(("created", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "exclude-details.case-sensitive" => Some(("excludeDetails.caseSensitive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "exclude-details.expression-value" => Some(("excludeDetails.expressionValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "exclude-details.field" => Some(("excludeDetails.field", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "exclude-details.field-index" => Some(("excludeDetails.fieldIndex", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "exclude-details.kind" => Some(("excludeDetails.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "exclude-details.match-type" => Some(("excludeDetails.matchType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "include-details.case-sensitive" => Some(("includeDetails.caseSensitive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "include-details.expression-value" => Some(("includeDetails.expressionValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "include-details.field" => Some(("includeDetails.field", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "include-details.field-index" => Some(("includeDetails.fieldIndex", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "include-details.kind" => Some(("includeDetails.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "include-details.match-type" => Some(("includeDetails.matchType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lowercase-details.field" => Some(("lowercaseDetails.field", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "lowercase-details.field-index" => Some(("lowercaseDetails.fieldIndex", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.href" => Some(("parentLink.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.type" => Some(("parentLink.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "search-and-replace-details.case-sensitive" => Some(("searchAndReplaceDetails.caseSensitive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "search-and-replace-details.field" => Some(("searchAndReplaceDetails.field", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "search-and-replace-details.field-index" => Some(("searchAndReplaceDetails.fieldIndex", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "search-and-replace-details.replace-string" => Some(("searchAndReplaceDetails.replaceString", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "search-and-replace-details.search-string" => Some(("searchAndReplaceDetails.searchString", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "updated" => Some(("updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uppercase-details.field" => Some(("uppercaseDetails.field", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uppercase-details.field-index" => Some(("uppercaseDetails.fieldIndex", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "advanced-details", "case-sensitive", "created", "exclude-details", "expression-value", "extract-a", "extract-b", "field", "field-a", "field-a-index", "field-a-required", "field-b", "field-b-index", "field-b-required", "field-index", "href", "id", "include-details", "kind", "lowercase-details", "match-type", "name", "output-constructor", "output-to-field", "output-to-field-index", "override-output-field", "parent-link", "replace-string", "search-and-replace-details", "search-string", "self-link", "type", "updated", "uppercase-details"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Filter = json::value::from_value(object).unwrap();
        let mut call = self.hub.management().filters_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("filter-id").unwrap_or(""));
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

    async fn _management_goals_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().goals_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""), opt.value_of("goal-id").unwrap_or(""));
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

    async fn _management_goals_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "account-id" => Some(("accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active" => Some(("active", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "created" => Some(("created", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event-details.use-event-value" => Some(("eventDetails.useEventValue", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "internal-web-property-id" => Some(("internalWebPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.href" => Some(("parentLink.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.type" => Some(("parentLink.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile-id" => Some(("profileId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "updated" => Some(("updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "url-destination-details.case-sensitive" => Some(("urlDestinationDetails.caseSensitive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "url-destination-details.first-step-required" => Some(("urlDestinationDetails.firstStepRequired", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "url-destination-details.match-type" => Some(("urlDestinationDetails.matchType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "url-destination-details.url" => Some(("urlDestinationDetails.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "value" => Some(("value", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "visit-num-pages-details.comparison-type" => Some(("visitNumPagesDetails.comparisonType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visit-num-pages-details.comparison-value" => Some(("visitNumPagesDetails.comparisonValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visit-time-on-site-details.comparison-type" => Some(("visitTimeOnSiteDetails.comparisonType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visit-time-on-site-details.comparison-value" => Some(("visitTimeOnSiteDetails.comparisonValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-property-id" => Some(("webPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "active", "case-sensitive", "comparison-type", "comparison-value", "created", "event-details", "first-step-required", "href", "id", "internal-web-property-id", "kind", "match-type", "name", "parent-link", "profile-id", "self-link", "type", "updated", "url", "url-destination-details", "use-event-value", "value", "visit-num-pages-details", "visit-time-on-site-details", "web-property-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Goal = json::value::from_value(object).unwrap();
        let mut call = self.hub.management().goals_insert(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""));
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

    async fn _management_goals_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().goals_list(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(        value.map(|v| arg_from_str(v, err, "start-index", "int32")).unwrap_or(-0));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["max-results", "start-index"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _management_goals_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "account-id" => Some(("accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active" => Some(("active", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "created" => Some(("created", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event-details.use-event-value" => Some(("eventDetails.useEventValue", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "internal-web-property-id" => Some(("internalWebPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.href" => Some(("parentLink.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.type" => Some(("parentLink.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile-id" => Some(("profileId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "updated" => Some(("updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "url-destination-details.case-sensitive" => Some(("urlDestinationDetails.caseSensitive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "url-destination-details.first-step-required" => Some(("urlDestinationDetails.firstStepRequired", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "url-destination-details.match-type" => Some(("urlDestinationDetails.matchType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "url-destination-details.url" => Some(("urlDestinationDetails.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "value" => Some(("value", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "visit-num-pages-details.comparison-type" => Some(("visitNumPagesDetails.comparisonType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visit-num-pages-details.comparison-value" => Some(("visitNumPagesDetails.comparisonValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visit-time-on-site-details.comparison-type" => Some(("visitTimeOnSiteDetails.comparisonType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visit-time-on-site-details.comparison-value" => Some(("visitTimeOnSiteDetails.comparisonValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-property-id" => Some(("webPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "active", "case-sensitive", "comparison-type", "comparison-value", "created", "event-details", "first-step-required", "href", "id", "internal-web-property-id", "kind", "match-type", "name", "parent-link", "profile-id", "self-link", "type", "updated", "url", "url-destination-details", "use-event-value", "value", "visit-num-pages-details", "visit-time-on-site-details", "web-property-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Goal = json::value::from_value(object).unwrap();
        let mut call = self.hub.management().goals_patch(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""), opt.value_of("goal-id").unwrap_or(""));
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

    async fn _management_goals_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "account-id" => Some(("accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active" => Some(("active", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "created" => Some(("created", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event-details.use-event-value" => Some(("eventDetails.useEventValue", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "internal-web-property-id" => Some(("internalWebPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.href" => Some(("parentLink.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.type" => Some(("parentLink.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile-id" => Some(("profileId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "updated" => Some(("updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "url-destination-details.case-sensitive" => Some(("urlDestinationDetails.caseSensitive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "url-destination-details.first-step-required" => Some(("urlDestinationDetails.firstStepRequired", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "url-destination-details.match-type" => Some(("urlDestinationDetails.matchType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "url-destination-details.url" => Some(("urlDestinationDetails.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "value" => Some(("value", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "visit-num-pages-details.comparison-type" => Some(("visitNumPagesDetails.comparisonType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visit-num-pages-details.comparison-value" => Some(("visitNumPagesDetails.comparisonValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visit-time-on-site-details.comparison-type" => Some(("visitTimeOnSiteDetails.comparisonType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visit-time-on-site-details.comparison-value" => Some(("visitTimeOnSiteDetails.comparisonValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-property-id" => Some(("webPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "active", "case-sensitive", "comparison-type", "comparison-value", "created", "event-details", "first-step-required", "href", "id", "internal-web-property-id", "kind", "match-type", "name", "parent-link", "profile-id", "self-link", "type", "updated", "url", "url-destination-details", "use-event-value", "value", "visit-num-pages-details", "visit-time-on-site-details", "web-property-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Goal = json::value::from_value(object).unwrap();
        let mut call = self.hub.management().goals_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""), opt.value_of("goal-id").unwrap_or(""));
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

    async fn _management_profile_filter_links_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().profile_filter_links_delete(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""), opt.value_of("link-id").unwrap_or(""));
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

    async fn _management_profile_filter_links_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().profile_filter_links_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""), opt.value_of("link-id").unwrap_or(""));
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

    async fn _management_profile_filter_links_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "filter-ref.account-id" => Some(("filterRef.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "filter-ref.href" => Some(("filterRef.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "filter-ref.id" => Some(("filterRef.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "filter-ref.kind" => Some(("filterRef.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "filter-ref.name" => Some(("filterRef.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile-ref.account-id" => Some(("profileRef.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile-ref.href" => Some(("profileRef.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile-ref.id" => Some(("profileRef.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile-ref.internal-web-property-id" => Some(("profileRef.internalWebPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile-ref.kind" => Some(("profileRef.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile-ref.name" => Some(("profileRef.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile-ref.web-property-id" => Some(("profileRef.webPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rank" => Some(("rank", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "filter-ref", "href", "id", "internal-web-property-id", "kind", "name", "profile-ref", "rank", "self-link", "web-property-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ProfileFilterLink = json::value::from_value(object).unwrap();
        let mut call = self.hub.management().profile_filter_links_insert(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""));
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

    async fn _management_profile_filter_links_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().profile_filter_links_list(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(        value.map(|v| arg_from_str(v, err, "start-index", "int32")).unwrap_or(-0));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["max-results", "start-index"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _management_profile_filter_links_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "filter-ref.account-id" => Some(("filterRef.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "filter-ref.href" => Some(("filterRef.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "filter-ref.id" => Some(("filterRef.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "filter-ref.kind" => Some(("filterRef.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "filter-ref.name" => Some(("filterRef.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile-ref.account-id" => Some(("profileRef.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile-ref.href" => Some(("profileRef.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile-ref.id" => Some(("profileRef.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile-ref.internal-web-property-id" => Some(("profileRef.internalWebPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile-ref.kind" => Some(("profileRef.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile-ref.name" => Some(("profileRef.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile-ref.web-property-id" => Some(("profileRef.webPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rank" => Some(("rank", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "filter-ref", "href", "id", "internal-web-property-id", "kind", "name", "profile-ref", "rank", "self-link", "web-property-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ProfileFilterLink = json::value::from_value(object).unwrap();
        let mut call = self.hub.management().profile_filter_links_patch(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""), opt.value_of("link-id").unwrap_or(""));
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

    async fn _management_profile_filter_links_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "filter-ref.account-id" => Some(("filterRef.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "filter-ref.href" => Some(("filterRef.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "filter-ref.id" => Some(("filterRef.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "filter-ref.kind" => Some(("filterRef.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "filter-ref.name" => Some(("filterRef.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile-ref.account-id" => Some(("profileRef.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile-ref.href" => Some(("profileRef.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile-ref.id" => Some(("profileRef.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile-ref.internal-web-property-id" => Some(("profileRef.internalWebPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile-ref.kind" => Some(("profileRef.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile-ref.name" => Some(("profileRef.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile-ref.web-property-id" => Some(("profileRef.webPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rank" => Some(("rank", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "filter-ref", "href", "id", "internal-web-property-id", "kind", "name", "profile-ref", "rank", "self-link", "web-property-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ProfileFilterLink = json::value::from_value(object).unwrap();
        let mut call = self.hub.management().profile_filter_links_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""), opt.value_of("link-id").unwrap_or(""));
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

    async fn _management_profile_user_links_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().profile_user_links_delete(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""), opt.value_of("link-id").unwrap_or(""));
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

    async fn _management_profile_user_links_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "entity.account-ref.href" => Some(("entity.accountRef.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.account-ref.id" => Some(("entity.accountRef.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.account-ref.kind" => Some(("entity.accountRef.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.account-ref.name" => Some(("entity.accountRef.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.account-id" => Some(("entity.profileRef.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.href" => Some(("entity.profileRef.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.id" => Some(("entity.profileRef.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.internal-web-property-id" => Some(("entity.profileRef.internalWebPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.kind" => Some(("entity.profileRef.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.name" => Some(("entity.profileRef.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.web-property-id" => Some(("entity.profileRef.webPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.account-id" => Some(("entity.webPropertyRef.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.href" => Some(("entity.webPropertyRef.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.id" => Some(("entity.webPropertyRef.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.internal-web-property-id" => Some(("entity.webPropertyRef.internalWebPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.kind" => Some(("entity.webPropertyRef.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.name" => Some(("entity.webPropertyRef.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "permissions.effective" => Some(("permissions.effective", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "permissions.local" => Some(("permissions.local", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-ref.email" => Some(("userRef.email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-ref.id" => Some(("userRef.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-ref.kind" => Some(("userRef.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "account-ref", "effective", "email", "entity", "href", "id", "internal-web-property-id", "kind", "local", "name", "permissions", "profile-ref", "self-link", "user-ref", "web-property-id", "web-property-ref"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::EntityUserLink = json::value::from_value(object).unwrap();
        let mut call = self.hub.management().profile_user_links_insert(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""));
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

    async fn _management_profile_user_links_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().profile_user_links_list(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(        value.map(|v| arg_from_str(v, err, "start-index", "int32")).unwrap_or(-0));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["max-results", "start-index"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _management_profile_user_links_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "entity.account-ref.href" => Some(("entity.accountRef.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.account-ref.id" => Some(("entity.accountRef.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.account-ref.kind" => Some(("entity.accountRef.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.account-ref.name" => Some(("entity.accountRef.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.account-id" => Some(("entity.profileRef.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.href" => Some(("entity.profileRef.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.id" => Some(("entity.profileRef.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.internal-web-property-id" => Some(("entity.profileRef.internalWebPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.kind" => Some(("entity.profileRef.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.name" => Some(("entity.profileRef.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.web-property-id" => Some(("entity.profileRef.webPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.account-id" => Some(("entity.webPropertyRef.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.href" => Some(("entity.webPropertyRef.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.id" => Some(("entity.webPropertyRef.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.internal-web-property-id" => Some(("entity.webPropertyRef.internalWebPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.kind" => Some(("entity.webPropertyRef.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.name" => Some(("entity.webPropertyRef.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "permissions.effective" => Some(("permissions.effective", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "permissions.local" => Some(("permissions.local", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-ref.email" => Some(("userRef.email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-ref.id" => Some(("userRef.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-ref.kind" => Some(("userRef.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "account-ref", "effective", "email", "entity", "href", "id", "internal-web-property-id", "kind", "local", "name", "permissions", "profile-ref", "self-link", "user-ref", "web-property-id", "web-property-ref"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::EntityUserLink = json::value::from_value(object).unwrap();
        let mut call = self.hub.management().profile_user_links_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""), opt.value_of("link-id").unwrap_or(""));
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

    async fn _management_profiles_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().profiles_delete(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""));
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

    async fn _management_profiles_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().profiles_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""));
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

    async fn _management_profiles_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "account-id" => Some(("accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bot-filtering-enabled" => Some(("botFilteringEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "child-link.href" => Some(("childLink.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "child-link.type" => Some(("childLink.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created" => Some(("created", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "currency" => Some(("currency", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-page" => Some(("defaultPage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "e-commerce-tracking" => Some(("eCommerceTracking", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "enhanced-e-commerce-tracking" => Some(("enhancedECommerceTracking", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "exclude-query-parameters" => Some(("excludeQueryParameters", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "internal-web-property-id" => Some(("internalWebPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.href" => Some(("parentLink.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.type" => Some(("parentLink.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "permissions.effective" => Some(("permissions.effective", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "site-search-category-parameters" => Some(("siteSearchCategoryParameters", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "site-search-query-parameters" => Some(("siteSearchQueryParameters", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "starred" => Some(("starred", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "strip-site-search-category-parameters" => Some(("stripSiteSearchCategoryParameters", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "strip-site-search-query-parameters" => Some(("stripSiteSearchQueryParameters", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "timezone" => Some(("timezone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "updated" => Some(("updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-property-id" => Some(("webPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "website-url" => Some(("websiteUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "bot-filtering-enabled", "child-link", "created", "currency", "default-page", "e-commerce-tracking", "effective", "enhanced-e-commerce-tracking", "exclude-query-parameters", "href", "id", "internal-web-property-id", "kind", "name", "parent-link", "permissions", "self-link", "site-search-category-parameters", "site-search-query-parameters", "starred", "strip-site-search-category-parameters", "strip-site-search-query-parameters", "timezone", "type", "updated", "web-property-id", "website-url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Profile = json::value::from_value(object).unwrap();
        let mut call = self.hub.management().profiles_insert(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""));
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

    async fn _management_profiles_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().profiles_list(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(        value.map(|v| arg_from_str(v, err, "start-index", "int32")).unwrap_or(-0));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["max-results", "start-index"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _management_profiles_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "account-id" => Some(("accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bot-filtering-enabled" => Some(("botFilteringEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "child-link.href" => Some(("childLink.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "child-link.type" => Some(("childLink.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created" => Some(("created", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "currency" => Some(("currency", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-page" => Some(("defaultPage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "e-commerce-tracking" => Some(("eCommerceTracking", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "enhanced-e-commerce-tracking" => Some(("enhancedECommerceTracking", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "exclude-query-parameters" => Some(("excludeQueryParameters", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "internal-web-property-id" => Some(("internalWebPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.href" => Some(("parentLink.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.type" => Some(("parentLink.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "permissions.effective" => Some(("permissions.effective", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "site-search-category-parameters" => Some(("siteSearchCategoryParameters", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "site-search-query-parameters" => Some(("siteSearchQueryParameters", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "starred" => Some(("starred", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "strip-site-search-category-parameters" => Some(("stripSiteSearchCategoryParameters", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "strip-site-search-query-parameters" => Some(("stripSiteSearchQueryParameters", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "timezone" => Some(("timezone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "updated" => Some(("updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-property-id" => Some(("webPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "website-url" => Some(("websiteUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "bot-filtering-enabled", "child-link", "created", "currency", "default-page", "e-commerce-tracking", "effective", "enhanced-e-commerce-tracking", "exclude-query-parameters", "href", "id", "internal-web-property-id", "kind", "name", "parent-link", "permissions", "self-link", "site-search-category-parameters", "site-search-query-parameters", "starred", "strip-site-search-category-parameters", "strip-site-search-query-parameters", "timezone", "type", "updated", "web-property-id", "website-url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Profile = json::value::from_value(object).unwrap();
        let mut call = self.hub.management().profiles_patch(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""));
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

    async fn _management_profiles_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "account-id" => Some(("accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "bot-filtering-enabled" => Some(("botFilteringEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "child-link.href" => Some(("childLink.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "child-link.type" => Some(("childLink.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created" => Some(("created", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "currency" => Some(("currency", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-page" => Some(("defaultPage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "e-commerce-tracking" => Some(("eCommerceTracking", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "enhanced-e-commerce-tracking" => Some(("enhancedECommerceTracking", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "exclude-query-parameters" => Some(("excludeQueryParameters", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "internal-web-property-id" => Some(("internalWebPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.href" => Some(("parentLink.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.type" => Some(("parentLink.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "permissions.effective" => Some(("permissions.effective", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "site-search-category-parameters" => Some(("siteSearchCategoryParameters", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "site-search-query-parameters" => Some(("siteSearchQueryParameters", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "starred" => Some(("starred", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "strip-site-search-category-parameters" => Some(("stripSiteSearchCategoryParameters", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "strip-site-search-query-parameters" => Some(("stripSiteSearchQueryParameters", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "timezone" => Some(("timezone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "updated" => Some(("updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-property-id" => Some(("webPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "website-url" => Some(("websiteUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "bot-filtering-enabled", "child-link", "created", "currency", "default-page", "e-commerce-tracking", "effective", "enhanced-e-commerce-tracking", "exclude-query-parameters", "href", "id", "internal-web-property-id", "kind", "name", "parent-link", "permissions", "self-link", "site-search-category-parameters", "site-search-query-parameters", "starred", "strip-site-search-category-parameters", "strip-site-search-query-parameters", "timezone", "type", "updated", "web-property-id", "website-url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Profile = json::value::from_value(object).unwrap();
        let mut call = self.hub.management().profiles_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""));
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

    async fn _management_remarketing_audience_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().remarketing_audience_delete(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("remarketing-audience-id").unwrap_or(""));
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

    async fn _management_remarketing_audience_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().remarketing_audience_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("remarketing-audience-id").unwrap_or(""));
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

    async fn _management_remarketing_audience_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "account-id" => Some(("accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "audience-definition.include-conditions.days-to-look-back" => Some(("audienceDefinition.includeConditions.daysToLookBack", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "audience-definition.include-conditions.is-smart-list" => Some(("audienceDefinition.includeConditions.isSmartList", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "audience-definition.include-conditions.kind" => Some(("audienceDefinition.includeConditions.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "audience-definition.include-conditions.membership-duration-days" => Some(("audienceDefinition.includeConditions.membershipDurationDays", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "audience-definition.include-conditions.segment" => Some(("audienceDefinition.includeConditions.segment", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "audience-type" => Some(("audienceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created" => Some(("created", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "internal-web-property-id" => Some(("internalWebPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "linked-views" => Some(("linkedViews", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state-based-audience-definition.exclude-conditions.exclusion-duration" => Some(("stateBasedAudienceDefinition.excludeConditions.exclusionDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state-based-audience-definition.exclude-conditions.segment" => Some(("stateBasedAudienceDefinition.excludeConditions.segment", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state-based-audience-definition.include-conditions.days-to-look-back" => Some(("stateBasedAudienceDefinition.includeConditions.daysToLookBack", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "state-based-audience-definition.include-conditions.is-smart-list" => Some(("stateBasedAudienceDefinition.includeConditions.isSmartList", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "state-based-audience-definition.include-conditions.kind" => Some(("stateBasedAudienceDefinition.includeConditions.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state-based-audience-definition.include-conditions.membership-duration-days" => Some(("stateBasedAudienceDefinition.includeConditions.membershipDurationDays", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "state-based-audience-definition.include-conditions.segment" => Some(("stateBasedAudienceDefinition.includeConditions.segment", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "updated" => Some(("updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-property-id" => Some(("webPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "audience-definition", "audience-type", "created", "days-to-look-back", "description", "exclude-conditions", "exclusion-duration", "id", "include-conditions", "internal-web-property-id", "is-smart-list", "kind", "linked-views", "membership-duration-days", "name", "segment", "state-based-audience-definition", "updated", "web-property-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::RemarketingAudience = json::value::from_value(object).unwrap();
        let mut call = self.hub.management().remarketing_audience_insert(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""));
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

    async fn _management_remarketing_audience_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().remarketing_audience_list(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "type" => {
                    call = call.type_(value.unwrap_or(""));
                },
                "start-index" => {
                    call = call.start_index(        value.map(|v| arg_from_str(v, err, "start-index", "int32")).unwrap_or(-0));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["max-results", "start-index", "type"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _management_remarketing_audience_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "account-id" => Some(("accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "audience-definition.include-conditions.days-to-look-back" => Some(("audienceDefinition.includeConditions.daysToLookBack", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "audience-definition.include-conditions.is-smart-list" => Some(("audienceDefinition.includeConditions.isSmartList", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "audience-definition.include-conditions.kind" => Some(("audienceDefinition.includeConditions.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "audience-definition.include-conditions.membership-duration-days" => Some(("audienceDefinition.includeConditions.membershipDurationDays", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "audience-definition.include-conditions.segment" => Some(("audienceDefinition.includeConditions.segment", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "audience-type" => Some(("audienceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created" => Some(("created", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "internal-web-property-id" => Some(("internalWebPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "linked-views" => Some(("linkedViews", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state-based-audience-definition.exclude-conditions.exclusion-duration" => Some(("stateBasedAudienceDefinition.excludeConditions.exclusionDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state-based-audience-definition.exclude-conditions.segment" => Some(("stateBasedAudienceDefinition.excludeConditions.segment", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state-based-audience-definition.include-conditions.days-to-look-back" => Some(("stateBasedAudienceDefinition.includeConditions.daysToLookBack", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "state-based-audience-definition.include-conditions.is-smart-list" => Some(("stateBasedAudienceDefinition.includeConditions.isSmartList", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "state-based-audience-definition.include-conditions.kind" => Some(("stateBasedAudienceDefinition.includeConditions.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state-based-audience-definition.include-conditions.membership-duration-days" => Some(("stateBasedAudienceDefinition.includeConditions.membershipDurationDays", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "state-based-audience-definition.include-conditions.segment" => Some(("stateBasedAudienceDefinition.includeConditions.segment", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "updated" => Some(("updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-property-id" => Some(("webPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "audience-definition", "audience-type", "created", "days-to-look-back", "description", "exclude-conditions", "exclusion-duration", "id", "include-conditions", "internal-web-property-id", "is-smart-list", "kind", "linked-views", "membership-duration-days", "name", "segment", "state-based-audience-definition", "updated", "web-property-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::RemarketingAudience = json::value::from_value(object).unwrap();
        let mut call = self.hub.management().remarketing_audience_patch(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("remarketing-audience-id").unwrap_or(""));
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

    async fn _management_remarketing_audience_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "account-id" => Some(("accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "audience-definition.include-conditions.days-to-look-back" => Some(("audienceDefinition.includeConditions.daysToLookBack", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "audience-definition.include-conditions.is-smart-list" => Some(("audienceDefinition.includeConditions.isSmartList", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "audience-definition.include-conditions.kind" => Some(("audienceDefinition.includeConditions.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "audience-definition.include-conditions.membership-duration-days" => Some(("audienceDefinition.includeConditions.membershipDurationDays", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "audience-definition.include-conditions.segment" => Some(("audienceDefinition.includeConditions.segment", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "audience-type" => Some(("audienceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created" => Some(("created", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "internal-web-property-id" => Some(("internalWebPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "linked-views" => Some(("linkedViews", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state-based-audience-definition.exclude-conditions.exclusion-duration" => Some(("stateBasedAudienceDefinition.excludeConditions.exclusionDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state-based-audience-definition.exclude-conditions.segment" => Some(("stateBasedAudienceDefinition.excludeConditions.segment", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state-based-audience-definition.include-conditions.days-to-look-back" => Some(("stateBasedAudienceDefinition.includeConditions.daysToLookBack", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "state-based-audience-definition.include-conditions.is-smart-list" => Some(("stateBasedAudienceDefinition.includeConditions.isSmartList", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "state-based-audience-definition.include-conditions.kind" => Some(("stateBasedAudienceDefinition.includeConditions.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state-based-audience-definition.include-conditions.membership-duration-days" => Some(("stateBasedAudienceDefinition.includeConditions.membershipDurationDays", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "state-based-audience-definition.include-conditions.segment" => Some(("stateBasedAudienceDefinition.includeConditions.segment", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "updated" => Some(("updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-property-id" => Some(("webPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "audience-definition", "audience-type", "created", "days-to-look-back", "description", "exclude-conditions", "exclusion-duration", "id", "include-conditions", "internal-web-property-id", "is-smart-list", "kind", "linked-views", "membership-duration-days", "name", "segment", "state-based-audience-definition", "updated", "web-property-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::RemarketingAudience = json::value::from_value(object).unwrap();
        let mut call = self.hub.management().remarketing_audience_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("remarketing-audience-id").unwrap_or(""));
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

    async fn _management_segments_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().segments_list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(        value.map(|v| arg_from_str(v, err, "start-index", "int32")).unwrap_or(-0));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["max-results", "start-index"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _management_unsampled_reports_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().unsampled_reports_delete(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""), opt.value_of("unsampled-report-id").unwrap_or(""));
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

    async fn _management_unsampled_reports_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().unsampled_reports_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""), opt.value_of("unsampled-report-id").unwrap_or(""));
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

    async fn _management_unsampled_reports_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "account-id" => Some(("accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cloud-storage-download-details.bucket-id" => Some(("cloudStorageDownloadDetails.bucketId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cloud-storage-download-details.object-id" => Some(("cloudStorageDownloadDetails.objectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created" => Some(("created", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dimensions" => Some(("dimensions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "download-type" => Some(("downloadType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "drive-download-details.document-id" => Some(("driveDownloadDetails.documentId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "end-date" => Some(("end-date", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "filters" => Some(("filters", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metrics" => Some(("metrics", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile-id" => Some(("profileId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "segment" => Some(("segment", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "start-date" => Some(("start-date", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "status" => Some(("status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "title" => Some(("title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "updated" => Some(("updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-property-id" => Some(("webPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "bucket-id", "cloud-storage-download-details", "created", "dimensions", "document-id", "download-type", "drive-download-details", "end-date", "filters", "id", "kind", "metrics", "object-id", "profile-id", "segment", "self-link", "start-date", "status", "title", "updated", "web-property-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::UnsampledReport = json::value::from_value(object).unwrap();
        let mut call = self.hub.management().unsampled_reports_insert(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""));
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

    async fn _management_unsampled_reports_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().unsampled_reports_list(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(        value.map(|v| arg_from_str(v, err, "start-index", "int32")).unwrap_or(-0));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["max-results", "start-index"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _management_uploads_delete_upload_data(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "custom-data-import-uids" => Some(("customDataImportUids", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["custom-data-import-uids"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::AnalyticsDataimportDeleteUploadDataRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.management().uploads_delete_upload_data(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("custom-data-source-id").unwrap_or(""));
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

    async fn _management_uploads_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().uploads_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("custom-data-source-id").unwrap_or(""), opt.value_of("upload-id").unwrap_or(""));
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

    async fn _management_uploads_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().uploads_list(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("custom-data-source-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(        value.map(|v| arg_from_str(v, err, "start-index", "int32")).unwrap_or(-0));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["max-results", "start-index"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _management_uploads_upload_data(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().uploads_upload_data(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("custom-data-source-id").unwrap_or(""));
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

    async fn _management_web_property_ad_words_links_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().web_property_ad_words_links_delete(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("web-property-ad-words-link-id").unwrap_or(""));
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

    async fn _management_web_property_ad_words_links_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().web_property_ad_words_links_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("web-property-ad-words-link-id").unwrap_or(""));
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

    async fn _management_web_property_ad_words_links_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "entity.web-property-ref.account-id" => Some(("entity.webPropertyRef.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.href" => Some(("entity.webPropertyRef.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.id" => Some(("entity.webPropertyRef.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.internal-web-property-id" => Some(("entity.webPropertyRef.internalWebPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.kind" => Some(("entity.webPropertyRef.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.name" => Some(("entity.webPropertyRef.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile-ids" => Some(("profileIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "entity", "href", "id", "internal-web-property-id", "kind", "name", "profile-ids", "self-link", "web-property-ref"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::EntityAdWordsLink = json::value::from_value(object).unwrap();
        let mut call = self.hub.management().web_property_ad_words_links_insert(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""));
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

    async fn _management_web_property_ad_words_links_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().web_property_ad_words_links_list(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(        value.map(|v| arg_from_str(v, err, "start-index", "int32")).unwrap_or(-0));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["max-results", "start-index"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _management_web_property_ad_words_links_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "entity.web-property-ref.account-id" => Some(("entity.webPropertyRef.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.href" => Some(("entity.webPropertyRef.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.id" => Some(("entity.webPropertyRef.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.internal-web-property-id" => Some(("entity.webPropertyRef.internalWebPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.kind" => Some(("entity.webPropertyRef.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.name" => Some(("entity.webPropertyRef.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile-ids" => Some(("profileIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "entity", "href", "id", "internal-web-property-id", "kind", "name", "profile-ids", "self-link", "web-property-ref"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::EntityAdWordsLink = json::value::from_value(object).unwrap();
        let mut call = self.hub.management().web_property_ad_words_links_patch(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("web-property-ad-words-link-id").unwrap_or(""));
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

    async fn _management_web_property_ad_words_links_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "entity.web-property-ref.account-id" => Some(("entity.webPropertyRef.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.href" => Some(("entity.webPropertyRef.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.id" => Some(("entity.webPropertyRef.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.internal-web-property-id" => Some(("entity.webPropertyRef.internalWebPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.kind" => Some(("entity.webPropertyRef.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.name" => Some(("entity.webPropertyRef.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile-ids" => Some(("profileIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "entity", "href", "id", "internal-web-property-id", "kind", "name", "profile-ids", "self-link", "web-property-ref"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::EntityAdWordsLink = json::value::from_value(object).unwrap();
        let mut call = self.hub.management().web_property_ad_words_links_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("web-property-ad-words-link-id").unwrap_or(""));
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

    async fn _management_webproperties_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().webproperties_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""));
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

    async fn _management_webproperties_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "account-id" => Some(("accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "child-link.href" => Some(("childLink.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "child-link.type" => Some(("childLink.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created" => Some(("created", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "data-retention-reset-on-new-activity" => Some(("dataRetentionResetOnNewActivity", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "data-retention-ttl" => Some(("dataRetentionTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-profile-id" => Some(("defaultProfileId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "industry-vertical" => Some(("industryVertical", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "internal-web-property-id" => Some(("internalWebPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "level" => Some(("level", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.href" => Some(("parentLink.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.type" => Some(("parentLink.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "permissions.effective" => Some(("permissions.effective", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "profile-count" => Some(("profileCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "starred" => Some(("starred", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "updated" => Some(("updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "website-url" => Some(("websiteUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "child-link", "created", "data-retention-reset-on-new-activity", "data-retention-ttl", "default-profile-id", "effective", "href", "id", "industry-vertical", "internal-web-property-id", "kind", "level", "name", "parent-link", "permissions", "profile-count", "self-link", "starred", "type", "updated", "website-url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Webproperty = json::value::from_value(object).unwrap();
        let mut call = self.hub.management().webproperties_insert(request, opt.value_of("account-id").unwrap_or(""));
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

    async fn _management_webproperties_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().webproperties_list(opt.value_of("account-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(        value.map(|v| arg_from_str(v, err, "start-index", "int32")).unwrap_or(-0));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["max-results", "start-index"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _management_webproperties_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "account-id" => Some(("accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "child-link.href" => Some(("childLink.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "child-link.type" => Some(("childLink.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created" => Some(("created", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "data-retention-reset-on-new-activity" => Some(("dataRetentionResetOnNewActivity", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "data-retention-ttl" => Some(("dataRetentionTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-profile-id" => Some(("defaultProfileId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "industry-vertical" => Some(("industryVertical", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "internal-web-property-id" => Some(("internalWebPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "level" => Some(("level", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.href" => Some(("parentLink.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.type" => Some(("parentLink.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "permissions.effective" => Some(("permissions.effective", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "profile-count" => Some(("profileCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "starred" => Some(("starred", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "updated" => Some(("updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "website-url" => Some(("websiteUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "child-link", "created", "data-retention-reset-on-new-activity", "data-retention-ttl", "default-profile-id", "effective", "href", "id", "industry-vertical", "internal-web-property-id", "kind", "level", "name", "parent-link", "permissions", "profile-count", "self-link", "starred", "type", "updated", "website-url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Webproperty = json::value::from_value(object).unwrap();
        let mut call = self.hub.management().webproperties_patch(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""));
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

    async fn _management_webproperties_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "account-id" => Some(("accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "child-link.href" => Some(("childLink.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "child-link.type" => Some(("childLink.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created" => Some(("created", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "data-retention-reset-on-new-activity" => Some(("dataRetentionResetOnNewActivity", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "data-retention-ttl" => Some(("dataRetentionTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-profile-id" => Some(("defaultProfileId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "industry-vertical" => Some(("industryVertical", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "internal-web-property-id" => Some(("internalWebPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "level" => Some(("level", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.href" => Some(("parentLink.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-link.type" => Some(("parentLink.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "permissions.effective" => Some(("permissions.effective", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "profile-count" => Some(("profileCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "starred" => Some(("starred", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "updated" => Some(("updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "website-url" => Some(("websiteUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "child-link", "created", "data-retention-reset-on-new-activity", "data-retention-ttl", "default-profile-id", "effective", "href", "id", "industry-vertical", "internal-web-property-id", "kind", "level", "name", "parent-link", "permissions", "profile-count", "self-link", "starred", "type", "updated", "website-url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Webproperty = json::value::from_value(object).unwrap();
        let mut call = self.hub.management().webproperties_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""));
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

    async fn _management_webproperty_user_links_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().webproperty_user_links_delete(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("link-id").unwrap_or(""));
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

    async fn _management_webproperty_user_links_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "entity.account-ref.href" => Some(("entity.accountRef.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.account-ref.id" => Some(("entity.accountRef.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.account-ref.kind" => Some(("entity.accountRef.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.account-ref.name" => Some(("entity.accountRef.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.account-id" => Some(("entity.profileRef.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.href" => Some(("entity.profileRef.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.id" => Some(("entity.profileRef.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.internal-web-property-id" => Some(("entity.profileRef.internalWebPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.kind" => Some(("entity.profileRef.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.name" => Some(("entity.profileRef.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.web-property-id" => Some(("entity.profileRef.webPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.account-id" => Some(("entity.webPropertyRef.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.href" => Some(("entity.webPropertyRef.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.id" => Some(("entity.webPropertyRef.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.internal-web-property-id" => Some(("entity.webPropertyRef.internalWebPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.kind" => Some(("entity.webPropertyRef.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.name" => Some(("entity.webPropertyRef.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "permissions.effective" => Some(("permissions.effective", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "permissions.local" => Some(("permissions.local", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-ref.email" => Some(("userRef.email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-ref.id" => Some(("userRef.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-ref.kind" => Some(("userRef.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "account-ref", "effective", "email", "entity", "href", "id", "internal-web-property-id", "kind", "local", "name", "permissions", "profile-ref", "self-link", "user-ref", "web-property-id", "web-property-ref"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::EntityUserLink = json::value::from_value(object).unwrap();
        let mut call = self.hub.management().webproperty_user_links_insert(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""));
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

    async fn _management_webproperty_user_links_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().webproperty_user_links_list(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(        value.map(|v| arg_from_str(v, err, "start-index", "int32")).unwrap_or(-0));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["max-results", "start-index"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
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

    async fn _management_webproperty_user_links_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "entity.account-ref.href" => Some(("entity.accountRef.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.account-ref.id" => Some(("entity.accountRef.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.account-ref.kind" => Some(("entity.accountRef.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.account-ref.name" => Some(("entity.accountRef.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.account-id" => Some(("entity.profileRef.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.href" => Some(("entity.profileRef.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.id" => Some(("entity.profileRef.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.internal-web-property-id" => Some(("entity.profileRef.internalWebPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.kind" => Some(("entity.profileRef.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.name" => Some(("entity.profileRef.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.profile-ref.web-property-id" => Some(("entity.profileRef.webPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.account-id" => Some(("entity.webPropertyRef.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.href" => Some(("entity.webPropertyRef.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.id" => Some(("entity.webPropertyRef.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.internal-web-property-id" => Some(("entity.webPropertyRef.internalWebPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.kind" => Some(("entity.webPropertyRef.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entity.web-property-ref.name" => Some(("entity.webPropertyRef.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "permissions.effective" => Some(("permissions.effective", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "permissions.local" => Some(("permissions.local", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-ref.email" => Some(("userRef.email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-ref.id" => Some(("userRef.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "user-ref.kind" => Some(("userRef.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "account-ref", "effective", "email", "entity", "href", "id", "internal-web-property-id", "kind", "local", "name", "permissions", "profile-ref", "self-link", "user-ref", "web-property-id", "web-property-ref"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::EntityUserLink = json::value::from_value(object).unwrap();
        let mut call = self.hub.management().webproperty_user_links_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("link-id").unwrap_or(""));
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

    async fn _metadata_columns_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.metadata().columns_list(opt.value_of("report-type").unwrap_or(""));
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

    async fn _provisioning_create_account_ticket(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "account.child-link.href" => Some(("account.childLink.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "account.child-link.type" => Some(("account.childLink.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "account.created" => Some(("account.created", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "account.id" => Some(("account.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "account.kind" => Some(("account.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "account.name" => Some(("account.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "account.permissions.effective" => Some(("account.permissions.effective", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "account.self-link" => Some(("account.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "account.starred" => Some(("account.starred", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "account.updated" => Some(("account.updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile.account-id" => Some(("profile.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile.bot-filtering-enabled" => Some(("profile.botFilteringEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "profile.child-link.href" => Some(("profile.childLink.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile.child-link.type" => Some(("profile.childLink.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile.created" => Some(("profile.created", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile.currency" => Some(("profile.currency", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile.default-page" => Some(("profile.defaultPage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile.e-commerce-tracking" => Some(("profile.eCommerceTracking", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "profile.enhanced-e-commerce-tracking" => Some(("profile.enhancedECommerceTracking", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "profile.exclude-query-parameters" => Some(("profile.excludeQueryParameters", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile.id" => Some(("profile.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile.internal-web-property-id" => Some(("profile.internalWebPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile.kind" => Some(("profile.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile.name" => Some(("profile.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile.parent-link.href" => Some(("profile.parentLink.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile.parent-link.type" => Some(("profile.parentLink.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile.permissions.effective" => Some(("profile.permissions.effective", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "profile.self-link" => Some(("profile.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile.site-search-category-parameters" => Some(("profile.siteSearchCategoryParameters", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile.site-search-query-parameters" => Some(("profile.siteSearchQueryParameters", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile.starred" => Some(("profile.starred", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "profile.strip-site-search-category-parameters" => Some(("profile.stripSiteSearchCategoryParameters", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "profile.strip-site-search-query-parameters" => Some(("profile.stripSiteSearchQueryParameters", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "profile.timezone" => Some(("profile.timezone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile.type" => Some(("profile.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile.updated" => Some(("profile.updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile.web-property-id" => Some(("profile.webPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile.website-url" => Some(("profile.websiteUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "redirect-uri" => Some(("redirectUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "webproperty.account-id" => Some(("webproperty.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "webproperty.child-link.href" => Some(("webproperty.childLink.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "webproperty.child-link.type" => Some(("webproperty.childLink.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "webproperty.created" => Some(("webproperty.created", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "webproperty.data-retention-reset-on-new-activity" => Some(("webproperty.dataRetentionResetOnNewActivity", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "webproperty.data-retention-ttl" => Some(("webproperty.dataRetentionTtl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "webproperty.default-profile-id" => Some(("webproperty.defaultProfileId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "webproperty.id" => Some(("webproperty.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "webproperty.industry-vertical" => Some(("webproperty.industryVertical", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "webproperty.internal-web-property-id" => Some(("webproperty.internalWebPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "webproperty.kind" => Some(("webproperty.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "webproperty.level" => Some(("webproperty.level", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "webproperty.name" => Some(("webproperty.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "webproperty.parent-link.href" => Some(("webproperty.parentLink.href", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "webproperty.parent-link.type" => Some(("webproperty.parentLink.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "webproperty.permissions.effective" => Some(("webproperty.permissions.effective", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "webproperty.profile-count" => Some(("webproperty.profileCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "webproperty.self-link" => Some(("webproperty.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "webproperty.starred" => Some(("webproperty.starred", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "webproperty.updated" => Some(("webproperty.updated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "webproperty.website-url" => Some(("webproperty.websiteUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account", "account-id", "bot-filtering-enabled", "child-link", "created", "currency", "data-retention-reset-on-new-activity", "data-retention-ttl", "default-page", "default-profile-id", "e-commerce-tracking", "effective", "enhanced-e-commerce-tracking", "exclude-query-parameters", "href", "id", "industry-vertical", "internal-web-property-id", "kind", "level", "name", "parent-link", "permissions", "profile", "profile-count", "redirect-uri", "self-link", "site-search-category-parameters", "site-search-query-parameters", "starred", "strip-site-search-category-parameters", "strip-site-search-query-parameters", "timezone", "type", "updated", "web-property-id", "webproperty", "website-url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::AccountTicket = json::value::from_value(object).unwrap();
        let mut call = self.hub.provisioning().create_account_ticket(request);
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

    async fn _provisioning_create_account_tree(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "account-name" => Some(("accountName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile-name" => Some(("profileName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "timezone" => Some(("timezone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "webproperty-name" => Some(("webpropertyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "website-url" => Some(("websiteUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-name", "kind", "profile-name", "timezone", "webproperty-name", "website-url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::AccountTreeRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.provisioning().create_account_tree(request);
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

    async fn _user_deletion_user_deletion_request_upsert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "deletion-request-time" => Some(("deletionRequestTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "firebase-project-id" => Some(("firebaseProjectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id.type" => Some(("id.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id.user-id" => Some(("id.userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "property-id" => Some(("propertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "web-property-id" => Some(("webPropertyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["deletion-request-time", "firebase-project-id", "id", "kind", "property-id", "type", "user-id", "web-property-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::UserDeletionRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.user_deletion().user_deletion_request_upsert(request);
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
            ("data", Some(opt)) => {
                match opt.subcommand() {
                    ("ga-get", Some(opt)) => {
                        call_result = self._data_ga_get(opt, dry_run, &mut err).await;
                    },
                    ("mcf-get", Some(opt)) => {
                        call_result = self._data_mcf_get(opt, dry_run, &mut err).await;
                    },
                    ("realtime-get", Some(opt)) => {
                        call_result = self._data_realtime_get(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("data".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("management", Some(opt)) => {
                match opt.subcommand() {
                    ("account-summaries-list", Some(opt)) => {
                        call_result = self._management_account_summaries_list(opt, dry_run, &mut err).await;
                    },
                    ("account-user-links-delete", Some(opt)) => {
                        call_result = self._management_account_user_links_delete(opt, dry_run, &mut err).await;
                    },
                    ("account-user-links-insert", Some(opt)) => {
                        call_result = self._management_account_user_links_insert(opt, dry_run, &mut err).await;
                    },
                    ("account-user-links-list", Some(opt)) => {
                        call_result = self._management_account_user_links_list(opt, dry_run, &mut err).await;
                    },
                    ("account-user-links-update", Some(opt)) => {
                        call_result = self._management_account_user_links_update(opt, dry_run, &mut err).await;
                    },
                    ("accounts-list", Some(opt)) => {
                        call_result = self._management_accounts_list(opt, dry_run, &mut err).await;
                    },
                    ("client-id-hash-client-id", Some(opt)) => {
                        call_result = self._management_client_id_hash_client_id(opt, dry_run, &mut err).await;
                    },
                    ("custom-data-sources-list", Some(opt)) => {
                        call_result = self._management_custom_data_sources_list(opt, dry_run, &mut err).await;
                    },
                    ("custom-dimensions-get", Some(opt)) => {
                        call_result = self._management_custom_dimensions_get(opt, dry_run, &mut err).await;
                    },
                    ("custom-dimensions-insert", Some(opt)) => {
                        call_result = self._management_custom_dimensions_insert(opt, dry_run, &mut err).await;
                    },
                    ("custom-dimensions-list", Some(opt)) => {
                        call_result = self._management_custom_dimensions_list(opt, dry_run, &mut err).await;
                    },
                    ("custom-dimensions-patch", Some(opt)) => {
                        call_result = self._management_custom_dimensions_patch(opt, dry_run, &mut err).await;
                    },
                    ("custom-dimensions-update", Some(opt)) => {
                        call_result = self._management_custom_dimensions_update(opt, dry_run, &mut err).await;
                    },
                    ("custom-metrics-get", Some(opt)) => {
                        call_result = self._management_custom_metrics_get(opt, dry_run, &mut err).await;
                    },
                    ("custom-metrics-insert", Some(opt)) => {
                        call_result = self._management_custom_metrics_insert(opt, dry_run, &mut err).await;
                    },
                    ("custom-metrics-list", Some(opt)) => {
                        call_result = self._management_custom_metrics_list(opt, dry_run, &mut err).await;
                    },
                    ("custom-metrics-patch", Some(opt)) => {
                        call_result = self._management_custom_metrics_patch(opt, dry_run, &mut err).await;
                    },
                    ("custom-metrics-update", Some(opt)) => {
                        call_result = self._management_custom_metrics_update(opt, dry_run, &mut err).await;
                    },
                    ("experiments-delete", Some(opt)) => {
                        call_result = self._management_experiments_delete(opt, dry_run, &mut err).await;
                    },
                    ("experiments-get", Some(opt)) => {
                        call_result = self._management_experiments_get(opt, dry_run, &mut err).await;
                    },
                    ("experiments-insert", Some(opt)) => {
                        call_result = self._management_experiments_insert(opt, dry_run, &mut err).await;
                    },
                    ("experiments-list", Some(opt)) => {
                        call_result = self._management_experiments_list(opt, dry_run, &mut err).await;
                    },
                    ("experiments-patch", Some(opt)) => {
                        call_result = self._management_experiments_patch(opt, dry_run, &mut err).await;
                    },
                    ("experiments-update", Some(opt)) => {
                        call_result = self._management_experiments_update(opt, dry_run, &mut err).await;
                    },
                    ("filters-delete", Some(opt)) => {
                        call_result = self._management_filters_delete(opt, dry_run, &mut err).await;
                    },
                    ("filters-get", Some(opt)) => {
                        call_result = self._management_filters_get(opt, dry_run, &mut err).await;
                    },
                    ("filters-insert", Some(opt)) => {
                        call_result = self._management_filters_insert(opt, dry_run, &mut err).await;
                    },
                    ("filters-list", Some(opt)) => {
                        call_result = self._management_filters_list(opt, dry_run, &mut err).await;
                    },
                    ("filters-patch", Some(opt)) => {
                        call_result = self._management_filters_patch(opt, dry_run, &mut err).await;
                    },
                    ("filters-update", Some(opt)) => {
                        call_result = self._management_filters_update(opt, dry_run, &mut err).await;
                    },
                    ("goals-get", Some(opt)) => {
                        call_result = self._management_goals_get(opt, dry_run, &mut err).await;
                    },
                    ("goals-insert", Some(opt)) => {
                        call_result = self._management_goals_insert(opt, dry_run, &mut err).await;
                    },
                    ("goals-list", Some(opt)) => {
                        call_result = self._management_goals_list(opt, dry_run, &mut err).await;
                    },
                    ("goals-patch", Some(opt)) => {
                        call_result = self._management_goals_patch(opt, dry_run, &mut err).await;
                    },
                    ("goals-update", Some(opt)) => {
                        call_result = self._management_goals_update(opt, dry_run, &mut err).await;
                    },
                    ("profile-filter-links-delete", Some(opt)) => {
                        call_result = self._management_profile_filter_links_delete(opt, dry_run, &mut err).await;
                    },
                    ("profile-filter-links-get", Some(opt)) => {
                        call_result = self._management_profile_filter_links_get(opt, dry_run, &mut err).await;
                    },
                    ("profile-filter-links-insert", Some(opt)) => {
                        call_result = self._management_profile_filter_links_insert(opt, dry_run, &mut err).await;
                    },
                    ("profile-filter-links-list", Some(opt)) => {
                        call_result = self._management_profile_filter_links_list(opt, dry_run, &mut err).await;
                    },
                    ("profile-filter-links-patch", Some(opt)) => {
                        call_result = self._management_profile_filter_links_patch(opt, dry_run, &mut err).await;
                    },
                    ("profile-filter-links-update", Some(opt)) => {
                        call_result = self._management_profile_filter_links_update(opt, dry_run, &mut err).await;
                    },
                    ("profile-user-links-delete", Some(opt)) => {
                        call_result = self._management_profile_user_links_delete(opt, dry_run, &mut err).await;
                    },
                    ("profile-user-links-insert", Some(opt)) => {
                        call_result = self._management_profile_user_links_insert(opt, dry_run, &mut err).await;
                    },
                    ("profile-user-links-list", Some(opt)) => {
                        call_result = self._management_profile_user_links_list(opt, dry_run, &mut err).await;
                    },
                    ("profile-user-links-update", Some(opt)) => {
                        call_result = self._management_profile_user_links_update(opt, dry_run, &mut err).await;
                    },
                    ("profiles-delete", Some(opt)) => {
                        call_result = self._management_profiles_delete(opt, dry_run, &mut err).await;
                    },
                    ("profiles-get", Some(opt)) => {
                        call_result = self._management_profiles_get(opt, dry_run, &mut err).await;
                    },
                    ("profiles-insert", Some(opt)) => {
                        call_result = self._management_profiles_insert(opt, dry_run, &mut err).await;
                    },
                    ("profiles-list", Some(opt)) => {
                        call_result = self._management_profiles_list(opt, dry_run, &mut err).await;
                    },
                    ("profiles-patch", Some(opt)) => {
                        call_result = self._management_profiles_patch(opt, dry_run, &mut err).await;
                    },
                    ("profiles-update", Some(opt)) => {
                        call_result = self._management_profiles_update(opt, dry_run, &mut err).await;
                    },
                    ("remarketing-audience-delete", Some(opt)) => {
                        call_result = self._management_remarketing_audience_delete(opt, dry_run, &mut err).await;
                    },
                    ("remarketing-audience-get", Some(opt)) => {
                        call_result = self._management_remarketing_audience_get(opt, dry_run, &mut err).await;
                    },
                    ("remarketing-audience-insert", Some(opt)) => {
                        call_result = self._management_remarketing_audience_insert(opt, dry_run, &mut err).await;
                    },
                    ("remarketing-audience-list", Some(opt)) => {
                        call_result = self._management_remarketing_audience_list(opt, dry_run, &mut err).await;
                    },
                    ("remarketing-audience-patch", Some(opt)) => {
                        call_result = self._management_remarketing_audience_patch(opt, dry_run, &mut err).await;
                    },
                    ("remarketing-audience-update", Some(opt)) => {
                        call_result = self._management_remarketing_audience_update(opt, dry_run, &mut err).await;
                    },
                    ("segments-list", Some(opt)) => {
                        call_result = self._management_segments_list(opt, dry_run, &mut err).await;
                    },
                    ("unsampled-reports-delete", Some(opt)) => {
                        call_result = self._management_unsampled_reports_delete(opt, dry_run, &mut err).await;
                    },
                    ("unsampled-reports-get", Some(opt)) => {
                        call_result = self._management_unsampled_reports_get(opt, dry_run, &mut err).await;
                    },
                    ("unsampled-reports-insert", Some(opt)) => {
                        call_result = self._management_unsampled_reports_insert(opt, dry_run, &mut err).await;
                    },
                    ("unsampled-reports-list", Some(opt)) => {
                        call_result = self._management_unsampled_reports_list(opt, dry_run, &mut err).await;
                    },
                    ("uploads-delete-upload-data", Some(opt)) => {
                        call_result = self._management_uploads_delete_upload_data(opt, dry_run, &mut err).await;
                    },
                    ("uploads-get", Some(opt)) => {
                        call_result = self._management_uploads_get(opt, dry_run, &mut err).await;
                    },
                    ("uploads-list", Some(opt)) => {
                        call_result = self._management_uploads_list(opt, dry_run, &mut err).await;
                    },
                    ("uploads-upload-data", Some(opt)) => {
                        call_result = self._management_uploads_upload_data(opt, dry_run, &mut err).await;
                    },
                    ("web-property-ad-words-links-delete", Some(opt)) => {
                        call_result = self._management_web_property_ad_words_links_delete(opt, dry_run, &mut err).await;
                    },
                    ("web-property-ad-words-links-get", Some(opt)) => {
                        call_result = self._management_web_property_ad_words_links_get(opt, dry_run, &mut err).await;
                    },
                    ("web-property-ad-words-links-insert", Some(opt)) => {
                        call_result = self._management_web_property_ad_words_links_insert(opt, dry_run, &mut err).await;
                    },
                    ("web-property-ad-words-links-list", Some(opt)) => {
                        call_result = self._management_web_property_ad_words_links_list(opt, dry_run, &mut err).await;
                    },
                    ("web-property-ad-words-links-patch", Some(opt)) => {
                        call_result = self._management_web_property_ad_words_links_patch(opt, dry_run, &mut err).await;
                    },
                    ("web-property-ad-words-links-update", Some(opt)) => {
                        call_result = self._management_web_property_ad_words_links_update(opt, dry_run, &mut err).await;
                    },
                    ("webproperties-get", Some(opt)) => {
                        call_result = self._management_webproperties_get(opt, dry_run, &mut err).await;
                    },
                    ("webproperties-insert", Some(opt)) => {
                        call_result = self._management_webproperties_insert(opt, dry_run, &mut err).await;
                    },
                    ("webproperties-list", Some(opt)) => {
                        call_result = self._management_webproperties_list(opt, dry_run, &mut err).await;
                    },
                    ("webproperties-patch", Some(opt)) => {
                        call_result = self._management_webproperties_patch(opt, dry_run, &mut err).await;
                    },
                    ("webproperties-update", Some(opt)) => {
                        call_result = self._management_webproperties_update(opt, dry_run, &mut err).await;
                    },
                    ("webproperty-user-links-delete", Some(opt)) => {
                        call_result = self._management_webproperty_user_links_delete(opt, dry_run, &mut err).await;
                    },
                    ("webproperty-user-links-insert", Some(opt)) => {
                        call_result = self._management_webproperty_user_links_insert(opt, dry_run, &mut err).await;
                    },
                    ("webproperty-user-links-list", Some(opt)) => {
                        call_result = self._management_webproperty_user_links_list(opt, dry_run, &mut err).await;
                    },
                    ("webproperty-user-links-update", Some(opt)) => {
                        call_result = self._management_webproperty_user_links_update(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("management".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("metadata", Some(opt)) => {
                match opt.subcommand() {
                    ("columns-list", Some(opt)) => {
                        call_result = self._metadata_columns_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("metadata".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("provisioning", Some(opt)) => {
                match opt.subcommand() {
                    ("create-account-ticket", Some(opt)) => {
                        call_result = self._provisioning_create_account_ticket(opt, dry_run, &mut err).await;
                    },
                    ("create-account-tree", Some(opt)) => {
                        call_result = self._provisioning_create_account_tree(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("provisioning".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("user-deletion", Some(opt)) => {
                match opt.subcommand() {
                    ("user-deletion-request-upsert", Some(opt)) => {
                        call_result = self._user_deletion_user_deletion_request_upsert(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("user-deletion".to_string()));
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

            match client::application_secret_from_directory(&config_dir, "analytics3-secret.json",
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
        ).persist_tokens_to_disk(format!("{}/analytics3", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::Analytics::new(client, auth),
            gp: vec!["alt", "fields", "key", "oauth-token", "pretty-print", "quota-user", "user-ip"],
            gpm: vec![
                    ("oauth-token", "oauth_token"),
                    ("pretty-print", "prettyPrint"),
                    ("quota-user", "quotaUser"),
                    ("user-ip", "userIp"),
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
        ("data", "methods: 'ga-get', 'mcf-get' and 'realtime-get'", vec![
            ("ga-get",
                    Some(r##"Returns Analytics data for a view (profile)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/data_ga-get",
                  vec![
                    (Some(r##"ids"##),
                     None,
                     Some(r##"Unique table ID for retrieving Analytics data. Table ID is of the form ga:XXXX, where XXXX is the Analytics view (profile) ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"start-date"##),
                     None,
                     Some(r##"Start date for fetching Analytics data. Requests can specify a start date formatted as YYYY-MM-DD, or as a relative date (e.g., today, yesterday, or 7daysAgo). The default value is 7daysAgo."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"end-date"##),
                     None,
                     Some(r##"End date for fetching Analytics data. Request can should specify an end date formatted as YYYY-MM-DD, or as a relative date (e.g., today, yesterday, or 7daysAgo). The default value is yesterday."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"metrics"##),
                     None,
                     Some(r##"A comma-separated list of Analytics metrics. E.g., 'ga:sessions,ga:pageviews'. At least one metric must be specified."##),
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
            ("mcf-get",
                    Some(r##"Returns Analytics Multi-Channel Funnels data for a view (profile)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/data_mcf-get",
                  vec![
                    (Some(r##"ids"##),
                     None,
                     Some(r##"Unique table ID for retrieving Analytics data. Table ID is of the form ga:XXXX, where XXXX is the Analytics view (profile) ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"start-date"##),
                     None,
                     Some(r##"Start date for fetching Analytics data. Requests can specify a start date formatted as YYYY-MM-DD, or as a relative date (e.g., today, yesterday, or 7daysAgo). The default value is 7daysAgo."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"end-date"##),
                     None,
                     Some(r##"End date for fetching Analytics data. Requests can specify a start date formatted as YYYY-MM-DD, or as a relative date (e.g., today, yesterday, or 7daysAgo). The default value is 7daysAgo."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"metrics"##),
                     None,
                     Some(r##"A comma-separated list of Multi-Channel Funnels metrics. E.g., 'mcf:totalConversions,mcf:totalConversionValue'. At least one metric must be specified."##),
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
            ("realtime-get",
                    Some(r##"Returns real time data for a view (profile)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/data_realtime-get",
                  vec![
                    (Some(r##"ids"##),
                     None,
                     Some(r##"Unique table ID for retrieving real time data. Table ID is of the form ga:XXXX, where XXXX is the Analytics view (profile) ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"metrics"##),
                     None,
                     Some(r##"A comma-separated list of real time metrics. E.g., 'rt:activeUsers'. At least one metric must be specified."##),
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
        
        ("management", "methods: 'account-summaries-list', 'account-user-links-delete', 'account-user-links-insert', 'account-user-links-list', 'account-user-links-update', 'accounts-list', 'client-id-hash-client-id', 'custom-data-sources-list', 'custom-dimensions-get', 'custom-dimensions-insert', 'custom-dimensions-list', 'custom-dimensions-patch', 'custom-dimensions-update', 'custom-metrics-get', 'custom-metrics-insert', 'custom-metrics-list', 'custom-metrics-patch', 'custom-metrics-update', 'experiments-delete', 'experiments-get', 'experiments-insert', 'experiments-list', 'experiments-patch', 'experiments-update', 'filters-delete', 'filters-get', 'filters-insert', 'filters-list', 'filters-patch', 'filters-update', 'goals-get', 'goals-insert', 'goals-list', 'goals-patch', 'goals-update', 'profile-filter-links-delete', 'profile-filter-links-get', 'profile-filter-links-insert', 'profile-filter-links-list', 'profile-filter-links-patch', 'profile-filter-links-update', 'profile-user-links-delete', 'profile-user-links-insert', 'profile-user-links-list', 'profile-user-links-update', 'profiles-delete', 'profiles-get', 'profiles-insert', 'profiles-list', 'profiles-patch', 'profiles-update', 'remarketing-audience-delete', 'remarketing-audience-get', 'remarketing-audience-insert', 'remarketing-audience-list', 'remarketing-audience-patch', 'remarketing-audience-update', 'segments-list', 'unsampled-reports-delete', 'unsampled-reports-get', 'unsampled-reports-insert', 'unsampled-reports-list', 'uploads-delete-upload-data', 'uploads-get', 'uploads-list', 'uploads-upload-data', 'web-property-ad-words-links-delete', 'web-property-ad-words-links-get', 'web-property-ad-words-links-insert', 'web-property-ad-words-links-list', 'web-property-ad-words-links-patch', 'web-property-ad-words-links-update', 'webproperties-get', 'webproperties-insert', 'webproperties-list', 'webproperties-patch', 'webproperties-update', 'webproperty-user-links-delete', 'webproperty-user-links-insert', 'webproperty-user-links-list' and 'webproperty-user-links-update'", vec![
            ("account-summaries-list",
                    Some(r##"Lists account summaries (lightweight tree comprised of accounts/properties/profiles) to which the user has access."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_account-summaries-list",
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
            ("account-user-links-delete",
                    Some(r##"Removes a user from the given account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_account-user-links-delete",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to delete the user link for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"link-id"##),
                     None,
                     Some(r##"Link ID to delete the user link for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("account-user-links-insert",
                    Some(r##"Adds a new user to the given account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_account-user-links-insert",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to create the user link for."##),
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
            ("account-user-links-list",
                    Some(r##"Lists account-user links for a given account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_account-user-links-list",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to retrieve the user links for."##),
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
            ("account-user-links-update",
                    Some(r##"Updates permissions for an existing user on the given account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_account-user-links-update",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to update the account-user link for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"link-id"##),
                     None,
                     Some(r##"Link ID to update the account-user link for."##),
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
            ("accounts-list",
                    Some(r##"Lists all accounts to which the user has access."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_accounts-list",
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
            ("client-id-hash-client-id",
                    Some(r##"Hashes the given Client ID."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_client-id-hash-client-id",
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
            ("custom-data-sources-list",
                    Some(r##"List custom data sources to which the user has access."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_custom-data-sources-list",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account Id for the custom data sources to retrieve."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property Id for the custom data sources to retrieve."##),
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
            ("custom-dimensions-get",
                    Some(r##"Get a custom dimension to which the user has access."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_custom-dimensions-get",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID for the custom dimension to retrieve."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID for the custom dimension to retrieve."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"custom-dimension-id"##),
                     None,
                     Some(r##"The ID of the custom dimension to retrieve."##),
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
            ("custom-dimensions-insert",
                    Some(r##"Create a new custom dimension."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_custom-dimensions-insert",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID for the custom dimension to create."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID for the custom dimension to create."##),
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
            ("custom-dimensions-list",
                    Some(r##"Lists custom dimensions to which the user has access."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_custom-dimensions-list",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID for the custom dimensions to retrieve."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID for the custom dimensions to retrieve."##),
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
            ("custom-dimensions-patch",
                    Some(r##"Updates an existing custom dimension. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_custom-dimensions-patch",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID for the custom dimension to update."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID for the custom dimension to update."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"custom-dimension-id"##),
                     None,
                     Some(r##"Custom dimension ID for the custom dimension to update."##),
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
            ("custom-dimensions-update",
                    Some(r##"Updates an existing custom dimension."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_custom-dimensions-update",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID for the custom dimension to update."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID for the custom dimension to update."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"custom-dimension-id"##),
                     None,
                     Some(r##"Custom dimension ID for the custom dimension to update."##),
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
            ("custom-metrics-get",
                    Some(r##"Get a custom metric to which the user has access."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_custom-metrics-get",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID for the custom metric to retrieve."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID for the custom metric to retrieve."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"custom-metric-id"##),
                     None,
                     Some(r##"The ID of the custom metric to retrieve."##),
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
            ("custom-metrics-insert",
                    Some(r##"Create a new custom metric."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_custom-metrics-insert",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID for the custom metric to create."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID for the custom dimension to create."##),
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
            ("custom-metrics-list",
                    Some(r##"Lists custom metrics to which the user has access."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_custom-metrics-list",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID for the custom metrics to retrieve."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID for the custom metrics to retrieve."##),
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
            ("custom-metrics-patch",
                    Some(r##"Updates an existing custom metric. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_custom-metrics-patch",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID for the custom metric to update."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID for the custom metric to update."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"custom-metric-id"##),
                     None,
                     Some(r##"Custom metric ID for the custom metric to update."##),
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
            ("custom-metrics-update",
                    Some(r##"Updates an existing custom metric."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_custom-metrics-update",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID for the custom metric to update."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID for the custom metric to update."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"custom-metric-id"##),
                     None,
                     Some(r##"Custom metric ID for the custom metric to update."##),
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
            ("experiments-delete",
                    Some(r##"Delete an experiment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_experiments-delete",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to which the experiment belongs"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID to which the experiment belongs"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"profile-id"##),
                     None,
                     Some(r##"View (Profile) ID to which the experiment belongs"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"experiment-id"##),
                     None,
                     Some(r##"ID of the experiment to delete"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("experiments-get",
                    Some(r##"Returns an experiment to which the user has access."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_experiments-get",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to retrieve the experiment for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID to retrieve the experiment for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"profile-id"##),
                     None,
                     Some(r##"View (Profile) ID to retrieve the experiment for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"experiment-id"##),
                     None,
                     Some(r##"Experiment ID to retrieve the experiment for."##),
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
            ("experiments-insert",
                    Some(r##"Create a new experiment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_experiments-insert",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to create the experiment for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID to create the experiment for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"profile-id"##),
                     None,
                     Some(r##"View (Profile) ID to create the experiment for."##),
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
            ("experiments-list",
                    Some(r##"Lists experiments to which the user has access."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_experiments-list",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to retrieve experiments for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID to retrieve experiments for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"profile-id"##),
                     None,
                     Some(r##"View (Profile) ID to retrieve experiments for."##),
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
            ("experiments-patch",
                    Some(r##"Update an existing experiment. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_experiments-patch",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID of the experiment to update."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID of the experiment to update."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"profile-id"##),
                     None,
                     Some(r##"View (Profile) ID of the experiment to update."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"experiment-id"##),
                     None,
                     Some(r##"Experiment ID of the experiment to update."##),
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
            ("experiments-update",
                    Some(r##"Update an existing experiment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_experiments-update",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID of the experiment to update."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID of the experiment to update."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"profile-id"##),
                     None,
                     Some(r##"View (Profile) ID of the experiment to update."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"experiment-id"##),
                     None,
                     Some(r##"Experiment ID of the experiment to update."##),
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
            ("filters-delete",
                    Some(r##"Delete a filter."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_filters-delete",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to delete the filter for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"filter-id"##),
                     None,
                     Some(r##"ID of the filter to be deleted."##),
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
            ("filters-get",
                    Some(r##"Returns filters to which the user has access."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_filters-get",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to retrieve filters for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"filter-id"##),
                     None,
                     Some(r##"Filter ID to retrieve filters for."##),
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
            ("filters-insert",
                    Some(r##"Create a new filter."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_filters-insert",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to create filter for."##),
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
            ("filters-list",
                    Some(r##"Lists all filters for an account"##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_filters-list",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to retrieve filters for."##),
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
            ("filters-patch",
                    Some(r##"Updates an existing filter. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_filters-patch",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to which the filter belongs."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"filter-id"##),
                     None,
                     Some(r##"ID of the filter to be updated."##),
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
            ("filters-update",
                    Some(r##"Updates an existing filter."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_filters-update",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to which the filter belongs."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"filter-id"##),
                     None,
                     Some(r##"ID of the filter to be updated."##),
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
            ("goals-get",
                    Some(r##"Gets a goal to which the user has access."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_goals-get",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to retrieve the goal for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID to retrieve the goal for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"profile-id"##),
                     None,
                     Some(r##"View (Profile) ID to retrieve the goal for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"goal-id"##),
                     None,
                     Some(r##"Goal ID to retrieve the goal for."##),
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
            ("goals-insert",
                    Some(r##"Create a new goal."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_goals-insert",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to create the goal for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID to create the goal for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"profile-id"##),
                     None,
                     Some(r##"View (Profile) ID to create the goal for."##),
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
            ("goals-list",
                    Some(r##"Lists goals to which the user has access."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_goals-list",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to retrieve goals for. Can either be a specific account ID or '~all', which refers to all the accounts that user has access to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID to retrieve goals for. Can either be a specific web property ID or '~all', which refers to all the web properties that user has access to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"profile-id"##),
                     None,
                     Some(r##"View (Profile) ID to retrieve goals for. Can either be a specific view (profile) ID or '~all', which refers to all the views (profiles) that user has access to."##),
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
            ("goals-patch",
                    Some(r##"Updates an existing goal. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_goals-patch",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to update the goal."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID to update the goal."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"profile-id"##),
                     None,
                     Some(r##"View (Profile) ID to update the goal."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"goal-id"##),
                     None,
                     Some(r##"Index of the goal to be updated."##),
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
            ("goals-update",
                    Some(r##"Updates an existing goal."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_goals-update",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to update the goal."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID to update the goal."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"profile-id"##),
                     None,
                     Some(r##"View (Profile) ID to update the goal."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"goal-id"##),
                     None,
                     Some(r##"Index of the goal to be updated."##),
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
            ("profile-filter-links-delete",
                    Some(r##"Delete a profile filter link."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_profile-filter-links-delete",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to which the profile filter link belongs."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property Id to which the profile filter link belongs."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"profile-id"##),
                     None,
                     Some(r##"Profile ID to which the filter link belongs."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"link-id"##),
                     None,
                     Some(r##"ID of the profile filter link to delete."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("profile-filter-links-get",
                    Some(r##"Returns a single profile filter link."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_profile-filter-links-get",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to retrieve profile filter link for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property Id to retrieve profile filter link for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"profile-id"##),
                     None,
                     Some(r##"Profile ID to retrieve filter link for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"link-id"##),
                     None,
                     Some(r##"ID of the profile filter link."##),
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
            ("profile-filter-links-insert",
                    Some(r##"Create a new profile filter link."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_profile-filter-links-insert",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to create profile filter link for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property Id to create profile filter link for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"profile-id"##),
                     None,
                     Some(r##"Profile ID to create filter link for."##),
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
            ("profile-filter-links-list",
                    Some(r##"Lists all profile filter links for a profile."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_profile-filter-links-list",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to retrieve profile filter links for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property Id for profile filter links for. Can either be a specific web property ID or '~all', which refers to all the web properties that user has access to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"profile-id"##),
                     None,
                     Some(r##"Profile ID to retrieve filter links for. Can either be a specific profile ID or '~all', which refers to all the profiles that user has access to."##),
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
            ("profile-filter-links-patch",
                    Some(r##"Update an existing profile filter link. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_profile-filter-links-patch",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to which profile filter link belongs."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property Id to which profile filter link belongs"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"profile-id"##),
                     None,
                     Some(r##"Profile ID to which filter link belongs"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"link-id"##),
                     None,
                     Some(r##"ID of the profile filter link to be updated."##),
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
            ("profile-filter-links-update",
                    Some(r##"Update an existing profile filter link."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_profile-filter-links-update",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to which profile filter link belongs."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property Id to which profile filter link belongs"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"profile-id"##),
                     None,
                     Some(r##"Profile ID to which filter link belongs"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"link-id"##),
                     None,
                     Some(r##"ID of the profile filter link to be updated."##),
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
            ("profile-user-links-delete",
                    Some(r##"Removes a user from the given view (profile)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_profile-user-links-delete",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to delete the user link for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web Property ID to delete the user link for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"profile-id"##),
                     None,
                     Some(r##"View (Profile) ID to delete the user link for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"link-id"##),
                     None,
                     Some(r##"Link ID to delete the user link for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("profile-user-links-insert",
                    Some(r##"Adds a new user to the given view (profile)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_profile-user-links-insert",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to create the user link for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web Property ID to create the user link for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"profile-id"##),
                     None,
                     Some(r##"View (Profile) ID to create the user link for."##),
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
            ("profile-user-links-list",
                    Some(r##"Lists profile-user links for a given view (profile)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_profile-user-links-list",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID which the given view (profile) belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web Property ID which the given view (profile) belongs to. Can either be a specific web property ID or '~all', which refers to all the web properties that user has access to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"profile-id"##),
                     None,
                     Some(r##"View (Profile) ID to retrieve the profile-user links for. Can either be a specific profile ID or '~all', which refers to all the profiles that user has access to."##),
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
            ("profile-user-links-update",
                    Some(r##"Updates permissions for an existing user on the given view (profile)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_profile-user-links-update",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to update the user link for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web Property ID to update the user link for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"profile-id"##),
                     None,
                     Some(r##"View (Profile ID) to update the user link for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"link-id"##),
                     None,
                     Some(r##"Link ID to update the user link for."##),
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
            ("profiles-delete",
                    Some(r##"Deletes a view (profile)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_profiles-delete",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to delete the view (profile) for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID to delete the view (profile) for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"profile-id"##),
                     None,
                     Some(r##"ID of the view (profile) to be deleted."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("profiles-get",
                    Some(r##"Gets a view (profile) to which the user has access."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_profiles-get",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to retrieve the view (profile) for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID to retrieve the view (profile) for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"profile-id"##),
                     None,
                     Some(r##"View (Profile) ID to retrieve the view (profile) for."##),
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
            ("profiles-insert",
                    Some(r##"Create a new view (profile)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_profiles-insert",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to create the view (profile) for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID to create the view (profile) for."##),
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
            ("profiles-list",
                    Some(r##"Lists views (profiles) to which the user has access."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_profiles-list",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID for the view (profiles) to retrieve. Can either be a specific account ID or '~all', which refers to all the accounts to which the user has access."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID for the views (profiles) to retrieve. Can either be a specific web property ID or '~all', which refers to all the web properties to which the user has access."##),
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
            ("profiles-patch",
                    Some(r##"Updates an existing view (profile). This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_profiles-patch",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to which the view (profile) belongs"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID to which the view (profile) belongs"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"profile-id"##),
                     None,
                     Some(r##"ID of the view (profile) to be updated."##),
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
            ("profiles-update",
                    Some(r##"Updates an existing view (profile)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_profiles-update",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to which the view (profile) belongs"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID to which the view (profile) belongs"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"profile-id"##),
                     None,
                     Some(r##"ID of the view (profile) to be updated."##),
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
            ("remarketing-audience-delete",
                    Some(r##"Delete a remarketing audience."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_remarketing-audience-delete",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to which the remarketing audience belongs."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID to which the remarketing audience belongs."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"remarketing-audience-id"##),
                     None,
                     Some(r##"The ID of the remarketing audience to delete."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("remarketing-audience-get",
                    Some(r##"Gets a remarketing audience to which the user has access."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_remarketing-audience-get",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The account ID of the remarketing audience to retrieve."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"The web property ID of the remarketing audience to retrieve."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"remarketing-audience-id"##),
                     None,
                     Some(r##"The ID of the remarketing audience to retrieve."##),
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
            ("remarketing-audience-insert",
                    Some(r##"Creates a new remarketing audience."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_remarketing-audience-insert",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The account ID for which to create the remarketing audience."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID for which to create the remarketing audience."##),
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
            ("remarketing-audience-list",
                    Some(r##"Lists remarketing audiences to which the user has access."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_remarketing-audience-list",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The account ID of the remarketing audiences to retrieve."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"The web property ID of the remarketing audiences to retrieve."##),
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
            ("remarketing-audience-patch",
                    Some(r##"Updates an existing remarketing audience. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_remarketing-audience-patch",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The account ID of the remarketing audience to update."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"The web property ID of the remarketing audience to update."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"remarketing-audience-id"##),
                     None,
                     Some(r##"The ID of the remarketing audience to update."##),
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
            ("remarketing-audience-update",
                    Some(r##"Updates an existing remarketing audience."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_remarketing-audience-update",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The account ID of the remarketing audience to update."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"The web property ID of the remarketing audience to update."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"remarketing-audience-id"##),
                     None,
                     Some(r##"The ID of the remarketing audience to update."##),
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
            ("segments-list",
                    Some(r##"Lists segments to which the user has access."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_segments-list",
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
            ("unsampled-reports-delete",
                    Some(r##"Deletes an unsampled report."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_unsampled-reports-delete",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to delete the unsampled report for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID to delete the unsampled reports for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"profile-id"##),
                     None,
                     Some(r##"View (Profile) ID to delete the unsampled report for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"unsampled-report-id"##),
                     None,
                     Some(r##"ID of the unsampled report to be deleted."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("unsampled-reports-get",
                    Some(r##"Returns a single unsampled report."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_unsampled-reports-get",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to retrieve unsampled report for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID to retrieve unsampled reports for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"profile-id"##),
                     None,
                     Some(r##"View (Profile) ID to retrieve unsampled report for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"unsampled-report-id"##),
                     None,
                     Some(r##"ID of the unsampled report to retrieve."##),
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
            ("unsampled-reports-insert",
                    Some(r##"Create a new unsampled report."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_unsampled-reports-insert",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to create the unsampled report for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID to create the unsampled report for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"profile-id"##),
                     None,
                     Some(r##"View (Profile) ID to create the unsampled report for."##),
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
            ("unsampled-reports-list",
                    Some(r##"Lists unsampled reports to which the user has access."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_unsampled-reports-list",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to retrieve unsampled reports for. Must be a specific account ID, ~all is not supported."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID to retrieve unsampled reports for. Must be a specific web property ID, ~all is not supported."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"profile-id"##),
                     None,
                     Some(r##"View (Profile) ID to retrieve unsampled reports for. Must be a specific view (profile) ID, ~all is not supported."##),
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
            ("uploads-delete-upload-data",
                    Some(r##"Delete data associated with a previous upload."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_uploads-delete-upload-data",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account Id for the uploads to be deleted."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property Id for the uploads to be deleted."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"custom-data-source-id"##),
                     None,
                     Some(r##"Custom data source Id for the uploads to be deleted."##),
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
                  ]),
            ("uploads-get",
                    Some(r##"List uploads to which the user has access."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_uploads-get",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account Id for the upload to retrieve."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property Id for the upload to retrieve."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"custom-data-source-id"##),
                     None,
                     Some(r##"Custom data source Id for upload to retrieve."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"upload-id"##),
                     None,
                     Some(r##"Upload Id to retrieve."##),
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
            ("uploads-list",
                    Some(r##"List uploads to which the user has access."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_uploads-list",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account Id for the uploads to retrieve."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property Id for the uploads to retrieve."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"custom-data-source-id"##),
                     None,
                     Some(r##"Custom data source Id for uploads to retrieve."##),
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
            ("uploads-upload-data",
                    Some(r##"Upload data for a custom data source."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_uploads-upload-data",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account Id associated with the upload."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property UA-string associated with the upload."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"custom-data-source-id"##),
                     None,
                     Some(r##"Custom data source Id to which the data being uploaded belongs."##),
                     Some(true),
                     Some(false)),
        
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
            ("web-property-ad-words-links-delete",
                    Some(r##"Deletes a web property-Google Ads link."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_web-property-ad-words-links-delete",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"ID of the account which the given web property belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID to delete the Google Ads link for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-ad-words-link-id"##),
                     None,
                     Some(r##"Web property Google Ads link ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("web-property-ad-words-links-get",
                    Some(r##"Returns a web property-Google Ads link to which the user has access."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_web-property-ad-words-links-get",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"ID of the account which the given web property belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID to retrieve the Google Ads link for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-ad-words-link-id"##),
                     None,
                     Some(r##"Web property-Google Ads link ID."##),
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
            ("web-property-ad-words-links-insert",
                    Some(r##"Creates a webProperty-Google Ads link."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_web-property-ad-words-links-insert",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"ID of the Google Analytics account to create the link for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID to create the link for."##),
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
            ("web-property-ad-words-links-list",
                    Some(r##"Lists webProperty-Google Ads links for a given web property."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_web-property-ad-words-links-list",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"ID of the account which the given web property belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID to retrieve the Google Ads links for."##),
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
            ("web-property-ad-words-links-patch",
                    Some(r##"Updates an existing webProperty-Google Ads link. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_web-property-ad-words-links-patch",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"ID of the account which the given web property belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID to retrieve the Google Ads link for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-ad-words-link-id"##),
                     None,
                     Some(r##"Web property-Google Ads link ID."##),
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
            ("web-property-ad-words-links-update",
                    Some(r##"Updates an existing webProperty-Google Ads link."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_web-property-ad-words-links-update",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"ID of the account which the given web property belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID to retrieve the Google Ads link for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-ad-words-link-id"##),
                     None,
                     Some(r##"Web property-Google Ads link ID."##),
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
            ("webproperties-get",
                    Some(r##"Gets a web property to which the user has access."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_webproperties-get",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to retrieve the web property for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"ID to retrieve the web property for."##),
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
            ("webproperties-insert",
                    Some(r##"Create a new property if the account has fewer than 20 properties. Web properties are visible in the Google Analytics interface only if they have at least one profile."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_webproperties-insert",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to create the web property for."##),
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
            ("webproperties-list",
                    Some(r##"Lists web properties to which the user has access."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_webproperties-list",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to retrieve web properties for. Can either be a specific account ID or '~all', which refers to all the accounts that user has access to."##),
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
            ("webproperties-patch",
                    Some(r##"Updates an existing web property. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_webproperties-patch",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to which the web property belongs"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID"##),
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
            ("webproperties-update",
                    Some(r##"Updates an existing web property."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_webproperties-update",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to which the web property belongs"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID"##),
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
            ("webproperty-user-links-delete",
                    Some(r##"Removes a user from the given web property."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_webproperty-user-links-delete",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to delete the user link for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web Property ID to delete the user link for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"link-id"##),
                     None,
                     Some(r##"Link ID to delete the user link for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("webproperty-user-links-insert",
                    Some(r##"Adds a new user to the given web property."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_webproperty-user-links-insert",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to create the user link for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web Property ID to create the user link for."##),
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
            ("webproperty-user-links-list",
                    Some(r##"Lists webProperty-user links for a given web property."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_webproperty-user-links-list",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID which the given web property belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web Property ID for the webProperty-user links to retrieve. Can either be a specific web property ID or '~all', which refers to all the web properties that user has access to."##),
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
            ("webproperty-user-links-update",
                    Some(r##"Updates permissions for an existing user on the given web property."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_webproperty-user-links-update",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"Account ID to update the account-user link for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID to update the account-user link for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"link-id"##),
                     None,
                     Some(r##"Link ID to update the account-user link for."##),
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
        
        ("metadata", "methods: 'columns-list'", vec![
            ("columns-list",
                    Some(r##"Lists all columns for a report type"##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/metadata_columns-list",
                  vec![
                    (Some(r##"report-type"##),
                     None,
                     Some(r##"Report type. Allowed Values: 'ga'. Where 'ga' corresponds to the Core Reporting API"##),
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
        
        ("provisioning", "methods: 'create-account-ticket' and 'create-account-tree'", vec![
            ("create-account-ticket",
                    Some(r##"Creates an account ticket."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/provisioning_create-account-ticket",
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
            ("create-account-tree",
                    Some(r##"Provision account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/provisioning_create-account-tree",
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
        
        ("user-deletion", "methods: 'user-deletion-request-upsert'", vec![
            ("user-deletion-request-upsert",
                    Some(r##"Insert or update a user deletion requests."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/user-deletion_user-deletion-request-upsert",
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
    
    let mut app = App::new("analytics3")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+20190807")
           .about("Views and manages your Google Analytics data.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_analytics3_cli")
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
