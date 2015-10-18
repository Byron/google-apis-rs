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
extern crate google_plus1 as api;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

mod cmn;

use cmn::{InvalidOptionsError, CLIError, JsonTokenStorage, arg_from_str, writer_from_opts, parse_kv_arg, 
          input_file_from_opts, input_mime_from_opts, FieldCursor, FieldError, CallType, UploadProtocol,
          calltype_from_str, remove_json_null_values, ComplexType, JsonType, JsonTypeInfo};

use std::default::Default;
use std::str::FromStr;

use oauth2::{Authenticator, DefaultAuthenticatorDelegate};
use serde_json as json;
use clap::ArgMatches;

enum DoitError {
    IoError(String, io::Error),
    ApiError(api::Error),
}

struct Engine<'n, 'a> {
    opt: ArgMatches<'n, 'a>,
    hub: api::Plus<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n, 'a> Engine<'n, 'a> {
    fn _activities_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.activities().get(opt.value_of("activity-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(), 
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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

    fn _activities_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.activities().list(opt.value_of("user-id").unwrap_or(""), opt.value_of("collection").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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

    fn _activities_search(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.activities().search(opt.value_of("query").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "language" => {
                    call = call.language(value.unwrap_or(""));
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
                                                                           v.extend(["order-by", "page-token", "language", "max-results"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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

    fn _comments_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.comments().get(opt.value_of("comment-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(), 
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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

    fn _comments_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.comments().list(opt.value_of("activity-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "sort-order" => {
                    call = call.sort_order(value.unwrap_or(""));
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
                                                                           v.extend(["page-token", "sort-order", "max-results"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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

    fn _moments_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
                    "start-date" => Some(("startDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "target.start-date" => Some(("target.startDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "target.end-date" => Some(("target.endDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "target.text" => Some(("target.text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "target.image" => Some(("target.image", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "target.birth-date" => Some(("target.birthDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "target.date-published" => Some(("target.datePublished", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "target.address-locality" => Some(("target.addressLocality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "target.additional-name" => Some(("target.additionalName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "target.worst-rating" => Some(("target.worstRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "target.duration" => Some(("target.duration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "target.thumbnail-url" => Some(("target.thumbnailUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "target.id" => Some(("target.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "target.post-office-box-number" => Some(("target.postOfficeBoxNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "target.caption" => Some(("target.caption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "target.best-rating" => Some(("target.bestRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "target.address-country" => Some(("target.addressCountry", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "target.width" => Some(("target.width", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "target.street-address" => Some(("target.streetAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "target.latitude" => Some(("target.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "target.embed-url" => Some(("target.embedUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "target.type" => Some(("target.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "target.date-modified" => Some(("target.dateModified", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "target.content-size" => Some(("target.contentSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "target.content-url" => Some(("target.contentUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "target.description" => Some(("target.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "target.family-name" => Some(("target.familyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "target.date-created" => Some(("target.dateCreated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "target.postal-code" => Some(("target.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "target.attendee-count" => Some(("target.attendeeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "target.height" => Some(("target.height", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "target.ticker-symbol" => Some(("target.tickerSymbol", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "target.player-type" => Some(("target.playerType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "target.kind" => Some(("target.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "target.name" => Some(("target.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "target.url" => Some(("target.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "target.gender" => Some(("target.gender", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "target.longitude" => Some(("target.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "target.address-region" => Some(("target.addressRegion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "target.rating-value" => Some(("target.ratingValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "target.given-name" => Some(("target.givenName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object.start-date" => Some(("object.startDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object.end-date" => Some(("object.endDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object.text" => Some(("object.text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object.image" => Some(("object.image", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object.birth-date" => Some(("object.birthDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object.date-published" => Some(("object.datePublished", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object.address-locality" => Some(("object.addressLocality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object.additional-name" => Some(("object.additionalName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "object.worst-rating" => Some(("object.worstRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object.duration" => Some(("object.duration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object.thumbnail-url" => Some(("object.thumbnailUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object.id" => Some(("object.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object.post-office-box-number" => Some(("object.postOfficeBoxNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object.caption" => Some(("object.caption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object.best-rating" => Some(("object.bestRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object.address-country" => Some(("object.addressCountry", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "object.width" => Some(("object.width", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object.street-address" => Some(("object.streetAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object.latitude" => Some(("object.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "object.embed-url" => Some(("object.embedUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object.type" => Some(("object.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object.date-modified" => Some(("object.dateModified", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object.content-size" => Some(("object.contentSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object.content-url" => Some(("object.contentUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object.description" => Some(("object.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object.family-name" => Some(("object.familyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object.date-created" => Some(("object.dateCreated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object.postal-code" => Some(("object.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object.attendee-count" => Some(("object.attendeeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "object.height" => Some(("object.height", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object.ticker-symbol" => Some(("object.tickerSymbol", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object.player-type" => Some(("object.playerType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object.kind" => Some(("object.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object.name" => Some(("object.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object.url" => Some(("object.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object.gender" => Some(("object.gender", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object.longitude" => Some(("object.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "object.address-region" => Some(("object.addressRegion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object.rating-value" => Some(("object.ratingValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "object.given-name" => Some(("object.givenName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result.start-date" => Some(("result.startDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result.end-date" => Some(("result.endDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result.text" => Some(("result.text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result.image" => Some(("result.image", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result.birth-date" => Some(("result.birthDate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result.date-published" => Some(("result.datePublished", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result.address-locality" => Some(("result.addressLocality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result.additional-name" => Some(("result.additionalName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "result.worst-rating" => Some(("result.worstRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result.duration" => Some(("result.duration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result.thumbnail-url" => Some(("result.thumbnailUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result.id" => Some(("result.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result.post-office-box-number" => Some(("result.postOfficeBoxNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result.caption" => Some(("result.caption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result.best-rating" => Some(("result.bestRating", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result.address-country" => Some(("result.addressCountry", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "result.width" => Some(("result.width", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result.street-address" => Some(("result.streetAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result.latitude" => Some(("result.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "result.embed-url" => Some(("result.embedUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result.type" => Some(("result.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result.date-modified" => Some(("result.dateModified", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result.content-size" => Some(("result.contentSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result.content-url" => Some(("result.contentUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result.description" => Some(("result.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result.family-name" => Some(("result.familyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result.date-created" => Some(("result.dateCreated", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result.postal-code" => Some(("result.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result.attendee-count" => Some(("result.attendeeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "result.height" => Some(("result.height", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result.ticker-symbol" => Some(("result.tickerSymbol", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result.player-type" => Some(("result.playerType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result.kind" => Some(("result.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result.name" => Some(("result.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result.url" => Some(("result.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result.gender" => Some(("result.gender", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result.longitude" => Some(("result.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "result.address-region" => Some(("result.addressRegion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result.rating-value" => Some(("result.ratingValue", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result.given-name" => Some(("result.givenName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["additional-name", "address-country", "address-locality", "address-region", "attendee-count", "best-rating", "birth-date", "caption", "content-size", "content-url", "date-created", "date-modified", "date-published", "description", "duration", "embed-url", "end-date", "family-name", "gender", "given-name", "height", "id", "image", "kind", "latitude", "longitude", "name", "object", "player-type", "post-office-box-number", "postal-code", "rating-value", "result", "start-date", "street-address", "target", "text", "thumbnail-url", "ticker-symbol", "type", "url", "width", "worst-rating"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Moment = json::value::from_value(object).unwrap();
        let mut call = self.hub.moments().insert(request, opt.value_of("user-id").unwrap_or(""), opt.value_of("collection").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "debug" => {
                    call = call.debug(arg_from_str(value.unwrap_or("false"), err, "debug", "boolean"));
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
                                                                           v.extend(["debug"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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

    fn _moments_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.moments().list(opt.value_of("user-id").unwrap_or(""), opt.value_of("collection").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "type" => {
                    call = call.type_(value.unwrap_or(""));
                },
                "target-url" => {
                    call = call.target_url(value.unwrap_or(""));
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
                                                                           v.extend(["page-token", "type", "max-results", "target-url"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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

    fn _people_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.people().get(opt.value_of("user-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(), 
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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

    fn _people_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.people().list(opt.value_of("user-id").unwrap_or(""), opt.value_of("collection").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
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
                                                                           v.extend(["order-by", "page-token", "max-results"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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

    fn _people_list_by_activity(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.people().list_by_activity(opt.value_of("activity-id").unwrap_or(""), opt.value_of("collection").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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

    fn _people_search(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.people().search(opt.value_of("query").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "language" => {
                    call = call.language(value.unwrap_or(""));
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
                                                                           v.extend(["page-token", "language", "max-results"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
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
            ("activities", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._activities_get(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._activities_list(opt, dry_run, &mut err);
                    },
                    ("search", Some(opt)) => {
                        call_result = self._activities_search(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("activities".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("comments", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._comments_get(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._comments_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("comments".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("moments", Some(opt)) => {
                match opt.subcommand() {
                    ("insert", Some(opt)) => {
                        call_result = self._moments_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._moments_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("moments".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("people", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._people_get(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._people_list(opt, dry_run, &mut err);
                    },
                    ("list-by-activity", Some(opt)) => {
                        call_result = self._people_list_by_activity(opt, dry_run, &mut err);
                    },
                    ("search", Some(opt)) => {
                        call_result = self._people_search(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("people".to_string()));
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
    fn new(opt: ArgMatches<'a, 'n>) -> Result<Engine<'a, 'n>, InvalidOptionsError> {
        let (config_dir, secret) = {
            let config_dir = match cmn::assure_config_dir_exists(opt.value_of("folder").unwrap_or("~/.google-service-cli")) {
                Err(e) => return Err(InvalidOptionsError::single(e, 3)),
                Ok(p) => p,
            };

            match cmn::application_secret_from_directory(&config_dir, "plus1-secret.json", 
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
                                          program_name: "plus1",
                                          db_dir: config_dir.clone(),
                                        }, None);

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
            hub: api::Plus::new(client, auth),
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
    let arg_data = [
        ("activities", "methods: 'get', 'list' and 'search'", vec![
            ("get",  
                    Some(r##"Get an activity."##),
                    "Details at http://byron.github.io/google-apis-rs/google_plus1_cli/activities_get",
                  vec![
                    (Some(r##"activity-id"##),
                     None,
                     Some(r##"The ID of the activity to get."##),
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
                    Some(r##"List all of the activities in the specified collection for a particular user."##),
                    "Details at http://byron.github.io/google-apis-rs/google_plus1_cli/activities_list",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The ID of the user to get activities for. The special value "me" can be used to indicate the authenticated user."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"collection"##),
                     None,
                     Some(r##"The collection of activities to list."##),
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
            ("search",  
                    Some(r##"Search public activities."##),
                    "Details at http://byron.github.io/google-apis-rs/google_plus1_cli/activities_search",
                  vec![
                    (Some(r##"query"##),
                     None,
                     Some(r##"Full-text search query string."##),
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
        
        ("comments", "methods: 'get' and 'list'", vec![
            ("get",  
                    Some(r##"Get a comment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_plus1_cli/comments_get",
                  vec![
                    (Some(r##"comment-id"##),
                     None,
                     Some(r##"The ID of the comment to get."##),
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
                    Some(r##"List all of the comments for an activity."##),
                    "Details at http://byron.github.io/google-apis-rs/google_plus1_cli/comments_list",
                  vec![
                    (Some(r##"activity-id"##),
                     None,
                     Some(r##"The ID of the activity to get comments for."##),
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
        
        ("moments", "methods: 'insert' and 'list'", vec![
            ("insert",  
                    Some(r##"Record a moment representing a user's action such as making a purchase or commenting on a blog."##),
                    "Details at http://byron.github.io/google-apis-rs/google_plus1_cli/moments_insert",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The ID of the user to record actions for. The only valid values are "me" and the ID of the authenticated user."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"collection"##),
                     None,
                     Some(r##"The collection to which to write moments."##),
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
                    Some(r##"List all of the moments for a particular user."##),
                    "Details at http://byron.github.io/google-apis-rs/google_plus1_cli/moments_list",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The ID of the user to get moments for. The special value "me" can be used to indicate the authenticated user."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"collection"##),
                     None,
                     Some(r##"The collection of moments to list."##),
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
        
        ("people", "methods: 'get', 'list', 'list-by-activity' and 'search'", vec![
            ("get",  
                    Some(r##"Get a person's profile. If your app uses scope https://www.googleapis.com/auth/plus.login, this method is guaranteed to return ageRange and language."##),
                    "Details at http://byron.github.io/google-apis-rs/google_plus1_cli/people_get",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The ID of the person to get the profile for. The special value "me" can be used to indicate the authenticated user."##),
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
                    Some(r##"List all of the people in the specified collection."##),
                    "Details at http://byron.github.io/google-apis-rs/google_plus1_cli/people_list",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"Get the collection of people for the person identified. Use "me" to indicate the authenticated user."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"collection"##),
                     None,
                     Some(r##"The collection of people to list."##),
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
            ("list-by-activity",  
                    Some(r##"List all of the people in the specified collection for a particular activity."##),
                    "Details at http://byron.github.io/google-apis-rs/google_plus1_cli/people_list-by-activity",
                  vec![
                    (Some(r##"activity-id"##),
                     None,
                     Some(r##"The ID of the activity to get the list of people for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"collection"##),
                     None,
                     Some(r##"The collection of people to list."##),
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
            ("search",  
                    Some(r##"Search all public profiles."##),
                    "Details at http://byron.github.io/google-apis-rs/google_plus1_cli/people_search",
                  vec![
                    (Some(r##"query"##),
                     None,
                     Some(r##"Specify a query string for full text search of public text in all profiles."##),
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
    
    let mut app = App::new("plus1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("0.3.2+20151014")
           .about("The Google+ API enables developers to build on top of the Google+ platform.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_plus1_cli")
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
           
           for &(main_command_name, ref about, ref subcommands) in arg_data.iter() {
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