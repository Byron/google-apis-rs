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
extern crate google_plusdomains1 as api;

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
    hub: api::PlusDomains<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
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

    fn _activities_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Activity::default();
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
            fn request_access_init(request: &mut api::Activity) {
                if request.access.is_none() {
                    request.access = Some(Default::default());
                }
            }
            
            fn request_actor_image_init(request: &mut api::Activity) {
                request_actor_init(request);
                if request.actor.as_mut().unwrap().image.is_none() {
                    request.actor.as_mut().unwrap().image = Some(Default::default());
                }
            }
            
            fn request_actor_init(request: &mut api::Activity) {
                if request.actor.is_none() {
                    request.actor = Some(Default::default());
                }
            }
            
            fn request_actor_name_init(request: &mut api::Activity) {
                request_actor_init(request);
                if request.actor.as_mut().unwrap().name.is_none() {
                    request.actor.as_mut().unwrap().name = Some(Default::default());
                }
            }
            
            fn request_location_address_init(request: &mut api::Activity) {
                request_location_init(request);
                if request.location.as_mut().unwrap().address.is_none() {
                    request.location.as_mut().unwrap().address = Some(Default::default());
                }
            }
            
            fn request_location_init(request: &mut api::Activity) {
                if request.location.is_none() {
                    request.location = Some(Default::default());
                }
            }
            
            fn request_location_position_init(request: &mut api::Activity) {
                request_location_init(request);
                if request.location.as_mut().unwrap().position.is_none() {
                    request.location.as_mut().unwrap().position = Some(Default::default());
                }
            }
            
            fn request_object_actor_image_init(request: &mut api::Activity) {
                request_object_actor_init(request);
                if request.object.as_mut().unwrap().actor.as_mut().unwrap().image.is_none() {
                    request.object.as_mut().unwrap().actor.as_mut().unwrap().image = Some(Default::default());
                }
            }
            
            fn request_object_actor_init(request: &mut api::Activity) {
                request_object_init(request);
                if request.object.as_mut().unwrap().actor.is_none() {
                    request.object.as_mut().unwrap().actor = Some(Default::default());
                }
            }
            
            fn request_object_init(request: &mut api::Activity) {
                if request.object.is_none() {
                    request.object = Some(Default::default());
                }
            }
            
            fn request_object_plusoners_init(request: &mut api::Activity) {
                request_object_init(request);
                if request.object.as_mut().unwrap().plusoners.is_none() {
                    request.object.as_mut().unwrap().plusoners = Some(Default::default());
                }
            }
            
            fn request_object_replies_init(request: &mut api::Activity) {
                request_object_init(request);
                if request.object.as_mut().unwrap().replies.is_none() {
                    request.object.as_mut().unwrap().replies = Some(Default::default());
                }
            }
            
            fn request_object_resharers_init(request: &mut api::Activity) {
                request_object_init(request);
                if request.object.as_mut().unwrap().resharers.is_none() {
                    request.object.as_mut().unwrap().resharers = Some(Default::default());
                }
            }
            
            fn request_object_status_for_viewer_init(request: &mut api::Activity) {
                request_object_init(request);
                if request.object.as_mut().unwrap().status_for_viewer.is_none() {
                    request.object.as_mut().unwrap().status_for_viewer = Some(Default::default());
                }
            }
            
            fn request_provider_init(request: &mut api::Activity) {
                if request.provider.is_none() {
                    request.provider = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "place-name" => {
                        request.place_name = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "provider.title" => {
                        request_provider_init(&mut request);
                        request.provider.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "title" => {
                        request_provider_init(&mut request);
                        request.title = Some(value.unwrap_or("").to_string());
                    },
                "url" => {
                        request_provider_init(&mut request);
                        request.url = Some(value.unwrap_or("").to_string());
                    },
                "geocode" => {
                        request_provider_init(&mut request);
                        request.geocode = Some(value.unwrap_or("").to_string());
                    },
                "object.resharers.total-items" => {
                        request_object_resharers_init(&mut request);
                        request.object.as_mut().unwrap().resharers.as_mut().unwrap().total_items = Some(arg_from_str(value.unwrap_or("-0"), err, "object.resharers.total-items", "integer"));
                    },
                "object.resharers.self-link" => {
                        request_object_resharers_init(&mut request);
                        request.object.as_mut().unwrap().resharers.as_mut().unwrap().self_link = Some(value.unwrap_or("").to_string());
                    },
                "object.original-content" => {
                        request_object_resharers_init(&mut request);
                        request.object.as_mut().unwrap().original_content = Some(value.unwrap_or("").to_string());
                    },
                "object.plusoners.total-items" => {
                        request_object_plusoners_init(&mut request);
                        request.object.as_mut().unwrap().plusoners.as_mut().unwrap().total_items = Some(arg_from_str(value.unwrap_or("-0"), err, "object.plusoners.total-items", "integer"));
                    },
                "object.plusoners.self-link" => {
                        request_object_plusoners_init(&mut request);
                        request.object.as_mut().unwrap().plusoners.as_mut().unwrap().self_link = Some(value.unwrap_or("").to_string());
                    },
                "object.actor.url" => {
                        request_object_actor_init(&mut request);
                        request.object.as_mut().unwrap().actor.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "object.actor.image.url" => {
                        request_object_actor_image_init(&mut request);
                        request.object.as_mut().unwrap().actor.as_mut().unwrap().image.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "object.actor.display-name" => {
                        request_object_actor_image_init(&mut request);
                        request.object.as_mut().unwrap().actor.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "object.actor.id" => {
                        request_object_actor_image_init(&mut request);
                        request.object.as_mut().unwrap().actor.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "object.content" => {
                        request_object_actor_init(&mut request);
                        request.object.as_mut().unwrap().content = Some(value.unwrap_or("").to_string());
                    },
                "object.url" => {
                        request_object_actor_init(&mut request);
                        request.object.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "object.status-for-viewer.can-plusone" => {
                        request_object_status_for_viewer_init(&mut request);
                        request.object.as_mut().unwrap().status_for_viewer.as_mut().unwrap().can_plusone = Some(arg_from_str(value.unwrap_or("false"), err, "object.status-for-viewer.can-plusone", "boolean"));
                    },
                "object.status-for-viewer.can-update" => {
                        request_object_status_for_viewer_init(&mut request);
                        request.object.as_mut().unwrap().status_for_viewer.as_mut().unwrap().can_update = Some(arg_from_str(value.unwrap_or("false"), err, "object.status-for-viewer.can-update", "boolean"));
                    },
                "object.status-for-viewer.is-plus-oned" => {
                        request_object_status_for_viewer_init(&mut request);
                        request.object.as_mut().unwrap().status_for_viewer.as_mut().unwrap().is_plus_oned = Some(arg_from_str(value.unwrap_or("false"), err, "object.status-for-viewer.is-plus-oned", "boolean"));
                    },
                "object.status-for-viewer.resharing-disabled" => {
                        request_object_status_for_viewer_init(&mut request);
                        request.object.as_mut().unwrap().status_for_viewer.as_mut().unwrap().resharing_disabled = Some(arg_from_str(value.unwrap_or("false"), err, "object.status-for-viewer.resharing-disabled", "boolean"));
                    },
                "object.status-for-viewer.can-comment" => {
                        request_object_status_for_viewer_init(&mut request);
                        request.object.as_mut().unwrap().status_for_viewer.as_mut().unwrap().can_comment = Some(arg_from_str(value.unwrap_or("false"), err, "object.status-for-viewer.can-comment", "boolean"));
                    },
                "object.replies.total-items" => {
                        request_object_replies_init(&mut request);
                        request.object.as_mut().unwrap().replies.as_mut().unwrap().total_items = Some(arg_from_str(value.unwrap_or("-0"), err, "object.replies.total-items", "integer"));
                    },
                "object.replies.self-link" => {
                        request_object_replies_init(&mut request);
                        request.object.as_mut().unwrap().replies.as_mut().unwrap().self_link = Some(value.unwrap_or("").to_string());
                    },
                "object.id" => {
                        request_object_replies_init(&mut request);
                        request.object.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "object.object-type" => {
                        request_object_replies_init(&mut request);
                        request.object.as_mut().unwrap().object_type = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_object_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "actor.url" => {
                        request_actor_init(&mut request);
                        request.actor.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "actor.image.url" => {
                        request_actor_image_init(&mut request);
                        request.actor.as_mut().unwrap().image.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "actor.display-name" => {
                        request_actor_image_init(&mut request);
                        request.actor.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "actor.id" => {
                        request_actor_image_init(&mut request);
                        request.actor.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "actor.name.given-name" => {
                        request_actor_name_init(&mut request);
                        request.actor.as_mut().unwrap().name.as_mut().unwrap().given_name = Some(value.unwrap_or("").to_string());
                    },
                "actor.name.family-name" => {
                        request_actor_name_init(&mut request);
                        request.actor.as_mut().unwrap().name.as_mut().unwrap().family_name = Some(value.unwrap_or("").to_string());
                    },
                "place-id" => {
                        request_actor_init(&mut request);
                        request.place_id = Some(value.unwrap_or("").to_string());
                    },
                "access.kind" => {
                        request_access_init(&mut request);
                        request.access.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "access.description" => {
                        request_access_init(&mut request);
                        request.access.as_mut().unwrap().description = Some(value.unwrap_or("").to_string());
                    },
                "access.domain-restricted" => {
                        request_access_init(&mut request);
                        request.access.as_mut().unwrap().domain_restricted = Some(arg_from_str(value.unwrap_or("false"), err, "access.domain-restricted", "boolean"));
                    },
                "etag" => {
                        request_access_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "radius" => {
                        request_access_init(&mut request);
                        request.radius = Some(value.unwrap_or("").to_string());
                    },
                "location.position.latitude" => {
                        request_location_position_init(&mut request);
                        request.location.as_mut().unwrap().position.as_mut().unwrap().latitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "location.position.latitude", "number"));
                    },
                "location.position.longitude" => {
                        request_location_position_init(&mut request);
                        request.location.as_mut().unwrap().position.as_mut().unwrap().longitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "location.position.longitude", "number"));
                    },
                "location.kind" => {
                        request_location_position_init(&mut request);
                        request.location.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "location.display-name" => {
                        request_location_position_init(&mut request);
                        request.location.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "location.id" => {
                        request_location_position_init(&mut request);
                        request.location.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "location.address.formatted" => {
                        request_location_address_init(&mut request);
                        request.location.as_mut().unwrap().address.as_mut().unwrap().formatted = Some(value.unwrap_or("").to_string());
                    },
                "verb" => {
                        request_location_init(&mut request);
                        request.verb = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_location_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "crosspost-source" => {
                        request_location_init(&mut request);
                        request.crosspost_source = Some(value.unwrap_or("").to_string());
                    },
                "annotation" => {
                        request_location_init(&mut request);
                        request.annotation = Some(value.unwrap_or("").to_string());
                    },
                "address" => {
                        request_location_init(&mut request);
                        request.address = Some(value.unwrap_or("").to_string());
                    },
                "published" => {
                        request_location_init(&mut request);
                        request.published = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["access", "actor", "address", "annotation", "can-comment", "can-plusone", "can-update", "content", "crosspost-source", "description", "display-name", "domain-restricted", "etag", "family-name", "formatted", "geocode", "given-name", "id", "image", "is-plus-oned", "kind", "latitude", "location", "longitude", "name", "object", "object-type", "original-content", "place-id", "place-name", "plusoners", "position", "provider", "published", "radius", "replies", "resharers", "resharing-disabled", "self-link", "status-for-viewer", "title", "total-items", "updated", "url", "verb"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.activities().insert(request, opt.value_of("user-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "preview" => {
                    call = call.preview(arg_from_str(value.unwrap_or("false"), err, "preview", "boolean"));
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
                                                Vec::new() + &self.gp + &["preview"]
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

    fn _audiences_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.audiences().list(opt.value_of("user-id").unwrap_or(""));
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

    fn _circles_add_people(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.circles().add_people(opt.value_of("circle-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-id" => {
                    call = call.add_user_id(value.unwrap_or(""));
                },
                "email" => {
                    call = call.add_email(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["user-id", "email"]
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

    fn _circles_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.circles().get(opt.value_of("circle-id").unwrap_or(""));
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

    fn _circles_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Circle::default();
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
            fn request_people_init(request: &mut api::Circle) {
                if request.people.is_none() {
                    request.people = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "display-name" => {
                        request.display_name = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "people.total-items" => {
                        request_people_init(&mut request);
                        request.people.as_mut().unwrap().total_items = Some(arg_from_str(value.unwrap_or("-0"), err, "people.total-items", "integer"));
                    },
                "etag" => {
                        request_people_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_people_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_people_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["description", "display-name", "etag", "id", "kind", "people", "self-link", "total-items"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.circles().insert(request, opt.value_of("user-id").unwrap_or(""));
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

    fn _circles_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.circles().list(opt.value_of("user-id").unwrap_or(""));
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

    fn _circles_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Circle::default();
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
            fn request_people_init(request: &mut api::Circle) {
                if request.people.is_none() {
                    request.people = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "display-name" => {
                        request.display_name = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "people.total-items" => {
                        request_people_init(&mut request);
                        request.people.as_mut().unwrap().total_items = Some(arg_from_str(value.unwrap_or("-0"), err, "people.total-items", "integer"));
                    },
                "etag" => {
                        request_people_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_people_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_people_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["description", "display-name", "etag", "id", "kind", "people", "self-link", "total-items"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.circles().patch(request, opt.value_of("circle-id").unwrap_or(""));
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

    fn _circles_remove(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.circles().remove(opt.value_of("circle-id").unwrap_or(""));
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

    fn _circles_remove_people(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.circles().remove_people(opt.value_of("circle-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-id" => {
                    call = call.add_user_id(value.unwrap_or(""));
                },
                "email" => {
                    call = call.add_email(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["user-id", "email"]
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

    fn _circles_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Circle::default();
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
            fn request_people_init(request: &mut api::Circle) {
                if request.people.is_none() {
                    request.people = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "display-name" => {
                        request.display_name = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "people.total-items" => {
                        request_people_init(&mut request);
                        request.people.as_mut().unwrap().total_items = Some(arg_from_str(value.unwrap_or("-0"), err, "people.total-items", "integer"));
                    },
                "etag" => {
                        request_people_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_people_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_people_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["description", "display-name", "etag", "id", "kind", "people", "self-link", "total-items"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.circles().update(request, opt.value_of("circle-id").unwrap_or(""));
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

    fn _comments_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Comment::default();
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
            fn request_actor_image_init(request: &mut api::Comment) {
                request_actor_init(request);
                if request.actor.as_mut().unwrap().image.is_none() {
                    request.actor.as_mut().unwrap().image = Some(Default::default());
                }
            }
            
            fn request_actor_init(request: &mut api::Comment) {
                if request.actor.is_none() {
                    request.actor = Some(Default::default());
                }
            }
            
            fn request_object_init(request: &mut api::Comment) {
                if request.object.is_none() {
                    request.object = Some(Default::default());
                }
            }
            
            fn request_plusoners_init(request: &mut api::Comment) {
                if request.plusoners.is_none() {
                    request.plusoners = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "plusoners.total-items" => {
                        request_plusoners_init(&mut request);
                        request.plusoners.as_mut().unwrap().total_items = Some(arg_from_str(value.unwrap_or("-0"), err, "plusoners.total-items", "integer"));
                    },
                "object.content" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().content = Some(value.unwrap_or("").to_string());
                    },
                "object.original-content" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().original_content = Some(value.unwrap_or("").to_string());
                    },
                "object.object-type" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().object_type = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request_object_init(&mut request);
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "actor.url" => {
                        request_actor_init(&mut request);
                        request.actor.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "actor.image.url" => {
                        request_actor_image_init(&mut request);
                        request.actor.as_mut().unwrap().image.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "actor.display-name" => {
                        request_actor_image_init(&mut request);
                        request.actor.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "actor.id" => {
                        request_actor_image_init(&mut request);
                        request.actor.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "verb" => {
                        request_actor_init(&mut request);
                        request.verb = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_actor_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "published" => {
                        request_actor_init(&mut request);
                        request.published = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_actor_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_actor_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["actor", "content", "display-name", "etag", "id", "image", "kind", "object", "object-type", "original-content", "plusoners", "published", "self-link", "total-items", "updated", "url", "verb"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.comments().insert(request, opt.value_of("activity-id").unwrap_or(""));
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

    fn _media_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Media::default();
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
            fn request_author_image_init(request: &mut api::Media) {
                request_author_init(request);
                if request.author.as_mut().unwrap().image.is_none() {
                    request.author.as_mut().unwrap().image = Some(Default::default());
                }
            }
            
            fn request_author_init(request: &mut api::Media) {
                if request.author.is_none() {
                    request.author = Some(Default::default());
                }
            }
            
            fn request_exif_init(request: &mut api::Media) {
                if request.exif.is_none() {
                    request.exif = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "display-name" => {
                        request.display_name = Some(value.unwrap_or("").to_string());
                    },
                "exif.time" => {
                        request_exif_init(&mut request);
                        request.exif.as_mut().unwrap().time = Some(value.unwrap_or("").to_string());
                    },
                "author.url" => {
                        request_author_init(&mut request);
                        request.author.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "author.image.url" => {
                        request_author_image_init(&mut request);
                        request.author.as_mut().unwrap().image.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "author.display-name" => {
                        request_author_image_init(&mut request);
                        request.author.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "author.id" => {
                        request_author_image_init(&mut request);
                        request.author.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "url" => {
                        request_author_init(&mut request);
                        request.url = Some(value.unwrap_or("").to_string());
                    },
                "media-url" => {
                        request_author_init(&mut request);
                        request.media_url = Some(value.unwrap_or("").to_string());
                    },
                "video-status" => {
                        request_author_init(&mut request);
                        request.video_status = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_author_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "height" => {
                        request_author_init(&mut request);
                        request.height = Some(arg_from_str(value.unwrap_or("-0"), err, "height", "integer"));
                    },
                "video-duration" => {
                        request_author_init(&mut request);
                        request.video_duration = Some(value.unwrap_or("").to_string());
                    },
                "size-bytes" => {
                        request_author_init(&mut request);
                        request.size_bytes = Some(value.unwrap_or("").to_string());
                    },
                "width" => {
                        request_author_init(&mut request);
                        request.width = Some(arg_from_str(value.unwrap_or("-0"), err, "width", "integer"));
                    },
                "summary" => {
                        request_author_init(&mut request);
                        request.summary = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_author_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "published" => {
                        request_author_init(&mut request);
                        request.published = Some(value.unwrap_or("").to_string());
                    },
                "media-created-time" => {
                        request_author_init(&mut request);
                        request.media_created_time = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_author_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["author", "display-name", "etag", "exif", "height", "id", "image", "kind", "media-created-time", "media-url", "published", "size-bytes", "summary", "time", "updated", "url", "video-duration", "video-status", "width"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.media().insert(request, opt.value_of("user-id").unwrap_or(""), opt.value_of("collection").unwrap_or(""));
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
        let vals = opt.values_of("mode").unwrap();
        let protocol = calltype_from_str(vals[0], ["simple", "resumable"].iter().map(|&v| v.to_string()).collect(), err);
        let mut input_file = input_file_from_opts(vals[1], err);
        let mime_type = input_mime_from_opts(opt.value_of("mime").unwrap_or("application/octet-stream"), err);
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
                CallType::Upload(UploadProtocol::Simple) => call.upload(input_file.unwrap(), mime_type.unwrap()),
                CallType::Upload(UploadProtocol::Resumable) => call.upload_resumable(input_file.unwrap(), mime_type.unwrap()),
                CallType::Standard => unreachable!()
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

    fn _people_list_by_circle(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.people().list_by_circle(opt.value_of("circle-id").unwrap_or(""));
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
                    ("insert", Some(opt)) => {
                        call_result = self._activities_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._activities_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("activities".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("audiences", Some(opt)) => {
                match opt.subcommand() {
                    ("list", Some(opt)) => {
                        call_result = self._audiences_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("audiences".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("circles", Some(opt)) => {
                match opt.subcommand() {
                    ("add-people", Some(opt)) => {
                        call_result = self._circles_add_people(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._circles_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._circles_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._circles_list(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._circles_patch(opt, dry_run, &mut err);
                    },
                    ("remove", Some(opt)) => {
                        call_result = self._circles_remove(opt, dry_run, &mut err);
                    },
                    ("remove-people", Some(opt)) => {
                        call_result = self._circles_remove_people(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._circles_update(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("circles".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("comments", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._comments_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._comments_insert(opt, dry_run, &mut err);
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
            ("media", Some(opt)) => {
                match opt.subcommand() {
                    ("insert", Some(opt)) => {
                        call_result = self._media_insert(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("media".to_string()));
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
                    ("list-by-circle", Some(opt)) => {
                        call_result = self._people_list_by_circle(opt, dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "plusdomains1-secret.json", 
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
                                          program_name: "plusdomains1",
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
            hub: api::PlusDomains::new(client, auth),
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
    let upload_value_names = ["mode", "file"];
    let arg_data = [
        ("activities", "methods: 'get', 'insert' and 'list'", vec![
            ("get",  
                    Some(r##"Get an activity."##),
                    "Details at http://byron.github.io/google-apis-rs/google_plusdomains1_cli/activities_get",
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
            ("insert",  
                    Some(r##"Create a new activity for the authenticated user."##),
                    "Details at http://byron.github.io/google-apis-rs/google_plusdomains1_cli/activities_insert",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The ID of the user to create the activity on behalf of. Its value should be "me", to indicate the authenticated user."##),
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
                    Some(r##"List all of the activities in the specified collection for a particular user."##),
                    "Details at http://byron.github.io/google-apis-rs/google_plusdomains1_cli/activities_list",
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
            ]),
        
        ("audiences", "methods: 'list'", vec![
            ("list",  
                    Some(r##"List all of the audiences to which a user can share."##),
                    "Details at http://byron.github.io/google-apis-rs/google_plusdomains1_cli/audiences_list",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The ID of the user to get audiences for. The special value "me" can be used to indicate the authenticated user."##),
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
        
        ("circles", "methods: 'add-people', 'get', 'insert', 'list', 'patch', 'remove', 'remove-people' and 'update'", vec![
            ("add-people",  
                    Some(r##"Add a person to a circle. Google+ limits certain circle operations, including the number of circle adds. Learn More."##),
                    "Details at http://byron.github.io/google-apis-rs/google_plusdomains1_cli/circles_add-people",
                  vec![
                    (Some(r##"circle-id"##),
                     None,
                     Some(r##"The ID of the circle to add the person to."##),
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
            ("get",  
                    Some(r##"Get a circle."##),
                    "Details at http://byron.github.io/google-apis-rs/google_plusdomains1_cli/circles_get",
                  vec![
                    (Some(r##"circle-id"##),
                     None,
                     Some(r##"The ID of the circle to get."##),
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
                    Some(r##"Create a new circle for the authenticated user."##),
                    "Details at http://byron.github.io/google-apis-rs/google_plusdomains1_cli/circles_insert",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The ID of the user to create the circle on behalf of. The value "me" can be used to indicate the authenticated user."##),
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
                    Some(r##"List all of the circles for a user."##),
                    "Details at http://byron.github.io/google-apis-rs/google_plusdomains1_cli/circles_list",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The ID of the user to get circles for. The special value "me" can be used to indicate the authenticated user."##),
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
                    Some(r##"Update a circle's description. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_plusdomains1_cli/circles_patch",
                  vec![
                    (Some(r##"circle-id"##),
                     None,
                     Some(r##"The ID of the circle to update."##),
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
            ("remove",  
                    Some(r##"Delete a circle."##),
                    "Details at http://byron.github.io/google-apis-rs/google_plusdomains1_cli/circles_remove",
                  vec![
                    (Some(r##"circle-id"##),
                     None,
                     Some(r##"The ID of the circle to delete."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("remove-people",  
                    Some(r##"Remove a person from a circle."##),
                    "Details at http://byron.github.io/google-apis-rs/google_plusdomains1_cli/circles_remove-people",
                  vec![
                    (Some(r##"circle-id"##),
                     None,
                     Some(r##"The ID of the circle to remove the person from."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("update",  
                    Some(r##"Update a circle's description."##),
                    "Details at http://byron.github.io/google-apis-rs/google_plusdomains1_cli/circles_update",
                  vec![
                    (Some(r##"circle-id"##),
                     None,
                     Some(r##"The ID of the circle to update."##),
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
        
        ("comments", "methods: 'get', 'insert' and 'list'", vec![
            ("get",  
                    Some(r##"Get a comment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_plusdomains1_cli/comments_get",
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
            ("insert",  
                    Some(r##"Create a new comment in reply to an activity."##),
                    "Details at http://byron.github.io/google-apis-rs/google_plusdomains1_cli/comments_insert",
                  vec![
                    (Some(r##"activity-id"##),
                     None,
                     Some(r##"The ID of the activity to reply to."##),
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
                    Some(r##"List all of the comments for an activity."##),
                    "Details at http://byron.github.io/google-apis-rs/google_plusdomains1_cli/comments_list",
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
        
        ("media", "methods: 'insert'", vec![
            ("insert",  
                    Some(r##"Add a new media item to an album. The current upload size limitations are 36MB for a photo and 1GB for a video. Uploads do not count against quota if photos are less than 2048 pixels on their longest side or videos are less than 15 minutes in length."##),
                    "Details at http://byron.github.io/google-apis-rs/google_plusdomains1_cli/media_insert",
                  vec![
                    (Some(r##"user-id"##),
                     None,
                     Some(r##"The ID of the user to create the activity on behalf of."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"collection"##),
                     None,
                     None,
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"mode"##),
                     Some(r##"u"##),
                     Some(r##"Specify the upload protocol (simple|resumable) and the file to upload"##),
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
        
        ("people", "methods: 'get', 'list', 'list-by-activity' and 'list-by-circle'", vec![
            ("get",  
                    Some(r##"Get a person's profile."##),
                    "Details at http://byron.github.io/google-apis-rs/google_plusdomains1_cli/people_get",
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
                    "Details at http://byron.github.io/google-apis-rs/google_plusdomains1_cli/people_list",
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
                    "Details at http://byron.github.io/google-apis-rs/google_plusdomains1_cli/people_list-by-activity",
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
            ("list-by-circle",  
                    Some(r##"List all of the people who are members of a circle."##),
                    "Details at http://byron.github.io/google-apis-rs/google_plusdomains1_cli/people_list-by-circle",
                  vec![
                    (Some(r##"circle-id"##),
                     None,
                     Some(r##"The ID of the circle to get the members of."##),
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
    
    let mut app = App::new("plusdomains1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("0.2.0+20150303")
           .about("The Google+ API enables developers to build on top of the Google+ platform.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_plusdomains1_cli")
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