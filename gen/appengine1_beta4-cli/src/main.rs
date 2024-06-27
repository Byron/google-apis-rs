// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_appengine1_beta4::{api, Error, oauth2, client::chrono, FieldMask};


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
    hub: api::Appengine<S>,
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
    async fn _apps_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "auth-domain" => Some(("authDomain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "code-bucket" => Some(("codeBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-bucket" => Some(("defaultBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-cookie-expiration" => Some(("defaultCookieExpiration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-hostname" => Some(("defaultHostname", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "iap.enabled" => Some(("iap.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "iap.oauth2-client-id" => Some(("iap.oauth2ClientId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "iap.oauth2-client-secret" => Some(("iap.oauth2ClientSecret", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "iap.oauth2-client-secret-sha256" => Some(("iap.oauth2ClientSecretSha256", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location" => Some(("location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["auth-domain", "code-bucket", "default-bucket", "default-cookie-expiration", "default-hostname", "enabled", "iap", "id", "location", "name", "oauth2-client-id", "oauth2-client-secret", "oauth2-client-secret-sha256"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Application = json::value::from_value(object).unwrap();
        let mut call = self.hub.apps().create(request);
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

    async fn _apps_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().get(opt.value_of("apps-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "ensure-resources-exist" => {
                    call = call.ensure_resources_exist(        value.map(|v| arg_from_str(v, err, "ensure-resources-exist", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["ensure-resources-exist"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _apps_locations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().locations_get(opt.value_of("apps-id").unwrap_or(""), opt.value_of("locations-id").unwrap_or(""));
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

    async fn _apps_locations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().locations_list(opt.value_of("apps-id").unwrap_or(""));
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

    async fn _apps_modules_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().modules_delete(opt.value_of("apps-id").unwrap_or(""), opt.value_of("modules-id").unwrap_or(""));
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

    async fn _apps_modules_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().modules_get(opt.value_of("apps-id").unwrap_or(""), opt.value_of("modules-id").unwrap_or(""));
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

    async fn _apps_modules_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().modules_list(opt.value_of("apps-id").unwrap_or(""));
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

    async fn _apps_modules_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "split.allocations" => Some(("split.allocations", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Map })),
                    "split.shard-by" => Some(("split.shardBy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["allocations", "id", "name", "shard-by", "split"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Module = json::value::from_value(object).unwrap();
        let mut call = self.hub.apps().modules_patch(request, opt.value_of("apps-id").unwrap_or(""), opt.value_of("modules-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "migrate-traffic" => {
                    call = call.migrate_traffic(        value.map(|v| arg_from_str(v, err, "migrate-traffic", "boolean")).unwrap_or(false));
                },
                "mask" => {
                    call = call.mask(        value.map(|v| arg_from_str(v, err, "mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
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
                                                                           v.extend(["mask", "migrate-traffic"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _apps_modules_versions_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "api-config.auth-fail-action" => Some(("apiConfig.authFailAction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-config.login" => Some(("apiConfig.login", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-config.script" => Some(("apiConfig.script", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-config.security-level" => Some(("apiConfig.securityLevel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-config.url" => Some(("apiConfig.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "automatic-scaling.cool-down-period" => Some(("automaticScaling.coolDownPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "automatic-scaling.cpu-utilization.aggregation-window-length" => Some(("automaticScaling.cpuUtilization.aggregationWindowLength", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "automatic-scaling.cpu-utilization.target-utilization" => Some(("automaticScaling.cpuUtilization.targetUtilization", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "automatic-scaling.disk-utilization.target-read-bytes-per-sec" => Some(("automaticScaling.diskUtilization.targetReadBytesPerSec", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.disk-utilization.target-read-ops-per-sec" => Some(("automaticScaling.diskUtilization.targetReadOpsPerSec", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.disk-utilization.target-write-bytes-per-sec" => Some(("automaticScaling.diskUtilization.targetWriteBytesPerSec", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.disk-utilization.target-write-ops-per-sec" => Some(("automaticScaling.diskUtilization.targetWriteOpsPerSec", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.max-concurrent-requests" => Some(("automaticScaling.maxConcurrentRequests", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.max-idle-instances" => Some(("automaticScaling.maxIdleInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.max-pending-latency" => Some(("automaticScaling.maxPendingLatency", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "automatic-scaling.max-total-instances" => Some(("automaticScaling.maxTotalInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.min-idle-instances" => Some(("automaticScaling.minIdleInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.min-pending-latency" => Some(("automaticScaling.minPendingLatency", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "automatic-scaling.min-total-instances" => Some(("automaticScaling.minTotalInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.network-utilization.target-received-bytes-per-sec" => Some(("automaticScaling.networkUtilization.targetReceivedBytesPerSec", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.network-utilization.target-received-packets-per-sec" => Some(("automaticScaling.networkUtilization.targetReceivedPacketsPerSec", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.network-utilization.target-sent-bytes-per-sec" => Some(("automaticScaling.networkUtilization.targetSentBytesPerSec", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.network-utilization.target-sent-packets-per-sec" => Some(("automaticScaling.networkUtilization.targetSentPacketsPerSec", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.request-utilization.target-concurrent-requests" => Some(("automaticScaling.requestUtilization.targetConcurrentRequests", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.request-utilization.target-request-count-per-sec" => Some(("automaticScaling.requestUtilization.targetRequestCountPerSec", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "basic-scaling.idle-timeout" => Some(("basicScaling.idleTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "basic-scaling.max-instances" => Some(("basicScaling.maxInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "beta-settings" => Some(("betaSettings", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "creation-time" => Some(("creationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-expiration" => Some(("defaultExpiration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployer" => Some(("deployer", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment.container.image" => Some(("deployment.container.image", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "endpoints-api-service.config-id" => Some(("endpointsApiService.configId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "endpoints-api-service.disable-trace-sampling" => Some(("endpointsApiService.disableTraceSampling", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "endpoints-api-service.name" => Some(("endpointsApiService.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "endpoints-api-service.rollout-strategy" => Some(("endpointsApiService.rolloutStrategy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "env" => Some(("env", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "env-variables" => Some(("envVariables", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "health-check.check-interval" => Some(("healthCheck.checkInterval", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-check.disable-health-check" => Some(("healthCheck.disableHealthCheck", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "health-check.healthy-threshold" => Some(("healthCheck.healthyThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "health-check.host" => Some(("healthCheck.host", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-check.restart-threshold" => Some(("healthCheck.restartThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "health-check.timeout" => Some(("healthCheck.timeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-check.unhealthy-threshold" => Some(("healthCheck.unhealthyThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inbound-services" => Some(("inboundServices", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "instance-class" => Some(("instanceClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "manual-scaling.instances" => Some(("manualScaling.instances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "network.forwarded-ports" => Some(("network.forwardedPorts", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "network.instance-tag" => Some(("network.instanceTag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "network.name" => Some(("network.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nobuild-files-regex" => Some(("nobuildFilesRegex", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "resources.cpu" => Some(("resources.cpu", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "resources.disk-gb" => Some(("resources.diskGb", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "resources.memory-gb" => Some(("resources.memoryGb", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "runtime" => Some(("runtime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-api-version" => Some(("runtimeApiVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "serving-status" => Some(("servingStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "threadsafe" => Some(("threadsafe", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "vm" => Some(("vm", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["aggregation-window-length", "api-config", "auth-fail-action", "automatic-scaling", "basic-scaling", "beta-settings", "check-interval", "config-id", "container", "cool-down-period", "cpu", "cpu-utilization", "creation-time", "default-expiration", "deployer", "deployment", "disable-health-check", "disable-trace-sampling", "disk-gb", "disk-utilization", "endpoints-api-service", "env", "env-variables", "forwarded-ports", "health-check", "healthy-threshold", "host", "id", "idle-timeout", "image", "inbound-services", "instance-class", "instance-tag", "instances", "login", "manual-scaling", "max-concurrent-requests", "max-idle-instances", "max-instances", "max-pending-latency", "max-total-instances", "memory-gb", "min-idle-instances", "min-pending-latency", "min-total-instances", "name", "network", "network-utilization", "nobuild-files-regex", "request-utilization", "resources", "restart-threshold", "rollout-strategy", "runtime", "runtime-api-version", "script", "security-level", "serving-status", "target-concurrent-requests", "target-read-bytes-per-sec", "target-read-ops-per-sec", "target-received-bytes-per-sec", "target-received-packets-per-sec", "target-request-count-per-sec", "target-sent-bytes-per-sec", "target-sent-packets-per-sec", "target-utilization", "target-write-bytes-per-sec", "target-write-ops-per-sec", "threadsafe", "timeout", "unhealthy-threshold", "url", "vm"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Version = json::value::from_value(object).unwrap();
        let mut call = self.hub.apps().modules_versions_create(request, opt.value_of("apps-id").unwrap_or(""), opt.value_of("modules-id").unwrap_or(""));
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

    async fn _apps_modules_versions_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().modules_versions_delete(opt.value_of("apps-id").unwrap_or(""), opt.value_of("modules-id").unwrap_or(""), opt.value_of("versions-id").unwrap_or(""));
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

    async fn _apps_modules_versions_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().modules_versions_get(opt.value_of("apps-id").unwrap_or(""), opt.value_of("modules-id").unwrap_or(""), opt.value_of("versions-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "view" => {
                    call = call.view(value.unwrap_or(""));
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
                                                                           v.extend(["view"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _apps_modules_versions_instances_debug(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "ssh-key" => Some(("sshKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["ssh-key"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::DebugInstanceRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.apps().modules_versions_instances_debug(request, opt.value_of("apps-id").unwrap_or(""), opt.value_of("modules-id").unwrap_or(""), opt.value_of("versions-id").unwrap_or(""), opt.value_of("instances-id").unwrap_or(""));
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

    async fn _apps_modules_versions_instances_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().modules_versions_instances_delete(opt.value_of("apps-id").unwrap_or(""), opt.value_of("modules-id").unwrap_or(""), opt.value_of("versions-id").unwrap_or(""), opt.value_of("instances-id").unwrap_or(""));
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

    async fn _apps_modules_versions_instances_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().modules_versions_instances_get(opt.value_of("apps-id").unwrap_or(""), opt.value_of("modules-id").unwrap_or(""), opt.value_of("versions-id").unwrap_or(""), opt.value_of("instances-id").unwrap_or(""));
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

    async fn _apps_modules_versions_instances_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().modules_versions_instances_list(opt.value_of("apps-id").unwrap_or(""), opt.value_of("modules-id").unwrap_or(""), opt.value_of("versions-id").unwrap_or(""));
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

    async fn _apps_modules_versions_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().modules_versions_list(opt.value_of("apps-id").unwrap_or(""), opt.value_of("modules-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "view" => {
                    call = call.view(value.unwrap_or(""));
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
                                                                           v.extend(["page-size", "page-token", "view"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _apps_modules_versions_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "api-config.auth-fail-action" => Some(("apiConfig.authFailAction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-config.login" => Some(("apiConfig.login", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-config.script" => Some(("apiConfig.script", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-config.security-level" => Some(("apiConfig.securityLevel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-config.url" => Some(("apiConfig.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "automatic-scaling.cool-down-period" => Some(("automaticScaling.coolDownPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "automatic-scaling.cpu-utilization.aggregation-window-length" => Some(("automaticScaling.cpuUtilization.aggregationWindowLength", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "automatic-scaling.cpu-utilization.target-utilization" => Some(("automaticScaling.cpuUtilization.targetUtilization", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "automatic-scaling.disk-utilization.target-read-bytes-per-sec" => Some(("automaticScaling.diskUtilization.targetReadBytesPerSec", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.disk-utilization.target-read-ops-per-sec" => Some(("automaticScaling.diskUtilization.targetReadOpsPerSec", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.disk-utilization.target-write-bytes-per-sec" => Some(("automaticScaling.diskUtilization.targetWriteBytesPerSec", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.disk-utilization.target-write-ops-per-sec" => Some(("automaticScaling.diskUtilization.targetWriteOpsPerSec", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.max-concurrent-requests" => Some(("automaticScaling.maxConcurrentRequests", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.max-idle-instances" => Some(("automaticScaling.maxIdleInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.max-pending-latency" => Some(("automaticScaling.maxPendingLatency", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "automatic-scaling.max-total-instances" => Some(("automaticScaling.maxTotalInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.min-idle-instances" => Some(("automaticScaling.minIdleInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.min-pending-latency" => Some(("automaticScaling.minPendingLatency", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "automatic-scaling.min-total-instances" => Some(("automaticScaling.minTotalInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.network-utilization.target-received-bytes-per-sec" => Some(("automaticScaling.networkUtilization.targetReceivedBytesPerSec", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.network-utilization.target-received-packets-per-sec" => Some(("automaticScaling.networkUtilization.targetReceivedPacketsPerSec", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.network-utilization.target-sent-bytes-per-sec" => Some(("automaticScaling.networkUtilization.targetSentBytesPerSec", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.network-utilization.target-sent-packets-per-sec" => Some(("automaticScaling.networkUtilization.targetSentPacketsPerSec", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.request-utilization.target-concurrent-requests" => Some(("automaticScaling.requestUtilization.targetConcurrentRequests", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.request-utilization.target-request-count-per-sec" => Some(("automaticScaling.requestUtilization.targetRequestCountPerSec", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "basic-scaling.idle-timeout" => Some(("basicScaling.idleTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "basic-scaling.max-instances" => Some(("basicScaling.maxInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "beta-settings" => Some(("betaSettings", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "creation-time" => Some(("creationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-expiration" => Some(("defaultExpiration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployer" => Some(("deployer", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment.container.image" => Some(("deployment.container.image", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "endpoints-api-service.config-id" => Some(("endpointsApiService.configId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "endpoints-api-service.disable-trace-sampling" => Some(("endpointsApiService.disableTraceSampling", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "endpoints-api-service.name" => Some(("endpointsApiService.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "endpoints-api-service.rollout-strategy" => Some(("endpointsApiService.rolloutStrategy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "env" => Some(("env", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "env-variables" => Some(("envVariables", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "health-check.check-interval" => Some(("healthCheck.checkInterval", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-check.disable-health-check" => Some(("healthCheck.disableHealthCheck", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "health-check.healthy-threshold" => Some(("healthCheck.healthyThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "health-check.host" => Some(("healthCheck.host", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-check.restart-threshold" => Some(("healthCheck.restartThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "health-check.timeout" => Some(("healthCheck.timeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-check.unhealthy-threshold" => Some(("healthCheck.unhealthyThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inbound-services" => Some(("inboundServices", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "instance-class" => Some(("instanceClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "manual-scaling.instances" => Some(("manualScaling.instances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "network.forwarded-ports" => Some(("network.forwardedPorts", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "network.instance-tag" => Some(("network.instanceTag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "network.name" => Some(("network.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nobuild-files-regex" => Some(("nobuildFilesRegex", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "resources.cpu" => Some(("resources.cpu", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "resources.disk-gb" => Some(("resources.diskGb", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "resources.memory-gb" => Some(("resources.memoryGb", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "runtime" => Some(("runtime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime-api-version" => Some(("runtimeApiVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "serving-status" => Some(("servingStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "threadsafe" => Some(("threadsafe", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "vm" => Some(("vm", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["aggregation-window-length", "api-config", "auth-fail-action", "automatic-scaling", "basic-scaling", "beta-settings", "check-interval", "config-id", "container", "cool-down-period", "cpu", "cpu-utilization", "creation-time", "default-expiration", "deployer", "deployment", "disable-health-check", "disable-trace-sampling", "disk-gb", "disk-utilization", "endpoints-api-service", "env", "env-variables", "forwarded-ports", "health-check", "healthy-threshold", "host", "id", "idle-timeout", "image", "inbound-services", "instance-class", "instance-tag", "instances", "login", "manual-scaling", "max-concurrent-requests", "max-idle-instances", "max-instances", "max-pending-latency", "max-total-instances", "memory-gb", "min-idle-instances", "min-pending-latency", "min-total-instances", "name", "network", "network-utilization", "nobuild-files-regex", "request-utilization", "resources", "restart-threshold", "rollout-strategy", "runtime", "runtime-api-version", "script", "security-level", "serving-status", "target-concurrent-requests", "target-read-bytes-per-sec", "target-read-ops-per-sec", "target-received-bytes-per-sec", "target-received-packets-per-sec", "target-request-count-per-sec", "target-sent-bytes-per-sec", "target-sent-packets-per-sec", "target-utilization", "target-write-bytes-per-sec", "target-write-ops-per-sec", "threadsafe", "timeout", "unhealthy-threshold", "url", "vm"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Version = json::value::from_value(object).unwrap();
        let mut call = self.hub.apps().modules_versions_patch(request, opt.value_of("apps-id").unwrap_or(""), opt.value_of("modules-id").unwrap_or(""), opt.value_of("versions-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "mask" => {
                    call = call.mask(        value.map(|v| arg_from_str(v, err, "mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
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
                                                                           v.extend(["mask"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _apps_operations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().operations_get(opt.value_of("apps-id").unwrap_or(""), opt.value_of("operations-id").unwrap_or(""));
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

    async fn _apps_operations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().operations_list(opt.value_of("apps-id").unwrap_or(""));
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

    async fn _apps_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "auth-domain" => Some(("authDomain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "code-bucket" => Some(("codeBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-bucket" => Some(("defaultBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-cookie-expiration" => Some(("defaultCookieExpiration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-hostname" => Some(("defaultHostname", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "iap.enabled" => Some(("iap.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "iap.oauth2-client-id" => Some(("iap.oauth2ClientId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "iap.oauth2-client-secret" => Some(("iap.oauth2ClientSecret", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "iap.oauth2-client-secret-sha256" => Some(("iap.oauth2ClientSecretSha256", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location" => Some(("location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["auth-domain", "code-bucket", "default-bucket", "default-cookie-expiration", "default-hostname", "enabled", "iap", "id", "location", "name", "oauth2-client-id", "oauth2-client-secret", "oauth2-client-secret-sha256"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Application = json::value::from_value(object).unwrap();
        let mut call = self.hub.apps().patch(request, opt.value_of("apps-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "mask" => {
                    call = call.mask(        value.map(|v| arg_from_str(v, err, "mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
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
                                                                           v.extend(["mask"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
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
            ("apps", Some(opt)) => {
                match opt.subcommand() {
                    ("create", Some(opt)) => {
                        call_result = self._apps_create(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._apps_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-get", Some(opt)) => {
                        call_result = self._apps_locations_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-list", Some(opt)) => {
                        call_result = self._apps_locations_list(opt, dry_run, &mut err).await;
                    },
                    ("modules-delete", Some(opt)) => {
                        call_result = self._apps_modules_delete(opt, dry_run, &mut err).await;
                    },
                    ("modules-get", Some(opt)) => {
                        call_result = self._apps_modules_get(opt, dry_run, &mut err).await;
                    },
                    ("modules-list", Some(opt)) => {
                        call_result = self._apps_modules_list(opt, dry_run, &mut err).await;
                    },
                    ("modules-patch", Some(opt)) => {
                        call_result = self._apps_modules_patch(opt, dry_run, &mut err).await;
                    },
                    ("modules-versions-create", Some(opt)) => {
                        call_result = self._apps_modules_versions_create(opt, dry_run, &mut err).await;
                    },
                    ("modules-versions-delete", Some(opt)) => {
                        call_result = self._apps_modules_versions_delete(opt, dry_run, &mut err).await;
                    },
                    ("modules-versions-get", Some(opt)) => {
                        call_result = self._apps_modules_versions_get(opt, dry_run, &mut err).await;
                    },
                    ("modules-versions-instances-debug", Some(opt)) => {
                        call_result = self._apps_modules_versions_instances_debug(opt, dry_run, &mut err).await;
                    },
                    ("modules-versions-instances-delete", Some(opt)) => {
                        call_result = self._apps_modules_versions_instances_delete(opt, dry_run, &mut err).await;
                    },
                    ("modules-versions-instances-get", Some(opt)) => {
                        call_result = self._apps_modules_versions_instances_get(opt, dry_run, &mut err).await;
                    },
                    ("modules-versions-instances-list", Some(opt)) => {
                        call_result = self._apps_modules_versions_instances_list(opt, dry_run, &mut err).await;
                    },
                    ("modules-versions-list", Some(opt)) => {
                        call_result = self._apps_modules_versions_list(opt, dry_run, &mut err).await;
                    },
                    ("modules-versions-patch", Some(opt)) => {
                        call_result = self._apps_modules_versions_patch(opt, dry_run, &mut err).await;
                    },
                    ("operations-get", Some(opt)) => {
                        call_result = self._apps_operations_get(opt, dry_run, &mut err).await;
                    },
                    ("operations-list", Some(opt)) => {
                        call_result = self._apps_operations_list(opt, dry_run, &mut err).await;
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._apps_patch(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("apps".to_string()));
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

            match client::application_secret_from_directory(&config_dir, "appengine1-beta4-secret.json",
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
        ).persist_tokens_to_disk(format!("{}/appengine1-beta4", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::Appengine::new(client, auth),
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
        ("apps", "methods: 'create', 'get', 'locations-get', 'locations-list', 'modules-delete', 'modules-get', 'modules-list', 'modules-patch', 'modules-versions-create', 'modules-versions-delete', 'modules-versions-get', 'modules-versions-instances-debug', 'modules-versions-instances-delete', 'modules-versions-instances-get', 'modules-versions-instances-list', 'modules-versions-list', 'modules-versions-patch', 'operations-get', 'operations-list' and 'patch'", vec![
            ("create",
                    Some(r##"Creates an App Engine application for a Google Cloud Platform project. Required fields:
        id - The ID of the target Cloud Platform project.
        location - The region (https://cloud.google.com/appengine/docs/locations) where you want the App Engine application located.For more information about App Engine applications, see Managing Projects, Applications, and Billing (https://cloud.google.com/appengine/docs/python/console/)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_beta4_cli/apps_create",
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
            ("get",
                    Some(r##"Gets information about an application."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_beta4_cli/apps_get",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the application to get. Example: apps/myapp."##),
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
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_beta4_cli/apps_locations-get",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Resource name for the location."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"locations-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_beta4_cli/apps_locations-list",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. The resource that owns the locations collection, if applicable."##),
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
            ("modules-delete",
                    Some(r##"Deletes the specified module and all enclosed versions."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_beta4_cli/apps_modules-delete",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource requested. Example: apps/myapp/modules/default."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"modules-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
            ("modules-get",
                    Some(r##"Gets the current configuration of the specified module."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_beta4_cli/apps_modules-get",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource requested. Example: apps/myapp/modules/default."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"modules-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
            ("modules-list",
                    Some(r##"Lists all the modules in the application."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_beta4_cli/apps_modules-list",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource requested. Example: apps/myapp."##),
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
            ("modules-patch",
                    Some(r##"Updates the configuration of the specified module."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_beta4_cli/apps_modules-patch",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource to update. Example: apps/myapp/modules/default."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"modules-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
            ("modules-versions-create",
                    Some(r##"Deploys code and resource files to a new version."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_beta4_cli/apps_modules-versions-create",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource to update. Example: apps/myapp/modules/default."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"modules-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
            ("modules-versions-delete",
                    Some(r##"Deletes an existing version."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_beta4_cli/apps_modules-versions-delete",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource requested. Example: apps/myapp/modules/default/versions/v1."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"modules-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"versions-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
            ("modules-versions-get",
                    Some(r##"Gets the specified Version resource. By default, only a BASIC_VIEW will be returned. Specify the FULL_VIEW parameter to get the full resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_beta4_cli/apps_modules-versions-get",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource requested. Example: apps/myapp/modules/default/versions/v1."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"modules-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"versions-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
            ("modules-versions-instances-debug",
                    Some(r##"Enables debugging on a VM instance. This allows you to use the SSH command to connect to the virtual machine where the instance lives. While in "debug mode", the instance continues to serve live traffic. You should delete the instance when you are done debugging and then allow the system to take over and determine if another instance should be started.Only applicable for instances in App Engine flexible environment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_beta4_cli/apps_modules-versions-instances-debug",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource requested. Example: apps/myapp/modules/default/versions/v1/instances/instance-1."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"modules-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"versions-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instances-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
            ("modules-versions-instances-delete",
                    Some(r##"Stops a running instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_beta4_cli/apps_modules-versions-instances-delete",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource requested. Example: apps/myapp/modules/default/versions/v1/instances/instance-1."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"modules-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"versions-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instances-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
            ("modules-versions-instances-get",
                    Some(r##"Gets instance information."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_beta4_cli/apps_modules-versions-instances-get",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource requested. Example: apps/myapp/modules/default/versions/v1/instances/instance-1."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"modules-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"versions-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instances-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
            ("modules-versions-instances-list",
                    Some(r##"Lists the instances of a version.Tip: To aggregate details about instances over time, see the Stackdriver Monitoring API (https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.timeSeries/list)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_beta4_cli/apps_modules-versions-instances-list",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource requested. Example: apps/myapp/modules/default/versions/v1."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"modules-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"versions-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
            ("modules-versions-list",
                    Some(r##"Lists the versions of a module."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_beta4_cli/apps_modules-versions-list",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource requested. Example: apps/myapp/modules/default."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"modules-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
            ("modules-versions-patch",
                    Some(r##"Updates the specified Version resource. You can specify the following fields depending on the App Engine environment and type of scaling that the version resource uses:
        serving_status (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta4/apps.modules.versions#Version.FIELDS.serving_status):  For Version resources that use basic scaling, manual scaling, or run in  the App Engine flexible environment.
        instance_class (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta4/apps.modules.versions#Version.FIELDS.instance_class):  For Version resources that run in the App Engine standard environment.
        automatic_scaling.min_idle_instances (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta4/apps.modules.versions#Version.FIELDS.automatic_scaling):  For Version resources that use automatic scaling and run in the App  Engine standard environment.
        automatic_scaling.max_idle_instances (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta4/apps.modules.versions#Version.FIELDS.automatic_scaling):  For Version resources that use automatic scaling and run in the App  Engine standard environment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_beta4_cli/apps_modules-versions-patch",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource to update. Example: apps/myapp/modules/default/versions/1."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"modules-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"versions-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_beta4_cli/apps_operations-get",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. The name of the operation resource."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"operations-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
            ("operations-list",
                    Some(r##"Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns UNIMPLEMENTED.NOTE: the name binding allows API services to override the binding to use different resource name schemes, such as users/*/operations. To override the binding, API services can add a binding such as "/v1/{name=users/*}/operations" to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_beta4_cli/apps_operations-list",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. The name of the operation's parent resource."##),
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
                    Some(r##"Updates the specified Application resource. You can update the following fields:
        auth_domain (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta4/apps#Application.FIELDS.auth_domain)
        default_cookie_expiration (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta4/apps#Application.FIELDS.default_cookie_expiration)"##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_beta4_cli/apps_patch",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the Application resource to update. Example: apps/myapp."##),
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
    
    let mut app = App::new("appengine1-beta4")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+20181005")
           .about("The App Engine Admin API enables developers to provision and manage their App Engine applications.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_appengine1_beta4_cli")
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
