// DO NOT EDIT !
// This file was generated automatically from 'src/mako/cli/main.rs.mako'
// DO NOT EDIT !
#![feature(plugin, exit_status)]
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;
extern crate yup_oauth2 as oauth2;
extern crate yup_hyper_mock as mock;
extern crate serde;
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
          calltype_from_str, remove_json_null_values};

use std::default::Default;
use std::str::FromStr;

use oauth2::{Authenticator, DefaultAuthenticatorDelegate};
use serde::json;
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
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
                                                Vec::new() + &self.gp + &["page-token", "max-results"]
                                                            ));
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
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
                                                Vec::new() + &self.gp + &["order-by", "page-token", "language", "max-results"]
                                                            ));
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
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
                                                Vec::new() + &self.gp + &["page-token", "sort-order", "max-results"]
                                                            ));
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _moments_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Moment::default();
        let mut field_cursor = FieldCursor::default();
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
            fn request_object_init(request: &mut api::Moment) {
                if request.object.is_none() {
                    request.object = Some(Default::default());
                }
            }
            
            fn request_result_init(request: &mut api::Moment) {
                if request.result.is_none() {
                    request.result = Some(Default::default());
                }
            }
            
            fn request_target_init(request: &mut api::Moment) {
                if request.target.is_none() {
                    request.target = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "start-date" => {
                        request.start_date = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "target.start-date" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().start_date = Some(value.unwrap_or("").to_string());
                    },
                "target.end-date" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().end_date = Some(value.unwrap_or("").to_string());
                    },
                "target.text" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().text = Some(value.unwrap_or("").to_string());
                    },
                "target.image" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().image = Some(value.unwrap_or("").to_string());
                    },
                "target.birth-date" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().birth_date = Some(value.unwrap_or("").to_string());
                    },
                "target.date-published" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().date_published = Some(value.unwrap_or("").to_string());
                    },
                "target.address-locality" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().address_locality = Some(value.unwrap_or("").to_string());
                    },
                "target.additional-name" => {
                        request_target_init(&mut request);
                        if request.target.as_mut().unwrap().additional_name.is_none() {
                           request.target.as_mut().unwrap().additional_name = Some(Default::default());
                        }
                                        request.target.as_mut().unwrap().additional_name.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "target.worst-rating" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().worst_rating = Some(value.unwrap_or("").to_string());
                    },
                "target.duration" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().duration = Some(value.unwrap_or("").to_string());
                    },
                "target.thumbnail-url" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().thumbnail_url = Some(value.unwrap_or("").to_string());
                    },
                "target.id" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "target.post-office-box-number" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().post_office_box_number = Some(value.unwrap_or("").to_string());
                    },
                "target.caption" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().caption = Some(value.unwrap_or("").to_string());
                    },
                "target.best-rating" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().best_rating = Some(value.unwrap_or("").to_string());
                    },
                "target.address-country" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().address_country = Some(arg_from_str(value.unwrap_or("-0"), err, "target.address-country", "int64"));
                    },
                "target.width" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().width = Some(value.unwrap_or("").to_string());
                    },
                "target.street-address" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().street_address = Some(value.unwrap_or("").to_string());
                    },
                "target.latitude" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().latitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "target.latitude", "number"));
                    },
                "target.embed-url" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().embed_url = Some(value.unwrap_or("").to_string());
                    },
                "target.type" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "target.date-modified" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().date_modified = Some(value.unwrap_or("").to_string());
                    },
                "target.content-size" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().content_size = Some(value.unwrap_or("").to_string());
                    },
                "target.content-url" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().content_url = Some(value.unwrap_or("").to_string());
                    },
                "target.description" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().description = Some(value.unwrap_or("").to_string());
                    },
                "target.family-name" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().family_name = Some(value.unwrap_or("").to_string());
                    },
                "target.date-created" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().date_created = Some(value.unwrap_or("").to_string());
                    },
                "target.postal-code" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().postal_code = Some(value.unwrap_or("").to_string());
                    },
                "target.attendee-count" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().attendee_count = Some(arg_from_str(value.unwrap_or("-0"), err, "target.attendee-count", "integer"));
                    },
                "target.height" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().height = Some(value.unwrap_or("").to_string());
                    },
                "target.ticker-symbol" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().ticker_symbol = Some(value.unwrap_or("").to_string());
                    },
                "target.player-type" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().player_type = Some(value.unwrap_or("").to_string());
                    },
                "target.kind" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "target.name" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "target.url" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "target.gender" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().gender = Some(value.unwrap_or("").to_string());
                    },
                "target.longitude" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().longitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "target.longitude", "number"));
                    },
                "target.address-region" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().address_region = Some(value.unwrap_or("").to_string());
                    },
                "target.rating-value" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().rating_value = Some(value.unwrap_or("").to_string());
                    },
                "target.given-name" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().given_name = Some(value.unwrap_or("").to_string());
                    },
                "object.start-date" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().start_date = Some(value.unwrap_or("").to_string());
                    },
                "object.end-date" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().end_date = Some(value.unwrap_or("").to_string());
                    },
                "object.text" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().text = Some(value.unwrap_or("").to_string());
                    },
                "object.image" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().image = Some(value.unwrap_or("").to_string());
                    },
                "object.birth-date" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().birth_date = Some(value.unwrap_or("").to_string());
                    },
                "object.date-published" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().date_published = Some(value.unwrap_or("").to_string());
                    },
                "object.address-locality" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().address_locality = Some(value.unwrap_or("").to_string());
                    },
                "object.additional-name" => {
                        request_object_init(&mut request);
                        if request.object.as_mut().unwrap().additional_name.is_none() {
                           request.object.as_mut().unwrap().additional_name = Some(Default::default());
                        }
                                        request.object.as_mut().unwrap().additional_name.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "object.worst-rating" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().worst_rating = Some(value.unwrap_or("").to_string());
                    },
                "object.duration" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().duration = Some(value.unwrap_or("").to_string());
                    },
                "object.thumbnail-url" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().thumbnail_url = Some(value.unwrap_or("").to_string());
                    },
                "object.id" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "object.post-office-box-number" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().post_office_box_number = Some(value.unwrap_or("").to_string());
                    },
                "object.caption" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().caption = Some(value.unwrap_or("").to_string());
                    },
                "object.best-rating" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().best_rating = Some(value.unwrap_or("").to_string());
                    },
                "object.address-country" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().address_country = Some(arg_from_str(value.unwrap_or("-0"), err, "object.address-country", "int64"));
                    },
                "object.width" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().width = Some(value.unwrap_or("").to_string());
                    },
                "object.street-address" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().street_address = Some(value.unwrap_or("").to_string());
                    },
                "object.latitude" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().latitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "object.latitude", "number"));
                    },
                "object.embed-url" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().embed_url = Some(value.unwrap_or("").to_string());
                    },
                "object.type" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "object.date-modified" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().date_modified = Some(value.unwrap_or("").to_string());
                    },
                "object.content-size" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().content_size = Some(value.unwrap_or("").to_string());
                    },
                "object.content-url" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().content_url = Some(value.unwrap_or("").to_string());
                    },
                "object.description" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().description = Some(value.unwrap_or("").to_string());
                    },
                "object.family-name" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().family_name = Some(value.unwrap_or("").to_string());
                    },
                "object.date-created" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().date_created = Some(value.unwrap_or("").to_string());
                    },
                "object.postal-code" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().postal_code = Some(value.unwrap_or("").to_string());
                    },
                "object.attendee-count" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().attendee_count = Some(arg_from_str(value.unwrap_or("-0"), err, "object.attendee-count", "integer"));
                    },
                "object.height" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().height = Some(value.unwrap_or("").to_string());
                    },
                "object.ticker-symbol" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().ticker_symbol = Some(value.unwrap_or("").to_string());
                    },
                "object.player-type" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().player_type = Some(value.unwrap_or("").to_string());
                    },
                "object.kind" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "object.name" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "object.url" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "object.gender" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().gender = Some(value.unwrap_or("").to_string());
                    },
                "object.longitude" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().longitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "object.longitude", "number"));
                    },
                "object.address-region" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().address_region = Some(value.unwrap_or("").to_string());
                    },
                "object.rating-value" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().rating_value = Some(value.unwrap_or("").to_string());
                    },
                "object.given-name" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().given_name = Some(value.unwrap_or("").to_string());
                    },
                "result.start-date" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().start_date = Some(value.unwrap_or("").to_string());
                    },
                "result.end-date" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().end_date = Some(value.unwrap_or("").to_string());
                    },
                "result.text" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().text = Some(value.unwrap_or("").to_string());
                    },
                "result.image" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().image = Some(value.unwrap_or("").to_string());
                    },
                "result.birth-date" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().birth_date = Some(value.unwrap_or("").to_string());
                    },
                "result.date-published" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().date_published = Some(value.unwrap_or("").to_string());
                    },
                "result.address-locality" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().address_locality = Some(value.unwrap_or("").to_string());
                    },
                "result.additional-name" => {
                        request_result_init(&mut request);
                        if request.result.as_mut().unwrap().additional_name.is_none() {
                           request.result.as_mut().unwrap().additional_name = Some(Default::default());
                        }
                                        request.result.as_mut().unwrap().additional_name.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "result.worst-rating" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().worst_rating = Some(value.unwrap_or("").to_string());
                    },
                "result.duration" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().duration = Some(value.unwrap_or("").to_string());
                    },
                "result.thumbnail-url" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().thumbnail_url = Some(value.unwrap_or("").to_string());
                    },
                "result.id" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "result.post-office-box-number" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().post_office_box_number = Some(value.unwrap_or("").to_string());
                    },
                "result.caption" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().caption = Some(value.unwrap_or("").to_string());
                    },
                "result.best-rating" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().best_rating = Some(value.unwrap_or("").to_string());
                    },
                "result.address-country" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().address_country = Some(arg_from_str(value.unwrap_or("-0"), err, "result.address-country", "int64"));
                    },
                "result.width" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().width = Some(value.unwrap_or("").to_string());
                    },
                "result.street-address" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().street_address = Some(value.unwrap_or("").to_string());
                    },
                "result.latitude" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().latitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "result.latitude", "number"));
                    },
                "result.embed-url" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().embed_url = Some(value.unwrap_or("").to_string());
                    },
                "result.type" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "result.date-modified" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().date_modified = Some(value.unwrap_or("").to_string());
                    },
                "result.content-size" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().content_size = Some(value.unwrap_or("").to_string());
                    },
                "result.content-url" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().content_url = Some(value.unwrap_or("").to_string());
                    },
                "result.description" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().description = Some(value.unwrap_or("").to_string());
                    },
                "result.family-name" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().family_name = Some(value.unwrap_or("").to_string());
                    },
                "result.date-created" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().date_created = Some(value.unwrap_or("").to_string());
                    },
                "result.postal-code" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().postal_code = Some(value.unwrap_or("").to_string());
                    },
                "result.attendee-count" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().attendee_count = Some(arg_from_str(value.unwrap_or("-0"), err, "result.attendee-count", "integer"));
                    },
                "result.height" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().height = Some(value.unwrap_or("").to_string());
                    },
                "result.ticker-symbol" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().ticker_symbol = Some(value.unwrap_or("").to_string());
                    },
                "result.player-type" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().player_type = Some(value.unwrap_or("").to_string());
                    },
                "result.kind" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "result.name" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "result.url" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "result.gender" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().gender = Some(value.unwrap_or("").to_string());
                    },
                "result.longitude" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().longitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "result.longitude", "number"));
                    },
                "result.address-region" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().address_region = Some(value.unwrap_or("").to_string());
                    },
                "result.rating-value" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().rating_value = Some(value.unwrap_or("").to_string());
                    },
                "result.given-name" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().given_name = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_result_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_result_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["additional-name", "address-country", "address-locality", "address-region", "attendee-count", "best-rating", "birth-date", "caption", "content-size", "content-url", "date-created", "date-modified", "date-published", "description", "duration", "embed-url", "end-date", "family-name", "gender", "given-name", "height", "id", "image", "kind", "latitude", "longitude", "name", "object", "player-type", "post-office-box-number", "postal-code", "rating-value", "result", "start-date", "street-address", "target", "text", "thumbnail-url", "ticker-symbol", "type", "url", "width", "worst-rating"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
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
                                                Vec::new() + &self.gp + &["debug"]
                                                            ));
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
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
                                                Vec::new() + &self.gp + &["page-token", "type", "max-results", "target-url"]
                                                            ));
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _moments_remove(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.moments().remove(opt.value_of("id").unwrap_or(""));
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
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
                                                Vec::new() + &self.gp + &["order-by", "page-token", "max-results"]
                                                            ));
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
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
                                                Vec::new() + &self.gp + &["page-token", "max-results"]
                                                            ));
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
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
                                                Vec::new() + &self.gp + &["page-token", "language", "max-results"]
                                                            ));
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
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
                    ("remove", Some(opt)) => {
                        call_result = self._moments_remove(opt, dry_run, &mut err);
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
                                                    connector: hyper::net::HttpConnector(None) 
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
                        connector: hyper::net::HttpConnector(None) 
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
        
        ("moments", "methods: 'insert', 'list' and 'remove'", vec![
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
            ("remove",  
                    Some(r##"Delete a moment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_plus1_cli/moments_remove",
                  vec![
                    (Some(r##"id"##),
                     None,
                     Some(r##"The ID of the moment to delete."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
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
           .version("0.2.0+20150303")
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
               let mut mcmd = SubCommand::new(main_command_name).about(about);
           
               for &(sub_command_name, ref desc, url_info, ref args) in subcommands {
                   let mut scmd = SubCommand::new(sub_command_name);
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
                       let mut arg = Arg::with_name(arg_name_str);
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
            env::set_exit_status(err.exit_code);
            writeln!(io::stderr(), "{}", err).ok();
        },
        Ok(engine) => {
            if let Err(doit_err) = engine.doit() {
                env::set_exit_status(1);
                match doit_err {
                    DoitError::IoError(path, err) => {
                        writeln!(io::stderr(), "Failed to open output file '{}': {}", path, err).ok();
                    },
                    DoitError::ApiError(err) => {
                        if debug {
                            writeln!(io::stderr(), "{:?}", err).ok();
                        } else {
                            writeln!(io::stderr(), "{}", err).ok();
                        }
                    }
                }
            }
        }
    }
}