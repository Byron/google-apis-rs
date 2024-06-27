// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_dlp2::{api, Error, oauth2, client::chrono, FieldMask};


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
    hub: api::DLP<S>,
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
    async fn _info_types_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.info_types().list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "parent" => {
                    call = call.parent(value.unwrap_or(""));
                },
                "location-id" => {
                    call = call.location_id(value.unwrap_or(""));
                },
                "language-code" => {
                    call = call.language_code(value.unwrap_or(""));
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
                                                                           v.extend(["filter", "language-code", "location-id", "parent"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _locations_info_types_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.locations().info_types_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "location-id" => {
                    call = call.location_id(value.unwrap_or(""));
                },
                "language-code" => {
                    call = call.language_code(value.unwrap_or(""));
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
                                                                           v.extend(["filter", "language-code", "location-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _organizations_deidentify_templates_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "deidentify-template.create-time" => Some(("deidentifyTemplate.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.description" => Some(("deidentifyTemplate.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.display-name" => Some(("deidentifyTemplate.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.name" => Some(("deidentifyTemplate.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.update-time" => Some(("deidentifyTemplate.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-id" => Some(("locationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "template-id" => Some(("templateId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["create-time", "deidentify-template", "description", "display-name", "location-id", "name", "template-id", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2CreateDeidentifyTemplateRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.organizations().deidentify_templates_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _organizations_deidentify_templates_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().deidentify_templates_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _organizations_deidentify_templates_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().deidentify_templates_get(opt.value_of("name").unwrap_or(""));
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

    async fn _organizations_deidentify_templates_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().deidentify_templates_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "location-id" => {
                    call = call.location_id(value.unwrap_or(""));
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
                                                                           v.extend(["location-id", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _organizations_deidentify_templates_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "deidentify-template.create-time" => Some(("deidentifyTemplate.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.description" => Some(("deidentifyTemplate.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.display-name" => Some(("deidentifyTemplate.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.name" => Some(("deidentifyTemplate.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.update-time" => Some(("deidentifyTemplate.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-mask" => Some(("updateMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["create-time", "deidentify-template", "description", "display-name", "name", "update-mask", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2UpdateDeidentifyTemplateRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.organizations().deidentify_templates_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _organizations_inspect_templates_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "inspect-template.create-time" => Some(("inspectTemplate.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.description" => Some(("inspectTemplate.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.display-name" => Some(("inspectTemplate.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.content-options" => Some(("inspectTemplate.inspectConfig.contentOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inspect-template.inspect-config.exclude-info-types" => Some(("inspectTemplate.inspectConfig.excludeInfoTypes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.include-quote" => Some(("inspectTemplate.inspectConfig.includeQuote", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.limits.max-findings-per-item" => Some(("inspectTemplate.inspectConfig.limits.maxFindingsPerItem", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.limits.max-findings-per-request" => Some(("inspectTemplate.inspectConfig.limits.maxFindingsPerRequest", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.min-likelihood" => Some(("inspectTemplate.inspectConfig.minLikelihood", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.name" => Some(("inspectTemplate.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.update-time" => Some(("inspectTemplate.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-id" => Some(("locationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "template-id" => Some(("templateId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["content-options", "create-time", "description", "display-name", "exclude-info-types", "include-quote", "inspect-config", "inspect-template", "limits", "location-id", "max-findings-per-item", "max-findings-per-request", "min-likelihood", "name", "template-id", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2CreateInspectTemplateRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.organizations().inspect_templates_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _organizations_inspect_templates_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().inspect_templates_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _organizations_inspect_templates_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().inspect_templates_get(opt.value_of("name").unwrap_or(""));
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

    async fn _organizations_inspect_templates_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().inspect_templates_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "location-id" => {
                    call = call.location_id(value.unwrap_or(""));
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
                                                                           v.extend(["location-id", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _organizations_inspect_templates_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "inspect-template.create-time" => Some(("inspectTemplate.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.description" => Some(("inspectTemplate.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.display-name" => Some(("inspectTemplate.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.content-options" => Some(("inspectTemplate.inspectConfig.contentOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inspect-template.inspect-config.exclude-info-types" => Some(("inspectTemplate.inspectConfig.excludeInfoTypes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.include-quote" => Some(("inspectTemplate.inspectConfig.includeQuote", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.limits.max-findings-per-item" => Some(("inspectTemplate.inspectConfig.limits.maxFindingsPerItem", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.limits.max-findings-per-request" => Some(("inspectTemplate.inspectConfig.limits.maxFindingsPerRequest", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.min-likelihood" => Some(("inspectTemplate.inspectConfig.minLikelihood", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.name" => Some(("inspectTemplate.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.update-time" => Some(("inspectTemplate.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-mask" => Some(("updateMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["content-options", "create-time", "description", "display-name", "exclude-info-types", "include-quote", "inspect-config", "inspect-template", "limits", "max-findings-per-item", "max-findings-per-request", "min-likelihood", "name", "update-mask", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2UpdateInspectTemplateRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.organizations().inspect_templates_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _organizations_locations_column_data_profiles_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().locations_column_data_profiles_get(opt.value_of("name").unwrap_or(""));
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

    async fn _organizations_locations_column_data_profiles_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().locations_column_data_profiles_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
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
                                                                           v.extend(["filter", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _organizations_locations_connections_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "connection.cloud-sql.connection-name" => Some(("connection.cloudSql.connectionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "connection.cloud-sql.database-engine" => Some(("connection.cloudSql.databaseEngine", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "connection.cloud-sql.max-connections" => Some(("connection.cloudSql.maxConnections", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "connection.cloud-sql.username-password.password-secret-version-name" => Some(("connection.cloudSql.usernamePassword.passwordSecretVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "connection.cloud-sql.username-password.username" => Some(("connection.cloudSql.usernamePassword.username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "connection.name" => Some(("connection.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "connection.state" => Some(("connection.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cloud-sql", "connection", "connection-name", "database-engine", "max-connections", "name", "password-secret-version-name", "state", "username", "username-password"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2CreateConnectionRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.organizations().locations_connections_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _organizations_locations_connections_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().locations_connections_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _organizations_locations_connections_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().locations_connections_get(opt.value_of("name").unwrap_or(""));
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

    async fn _organizations_locations_connections_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "connection.cloud-sql.connection-name" => Some(("connection.cloudSql.connectionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "connection.cloud-sql.database-engine" => Some(("connection.cloudSql.databaseEngine", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "connection.cloud-sql.max-connections" => Some(("connection.cloudSql.maxConnections", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "connection.cloud-sql.username-password.password-secret-version-name" => Some(("connection.cloudSql.usernamePassword.passwordSecretVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "connection.cloud-sql.username-password.username" => Some(("connection.cloudSql.usernamePassword.username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "connection.name" => Some(("connection.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "connection.state" => Some(("connection.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-mask" => Some(("updateMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cloud-sql", "connection", "connection-name", "database-engine", "max-connections", "name", "password-secret-version-name", "state", "update-mask", "username", "username-password"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2UpdateConnectionRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.organizations().locations_connections_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _organizations_locations_connections_search(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().locations_connections_search(opt.value_of("parent").unwrap_or(""));
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

    async fn _organizations_locations_deidentify_templates_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "deidentify-template.create-time" => Some(("deidentifyTemplate.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.description" => Some(("deidentifyTemplate.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.display-name" => Some(("deidentifyTemplate.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.name" => Some(("deidentifyTemplate.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.update-time" => Some(("deidentifyTemplate.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-id" => Some(("locationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "template-id" => Some(("templateId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["create-time", "deidentify-template", "description", "display-name", "location-id", "name", "template-id", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2CreateDeidentifyTemplateRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.organizations().locations_deidentify_templates_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _organizations_locations_deidentify_templates_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().locations_deidentify_templates_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _organizations_locations_deidentify_templates_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().locations_deidentify_templates_get(opt.value_of("name").unwrap_or(""));
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

    async fn _organizations_locations_deidentify_templates_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().locations_deidentify_templates_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "location-id" => {
                    call = call.location_id(value.unwrap_or(""));
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
                                                                           v.extend(["location-id", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _organizations_locations_deidentify_templates_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "deidentify-template.create-time" => Some(("deidentifyTemplate.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.description" => Some(("deidentifyTemplate.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.display-name" => Some(("deidentifyTemplate.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.name" => Some(("deidentifyTemplate.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.update-time" => Some(("deidentifyTemplate.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-mask" => Some(("updateMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["create-time", "deidentify-template", "description", "display-name", "name", "update-mask", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2UpdateDeidentifyTemplateRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.organizations().locations_deidentify_templates_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _organizations_locations_discovery_configs_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "config-id" => Some(("configId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery-config.create-time" => Some(("discoveryConfig.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery-config.display-name" => Some(("discoveryConfig.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery-config.inspect-templates" => Some(("discoveryConfig.inspectTemplates", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "discovery-config.last-run-time" => Some(("discoveryConfig.lastRunTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery-config.name" => Some(("discoveryConfig.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery-config.org-config.location.folder-id" => Some(("discoveryConfig.orgConfig.location.folderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery-config.org-config.location.organization-id" => Some(("discoveryConfig.orgConfig.location.organizationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery-config.org-config.project-id" => Some(("discoveryConfig.orgConfig.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery-config.status" => Some(("discoveryConfig.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery-config.update-time" => Some(("discoveryConfig.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["config-id", "create-time", "discovery-config", "display-name", "folder-id", "inspect-templates", "last-run-time", "location", "name", "org-config", "organization-id", "project-id", "status", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2CreateDiscoveryConfigRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.organizations().locations_discovery_configs_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _organizations_locations_discovery_configs_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().locations_discovery_configs_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _organizations_locations_discovery_configs_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().locations_discovery_configs_get(opt.value_of("name").unwrap_or(""));
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

    async fn _organizations_locations_discovery_configs_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().locations_discovery_configs_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
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
                                                                           v.extend(["order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _organizations_locations_discovery_configs_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "discovery-config.create-time" => Some(("discoveryConfig.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery-config.display-name" => Some(("discoveryConfig.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery-config.inspect-templates" => Some(("discoveryConfig.inspectTemplates", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "discovery-config.last-run-time" => Some(("discoveryConfig.lastRunTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery-config.name" => Some(("discoveryConfig.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery-config.org-config.location.folder-id" => Some(("discoveryConfig.orgConfig.location.folderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery-config.org-config.location.organization-id" => Some(("discoveryConfig.orgConfig.location.organizationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery-config.org-config.project-id" => Some(("discoveryConfig.orgConfig.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery-config.status" => Some(("discoveryConfig.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery-config.update-time" => Some(("discoveryConfig.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-mask" => Some(("updateMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["create-time", "discovery-config", "display-name", "folder-id", "inspect-templates", "last-run-time", "location", "name", "org-config", "organization-id", "project-id", "status", "update-mask", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2UpdateDiscoveryConfigRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.organizations().locations_discovery_configs_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _organizations_locations_dlp_jobs_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().locations_dlp_jobs_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "type" => {
                    call = call.type_(value.unwrap_or(""));
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
                "location-id" => {
                    call = call.location_id(value.unwrap_or(""));
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
                                                                           v.extend(["filter", "location-id", "order-by", "page-size", "page-token", "type"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _organizations_locations_inspect_templates_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "inspect-template.create-time" => Some(("inspectTemplate.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.description" => Some(("inspectTemplate.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.display-name" => Some(("inspectTemplate.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.content-options" => Some(("inspectTemplate.inspectConfig.contentOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inspect-template.inspect-config.exclude-info-types" => Some(("inspectTemplate.inspectConfig.excludeInfoTypes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.include-quote" => Some(("inspectTemplate.inspectConfig.includeQuote", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.limits.max-findings-per-item" => Some(("inspectTemplate.inspectConfig.limits.maxFindingsPerItem", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.limits.max-findings-per-request" => Some(("inspectTemplate.inspectConfig.limits.maxFindingsPerRequest", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.min-likelihood" => Some(("inspectTemplate.inspectConfig.minLikelihood", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.name" => Some(("inspectTemplate.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.update-time" => Some(("inspectTemplate.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-id" => Some(("locationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "template-id" => Some(("templateId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["content-options", "create-time", "description", "display-name", "exclude-info-types", "include-quote", "inspect-config", "inspect-template", "limits", "location-id", "max-findings-per-item", "max-findings-per-request", "min-likelihood", "name", "template-id", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2CreateInspectTemplateRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.organizations().locations_inspect_templates_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _organizations_locations_inspect_templates_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().locations_inspect_templates_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _organizations_locations_inspect_templates_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().locations_inspect_templates_get(opt.value_of("name").unwrap_or(""));
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

    async fn _organizations_locations_inspect_templates_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().locations_inspect_templates_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "location-id" => {
                    call = call.location_id(value.unwrap_or(""));
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
                                                                           v.extend(["location-id", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _organizations_locations_inspect_templates_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "inspect-template.create-time" => Some(("inspectTemplate.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.description" => Some(("inspectTemplate.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.display-name" => Some(("inspectTemplate.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.content-options" => Some(("inspectTemplate.inspectConfig.contentOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inspect-template.inspect-config.exclude-info-types" => Some(("inspectTemplate.inspectConfig.excludeInfoTypes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.include-quote" => Some(("inspectTemplate.inspectConfig.includeQuote", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.limits.max-findings-per-item" => Some(("inspectTemplate.inspectConfig.limits.maxFindingsPerItem", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.limits.max-findings-per-request" => Some(("inspectTemplate.inspectConfig.limits.maxFindingsPerRequest", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.min-likelihood" => Some(("inspectTemplate.inspectConfig.minLikelihood", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.name" => Some(("inspectTemplate.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.update-time" => Some(("inspectTemplate.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-mask" => Some(("updateMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["content-options", "create-time", "description", "display-name", "exclude-info-types", "include-quote", "inspect-config", "inspect-template", "limits", "max-findings-per-item", "max-findings-per-request", "min-likelihood", "name", "update-mask", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2UpdateInspectTemplateRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.organizations().locations_inspect_templates_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _organizations_locations_job_triggers_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "job-trigger.create-time" => Some(("jobTrigger.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.description" => Some(("jobTrigger.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.display-name" => Some(("jobTrigger.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-config.content-options" => Some(("jobTrigger.inspectJob.inspectConfig.contentOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-trigger.inspect-job.inspect-config.exclude-info-types" => Some(("jobTrigger.inspectJob.inspectConfig.excludeInfoTypes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-config.include-quote" => Some(("jobTrigger.inspectJob.inspectConfig.includeQuote", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-config.limits.max-findings-per-item" => Some(("jobTrigger.inspectJob.inspectConfig.limits.maxFindingsPerItem", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-config.limits.max-findings-per-request" => Some(("jobTrigger.inspectJob.inspectConfig.limits.maxFindingsPerRequest", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-config.min-likelihood" => Some(("jobTrigger.inspectJob.inspectConfig.minLikelihood", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-template-name" => Some(("jobTrigger.inspectJob.inspectTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.rows-limit" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.rowsLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.rows-limit-percent" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.rowsLimitPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.sample-method" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.sampleMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.table-reference.dataset-id" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.tableReference.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.table-reference.project-id" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.tableReference.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.table-reference.table-id" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.tableReference.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.bytes-limit-per-file" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.bytesLimitPerFile", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.bytes-limit-per-file-percent" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.bytesLimitPerFilePercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.file-set.regex-file-set.bucket-name" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.fileSet.regexFileSet.bucketName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.file-set.regex-file-set.exclude-regex" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.fileSet.regexFileSet.excludeRegex", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.file-set.regex-file-set.include-regex" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.fileSet.regexFileSet.includeRegex", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.file-set.url" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.fileSet.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.file-types" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.fileTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.files-limit-percent" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.filesLimitPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.sample-method" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.sampleMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.datastore-options.kind.name" => Some(("jobTrigger.inspectJob.storageConfig.datastoreOptions.kind.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.datastore-options.partition-id.namespace-id" => Some(("jobTrigger.inspectJob.storageConfig.datastoreOptions.partitionId.namespaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.datastore-options.partition-id.project-id" => Some(("jobTrigger.inspectJob.storageConfig.datastoreOptions.partitionId.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.hybrid-options.description" => Some(("jobTrigger.inspectJob.storageConfig.hybridOptions.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.hybrid-options.labels" => Some(("jobTrigger.inspectJob.storageConfig.hybridOptions.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job-trigger.inspect-job.storage-config.hybrid-options.required-finding-label-keys" => Some(("jobTrigger.inspectJob.storageConfig.hybridOptions.requiredFindingLabelKeys", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-trigger.inspect-job.storage-config.timespan-config.enable-auto-population-of-timespan-config" => Some(("jobTrigger.inspectJob.storageConfig.timespanConfig.enableAutoPopulationOfTimespanConfig", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.timespan-config.end-time" => Some(("jobTrigger.inspectJob.storageConfig.timespanConfig.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.timespan-config.start-time" => Some(("jobTrigger.inspectJob.storageConfig.timespanConfig.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.timespan-config.timestamp-field.name" => Some(("jobTrigger.inspectJob.storageConfig.timespanConfig.timestampField.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.last-run-time" => Some(("jobTrigger.lastRunTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.name" => Some(("jobTrigger.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.status" => Some(("jobTrigger.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.update-time" => Some(("jobTrigger.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-id" => Some(("locationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger-id" => Some(("triggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["big-query-options", "bucket-name", "bytes-limit-per-file", "bytes-limit-per-file-percent", "cloud-storage-options", "content-options", "create-time", "dataset-id", "datastore-options", "description", "display-name", "enable-auto-population-of-timespan-config", "end-time", "exclude-info-types", "exclude-regex", "file-set", "file-types", "files-limit-percent", "hybrid-options", "include-quote", "include-regex", "inspect-config", "inspect-job", "inspect-template-name", "job-trigger", "kind", "labels", "last-run-time", "limits", "location-id", "max-findings-per-item", "max-findings-per-request", "min-likelihood", "name", "namespace-id", "partition-id", "project-id", "regex-file-set", "required-finding-label-keys", "rows-limit", "rows-limit-percent", "sample-method", "start-time", "status", "storage-config", "table-id", "table-reference", "timespan-config", "timestamp-field", "trigger-id", "update-time", "url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2CreateJobTriggerRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.organizations().locations_job_triggers_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _organizations_locations_job_triggers_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().locations_job_triggers_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _organizations_locations_job_triggers_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().locations_job_triggers_get(opt.value_of("name").unwrap_or(""));
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

    async fn _organizations_locations_job_triggers_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().locations_job_triggers_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "type" => {
                    call = call.type_(value.unwrap_or(""));
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
                "location-id" => {
                    call = call.location_id(value.unwrap_or(""));
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
                                                                           v.extend(["filter", "location-id", "order-by", "page-size", "page-token", "type"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _organizations_locations_job_triggers_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "job-trigger.create-time" => Some(("jobTrigger.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.description" => Some(("jobTrigger.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.display-name" => Some(("jobTrigger.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-config.content-options" => Some(("jobTrigger.inspectJob.inspectConfig.contentOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-trigger.inspect-job.inspect-config.exclude-info-types" => Some(("jobTrigger.inspectJob.inspectConfig.excludeInfoTypes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-config.include-quote" => Some(("jobTrigger.inspectJob.inspectConfig.includeQuote", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-config.limits.max-findings-per-item" => Some(("jobTrigger.inspectJob.inspectConfig.limits.maxFindingsPerItem", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-config.limits.max-findings-per-request" => Some(("jobTrigger.inspectJob.inspectConfig.limits.maxFindingsPerRequest", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-config.min-likelihood" => Some(("jobTrigger.inspectJob.inspectConfig.minLikelihood", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-template-name" => Some(("jobTrigger.inspectJob.inspectTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.rows-limit" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.rowsLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.rows-limit-percent" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.rowsLimitPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.sample-method" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.sampleMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.table-reference.dataset-id" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.tableReference.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.table-reference.project-id" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.tableReference.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.table-reference.table-id" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.tableReference.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.bytes-limit-per-file" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.bytesLimitPerFile", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.bytes-limit-per-file-percent" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.bytesLimitPerFilePercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.file-set.regex-file-set.bucket-name" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.fileSet.regexFileSet.bucketName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.file-set.regex-file-set.exclude-regex" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.fileSet.regexFileSet.excludeRegex", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.file-set.regex-file-set.include-regex" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.fileSet.regexFileSet.includeRegex", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.file-set.url" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.fileSet.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.file-types" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.fileTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.files-limit-percent" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.filesLimitPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.sample-method" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.sampleMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.datastore-options.kind.name" => Some(("jobTrigger.inspectJob.storageConfig.datastoreOptions.kind.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.datastore-options.partition-id.namespace-id" => Some(("jobTrigger.inspectJob.storageConfig.datastoreOptions.partitionId.namespaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.datastore-options.partition-id.project-id" => Some(("jobTrigger.inspectJob.storageConfig.datastoreOptions.partitionId.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.hybrid-options.description" => Some(("jobTrigger.inspectJob.storageConfig.hybridOptions.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.hybrid-options.labels" => Some(("jobTrigger.inspectJob.storageConfig.hybridOptions.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job-trigger.inspect-job.storage-config.hybrid-options.required-finding-label-keys" => Some(("jobTrigger.inspectJob.storageConfig.hybridOptions.requiredFindingLabelKeys", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-trigger.inspect-job.storage-config.timespan-config.enable-auto-population-of-timespan-config" => Some(("jobTrigger.inspectJob.storageConfig.timespanConfig.enableAutoPopulationOfTimespanConfig", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.timespan-config.end-time" => Some(("jobTrigger.inspectJob.storageConfig.timespanConfig.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.timespan-config.start-time" => Some(("jobTrigger.inspectJob.storageConfig.timespanConfig.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.timespan-config.timestamp-field.name" => Some(("jobTrigger.inspectJob.storageConfig.timespanConfig.timestampField.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.last-run-time" => Some(("jobTrigger.lastRunTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.name" => Some(("jobTrigger.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.status" => Some(("jobTrigger.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.update-time" => Some(("jobTrigger.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-mask" => Some(("updateMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["big-query-options", "bucket-name", "bytes-limit-per-file", "bytes-limit-per-file-percent", "cloud-storage-options", "content-options", "create-time", "dataset-id", "datastore-options", "description", "display-name", "enable-auto-population-of-timespan-config", "end-time", "exclude-info-types", "exclude-regex", "file-set", "file-types", "files-limit-percent", "hybrid-options", "include-quote", "include-regex", "inspect-config", "inspect-job", "inspect-template-name", "job-trigger", "kind", "labels", "last-run-time", "limits", "max-findings-per-item", "max-findings-per-request", "min-likelihood", "name", "namespace-id", "partition-id", "project-id", "regex-file-set", "required-finding-label-keys", "rows-limit", "rows-limit-percent", "sample-method", "start-time", "status", "storage-config", "table-id", "table-reference", "timespan-config", "timestamp-field", "update-mask", "update-time", "url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2UpdateJobTriggerRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.organizations().locations_job_triggers_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _organizations_locations_project_data_profiles_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().locations_project_data_profiles_get(opt.value_of("name").unwrap_or(""));
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

    async fn _organizations_locations_project_data_profiles_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().locations_project_data_profiles_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
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
                                                                           v.extend(["filter", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _organizations_locations_stored_info_types_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "config.description" => Some(("config.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.dictionary.cloud-storage-path.path" => Some(("config.dictionary.cloudStoragePath.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.dictionary.word-list.words" => Some(("config.dictionary.wordList.words", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.display-name" => Some(("config.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.field.name" => Some(("config.largeCustomDictionary.bigQueryField.field.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.table.dataset-id" => Some(("config.largeCustomDictionary.bigQueryField.table.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.table.project-id" => Some(("config.largeCustomDictionary.bigQueryField.table.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.table.table-id" => Some(("config.largeCustomDictionary.bigQueryField.table.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.cloud-storage-file-set.url" => Some(("config.largeCustomDictionary.cloudStorageFileSet.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.output-path.path" => Some(("config.largeCustomDictionary.outputPath.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.regex.group-indexes" => Some(("config.regex.groupIndexes", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Vec })),
                    "config.regex.pattern" => Some(("config.regex.pattern", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-id" => Some(("locationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "stored-info-type-id" => Some(("storedInfoTypeId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["big-query-field", "cloud-storage-file-set", "cloud-storage-path", "config", "dataset-id", "description", "dictionary", "display-name", "field", "group-indexes", "large-custom-dictionary", "location-id", "name", "output-path", "path", "pattern", "project-id", "regex", "stored-info-type-id", "table", "table-id", "url", "word-list", "words"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2CreateStoredInfoTypeRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.organizations().locations_stored_info_types_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _organizations_locations_stored_info_types_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().locations_stored_info_types_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _organizations_locations_stored_info_types_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().locations_stored_info_types_get(opt.value_of("name").unwrap_or(""));
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

    async fn _organizations_locations_stored_info_types_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().locations_stored_info_types_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "location-id" => {
                    call = call.location_id(value.unwrap_or(""));
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
                                                                           v.extend(["location-id", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _organizations_locations_stored_info_types_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "config.description" => Some(("config.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.dictionary.cloud-storage-path.path" => Some(("config.dictionary.cloudStoragePath.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.dictionary.word-list.words" => Some(("config.dictionary.wordList.words", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.display-name" => Some(("config.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.field.name" => Some(("config.largeCustomDictionary.bigQueryField.field.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.table.dataset-id" => Some(("config.largeCustomDictionary.bigQueryField.table.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.table.project-id" => Some(("config.largeCustomDictionary.bigQueryField.table.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.table.table-id" => Some(("config.largeCustomDictionary.bigQueryField.table.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.cloud-storage-file-set.url" => Some(("config.largeCustomDictionary.cloudStorageFileSet.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.output-path.path" => Some(("config.largeCustomDictionary.outputPath.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.regex.group-indexes" => Some(("config.regex.groupIndexes", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Vec })),
                    "config.regex.pattern" => Some(("config.regex.pattern", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-mask" => Some(("updateMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["big-query-field", "cloud-storage-file-set", "cloud-storage-path", "config", "dataset-id", "description", "dictionary", "display-name", "field", "group-indexes", "large-custom-dictionary", "name", "output-path", "path", "pattern", "project-id", "regex", "table", "table-id", "update-mask", "url", "word-list", "words"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2UpdateStoredInfoTypeRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.organizations().locations_stored_info_types_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _organizations_locations_table_data_profiles_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().locations_table_data_profiles_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _organizations_locations_table_data_profiles_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().locations_table_data_profiles_get(opt.value_of("name").unwrap_or(""));
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

    async fn _organizations_locations_table_data_profiles_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().locations_table_data_profiles_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
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
                                                                           v.extend(["filter", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _organizations_stored_info_types_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "config.description" => Some(("config.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.dictionary.cloud-storage-path.path" => Some(("config.dictionary.cloudStoragePath.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.dictionary.word-list.words" => Some(("config.dictionary.wordList.words", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.display-name" => Some(("config.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.field.name" => Some(("config.largeCustomDictionary.bigQueryField.field.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.table.dataset-id" => Some(("config.largeCustomDictionary.bigQueryField.table.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.table.project-id" => Some(("config.largeCustomDictionary.bigQueryField.table.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.table.table-id" => Some(("config.largeCustomDictionary.bigQueryField.table.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.cloud-storage-file-set.url" => Some(("config.largeCustomDictionary.cloudStorageFileSet.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.output-path.path" => Some(("config.largeCustomDictionary.outputPath.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.regex.group-indexes" => Some(("config.regex.groupIndexes", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Vec })),
                    "config.regex.pattern" => Some(("config.regex.pattern", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-id" => Some(("locationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "stored-info-type-id" => Some(("storedInfoTypeId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["big-query-field", "cloud-storage-file-set", "cloud-storage-path", "config", "dataset-id", "description", "dictionary", "display-name", "field", "group-indexes", "large-custom-dictionary", "location-id", "name", "output-path", "path", "pattern", "project-id", "regex", "stored-info-type-id", "table", "table-id", "url", "word-list", "words"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2CreateStoredInfoTypeRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.organizations().stored_info_types_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _organizations_stored_info_types_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().stored_info_types_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _organizations_stored_info_types_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().stored_info_types_get(opt.value_of("name").unwrap_or(""));
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

    async fn _organizations_stored_info_types_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().stored_info_types_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "location-id" => {
                    call = call.location_id(value.unwrap_or(""));
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
                                                                           v.extend(["location-id", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _organizations_stored_info_types_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "config.description" => Some(("config.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.dictionary.cloud-storage-path.path" => Some(("config.dictionary.cloudStoragePath.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.dictionary.word-list.words" => Some(("config.dictionary.wordList.words", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.display-name" => Some(("config.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.field.name" => Some(("config.largeCustomDictionary.bigQueryField.field.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.table.dataset-id" => Some(("config.largeCustomDictionary.bigQueryField.table.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.table.project-id" => Some(("config.largeCustomDictionary.bigQueryField.table.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.table.table-id" => Some(("config.largeCustomDictionary.bigQueryField.table.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.cloud-storage-file-set.url" => Some(("config.largeCustomDictionary.cloudStorageFileSet.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.output-path.path" => Some(("config.largeCustomDictionary.outputPath.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.regex.group-indexes" => Some(("config.regex.groupIndexes", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Vec })),
                    "config.regex.pattern" => Some(("config.regex.pattern", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-mask" => Some(("updateMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["big-query-field", "cloud-storage-file-set", "cloud-storage-path", "config", "dataset-id", "description", "dictionary", "display-name", "field", "group-indexes", "large-custom-dictionary", "name", "output-path", "path", "pattern", "project-id", "regex", "table", "table-id", "update-mask", "url", "word-list", "words"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2UpdateStoredInfoTypeRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.organizations().stored_info_types_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_content_deidentify(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "deidentify-template-name" => Some(("deidentifyTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-config.content-options" => Some(("inspectConfig.contentOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inspect-config.exclude-info-types" => Some(("inspectConfig.excludeInfoTypes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-config.include-quote" => Some(("inspectConfig.includeQuote", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-config.limits.max-findings-per-item" => Some(("inspectConfig.limits.maxFindingsPerItem", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-config.limits.max-findings-per-request" => Some(("inspectConfig.limits.maxFindingsPerRequest", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-config.min-likelihood" => Some(("inspectConfig.minLikelihood", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template-name" => Some(("inspectTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "item.byte-item.data" => Some(("item.byteItem.data", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "item.byte-item.type" => Some(("item.byteItem.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "item.value" => Some(("item.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-id" => Some(("locationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["byte-item", "content-options", "data", "deidentify-template-name", "exclude-info-types", "include-quote", "inspect-config", "inspect-template-name", "item", "limits", "location-id", "max-findings-per-item", "max-findings-per-request", "min-likelihood", "type", "value"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2DeidentifyContentRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().content_deidentify(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_content_inspect(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "inspect-config.content-options" => Some(("inspectConfig.contentOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inspect-config.exclude-info-types" => Some(("inspectConfig.excludeInfoTypes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-config.include-quote" => Some(("inspectConfig.includeQuote", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-config.limits.max-findings-per-item" => Some(("inspectConfig.limits.maxFindingsPerItem", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-config.limits.max-findings-per-request" => Some(("inspectConfig.limits.maxFindingsPerRequest", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-config.min-likelihood" => Some(("inspectConfig.minLikelihood", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template-name" => Some(("inspectTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "item.byte-item.data" => Some(("item.byteItem.data", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "item.byte-item.type" => Some(("item.byteItem.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "item.value" => Some(("item.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-id" => Some(("locationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["byte-item", "content-options", "data", "exclude-info-types", "include-quote", "inspect-config", "inspect-template-name", "item", "limits", "location-id", "max-findings-per-item", "max-findings-per-request", "min-likelihood", "type", "value"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2InspectContentRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().content_inspect(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_content_reidentify(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "inspect-config.content-options" => Some(("inspectConfig.contentOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inspect-config.exclude-info-types" => Some(("inspectConfig.excludeInfoTypes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-config.include-quote" => Some(("inspectConfig.includeQuote", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-config.limits.max-findings-per-item" => Some(("inspectConfig.limits.maxFindingsPerItem", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-config.limits.max-findings-per-request" => Some(("inspectConfig.limits.maxFindingsPerRequest", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-config.min-likelihood" => Some(("inspectConfig.minLikelihood", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template-name" => Some(("inspectTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "item.byte-item.data" => Some(("item.byteItem.data", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "item.byte-item.type" => Some(("item.byteItem.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "item.value" => Some(("item.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-id" => Some(("locationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "reidentify-template-name" => Some(("reidentifyTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["byte-item", "content-options", "data", "exclude-info-types", "include-quote", "inspect-config", "inspect-template-name", "item", "limits", "location-id", "max-findings-per-item", "max-findings-per-request", "min-likelihood", "reidentify-template-name", "type", "value"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2ReidentifyContentRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().content_reidentify(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_deidentify_templates_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "deidentify-template.create-time" => Some(("deidentifyTemplate.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.description" => Some(("deidentifyTemplate.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.display-name" => Some(("deidentifyTemplate.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.name" => Some(("deidentifyTemplate.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.update-time" => Some(("deidentifyTemplate.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-id" => Some(("locationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "template-id" => Some(("templateId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["create-time", "deidentify-template", "description", "display-name", "location-id", "name", "template-id", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2CreateDeidentifyTemplateRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().deidentify_templates_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_deidentify_templates_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().deidentify_templates_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_deidentify_templates_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().deidentify_templates_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_deidentify_templates_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().deidentify_templates_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "location-id" => {
                    call = call.location_id(value.unwrap_or(""));
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
                                                                           v.extend(["location-id", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_deidentify_templates_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "deidentify-template.create-time" => Some(("deidentifyTemplate.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.description" => Some(("deidentifyTemplate.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.display-name" => Some(("deidentifyTemplate.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.name" => Some(("deidentifyTemplate.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.update-time" => Some(("deidentifyTemplate.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-mask" => Some(("updateMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["create-time", "deidentify-template", "description", "display-name", "name", "update-mask", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2UpdateDeidentifyTemplateRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().deidentify_templates_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_dlp_jobs_cancel(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::GooglePrivacyDlpV2CancelDlpJobRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().dlp_jobs_cancel(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_dlp_jobs_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "inspect-job.inspect-config.content-options" => Some(("inspectJob.inspectConfig.contentOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inspect-job.inspect-config.exclude-info-types" => Some(("inspectJob.inspectConfig.excludeInfoTypes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-job.inspect-config.include-quote" => Some(("inspectJob.inspectConfig.includeQuote", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-job.inspect-config.limits.max-findings-per-item" => Some(("inspectJob.inspectConfig.limits.maxFindingsPerItem", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-job.inspect-config.limits.max-findings-per-request" => Some(("inspectJob.inspectConfig.limits.maxFindingsPerRequest", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-job.inspect-config.min-likelihood" => Some(("inspectJob.inspectConfig.minLikelihood", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.inspect-template-name" => Some(("inspectJob.inspectTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.big-query-options.rows-limit" => Some(("inspectJob.storageConfig.bigQueryOptions.rowsLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.big-query-options.rows-limit-percent" => Some(("inspectJob.storageConfig.bigQueryOptions.rowsLimitPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.big-query-options.sample-method" => Some(("inspectJob.storageConfig.bigQueryOptions.sampleMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.big-query-options.table-reference.dataset-id" => Some(("inspectJob.storageConfig.bigQueryOptions.tableReference.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.big-query-options.table-reference.project-id" => Some(("inspectJob.storageConfig.bigQueryOptions.tableReference.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.big-query-options.table-reference.table-id" => Some(("inspectJob.storageConfig.bigQueryOptions.tableReference.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.cloud-storage-options.bytes-limit-per-file" => Some(("inspectJob.storageConfig.cloudStorageOptions.bytesLimitPerFile", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.cloud-storage-options.bytes-limit-per-file-percent" => Some(("inspectJob.storageConfig.cloudStorageOptions.bytesLimitPerFilePercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.cloud-storage-options.file-set.regex-file-set.bucket-name" => Some(("inspectJob.storageConfig.cloudStorageOptions.fileSet.regexFileSet.bucketName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.cloud-storage-options.file-set.regex-file-set.exclude-regex" => Some(("inspectJob.storageConfig.cloudStorageOptions.fileSet.regexFileSet.excludeRegex", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inspect-job.storage-config.cloud-storage-options.file-set.regex-file-set.include-regex" => Some(("inspectJob.storageConfig.cloudStorageOptions.fileSet.regexFileSet.includeRegex", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inspect-job.storage-config.cloud-storage-options.file-set.url" => Some(("inspectJob.storageConfig.cloudStorageOptions.fileSet.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.cloud-storage-options.file-types" => Some(("inspectJob.storageConfig.cloudStorageOptions.fileTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inspect-job.storage-config.cloud-storage-options.files-limit-percent" => Some(("inspectJob.storageConfig.cloudStorageOptions.filesLimitPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.cloud-storage-options.sample-method" => Some(("inspectJob.storageConfig.cloudStorageOptions.sampleMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.datastore-options.kind.name" => Some(("inspectJob.storageConfig.datastoreOptions.kind.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.datastore-options.partition-id.namespace-id" => Some(("inspectJob.storageConfig.datastoreOptions.partitionId.namespaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.datastore-options.partition-id.project-id" => Some(("inspectJob.storageConfig.datastoreOptions.partitionId.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.hybrid-options.description" => Some(("inspectJob.storageConfig.hybridOptions.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.hybrid-options.labels" => Some(("inspectJob.storageConfig.hybridOptions.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "inspect-job.storage-config.hybrid-options.required-finding-label-keys" => Some(("inspectJob.storageConfig.hybridOptions.requiredFindingLabelKeys", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inspect-job.storage-config.timespan-config.enable-auto-population-of-timespan-config" => Some(("inspectJob.storageConfig.timespanConfig.enableAutoPopulationOfTimespanConfig", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.timespan-config.end-time" => Some(("inspectJob.storageConfig.timespanConfig.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.timespan-config.start-time" => Some(("inspectJob.storageConfig.timespanConfig.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.timespan-config.timestamp-field.name" => Some(("inspectJob.storageConfig.timespanConfig.timestampField.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-id" => Some(("jobId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-id" => Some(("locationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "risk-job.privacy-metric.categorical-stats-config.field.name" => Some(("riskJob.privacyMetric.categoricalStatsConfig.field.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "risk-job.privacy-metric.delta-presence-estimation-config.region-code" => Some(("riskJob.privacyMetric.deltaPresenceEstimationConfig.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "risk-job.privacy-metric.k-anonymity-config.entity-id.field.name" => Some(("riskJob.privacyMetric.kAnonymityConfig.entityId.field.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "risk-job.privacy-metric.k-map-estimation-config.region-code" => Some(("riskJob.privacyMetric.kMapEstimationConfig.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "risk-job.privacy-metric.l-diversity-config.sensitive-attribute.name" => Some(("riskJob.privacyMetric.lDiversityConfig.sensitiveAttribute.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "risk-job.privacy-metric.numerical-stats-config.field.name" => Some(("riskJob.privacyMetric.numericalStatsConfig.field.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "risk-job.source-table.dataset-id" => Some(("riskJob.sourceTable.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "risk-job.source-table.project-id" => Some(("riskJob.sourceTable.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "risk-job.source-table.table-id" => Some(("riskJob.sourceTable.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["big-query-options", "bucket-name", "bytes-limit-per-file", "bytes-limit-per-file-percent", "categorical-stats-config", "cloud-storage-options", "content-options", "dataset-id", "datastore-options", "delta-presence-estimation-config", "description", "enable-auto-population-of-timespan-config", "end-time", "entity-id", "exclude-info-types", "exclude-regex", "field", "file-set", "file-types", "files-limit-percent", "hybrid-options", "include-quote", "include-regex", "inspect-config", "inspect-job", "inspect-template-name", "job-id", "k-anonymity-config", "k-map-estimation-config", "kind", "l-diversity-config", "labels", "limits", "location-id", "max-findings-per-item", "max-findings-per-request", "min-likelihood", "name", "namespace-id", "numerical-stats-config", "partition-id", "privacy-metric", "project-id", "regex-file-set", "region-code", "required-finding-label-keys", "risk-job", "rows-limit", "rows-limit-percent", "sample-method", "sensitive-attribute", "source-table", "start-time", "storage-config", "table-id", "table-reference", "timespan-config", "timestamp-field", "url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2CreateDlpJobRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().dlp_jobs_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_dlp_jobs_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().dlp_jobs_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_dlp_jobs_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().dlp_jobs_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_dlp_jobs_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().dlp_jobs_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "type" => {
                    call = call.type_(value.unwrap_or(""));
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
                "location-id" => {
                    call = call.location_id(value.unwrap_or(""));
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
                                                                           v.extend(["filter", "location-id", "order-by", "page-size", "page-token", "type"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_image_redact(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "byte-item.data" => Some(("byteItem.data", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "byte-item.type" => Some(("byteItem.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "include-findings" => Some(("includeFindings", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-config.content-options" => Some(("inspectConfig.contentOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inspect-config.exclude-info-types" => Some(("inspectConfig.excludeInfoTypes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-config.include-quote" => Some(("inspectConfig.includeQuote", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-config.limits.max-findings-per-item" => Some(("inspectConfig.limits.maxFindingsPerItem", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-config.limits.max-findings-per-request" => Some(("inspectConfig.limits.maxFindingsPerRequest", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-config.min-likelihood" => Some(("inspectConfig.minLikelihood", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-id" => Some(("locationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["byte-item", "content-options", "data", "exclude-info-types", "include-findings", "include-quote", "inspect-config", "limits", "location-id", "max-findings-per-item", "max-findings-per-request", "min-likelihood", "type"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2RedactImageRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().image_redact(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_inspect_templates_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "inspect-template.create-time" => Some(("inspectTemplate.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.description" => Some(("inspectTemplate.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.display-name" => Some(("inspectTemplate.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.content-options" => Some(("inspectTemplate.inspectConfig.contentOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inspect-template.inspect-config.exclude-info-types" => Some(("inspectTemplate.inspectConfig.excludeInfoTypes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.include-quote" => Some(("inspectTemplate.inspectConfig.includeQuote", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.limits.max-findings-per-item" => Some(("inspectTemplate.inspectConfig.limits.maxFindingsPerItem", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.limits.max-findings-per-request" => Some(("inspectTemplate.inspectConfig.limits.maxFindingsPerRequest", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.min-likelihood" => Some(("inspectTemplate.inspectConfig.minLikelihood", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.name" => Some(("inspectTemplate.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.update-time" => Some(("inspectTemplate.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-id" => Some(("locationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "template-id" => Some(("templateId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["content-options", "create-time", "description", "display-name", "exclude-info-types", "include-quote", "inspect-config", "inspect-template", "limits", "location-id", "max-findings-per-item", "max-findings-per-request", "min-likelihood", "name", "template-id", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2CreateInspectTemplateRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().inspect_templates_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_inspect_templates_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().inspect_templates_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_inspect_templates_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().inspect_templates_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_inspect_templates_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().inspect_templates_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "location-id" => {
                    call = call.location_id(value.unwrap_or(""));
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
                                                                           v.extend(["location-id", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_inspect_templates_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "inspect-template.create-time" => Some(("inspectTemplate.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.description" => Some(("inspectTemplate.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.display-name" => Some(("inspectTemplate.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.content-options" => Some(("inspectTemplate.inspectConfig.contentOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inspect-template.inspect-config.exclude-info-types" => Some(("inspectTemplate.inspectConfig.excludeInfoTypes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.include-quote" => Some(("inspectTemplate.inspectConfig.includeQuote", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.limits.max-findings-per-item" => Some(("inspectTemplate.inspectConfig.limits.maxFindingsPerItem", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.limits.max-findings-per-request" => Some(("inspectTemplate.inspectConfig.limits.maxFindingsPerRequest", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.min-likelihood" => Some(("inspectTemplate.inspectConfig.minLikelihood", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.name" => Some(("inspectTemplate.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.update-time" => Some(("inspectTemplate.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-mask" => Some(("updateMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["content-options", "create-time", "description", "display-name", "exclude-info-types", "include-quote", "inspect-config", "inspect-template", "limits", "max-findings-per-item", "max-findings-per-request", "min-likelihood", "name", "update-mask", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2UpdateInspectTemplateRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().inspect_templates_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_job_triggers_activate(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::GooglePrivacyDlpV2ActivateJobTriggerRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().job_triggers_activate(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_job_triggers_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "job-trigger.create-time" => Some(("jobTrigger.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.description" => Some(("jobTrigger.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.display-name" => Some(("jobTrigger.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-config.content-options" => Some(("jobTrigger.inspectJob.inspectConfig.contentOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-trigger.inspect-job.inspect-config.exclude-info-types" => Some(("jobTrigger.inspectJob.inspectConfig.excludeInfoTypes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-config.include-quote" => Some(("jobTrigger.inspectJob.inspectConfig.includeQuote", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-config.limits.max-findings-per-item" => Some(("jobTrigger.inspectJob.inspectConfig.limits.maxFindingsPerItem", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-config.limits.max-findings-per-request" => Some(("jobTrigger.inspectJob.inspectConfig.limits.maxFindingsPerRequest", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-config.min-likelihood" => Some(("jobTrigger.inspectJob.inspectConfig.minLikelihood", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-template-name" => Some(("jobTrigger.inspectJob.inspectTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.rows-limit" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.rowsLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.rows-limit-percent" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.rowsLimitPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.sample-method" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.sampleMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.table-reference.dataset-id" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.tableReference.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.table-reference.project-id" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.tableReference.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.table-reference.table-id" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.tableReference.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.bytes-limit-per-file" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.bytesLimitPerFile", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.bytes-limit-per-file-percent" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.bytesLimitPerFilePercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.file-set.regex-file-set.bucket-name" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.fileSet.regexFileSet.bucketName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.file-set.regex-file-set.exclude-regex" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.fileSet.regexFileSet.excludeRegex", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.file-set.regex-file-set.include-regex" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.fileSet.regexFileSet.includeRegex", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.file-set.url" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.fileSet.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.file-types" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.fileTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.files-limit-percent" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.filesLimitPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.sample-method" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.sampleMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.datastore-options.kind.name" => Some(("jobTrigger.inspectJob.storageConfig.datastoreOptions.kind.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.datastore-options.partition-id.namespace-id" => Some(("jobTrigger.inspectJob.storageConfig.datastoreOptions.partitionId.namespaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.datastore-options.partition-id.project-id" => Some(("jobTrigger.inspectJob.storageConfig.datastoreOptions.partitionId.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.hybrid-options.description" => Some(("jobTrigger.inspectJob.storageConfig.hybridOptions.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.hybrid-options.labels" => Some(("jobTrigger.inspectJob.storageConfig.hybridOptions.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job-trigger.inspect-job.storage-config.hybrid-options.required-finding-label-keys" => Some(("jobTrigger.inspectJob.storageConfig.hybridOptions.requiredFindingLabelKeys", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-trigger.inspect-job.storage-config.timespan-config.enable-auto-population-of-timespan-config" => Some(("jobTrigger.inspectJob.storageConfig.timespanConfig.enableAutoPopulationOfTimespanConfig", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.timespan-config.end-time" => Some(("jobTrigger.inspectJob.storageConfig.timespanConfig.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.timespan-config.start-time" => Some(("jobTrigger.inspectJob.storageConfig.timespanConfig.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.timespan-config.timestamp-field.name" => Some(("jobTrigger.inspectJob.storageConfig.timespanConfig.timestampField.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.last-run-time" => Some(("jobTrigger.lastRunTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.name" => Some(("jobTrigger.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.status" => Some(("jobTrigger.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.update-time" => Some(("jobTrigger.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-id" => Some(("locationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger-id" => Some(("triggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["big-query-options", "bucket-name", "bytes-limit-per-file", "bytes-limit-per-file-percent", "cloud-storage-options", "content-options", "create-time", "dataset-id", "datastore-options", "description", "display-name", "enable-auto-population-of-timespan-config", "end-time", "exclude-info-types", "exclude-regex", "file-set", "file-types", "files-limit-percent", "hybrid-options", "include-quote", "include-regex", "inspect-config", "inspect-job", "inspect-template-name", "job-trigger", "kind", "labels", "last-run-time", "limits", "location-id", "max-findings-per-item", "max-findings-per-request", "min-likelihood", "name", "namespace-id", "partition-id", "project-id", "regex-file-set", "required-finding-label-keys", "rows-limit", "rows-limit-percent", "sample-method", "start-time", "status", "storage-config", "table-id", "table-reference", "timespan-config", "timestamp-field", "trigger-id", "update-time", "url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2CreateJobTriggerRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().job_triggers_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_job_triggers_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().job_triggers_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_job_triggers_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().job_triggers_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_job_triggers_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().job_triggers_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "type" => {
                    call = call.type_(value.unwrap_or(""));
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
                "location-id" => {
                    call = call.location_id(value.unwrap_or(""));
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
                                                                           v.extend(["filter", "location-id", "order-by", "page-size", "page-token", "type"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_job_triggers_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "job-trigger.create-time" => Some(("jobTrigger.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.description" => Some(("jobTrigger.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.display-name" => Some(("jobTrigger.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-config.content-options" => Some(("jobTrigger.inspectJob.inspectConfig.contentOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-trigger.inspect-job.inspect-config.exclude-info-types" => Some(("jobTrigger.inspectJob.inspectConfig.excludeInfoTypes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-config.include-quote" => Some(("jobTrigger.inspectJob.inspectConfig.includeQuote", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-config.limits.max-findings-per-item" => Some(("jobTrigger.inspectJob.inspectConfig.limits.maxFindingsPerItem", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-config.limits.max-findings-per-request" => Some(("jobTrigger.inspectJob.inspectConfig.limits.maxFindingsPerRequest", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-config.min-likelihood" => Some(("jobTrigger.inspectJob.inspectConfig.minLikelihood", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-template-name" => Some(("jobTrigger.inspectJob.inspectTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.rows-limit" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.rowsLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.rows-limit-percent" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.rowsLimitPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.sample-method" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.sampleMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.table-reference.dataset-id" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.tableReference.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.table-reference.project-id" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.tableReference.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.table-reference.table-id" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.tableReference.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.bytes-limit-per-file" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.bytesLimitPerFile", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.bytes-limit-per-file-percent" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.bytesLimitPerFilePercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.file-set.regex-file-set.bucket-name" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.fileSet.regexFileSet.bucketName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.file-set.regex-file-set.exclude-regex" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.fileSet.regexFileSet.excludeRegex", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.file-set.regex-file-set.include-regex" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.fileSet.regexFileSet.includeRegex", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.file-set.url" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.fileSet.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.file-types" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.fileTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.files-limit-percent" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.filesLimitPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.sample-method" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.sampleMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.datastore-options.kind.name" => Some(("jobTrigger.inspectJob.storageConfig.datastoreOptions.kind.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.datastore-options.partition-id.namespace-id" => Some(("jobTrigger.inspectJob.storageConfig.datastoreOptions.partitionId.namespaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.datastore-options.partition-id.project-id" => Some(("jobTrigger.inspectJob.storageConfig.datastoreOptions.partitionId.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.hybrid-options.description" => Some(("jobTrigger.inspectJob.storageConfig.hybridOptions.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.hybrid-options.labels" => Some(("jobTrigger.inspectJob.storageConfig.hybridOptions.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job-trigger.inspect-job.storage-config.hybrid-options.required-finding-label-keys" => Some(("jobTrigger.inspectJob.storageConfig.hybridOptions.requiredFindingLabelKeys", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-trigger.inspect-job.storage-config.timespan-config.enable-auto-population-of-timespan-config" => Some(("jobTrigger.inspectJob.storageConfig.timespanConfig.enableAutoPopulationOfTimespanConfig", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.timespan-config.end-time" => Some(("jobTrigger.inspectJob.storageConfig.timespanConfig.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.timespan-config.start-time" => Some(("jobTrigger.inspectJob.storageConfig.timespanConfig.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.timespan-config.timestamp-field.name" => Some(("jobTrigger.inspectJob.storageConfig.timespanConfig.timestampField.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.last-run-time" => Some(("jobTrigger.lastRunTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.name" => Some(("jobTrigger.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.status" => Some(("jobTrigger.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.update-time" => Some(("jobTrigger.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-mask" => Some(("updateMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["big-query-options", "bucket-name", "bytes-limit-per-file", "bytes-limit-per-file-percent", "cloud-storage-options", "content-options", "create-time", "dataset-id", "datastore-options", "description", "display-name", "enable-auto-population-of-timespan-config", "end-time", "exclude-info-types", "exclude-regex", "file-set", "file-types", "files-limit-percent", "hybrid-options", "include-quote", "include-regex", "inspect-config", "inspect-job", "inspect-template-name", "job-trigger", "kind", "labels", "last-run-time", "limits", "max-findings-per-item", "max-findings-per-request", "min-likelihood", "name", "namespace-id", "partition-id", "project-id", "regex-file-set", "required-finding-label-keys", "rows-limit", "rows-limit-percent", "sample-method", "start-time", "status", "storage-config", "table-id", "table-reference", "timespan-config", "timestamp-field", "update-mask", "update-time", "url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2UpdateJobTriggerRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().job_triggers_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_column_data_profiles_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_column_data_profiles_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_column_data_profiles_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_column_data_profiles_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
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
                                                                           v.extend(["filter", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_connections_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "connection.cloud-sql.connection-name" => Some(("connection.cloudSql.connectionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "connection.cloud-sql.database-engine" => Some(("connection.cloudSql.databaseEngine", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "connection.cloud-sql.max-connections" => Some(("connection.cloudSql.maxConnections", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "connection.cloud-sql.username-password.password-secret-version-name" => Some(("connection.cloudSql.usernamePassword.passwordSecretVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "connection.cloud-sql.username-password.username" => Some(("connection.cloudSql.usernamePassword.username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "connection.name" => Some(("connection.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "connection.state" => Some(("connection.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cloud-sql", "connection", "connection-name", "database-engine", "max-connections", "name", "password-secret-version-name", "state", "username", "username-password"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2CreateConnectionRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_connections_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_connections_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_connections_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_connections_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_connections_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_connections_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_connections_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_connections_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "connection.cloud-sql.connection-name" => Some(("connection.cloudSql.connectionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "connection.cloud-sql.database-engine" => Some(("connection.cloudSql.databaseEngine", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "connection.cloud-sql.max-connections" => Some(("connection.cloudSql.maxConnections", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "connection.cloud-sql.username-password.password-secret-version-name" => Some(("connection.cloudSql.usernamePassword.passwordSecretVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "connection.cloud-sql.username-password.username" => Some(("connection.cloudSql.usernamePassword.username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "connection.name" => Some(("connection.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "connection.state" => Some(("connection.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-mask" => Some(("updateMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cloud-sql", "connection", "connection-name", "database-engine", "max-connections", "name", "password-secret-version-name", "state", "update-mask", "username", "username-password"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2UpdateConnectionRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_connections_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_connections_search(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_connections_search(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_content_deidentify(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "deidentify-template-name" => Some(("deidentifyTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-config.content-options" => Some(("inspectConfig.contentOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inspect-config.exclude-info-types" => Some(("inspectConfig.excludeInfoTypes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-config.include-quote" => Some(("inspectConfig.includeQuote", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-config.limits.max-findings-per-item" => Some(("inspectConfig.limits.maxFindingsPerItem", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-config.limits.max-findings-per-request" => Some(("inspectConfig.limits.maxFindingsPerRequest", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-config.min-likelihood" => Some(("inspectConfig.minLikelihood", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template-name" => Some(("inspectTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "item.byte-item.data" => Some(("item.byteItem.data", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "item.byte-item.type" => Some(("item.byteItem.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "item.value" => Some(("item.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-id" => Some(("locationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["byte-item", "content-options", "data", "deidentify-template-name", "exclude-info-types", "include-quote", "inspect-config", "inspect-template-name", "item", "limits", "location-id", "max-findings-per-item", "max-findings-per-request", "min-likelihood", "type", "value"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2DeidentifyContentRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_content_deidentify(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_content_inspect(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "inspect-config.content-options" => Some(("inspectConfig.contentOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inspect-config.exclude-info-types" => Some(("inspectConfig.excludeInfoTypes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-config.include-quote" => Some(("inspectConfig.includeQuote", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-config.limits.max-findings-per-item" => Some(("inspectConfig.limits.maxFindingsPerItem", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-config.limits.max-findings-per-request" => Some(("inspectConfig.limits.maxFindingsPerRequest", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-config.min-likelihood" => Some(("inspectConfig.minLikelihood", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template-name" => Some(("inspectTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "item.byte-item.data" => Some(("item.byteItem.data", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "item.byte-item.type" => Some(("item.byteItem.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "item.value" => Some(("item.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-id" => Some(("locationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["byte-item", "content-options", "data", "exclude-info-types", "include-quote", "inspect-config", "inspect-template-name", "item", "limits", "location-id", "max-findings-per-item", "max-findings-per-request", "min-likelihood", "type", "value"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2InspectContentRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_content_inspect(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_content_reidentify(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "inspect-config.content-options" => Some(("inspectConfig.contentOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inspect-config.exclude-info-types" => Some(("inspectConfig.excludeInfoTypes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-config.include-quote" => Some(("inspectConfig.includeQuote", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-config.limits.max-findings-per-item" => Some(("inspectConfig.limits.maxFindingsPerItem", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-config.limits.max-findings-per-request" => Some(("inspectConfig.limits.maxFindingsPerRequest", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-config.min-likelihood" => Some(("inspectConfig.minLikelihood", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template-name" => Some(("inspectTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "item.byte-item.data" => Some(("item.byteItem.data", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "item.byte-item.type" => Some(("item.byteItem.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "item.value" => Some(("item.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-id" => Some(("locationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "reidentify-template-name" => Some(("reidentifyTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["byte-item", "content-options", "data", "exclude-info-types", "include-quote", "inspect-config", "inspect-template-name", "item", "limits", "location-id", "max-findings-per-item", "max-findings-per-request", "min-likelihood", "reidentify-template-name", "type", "value"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2ReidentifyContentRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_content_reidentify(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_deidentify_templates_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "deidentify-template.create-time" => Some(("deidentifyTemplate.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.description" => Some(("deidentifyTemplate.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.display-name" => Some(("deidentifyTemplate.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.name" => Some(("deidentifyTemplate.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.update-time" => Some(("deidentifyTemplate.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-id" => Some(("locationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "template-id" => Some(("templateId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["create-time", "deidentify-template", "description", "display-name", "location-id", "name", "template-id", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2CreateDeidentifyTemplateRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_deidentify_templates_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_deidentify_templates_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_deidentify_templates_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_deidentify_templates_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_deidentify_templates_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_deidentify_templates_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_deidentify_templates_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "location-id" => {
                    call = call.location_id(value.unwrap_or(""));
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
                                                                           v.extend(["location-id", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_deidentify_templates_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "deidentify-template.create-time" => Some(("deidentifyTemplate.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.description" => Some(("deidentifyTemplate.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.display-name" => Some(("deidentifyTemplate.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.name" => Some(("deidentifyTemplate.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.update-time" => Some(("deidentifyTemplate.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-mask" => Some(("updateMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["create-time", "deidentify-template", "description", "display-name", "name", "update-mask", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2UpdateDeidentifyTemplateRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_deidentify_templates_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_discovery_configs_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "config-id" => Some(("configId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery-config.create-time" => Some(("discoveryConfig.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery-config.display-name" => Some(("discoveryConfig.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery-config.inspect-templates" => Some(("discoveryConfig.inspectTemplates", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "discovery-config.last-run-time" => Some(("discoveryConfig.lastRunTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery-config.name" => Some(("discoveryConfig.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery-config.org-config.location.folder-id" => Some(("discoveryConfig.orgConfig.location.folderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery-config.org-config.location.organization-id" => Some(("discoveryConfig.orgConfig.location.organizationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery-config.org-config.project-id" => Some(("discoveryConfig.orgConfig.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery-config.status" => Some(("discoveryConfig.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery-config.update-time" => Some(("discoveryConfig.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["config-id", "create-time", "discovery-config", "display-name", "folder-id", "inspect-templates", "last-run-time", "location", "name", "org-config", "organization-id", "project-id", "status", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2CreateDiscoveryConfigRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_discovery_configs_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_discovery_configs_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_discovery_configs_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_discovery_configs_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_discovery_configs_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_discovery_configs_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_discovery_configs_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
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
                                                                           v.extend(["order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_discovery_configs_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "discovery-config.create-time" => Some(("discoveryConfig.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery-config.display-name" => Some(("discoveryConfig.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery-config.inspect-templates" => Some(("discoveryConfig.inspectTemplates", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "discovery-config.last-run-time" => Some(("discoveryConfig.lastRunTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery-config.name" => Some(("discoveryConfig.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery-config.org-config.location.folder-id" => Some(("discoveryConfig.orgConfig.location.folderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery-config.org-config.location.organization-id" => Some(("discoveryConfig.orgConfig.location.organizationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery-config.org-config.project-id" => Some(("discoveryConfig.orgConfig.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery-config.status" => Some(("discoveryConfig.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "discovery-config.update-time" => Some(("discoveryConfig.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-mask" => Some(("updateMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["create-time", "discovery-config", "display-name", "folder-id", "inspect-templates", "last-run-time", "location", "name", "org-config", "organization-id", "project-id", "status", "update-mask", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2UpdateDiscoveryConfigRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_discovery_configs_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_dlp_jobs_cancel(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::GooglePrivacyDlpV2CancelDlpJobRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_dlp_jobs_cancel(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_dlp_jobs_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "inspect-job.inspect-config.content-options" => Some(("inspectJob.inspectConfig.contentOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inspect-job.inspect-config.exclude-info-types" => Some(("inspectJob.inspectConfig.excludeInfoTypes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-job.inspect-config.include-quote" => Some(("inspectJob.inspectConfig.includeQuote", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-job.inspect-config.limits.max-findings-per-item" => Some(("inspectJob.inspectConfig.limits.maxFindingsPerItem", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-job.inspect-config.limits.max-findings-per-request" => Some(("inspectJob.inspectConfig.limits.maxFindingsPerRequest", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-job.inspect-config.min-likelihood" => Some(("inspectJob.inspectConfig.minLikelihood", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.inspect-template-name" => Some(("inspectJob.inspectTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.big-query-options.rows-limit" => Some(("inspectJob.storageConfig.bigQueryOptions.rowsLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.big-query-options.rows-limit-percent" => Some(("inspectJob.storageConfig.bigQueryOptions.rowsLimitPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.big-query-options.sample-method" => Some(("inspectJob.storageConfig.bigQueryOptions.sampleMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.big-query-options.table-reference.dataset-id" => Some(("inspectJob.storageConfig.bigQueryOptions.tableReference.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.big-query-options.table-reference.project-id" => Some(("inspectJob.storageConfig.bigQueryOptions.tableReference.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.big-query-options.table-reference.table-id" => Some(("inspectJob.storageConfig.bigQueryOptions.tableReference.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.cloud-storage-options.bytes-limit-per-file" => Some(("inspectJob.storageConfig.cloudStorageOptions.bytesLimitPerFile", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.cloud-storage-options.bytes-limit-per-file-percent" => Some(("inspectJob.storageConfig.cloudStorageOptions.bytesLimitPerFilePercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.cloud-storage-options.file-set.regex-file-set.bucket-name" => Some(("inspectJob.storageConfig.cloudStorageOptions.fileSet.regexFileSet.bucketName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.cloud-storage-options.file-set.regex-file-set.exclude-regex" => Some(("inspectJob.storageConfig.cloudStorageOptions.fileSet.regexFileSet.excludeRegex", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inspect-job.storage-config.cloud-storage-options.file-set.regex-file-set.include-regex" => Some(("inspectJob.storageConfig.cloudStorageOptions.fileSet.regexFileSet.includeRegex", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inspect-job.storage-config.cloud-storage-options.file-set.url" => Some(("inspectJob.storageConfig.cloudStorageOptions.fileSet.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.cloud-storage-options.file-types" => Some(("inspectJob.storageConfig.cloudStorageOptions.fileTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inspect-job.storage-config.cloud-storage-options.files-limit-percent" => Some(("inspectJob.storageConfig.cloudStorageOptions.filesLimitPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.cloud-storage-options.sample-method" => Some(("inspectJob.storageConfig.cloudStorageOptions.sampleMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.datastore-options.kind.name" => Some(("inspectJob.storageConfig.datastoreOptions.kind.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.datastore-options.partition-id.namespace-id" => Some(("inspectJob.storageConfig.datastoreOptions.partitionId.namespaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.datastore-options.partition-id.project-id" => Some(("inspectJob.storageConfig.datastoreOptions.partitionId.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.hybrid-options.description" => Some(("inspectJob.storageConfig.hybridOptions.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.hybrid-options.labels" => Some(("inspectJob.storageConfig.hybridOptions.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "inspect-job.storage-config.hybrid-options.required-finding-label-keys" => Some(("inspectJob.storageConfig.hybridOptions.requiredFindingLabelKeys", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inspect-job.storage-config.timespan-config.enable-auto-population-of-timespan-config" => Some(("inspectJob.storageConfig.timespanConfig.enableAutoPopulationOfTimespanConfig", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.timespan-config.end-time" => Some(("inspectJob.storageConfig.timespanConfig.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.timespan-config.start-time" => Some(("inspectJob.storageConfig.timespanConfig.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.timespan-config.timestamp-field.name" => Some(("inspectJob.storageConfig.timespanConfig.timestampField.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-id" => Some(("jobId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-id" => Some(("locationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "risk-job.privacy-metric.categorical-stats-config.field.name" => Some(("riskJob.privacyMetric.categoricalStatsConfig.field.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "risk-job.privacy-metric.delta-presence-estimation-config.region-code" => Some(("riskJob.privacyMetric.deltaPresenceEstimationConfig.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "risk-job.privacy-metric.k-anonymity-config.entity-id.field.name" => Some(("riskJob.privacyMetric.kAnonymityConfig.entityId.field.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "risk-job.privacy-metric.k-map-estimation-config.region-code" => Some(("riskJob.privacyMetric.kMapEstimationConfig.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "risk-job.privacy-metric.l-diversity-config.sensitive-attribute.name" => Some(("riskJob.privacyMetric.lDiversityConfig.sensitiveAttribute.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "risk-job.privacy-metric.numerical-stats-config.field.name" => Some(("riskJob.privacyMetric.numericalStatsConfig.field.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "risk-job.source-table.dataset-id" => Some(("riskJob.sourceTable.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "risk-job.source-table.project-id" => Some(("riskJob.sourceTable.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "risk-job.source-table.table-id" => Some(("riskJob.sourceTable.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["big-query-options", "bucket-name", "bytes-limit-per-file", "bytes-limit-per-file-percent", "categorical-stats-config", "cloud-storage-options", "content-options", "dataset-id", "datastore-options", "delta-presence-estimation-config", "description", "enable-auto-population-of-timespan-config", "end-time", "entity-id", "exclude-info-types", "exclude-regex", "field", "file-set", "file-types", "files-limit-percent", "hybrid-options", "include-quote", "include-regex", "inspect-config", "inspect-job", "inspect-template-name", "job-id", "k-anonymity-config", "k-map-estimation-config", "kind", "l-diversity-config", "labels", "limits", "location-id", "max-findings-per-item", "max-findings-per-request", "min-likelihood", "name", "namespace-id", "numerical-stats-config", "partition-id", "privacy-metric", "project-id", "regex-file-set", "region-code", "required-finding-label-keys", "risk-job", "rows-limit", "rows-limit-percent", "sample-method", "sensitive-attribute", "source-table", "start-time", "storage-config", "table-id", "table-reference", "timespan-config", "timestamp-field", "url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2CreateDlpJobRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_dlp_jobs_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_dlp_jobs_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_dlp_jobs_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_dlp_jobs_finish(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::GooglePrivacyDlpV2FinishDlpJobRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_dlp_jobs_finish(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_dlp_jobs_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_dlp_jobs_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_dlp_jobs_hybrid_inspect(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "hybrid-item.finding-details.container-details.full-path" => Some(("hybridItem.findingDetails.containerDetails.fullPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hybrid-item.finding-details.container-details.project-id" => Some(("hybridItem.findingDetails.containerDetails.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hybrid-item.finding-details.container-details.relative-path" => Some(("hybridItem.findingDetails.containerDetails.relativePath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hybrid-item.finding-details.container-details.root-path" => Some(("hybridItem.findingDetails.containerDetails.rootPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hybrid-item.finding-details.container-details.type" => Some(("hybridItem.findingDetails.containerDetails.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hybrid-item.finding-details.container-details.update-time" => Some(("hybridItem.findingDetails.containerDetails.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hybrid-item.finding-details.container-details.version" => Some(("hybridItem.findingDetails.containerDetails.version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hybrid-item.finding-details.file-offset" => Some(("hybridItem.findingDetails.fileOffset", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hybrid-item.finding-details.labels" => Some(("hybridItem.findingDetails.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "hybrid-item.finding-details.row-offset" => Some(("hybridItem.findingDetails.rowOffset", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hybrid-item.item.byte-item.data" => Some(("hybridItem.item.byteItem.data", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hybrid-item.item.byte-item.type" => Some(("hybridItem.item.byteItem.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hybrid-item.item.value" => Some(("hybridItem.item.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["byte-item", "container-details", "data", "file-offset", "finding-details", "full-path", "hybrid-item", "item", "labels", "project-id", "relative-path", "root-path", "row-offset", "type", "update-time", "value", "version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2HybridInspectDlpJobRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_dlp_jobs_hybrid_inspect(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_dlp_jobs_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_dlp_jobs_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "type" => {
                    call = call.type_(value.unwrap_or(""));
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
                "location-id" => {
                    call = call.location_id(value.unwrap_or(""));
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
                                                                           v.extend(["filter", "location-id", "order-by", "page-size", "page-token", "type"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_image_redact(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "byte-item.data" => Some(("byteItem.data", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "byte-item.type" => Some(("byteItem.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "include-findings" => Some(("includeFindings", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-config.content-options" => Some(("inspectConfig.contentOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inspect-config.exclude-info-types" => Some(("inspectConfig.excludeInfoTypes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-config.include-quote" => Some(("inspectConfig.includeQuote", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-config.limits.max-findings-per-item" => Some(("inspectConfig.limits.maxFindingsPerItem", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-config.limits.max-findings-per-request" => Some(("inspectConfig.limits.maxFindingsPerRequest", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-config.min-likelihood" => Some(("inspectConfig.minLikelihood", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-id" => Some(("locationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["byte-item", "content-options", "data", "exclude-info-types", "include-findings", "include-quote", "inspect-config", "limits", "location-id", "max-findings-per-item", "max-findings-per-request", "min-likelihood", "type"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2RedactImageRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_image_redact(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_inspect_templates_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "inspect-template.create-time" => Some(("inspectTemplate.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.description" => Some(("inspectTemplate.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.display-name" => Some(("inspectTemplate.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.content-options" => Some(("inspectTemplate.inspectConfig.contentOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inspect-template.inspect-config.exclude-info-types" => Some(("inspectTemplate.inspectConfig.excludeInfoTypes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.include-quote" => Some(("inspectTemplate.inspectConfig.includeQuote", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.limits.max-findings-per-item" => Some(("inspectTemplate.inspectConfig.limits.maxFindingsPerItem", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.limits.max-findings-per-request" => Some(("inspectTemplate.inspectConfig.limits.maxFindingsPerRequest", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.min-likelihood" => Some(("inspectTemplate.inspectConfig.minLikelihood", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.name" => Some(("inspectTemplate.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.update-time" => Some(("inspectTemplate.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-id" => Some(("locationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "template-id" => Some(("templateId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["content-options", "create-time", "description", "display-name", "exclude-info-types", "include-quote", "inspect-config", "inspect-template", "limits", "location-id", "max-findings-per-item", "max-findings-per-request", "min-likelihood", "name", "template-id", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2CreateInspectTemplateRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_inspect_templates_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_inspect_templates_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_inspect_templates_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_inspect_templates_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_inspect_templates_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_inspect_templates_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_inspect_templates_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "location-id" => {
                    call = call.location_id(value.unwrap_or(""));
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
                                                                           v.extend(["location-id", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_inspect_templates_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "inspect-template.create-time" => Some(("inspectTemplate.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.description" => Some(("inspectTemplate.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.display-name" => Some(("inspectTemplate.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.content-options" => Some(("inspectTemplate.inspectConfig.contentOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inspect-template.inspect-config.exclude-info-types" => Some(("inspectTemplate.inspectConfig.excludeInfoTypes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.include-quote" => Some(("inspectTemplate.inspectConfig.includeQuote", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.limits.max-findings-per-item" => Some(("inspectTemplate.inspectConfig.limits.maxFindingsPerItem", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.limits.max-findings-per-request" => Some(("inspectTemplate.inspectConfig.limits.maxFindingsPerRequest", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.min-likelihood" => Some(("inspectTemplate.inspectConfig.minLikelihood", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.name" => Some(("inspectTemplate.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.update-time" => Some(("inspectTemplate.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-mask" => Some(("updateMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["content-options", "create-time", "description", "display-name", "exclude-info-types", "include-quote", "inspect-config", "inspect-template", "limits", "max-findings-per-item", "max-findings-per-request", "min-likelihood", "name", "update-mask", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2UpdateInspectTemplateRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_inspect_templates_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_job_triggers_activate(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::GooglePrivacyDlpV2ActivateJobTriggerRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_job_triggers_activate(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_job_triggers_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "job-trigger.create-time" => Some(("jobTrigger.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.description" => Some(("jobTrigger.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.display-name" => Some(("jobTrigger.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-config.content-options" => Some(("jobTrigger.inspectJob.inspectConfig.contentOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-trigger.inspect-job.inspect-config.exclude-info-types" => Some(("jobTrigger.inspectJob.inspectConfig.excludeInfoTypes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-config.include-quote" => Some(("jobTrigger.inspectJob.inspectConfig.includeQuote", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-config.limits.max-findings-per-item" => Some(("jobTrigger.inspectJob.inspectConfig.limits.maxFindingsPerItem", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-config.limits.max-findings-per-request" => Some(("jobTrigger.inspectJob.inspectConfig.limits.maxFindingsPerRequest", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-config.min-likelihood" => Some(("jobTrigger.inspectJob.inspectConfig.minLikelihood", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-template-name" => Some(("jobTrigger.inspectJob.inspectTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.rows-limit" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.rowsLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.rows-limit-percent" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.rowsLimitPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.sample-method" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.sampleMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.table-reference.dataset-id" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.tableReference.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.table-reference.project-id" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.tableReference.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.table-reference.table-id" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.tableReference.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.bytes-limit-per-file" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.bytesLimitPerFile", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.bytes-limit-per-file-percent" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.bytesLimitPerFilePercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.file-set.regex-file-set.bucket-name" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.fileSet.regexFileSet.bucketName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.file-set.regex-file-set.exclude-regex" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.fileSet.regexFileSet.excludeRegex", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.file-set.regex-file-set.include-regex" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.fileSet.regexFileSet.includeRegex", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.file-set.url" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.fileSet.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.file-types" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.fileTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.files-limit-percent" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.filesLimitPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.sample-method" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.sampleMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.datastore-options.kind.name" => Some(("jobTrigger.inspectJob.storageConfig.datastoreOptions.kind.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.datastore-options.partition-id.namespace-id" => Some(("jobTrigger.inspectJob.storageConfig.datastoreOptions.partitionId.namespaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.datastore-options.partition-id.project-id" => Some(("jobTrigger.inspectJob.storageConfig.datastoreOptions.partitionId.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.hybrid-options.description" => Some(("jobTrigger.inspectJob.storageConfig.hybridOptions.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.hybrid-options.labels" => Some(("jobTrigger.inspectJob.storageConfig.hybridOptions.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job-trigger.inspect-job.storage-config.hybrid-options.required-finding-label-keys" => Some(("jobTrigger.inspectJob.storageConfig.hybridOptions.requiredFindingLabelKeys", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-trigger.inspect-job.storage-config.timespan-config.enable-auto-population-of-timespan-config" => Some(("jobTrigger.inspectJob.storageConfig.timespanConfig.enableAutoPopulationOfTimespanConfig", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.timespan-config.end-time" => Some(("jobTrigger.inspectJob.storageConfig.timespanConfig.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.timespan-config.start-time" => Some(("jobTrigger.inspectJob.storageConfig.timespanConfig.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.timespan-config.timestamp-field.name" => Some(("jobTrigger.inspectJob.storageConfig.timespanConfig.timestampField.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.last-run-time" => Some(("jobTrigger.lastRunTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.name" => Some(("jobTrigger.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.status" => Some(("jobTrigger.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.update-time" => Some(("jobTrigger.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-id" => Some(("locationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "trigger-id" => Some(("triggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["big-query-options", "bucket-name", "bytes-limit-per-file", "bytes-limit-per-file-percent", "cloud-storage-options", "content-options", "create-time", "dataset-id", "datastore-options", "description", "display-name", "enable-auto-population-of-timespan-config", "end-time", "exclude-info-types", "exclude-regex", "file-set", "file-types", "files-limit-percent", "hybrid-options", "include-quote", "include-regex", "inspect-config", "inspect-job", "inspect-template-name", "job-trigger", "kind", "labels", "last-run-time", "limits", "location-id", "max-findings-per-item", "max-findings-per-request", "min-likelihood", "name", "namespace-id", "partition-id", "project-id", "regex-file-set", "required-finding-label-keys", "rows-limit", "rows-limit-percent", "sample-method", "start-time", "status", "storage-config", "table-id", "table-reference", "timespan-config", "timestamp-field", "trigger-id", "update-time", "url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2CreateJobTriggerRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_job_triggers_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_job_triggers_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_job_triggers_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_job_triggers_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_job_triggers_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_job_triggers_hybrid_inspect(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "hybrid-item.finding-details.container-details.full-path" => Some(("hybridItem.findingDetails.containerDetails.fullPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hybrid-item.finding-details.container-details.project-id" => Some(("hybridItem.findingDetails.containerDetails.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hybrid-item.finding-details.container-details.relative-path" => Some(("hybridItem.findingDetails.containerDetails.relativePath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hybrid-item.finding-details.container-details.root-path" => Some(("hybridItem.findingDetails.containerDetails.rootPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hybrid-item.finding-details.container-details.type" => Some(("hybridItem.findingDetails.containerDetails.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hybrid-item.finding-details.container-details.update-time" => Some(("hybridItem.findingDetails.containerDetails.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hybrid-item.finding-details.container-details.version" => Some(("hybridItem.findingDetails.containerDetails.version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hybrid-item.finding-details.file-offset" => Some(("hybridItem.findingDetails.fileOffset", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hybrid-item.finding-details.labels" => Some(("hybridItem.findingDetails.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "hybrid-item.finding-details.row-offset" => Some(("hybridItem.findingDetails.rowOffset", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hybrid-item.item.byte-item.data" => Some(("hybridItem.item.byteItem.data", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hybrid-item.item.byte-item.type" => Some(("hybridItem.item.byteItem.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "hybrid-item.item.value" => Some(("hybridItem.item.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["byte-item", "container-details", "data", "file-offset", "finding-details", "full-path", "hybrid-item", "item", "labels", "project-id", "relative-path", "root-path", "row-offset", "type", "update-time", "value", "version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2HybridInspectJobTriggerRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_job_triggers_hybrid_inspect(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_job_triggers_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_job_triggers_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "type" => {
                    call = call.type_(value.unwrap_or(""));
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
                "location-id" => {
                    call = call.location_id(value.unwrap_or(""));
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
                                                                           v.extend(["filter", "location-id", "order-by", "page-size", "page-token", "type"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_job_triggers_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "job-trigger.create-time" => Some(("jobTrigger.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.description" => Some(("jobTrigger.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.display-name" => Some(("jobTrigger.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-config.content-options" => Some(("jobTrigger.inspectJob.inspectConfig.contentOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-trigger.inspect-job.inspect-config.exclude-info-types" => Some(("jobTrigger.inspectJob.inspectConfig.excludeInfoTypes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-config.include-quote" => Some(("jobTrigger.inspectJob.inspectConfig.includeQuote", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-config.limits.max-findings-per-item" => Some(("jobTrigger.inspectJob.inspectConfig.limits.maxFindingsPerItem", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-config.limits.max-findings-per-request" => Some(("jobTrigger.inspectJob.inspectConfig.limits.maxFindingsPerRequest", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-config.min-likelihood" => Some(("jobTrigger.inspectJob.inspectConfig.minLikelihood", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-template-name" => Some(("jobTrigger.inspectJob.inspectTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.rows-limit" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.rowsLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.rows-limit-percent" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.rowsLimitPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.sample-method" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.sampleMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.table-reference.dataset-id" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.tableReference.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.table-reference.project-id" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.tableReference.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.table-reference.table-id" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.tableReference.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.bytes-limit-per-file" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.bytesLimitPerFile", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.bytes-limit-per-file-percent" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.bytesLimitPerFilePercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.file-set.regex-file-set.bucket-name" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.fileSet.regexFileSet.bucketName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.file-set.regex-file-set.exclude-regex" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.fileSet.regexFileSet.excludeRegex", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.file-set.regex-file-set.include-regex" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.fileSet.regexFileSet.includeRegex", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.file-set.url" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.fileSet.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.file-types" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.fileTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.files-limit-percent" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.filesLimitPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.sample-method" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.sampleMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.datastore-options.kind.name" => Some(("jobTrigger.inspectJob.storageConfig.datastoreOptions.kind.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.datastore-options.partition-id.namespace-id" => Some(("jobTrigger.inspectJob.storageConfig.datastoreOptions.partitionId.namespaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.datastore-options.partition-id.project-id" => Some(("jobTrigger.inspectJob.storageConfig.datastoreOptions.partitionId.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.hybrid-options.description" => Some(("jobTrigger.inspectJob.storageConfig.hybridOptions.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.hybrid-options.labels" => Some(("jobTrigger.inspectJob.storageConfig.hybridOptions.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "job-trigger.inspect-job.storage-config.hybrid-options.required-finding-label-keys" => Some(("jobTrigger.inspectJob.storageConfig.hybridOptions.requiredFindingLabelKeys", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-trigger.inspect-job.storage-config.timespan-config.enable-auto-population-of-timespan-config" => Some(("jobTrigger.inspectJob.storageConfig.timespanConfig.enableAutoPopulationOfTimespanConfig", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.timespan-config.end-time" => Some(("jobTrigger.inspectJob.storageConfig.timespanConfig.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.timespan-config.start-time" => Some(("jobTrigger.inspectJob.storageConfig.timespanConfig.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.timespan-config.timestamp-field.name" => Some(("jobTrigger.inspectJob.storageConfig.timespanConfig.timestampField.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.last-run-time" => Some(("jobTrigger.lastRunTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.name" => Some(("jobTrigger.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.status" => Some(("jobTrigger.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.update-time" => Some(("jobTrigger.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-mask" => Some(("updateMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["big-query-options", "bucket-name", "bytes-limit-per-file", "bytes-limit-per-file-percent", "cloud-storage-options", "content-options", "create-time", "dataset-id", "datastore-options", "description", "display-name", "enable-auto-population-of-timespan-config", "end-time", "exclude-info-types", "exclude-regex", "file-set", "file-types", "files-limit-percent", "hybrid-options", "include-quote", "include-regex", "inspect-config", "inspect-job", "inspect-template-name", "job-trigger", "kind", "labels", "last-run-time", "limits", "max-findings-per-item", "max-findings-per-request", "min-likelihood", "name", "namespace-id", "partition-id", "project-id", "regex-file-set", "required-finding-label-keys", "rows-limit", "rows-limit-percent", "sample-method", "start-time", "status", "storage-config", "table-id", "table-reference", "timespan-config", "timestamp-field", "update-mask", "update-time", "url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2UpdateJobTriggerRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_job_triggers_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_project_data_profiles_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_project_data_profiles_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_project_data_profiles_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_project_data_profiles_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
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
                                                                           v.extend(["filter", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_stored_info_types_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "config.description" => Some(("config.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.dictionary.cloud-storage-path.path" => Some(("config.dictionary.cloudStoragePath.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.dictionary.word-list.words" => Some(("config.dictionary.wordList.words", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.display-name" => Some(("config.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.field.name" => Some(("config.largeCustomDictionary.bigQueryField.field.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.table.dataset-id" => Some(("config.largeCustomDictionary.bigQueryField.table.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.table.project-id" => Some(("config.largeCustomDictionary.bigQueryField.table.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.table.table-id" => Some(("config.largeCustomDictionary.bigQueryField.table.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.cloud-storage-file-set.url" => Some(("config.largeCustomDictionary.cloudStorageFileSet.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.output-path.path" => Some(("config.largeCustomDictionary.outputPath.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.regex.group-indexes" => Some(("config.regex.groupIndexes", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Vec })),
                    "config.regex.pattern" => Some(("config.regex.pattern", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-id" => Some(("locationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "stored-info-type-id" => Some(("storedInfoTypeId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["big-query-field", "cloud-storage-file-set", "cloud-storage-path", "config", "dataset-id", "description", "dictionary", "display-name", "field", "group-indexes", "large-custom-dictionary", "location-id", "name", "output-path", "path", "pattern", "project-id", "regex", "stored-info-type-id", "table", "table-id", "url", "word-list", "words"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2CreateStoredInfoTypeRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_stored_info_types_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_stored_info_types_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_stored_info_types_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_stored_info_types_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_stored_info_types_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_stored_info_types_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_stored_info_types_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "location-id" => {
                    call = call.location_id(value.unwrap_or(""));
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
                                                                           v.extend(["location-id", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_stored_info_types_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "config.description" => Some(("config.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.dictionary.cloud-storage-path.path" => Some(("config.dictionary.cloudStoragePath.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.dictionary.word-list.words" => Some(("config.dictionary.wordList.words", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.display-name" => Some(("config.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.field.name" => Some(("config.largeCustomDictionary.bigQueryField.field.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.table.dataset-id" => Some(("config.largeCustomDictionary.bigQueryField.table.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.table.project-id" => Some(("config.largeCustomDictionary.bigQueryField.table.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.table.table-id" => Some(("config.largeCustomDictionary.bigQueryField.table.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.cloud-storage-file-set.url" => Some(("config.largeCustomDictionary.cloudStorageFileSet.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.output-path.path" => Some(("config.largeCustomDictionary.outputPath.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.regex.group-indexes" => Some(("config.regex.groupIndexes", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Vec })),
                    "config.regex.pattern" => Some(("config.regex.pattern", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-mask" => Some(("updateMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["big-query-field", "cloud-storage-file-set", "cloud-storage-path", "config", "dataset-id", "description", "dictionary", "display-name", "field", "group-indexes", "large-custom-dictionary", "name", "output-path", "path", "pattern", "project-id", "regex", "table", "table-id", "update-mask", "url", "word-list", "words"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2UpdateStoredInfoTypeRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_stored_info_types_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_table_data_profiles_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_table_data_profiles_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_table_data_profiles_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_table_data_profiles_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_table_data_profiles_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_table_data_profiles_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
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
                                                                           v.extend(["filter", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_stored_info_types_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "config.description" => Some(("config.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.dictionary.cloud-storage-path.path" => Some(("config.dictionary.cloudStoragePath.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.dictionary.word-list.words" => Some(("config.dictionary.wordList.words", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.display-name" => Some(("config.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.field.name" => Some(("config.largeCustomDictionary.bigQueryField.field.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.table.dataset-id" => Some(("config.largeCustomDictionary.bigQueryField.table.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.table.project-id" => Some(("config.largeCustomDictionary.bigQueryField.table.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.table.table-id" => Some(("config.largeCustomDictionary.bigQueryField.table.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.cloud-storage-file-set.url" => Some(("config.largeCustomDictionary.cloudStorageFileSet.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.output-path.path" => Some(("config.largeCustomDictionary.outputPath.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.regex.group-indexes" => Some(("config.regex.groupIndexes", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Vec })),
                    "config.regex.pattern" => Some(("config.regex.pattern", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-id" => Some(("locationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "stored-info-type-id" => Some(("storedInfoTypeId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["big-query-field", "cloud-storage-file-set", "cloud-storage-path", "config", "dataset-id", "description", "dictionary", "display-name", "field", "group-indexes", "large-custom-dictionary", "location-id", "name", "output-path", "path", "pattern", "project-id", "regex", "stored-info-type-id", "table", "table-id", "url", "word-list", "words"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2CreateStoredInfoTypeRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().stored_info_types_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_stored_info_types_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().stored_info_types_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_stored_info_types_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().stored_info_types_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_stored_info_types_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().stored_info_types_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "location-id" => {
                    call = call.location_id(value.unwrap_or(""));
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
                                                                           v.extend(["location-id", "order-by", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_stored_info_types_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "config.description" => Some(("config.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.dictionary.cloud-storage-path.path" => Some(("config.dictionary.cloudStoragePath.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.dictionary.word-list.words" => Some(("config.dictionary.wordList.words", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.display-name" => Some(("config.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.field.name" => Some(("config.largeCustomDictionary.bigQueryField.field.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.table.dataset-id" => Some(("config.largeCustomDictionary.bigQueryField.table.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.table.project-id" => Some(("config.largeCustomDictionary.bigQueryField.table.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.table.table-id" => Some(("config.largeCustomDictionary.bigQueryField.table.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.cloud-storage-file-set.url" => Some(("config.largeCustomDictionary.cloudStorageFileSet.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.output-path.path" => Some(("config.largeCustomDictionary.outputPath.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.regex.group-indexes" => Some(("config.regex.groupIndexes", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Vec })),
                    "config.regex.pattern" => Some(("config.regex.pattern", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-mask" => Some(("updateMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["big-query-field", "cloud-storage-file-set", "cloud-storage-path", "config", "dataset-id", "description", "dictionary", "display-name", "field", "group-indexes", "large-custom-dictionary", "name", "output-path", "path", "pattern", "project-id", "regex", "table", "table-id", "update-mask", "url", "word-list", "words"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2UpdateStoredInfoTypeRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().stored_info_types_patch(request, opt.value_of("name").unwrap_or(""));
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
            ("info-types", Some(opt)) => {
                match opt.subcommand() {
                    ("list", Some(opt)) => {
                        call_result = self._info_types_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("info-types".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("locations", Some(opt)) => {
                match opt.subcommand() {
                    ("info-types-list", Some(opt)) => {
                        call_result = self._locations_info_types_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("locations".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("organizations", Some(opt)) => {
                match opt.subcommand() {
                    ("deidentify-templates-create", Some(opt)) => {
                        call_result = self._organizations_deidentify_templates_create(opt, dry_run, &mut err).await;
                    },
                    ("deidentify-templates-delete", Some(opt)) => {
                        call_result = self._organizations_deidentify_templates_delete(opt, dry_run, &mut err).await;
                    },
                    ("deidentify-templates-get", Some(opt)) => {
                        call_result = self._organizations_deidentify_templates_get(opt, dry_run, &mut err).await;
                    },
                    ("deidentify-templates-list", Some(opt)) => {
                        call_result = self._organizations_deidentify_templates_list(opt, dry_run, &mut err).await;
                    },
                    ("deidentify-templates-patch", Some(opt)) => {
                        call_result = self._organizations_deidentify_templates_patch(opt, dry_run, &mut err).await;
                    },
                    ("inspect-templates-create", Some(opt)) => {
                        call_result = self._organizations_inspect_templates_create(opt, dry_run, &mut err).await;
                    },
                    ("inspect-templates-delete", Some(opt)) => {
                        call_result = self._organizations_inspect_templates_delete(opt, dry_run, &mut err).await;
                    },
                    ("inspect-templates-get", Some(opt)) => {
                        call_result = self._organizations_inspect_templates_get(opt, dry_run, &mut err).await;
                    },
                    ("inspect-templates-list", Some(opt)) => {
                        call_result = self._organizations_inspect_templates_list(opt, dry_run, &mut err).await;
                    },
                    ("inspect-templates-patch", Some(opt)) => {
                        call_result = self._organizations_inspect_templates_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-column-data-profiles-get", Some(opt)) => {
                        call_result = self._organizations_locations_column_data_profiles_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-column-data-profiles-list", Some(opt)) => {
                        call_result = self._organizations_locations_column_data_profiles_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-connections-create", Some(opt)) => {
                        call_result = self._organizations_locations_connections_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-connections-delete", Some(opt)) => {
                        call_result = self._organizations_locations_connections_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-connections-get", Some(opt)) => {
                        call_result = self._organizations_locations_connections_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-connections-patch", Some(opt)) => {
                        call_result = self._organizations_locations_connections_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-connections-search", Some(opt)) => {
                        call_result = self._organizations_locations_connections_search(opt, dry_run, &mut err).await;
                    },
                    ("locations-deidentify-templates-create", Some(opt)) => {
                        call_result = self._organizations_locations_deidentify_templates_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-deidentify-templates-delete", Some(opt)) => {
                        call_result = self._organizations_locations_deidentify_templates_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-deidentify-templates-get", Some(opt)) => {
                        call_result = self._organizations_locations_deidentify_templates_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-deidentify-templates-list", Some(opt)) => {
                        call_result = self._organizations_locations_deidentify_templates_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-deidentify-templates-patch", Some(opt)) => {
                        call_result = self._organizations_locations_deidentify_templates_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-discovery-configs-create", Some(opt)) => {
                        call_result = self._organizations_locations_discovery_configs_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-discovery-configs-delete", Some(opt)) => {
                        call_result = self._organizations_locations_discovery_configs_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-discovery-configs-get", Some(opt)) => {
                        call_result = self._organizations_locations_discovery_configs_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-discovery-configs-list", Some(opt)) => {
                        call_result = self._organizations_locations_discovery_configs_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-discovery-configs-patch", Some(opt)) => {
                        call_result = self._organizations_locations_discovery_configs_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-dlp-jobs-list", Some(opt)) => {
                        call_result = self._organizations_locations_dlp_jobs_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-inspect-templates-create", Some(opt)) => {
                        call_result = self._organizations_locations_inspect_templates_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-inspect-templates-delete", Some(opt)) => {
                        call_result = self._organizations_locations_inspect_templates_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-inspect-templates-get", Some(opt)) => {
                        call_result = self._organizations_locations_inspect_templates_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-inspect-templates-list", Some(opt)) => {
                        call_result = self._organizations_locations_inspect_templates_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-inspect-templates-patch", Some(opt)) => {
                        call_result = self._organizations_locations_inspect_templates_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-job-triggers-create", Some(opt)) => {
                        call_result = self._organizations_locations_job_triggers_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-job-triggers-delete", Some(opt)) => {
                        call_result = self._organizations_locations_job_triggers_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-job-triggers-get", Some(opt)) => {
                        call_result = self._organizations_locations_job_triggers_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-job-triggers-list", Some(opt)) => {
                        call_result = self._organizations_locations_job_triggers_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-job-triggers-patch", Some(opt)) => {
                        call_result = self._organizations_locations_job_triggers_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-project-data-profiles-get", Some(opt)) => {
                        call_result = self._organizations_locations_project_data_profiles_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-project-data-profiles-list", Some(opt)) => {
                        call_result = self._organizations_locations_project_data_profiles_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-stored-info-types-create", Some(opt)) => {
                        call_result = self._organizations_locations_stored_info_types_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-stored-info-types-delete", Some(opt)) => {
                        call_result = self._organizations_locations_stored_info_types_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-stored-info-types-get", Some(opt)) => {
                        call_result = self._organizations_locations_stored_info_types_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-stored-info-types-list", Some(opt)) => {
                        call_result = self._organizations_locations_stored_info_types_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-stored-info-types-patch", Some(opt)) => {
                        call_result = self._organizations_locations_stored_info_types_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-table-data-profiles-delete", Some(opt)) => {
                        call_result = self._organizations_locations_table_data_profiles_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-table-data-profiles-get", Some(opt)) => {
                        call_result = self._organizations_locations_table_data_profiles_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-table-data-profiles-list", Some(opt)) => {
                        call_result = self._organizations_locations_table_data_profiles_list(opt, dry_run, &mut err).await;
                    },
                    ("stored-info-types-create", Some(opt)) => {
                        call_result = self._organizations_stored_info_types_create(opt, dry_run, &mut err).await;
                    },
                    ("stored-info-types-delete", Some(opt)) => {
                        call_result = self._organizations_stored_info_types_delete(opt, dry_run, &mut err).await;
                    },
                    ("stored-info-types-get", Some(opt)) => {
                        call_result = self._organizations_stored_info_types_get(opt, dry_run, &mut err).await;
                    },
                    ("stored-info-types-list", Some(opt)) => {
                        call_result = self._organizations_stored_info_types_list(opt, dry_run, &mut err).await;
                    },
                    ("stored-info-types-patch", Some(opt)) => {
                        call_result = self._organizations_stored_info_types_patch(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("organizations".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("projects", Some(opt)) => {
                match opt.subcommand() {
                    ("content-deidentify", Some(opt)) => {
                        call_result = self._projects_content_deidentify(opt, dry_run, &mut err).await;
                    },
                    ("content-inspect", Some(opt)) => {
                        call_result = self._projects_content_inspect(opt, dry_run, &mut err).await;
                    },
                    ("content-reidentify", Some(opt)) => {
                        call_result = self._projects_content_reidentify(opt, dry_run, &mut err).await;
                    },
                    ("deidentify-templates-create", Some(opt)) => {
                        call_result = self._projects_deidentify_templates_create(opt, dry_run, &mut err).await;
                    },
                    ("deidentify-templates-delete", Some(opt)) => {
                        call_result = self._projects_deidentify_templates_delete(opt, dry_run, &mut err).await;
                    },
                    ("deidentify-templates-get", Some(opt)) => {
                        call_result = self._projects_deidentify_templates_get(opt, dry_run, &mut err).await;
                    },
                    ("deidentify-templates-list", Some(opt)) => {
                        call_result = self._projects_deidentify_templates_list(opt, dry_run, &mut err).await;
                    },
                    ("deidentify-templates-patch", Some(opt)) => {
                        call_result = self._projects_deidentify_templates_patch(opt, dry_run, &mut err).await;
                    },
                    ("dlp-jobs-cancel", Some(opt)) => {
                        call_result = self._projects_dlp_jobs_cancel(opt, dry_run, &mut err).await;
                    },
                    ("dlp-jobs-create", Some(opt)) => {
                        call_result = self._projects_dlp_jobs_create(opt, dry_run, &mut err).await;
                    },
                    ("dlp-jobs-delete", Some(opt)) => {
                        call_result = self._projects_dlp_jobs_delete(opt, dry_run, &mut err).await;
                    },
                    ("dlp-jobs-get", Some(opt)) => {
                        call_result = self._projects_dlp_jobs_get(opt, dry_run, &mut err).await;
                    },
                    ("dlp-jobs-list", Some(opt)) => {
                        call_result = self._projects_dlp_jobs_list(opt, dry_run, &mut err).await;
                    },
                    ("image-redact", Some(opt)) => {
                        call_result = self._projects_image_redact(opt, dry_run, &mut err).await;
                    },
                    ("inspect-templates-create", Some(opt)) => {
                        call_result = self._projects_inspect_templates_create(opt, dry_run, &mut err).await;
                    },
                    ("inspect-templates-delete", Some(opt)) => {
                        call_result = self._projects_inspect_templates_delete(opt, dry_run, &mut err).await;
                    },
                    ("inspect-templates-get", Some(opt)) => {
                        call_result = self._projects_inspect_templates_get(opt, dry_run, &mut err).await;
                    },
                    ("inspect-templates-list", Some(opt)) => {
                        call_result = self._projects_inspect_templates_list(opt, dry_run, &mut err).await;
                    },
                    ("inspect-templates-patch", Some(opt)) => {
                        call_result = self._projects_inspect_templates_patch(opt, dry_run, &mut err).await;
                    },
                    ("job-triggers-activate", Some(opt)) => {
                        call_result = self._projects_job_triggers_activate(opt, dry_run, &mut err).await;
                    },
                    ("job-triggers-create", Some(opt)) => {
                        call_result = self._projects_job_triggers_create(opt, dry_run, &mut err).await;
                    },
                    ("job-triggers-delete", Some(opt)) => {
                        call_result = self._projects_job_triggers_delete(opt, dry_run, &mut err).await;
                    },
                    ("job-triggers-get", Some(opt)) => {
                        call_result = self._projects_job_triggers_get(opt, dry_run, &mut err).await;
                    },
                    ("job-triggers-list", Some(opt)) => {
                        call_result = self._projects_job_triggers_list(opt, dry_run, &mut err).await;
                    },
                    ("job-triggers-patch", Some(opt)) => {
                        call_result = self._projects_job_triggers_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-column-data-profiles-get", Some(opt)) => {
                        call_result = self._projects_locations_column_data_profiles_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-column-data-profiles-list", Some(opt)) => {
                        call_result = self._projects_locations_column_data_profiles_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-connections-create", Some(opt)) => {
                        call_result = self._projects_locations_connections_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-connections-delete", Some(opt)) => {
                        call_result = self._projects_locations_connections_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-connections-get", Some(opt)) => {
                        call_result = self._projects_locations_connections_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-connections-list", Some(opt)) => {
                        call_result = self._projects_locations_connections_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-connections-patch", Some(opt)) => {
                        call_result = self._projects_locations_connections_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-connections-search", Some(opt)) => {
                        call_result = self._projects_locations_connections_search(opt, dry_run, &mut err).await;
                    },
                    ("locations-content-deidentify", Some(opt)) => {
                        call_result = self._projects_locations_content_deidentify(opt, dry_run, &mut err).await;
                    },
                    ("locations-content-inspect", Some(opt)) => {
                        call_result = self._projects_locations_content_inspect(opt, dry_run, &mut err).await;
                    },
                    ("locations-content-reidentify", Some(opt)) => {
                        call_result = self._projects_locations_content_reidentify(opt, dry_run, &mut err).await;
                    },
                    ("locations-deidentify-templates-create", Some(opt)) => {
                        call_result = self._projects_locations_deidentify_templates_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-deidentify-templates-delete", Some(opt)) => {
                        call_result = self._projects_locations_deidentify_templates_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-deidentify-templates-get", Some(opt)) => {
                        call_result = self._projects_locations_deidentify_templates_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-deidentify-templates-list", Some(opt)) => {
                        call_result = self._projects_locations_deidentify_templates_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-deidentify-templates-patch", Some(opt)) => {
                        call_result = self._projects_locations_deidentify_templates_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-discovery-configs-create", Some(opt)) => {
                        call_result = self._projects_locations_discovery_configs_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-discovery-configs-delete", Some(opt)) => {
                        call_result = self._projects_locations_discovery_configs_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-discovery-configs-get", Some(opt)) => {
                        call_result = self._projects_locations_discovery_configs_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-discovery-configs-list", Some(opt)) => {
                        call_result = self._projects_locations_discovery_configs_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-discovery-configs-patch", Some(opt)) => {
                        call_result = self._projects_locations_discovery_configs_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-dlp-jobs-cancel", Some(opt)) => {
                        call_result = self._projects_locations_dlp_jobs_cancel(opt, dry_run, &mut err).await;
                    },
                    ("locations-dlp-jobs-create", Some(opt)) => {
                        call_result = self._projects_locations_dlp_jobs_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-dlp-jobs-delete", Some(opt)) => {
                        call_result = self._projects_locations_dlp_jobs_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-dlp-jobs-finish", Some(opt)) => {
                        call_result = self._projects_locations_dlp_jobs_finish(opt, dry_run, &mut err).await;
                    },
                    ("locations-dlp-jobs-get", Some(opt)) => {
                        call_result = self._projects_locations_dlp_jobs_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-dlp-jobs-hybrid-inspect", Some(opt)) => {
                        call_result = self._projects_locations_dlp_jobs_hybrid_inspect(opt, dry_run, &mut err).await;
                    },
                    ("locations-dlp-jobs-list", Some(opt)) => {
                        call_result = self._projects_locations_dlp_jobs_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-image-redact", Some(opt)) => {
                        call_result = self._projects_locations_image_redact(opt, dry_run, &mut err).await;
                    },
                    ("locations-inspect-templates-create", Some(opt)) => {
                        call_result = self._projects_locations_inspect_templates_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-inspect-templates-delete", Some(opt)) => {
                        call_result = self._projects_locations_inspect_templates_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-inspect-templates-get", Some(opt)) => {
                        call_result = self._projects_locations_inspect_templates_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-inspect-templates-list", Some(opt)) => {
                        call_result = self._projects_locations_inspect_templates_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-inspect-templates-patch", Some(opt)) => {
                        call_result = self._projects_locations_inspect_templates_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-job-triggers-activate", Some(opt)) => {
                        call_result = self._projects_locations_job_triggers_activate(opt, dry_run, &mut err).await;
                    },
                    ("locations-job-triggers-create", Some(opt)) => {
                        call_result = self._projects_locations_job_triggers_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-job-triggers-delete", Some(opt)) => {
                        call_result = self._projects_locations_job_triggers_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-job-triggers-get", Some(opt)) => {
                        call_result = self._projects_locations_job_triggers_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-job-triggers-hybrid-inspect", Some(opt)) => {
                        call_result = self._projects_locations_job_triggers_hybrid_inspect(opt, dry_run, &mut err).await;
                    },
                    ("locations-job-triggers-list", Some(opt)) => {
                        call_result = self._projects_locations_job_triggers_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-job-triggers-patch", Some(opt)) => {
                        call_result = self._projects_locations_job_triggers_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-project-data-profiles-get", Some(opt)) => {
                        call_result = self._projects_locations_project_data_profiles_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-project-data-profiles-list", Some(opt)) => {
                        call_result = self._projects_locations_project_data_profiles_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-stored-info-types-create", Some(opt)) => {
                        call_result = self._projects_locations_stored_info_types_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-stored-info-types-delete", Some(opt)) => {
                        call_result = self._projects_locations_stored_info_types_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-stored-info-types-get", Some(opt)) => {
                        call_result = self._projects_locations_stored_info_types_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-stored-info-types-list", Some(opt)) => {
                        call_result = self._projects_locations_stored_info_types_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-stored-info-types-patch", Some(opt)) => {
                        call_result = self._projects_locations_stored_info_types_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-table-data-profiles-delete", Some(opt)) => {
                        call_result = self._projects_locations_table_data_profiles_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-table-data-profiles-get", Some(opt)) => {
                        call_result = self._projects_locations_table_data_profiles_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-table-data-profiles-list", Some(opt)) => {
                        call_result = self._projects_locations_table_data_profiles_list(opt, dry_run, &mut err).await;
                    },
                    ("stored-info-types-create", Some(opt)) => {
                        call_result = self._projects_stored_info_types_create(opt, dry_run, &mut err).await;
                    },
                    ("stored-info-types-delete", Some(opt)) => {
                        call_result = self._projects_stored_info_types_delete(opt, dry_run, &mut err).await;
                    },
                    ("stored-info-types-get", Some(opt)) => {
                        call_result = self._projects_stored_info_types_get(opt, dry_run, &mut err).await;
                    },
                    ("stored-info-types-list", Some(opt)) => {
                        call_result = self._projects_stored_info_types_list(opt, dry_run, &mut err).await;
                    },
                    ("stored-info-types-patch", Some(opt)) => {
                        call_result = self._projects_stored_info_types_patch(opt, dry_run, &mut err).await;
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

            match client::application_secret_from_directory(&config_dir, "dlp2-secret.json",
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
        ).persist_tokens_to_disk(format!("{}/dlp2", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::DLP::new(client, auth),
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
        ("info-types", "methods: 'list'", vec![
            ("list",
                    Some(r##"Returns a list of the sensitive information types that DLP API supports. See https://cloud.google.com/sensitive-data-protection/docs/infotypes-reference to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/info-types_list",
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
        
        ("locations", "methods: 'info-types-list'", vec![
            ("info-types-list",
                    Some(r##"Returns a list of the sensitive information types that DLP API supports. See https://cloud.google.com/sensitive-data-protection/docs/infotypes-reference to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/locations_info-types-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The parent resource name. The format of this value is as follows: locations/ LOCATION_ID"##),
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
        
        ("organizations", "methods: 'deidentify-templates-create', 'deidentify-templates-delete', 'deidentify-templates-get', 'deidentify-templates-list', 'deidentify-templates-patch', 'inspect-templates-create', 'inspect-templates-delete', 'inspect-templates-get', 'inspect-templates-list', 'inspect-templates-patch', 'locations-column-data-profiles-get', 'locations-column-data-profiles-list', 'locations-connections-create', 'locations-connections-delete', 'locations-connections-get', 'locations-connections-patch', 'locations-connections-search', 'locations-deidentify-templates-create', 'locations-deidentify-templates-delete', 'locations-deidentify-templates-get', 'locations-deidentify-templates-list', 'locations-deidentify-templates-patch', 'locations-discovery-configs-create', 'locations-discovery-configs-delete', 'locations-discovery-configs-get', 'locations-discovery-configs-list', 'locations-discovery-configs-patch', 'locations-dlp-jobs-list', 'locations-inspect-templates-create', 'locations-inspect-templates-delete', 'locations-inspect-templates-get', 'locations-inspect-templates-list', 'locations-inspect-templates-patch', 'locations-job-triggers-create', 'locations-job-triggers-delete', 'locations-job-triggers-get', 'locations-job-triggers-list', 'locations-job-triggers-patch', 'locations-project-data-profiles-get', 'locations-project-data-profiles-list', 'locations-stored-info-types-create', 'locations-stored-info-types-delete', 'locations-stored-info-types-get', 'locations-stored-info-types-list', 'locations-stored-info-types-patch', 'locations-table-data-profiles-delete', 'locations-table-data-profiles-get', 'locations-table-data-profiles-list', 'stored-info-types-create', 'stored-info-types-delete', 'stored-info-types-get', 'stored-info-types-list' and 'stored-info-types-patch'", vec![
            ("deidentify-templates-create",
                    Some(r##"Creates a DeidentifyTemplate for reusing frequently used configuration for de-identifying content, images, and storage. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_deidentify-templates-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID + Organizations scope, location specified: `organizations/`ORG_ID`/locations/`LOCATION_ID + Organizations scope, no location specified (defaults to global): `organizations/`ORG_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("deidentify-templates-delete",
                    Some(r##"Deletes a DeidentifyTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_deidentify-templates-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the organization and deidentify template to be deleted, for example `organizations/433245324/deidentifyTemplates/432452342` or projects/project-id/deidentifyTemplates/432452342."##),
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
            ("deidentify-templates-get",
                    Some(r##"Gets a DeidentifyTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_deidentify-templates-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the organization and deidentify template to be read, for example `organizations/433245324/deidentifyTemplates/432452342` or projects/project-id/deidentifyTemplates/432452342."##),
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
            ("deidentify-templates-list",
                    Some(r##"Lists DeidentifyTemplates. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_deidentify-templates-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID + Organizations scope, location specified: `organizations/`ORG_ID`/locations/`LOCATION_ID + Organizations scope, no location specified (defaults to global): `organizations/`ORG_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("deidentify-templates-patch",
                    Some(r##"Updates the DeidentifyTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_deidentify-templates-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of organization and deidentify template to be updated, for example `organizations/433245324/deidentifyTemplates/432452342` or projects/project-id/deidentifyTemplates/432452342."##),
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
            ("inspect-templates-create",
                    Some(r##"Creates an InspectTemplate for reusing frequently used configuration for inspecting content, images, and storage. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_inspect-templates-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID + Organizations scope, location specified: `organizations/`ORG_ID`/locations/`LOCATION_ID + Organizations scope, no location specified (defaults to global): `organizations/`ORG_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("inspect-templates-delete",
                    Some(r##"Deletes an InspectTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_inspect-templates-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the organization and inspectTemplate to be deleted, for example `organizations/433245324/inspectTemplates/432452342` or projects/project-id/inspectTemplates/432452342."##),
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
            ("inspect-templates-get",
                    Some(r##"Gets an InspectTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_inspect-templates-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the organization and inspectTemplate to be read, for example `organizations/433245324/inspectTemplates/432452342` or projects/project-id/inspectTemplates/432452342."##),
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
            ("inspect-templates-list",
                    Some(r##"Lists InspectTemplates. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_inspect-templates-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID + Organizations scope, location specified: `organizations/`ORG_ID`/locations/`LOCATION_ID + Organizations scope, no location specified (defaults to global): `organizations/`ORG_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("inspect-templates-patch",
                    Some(r##"Updates the InspectTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_inspect-templates-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of organization and inspectTemplate to be updated, for example `organizations/433245324/inspectTemplates/432452342` or projects/project-id/inspectTemplates/432452342."##),
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
            ("locations-column-data-profiles-get",
                    Some(r##"Gets a column data profile."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_locations-column-data-profiles-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name, for example `organizations/12345/locations/us/columnDataProfiles/53234423`."##),
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
            ("locations-column-data-profiles-list",
                    Some(r##"Lists column data profiles for an organization."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_locations-column-data-profiles-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Resource name of the organization or project, for example `organizations/433245324/locations/europe` or `projects/project-id/locations/asia`."##),
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
            ("locations-connections-create",
                    Some(r##"Create a Connection to an external data source."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_locations-connections-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization): + Projects scope: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Organizations scope: `organizations/`ORG_ID`/locations/`LOCATION_ID"##),
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
            ("locations-connections-delete",
                    Some(r##"Delete a Connection."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_locations-connections-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the Connection to be deleted, in the format: `projects/{project}/locations/{location}/connections/{connection}`."##),
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
            ("locations-connections-get",
                    Some(r##"Get a Connection by name."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_locations-connections-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name in the format: `projects/{project}/locations/{location}/connections/{connection}`."##),
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
            ("locations-connections-patch",
                    Some(r##"Update a Connection."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_locations-connections-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name in the format: `projects/{project}/locations/{location}/connections/{connection}`."##),
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
            ("locations-connections-search",
                    Some(r##"Searches for Connections in a parent."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_locations-connections-search",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent name, typically an organization, without location. For example: `organizations/12345678`."##),
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
            ("locations-deidentify-templates-create",
                    Some(r##"Creates a DeidentifyTemplate for reusing frequently used configuration for de-identifying content, images, and storage. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_locations-deidentify-templates-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID + Organizations scope, location specified: `organizations/`ORG_ID`/locations/`LOCATION_ID + Organizations scope, no location specified (defaults to global): `organizations/`ORG_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("locations-deidentify-templates-delete",
                    Some(r##"Deletes a DeidentifyTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_locations-deidentify-templates-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the organization and deidentify template to be deleted, for example `organizations/433245324/deidentifyTemplates/432452342` or projects/project-id/deidentifyTemplates/432452342."##),
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
            ("locations-deidentify-templates-get",
                    Some(r##"Gets a DeidentifyTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_locations-deidentify-templates-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the organization and deidentify template to be read, for example `organizations/433245324/deidentifyTemplates/432452342` or projects/project-id/deidentifyTemplates/432452342."##),
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
            ("locations-deidentify-templates-list",
                    Some(r##"Lists DeidentifyTemplates. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_locations-deidentify-templates-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID + Organizations scope, location specified: `organizations/`ORG_ID`/locations/`LOCATION_ID + Organizations scope, no location specified (defaults to global): `organizations/`ORG_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("locations-deidentify-templates-patch",
                    Some(r##"Updates the DeidentifyTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_locations-deidentify-templates-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of organization and deidentify template to be updated, for example `organizations/433245324/deidentifyTemplates/432452342` or projects/project-id/deidentifyTemplates/432452342."##),
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
            ("locations-discovery-configs-create",
                    Some(r##"Creates a config for discovery to scan and profile storage."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_locations-discovery-configs-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization): + Projects scope: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Organizations scope: `organizations/`ORG_ID`/locations/`LOCATION_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("locations-discovery-configs-delete",
                    Some(r##"Deletes a discovery configuration."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_locations-discovery-configs-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the project and the config, for example `projects/dlp-test-project/discoveryConfigs/53234423`."##),
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
            ("locations-discovery-configs-get",
                    Some(r##"Gets a discovery configuration."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_locations-discovery-configs-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the project and the configuration, for example `projects/dlp-test-project/discoveryConfigs/53234423`."##),
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
            ("locations-discovery-configs-list",
                    Some(r##"Lists discovery configurations."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_locations-discovery-configs-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value is as follows: `projects/`PROJECT_ID`/locations/`LOCATION_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("locations-discovery-configs-patch",
                    Some(r##"Updates a discovery configuration."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_locations-discovery-configs-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the project and the configuration, for example `projects/dlp-test-project/discoveryConfigs/53234423`."##),
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
            ("locations-dlp-jobs-list",
                    Some(r##"Lists DlpJobs that match the specified filter in the request. See https://cloud.google.com/sensitive-data-protection/docs/inspecting-storage and https://cloud.google.com/sensitive-data-protection/docs/compute-risk-analysis to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_locations-dlp-jobs-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("locations-inspect-templates-create",
                    Some(r##"Creates an InspectTemplate for reusing frequently used configuration for inspecting content, images, and storage. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_locations-inspect-templates-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID + Organizations scope, location specified: `organizations/`ORG_ID`/locations/`LOCATION_ID + Organizations scope, no location specified (defaults to global): `organizations/`ORG_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("locations-inspect-templates-delete",
                    Some(r##"Deletes an InspectTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_locations-inspect-templates-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the organization and inspectTemplate to be deleted, for example `organizations/433245324/inspectTemplates/432452342` or projects/project-id/inspectTemplates/432452342."##),
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
            ("locations-inspect-templates-get",
                    Some(r##"Gets an InspectTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_locations-inspect-templates-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the organization and inspectTemplate to be read, for example `organizations/433245324/inspectTemplates/432452342` or projects/project-id/inspectTemplates/432452342."##),
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
            ("locations-inspect-templates-list",
                    Some(r##"Lists InspectTemplates. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_locations-inspect-templates-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID + Organizations scope, location specified: `organizations/`ORG_ID`/locations/`LOCATION_ID + Organizations scope, no location specified (defaults to global): `organizations/`ORG_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("locations-inspect-templates-patch",
                    Some(r##"Updates the InspectTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_locations-inspect-templates-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of organization and inspectTemplate to be updated, for example `organizations/433245324/inspectTemplates/432452342` or projects/project-id/inspectTemplates/432452342."##),
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
            ("locations-job-triggers-create",
                    Some(r##"Creates a job trigger to run DLP actions such as scanning storage for sensitive information on a set schedule. See https://cloud.google.com/sensitive-data-protection/docs/creating-job-triggers to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_locations-job-triggers-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("locations-job-triggers-delete",
                    Some(r##"Deletes a job trigger. See https://cloud.google.com/sensitive-data-protection/docs/creating-job-triggers to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_locations-job-triggers-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the project and the triggeredJob, for example `projects/dlp-test-project/jobTriggers/53234423`."##),
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
            ("locations-job-triggers-get",
                    Some(r##"Gets a job trigger. See https://cloud.google.com/sensitive-data-protection/docs/creating-job-triggers to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_locations-job-triggers-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the project and the triggeredJob, for example `projects/dlp-test-project/jobTriggers/53234423`."##),
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
            ("locations-job-triggers-list",
                    Some(r##"Lists job triggers. See https://cloud.google.com/sensitive-data-protection/docs/creating-job-triggers to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_locations-job-triggers-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("locations-job-triggers-patch",
                    Some(r##"Updates a job trigger. See https://cloud.google.com/sensitive-data-protection/docs/creating-job-triggers to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_locations-job-triggers-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the project and the triggeredJob, for example `projects/dlp-test-project/jobTriggers/53234423`."##),
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
            ("locations-project-data-profiles-get",
                    Some(r##"Gets a project data profile."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_locations-project-data-profiles-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name, for example `organizations/12345/locations/us/projectDataProfiles/53234423`."##),
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
            ("locations-project-data-profiles-list",
                    Some(r##"Lists project data profiles for an organization."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_locations-project-data-profiles-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. organizations/{org_id}/locations/{loc_id}"##),
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
            ("locations-stored-info-types-create",
                    Some(r##"Creates a pre-built stored infoType to be used for inspection. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_locations-stored-info-types-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID + Organizations scope, location specified: `organizations/`ORG_ID`/locations/`LOCATION_ID + Organizations scope, no location specified (defaults to global): `organizations/`ORG_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("locations-stored-info-types-delete",
                    Some(r##"Deletes a stored infoType. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_locations-stored-info-types-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the organization and storedInfoType to be deleted, for example `organizations/433245324/storedInfoTypes/432452342` or projects/project-id/storedInfoTypes/432452342."##),
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
            ("locations-stored-info-types-get",
                    Some(r##"Gets a stored infoType. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_locations-stored-info-types-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the organization and storedInfoType to be read, for example `organizations/433245324/storedInfoTypes/432452342` or projects/project-id/storedInfoTypes/432452342."##),
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
            ("locations-stored-info-types-list",
                    Some(r##"Lists stored infoTypes. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_locations-stored-info-types-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("locations-stored-info-types-patch",
                    Some(r##"Updates the stored infoType by creating a new version. The existing version will continue to be used until the new version is ready. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_locations-stored-info-types-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of organization and storedInfoType to be updated, for example `organizations/433245324/storedInfoTypes/432452342` or projects/project-id/storedInfoTypes/432452342."##),
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
            ("locations-table-data-profiles-delete",
                    Some(r##"Delete a TableDataProfile. Will not prevent the profile from being regenerated if the table is still included in a discovery configuration."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_locations-table-data-profiles-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the table data profile."##),
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
            ("locations-table-data-profiles-get",
                    Some(r##"Gets a table data profile."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_locations-table-data-profiles-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name, for example `organizations/12345/locations/us/tableDataProfiles/53234423`."##),
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
            ("locations-table-data-profiles-list",
                    Some(r##"Lists table data profiles for an organization."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_locations-table-data-profiles-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Resource name of the organization or project, for example `organizations/433245324/locations/europe` or `projects/project-id/locations/asia`."##),
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
            ("stored-info-types-create",
                    Some(r##"Creates a pre-built stored infoType to be used for inspection. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_stored-info-types-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID + Organizations scope, location specified: `organizations/`ORG_ID`/locations/`LOCATION_ID + Organizations scope, no location specified (defaults to global): `organizations/`ORG_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("stored-info-types-delete",
                    Some(r##"Deletes a stored infoType. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_stored-info-types-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the organization and storedInfoType to be deleted, for example `organizations/433245324/storedInfoTypes/432452342` or projects/project-id/storedInfoTypes/432452342."##),
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
            ("stored-info-types-get",
                    Some(r##"Gets a stored infoType. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_stored-info-types-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the organization and storedInfoType to be read, for example `organizations/433245324/storedInfoTypes/432452342` or projects/project-id/storedInfoTypes/432452342."##),
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
            ("stored-info-types-list",
                    Some(r##"Lists stored infoTypes. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_stored-info-types-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("stored-info-types-patch",
                    Some(r##"Updates the stored infoType by creating a new version. The existing version will continue to be used until the new version is ready. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_stored-info-types-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of organization and storedInfoType to be updated, for example `organizations/433245324/storedInfoTypes/432452342` or projects/project-id/storedInfoTypes/432452342."##),
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
        
        ("projects", "methods: 'content-deidentify', 'content-inspect', 'content-reidentify', 'deidentify-templates-create', 'deidentify-templates-delete', 'deidentify-templates-get', 'deidentify-templates-list', 'deidentify-templates-patch', 'dlp-jobs-cancel', 'dlp-jobs-create', 'dlp-jobs-delete', 'dlp-jobs-get', 'dlp-jobs-list', 'image-redact', 'inspect-templates-create', 'inspect-templates-delete', 'inspect-templates-get', 'inspect-templates-list', 'inspect-templates-patch', 'job-triggers-activate', 'job-triggers-create', 'job-triggers-delete', 'job-triggers-get', 'job-triggers-list', 'job-triggers-patch', 'locations-column-data-profiles-get', 'locations-column-data-profiles-list', 'locations-connections-create', 'locations-connections-delete', 'locations-connections-get', 'locations-connections-list', 'locations-connections-patch', 'locations-connections-search', 'locations-content-deidentify', 'locations-content-inspect', 'locations-content-reidentify', 'locations-deidentify-templates-create', 'locations-deidentify-templates-delete', 'locations-deidentify-templates-get', 'locations-deidentify-templates-list', 'locations-deidentify-templates-patch', 'locations-discovery-configs-create', 'locations-discovery-configs-delete', 'locations-discovery-configs-get', 'locations-discovery-configs-list', 'locations-discovery-configs-patch', 'locations-dlp-jobs-cancel', 'locations-dlp-jobs-create', 'locations-dlp-jobs-delete', 'locations-dlp-jobs-finish', 'locations-dlp-jobs-get', 'locations-dlp-jobs-hybrid-inspect', 'locations-dlp-jobs-list', 'locations-image-redact', 'locations-inspect-templates-create', 'locations-inspect-templates-delete', 'locations-inspect-templates-get', 'locations-inspect-templates-list', 'locations-inspect-templates-patch', 'locations-job-triggers-activate', 'locations-job-triggers-create', 'locations-job-triggers-delete', 'locations-job-triggers-get', 'locations-job-triggers-hybrid-inspect', 'locations-job-triggers-list', 'locations-job-triggers-patch', 'locations-project-data-profiles-get', 'locations-project-data-profiles-list', 'locations-stored-info-types-create', 'locations-stored-info-types-delete', 'locations-stored-info-types-get', 'locations-stored-info-types-list', 'locations-stored-info-types-patch', 'locations-table-data-profiles-delete', 'locations-table-data-profiles-get', 'locations-table-data-profiles-list', 'stored-info-types-create', 'stored-info-types-delete', 'stored-info-types-get', 'stored-info-types-list' and 'stored-info-types-patch'", vec![
            ("content-deidentify",
                    Some(r##"De-identifies potentially sensitive info from a ContentItem. This method has limits on input size and output size. See https://cloud.google.com/sensitive-data-protection/docs/deidentify-sensitive-data to learn more. When no InfoTypes or CustomInfoTypes are specified in this request, the system will automatically choose what detectors to run. By default this may be all types, but may change over time as detectors are updated."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_content-deidentify",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("content-inspect",
                    Some(r##"Finds potentially sensitive info in content. This method has limits on input size, processing time, and output size. When no InfoTypes or CustomInfoTypes are specified in this request, the system will automatically choose what detectors to run. By default this may be all types, but may change over time as detectors are updated. For how to guides, see https://cloud.google.com/sensitive-data-protection/docs/inspecting-images and https://cloud.google.com/sensitive-data-protection/docs/inspecting-text,"##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_content-inspect",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("content-reidentify",
                    Some(r##"Re-identifies content that has been de-identified. See https://cloud.google.com/sensitive-data-protection/docs/pseudonymization#re-identification_in_free_text_code_example to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_content-reidentify",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("deidentify-templates-create",
                    Some(r##"Creates a DeidentifyTemplate for reusing frequently used configuration for de-identifying content, images, and storage. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_deidentify-templates-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID + Organizations scope, location specified: `organizations/`ORG_ID`/locations/`LOCATION_ID + Organizations scope, no location specified (defaults to global): `organizations/`ORG_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("deidentify-templates-delete",
                    Some(r##"Deletes a DeidentifyTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_deidentify-templates-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the organization and deidentify template to be deleted, for example `organizations/433245324/deidentifyTemplates/432452342` or projects/project-id/deidentifyTemplates/432452342."##),
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
            ("deidentify-templates-get",
                    Some(r##"Gets a DeidentifyTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_deidentify-templates-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the organization and deidentify template to be read, for example `organizations/433245324/deidentifyTemplates/432452342` or projects/project-id/deidentifyTemplates/432452342."##),
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
            ("deidentify-templates-list",
                    Some(r##"Lists DeidentifyTemplates. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_deidentify-templates-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID + Organizations scope, location specified: `organizations/`ORG_ID`/locations/`LOCATION_ID + Organizations scope, no location specified (defaults to global): `organizations/`ORG_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("deidentify-templates-patch",
                    Some(r##"Updates the DeidentifyTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_deidentify-templates-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of organization and deidentify template to be updated, for example `organizations/433245324/deidentifyTemplates/432452342` or projects/project-id/deidentifyTemplates/432452342."##),
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
            ("dlp-jobs-cancel",
                    Some(r##"Starts asynchronous cancellation on a long-running DlpJob. The server makes a best effort to cancel the DlpJob, but success is not guaranteed. See https://cloud.google.com/sensitive-data-protection/docs/inspecting-storage and https://cloud.google.com/sensitive-data-protection/docs/compute-risk-analysis to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_dlp-jobs-cancel",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the DlpJob resource to be cancelled."##),
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
            ("dlp-jobs-create",
                    Some(r##"Creates a new job to inspect storage or calculate risk metrics. See https://cloud.google.com/sensitive-data-protection/docs/inspecting-storage and https://cloud.google.com/sensitive-data-protection/docs/compute-risk-analysis to learn more. When no InfoTypes or CustomInfoTypes are specified in inspect jobs, the system will automatically choose what detectors to run. By default this may be all types, but may change over time as detectors are updated."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_dlp-jobs-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("dlp-jobs-delete",
                    Some(r##"Deletes a long-running DlpJob. This method indicates that the client is no longer interested in the DlpJob result. The job will be canceled if possible. See https://cloud.google.com/sensitive-data-protection/docs/inspecting-storage and https://cloud.google.com/sensitive-data-protection/docs/compute-risk-analysis to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_dlp-jobs-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the DlpJob resource to be deleted."##),
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
            ("dlp-jobs-get",
                    Some(r##"Gets the latest state of a long-running DlpJob. See https://cloud.google.com/sensitive-data-protection/docs/inspecting-storage and https://cloud.google.com/sensitive-data-protection/docs/compute-risk-analysis to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_dlp-jobs-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the DlpJob resource."##),
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
            ("dlp-jobs-list",
                    Some(r##"Lists DlpJobs that match the specified filter in the request. See https://cloud.google.com/sensitive-data-protection/docs/inspecting-storage and https://cloud.google.com/sensitive-data-protection/docs/compute-risk-analysis to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_dlp-jobs-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("image-redact",
                    Some(r##"Redacts potentially sensitive info from an image. This method has limits on input size, processing time, and output size. See https://cloud.google.com/sensitive-data-protection/docs/redacting-sensitive-data-images to learn more. When no InfoTypes or CustomInfoTypes are specified in this request, the system will automatically choose what detectors to run. By default this may be all types, but may change over time as detectors are updated."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_image-redact",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("inspect-templates-create",
                    Some(r##"Creates an InspectTemplate for reusing frequently used configuration for inspecting content, images, and storage. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_inspect-templates-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID + Organizations scope, location specified: `organizations/`ORG_ID`/locations/`LOCATION_ID + Organizations scope, no location specified (defaults to global): `organizations/`ORG_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("inspect-templates-delete",
                    Some(r##"Deletes an InspectTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_inspect-templates-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the organization and inspectTemplate to be deleted, for example `organizations/433245324/inspectTemplates/432452342` or projects/project-id/inspectTemplates/432452342."##),
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
            ("inspect-templates-get",
                    Some(r##"Gets an InspectTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_inspect-templates-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the organization and inspectTemplate to be read, for example `organizations/433245324/inspectTemplates/432452342` or projects/project-id/inspectTemplates/432452342."##),
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
            ("inspect-templates-list",
                    Some(r##"Lists InspectTemplates. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_inspect-templates-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID + Organizations scope, location specified: `organizations/`ORG_ID`/locations/`LOCATION_ID + Organizations scope, no location specified (defaults to global): `organizations/`ORG_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("inspect-templates-patch",
                    Some(r##"Updates the InspectTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_inspect-templates-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of organization and inspectTemplate to be updated, for example `organizations/433245324/inspectTemplates/432452342` or projects/project-id/inspectTemplates/432452342."##),
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
            ("job-triggers-activate",
                    Some(r##"Activate a job trigger. Causes the immediate execute of a trigger instead of waiting on the trigger event to occur."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_job-triggers-activate",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the trigger to activate, for example `projects/dlp-test-project/jobTriggers/53234423`."##),
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
            ("job-triggers-create",
                    Some(r##"Creates a job trigger to run DLP actions such as scanning storage for sensitive information on a set schedule. See https://cloud.google.com/sensitive-data-protection/docs/creating-job-triggers to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_job-triggers-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("job-triggers-delete",
                    Some(r##"Deletes a job trigger. See https://cloud.google.com/sensitive-data-protection/docs/creating-job-triggers to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_job-triggers-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the project and the triggeredJob, for example `projects/dlp-test-project/jobTriggers/53234423`."##),
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
            ("job-triggers-get",
                    Some(r##"Gets a job trigger. See https://cloud.google.com/sensitive-data-protection/docs/creating-job-triggers to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_job-triggers-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the project and the triggeredJob, for example `projects/dlp-test-project/jobTriggers/53234423`."##),
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
            ("job-triggers-list",
                    Some(r##"Lists job triggers. See https://cloud.google.com/sensitive-data-protection/docs/creating-job-triggers to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_job-triggers-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("job-triggers-patch",
                    Some(r##"Updates a job trigger. See https://cloud.google.com/sensitive-data-protection/docs/creating-job-triggers to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_job-triggers-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the project and the triggeredJob, for example `projects/dlp-test-project/jobTriggers/53234423`."##),
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
            ("locations-column-data-profiles-get",
                    Some(r##"Gets a column data profile."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-column-data-profiles-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name, for example `organizations/12345/locations/us/columnDataProfiles/53234423`."##),
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
            ("locations-column-data-profiles-list",
                    Some(r##"Lists column data profiles for an organization."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-column-data-profiles-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Resource name of the organization or project, for example `organizations/433245324/locations/europe` or `projects/project-id/locations/asia`."##),
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
            ("locations-connections-create",
                    Some(r##"Create a Connection to an external data source."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-connections-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization): + Projects scope: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Organizations scope: `organizations/`ORG_ID`/locations/`LOCATION_ID"##),
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
            ("locations-connections-delete",
                    Some(r##"Delete a Connection."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-connections-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the Connection to be deleted, in the format: `projects/{project}/locations/{location}/connections/{connection}`."##),
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
            ("locations-connections-get",
                    Some(r##"Get a Connection by name."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-connections-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name in the format: `projects/{project}/locations/{location}/connections/{connection}`."##),
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
            ("locations-connections-list",
                    Some(r##"Lists Connections in a parent."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-connections-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent name, for example: `projects/project-id/locations/global`."##),
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
            ("locations-connections-patch",
                    Some(r##"Update a Connection."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-connections-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name in the format: `projects/{project}/locations/{location}/connections/{connection}`."##),
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
            ("locations-connections-search",
                    Some(r##"Searches for Connections in a parent."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-connections-search",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent name, typically an organization, without location. For example: `organizations/12345678`."##),
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
            ("locations-content-deidentify",
                    Some(r##"De-identifies potentially sensitive info from a ContentItem. This method has limits on input size and output size. See https://cloud.google.com/sensitive-data-protection/docs/deidentify-sensitive-data to learn more. When no InfoTypes or CustomInfoTypes are specified in this request, the system will automatically choose what detectors to run. By default this may be all types, but may change over time as detectors are updated."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-content-deidentify",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("locations-content-inspect",
                    Some(r##"Finds potentially sensitive info in content. This method has limits on input size, processing time, and output size. When no InfoTypes or CustomInfoTypes are specified in this request, the system will automatically choose what detectors to run. By default this may be all types, but may change over time as detectors are updated. For how to guides, see https://cloud.google.com/sensitive-data-protection/docs/inspecting-images and https://cloud.google.com/sensitive-data-protection/docs/inspecting-text,"##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-content-inspect",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("locations-content-reidentify",
                    Some(r##"Re-identifies content that has been de-identified. See https://cloud.google.com/sensitive-data-protection/docs/pseudonymization#re-identification_in_free_text_code_example to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-content-reidentify",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("locations-deidentify-templates-create",
                    Some(r##"Creates a DeidentifyTemplate for reusing frequently used configuration for de-identifying content, images, and storage. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-deidentify-templates-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID + Organizations scope, location specified: `organizations/`ORG_ID`/locations/`LOCATION_ID + Organizations scope, no location specified (defaults to global): `organizations/`ORG_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("locations-deidentify-templates-delete",
                    Some(r##"Deletes a DeidentifyTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-deidentify-templates-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the organization and deidentify template to be deleted, for example `organizations/433245324/deidentifyTemplates/432452342` or projects/project-id/deidentifyTemplates/432452342."##),
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
            ("locations-deidentify-templates-get",
                    Some(r##"Gets a DeidentifyTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-deidentify-templates-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the organization and deidentify template to be read, for example `organizations/433245324/deidentifyTemplates/432452342` or projects/project-id/deidentifyTemplates/432452342."##),
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
            ("locations-deidentify-templates-list",
                    Some(r##"Lists DeidentifyTemplates. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-deidentify-templates-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID + Organizations scope, location specified: `organizations/`ORG_ID`/locations/`LOCATION_ID + Organizations scope, no location specified (defaults to global): `organizations/`ORG_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("locations-deidentify-templates-patch",
                    Some(r##"Updates the DeidentifyTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-deidentify-templates-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of organization and deidentify template to be updated, for example `organizations/433245324/deidentifyTemplates/432452342` or projects/project-id/deidentifyTemplates/432452342."##),
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
            ("locations-discovery-configs-create",
                    Some(r##"Creates a config for discovery to scan and profile storage."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-discovery-configs-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization): + Projects scope: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Organizations scope: `organizations/`ORG_ID`/locations/`LOCATION_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("locations-discovery-configs-delete",
                    Some(r##"Deletes a discovery configuration."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-discovery-configs-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the project and the config, for example `projects/dlp-test-project/discoveryConfigs/53234423`."##),
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
            ("locations-discovery-configs-get",
                    Some(r##"Gets a discovery configuration."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-discovery-configs-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the project and the configuration, for example `projects/dlp-test-project/discoveryConfigs/53234423`."##),
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
            ("locations-discovery-configs-list",
                    Some(r##"Lists discovery configurations."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-discovery-configs-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value is as follows: `projects/`PROJECT_ID`/locations/`LOCATION_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("locations-discovery-configs-patch",
                    Some(r##"Updates a discovery configuration."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-discovery-configs-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the project and the configuration, for example `projects/dlp-test-project/discoveryConfigs/53234423`."##),
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
            ("locations-dlp-jobs-cancel",
                    Some(r##"Starts asynchronous cancellation on a long-running DlpJob. The server makes a best effort to cancel the DlpJob, but success is not guaranteed. See https://cloud.google.com/sensitive-data-protection/docs/inspecting-storage and https://cloud.google.com/sensitive-data-protection/docs/compute-risk-analysis to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-dlp-jobs-cancel",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the DlpJob resource to be cancelled."##),
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
            ("locations-dlp-jobs-create",
                    Some(r##"Creates a new job to inspect storage or calculate risk metrics. See https://cloud.google.com/sensitive-data-protection/docs/inspecting-storage and https://cloud.google.com/sensitive-data-protection/docs/compute-risk-analysis to learn more. When no InfoTypes or CustomInfoTypes are specified in inspect jobs, the system will automatically choose what detectors to run. By default this may be all types, but may change over time as detectors are updated."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-dlp-jobs-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("locations-dlp-jobs-delete",
                    Some(r##"Deletes a long-running DlpJob. This method indicates that the client is no longer interested in the DlpJob result. The job will be canceled if possible. See https://cloud.google.com/sensitive-data-protection/docs/inspecting-storage and https://cloud.google.com/sensitive-data-protection/docs/compute-risk-analysis to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-dlp-jobs-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the DlpJob resource to be deleted."##),
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
            ("locations-dlp-jobs-finish",
                    Some(r##"Finish a running hybrid DlpJob. Triggers the finalization steps and running of any enabled actions that have not yet run."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-dlp-jobs-finish",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the DlpJob resource to be finished."##),
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
            ("locations-dlp-jobs-get",
                    Some(r##"Gets the latest state of a long-running DlpJob. See https://cloud.google.com/sensitive-data-protection/docs/inspecting-storage and https://cloud.google.com/sensitive-data-protection/docs/compute-risk-analysis to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-dlp-jobs-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the DlpJob resource."##),
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
            ("locations-dlp-jobs-hybrid-inspect",
                    Some(r##"Inspect hybrid content and store findings to a job. To review the findings, inspect the job. Inspection will occur asynchronously."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-dlp-jobs-hybrid-inspect",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the job to execute a hybrid inspect on, for example `projects/dlp-test-project/dlpJob/53234423`."##),
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
            ("locations-dlp-jobs-list",
                    Some(r##"Lists DlpJobs that match the specified filter in the request. See https://cloud.google.com/sensitive-data-protection/docs/inspecting-storage and https://cloud.google.com/sensitive-data-protection/docs/compute-risk-analysis to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-dlp-jobs-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("locations-image-redact",
                    Some(r##"Redacts potentially sensitive info from an image. This method has limits on input size, processing time, and output size. See https://cloud.google.com/sensitive-data-protection/docs/redacting-sensitive-data-images to learn more. When no InfoTypes or CustomInfoTypes are specified in this request, the system will automatically choose what detectors to run. By default this may be all types, but may change over time as detectors are updated."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-image-redact",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("locations-inspect-templates-create",
                    Some(r##"Creates an InspectTemplate for reusing frequently used configuration for inspecting content, images, and storage. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-inspect-templates-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID + Organizations scope, location specified: `organizations/`ORG_ID`/locations/`LOCATION_ID + Organizations scope, no location specified (defaults to global): `organizations/`ORG_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("locations-inspect-templates-delete",
                    Some(r##"Deletes an InspectTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-inspect-templates-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the organization and inspectTemplate to be deleted, for example `organizations/433245324/inspectTemplates/432452342` or projects/project-id/inspectTemplates/432452342."##),
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
            ("locations-inspect-templates-get",
                    Some(r##"Gets an InspectTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-inspect-templates-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the organization and inspectTemplate to be read, for example `organizations/433245324/inspectTemplates/432452342` or projects/project-id/inspectTemplates/432452342."##),
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
            ("locations-inspect-templates-list",
                    Some(r##"Lists InspectTemplates. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-inspect-templates-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID + Organizations scope, location specified: `organizations/`ORG_ID`/locations/`LOCATION_ID + Organizations scope, no location specified (defaults to global): `organizations/`ORG_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("locations-inspect-templates-patch",
                    Some(r##"Updates the InspectTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-inspect-templates-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of organization and inspectTemplate to be updated, for example `organizations/433245324/inspectTemplates/432452342` or projects/project-id/inspectTemplates/432452342."##),
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
            ("locations-job-triggers-activate",
                    Some(r##"Activate a job trigger. Causes the immediate execute of a trigger instead of waiting on the trigger event to occur."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-job-triggers-activate",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the trigger to activate, for example `projects/dlp-test-project/jobTriggers/53234423`."##),
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
            ("locations-job-triggers-create",
                    Some(r##"Creates a job trigger to run DLP actions such as scanning storage for sensitive information on a set schedule. See https://cloud.google.com/sensitive-data-protection/docs/creating-job-triggers to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-job-triggers-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("locations-job-triggers-delete",
                    Some(r##"Deletes a job trigger. See https://cloud.google.com/sensitive-data-protection/docs/creating-job-triggers to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-job-triggers-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the project and the triggeredJob, for example `projects/dlp-test-project/jobTriggers/53234423`."##),
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
            ("locations-job-triggers-get",
                    Some(r##"Gets a job trigger. See https://cloud.google.com/sensitive-data-protection/docs/creating-job-triggers to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-job-triggers-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the project and the triggeredJob, for example `projects/dlp-test-project/jobTriggers/53234423`."##),
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
            ("locations-job-triggers-hybrid-inspect",
                    Some(r##"Inspect hybrid content and store findings to a trigger. The inspection will be processed asynchronously. To review the findings monitor the jobs within the trigger."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-job-triggers-hybrid-inspect",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the trigger to execute a hybrid inspect on, for example `projects/dlp-test-project/jobTriggers/53234423`."##),
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
            ("locations-job-triggers-list",
                    Some(r##"Lists job triggers. See https://cloud.google.com/sensitive-data-protection/docs/creating-job-triggers to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-job-triggers-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("locations-job-triggers-patch",
                    Some(r##"Updates a job trigger. See https://cloud.google.com/sensitive-data-protection/docs/creating-job-triggers to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-job-triggers-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the project and the triggeredJob, for example `projects/dlp-test-project/jobTriggers/53234423`."##),
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
            ("locations-project-data-profiles-get",
                    Some(r##"Gets a project data profile."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-project-data-profiles-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name, for example `organizations/12345/locations/us/projectDataProfiles/53234423`."##),
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
            ("locations-project-data-profiles-list",
                    Some(r##"Lists project data profiles for an organization."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-project-data-profiles-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. organizations/{org_id}/locations/{loc_id}"##),
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
            ("locations-stored-info-types-create",
                    Some(r##"Creates a pre-built stored infoType to be used for inspection. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-stored-info-types-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID + Organizations scope, location specified: `organizations/`ORG_ID`/locations/`LOCATION_ID + Organizations scope, no location specified (defaults to global): `organizations/`ORG_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("locations-stored-info-types-delete",
                    Some(r##"Deletes a stored infoType. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-stored-info-types-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the organization and storedInfoType to be deleted, for example `organizations/433245324/storedInfoTypes/432452342` or projects/project-id/storedInfoTypes/432452342."##),
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
            ("locations-stored-info-types-get",
                    Some(r##"Gets a stored infoType. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-stored-info-types-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the organization and storedInfoType to be read, for example `organizations/433245324/storedInfoTypes/432452342` or projects/project-id/storedInfoTypes/432452342."##),
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
            ("locations-stored-info-types-list",
                    Some(r##"Lists stored infoTypes. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-stored-info-types-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("locations-stored-info-types-patch",
                    Some(r##"Updates the stored infoType by creating a new version. The existing version will continue to be used until the new version is ready. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-stored-info-types-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of organization and storedInfoType to be updated, for example `organizations/433245324/storedInfoTypes/432452342` or projects/project-id/storedInfoTypes/432452342."##),
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
            ("locations-table-data-profiles-delete",
                    Some(r##"Delete a TableDataProfile. Will not prevent the profile from being regenerated if the table is still included in a discovery configuration."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-table-data-profiles-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the table data profile."##),
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
            ("locations-table-data-profiles-get",
                    Some(r##"Gets a table data profile."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-table-data-profiles-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name, for example `organizations/12345/locations/us/tableDataProfiles/53234423`."##),
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
            ("locations-table-data-profiles-list",
                    Some(r##"Lists table data profiles for an organization."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_locations-table-data-profiles-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Resource name of the organization or project, for example `organizations/433245324/locations/europe` or `projects/project-id/locations/asia`."##),
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
            ("stored-info-types-create",
                    Some(r##"Creates a pre-built stored infoType to be used for inspection. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_stored-info-types-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID + Organizations scope, location specified: `organizations/`ORG_ID`/locations/`LOCATION_ID + Organizations scope, no location specified (defaults to global): `organizations/`ORG_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("stored-info-types-delete",
                    Some(r##"Deletes a stored infoType. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_stored-info-types-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the organization and storedInfoType to be deleted, for example `organizations/433245324/storedInfoTypes/432452342` or projects/project-id/storedInfoTypes/432452342."##),
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
            ("stored-info-types-get",
                    Some(r##"Gets a stored infoType. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_stored-info-types-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of the organization and storedInfoType to be read, for example `organizations/433245324/storedInfoTypes/432452342` or projects/project-id/storedInfoTypes/432452342."##),
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
            ("stored-info-types-list",
                    Some(r##"Lists stored infoTypes. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_stored-info-types-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/` LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3"##),
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
            ("stored-info-types-patch",
                    Some(r##"Updates the stored infoType by creating a new version. The existing version will continue to be used until the new version is ready. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_stored-info-types-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of organization and storedInfoType to be updated, for example `organizations/433245324/storedInfoTypes/432452342` or projects/project-id/storedInfoTypes/432452342."##),
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
    
    let mut app = App::new("dlp2")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+20240616")
           .about("Discover and protect your sensitive data. A fully managed service designed to help you discover, classify, and protect your valuable data assets with ease.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_dlp2_cli")
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
