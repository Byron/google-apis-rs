// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_analyticsdata1_beta::{api, Error, oauth2, client::chrono, FieldMask};


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
    hub: api::AnalyticsData<S>,
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
    async fn _properties_audience_exports_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "audience" => Some(("audience", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "audience-display-name" => Some(("audienceDisplayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "begin-creating-time" => Some(("beginCreatingTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creation-quota-tokens-charged" => Some(("creationQuotaTokensCharged", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "error-message" => Some(("errorMessage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "percentage-completed" => Some(("percentageCompleted", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "row-count" => Some(("rowCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["audience", "audience-display-name", "begin-creating-time", "creation-quota-tokens-charged", "error-message", "name", "percentage-completed", "row-count", "state"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::AudienceExport = json::value::from_value(object).unwrap();
        let mut call = self.hub.properties().audience_exports_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _properties_audience_exports_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.properties().audience_exports_get(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _properties_audience_exports_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.properties().audience_exports_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _properties_audience_exports_query(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "limit" => Some(("limit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "offset" => Some(("offset", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["limit", "offset"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::QueryAudienceExportRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.properties().audience_exports_query(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _properties_batch_run_pivot_reports(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::BatchRunPivotReportsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.properties().batch_run_pivot_reports(request, opt.value_of("property").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _properties_batch_run_reports(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::BatchRunReportsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.properties().batch_run_reports(request, opt.value_of("property").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _properties_check_compatibility(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "compatibility-filter" => Some(("compatibilityFilter", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.between-filter.from-value.double-value" => Some(("dimensionFilter.filter.betweenFilter.fromValue.doubleValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.between-filter.from-value.int64-value" => Some(("dimensionFilter.filter.betweenFilter.fromValue.int64Value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.between-filter.to-value.double-value" => Some(("dimensionFilter.filter.betweenFilter.toValue.doubleValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.between-filter.to-value.int64-value" => Some(("dimensionFilter.filter.betweenFilter.toValue.int64Value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.field-name" => Some(("dimensionFilter.filter.fieldName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.in-list-filter.case-sensitive" => Some(("dimensionFilter.filter.inListFilter.caseSensitive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.in-list-filter.values" => Some(("dimensionFilter.filter.inListFilter.values", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "dimension-filter.filter.numeric-filter.operation" => Some(("dimensionFilter.filter.numericFilter.operation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.numeric-filter.value.double-value" => Some(("dimensionFilter.filter.numericFilter.value.doubleValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.numeric-filter.value.int64-value" => Some(("dimensionFilter.filter.numericFilter.value.int64Value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.string-filter.case-sensitive" => Some(("dimensionFilter.filter.stringFilter.caseSensitive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.string-filter.match-type" => Some(("dimensionFilter.filter.stringFilter.matchType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.string-filter.value" => Some(("dimensionFilter.filter.stringFilter.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metric-filter.filter.between-filter.from-value.double-value" => Some(("metricFilter.filter.betweenFilter.fromValue.doubleValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "metric-filter.filter.between-filter.from-value.int64-value" => Some(("metricFilter.filter.betweenFilter.fromValue.int64Value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metric-filter.filter.between-filter.to-value.double-value" => Some(("metricFilter.filter.betweenFilter.toValue.doubleValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "metric-filter.filter.between-filter.to-value.int64-value" => Some(("metricFilter.filter.betweenFilter.toValue.int64Value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metric-filter.filter.field-name" => Some(("metricFilter.filter.fieldName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metric-filter.filter.in-list-filter.case-sensitive" => Some(("metricFilter.filter.inListFilter.caseSensitive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "metric-filter.filter.in-list-filter.values" => Some(("metricFilter.filter.inListFilter.values", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "metric-filter.filter.numeric-filter.operation" => Some(("metricFilter.filter.numericFilter.operation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metric-filter.filter.numeric-filter.value.double-value" => Some(("metricFilter.filter.numericFilter.value.doubleValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "metric-filter.filter.numeric-filter.value.int64-value" => Some(("metricFilter.filter.numericFilter.value.int64Value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metric-filter.filter.string-filter.case-sensitive" => Some(("metricFilter.filter.stringFilter.caseSensitive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "metric-filter.filter.string-filter.match-type" => Some(("metricFilter.filter.stringFilter.matchType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metric-filter.filter.string-filter.value" => Some(("metricFilter.filter.stringFilter.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["between-filter", "case-sensitive", "compatibility-filter", "dimension-filter", "double-value", "field-name", "filter", "from-value", "in-list-filter", "int64-value", "match-type", "metric-filter", "numeric-filter", "operation", "string-filter", "to-value", "value", "values"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CheckCompatibilityRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.properties().check_compatibility(request, opt.value_of("property").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _properties_get_metadata(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.properties().get_metadata(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _properties_run_pivot_report(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cohort-spec.cohort-report-settings.accumulate" => Some(("cohortSpec.cohortReportSettings.accumulate", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cohort-spec.cohorts-range.end-offset" => Some(("cohortSpec.cohortsRange.endOffset", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cohort-spec.cohorts-range.granularity" => Some(("cohortSpec.cohortsRange.granularity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cohort-spec.cohorts-range.start-offset" => Some(("cohortSpec.cohortsRange.startOffset", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "currency-code" => Some(("currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.between-filter.from-value.double-value" => Some(("dimensionFilter.filter.betweenFilter.fromValue.doubleValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.between-filter.from-value.int64-value" => Some(("dimensionFilter.filter.betweenFilter.fromValue.int64Value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.between-filter.to-value.double-value" => Some(("dimensionFilter.filter.betweenFilter.toValue.doubleValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.between-filter.to-value.int64-value" => Some(("dimensionFilter.filter.betweenFilter.toValue.int64Value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.field-name" => Some(("dimensionFilter.filter.fieldName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.in-list-filter.case-sensitive" => Some(("dimensionFilter.filter.inListFilter.caseSensitive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.in-list-filter.values" => Some(("dimensionFilter.filter.inListFilter.values", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "dimension-filter.filter.numeric-filter.operation" => Some(("dimensionFilter.filter.numericFilter.operation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.numeric-filter.value.double-value" => Some(("dimensionFilter.filter.numericFilter.value.doubleValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.numeric-filter.value.int64-value" => Some(("dimensionFilter.filter.numericFilter.value.int64Value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.string-filter.case-sensitive" => Some(("dimensionFilter.filter.stringFilter.caseSensitive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.string-filter.match-type" => Some(("dimensionFilter.filter.stringFilter.matchType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.string-filter.value" => Some(("dimensionFilter.filter.stringFilter.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "keep-empty-rows" => Some(("keepEmptyRows", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "metric-filter.filter.between-filter.from-value.double-value" => Some(("metricFilter.filter.betweenFilter.fromValue.doubleValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "metric-filter.filter.between-filter.from-value.int64-value" => Some(("metricFilter.filter.betweenFilter.fromValue.int64Value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metric-filter.filter.between-filter.to-value.double-value" => Some(("metricFilter.filter.betweenFilter.toValue.doubleValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "metric-filter.filter.between-filter.to-value.int64-value" => Some(("metricFilter.filter.betweenFilter.toValue.int64Value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metric-filter.filter.field-name" => Some(("metricFilter.filter.fieldName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metric-filter.filter.in-list-filter.case-sensitive" => Some(("metricFilter.filter.inListFilter.caseSensitive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "metric-filter.filter.in-list-filter.values" => Some(("metricFilter.filter.inListFilter.values", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "metric-filter.filter.numeric-filter.operation" => Some(("metricFilter.filter.numericFilter.operation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metric-filter.filter.numeric-filter.value.double-value" => Some(("metricFilter.filter.numericFilter.value.doubleValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "metric-filter.filter.numeric-filter.value.int64-value" => Some(("metricFilter.filter.numericFilter.value.int64Value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metric-filter.filter.string-filter.case-sensitive" => Some(("metricFilter.filter.stringFilter.caseSensitive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "metric-filter.filter.string-filter.match-type" => Some(("metricFilter.filter.stringFilter.matchType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metric-filter.filter.string-filter.value" => Some(("metricFilter.filter.stringFilter.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "property" => Some(("property", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "return-property-quota" => Some(("returnPropertyQuota", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["accumulate", "between-filter", "case-sensitive", "cohort-report-settings", "cohort-spec", "cohorts-range", "currency-code", "dimension-filter", "double-value", "end-offset", "field-name", "filter", "from-value", "granularity", "in-list-filter", "int64-value", "keep-empty-rows", "match-type", "metric-filter", "numeric-filter", "operation", "property", "return-property-quota", "start-offset", "string-filter", "to-value", "value", "values"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::RunPivotReportRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.properties().run_pivot_report(request, opt.value_of("property").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _properties_run_realtime_report(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "dimension-filter.filter.between-filter.from-value.double-value" => Some(("dimensionFilter.filter.betweenFilter.fromValue.doubleValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.between-filter.from-value.int64-value" => Some(("dimensionFilter.filter.betweenFilter.fromValue.int64Value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.between-filter.to-value.double-value" => Some(("dimensionFilter.filter.betweenFilter.toValue.doubleValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.between-filter.to-value.int64-value" => Some(("dimensionFilter.filter.betweenFilter.toValue.int64Value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.field-name" => Some(("dimensionFilter.filter.fieldName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.in-list-filter.case-sensitive" => Some(("dimensionFilter.filter.inListFilter.caseSensitive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.in-list-filter.values" => Some(("dimensionFilter.filter.inListFilter.values", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "dimension-filter.filter.numeric-filter.operation" => Some(("dimensionFilter.filter.numericFilter.operation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.numeric-filter.value.double-value" => Some(("dimensionFilter.filter.numericFilter.value.doubleValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.numeric-filter.value.int64-value" => Some(("dimensionFilter.filter.numericFilter.value.int64Value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.string-filter.case-sensitive" => Some(("dimensionFilter.filter.stringFilter.caseSensitive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.string-filter.match-type" => Some(("dimensionFilter.filter.stringFilter.matchType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.string-filter.value" => Some(("dimensionFilter.filter.stringFilter.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "limit" => Some(("limit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metric-aggregations" => Some(("metricAggregations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "metric-filter.filter.between-filter.from-value.double-value" => Some(("metricFilter.filter.betweenFilter.fromValue.doubleValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "metric-filter.filter.between-filter.from-value.int64-value" => Some(("metricFilter.filter.betweenFilter.fromValue.int64Value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metric-filter.filter.between-filter.to-value.double-value" => Some(("metricFilter.filter.betweenFilter.toValue.doubleValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "metric-filter.filter.between-filter.to-value.int64-value" => Some(("metricFilter.filter.betweenFilter.toValue.int64Value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metric-filter.filter.field-name" => Some(("metricFilter.filter.fieldName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metric-filter.filter.in-list-filter.case-sensitive" => Some(("metricFilter.filter.inListFilter.caseSensitive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "metric-filter.filter.in-list-filter.values" => Some(("metricFilter.filter.inListFilter.values", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "metric-filter.filter.numeric-filter.operation" => Some(("metricFilter.filter.numericFilter.operation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metric-filter.filter.numeric-filter.value.double-value" => Some(("metricFilter.filter.numericFilter.value.doubleValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "metric-filter.filter.numeric-filter.value.int64-value" => Some(("metricFilter.filter.numericFilter.value.int64Value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metric-filter.filter.string-filter.case-sensitive" => Some(("metricFilter.filter.stringFilter.caseSensitive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "metric-filter.filter.string-filter.match-type" => Some(("metricFilter.filter.stringFilter.matchType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metric-filter.filter.string-filter.value" => Some(("metricFilter.filter.stringFilter.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "return-property-quota" => Some(("returnPropertyQuota", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["between-filter", "case-sensitive", "dimension-filter", "double-value", "field-name", "filter", "from-value", "in-list-filter", "int64-value", "limit", "match-type", "metric-aggregations", "metric-filter", "numeric-filter", "operation", "return-property-quota", "string-filter", "to-value", "value", "values"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::RunRealtimeReportRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.properties().run_realtime_report(request, opt.value_of("property").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _properties_run_report(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cohort-spec.cohort-report-settings.accumulate" => Some(("cohortSpec.cohortReportSettings.accumulate", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cohort-spec.cohorts-range.end-offset" => Some(("cohortSpec.cohortsRange.endOffset", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cohort-spec.cohorts-range.granularity" => Some(("cohortSpec.cohortsRange.granularity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cohort-spec.cohorts-range.start-offset" => Some(("cohortSpec.cohortsRange.startOffset", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "currency-code" => Some(("currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.between-filter.from-value.double-value" => Some(("dimensionFilter.filter.betweenFilter.fromValue.doubleValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.between-filter.from-value.int64-value" => Some(("dimensionFilter.filter.betweenFilter.fromValue.int64Value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.between-filter.to-value.double-value" => Some(("dimensionFilter.filter.betweenFilter.toValue.doubleValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.between-filter.to-value.int64-value" => Some(("dimensionFilter.filter.betweenFilter.toValue.int64Value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.field-name" => Some(("dimensionFilter.filter.fieldName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.in-list-filter.case-sensitive" => Some(("dimensionFilter.filter.inListFilter.caseSensitive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.in-list-filter.values" => Some(("dimensionFilter.filter.inListFilter.values", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "dimension-filter.filter.numeric-filter.operation" => Some(("dimensionFilter.filter.numericFilter.operation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.numeric-filter.value.double-value" => Some(("dimensionFilter.filter.numericFilter.value.doubleValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.numeric-filter.value.int64-value" => Some(("dimensionFilter.filter.numericFilter.value.int64Value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.string-filter.case-sensitive" => Some(("dimensionFilter.filter.stringFilter.caseSensitive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.string-filter.match-type" => Some(("dimensionFilter.filter.stringFilter.matchType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dimension-filter.filter.string-filter.value" => Some(("dimensionFilter.filter.stringFilter.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "keep-empty-rows" => Some(("keepEmptyRows", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "limit" => Some(("limit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metric-aggregations" => Some(("metricAggregations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "metric-filter.filter.between-filter.from-value.double-value" => Some(("metricFilter.filter.betweenFilter.fromValue.doubleValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "metric-filter.filter.between-filter.from-value.int64-value" => Some(("metricFilter.filter.betweenFilter.fromValue.int64Value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metric-filter.filter.between-filter.to-value.double-value" => Some(("metricFilter.filter.betweenFilter.toValue.doubleValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "metric-filter.filter.between-filter.to-value.int64-value" => Some(("metricFilter.filter.betweenFilter.toValue.int64Value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metric-filter.filter.field-name" => Some(("metricFilter.filter.fieldName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metric-filter.filter.in-list-filter.case-sensitive" => Some(("metricFilter.filter.inListFilter.caseSensitive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "metric-filter.filter.in-list-filter.values" => Some(("metricFilter.filter.inListFilter.values", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "metric-filter.filter.numeric-filter.operation" => Some(("metricFilter.filter.numericFilter.operation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metric-filter.filter.numeric-filter.value.double-value" => Some(("metricFilter.filter.numericFilter.value.doubleValue", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "metric-filter.filter.numeric-filter.value.int64-value" => Some(("metricFilter.filter.numericFilter.value.int64Value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metric-filter.filter.string-filter.case-sensitive" => Some(("metricFilter.filter.stringFilter.caseSensitive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "metric-filter.filter.string-filter.match-type" => Some(("metricFilter.filter.stringFilter.matchType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metric-filter.filter.string-filter.value" => Some(("metricFilter.filter.stringFilter.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "offset" => Some(("offset", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "property" => Some(("property", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "return-property-quota" => Some(("returnPropertyQuota", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["accumulate", "between-filter", "case-sensitive", "cohort-report-settings", "cohort-spec", "cohorts-range", "currency-code", "dimension-filter", "double-value", "end-offset", "field-name", "filter", "from-value", "granularity", "in-list-filter", "int64-value", "keep-empty-rows", "limit", "match-type", "metric-aggregations", "metric-filter", "numeric-filter", "offset", "operation", "property", "return-property-quota", "start-offset", "string-filter", "to-value", "value", "values"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::RunReportRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.properties().run_report(request, opt.value_of("property").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
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
            ("properties", Some(opt)) => {
                match opt.subcommand() {
                    ("audience-exports-create", Some(opt)) => {
                        call_result = self._properties_audience_exports_create(opt, dry_run, &mut err).await;
                    },
                    ("audience-exports-get", Some(opt)) => {
                        call_result = self._properties_audience_exports_get(opt, dry_run, &mut err).await;
                    },
                    ("audience-exports-list", Some(opt)) => {
                        call_result = self._properties_audience_exports_list(opt, dry_run, &mut err).await;
                    },
                    ("audience-exports-query", Some(opt)) => {
                        call_result = self._properties_audience_exports_query(opt, dry_run, &mut err).await;
                    },
                    ("batch-run-pivot-reports", Some(opt)) => {
                        call_result = self._properties_batch_run_pivot_reports(opt, dry_run, &mut err).await;
                    },
                    ("batch-run-reports", Some(opt)) => {
                        call_result = self._properties_batch_run_reports(opt, dry_run, &mut err).await;
                    },
                    ("check-compatibility", Some(opt)) => {
                        call_result = self._properties_check_compatibility(opt, dry_run, &mut err).await;
                    },
                    ("get-metadata", Some(opt)) => {
                        call_result = self._properties_get_metadata(opt, dry_run, &mut err).await;
                    },
                    ("run-pivot-report", Some(opt)) => {
                        call_result = self._properties_run_pivot_report(opt, dry_run, &mut err).await;
                    },
                    ("run-realtime-report", Some(opt)) => {
                        call_result = self._properties_run_realtime_report(opt, dry_run, &mut err).await;
                    },
                    ("run-report", Some(opt)) => {
                        call_result = self._properties_run_report(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("properties".to_string()));
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

            match client::application_secret_from_directory(&config_dir, "analyticsdata1-beta-secret.json",
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
        ).persist_tokens_to_disk(format!("{}/analyticsdata1-beta", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::AnalyticsData::new(client, auth),
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
        ("properties", "methods: 'audience-exports-create', 'audience-exports-get', 'audience-exports-list', 'audience-exports-query', 'batch-run-pivot-reports', 'batch-run-reports', 'check-compatibility', 'get-metadata', 'run-pivot-report', 'run-realtime-report' and 'run-report'", vec![
            ("audience-exports-create",
                    Some(r##"Creates an audience export for later retrieval. This method quickly returns the audience export's resource name and initiates a long running asynchronous request to form an audience export. To export the users in an audience export, first create the audience export through this method and then send the audience resource name to the `QueryAudienceExport` method. See [Creating an Audience Export](https://developers.google.com/analytics/devguides/reporting/data/v1/audience-list-basics) for an introduction to Audience Exports with examples. An audience export is a snapshot of the users currently in the audience at the time of audience export creation. Creating audience exports for one audience on different days will return different results as users enter and exit the audience. Audiences in Google Analytics 4 allow you to segment your users in the ways that are important to your business. To learn more, see https://support.google.com/analytics/answer/9267572. Audience exports contain the users in each audience. Audience Export APIs have some methods at alpha and other methods at beta stability. The intention is to advance methods to beta stability after some feedback and adoption. To give your feedback on this API, complete the [Google Analytics Audience Export API Feedback](https://forms.gle/EeA5u5LW6PEggtCEA) form."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analyticsdata1_beta_cli/properties_audience-exports-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource where this audience export will be created. Format: `properties/{property}`"##),
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
            ("audience-exports-get",
                    Some(r##"Gets configuration metadata about a specific audience export. This method can be used to understand an audience export after it has been created. See [Creating an Audience Export](https://developers.google.com/analytics/devguides/reporting/data/v1/audience-list-basics) for an introduction to Audience Exports with examples. Audience Export APIs have some methods at alpha and other methods at beta stability. The intention is to advance methods to beta stability after some feedback and adoption. To give your feedback on this API, complete the [Google Analytics Audience Export API Feedback](https://forms.gle/EeA5u5LW6PEggtCEA) form."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analyticsdata1_beta_cli/properties_audience-exports-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The audience export resource name. Format: `properties/{property}/audienceExports/{audience_export}`"##),
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
            ("audience-exports-list",
                    Some(r##"Lists all audience exports for a property. This method can be used for you to find and reuse existing audience exports rather than creating unnecessary new audience exports. The same audience can have multiple audience exports that represent the export of users that were in an audience on different days. See [Creating an Audience Export](https://developers.google.com/analytics/devguides/reporting/data/v1/audience-list-basics) for an introduction to Audience Exports with examples. Audience Export APIs have some methods at alpha and other methods at beta stability. The intention is to advance methods to beta stability after some feedback and adoption. To give your feedback on this API, complete the [Google Analytics Audience Export API Feedback](https://forms.gle/EeA5u5LW6PEggtCEA) form."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analyticsdata1_beta_cli/properties_audience-exports-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. All audience exports for this property will be listed in the response. Format: `properties/{property}`"##),
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
            ("audience-exports-query",
                    Some(r##"Retrieves an audience export of users. After creating an audience, the users are not immediately available for exporting. First, a request to `CreateAudienceExport` is necessary to create an audience export of users, and then second, this method is used to retrieve the users in the audience export. See [Creating an Audience Export](https://developers.google.com/analytics/devguides/reporting/data/v1/audience-list-basics) for an introduction to Audience Exports with examples. Audiences in Google Analytics 4 allow you to segment your users in the ways that are important to your business. To learn more, see https://support.google.com/analytics/answer/9267572. Audience Export APIs have some methods at alpha and other methods at beta stability. The intention is to advance methods to beta stability after some feedback and adoption. To give your feedback on this API, complete the [Google Analytics Audience Export API Feedback](https://forms.gle/EeA5u5LW6PEggtCEA) form."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analyticsdata1_beta_cli/properties_audience-exports-query",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the audience export to retrieve users from. Format: `properties/{property}/audienceExports/{audience_export}`"##),
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
            ("batch-run-pivot-reports",
                    Some(r##"Returns multiple pivot reports in a batch. All reports must be for the same GA4 Property."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analyticsdata1_beta_cli/properties_batch-run-pivot-reports",
                  vec![
                    (Some(r##"property"##),
                     None,
                     Some(r##"A Google Analytics GA4 property identifier whose events are tracked. Specified in the URL path and not the body. To learn more, see [where to find your Property ID](https://developers.google.com/analytics/devguides/reporting/data/v1/property-id). This property must be specified for the batch. The property within RunPivotReportRequest may either be unspecified or consistent with this property. Example: properties/1234"##),
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
            ("batch-run-reports",
                    Some(r##"Returns multiple reports in a batch. All reports must be for the same GA4 Property."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analyticsdata1_beta_cli/properties_batch-run-reports",
                  vec![
                    (Some(r##"property"##),
                     None,
                     Some(r##"A Google Analytics GA4 property identifier whose events are tracked. Specified in the URL path and not the body. To learn more, see [where to find your Property ID](https://developers.google.com/analytics/devguides/reporting/data/v1/property-id). This property must be specified for the batch. The property within RunReportRequest may either be unspecified or consistent with this property. Example: properties/1234"##),
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
            ("check-compatibility",
                    Some(r##"This compatibility method lists dimensions and metrics that can be added to a report request and maintain compatibility. This method fails if the request's dimensions and metrics are incompatible. In Google Analytics, reports fail if they request incompatible dimensions and/or metrics; in that case, you will need to remove dimensions and/or metrics from the incompatible report until the report is compatible. The Realtime and Core reports have different compatibility rules. This method checks compatibility for Core reports."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analyticsdata1_beta_cli/properties_check-compatibility",
                  vec![
                    (Some(r##"property"##),
                     None,
                     Some(r##"A Google Analytics GA4 property identifier whose events are tracked. To learn more, see [where to find your Property ID](https://developers.google.com/analytics/devguides/reporting/data/v1/property-id). `property` should be the same value as in your `runReport` request. Example: properties/1234"##),
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
            ("get-metadata",
                    Some(r##"Returns metadata for dimensions and metrics available in reporting methods. Used to explore the dimensions and metrics. In this method, a Google Analytics GA4 Property Identifier is specified in the request, and the metadata response includes Custom dimensions and metrics as well as Universal metadata. For example if a custom metric with parameter name `levels_unlocked` is registered to a property, the Metadata response will contain `customEvent:levels_unlocked`. Universal metadata are dimensions and metrics applicable to any property such as `country` and `totalUsers`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analyticsdata1_beta_cli/properties_get-metadata",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name of the metadata to retrieve. This name field is specified in the URL path and not URL parameters. Property is a numeric Google Analytics GA4 Property identifier. To learn more, see [where to find your Property ID](https://developers.google.com/analytics/devguides/reporting/data/v1/property-id). Example: properties/1234/metadata Set the Property ID to 0 for dimensions and metrics common to all properties. In this special mode, this method will not return custom dimensions and metrics."##),
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
            ("run-pivot-report",
                    Some(r##"Returns a customized pivot report of your Google Analytics event data. Pivot reports are more advanced and expressive formats than regular reports. In a pivot report, dimensions are only visible if they are included in a pivot. Multiple pivots can be specified to further dissect your data."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analyticsdata1_beta_cli/properties_run-pivot-report",
                  vec![
                    (Some(r##"property"##),
                     None,
                     Some(r##"A Google Analytics GA4 property identifier whose events are tracked. Specified in the URL path and not the body. To learn more, see [where to find your Property ID](https://developers.google.com/analytics/devguides/reporting/data/v1/property-id). Within a batch request, this property should either be unspecified or consistent with the batch-level property. Example: properties/1234"##),
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
            ("run-realtime-report",
                    Some(r##"Returns a customized report of realtime event data for your property. Events appear in realtime reports seconds after they have been sent to the Google Analytics. Realtime reports show events and usage data for the periods of time ranging from the present moment to 30 minutes ago (up to 60 minutes for Google Analytics 360 properties). For a guide to constructing realtime requests & understanding responses, see [Creating a Realtime Report](https://developers.google.com/analytics/devguides/reporting/data/v1/realtime-basics)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analyticsdata1_beta_cli/properties_run-realtime-report",
                  vec![
                    (Some(r##"property"##),
                     None,
                     Some(r##"A Google Analytics GA4 property identifier whose events are tracked. Specified in the URL path and not the body. To learn more, see [where to find your Property ID](https://developers.google.com/analytics/devguides/reporting/data/v1/property-id). Example: properties/1234"##),
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
            ("run-report",
                    Some(r##"Returns a customized report of your Google Analytics event data. Reports contain statistics derived from data collected by the Google Analytics tracking code. The data returned from the API is as a table with columns for the requested dimensions and metrics. Metrics are individual measurements of user activity on your property, such as active users or event count. Dimensions break down metrics across some common criteria, such as country or event name. For a guide to constructing requests & understanding responses, see [Creating a Report](https://developers.google.com/analytics/devguides/reporting/data/v1/basics)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analyticsdata1_beta_cli/properties_run-report",
                  vec![
                    (Some(r##"property"##),
                     None,
                     Some(r##"A Google Analytics GA4 property identifier whose events are tracked. Specified in the URL path and not the body. To learn more, see [where to find your Property ID](https://developers.google.com/analytics/devguides/reporting/data/v1/property-id). Within a batch request, this property should either be unspecified or consistent with the batch-level property. Example: properties/1234"##),
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
    
    let mut app = App::new("analyticsdata1-beta")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+20240625")
           .about("Accesses report data in Google Analytics. Warning: Creating multiple Customer Applications, Accounts, or Projects to simulate or act as a single Customer Application, Account, or Project (respectively) or to circumvent Service-specific usage limits or quotas is a direct violation of Google Cloud Platform Terms of Service as well as Google APIs Terms of Service. These actions can result in immediate termination of your GCP project(s) without any warning. ")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_analyticsdata1_beta_cli")
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
