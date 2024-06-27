// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_tagmanager1::{api, Error, oauth2, client::chrono, FieldMask};


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
    hub: api::TagManager<S>,
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
    async fn _accounts_containers_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "domain-name" => Some(("domainName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "enabled-built-in-variable" => Some(("enabledBuiltInVariable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "notes" => Some(("notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "public-id" => Some(("publicId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-zone-country-id" => Some(("timeZoneCountryId", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "time-zone-id" => Some(("timeZoneId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "usage-context" => Some(("usageContext", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "container-id", "domain-name", "enabled-built-in-variable", "fingerprint", "name", "notes", "public-id", "time-zone-country-id", "time-zone-id", "usage-context"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Container = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_create(request, opt.value_of("account-id").unwrap_or(""));
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

    async fn _accounts_containers_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_delete(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""));
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

    async fn _accounts_containers_environments_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "authorization-code" => Some(("authorizationCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "authorization-timestamp-ms" => Some(("authorizationTimestampMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "container-version-id" => Some(("containerVersionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "enable-debug" => Some(("enableDebug", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "environment-id" => Some(("environmentId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "url" => Some(("url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "authorization-code", "authorization-timestamp-ms", "container-id", "container-version-id", "description", "enable-debug", "environment-id", "fingerprint", "name", "type", "url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Environment = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_environments_create(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""));
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

    async fn _accounts_containers_environments_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_environments_delete(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("environment-id").unwrap_or(""));
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

    async fn _accounts_containers_environments_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_environments_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("environment-id").unwrap_or(""));
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

    async fn _accounts_containers_environments_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_environments_list(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""));
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

    async fn _accounts_containers_environments_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "authorization-code" => Some(("authorizationCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "authorization-timestamp-ms" => Some(("authorizationTimestampMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "container-version-id" => Some(("containerVersionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "enable-debug" => Some(("enableDebug", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "environment-id" => Some(("environmentId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "url" => Some(("url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "authorization-code", "authorization-timestamp-ms", "container-id", "container-version-id", "description", "enable-debug", "environment-id", "fingerprint", "name", "type", "url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Environment = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_environments_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("environment-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
                                                                           v.extend(["fingerprint"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _accounts_containers_folders_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "folder-id" => Some(("folderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "container-id", "fingerprint", "folder-id", "name"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Folder = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_folders_create(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""));
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

    async fn _accounts_containers_folders_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_folders_delete(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("folder-id").unwrap_or(""));
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

    async fn _accounts_containers_folders_entities_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_folders_entities_list(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("folder-id").unwrap_or(""));
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

    async fn _accounts_containers_folders_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_folders_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("folder-id").unwrap_or(""));
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

    async fn _accounts_containers_folders_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_folders_list(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""));
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

    async fn _accounts_containers_folders_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "folder-id" => Some(("folderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "container-id", "fingerprint", "folder-id", "name"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Folder = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_folders_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("folder-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
                                                                           v.extend(["fingerprint"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _accounts_containers_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""));
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

    async fn _accounts_containers_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_list(opt.value_of("account-id").unwrap_or(""));
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

    async fn _accounts_containers_move_folders_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "folder-id" => Some(("folderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "container-id", "fingerprint", "folder-id", "name"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Folder = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_move_folders_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("folder-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "variable-id" => {
                    call = call.add_variable_id(value.unwrap_or(""));
                },
                "trigger-id" => {
                    call = call.add_trigger_id(value.unwrap_or(""));
                },
                "tag-id" => {
                    call = call.add_tag_id(value.unwrap_or(""));
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
                                                                           v.extend(["tag-id", "trigger-id", "variable-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
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

    async fn _accounts_containers_reauthorize_environments_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "authorization-code" => Some(("authorizationCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "authorization-timestamp-ms" => Some(("authorizationTimestampMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "container-version-id" => Some(("containerVersionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "enable-debug" => Some(("enableDebug", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "environment-id" => Some(("environmentId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "url" => Some(("url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "authorization-code", "authorization-timestamp-ms", "container-id", "container-version-id", "description", "enable-debug", "environment-id", "fingerprint", "name", "type", "url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Environment = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_reauthorize_environments_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("environment-id").unwrap_or(""));
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

    async fn _accounts_containers_tags_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "blocking-rule-id" => Some(("blockingRuleId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "blocking-trigger-id" => Some(("blockingTriggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "firing-rule-id" => Some(("firingRuleId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "firing-trigger-id" => Some(("firingTriggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "live-only" => Some(("liveOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "notes" => Some(("notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-folder-id" => Some(("parentFolderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "paused" => Some(("paused", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "priority.key" => Some(("priority.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "priority.type" => Some(("priority.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "priority.value" => Some(("priority.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "schedule-end-ms" => Some(("scheduleEndMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "schedule-start-ms" => Some(("scheduleStartMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag-firing-option" => Some(("tagFiringOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag-id" => Some(("tagId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "blocking-rule-id", "blocking-trigger-id", "container-id", "fingerprint", "firing-rule-id", "firing-trigger-id", "key", "live-only", "name", "notes", "parent-folder-id", "paused", "priority", "schedule-end-ms", "schedule-start-ms", "tag-firing-option", "tag-id", "type", "value"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Tag = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_tags_create(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""));
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

    async fn _accounts_containers_tags_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_tags_delete(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("tag-id").unwrap_or(""));
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

    async fn _accounts_containers_tags_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_tags_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("tag-id").unwrap_or(""));
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

    async fn _accounts_containers_tags_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_tags_list(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""));
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

    async fn _accounts_containers_tags_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "blocking-rule-id" => Some(("blockingRuleId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "blocking-trigger-id" => Some(("blockingTriggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "firing-rule-id" => Some(("firingRuleId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "firing-trigger-id" => Some(("firingTriggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "live-only" => Some(("liveOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "notes" => Some(("notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-folder-id" => Some(("parentFolderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "paused" => Some(("paused", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "priority.key" => Some(("priority.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "priority.type" => Some(("priority.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "priority.value" => Some(("priority.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "schedule-end-ms" => Some(("scheduleEndMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "schedule-start-ms" => Some(("scheduleStartMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag-firing-option" => Some(("tagFiringOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "tag-id" => Some(("tagId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "blocking-rule-id", "blocking-trigger-id", "container-id", "fingerprint", "firing-rule-id", "firing-trigger-id", "key", "live-only", "name", "notes", "parent-folder-id", "paused", "priority", "schedule-end-ms", "schedule-start-ms", "tag-firing-option", "tag-id", "type", "value"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Tag = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_tags_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("tag-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
                                                                           v.extend(["fingerprint"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _accounts_containers_triggers_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "check-validation.key" => Some(("checkValidation.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "check-validation.type" => Some(("checkValidation.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "check-validation.value" => Some(("checkValidation.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "continuous-time-min-milliseconds.key" => Some(("continuousTimeMinMilliseconds.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "continuous-time-min-milliseconds.type" => Some(("continuousTimeMinMilliseconds.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "continuous-time-min-milliseconds.value" => Some(("continuousTimeMinMilliseconds.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event-name.key" => Some(("eventName.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event-name.type" => Some(("eventName.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event-name.value" => Some(("eventName.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "horizontal-scroll-percentage-list.key" => Some(("horizontalScrollPercentageList.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "horizontal-scroll-percentage-list.type" => Some(("horizontalScrollPercentageList.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "horizontal-scroll-percentage-list.value" => Some(("horizontalScrollPercentageList.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "interval.key" => Some(("interval.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "interval.type" => Some(("interval.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "interval.value" => Some(("interval.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "interval-seconds.key" => Some(("intervalSeconds.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "interval-seconds.type" => Some(("intervalSeconds.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "interval-seconds.value" => Some(("intervalSeconds.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "limit.key" => Some(("limit.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "limit.type" => Some(("limit.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "limit.value" => Some(("limit.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "max-timer-length-seconds.key" => Some(("maxTimerLengthSeconds.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "max-timer-length-seconds.type" => Some(("maxTimerLengthSeconds.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "max-timer-length-seconds.value" => Some(("maxTimerLengthSeconds.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-folder-id" => Some(("parentFolderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "selector.key" => Some(("selector.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "selector.type" => Some(("selector.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "selector.value" => Some(("selector.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "total-time-min-milliseconds.key" => Some(("totalTimeMinMilliseconds.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "total-time-min-milliseconds.type" => Some(("totalTimeMinMilliseconds.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "total-time-min-milliseconds.value" => Some(("totalTimeMinMilliseconds.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger-id" => Some(("triggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "unique-trigger-id.key" => Some(("uniqueTriggerId.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "unique-trigger-id.type" => Some(("uniqueTriggerId.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "unique-trigger-id.value" => Some(("uniqueTriggerId.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vertical-scroll-percentage-list.key" => Some(("verticalScrollPercentageList.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vertical-scroll-percentage-list.type" => Some(("verticalScrollPercentageList.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vertical-scroll-percentage-list.value" => Some(("verticalScrollPercentageList.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visibility-selector.key" => Some(("visibilitySelector.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visibility-selector.type" => Some(("visibilitySelector.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visibility-selector.value" => Some(("visibilitySelector.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visible-percentage-max.key" => Some(("visiblePercentageMax.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visible-percentage-max.type" => Some(("visiblePercentageMax.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visible-percentage-max.value" => Some(("visiblePercentageMax.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visible-percentage-min.key" => Some(("visiblePercentageMin.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visible-percentage-min.type" => Some(("visiblePercentageMin.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visible-percentage-min.value" => Some(("visiblePercentageMin.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "wait-for-tags.key" => Some(("waitForTags.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "wait-for-tags.type" => Some(("waitForTags.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "wait-for-tags.value" => Some(("waitForTags.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "wait-for-tags-timeout.key" => Some(("waitForTagsTimeout.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "wait-for-tags-timeout.type" => Some(("waitForTagsTimeout.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "wait-for-tags-timeout.value" => Some(("waitForTagsTimeout.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "check-validation", "container-id", "continuous-time-min-milliseconds", "event-name", "fingerprint", "horizontal-scroll-percentage-list", "interval", "interval-seconds", "key", "limit", "max-timer-length-seconds", "name", "parent-folder-id", "selector", "total-time-min-milliseconds", "trigger-id", "type", "unique-trigger-id", "value", "vertical-scroll-percentage-list", "visibility-selector", "visible-percentage-max", "visible-percentage-min", "wait-for-tags", "wait-for-tags-timeout"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Trigger = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_triggers_create(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""));
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

    async fn _accounts_containers_triggers_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_triggers_delete(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("trigger-id").unwrap_or(""));
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

    async fn _accounts_containers_triggers_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_triggers_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("trigger-id").unwrap_or(""));
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

    async fn _accounts_containers_triggers_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_triggers_list(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""));
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

    async fn _accounts_containers_triggers_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "check-validation.key" => Some(("checkValidation.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "check-validation.type" => Some(("checkValidation.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "check-validation.value" => Some(("checkValidation.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "continuous-time-min-milliseconds.key" => Some(("continuousTimeMinMilliseconds.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "continuous-time-min-milliseconds.type" => Some(("continuousTimeMinMilliseconds.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "continuous-time-min-milliseconds.value" => Some(("continuousTimeMinMilliseconds.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event-name.key" => Some(("eventName.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event-name.type" => Some(("eventName.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event-name.value" => Some(("eventName.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "horizontal-scroll-percentage-list.key" => Some(("horizontalScrollPercentageList.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "horizontal-scroll-percentage-list.type" => Some(("horizontalScrollPercentageList.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "horizontal-scroll-percentage-list.value" => Some(("horizontalScrollPercentageList.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "interval.key" => Some(("interval.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "interval.type" => Some(("interval.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "interval.value" => Some(("interval.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "interval-seconds.key" => Some(("intervalSeconds.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "interval-seconds.type" => Some(("intervalSeconds.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "interval-seconds.value" => Some(("intervalSeconds.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "limit.key" => Some(("limit.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "limit.type" => Some(("limit.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "limit.value" => Some(("limit.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "max-timer-length-seconds.key" => Some(("maxTimerLengthSeconds.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "max-timer-length-seconds.type" => Some(("maxTimerLengthSeconds.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "max-timer-length-seconds.value" => Some(("maxTimerLengthSeconds.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-folder-id" => Some(("parentFolderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "selector.key" => Some(("selector.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "selector.type" => Some(("selector.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "selector.value" => Some(("selector.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "total-time-min-milliseconds.key" => Some(("totalTimeMinMilliseconds.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "total-time-min-milliseconds.type" => Some(("totalTimeMinMilliseconds.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "total-time-min-milliseconds.value" => Some(("totalTimeMinMilliseconds.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger-id" => Some(("triggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "unique-trigger-id.key" => Some(("uniqueTriggerId.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "unique-trigger-id.type" => Some(("uniqueTriggerId.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "unique-trigger-id.value" => Some(("uniqueTriggerId.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vertical-scroll-percentage-list.key" => Some(("verticalScrollPercentageList.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vertical-scroll-percentage-list.type" => Some(("verticalScrollPercentageList.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vertical-scroll-percentage-list.value" => Some(("verticalScrollPercentageList.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visibility-selector.key" => Some(("visibilitySelector.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visibility-selector.type" => Some(("visibilitySelector.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visibility-selector.value" => Some(("visibilitySelector.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visible-percentage-max.key" => Some(("visiblePercentageMax.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visible-percentage-max.type" => Some(("visiblePercentageMax.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visible-percentage-max.value" => Some(("visiblePercentageMax.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visible-percentage-min.key" => Some(("visiblePercentageMin.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visible-percentage-min.type" => Some(("visiblePercentageMin.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "visible-percentage-min.value" => Some(("visiblePercentageMin.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "wait-for-tags.key" => Some(("waitForTags.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "wait-for-tags.type" => Some(("waitForTags.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "wait-for-tags.value" => Some(("waitForTags.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "wait-for-tags-timeout.key" => Some(("waitForTagsTimeout.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "wait-for-tags-timeout.type" => Some(("waitForTagsTimeout.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "wait-for-tags-timeout.value" => Some(("waitForTagsTimeout.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "check-validation", "container-id", "continuous-time-min-milliseconds", "event-name", "fingerprint", "horizontal-scroll-percentage-list", "interval", "interval-seconds", "key", "limit", "max-timer-length-seconds", "name", "parent-folder-id", "selector", "total-time-min-milliseconds", "trigger-id", "type", "unique-trigger-id", "value", "vertical-scroll-percentage-list", "visibility-selector", "visible-percentage-max", "visible-percentage-min", "wait-for-tags", "wait-for-tags-timeout"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Trigger = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_triggers_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("trigger-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
                                                                           v.extend(["fingerprint"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _accounts_containers_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "domain-name" => Some(("domainName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "enabled-built-in-variable" => Some(("enabledBuiltInVariable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "notes" => Some(("notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "public-id" => Some(("publicId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-zone-country-id" => Some(("timeZoneCountryId", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "time-zone-id" => Some(("timeZoneId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "usage-context" => Some(("usageContext", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "container-id", "domain-name", "enabled-built-in-variable", "fingerprint", "name", "notes", "public-id", "time-zone-country-id", "time-zone-id", "usage-context"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Container = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
                                                                           v.extend(["fingerprint"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _accounts_containers_variables_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "disabling-trigger-id" => Some(("disablingTriggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "enabling-trigger-id" => Some(("enablingTriggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "notes" => Some(("notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-folder-id" => Some(("parentFolderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "schedule-end-ms" => Some(("scheduleEndMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "schedule-start-ms" => Some(("scheduleStartMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "variable-id" => Some(("variableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "container-id", "disabling-trigger-id", "enabling-trigger-id", "fingerprint", "name", "notes", "parent-folder-id", "schedule-end-ms", "schedule-start-ms", "type", "variable-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Variable = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_variables_create(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""));
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

    async fn _accounts_containers_variables_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_variables_delete(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("variable-id").unwrap_or(""));
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

    async fn _accounts_containers_variables_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_variables_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("variable-id").unwrap_or(""));
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

    async fn _accounts_containers_variables_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_variables_list(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""));
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

    async fn _accounts_containers_variables_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "disabling-trigger-id" => Some(("disablingTriggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "enabling-trigger-id" => Some(("enablingTriggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "notes" => Some(("notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent-folder-id" => Some(("parentFolderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "schedule-end-ms" => Some(("scheduleEndMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "schedule-start-ms" => Some(("scheduleStartMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "variable-id" => Some(("variableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "container-id", "disabling-trigger-id", "enabling-trigger-id", "fingerprint", "name", "notes", "parent-folder-id", "schedule-end-ms", "schedule-start-ms", "type", "variable-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Variable = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_variables_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("variable-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
                                                                           v.extend(["fingerprint"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _accounts_containers_versions_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "notes" => Some(("notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "quick-preview" => Some(("quickPreview", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["name", "notes", "quick-preview"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CreateContainerVersionRequestVersionOptions = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_versions_create(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""));
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

    async fn _accounts_containers_versions_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_versions_delete(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("container-version-id").unwrap_or(""));
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

    async fn _accounts_containers_versions_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_versions_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("container-version-id").unwrap_or(""));
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

    async fn _accounts_containers_versions_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_versions_list(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "include-deleted" => {
                    call = call.include_deleted(        value.map(|v| arg_from_str(v, err, "include-deleted", "boolean")).unwrap_or(false));
                },
                "headers" => {
                    call = call.headers(        value.map(|v| arg_from_str(v, err, "headers", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["headers", "include-deleted"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _accounts_containers_versions_publish(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_versions_publish(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("container-version-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
                                                                           v.extend(["fingerprint"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _accounts_containers_versions_restore(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_versions_restore(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("container-version-id").unwrap_or(""));
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

    async fn _accounts_containers_versions_undelete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_versions_undelete(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("container-version-id").unwrap_or(""));
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

    async fn _accounts_containers_versions_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "container.account-id" => Some(("container.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "container.container-id" => Some(("container.containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "container.domain-name" => Some(("container.domainName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "container.enabled-built-in-variable" => Some(("container.enabledBuiltInVariable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "container.fingerprint" => Some(("container.fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "container.name" => Some(("container.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "container.notes" => Some(("container.notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "container.public-id" => Some(("container.publicId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "container.time-zone-country-id" => Some(("container.timeZoneCountryId", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "container.time-zone-id" => Some(("container.timeZoneId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "container.usage-context" => Some(("container.usageContext", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "container-id" => Some(("containerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "container-version-id" => Some(("containerVersionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deleted" => Some(("deleted", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "notes" => Some(("notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "container", "container-id", "container-version-id", "deleted", "domain-name", "enabled-built-in-variable", "fingerprint", "name", "notes", "public-id", "time-zone-country-id", "time-zone-id", "usage-context"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ContainerVersion = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().containers_versions_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("container-version-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
                                                                           v.extend(["fingerprint"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _accounts_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().get(opt.value_of("account-id").unwrap_or(""));
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

    async fn _accounts_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().list();
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

    async fn _accounts_permissions_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "account-access.permission" => Some(("accountAccess.permission", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "account-id" => Some(("accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "email-address" => Some(("emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "permission-id" => Some(("permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-access", "account-id", "email-address", "permission", "permission-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::UserAccess = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().permissions_create(request, opt.value_of("account-id").unwrap_or(""));
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

    async fn _accounts_permissions_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().permissions_delete(opt.value_of("account-id").unwrap_or(""), opt.value_of("permission-id").unwrap_or(""));
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

    async fn _accounts_permissions_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().permissions_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("permission-id").unwrap_or(""));
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

    async fn _accounts_permissions_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().permissions_list(opt.value_of("account-id").unwrap_or(""));
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

    async fn _accounts_permissions_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "account-access.permission" => Some(("accountAccess.permission", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "account-id" => Some(("accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "email-address" => Some(("emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "permission-id" => Some(("permissionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-access", "account-id", "email-address", "permission", "permission-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::UserAccess = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().permissions_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("permission-id").unwrap_or(""));
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

    async fn _accounts_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "fingerprint" => Some(("fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "share-data" => Some(("shareData", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "fingerprint", "name", "share-data"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Account = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().update(request, opt.value_of("account-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
                                                                           v.extend(["fingerprint"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
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
            ("accounts", Some(opt)) => {
                match opt.subcommand() {
                    ("containers-create", Some(opt)) => {
                        call_result = self._accounts_containers_create(opt, dry_run, &mut err).await;
                    },
                    ("containers-delete", Some(opt)) => {
                        call_result = self._accounts_containers_delete(opt, dry_run, &mut err).await;
                    },
                    ("containers-environments-create", Some(opt)) => {
                        call_result = self._accounts_containers_environments_create(opt, dry_run, &mut err).await;
                    },
                    ("containers-environments-delete", Some(opt)) => {
                        call_result = self._accounts_containers_environments_delete(opt, dry_run, &mut err).await;
                    },
                    ("containers-environments-get", Some(opt)) => {
                        call_result = self._accounts_containers_environments_get(opt, dry_run, &mut err).await;
                    },
                    ("containers-environments-list", Some(opt)) => {
                        call_result = self._accounts_containers_environments_list(opt, dry_run, &mut err).await;
                    },
                    ("containers-environments-update", Some(opt)) => {
                        call_result = self._accounts_containers_environments_update(opt, dry_run, &mut err).await;
                    },
                    ("containers-folders-create", Some(opt)) => {
                        call_result = self._accounts_containers_folders_create(opt, dry_run, &mut err).await;
                    },
                    ("containers-folders-delete", Some(opt)) => {
                        call_result = self._accounts_containers_folders_delete(opt, dry_run, &mut err).await;
                    },
                    ("containers-folders-entities-list", Some(opt)) => {
                        call_result = self._accounts_containers_folders_entities_list(opt, dry_run, &mut err).await;
                    },
                    ("containers-folders-get", Some(opt)) => {
                        call_result = self._accounts_containers_folders_get(opt, dry_run, &mut err).await;
                    },
                    ("containers-folders-list", Some(opt)) => {
                        call_result = self._accounts_containers_folders_list(opt, dry_run, &mut err).await;
                    },
                    ("containers-folders-update", Some(opt)) => {
                        call_result = self._accounts_containers_folders_update(opt, dry_run, &mut err).await;
                    },
                    ("containers-get", Some(opt)) => {
                        call_result = self._accounts_containers_get(opt, dry_run, &mut err).await;
                    },
                    ("containers-list", Some(opt)) => {
                        call_result = self._accounts_containers_list(opt, dry_run, &mut err).await;
                    },
                    ("containers-move-folders-update", Some(opt)) => {
                        call_result = self._accounts_containers_move_folders_update(opt, dry_run, &mut err).await;
                    },
                    ("containers-reauthorize-environments-update", Some(opt)) => {
                        call_result = self._accounts_containers_reauthorize_environments_update(opt, dry_run, &mut err).await;
                    },
                    ("containers-tags-create", Some(opt)) => {
                        call_result = self._accounts_containers_tags_create(opt, dry_run, &mut err).await;
                    },
                    ("containers-tags-delete", Some(opt)) => {
                        call_result = self._accounts_containers_tags_delete(opt, dry_run, &mut err).await;
                    },
                    ("containers-tags-get", Some(opt)) => {
                        call_result = self._accounts_containers_tags_get(opt, dry_run, &mut err).await;
                    },
                    ("containers-tags-list", Some(opt)) => {
                        call_result = self._accounts_containers_tags_list(opt, dry_run, &mut err).await;
                    },
                    ("containers-tags-update", Some(opt)) => {
                        call_result = self._accounts_containers_tags_update(opt, dry_run, &mut err).await;
                    },
                    ("containers-triggers-create", Some(opt)) => {
                        call_result = self._accounts_containers_triggers_create(opt, dry_run, &mut err).await;
                    },
                    ("containers-triggers-delete", Some(opt)) => {
                        call_result = self._accounts_containers_triggers_delete(opt, dry_run, &mut err).await;
                    },
                    ("containers-triggers-get", Some(opt)) => {
                        call_result = self._accounts_containers_triggers_get(opt, dry_run, &mut err).await;
                    },
                    ("containers-triggers-list", Some(opt)) => {
                        call_result = self._accounts_containers_triggers_list(opt, dry_run, &mut err).await;
                    },
                    ("containers-triggers-update", Some(opt)) => {
                        call_result = self._accounts_containers_triggers_update(opt, dry_run, &mut err).await;
                    },
                    ("containers-update", Some(opt)) => {
                        call_result = self._accounts_containers_update(opt, dry_run, &mut err).await;
                    },
                    ("containers-variables-create", Some(opt)) => {
                        call_result = self._accounts_containers_variables_create(opt, dry_run, &mut err).await;
                    },
                    ("containers-variables-delete", Some(opt)) => {
                        call_result = self._accounts_containers_variables_delete(opt, dry_run, &mut err).await;
                    },
                    ("containers-variables-get", Some(opt)) => {
                        call_result = self._accounts_containers_variables_get(opt, dry_run, &mut err).await;
                    },
                    ("containers-variables-list", Some(opt)) => {
                        call_result = self._accounts_containers_variables_list(opt, dry_run, &mut err).await;
                    },
                    ("containers-variables-update", Some(opt)) => {
                        call_result = self._accounts_containers_variables_update(opt, dry_run, &mut err).await;
                    },
                    ("containers-versions-create", Some(opt)) => {
                        call_result = self._accounts_containers_versions_create(opt, dry_run, &mut err).await;
                    },
                    ("containers-versions-delete", Some(opt)) => {
                        call_result = self._accounts_containers_versions_delete(opt, dry_run, &mut err).await;
                    },
                    ("containers-versions-get", Some(opt)) => {
                        call_result = self._accounts_containers_versions_get(opt, dry_run, &mut err).await;
                    },
                    ("containers-versions-list", Some(opt)) => {
                        call_result = self._accounts_containers_versions_list(opt, dry_run, &mut err).await;
                    },
                    ("containers-versions-publish", Some(opt)) => {
                        call_result = self._accounts_containers_versions_publish(opt, dry_run, &mut err).await;
                    },
                    ("containers-versions-restore", Some(opt)) => {
                        call_result = self._accounts_containers_versions_restore(opt, dry_run, &mut err).await;
                    },
                    ("containers-versions-undelete", Some(opt)) => {
                        call_result = self._accounts_containers_versions_undelete(opt, dry_run, &mut err).await;
                    },
                    ("containers-versions-update", Some(opt)) => {
                        call_result = self._accounts_containers_versions_update(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._accounts_get(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._accounts_list(opt, dry_run, &mut err).await;
                    },
                    ("permissions-create", Some(opt)) => {
                        call_result = self._accounts_permissions_create(opt, dry_run, &mut err).await;
                    },
                    ("permissions-delete", Some(opt)) => {
                        call_result = self._accounts_permissions_delete(opt, dry_run, &mut err).await;
                    },
                    ("permissions-get", Some(opt)) => {
                        call_result = self._accounts_permissions_get(opt, dry_run, &mut err).await;
                    },
                    ("permissions-list", Some(opt)) => {
                        call_result = self._accounts_permissions_list(opt, dry_run, &mut err).await;
                    },
                    ("permissions-update", Some(opt)) => {
                        call_result = self._accounts_permissions_update(opt, dry_run, &mut err).await;
                    },
                    ("update", Some(opt)) => {
                        call_result = self._accounts_update(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("accounts".to_string()));
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

            match client::application_secret_from_directory(&config_dir, "tagmanager1-secret.json",
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
        ).persist_tokens_to_disk(format!("{}/tagmanager1", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::TagManager::new(client, auth),
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
        ("accounts", "methods: 'containers-create', 'containers-delete', 'containers-environments-create', 'containers-environments-delete', 'containers-environments-get', 'containers-environments-list', 'containers-environments-update', 'containers-folders-create', 'containers-folders-delete', 'containers-folders-entities-list', 'containers-folders-get', 'containers-folders-list', 'containers-folders-update', 'containers-get', 'containers-list', 'containers-move-folders-update', 'containers-reauthorize-environments-update', 'containers-tags-create', 'containers-tags-delete', 'containers-tags-get', 'containers-tags-list', 'containers-tags-update', 'containers-triggers-create', 'containers-triggers-delete', 'containers-triggers-get', 'containers-triggers-list', 'containers-triggers-update', 'containers-update', 'containers-variables-create', 'containers-variables-delete', 'containers-variables-get', 'containers-variables-list', 'containers-variables-update', 'containers-versions-create', 'containers-versions-delete', 'containers-versions-get', 'containers-versions-list', 'containers-versions-publish', 'containers-versions-restore', 'containers-versions-undelete', 'containers-versions-update', 'get', 'list', 'permissions-create', 'permissions-delete', 'permissions-get', 'permissions-list', 'permissions-update' and 'update'", vec![
            ("containers-create",
                    Some(r##"Creates a Container."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-create",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
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
            ("containers-delete",
                    Some(r##"Deletes a Container."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-delete",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-id"##),
                     None,
                     Some(r##"The GTM Container ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("containers-environments-create",
                    Some(r##"Creates a GTM Environment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-environments-create",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-id"##),
                     None,
                     Some(r##"The GTM Container ID."##),
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
            ("containers-environments-delete",
                    Some(r##"Deletes a GTM Environment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-environments-delete",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-id"##),
                     None,
                     Some(r##"The GTM Container ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"environment-id"##),
                     None,
                     Some(r##"The GTM Environment ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("containers-environments-get",
                    Some(r##"Gets a GTM Environment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-environments-get",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-id"##),
                     None,
                     Some(r##"The GTM Container ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"environment-id"##),
                     None,
                     Some(r##"The GTM Environment ID."##),
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
            ("containers-environments-list",
                    Some(r##"Lists all GTM Environments of a GTM Container."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-environments-list",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-id"##),
                     None,
                     Some(r##"The GTM Container ID."##),
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
            ("containers-environments-update",
                    Some(r##"Updates a GTM Environment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-environments-update",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-id"##),
                     None,
                     Some(r##"The GTM Container ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"environment-id"##),
                     None,
                     Some(r##"The GTM Environment ID."##),
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
            ("containers-folders-create",
                    Some(r##"Creates a GTM Folder."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-folders-create",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-id"##),
                     None,
                     Some(r##"The GTM Container ID."##),
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
            ("containers-folders-delete",
                    Some(r##"Deletes a GTM Folder."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-folders-delete",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-id"##),
                     None,
                     Some(r##"The GTM Container ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"folder-id"##),
                     None,
                     Some(r##"The GTM Folder ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("containers-folders-entities-list",
                    Some(r##"List all entities in a GTM Folder."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-folders-entities-list",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-id"##),
                     None,
                     Some(r##"The GTM Container ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"folder-id"##),
                     None,
                     Some(r##"The GTM Folder ID."##),
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
            ("containers-folders-get",
                    Some(r##"Gets a GTM Folder."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-folders-get",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-id"##),
                     None,
                     Some(r##"The GTM Container ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"folder-id"##),
                     None,
                     Some(r##"The GTM Folder ID."##),
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
            ("containers-folders-list",
                    Some(r##"Lists all GTM Folders of a Container."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-folders-list",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-id"##),
                     None,
                     Some(r##"The GTM Container ID."##),
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
            ("containers-folders-update",
                    Some(r##"Updates a GTM Folder."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-folders-update",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-id"##),
                     None,
                     Some(r##"The GTM Container ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"folder-id"##),
                     None,
                     Some(r##"The GTM Folder ID."##),
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
            ("containers-get",
                    Some(r##"Gets a Container."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-get",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-id"##),
                     None,
                     Some(r##"The GTM Container ID."##),
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
            ("containers-list",
                    Some(r##"Lists all Containers that belongs to a GTM Account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-list",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
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
            ("containers-move-folders-update",
                    Some(r##"Moves entities to a GTM Folder."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-move-folders-update",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-id"##),
                     None,
                     Some(r##"The GTM Container ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"folder-id"##),
                     None,
                     Some(r##"The GTM Folder ID."##),
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
            ("containers-reauthorize-environments-update",
                    Some(r##"Re-generates the authorization code for a GTM Environment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-reauthorize-environments-update",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-id"##),
                     None,
                     Some(r##"The GTM Container ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"environment-id"##),
                     None,
                     Some(r##"The GTM Environment ID."##),
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
            ("containers-tags-create",
                    Some(r##"Creates a GTM Tag."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-tags-create",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-id"##),
                     None,
                     Some(r##"The GTM Container ID."##),
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
            ("containers-tags-delete",
                    Some(r##"Deletes a GTM Tag."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-tags-delete",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-id"##),
                     None,
                     Some(r##"The GTM Container ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"tag-id"##),
                     None,
                     Some(r##"The GTM Tag ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("containers-tags-get",
                    Some(r##"Gets a GTM Tag."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-tags-get",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-id"##),
                     None,
                     Some(r##"The GTM Container ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"tag-id"##),
                     None,
                     Some(r##"The GTM Tag ID."##),
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
            ("containers-tags-list",
                    Some(r##"Lists all GTM Tags of a Container."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-tags-list",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-id"##),
                     None,
                     Some(r##"The GTM Container ID."##),
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
            ("containers-tags-update",
                    Some(r##"Updates a GTM Tag."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-tags-update",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-id"##),
                     None,
                     Some(r##"The GTM Container ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"tag-id"##),
                     None,
                     Some(r##"The GTM Tag ID."##),
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
            ("containers-triggers-create",
                    Some(r##"Creates a GTM Trigger."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-triggers-create",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-id"##),
                     None,
                     Some(r##"The GTM Container ID."##),
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
            ("containers-triggers-delete",
                    Some(r##"Deletes a GTM Trigger."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-triggers-delete",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-id"##),
                     None,
                     Some(r##"The GTM Container ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"trigger-id"##),
                     None,
                     Some(r##"The GTM Trigger ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("containers-triggers-get",
                    Some(r##"Gets a GTM Trigger."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-triggers-get",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-id"##),
                     None,
                     Some(r##"The GTM Container ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"trigger-id"##),
                     None,
                     Some(r##"The GTM Trigger ID."##),
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
            ("containers-triggers-list",
                    Some(r##"Lists all GTM Triggers of a Container."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-triggers-list",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-id"##),
                     None,
                     Some(r##"The GTM Container ID."##),
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
            ("containers-triggers-update",
                    Some(r##"Updates a GTM Trigger."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-triggers-update",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-id"##),
                     None,
                     Some(r##"The GTM Container ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"trigger-id"##),
                     None,
                     Some(r##"The GTM Trigger ID."##),
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
            ("containers-update",
                    Some(r##"Updates a Container."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-update",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-id"##),
                     None,
                     Some(r##"The GTM Container ID."##),
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
            ("containers-variables-create",
                    Some(r##"Creates a GTM Variable."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-variables-create",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-id"##),
                     None,
                     Some(r##"The GTM Container ID."##),
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
            ("containers-variables-delete",
                    Some(r##"Deletes a GTM Variable."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-variables-delete",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-id"##),
                     None,
                     Some(r##"The GTM Container ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"variable-id"##),
                     None,
                     Some(r##"The GTM Variable ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("containers-variables-get",
                    Some(r##"Gets a GTM Variable."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-variables-get",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-id"##),
                     None,
                     Some(r##"The GTM Container ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"variable-id"##),
                     None,
                     Some(r##"The GTM Variable ID."##),
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
            ("containers-variables-list",
                    Some(r##"Lists all GTM Variables of a Container."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-variables-list",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-id"##),
                     None,
                     Some(r##"The GTM Container ID."##),
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
            ("containers-variables-update",
                    Some(r##"Updates a GTM Variable."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-variables-update",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-id"##),
                     None,
                     Some(r##"The GTM Container ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"variable-id"##),
                     None,
                     Some(r##"The GTM Variable ID."##),
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
            ("containers-versions-create",
                    Some(r##"Creates a Container Version."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-versions-create",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-id"##),
                     None,
                     Some(r##"The GTM Container ID."##),
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
            ("containers-versions-delete",
                    Some(r##"Deletes a Container Version."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-versions-delete",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-id"##),
                     None,
                     Some(r##"The GTM Container ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-version-id"##),
                     None,
                     Some(r##"The GTM Container Version ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("containers-versions-get",
                    Some(r##"Gets a Container Version."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-versions-get",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-id"##),
                     None,
                     Some(r##"The GTM Container ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-version-id"##),
                     None,
                     Some(r##"The GTM Container Version ID. Specify published to retrieve the currently published version."##),
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
            ("containers-versions-list",
                    Some(r##"Lists all Container Versions of a GTM Container."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-versions-list",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-id"##),
                     None,
                     Some(r##"The GTM Container ID."##),
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
            ("containers-versions-publish",
                    Some(r##"Publishes a Container Version."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-versions-publish",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-id"##),
                     None,
                     Some(r##"The GTM Container ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-version-id"##),
                     None,
                     Some(r##"The GTM Container Version ID."##),
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
            ("containers-versions-restore",
                    Some(r##"Restores a Container Version. This will overwrite the container's current configuration (including its variables, triggers and tags). The operation will not have any effect on the version that is being served (i.e. the published version)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-versions-restore",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-id"##),
                     None,
                     Some(r##"The GTM Container ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-version-id"##),
                     None,
                     Some(r##"The GTM Container Version ID."##),
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
            ("containers-versions-undelete",
                    Some(r##"Undeletes a Container Version."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-versions-undelete",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-id"##),
                     None,
                     Some(r##"The GTM Container ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-version-id"##),
                     None,
                     Some(r##"The GTM Container Version ID."##),
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
            ("containers-versions-update",
                    Some(r##"Updates a Container Version."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-versions-update",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-id"##),
                     None,
                     Some(r##"The GTM Container ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"container-version-id"##),
                     None,
                     Some(r##"The GTM Container Version ID."##),
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
                    Some(r##"Gets a GTM Account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_get",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
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
                    Some(r##"Lists all GTM Accounts that a user has access to."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_list",
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
            ("permissions-create",
                    Some(r##"Creates a user's Account & Container Permissions."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_permissions-create",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
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
            ("permissions-delete",
                    Some(r##"Removes a user from the account, revoking access to it and all of its containers."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_permissions-delete",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"permission-id"##),
                     None,
                     Some(r##"The GTM User ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("permissions-get",
                    Some(r##"Gets a user's Account & Container Permissions."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_permissions-get",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"permission-id"##),
                     None,
                     Some(r##"The GTM User ID."##),
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
            ("permissions-list",
                    Some(r##"List all users that have access to the account along with Account and Container Permissions granted to each of them."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_permissions-list",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
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
            ("permissions-update",
                    Some(r##"Updates a user's Account & Container Permissions."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_permissions-update",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"permission-id"##),
                     None,
                     Some(r##"The GTM User ID."##),
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
                    Some(r##"Updates a GTM Account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_update",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The GTM Account ID."##),
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
    
    let mut app = App::new("tagmanager1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+20240619")
           .about("This API allows clients to access and modify container and tag configuration.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_tagmanager1_cli")
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
