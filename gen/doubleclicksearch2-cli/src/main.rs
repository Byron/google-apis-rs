// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_doubleclicksearch2::{api, Error, oauth2, client::chrono, FieldMask};


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
    hub: api::Doubleclicksearch<S>,
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
    async fn _conversion_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let end_date: i32 = arg_from_str(&opt.value_of("end-date").unwrap_or(""), err, "<end-date>", "integer");
        let row_count: i32 = arg_from_str(&opt.value_of("row-count").unwrap_or(""), err, "<row-count>", "integer");
        let start_date: i32 = arg_from_str(&opt.value_of("start-date").unwrap_or(""), err, "<start-date>", "integer");
        let start_row: u32 = arg_from_str(&opt.value_of("start-row").unwrap_or(""), err, "<start-row>", "integer");
        let mut call = self.hub.conversion().get(opt.value_of("agency-id").unwrap_or(""), opt.value_of("advertiser-id").unwrap_or(""), opt.value_of("engine-account-id").unwrap_or(""), end_date, row_count, start_date, start_row);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "customer-id" => {
                    call = call.customer_id(value.unwrap_or(""));
                },
                "criterion-id" => {
                    call = call.criterion_id(        value.map(|v| arg_from_str(v, err, "criterion-id", "int64")).unwrap_or(-0));
                },
                "campaign-id" => {
                    call = call.campaign_id(        value.map(|v| arg_from_str(v, err, "campaign-id", "int64")).unwrap_or(-0));
                },
                "ad-id" => {
                    call = call.ad_id(        value.map(|v| arg_from_str(v, err, "ad-id", "int64")).unwrap_or(-0));
                },
                "ad-group-id" => {
                    call = call.ad_group_id(        value.map(|v| arg_from_str(v, err, "ad-group-id", "int64")).unwrap_or(-0));
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
                                                                           v.extend(["ad-group-id", "ad-id", "campaign-id", "criterion-id", "customer-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _conversion_get_by_customer_id(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let end_date: i32 = arg_from_str(&opt.value_of("end-date").unwrap_or(""), err, "<end-date>", "integer");
        let row_count: i32 = arg_from_str(&opt.value_of("row-count").unwrap_or(""), err, "<row-count>", "integer");
        let start_date: i32 = arg_from_str(&opt.value_of("start-date").unwrap_or(""), err, "<start-date>", "integer");
        let start_row: u32 = arg_from_str(&opt.value_of("start-row").unwrap_or(""), err, "<start-row>", "integer");
        let mut call = self.hub.conversion().get_by_customer_id(opt.value_of("customer-id").unwrap_or(""), end_date, row_count, start_date, start_row);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "engine-account-id" => {
                    call = call.engine_account_id(        value.map(|v| arg_from_str(v, err, "engine-account-id", "int64")).unwrap_or(-0));
                },
                "criterion-id" => {
                    call = call.criterion_id(        value.map(|v| arg_from_str(v, err, "criterion-id", "int64")).unwrap_or(-0));
                },
                "campaign-id" => {
                    call = call.campaign_id(        value.map(|v| arg_from_str(v, err, "campaign-id", "int64")).unwrap_or(-0));
                },
                "agency-id" => {
                    call = call.agency_id(        value.map(|v| arg_from_str(v, err, "agency-id", "int64")).unwrap_or(-0));
                },
                "advertiser-id" => {
                    call = call.advertiser_id(        value.map(|v| arg_from_str(v, err, "advertiser-id", "int64")).unwrap_or(-0));
                },
                "ad-id" => {
                    call = call.ad_id(        value.map(|v| arg_from_str(v, err, "ad-id", "int64")).unwrap_or(-0));
                },
                "ad-group-id" => {
                    call = call.ad_group_id(        value.map(|v| arg_from_str(v, err, "ad-group-id", "int64")).unwrap_or(-0));
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
                                                                           v.extend(["ad-group-id", "ad-id", "advertiser-id", "agency-id", "campaign-id", "criterion-id", "engine-account-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _conversion_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["kind"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ConversionList = json::value::from_value(object).unwrap();
        let mut call = self.hub.conversion().insert(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _conversion_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["kind"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ConversionList = json::value::from_value(object).unwrap();
        let mut call = self.hub.conversion().update(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _conversion_update_availability(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::UpdateAvailabilityRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.conversion().update_availability(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _reports_generate(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "download-format" => Some(("downloadFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "include-deleted-entities" => Some(("includeDeletedEntities", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "include-removed-entities" => Some(("includeRemovedEntities", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "max-rows-per-file" => Some(("maxRowsPerFile", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "report-scope.ad-group-id" => Some(("reportScope.adGroupId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "report-scope.ad-id" => Some(("reportScope.adId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "report-scope.advertiser-id" => Some(("reportScope.advertiserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "report-scope.agency-id" => Some(("reportScope.agencyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "report-scope.campaign-id" => Some(("reportScope.campaignId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "report-scope.engine-account-id" => Some(("reportScope.engineAccountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "report-scope.keyword-id" => Some(("reportScope.keywordId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "report-type" => Some(("reportType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "row-count" => Some(("rowCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "start-row" => Some(("startRow", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "statistics-currency" => Some(("statisticsCurrency", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-range.changed-attributes-since-timestamp" => Some(("timeRange.changedAttributesSinceTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-range.changed-metrics-since-timestamp" => Some(("timeRange.changedMetricsSinceTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-range.end-date" => Some(("timeRange.endDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-range.start-date" => Some(("timeRange.startDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "verify-single-time-zone" => Some(("verifySingleTimeZone", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["ad-group-id", "ad-id", "advertiser-id", "agency-id", "campaign-id", "changed-attributes-since-timestamp", "changed-metrics-since-timestamp", "download-format", "end-date", "engine-account-id", "include-deleted-entities", "include-removed-entities", "keyword-id", "max-rows-per-file", "report-scope", "report-type", "row-count", "start-date", "start-row", "statistics-currency", "time-range", "verify-single-time-zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ReportRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.reports().generate(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _reports_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.reports().get(opt.value_of("report-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _reports_get_file(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let report_fragment: i32 = arg_from_str(&opt.value_of("report-fragment").unwrap_or(""), err, "<report-fragment>", "integer");
        let mut download_mode = false;
        let mut call = self.hub.reports().get_file(opt.value_of("report-id").unwrap_or(""), report_fragment);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            if key == "alt" && value.unwrap_or("unset") == "media" {
                                download_mode = true;
                            }
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    if !download_mode {
                    } else {
                    let bytes = hyper::body::to_bytes(response.into_body()).await.expect("a string as API currently is inefficient").to_vec();
                    ostream.write_all(&bytes).expect("write to be complete");
                    ostream.flush().expect("io to never fail which should really be fixed one day");
                    }
                    Ok(())
                }
            }
        }
    }

    async fn _reports_get_id_mapping_file(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut download_mode = false;
        let mut call = self.hub.reports().get_id_mapping_file(opt.value_of("agency-id").unwrap_or(""), opt.value_of("advertiser-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            if key == "alt" && value.unwrap_or("unset") == "media" {
                                download_mode = true;
                            }
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    if !download_mode {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    } else {
                    let bytes = hyper::body::to_bytes(response.into_body()).await.expect("a string as API currently is inefficient").to_vec();
                    ostream.write_all(&bytes).expect("write to be complete");
                    ostream.flush().expect("io to never fail which should really be fixed one day");
                    }
                    Ok(())
                }
            }
        }
    }

    async fn _reports_request(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "download-format" => Some(("downloadFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "include-deleted-entities" => Some(("includeDeletedEntities", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "include-removed-entities" => Some(("includeRemovedEntities", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "max-rows-per-file" => Some(("maxRowsPerFile", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "report-scope.ad-group-id" => Some(("reportScope.adGroupId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "report-scope.ad-id" => Some(("reportScope.adId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "report-scope.advertiser-id" => Some(("reportScope.advertiserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "report-scope.agency-id" => Some(("reportScope.agencyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "report-scope.campaign-id" => Some(("reportScope.campaignId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "report-scope.engine-account-id" => Some(("reportScope.engineAccountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "report-scope.keyword-id" => Some(("reportScope.keywordId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "report-type" => Some(("reportType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "row-count" => Some(("rowCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "start-row" => Some(("startRow", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "statistics-currency" => Some(("statisticsCurrency", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-range.changed-attributes-since-timestamp" => Some(("timeRange.changedAttributesSinceTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-range.changed-metrics-since-timestamp" => Some(("timeRange.changedMetricsSinceTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-range.end-date" => Some(("timeRange.endDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-range.start-date" => Some(("timeRange.startDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "verify-single-time-zone" => Some(("verifySingleTimeZone", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["ad-group-id", "ad-id", "advertiser-id", "agency-id", "campaign-id", "changed-attributes-since-timestamp", "changed-metrics-since-timestamp", "download-format", "end-date", "engine-account-id", "include-deleted-entities", "include-removed-entities", "keyword-id", "max-rows-per-file", "report-scope", "report-type", "row-count", "start-date", "start-row", "statistics-currency", "time-range", "verify-single-time-zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ReportRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.reports().request(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _saved_columns_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.saved_columns().list(opt.value_of("agency-id").unwrap_or(""), opt.value_of("advertiser-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
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
            ("conversion", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._conversion_get(opt, dry_run, &mut err).await;
                    },
                    ("get-by-customer-id", Some(opt)) => {
                        call_result = self._conversion_get_by_customer_id(opt, dry_run, &mut err).await;
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._conversion_insert(opt, dry_run, &mut err).await;
                    },
                    ("update", Some(opt)) => {
                        call_result = self._conversion_update(opt, dry_run, &mut err).await;
                    },
                    ("update-availability", Some(opt)) => {
                        call_result = self._conversion_update_availability(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("conversion".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("reports", Some(opt)) => {
                match opt.subcommand() {
                    ("generate", Some(opt)) => {
                        call_result = self._reports_generate(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._reports_get(opt, dry_run, &mut err).await;
                    },
                    ("get-file", Some(opt)) => {
                        call_result = self._reports_get_file(opt, dry_run, &mut err).await;
                    },
                    ("get-id-mapping-file", Some(opt)) => {
                        call_result = self._reports_get_id_mapping_file(opt, dry_run, &mut err).await;
                    },
                    ("request", Some(opt)) => {
                        call_result = self._reports_request(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("reports".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("saved-columns", Some(opt)) => {
                match opt.subcommand() {
                    ("list", Some(opt)) => {
                        call_result = self._saved_columns_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("saved-columns".to_string()));
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

            match client::application_secret_from_directory(&config_dir, "doubleclicksearch2-secret.json",
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
        ).persist_tokens_to_disk(format!("{}/doubleclicksearch2", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::Doubleclicksearch::new(client, auth),
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
        ("conversion", "methods: 'get', 'get-by-customer-id', 'insert', 'update' and 'update-availability'", vec![
            ("get",
                    Some(r##"Retrieves a list of conversions from a DoubleClick Search engine account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_doubleclicksearch2_cli/conversion_get",
                  vec![
                    (Some(r##"agency-id"##),
                     None,
                     Some(r##"Numeric ID of the agency."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Numeric ID of the advertiser."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"engine-account-id"##),
                     None,
                     Some(r##"Numeric ID of the engine account."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"end-date"##),
                     None,
                     Some(r##"Last date (inclusive) on which to retrieve conversions. Format is yyyymmdd."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"row-count"##),
                     None,
                     Some(r##"The number of conversions to return per call."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"start-date"##),
                     None,
                     Some(r##"First date (inclusive) on which to retrieve conversions. Format is yyyymmdd."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"start-row"##),
                     None,
                     Some(r##"The 0-based starting index for retrieving conversions results."##),
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
            ("get-by-customer-id",
                    Some(r##"Retrieves a list of conversions from a DoubleClick Search engine account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_doubleclicksearch2_cli/conversion_get-by-customer-id",
                  vec![
                    (Some(r##"customer-id"##),
                     None,
                     Some(r##"Customer ID of a client account in the new Search Ads 360 experience."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"end-date"##),
                     None,
                     Some(r##"Last date (inclusive) on which to retrieve conversions. Format is yyyymmdd."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"row-count"##),
                     None,
                     Some(r##"The number of conversions to return per call."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"start-date"##),
                     None,
                     Some(r##"First date (inclusive) on which to retrieve conversions. Format is yyyymmdd."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"start-row"##),
                     None,
                     Some(r##"The 0-based starting index for retrieving conversions results."##),
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
                    Some(r##"Inserts a batch of new conversions into DoubleClick Search."##),
                    "Details at http://byron.github.io/google-apis-rs/google_doubleclicksearch2_cli/conversion_insert",
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
            ("update",
                    Some(r##"Updates a batch of conversions in DoubleClick Search."##),
                    "Details at http://byron.github.io/google-apis-rs/google_doubleclicksearch2_cli/conversion_update",
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
            ("update-availability",
                    Some(r##"Updates the availabilities of a batch of floodlight activities in DoubleClick Search."##),
                    "Details at http://byron.github.io/google-apis-rs/google_doubleclicksearch2_cli/conversion_update-availability",
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
        
        ("reports", "methods: 'generate', 'get', 'get-file', 'get-id-mapping-file' and 'request'", vec![
            ("generate",
                    Some(r##"Generates and returns a report immediately."##),
                    "Details at http://byron.github.io/google-apis-rs/google_doubleclicksearch2_cli/reports_generate",
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
                    Some(r##"Polls for the status of a report request."##),
                    "Details at http://byron.github.io/google-apis-rs/google_doubleclicksearch2_cli/reports_get",
                  vec![
                    (Some(r##"report-id"##),
                     None,
                     Some(r##"ID of the report request being polled."##),
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
            ("get-file",
                    Some(r##"Downloads a report file encoded in UTF-8."##),
                    "Details at http://byron.github.io/google-apis-rs/google_doubleclicksearch2_cli/reports_get-file",
                  vec![
                    (Some(r##"report-id"##),
                     None,
                     Some(r##"ID of the report."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"report-fragment"##),
                     None,
                     Some(r##"The index of the report fragment to download."##),
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
            ("get-id-mapping-file",
                    Some(r##"Downloads a csv file(encoded in UTF-8) that contains ID mappings between legacy SA360 and new SA360. The file includes all children entities of the given advertiser(e.g. engine accounts, campaigns, ad groups, etc.) that exist in both legacy SA360 and new SA360."##),
                    "Details at http://byron.github.io/google-apis-rs/google_doubleclicksearch2_cli/reports_get-id-mapping-file",
                  vec![
                    (Some(r##"agency-id"##),
                     None,
                     Some(r##"Legacy SA360 agency ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"Legacy SA360 advertiser ID."##),
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
            ("request",
                    Some(r##"Inserts a report request into the reporting system."##),
                    "Details at http://byron.github.io/google-apis-rs/google_doubleclicksearch2_cli/reports_request",
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
        
        ("saved-columns", "methods: 'list'", vec![
            ("list",
                    Some(r##"Retrieve the list of saved columns for a specified advertiser."##),
                    "Details at http://byron.github.io/google-apis-rs/google_doubleclicksearch2_cli/saved-columns_list",
                  vec![
                    (Some(r##"agency-id"##),
                     None,
                     Some(r##"DS ID of the agency."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"advertiser-id"##),
                     None,
                     Some(r##"DS ID of the advertiser."##),
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
    
    let mut app = App::new("doubleclicksearch2")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+20240618")
           .about("The Search Ads 360 API allows developers to automate uploading conversions and downloading reports from Search Ads 360.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_doubleclicksearch2_cli")
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
