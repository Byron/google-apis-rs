// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_jobs3::{api, Error, oauth2, client::chrono, FieldMask};


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
    hub: api::CloudTalentSolution<S>,
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
    async fn _projects_client_events_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "client-event.create-time" => Some(("clientEvent.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "client-event.event-id" => Some(("clientEvent.eventId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "client-event.extra-info" => Some(("clientEvent.extraInfo", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "client-event.job-event.jobs" => Some(("clientEvent.jobEvent.jobs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "client-event.job-event.type" => Some(("clientEvent.jobEvent.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "client-event.parent-event-id" => Some(("clientEvent.parentEventId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "client-event.request-id" => Some(("clientEvent.requestId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["client-event", "create-time", "event-id", "extra-info", "job-event", "jobs", "parent-event-id", "request-id", "type"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CreateClientEventRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().client_events_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_companies_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "company.career-site-uri" => Some(("company.careerSiteUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "company.derived-info.headquarters-location.lat-lng.latitude" => Some(("company.derivedInfo.headquartersLocation.latLng.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "company.derived-info.headquarters-location.lat-lng.longitude" => Some(("company.derivedInfo.headquartersLocation.latLng.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "company.derived-info.headquarters-location.location-type" => Some(("company.derivedInfo.headquartersLocation.locationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "company.derived-info.headquarters-location.postal-address.address-lines" => Some(("company.derivedInfo.headquartersLocation.postalAddress.addressLines", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "company.derived-info.headquarters-location.postal-address.administrative-area" => Some(("company.derivedInfo.headquartersLocation.postalAddress.administrativeArea", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "company.derived-info.headquarters-location.postal-address.language-code" => Some(("company.derivedInfo.headquartersLocation.postalAddress.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "company.derived-info.headquarters-location.postal-address.locality" => Some(("company.derivedInfo.headquartersLocation.postalAddress.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "company.derived-info.headquarters-location.postal-address.organization" => Some(("company.derivedInfo.headquartersLocation.postalAddress.organization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "company.derived-info.headquarters-location.postal-address.postal-code" => Some(("company.derivedInfo.headquartersLocation.postalAddress.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "company.derived-info.headquarters-location.postal-address.recipients" => Some(("company.derivedInfo.headquartersLocation.postalAddress.recipients", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "company.derived-info.headquarters-location.postal-address.region-code" => Some(("company.derivedInfo.headquartersLocation.postalAddress.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "company.derived-info.headquarters-location.postal-address.revision" => Some(("company.derivedInfo.headquartersLocation.postalAddress.revision", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "company.derived-info.headquarters-location.postal-address.sorting-code" => Some(("company.derivedInfo.headquartersLocation.postalAddress.sortingCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "company.derived-info.headquarters-location.postal-address.sublocality" => Some(("company.derivedInfo.headquartersLocation.postalAddress.sublocality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "company.derived-info.headquarters-location.radius-in-miles" => Some(("company.derivedInfo.headquartersLocation.radiusInMiles", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "company.display-name" => Some(("company.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "company.eeo-text" => Some(("company.eeoText", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "company.external-id" => Some(("company.externalId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "company.headquarters-address" => Some(("company.headquartersAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "company.hiring-agency" => Some(("company.hiringAgency", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "company.image-uri" => Some(("company.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "company.keyword-searchable-job-custom-attributes" => Some(("company.keywordSearchableJobCustomAttributes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "company.name" => Some(("company.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "company.size" => Some(("company.size", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "company.suspended" => Some(("company.suspended", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "company.website-uri" => Some(("company.websiteUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["address-lines", "administrative-area", "career-site-uri", "company", "derived-info", "display-name", "eeo-text", "external-id", "headquarters-address", "headquarters-location", "hiring-agency", "image-uri", "keyword-searchable-job-custom-attributes", "language-code", "lat-lng", "latitude", "locality", "location-type", "longitude", "name", "organization", "postal-address", "postal-code", "radius-in-miles", "recipients", "region-code", "revision", "size", "sorting-code", "sublocality", "suspended", "website-uri"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CreateCompanyRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().companies_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_companies_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().companies_delete(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_companies_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().companies_get(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_companies_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().companies_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "require-open-jobs" => {
                    call = call.require_open_jobs(        value.map(|v| arg_from_str(v, err, "require-open-jobs", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["page-size", "page-token", "require-open-jobs"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_companies_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "company.career-site-uri" => Some(("company.careerSiteUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "company.derived-info.headquarters-location.lat-lng.latitude" => Some(("company.derivedInfo.headquartersLocation.latLng.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "company.derived-info.headquarters-location.lat-lng.longitude" => Some(("company.derivedInfo.headquartersLocation.latLng.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "company.derived-info.headquarters-location.location-type" => Some(("company.derivedInfo.headquartersLocation.locationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "company.derived-info.headquarters-location.postal-address.address-lines" => Some(("company.derivedInfo.headquartersLocation.postalAddress.addressLines", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "company.derived-info.headquarters-location.postal-address.administrative-area" => Some(("company.derivedInfo.headquartersLocation.postalAddress.administrativeArea", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "company.derived-info.headquarters-location.postal-address.language-code" => Some(("company.derivedInfo.headquartersLocation.postalAddress.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "company.derived-info.headquarters-location.postal-address.locality" => Some(("company.derivedInfo.headquartersLocation.postalAddress.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "company.derived-info.headquarters-location.postal-address.organization" => Some(("company.derivedInfo.headquartersLocation.postalAddress.organization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "company.derived-info.headquarters-location.postal-address.postal-code" => Some(("company.derivedInfo.headquartersLocation.postalAddress.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "company.derived-info.headquarters-location.postal-address.recipients" => Some(("company.derivedInfo.headquartersLocation.postalAddress.recipients", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "company.derived-info.headquarters-location.postal-address.region-code" => Some(("company.derivedInfo.headquartersLocation.postalAddress.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "company.derived-info.headquarters-location.postal-address.revision" => Some(("company.derivedInfo.headquartersLocation.postalAddress.revision", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "company.derived-info.headquarters-location.postal-address.sorting-code" => Some(("company.derivedInfo.headquartersLocation.postalAddress.sortingCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "company.derived-info.headquarters-location.postal-address.sublocality" => Some(("company.derivedInfo.headquartersLocation.postalAddress.sublocality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "company.derived-info.headquarters-location.radius-in-miles" => Some(("company.derivedInfo.headquartersLocation.radiusInMiles", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "company.display-name" => Some(("company.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "company.eeo-text" => Some(("company.eeoText", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "company.external-id" => Some(("company.externalId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "company.headquarters-address" => Some(("company.headquartersAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "company.hiring-agency" => Some(("company.hiringAgency", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "company.image-uri" => Some(("company.imageUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "company.keyword-searchable-job-custom-attributes" => Some(("company.keywordSearchableJobCustomAttributes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "company.name" => Some(("company.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "company.size" => Some(("company.size", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "company.suspended" => Some(("company.suspended", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "company.website-uri" => Some(("company.websiteUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-mask" => Some(("updateMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["address-lines", "administrative-area", "career-site-uri", "company", "derived-info", "display-name", "eeo-text", "external-id", "headquarters-address", "headquarters-location", "hiring-agency", "image-uri", "keyword-searchable-job-custom-attributes", "language-code", "lat-lng", "latitude", "locality", "location-type", "longitude", "name", "organization", "postal-address", "postal-code", "radius-in-miles", "recipients", "region-code", "revision", "size", "sorting-code", "sublocality", "suspended", "update-mask", "website-uri"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::UpdateCompanyRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().companies_patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_complete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().complete(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "type" => {
                    call = call.type_(value.unwrap_or(""));
                },
                "scope" => {
                    call = call.scope(value.unwrap_or(""));
                },
                "query" => {
                    call = call.query(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "language-codes" => {
                    call = call.add_language_codes(value.unwrap_or(""));
                },
                "language-code" => {
                    call = call.language_code(value.unwrap_or(""));
                },
                "company-name" => {
                    call = call.company_name(value.unwrap_or(""));
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
                                                                           v.extend(["company-name", "language-code", "language-codes", "page-size", "query", "scope", "type"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_jobs_batch_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "filter" => Some(("filter", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["filter"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::BatchDeleteJobsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().jobs_batch_delete(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_jobs_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "job.addresses" => Some(("job.addresses", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.application-info.emails" => Some(("job.applicationInfo.emails", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.application-info.instruction" => Some(("job.applicationInfo.instruction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.application-info.uris" => Some(("job.applicationInfo.uris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.company-display-name" => Some(("job.companyDisplayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.company-name" => Some(("job.companyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.compensation-info.annualized-base-compensation-range.max-compensation.currency-code" => Some(("job.compensationInfo.annualizedBaseCompensationRange.maxCompensation.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.compensation-info.annualized-base-compensation-range.max-compensation.nanos" => Some(("job.compensationInfo.annualizedBaseCompensationRange.maxCompensation.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job.compensation-info.annualized-base-compensation-range.max-compensation.units" => Some(("job.compensationInfo.annualizedBaseCompensationRange.maxCompensation.units", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.compensation-info.annualized-base-compensation-range.min-compensation.currency-code" => Some(("job.compensationInfo.annualizedBaseCompensationRange.minCompensation.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.compensation-info.annualized-base-compensation-range.min-compensation.nanos" => Some(("job.compensationInfo.annualizedBaseCompensationRange.minCompensation.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job.compensation-info.annualized-base-compensation-range.min-compensation.units" => Some(("job.compensationInfo.annualizedBaseCompensationRange.minCompensation.units", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.compensation-info.annualized-total-compensation-range.max-compensation.currency-code" => Some(("job.compensationInfo.annualizedTotalCompensationRange.maxCompensation.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.compensation-info.annualized-total-compensation-range.max-compensation.nanos" => Some(("job.compensationInfo.annualizedTotalCompensationRange.maxCompensation.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job.compensation-info.annualized-total-compensation-range.max-compensation.units" => Some(("job.compensationInfo.annualizedTotalCompensationRange.maxCompensation.units", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.compensation-info.annualized-total-compensation-range.min-compensation.currency-code" => Some(("job.compensationInfo.annualizedTotalCompensationRange.minCompensation.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.compensation-info.annualized-total-compensation-range.min-compensation.nanos" => Some(("job.compensationInfo.annualizedTotalCompensationRange.minCompensation.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job.compensation-info.annualized-total-compensation-range.min-compensation.units" => Some(("job.compensationInfo.annualizedTotalCompensationRange.minCompensation.units", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.degree-types" => Some(("job.degreeTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.department" => Some(("job.department", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.derived-info.job-categories" => Some(("job.derivedInfo.jobCategories", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.description" => Some(("job.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.employment-types" => Some(("job.employmentTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.incentives" => Some(("job.incentives", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.job-benefits" => Some(("job.jobBenefits", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.job-end-time" => Some(("job.jobEndTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.job-level" => Some(("job.jobLevel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.job-start-time" => Some(("job.jobStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.language-code" => Some(("job.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.name" => Some(("job.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.posting-create-time" => Some(("job.postingCreateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.posting-expire-time" => Some(("job.postingExpireTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.posting-publish-time" => Some(("job.postingPublishTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.posting-region" => Some(("job.postingRegion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.posting-update-time" => Some(("job.postingUpdateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.processing-options.disable-street-address-resolution" => Some(("job.processingOptions.disableStreetAddressResolution", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job.processing-options.html-sanitization" => Some(("job.processingOptions.htmlSanitization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.promotion-value" => Some(("job.promotionValue", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job.qualifications" => Some(("job.qualifications", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.requisition-id" => Some(("job.requisitionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.responsibilities" => Some(("job.responsibilities", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.title" => Some(("job.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.visibility" => Some(("job.visibility", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["addresses", "annualized-base-compensation-range", "annualized-total-compensation-range", "application-info", "company-display-name", "company-name", "compensation-info", "currency-code", "degree-types", "department", "derived-info", "description", "disable-street-address-resolution", "emails", "employment-types", "html-sanitization", "incentives", "instruction", "job", "job-benefits", "job-categories", "job-end-time", "job-level", "job-start-time", "language-code", "max-compensation", "min-compensation", "name", "nanos", "posting-create-time", "posting-expire-time", "posting-publish-time", "posting-region", "posting-update-time", "processing-options", "promotion-value", "qualifications", "requisition-id", "responsibilities", "title", "units", "uris", "visibility"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CreateJobRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().jobs_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_jobs_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().jobs_delete(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_jobs_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().jobs_get(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_jobs_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().jobs_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "job-view" => {
                    call = call.job_view(value.unwrap_or(""));
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
                                                                           v.extend(["filter", "job-view", "page-size", "page-token"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_jobs_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "job.addresses" => Some(("job.addresses", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.application-info.emails" => Some(("job.applicationInfo.emails", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.application-info.instruction" => Some(("job.applicationInfo.instruction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.application-info.uris" => Some(("job.applicationInfo.uris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.company-display-name" => Some(("job.companyDisplayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.company-name" => Some(("job.companyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.compensation-info.annualized-base-compensation-range.max-compensation.currency-code" => Some(("job.compensationInfo.annualizedBaseCompensationRange.maxCompensation.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.compensation-info.annualized-base-compensation-range.max-compensation.nanos" => Some(("job.compensationInfo.annualizedBaseCompensationRange.maxCompensation.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job.compensation-info.annualized-base-compensation-range.max-compensation.units" => Some(("job.compensationInfo.annualizedBaseCompensationRange.maxCompensation.units", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.compensation-info.annualized-base-compensation-range.min-compensation.currency-code" => Some(("job.compensationInfo.annualizedBaseCompensationRange.minCompensation.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.compensation-info.annualized-base-compensation-range.min-compensation.nanos" => Some(("job.compensationInfo.annualizedBaseCompensationRange.minCompensation.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job.compensation-info.annualized-base-compensation-range.min-compensation.units" => Some(("job.compensationInfo.annualizedBaseCompensationRange.minCompensation.units", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.compensation-info.annualized-total-compensation-range.max-compensation.currency-code" => Some(("job.compensationInfo.annualizedTotalCompensationRange.maxCompensation.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.compensation-info.annualized-total-compensation-range.max-compensation.nanos" => Some(("job.compensationInfo.annualizedTotalCompensationRange.maxCompensation.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job.compensation-info.annualized-total-compensation-range.max-compensation.units" => Some(("job.compensationInfo.annualizedTotalCompensationRange.maxCompensation.units", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.compensation-info.annualized-total-compensation-range.min-compensation.currency-code" => Some(("job.compensationInfo.annualizedTotalCompensationRange.minCompensation.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.compensation-info.annualized-total-compensation-range.min-compensation.nanos" => Some(("job.compensationInfo.annualizedTotalCompensationRange.minCompensation.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job.compensation-info.annualized-total-compensation-range.min-compensation.units" => Some(("job.compensationInfo.annualizedTotalCompensationRange.minCompensation.units", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.degree-types" => Some(("job.degreeTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.department" => Some(("job.department", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.derived-info.job-categories" => Some(("job.derivedInfo.jobCategories", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.description" => Some(("job.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.employment-types" => Some(("job.employmentTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.incentives" => Some(("job.incentives", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.job-benefits" => Some(("job.jobBenefits", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job.job-end-time" => Some(("job.jobEndTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.job-level" => Some(("job.jobLevel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.job-start-time" => Some(("job.jobStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.language-code" => Some(("job.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.name" => Some(("job.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.posting-create-time" => Some(("job.postingCreateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.posting-expire-time" => Some(("job.postingExpireTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.posting-publish-time" => Some(("job.postingPublishTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.posting-region" => Some(("job.postingRegion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.posting-update-time" => Some(("job.postingUpdateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.processing-options.disable-street-address-resolution" => Some(("job.processingOptions.disableStreetAddressResolution", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job.processing-options.html-sanitization" => Some(("job.processingOptions.htmlSanitization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.promotion-value" => Some(("job.promotionValue", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job.qualifications" => Some(("job.qualifications", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.requisition-id" => Some(("job.requisitionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.responsibilities" => Some(("job.responsibilities", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.title" => Some(("job.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job.visibility" => Some(("job.visibility", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-mask" => Some(("updateMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["addresses", "annualized-base-compensation-range", "annualized-total-compensation-range", "application-info", "company-display-name", "company-name", "compensation-info", "currency-code", "degree-types", "department", "derived-info", "description", "disable-street-address-resolution", "emails", "employment-types", "html-sanitization", "incentives", "instruction", "job", "job-benefits", "job-categories", "job-end-time", "job-level", "job-start-time", "language-code", "max-compensation", "min-compensation", "name", "nanos", "posting-create-time", "posting-expire-time", "posting-publish-time", "posting-region", "posting-update-time", "processing-options", "promotion-value", "qualifications", "requisition-id", "responsibilities", "title", "units", "update-mask", "uris", "visibility"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::UpdateJobRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().jobs_patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_jobs_search(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "disable-keyword-match" => Some(("disableKeywordMatch", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "diversification-level" => Some(("diversificationLevel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "enable-broadening" => Some(("enableBroadening", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "histogram-facets.simple-histogram-facets" => Some(("histogramFacets.simpleHistogramFacets", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-query.commute-filter.allow-imprecise-addresses" => Some(("jobQuery.commuteFilter.allowImpreciseAddresses", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job-query.commute-filter.commute-method" => Some(("jobQuery.commuteFilter.commuteMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-query.commute-filter.departure-time.hours" => Some(("jobQuery.commuteFilter.departureTime.hours", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-query.commute-filter.departure-time.minutes" => Some(("jobQuery.commuteFilter.departureTime.minutes", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-query.commute-filter.departure-time.nanos" => Some(("jobQuery.commuteFilter.departureTime.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-query.commute-filter.departure-time.seconds" => Some(("jobQuery.commuteFilter.departureTime.seconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-query.commute-filter.road-traffic" => Some(("jobQuery.commuteFilter.roadTraffic", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-query.commute-filter.start-coordinates.latitude" => Some(("jobQuery.commuteFilter.startCoordinates.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "job-query.commute-filter.start-coordinates.longitude" => Some(("jobQuery.commuteFilter.startCoordinates.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "job-query.commute-filter.travel-duration" => Some(("jobQuery.commuteFilter.travelDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-query.company-display-names" => Some(("jobQuery.companyDisplayNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-query.company-names" => Some(("jobQuery.companyNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-query.compensation-filter.include-jobs-with-unspecified-compensation-range" => Some(("jobQuery.compensationFilter.includeJobsWithUnspecifiedCompensationRange", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job-query.compensation-filter.range.max-compensation.currency-code" => Some(("jobQuery.compensationFilter.range.maxCompensation.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-query.compensation-filter.range.max-compensation.nanos" => Some(("jobQuery.compensationFilter.range.maxCompensation.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-query.compensation-filter.range.max-compensation.units" => Some(("jobQuery.compensationFilter.range.maxCompensation.units", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-query.compensation-filter.range.min-compensation.currency-code" => Some(("jobQuery.compensationFilter.range.minCompensation.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-query.compensation-filter.range.min-compensation.nanos" => Some(("jobQuery.compensationFilter.range.minCompensation.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-query.compensation-filter.range.min-compensation.units" => Some(("jobQuery.compensationFilter.range.minCompensation.units", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-query.compensation-filter.type" => Some(("jobQuery.compensationFilter.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-query.compensation-filter.units" => Some(("jobQuery.compensationFilter.units", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-query.custom-attribute-filter" => Some(("jobQuery.customAttributeFilter", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-query.disable-spell-check" => Some(("jobQuery.disableSpellCheck", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job-query.employment-types" => Some(("jobQuery.employmentTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-query.job-categories" => Some(("jobQuery.jobCategories", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-query.language-codes" => Some(("jobQuery.languageCodes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-query.publish-time-range.end-time" => Some(("jobQuery.publishTimeRange.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-query.publish-time-range.start-time" => Some(("jobQuery.publishTimeRange.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-query.query" => Some(("jobQuery.query", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-query.query-language-code" => Some(("jobQuery.queryLanguageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-view" => Some(("jobView", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "offset" => Some(("offset", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "order-by" => Some(("orderBy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "page-size" => Some(("pageSize", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "page-token" => Some(("pageToken", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.device-info.device-type" => Some(("requestMetadata.deviceInfo.deviceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.device-info.id" => Some(("requestMetadata.deviceInfo.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.domain" => Some(("requestMetadata.domain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.session-id" => Some(("requestMetadata.sessionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.user-id" => Some(("requestMetadata.userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "require-precise-result-size" => Some(("requirePreciseResultSize", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "search-mode" => Some(("searchMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["allow-imprecise-addresses", "commute-filter", "commute-method", "company-display-names", "company-names", "compensation-filter", "currency-code", "custom-attribute-filter", "departure-time", "device-info", "device-type", "disable-keyword-match", "disable-spell-check", "diversification-level", "domain", "employment-types", "enable-broadening", "end-time", "histogram-facets", "hours", "id", "include-jobs-with-unspecified-compensation-range", "job-categories", "job-query", "job-view", "language-codes", "latitude", "longitude", "max-compensation", "min-compensation", "minutes", "nanos", "offset", "order-by", "page-size", "page-token", "publish-time-range", "query", "query-language-code", "range", "request-metadata", "require-precise-result-size", "road-traffic", "search-mode", "seconds", "session-id", "simple-histogram-facets", "start-coordinates", "start-time", "travel-duration", "type", "units", "user-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SearchJobsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().jobs_search(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_jobs_search_for_alert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "disable-keyword-match" => Some(("disableKeywordMatch", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "diversification-level" => Some(("diversificationLevel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "enable-broadening" => Some(("enableBroadening", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "histogram-facets.simple-histogram-facets" => Some(("histogramFacets.simpleHistogramFacets", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-query.commute-filter.allow-imprecise-addresses" => Some(("jobQuery.commuteFilter.allowImpreciseAddresses", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job-query.commute-filter.commute-method" => Some(("jobQuery.commuteFilter.commuteMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-query.commute-filter.departure-time.hours" => Some(("jobQuery.commuteFilter.departureTime.hours", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-query.commute-filter.departure-time.minutes" => Some(("jobQuery.commuteFilter.departureTime.minutes", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-query.commute-filter.departure-time.nanos" => Some(("jobQuery.commuteFilter.departureTime.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-query.commute-filter.departure-time.seconds" => Some(("jobQuery.commuteFilter.departureTime.seconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-query.commute-filter.road-traffic" => Some(("jobQuery.commuteFilter.roadTraffic", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-query.commute-filter.start-coordinates.latitude" => Some(("jobQuery.commuteFilter.startCoordinates.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "job-query.commute-filter.start-coordinates.longitude" => Some(("jobQuery.commuteFilter.startCoordinates.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "job-query.commute-filter.travel-duration" => Some(("jobQuery.commuteFilter.travelDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-query.company-display-names" => Some(("jobQuery.companyDisplayNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-query.company-names" => Some(("jobQuery.companyNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-query.compensation-filter.include-jobs-with-unspecified-compensation-range" => Some(("jobQuery.compensationFilter.includeJobsWithUnspecifiedCompensationRange", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job-query.compensation-filter.range.max-compensation.currency-code" => Some(("jobQuery.compensationFilter.range.maxCompensation.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-query.compensation-filter.range.max-compensation.nanos" => Some(("jobQuery.compensationFilter.range.maxCompensation.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-query.compensation-filter.range.max-compensation.units" => Some(("jobQuery.compensationFilter.range.maxCompensation.units", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-query.compensation-filter.range.min-compensation.currency-code" => Some(("jobQuery.compensationFilter.range.minCompensation.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-query.compensation-filter.range.min-compensation.nanos" => Some(("jobQuery.compensationFilter.range.minCompensation.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-query.compensation-filter.range.min-compensation.units" => Some(("jobQuery.compensationFilter.range.minCompensation.units", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-query.compensation-filter.type" => Some(("jobQuery.compensationFilter.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-query.compensation-filter.units" => Some(("jobQuery.compensationFilter.units", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-query.custom-attribute-filter" => Some(("jobQuery.customAttributeFilter", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-query.disable-spell-check" => Some(("jobQuery.disableSpellCheck", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job-query.employment-types" => Some(("jobQuery.employmentTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-query.job-categories" => Some(("jobQuery.jobCategories", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-query.language-codes" => Some(("jobQuery.languageCodes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-query.publish-time-range.end-time" => Some(("jobQuery.publishTimeRange.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-query.publish-time-range.start-time" => Some(("jobQuery.publishTimeRange.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-query.query" => Some(("jobQuery.query", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-query.query-language-code" => Some(("jobQuery.queryLanguageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-view" => Some(("jobView", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "offset" => Some(("offset", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "order-by" => Some(("orderBy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "page-size" => Some(("pageSize", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "page-token" => Some(("pageToken", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.device-info.device-type" => Some(("requestMetadata.deviceInfo.deviceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.device-info.id" => Some(("requestMetadata.deviceInfo.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.domain" => Some(("requestMetadata.domain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.session-id" => Some(("requestMetadata.sessionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-metadata.user-id" => Some(("requestMetadata.userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "require-precise-result-size" => Some(("requirePreciseResultSize", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "search-mode" => Some(("searchMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["allow-imprecise-addresses", "commute-filter", "commute-method", "company-display-names", "company-names", "compensation-filter", "currency-code", "custom-attribute-filter", "departure-time", "device-info", "device-type", "disable-keyword-match", "disable-spell-check", "diversification-level", "domain", "employment-types", "enable-broadening", "end-time", "histogram-facets", "hours", "id", "include-jobs-with-unspecified-compensation-range", "job-categories", "job-query", "job-view", "language-codes", "latitude", "longitude", "max-compensation", "min-compensation", "minutes", "nanos", "offset", "order-by", "page-size", "page-token", "publish-time-range", "query", "query-language-code", "range", "request-metadata", "require-precise-result-size", "road-traffic", "search-mode", "seconds", "session-id", "simple-histogram-facets", "start-coordinates", "start-time", "travel-duration", "type", "units", "user-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SearchJobsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().jobs_search_for_alert(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
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
                    ("client-events-create", Some(opt)) => {
                        call_result = self._projects_client_events_create(opt, dry_run, &mut err).await;
                    },
                    ("companies-create", Some(opt)) => {
                        call_result = self._projects_companies_create(opt, dry_run, &mut err).await;
                    },
                    ("companies-delete", Some(opt)) => {
                        call_result = self._projects_companies_delete(opt, dry_run, &mut err).await;
                    },
                    ("companies-get", Some(opt)) => {
                        call_result = self._projects_companies_get(opt, dry_run, &mut err).await;
                    },
                    ("companies-list", Some(opt)) => {
                        call_result = self._projects_companies_list(opt, dry_run, &mut err).await;
                    },
                    ("companies-patch", Some(opt)) => {
                        call_result = self._projects_companies_patch(opt, dry_run, &mut err).await;
                    },
                    ("complete", Some(opt)) => {
                        call_result = self._projects_complete(opt, dry_run, &mut err).await;
                    },
                    ("jobs-batch-delete", Some(opt)) => {
                        call_result = self._projects_jobs_batch_delete(opt, dry_run, &mut err).await;
                    },
                    ("jobs-create", Some(opt)) => {
                        call_result = self._projects_jobs_create(opt, dry_run, &mut err).await;
                    },
                    ("jobs-delete", Some(opt)) => {
                        call_result = self._projects_jobs_delete(opt, dry_run, &mut err).await;
                    },
                    ("jobs-get", Some(opt)) => {
                        call_result = self._projects_jobs_get(opt, dry_run, &mut err).await;
                    },
                    ("jobs-list", Some(opt)) => {
                        call_result = self._projects_jobs_list(opt, dry_run, &mut err).await;
                    },
                    ("jobs-patch", Some(opt)) => {
                        call_result = self._projects_jobs_patch(opt, dry_run, &mut err).await;
                    },
                    ("jobs-search", Some(opt)) => {
                        call_result = self._projects_jobs_search(opt, dry_run, &mut err).await;
                    },
                    ("jobs-search-for-alert", Some(opt)) => {
                        call_result = self._projects_jobs_search_for_alert(opt, dry_run, &mut err).await;
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

            match client::application_secret_from_directory(&config_dir, "jobs3-secret.json",
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
        ).persist_tokens_to_disk(format!("{}/jobs3", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::CloudTalentSolution::new(client, auth),
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
        ("projects", "methods: 'client-events-create', 'companies-create', 'companies-delete', 'companies-get', 'companies-list', 'companies-patch', 'complete', 'jobs-batch-delete', 'jobs-create', 'jobs-delete', 'jobs-get', 'jobs-list', 'jobs-patch', 'jobs-search' and 'jobs-search-for-alert'", vec![
            ("client-events-create",
                    Some(r##"Report events issued when end user interacts with customer's application that uses Cloud Talent Solution. You may inspect the created events in [self service tools](https://console.cloud.google.com/talent-solution/overview). [Learn more](https://cloud.google.com/talent-solution/docs/management-tools) about self service tools."##),
                    "Details at http://byron.github.io/google-apis-rs/google_jobs3_cli/projects_client-events-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Parent project name."##),
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
            ("companies-create",
                    Some(r##"Creates a new company entity."##),
                    "Details at http://byron.github.io/google-apis-rs/google_jobs3_cli/projects_companies-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Resource name of the project under which the company is created. The format is "projects/{project_id}", for example, "projects/api-test-project"."##),
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
            ("companies-delete",
                    Some(r##"Deletes specified company. Prerequisite: The company has no jobs associated with it."##),
                    "Details at http://byron.github.io/google-apis-rs/google_jobs3_cli/projects_companies-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name of the company to be deleted. The format is "projects/{project_id}/companies/{company_id}", for example, "projects/api-test-project/companies/foo"."##),
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
            ("companies-get",
                    Some(r##"Retrieves specified company."##),
                    "Details at http://byron.github.io/google-apis-rs/google_jobs3_cli/projects_companies-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name of the company to be retrieved. The format is "projects/{project_id}/companies/{company_id}", for example, "projects/api-test-project/companies/foo"."##),
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
            ("companies-list",
                    Some(r##"Lists all companies associated with the service account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_jobs3_cli/projects_companies-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. Resource name of the project under which the company is created. The format is "projects/{project_id}", for example, "projects/api-test-project"."##),
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
            ("companies-patch",
                    Some(r##"Updates specified company. Company names can't be updated. To update a company name, delete the company and all jobs associated with it, and only then re-create them."##),
                    "Details at http://byron.github.io/google-apis-rs/google_jobs3_cli/projects_companies-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required during company update. The resource name for a company. This is generated by the service when a company is created. The format is "projects/{project_id}/companies/{company_id}", for example, "projects/api-test-project/companies/foo"."##),
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
            ("complete",
                    Some(r##"Completes the specified prefix with keyword suggestions. Intended for use by a job search auto-complete search box."##),
                    "Details at http://byron.github.io/google-apis-rs/google_jobs3_cli/projects_complete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Resource name of project the completion is performed within. The format is "projects/{project_id}", for example, "projects/api-test-project"."##),
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
            ("jobs-batch-delete",
                    Some(r##"Deletes a list of Jobs by filter."##),
                    "Details at http://byron.github.io/google-apis-rs/google_jobs3_cli/projects_jobs-batch-delete",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The resource name of the project under which the job is created. The format is "projects/{project_id}", for example, "projects/api-test-project"."##),
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
            ("jobs-create",
                    Some(r##"Creates a new job. Typically, the job becomes searchable within 10 seconds, but it may take up to 5 minutes."##),
                    "Details at http://byron.github.io/google-apis-rs/google_jobs3_cli/projects_jobs-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The resource name of the project under which the job is created. The format is "projects/{project_id}", for example, "projects/api-test-project"."##),
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
            ("jobs-delete",
                    Some(r##"Deletes the specified job. Typically, the job becomes unsearchable within 10 seconds, but it may take up to 5 minutes."##),
                    "Details at http://byron.github.io/google-apis-rs/google_jobs3_cli/projects_jobs-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name of the job to be deleted. The format is "projects/{project_id}/jobs/{job_id}", for example, "projects/api-test-project/jobs/1234"."##),
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
            ("jobs-get",
                    Some(r##"Retrieves the specified job, whose status is OPEN or recently EXPIRED within the last 90 days."##),
                    "Details at http://byron.github.io/google-apis-rs/google_jobs3_cli/projects_jobs-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name of the job to retrieve. The format is "projects/{project_id}/jobs/{job_id}", for example, "projects/api-test-project/jobs/1234"."##),
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
            ("jobs-list",
                    Some(r##"Lists jobs by filter."##),
                    "Details at http://byron.github.io/google-apis-rs/google_jobs3_cli/projects_jobs-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The resource name of the project under which the job is created. The format is "projects/{project_id}", for example, "projects/api-test-project"."##),
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
            ("jobs-patch",
                    Some(r##"Updates specified job. Typically, updated contents become visible in search results within 10 seconds, but it may take up to 5 minutes."##),
                    "Details at http://byron.github.io/google-apis-rs/google_jobs3_cli/projects_jobs-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required during job update. The resource name for the job. This is generated by the service when a job is created. The format is "projects/{project_id}/jobs/{job_id}", for example, "projects/api-test-project/jobs/1234". Use of this field in job queries and API calls is preferred over the use of requisition_id since this value is unique."##),
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
            ("jobs-search",
                    Some(r##"Searches for jobs using the provided SearchJobsRequest. This call constrains the visibility of jobs present in the database, and only returns jobs that the caller has permission to search against."##),
                    "Details at http://byron.github.io/google-apis-rs/google_jobs3_cli/projects_jobs-search",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The resource name of the project to search within. The format is "projects/{project_id}", for example, "projects/api-test-project"."##),
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
            ("jobs-search-for-alert",
                    Some(r##"Searches for jobs using the provided SearchJobsRequest. This API call is intended for the use case of targeting passive job seekers (for example, job seekers who have signed up to receive email alerts about potential job opportunities), and has different algorithmic adjustments that are targeted to passive job seekers. This call constrains the visibility of jobs present in the database, and only returns jobs the caller has permission to search against."##),
                    "Details at http://byron.github.io/google-apis-rs/google_jobs3_cli/projects_jobs-search-for-alert",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The resource name of the project to search within. The format is "projects/{project_id}", for example, "projects/api-test-project"."##),
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
    
    let mut app = App::new("jobs3")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+20240620")
           .about("Cloud Talent Solution provides the capability to create, read, update, and delete job postings, as well as search jobs based on keywords and filters. ")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_jobs3_cli")
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
