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
extern crate google_analytics3 as api;

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
    hub: api::Analytics<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n, 'a> Engine<'n, 'a> {
    fn _data_ga_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.data().ga_get(opt.value_of("ids").unwrap_or(""), opt.value_of("start-date").unwrap_or(""), opt.value_of("end-date").unwrap_or(""), opt.value_of("metrics").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
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
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
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
                                                Vec::new() + &self.gp + &["sort", "max-results", "dimensions", "start-index", "sampling-level", "filters", "output", "segment"]
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

    fn _data_mcf_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.data().mcf_get(opt.value_of("ids").unwrap_or(""), opt.value_of("start-date").unwrap_or(""), opt.value_of("end-date").unwrap_or(""), opt.value_of("metrics").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
                },
                "sort" => {
                    call = call.sort(value.unwrap_or(""));
                },
                "sampling-level" => {
                    call = call.sampling_level(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
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
                                                Vec::new() + &self.gp + &["sort", "max-results", "dimensions", "start-index", "sampling-level", "filters"]
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

    fn _data_realtime_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.data().realtime_get(opt.value_of("ids").unwrap_or(""), opt.value_of("metrics").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "sort" => {
                    call = call.sort(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
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
                                                Vec::new() + &self.gp + &["sort", "max-results", "dimensions", "filters"]
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

    fn _management_account_summaries_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().account_summaries_list();
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
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
                                                Vec::new() + &self.gp + &["start-index", "max-results"]
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

    fn _management_account_user_links_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().account_user_links_delete(opt.value_of("account-id").unwrap_or(""), opt.value_of("link-id").unwrap_or(""));
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

    fn _management_account_user_links_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::EntityUserLink::default();
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
            fn request_entity_account_ref_init(request: &mut api::EntityUserLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().account_ref.is_none() {
                    request.entity.as_mut().unwrap().account_ref = Some(Default::default());
                }
            }
            
            fn request_entity_init(request: &mut api::EntityUserLink) {
                if request.entity.is_none() {
                    request.entity = Some(Default::default());
                }
            }
            
            fn request_entity_profile_ref_init(request: &mut api::EntityUserLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().profile_ref.is_none() {
                    request.entity.as_mut().unwrap().profile_ref = Some(Default::default());
                }
            }
            
            fn request_entity_web_property_ref_init(request: &mut api::EntityUserLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().web_property_ref.is_none() {
                    request.entity.as_mut().unwrap().web_property_ref = Some(Default::default());
                }
            }
            
            fn request_permissions_init(request: &mut api::EntityUserLink) {
                if request.permissions.is_none() {
                    request.permissions = Some(Default::default());
                }
            }
            
            fn request_user_ref_init(request: &mut api::EntityUserLink) {
                if request.user_ref.is_none() {
                    request.user_ref = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.kind" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.href" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.id" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.name" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.kind" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.name" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.internal-web-property-id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.href" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.web-property-id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.account-id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.kind" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.name" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.internal-web-property-id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.href" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.account-id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "user-ref.kind" => {
                        request_user_ref_init(&mut request);
                        request.user_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "user-ref.email" => {
                        request_user_ref_init(&mut request);
                        request.user_ref.as_mut().unwrap().email = Some(value.unwrap_or("").to_string());
                    },
                "user-ref.id" => {
                        request_user_ref_init(&mut request);
                        request.user_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_user_ref_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_user_ref_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "permissions.local" => {
                        request_permissions_init(&mut request);
                        if request.permissions.as_mut().unwrap().local.is_none() {
                           request.permissions.as_mut().unwrap().local = Some(Default::default());
                        }
                                        request.permissions.as_mut().unwrap().local.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "permissions.effective" => {
                        request_permissions_init(&mut request);
                        if request.permissions.as_mut().unwrap().effective.is_none() {
                           request.permissions.as_mut().unwrap().effective = Some(Default::default());
                        }
                                        request.permissions.as_mut().unwrap().effective.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "account-ref", "effective", "email", "entity", "href", "id", "internal-web-property-id", "kind", "local", "name", "permissions", "profile-ref", "self-link", "user-ref", "web-property-id", "web-property-ref"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.management().account_user_links_insert(request, opt.value_of("account-id").unwrap_or(""));
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

    fn _management_account_user_links_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().account_user_links_list(opt.value_of("account-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
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
                                                Vec::new() + &self.gp + &["start-index", "max-results"]
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

    fn _management_account_user_links_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::EntityUserLink::default();
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
            fn request_entity_account_ref_init(request: &mut api::EntityUserLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().account_ref.is_none() {
                    request.entity.as_mut().unwrap().account_ref = Some(Default::default());
                }
            }
            
            fn request_entity_init(request: &mut api::EntityUserLink) {
                if request.entity.is_none() {
                    request.entity = Some(Default::default());
                }
            }
            
            fn request_entity_profile_ref_init(request: &mut api::EntityUserLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().profile_ref.is_none() {
                    request.entity.as_mut().unwrap().profile_ref = Some(Default::default());
                }
            }
            
            fn request_entity_web_property_ref_init(request: &mut api::EntityUserLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().web_property_ref.is_none() {
                    request.entity.as_mut().unwrap().web_property_ref = Some(Default::default());
                }
            }
            
            fn request_permissions_init(request: &mut api::EntityUserLink) {
                if request.permissions.is_none() {
                    request.permissions = Some(Default::default());
                }
            }
            
            fn request_user_ref_init(request: &mut api::EntityUserLink) {
                if request.user_ref.is_none() {
                    request.user_ref = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.kind" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.href" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.id" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.name" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.kind" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.name" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.internal-web-property-id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.href" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.web-property-id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.account-id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.kind" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.name" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.internal-web-property-id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.href" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.account-id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "user-ref.kind" => {
                        request_user_ref_init(&mut request);
                        request.user_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "user-ref.email" => {
                        request_user_ref_init(&mut request);
                        request.user_ref.as_mut().unwrap().email = Some(value.unwrap_or("").to_string());
                    },
                "user-ref.id" => {
                        request_user_ref_init(&mut request);
                        request.user_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_user_ref_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_user_ref_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "permissions.local" => {
                        request_permissions_init(&mut request);
                        if request.permissions.as_mut().unwrap().local.is_none() {
                           request.permissions.as_mut().unwrap().local = Some(Default::default());
                        }
                                        request.permissions.as_mut().unwrap().local.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "permissions.effective" => {
                        request_permissions_init(&mut request);
                        if request.permissions.as_mut().unwrap().effective.is_none() {
                           request.permissions.as_mut().unwrap().effective = Some(Default::default());
                        }
                                        request.permissions.as_mut().unwrap().effective.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "account-ref", "effective", "email", "entity", "href", "id", "internal-web-property-id", "kind", "local", "name", "permissions", "profile-ref", "self-link", "user-ref", "web-property-id", "web-property-ref"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.management().account_user_links_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("link-id").unwrap_or(""));
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

    fn _management_accounts_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().accounts_list();
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
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
                                                Vec::new() + &self.gp + &["start-index", "max-results"]
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

    fn _management_custom_data_sources_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().custom_data_sources_list(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
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
                                                Vec::new() + &self.gp + &["start-index", "max-results"]
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

    fn _management_custom_dimensions_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().custom_dimensions_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("custom-dimension-id").unwrap_or(""));
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

    fn _management_custom_dimensions_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::CustomDimension::default();
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
            fn request_parent_link_init(request: &mut api::CustomDimension) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "index" => {
                        request.index = Some(arg_from_str(value.unwrap_or("-0"), err, "index", "integer"));
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "web-property-id" => {
                        request.web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "active" => {
                        request.active = Some(arg_from_str(value.unwrap_or("false"), err, "active", "boolean"));
                    },
                "scope" => {
                        request.scope = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_parent_link_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_parent_link_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_parent_link_init(&mut request);
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "active", "created", "href", "id", "index", "kind", "name", "parent-link", "scope", "self-link", "type", "updated", "web-property-id"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.management().custom_dimensions_insert(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""));
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

    fn _management_custom_dimensions_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().custom_dimensions_list(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
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
                                                Vec::new() + &self.gp + &["start-index", "max-results"]
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

    fn _management_custom_dimensions_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::CustomDimension::default();
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
            fn request_parent_link_init(request: &mut api::CustomDimension) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "index" => {
                        request.index = Some(arg_from_str(value.unwrap_or("-0"), err, "index", "integer"));
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "web-property-id" => {
                        request.web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "active" => {
                        request.active = Some(arg_from_str(value.unwrap_or("false"), err, "active", "boolean"));
                    },
                "scope" => {
                        request.scope = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_parent_link_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_parent_link_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_parent_link_init(&mut request);
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "active", "created", "href", "id", "index", "kind", "name", "parent-link", "scope", "self-link", "type", "updated", "web-property-id"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.management().custom_dimensions_patch(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("custom-dimension-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "ignore-custom-data-source-links" => {
                    call = call.ignore_custom_data_source_links(arg_from_str(value.unwrap_or("false"), err, "ignore-custom-data-source-links", "boolean"));
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
                                                Vec::new() + &self.gp + &["ignore-custom-data-source-links"]
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

    fn _management_custom_dimensions_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::CustomDimension::default();
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
            fn request_parent_link_init(request: &mut api::CustomDimension) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "index" => {
                        request.index = Some(arg_from_str(value.unwrap_or("-0"), err, "index", "integer"));
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "web-property-id" => {
                        request.web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "active" => {
                        request.active = Some(arg_from_str(value.unwrap_or("false"), err, "active", "boolean"));
                    },
                "scope" => {
                        request.scope = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_parent_link_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_parent_link_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_parent_link_init(&mut request);
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "active", "created", "href", "id", "index", "kind", "name", "parent-link", "scope", "self-link", "type", "updated", "web-property-id"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.management().custom_dimensions_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("custom-dimension-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "ignore-custom-data-source-links" => {
                    call = call.ignore_custom_data_source_links(arg_from_str(value.unwrap_or("false"), err, "ignore-custom-data-source-links", "boolean"));
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
                                                Vec::new() + &self.gp + &["ignore-custom-data-source-links"]
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

    fn _management_custom_metrics_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().custom_metrics_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("custom-metric-id").unwrap_or(""));
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

    fn _management_custom_metrics_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::CustomMetric::default();
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
            fn request_parent_link_init(request: &mut api::CustomMetric) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "index" => {
                        request.index = Some(arg_from_str(value.unwrap_or("-0"), err, "index", "integer"));
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "max-value" => {
                        request.max_value = Some(value.unwrap_or("").to_string());
                    },
                "min-value" => {
                        request.min_value = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "web-property-id" => {
                        request.web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "active" => {
                        request.active = Some(arg_from_str(value.unwrap_or("false"), err, "active", "boolean"));
                    },
                "scope" => {
                        request.scope = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_parent_link_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_parent_link_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_parent_link_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_parent_link_init(&mut request);
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "active", "created", "href", "id", "index", "kind", "max-value", "min-value", "name", "parent-link", "scope", "self-link", "type", "updated", "web-property-id"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.management().custom_metrics_insert(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""));
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

    fn _management_custom_metrics_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().custom_metrics_list(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
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
                                                Vec::new() + &self.gp + &["start-index", "max-results"]
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

    fn _management_custom_metrics_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::CustomMetric::default();
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
            fn request_parent_link_init(request: &mut api::CustomMetric) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "index" => {
                        request.index = Some(arg_from_str(value.unwrap_or("-0"), err, "index", "integer"));
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "max-value" => {
                        request.max_value = Some(value.unwrap_or("").to_string());
                    },
                "min-value" => {
                        request.min_value = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "web-property-id" => {
                        request.web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "active" => {
                        request.active = Some(arg_from_str(value.unwrap_or("false"), err, "active", "boolean"));
                    },
                "scope" => {
                        request.scope = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_parent_link_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_parent_link_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_parent_link_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_parent_link_init(&mut request);
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "active", "created", "href", "id", "index", "kind", "max-value", "min-value", "name", "parent-link", "scope", "self-link", "type", "updated", "web-property-id"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.management().custom_metrics_patch(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("custom-metric-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "ignore-custom-data-source-links" => {
                    call = call.ignore_custom_data_source_links(arg_from_str(value.unwrap_or("false"), err, "ignore-custom-data-source-links", "boolean"));
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
                                                Vec::new() + &self.gp + &["ignore-custom-data-source-links"]
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

    fn _management_custom_metrics_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::CustomMetric::default();
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
            fn request_parent_link_init(request: &mut api::CustomMetric) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "index" => {
                        request.index = Some(arg_from_str(value.unwrap_or("-0"), err, "index", "integer"));
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "max-value" => {
                        request.max_value = Some(value.unwrap_or("").to_string());
                    },
                "min-value" => {
                        request.min_value = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "web-property-id" => {
                        request.web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "active" => {
                        request.active = Some(arg_from_str(value.unwrap_or("false"), err, "active", "boolean"));
                    },
                "scope" => {
                        request.scope = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_parent_link_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_parent_link_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_parent_link_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_parent_link_init(&mut request);
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "active", "created", "href", "id", "index", "kind", "max-value", "min-value", "name", "parent-link", "scope", "self-link", "type", "updated", "web-property-id"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.management().custom_metrics_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("custom-metric-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "ignore-custom-data-source-links" => {
                    call = call.ignore_custom_data_source_links(arg_from_str(value.unwrap_or("false"), err, "ignore-custom-data-source-links", "boolean"));
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
                                                Vec::new() + &self.gp + &["ignore-custom-data-source-links"]
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

    fn _management_experiments_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().experiments_delete(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""), opt.value_of("experiment-id").unwrap_or(""));
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

    fn _management_experiments_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().experiments_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""), opt.value_of("experiment-id").unwrap_or(""));
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

    fn _management_experiments_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Experiment::default();
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
            fn request_parent_link_init(request: &mut api::Experiment) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "traffic-coverage" => {
                        request.traffic_coverage = Some(arg_from_str(value.unwrap_or("0.0"), err, "traffic-coverage", "number"));
                    },
                "web-property-id" => {
                        request.web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "optimization-type" => {
                        request.optimization_type = Some(value.unwrap_or("").to_string());
                    },
                "objective-metric" => {
                        request.objective_metric = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "equal-weighting" => {
                        request.equal_weighting = Some(arg_from_str(value.unwrap_or("false"), err, "equal-weighting", "boolean"));
                    },
                "account-id" => {
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet" => {
                        request.snippet = Some(value.unwrap_or("").to_string());
                    },
                "editable-in-ga-ui" => {
                        request.editable_in_ga_ui = Some(arg_from_str(value.unwrap_or("false"), err, "editable-in-ga-ui", "boolean"));
                    },
                "rewrite-variation-urls-as-original" => {
                        request.rewrite_variation_urls_as_original = Some(arg_from_str(value.unwrap_or("false"), err, "rewrite-variation-urls-as-original", "boolean"));
                    },
                "serving-framework" => {
                        request.serving_framework = Some(value.unwrap_or("").to_string());
                    },
                "status" => {
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "internal-web-property-id" => {
                        request.internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "winner-confidence-level" => {
                        request.winner_confidence_level = Some(arg_from_str(value.unwrap_or("0.0"), err, "winner-confidence-level", "number"));
                    },
                "start-time" => {
                        request.start_time = Some(value.unwrap_or("").to_string());
                    },
                "winner-found" => {
                        request.winner_found = Some(arg_from_str(value.unwrap_or("false"), err, "winner-found", "boolean"));
                    },
                "reason-experiment-ended" => {
                        request.reason_experiment_ended = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "minimum-experiment-length-in-days" => {
                        request.minimum_experiment_length_in_days = Some(arg_from_str(value.unwrap_or("-0"), err, "minimum-experiment-length-in-days", "integer"));
                    },
                "profile-id" => {
                        request.profile_id = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "end-time" => {
                        request_parent_link_init(&mut request);
                        request.end_time = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_parent_link_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "created", "description", "editable-in-ga-ui", "end-time", "equal-weighting", "href", "id", "internal-web-property-id", "kind", "minimum-experiment-length-in-days", "name", "objective-metric", "optimization-type", "parent-link", "profile-id", "reason-experiment-ended", "rewrite-variation-urls-as-original", "self-link", "serving-framework", "snippet", "start-time", "status", "traffic-coverage", "type", "updated", "web-property-id", "winner-confidence-level", "winner-found"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.management().experiments_insert(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""));
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

    fn _management_experiments_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().experiments_list(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
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
                                                Vec::new() + &self.gp + &["start-index", "max-results"]
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

    fn _management_experiments_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Experiment::default();
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
            fn request_parent_link_init(request: &mut api::Experiment) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "traffic-coverage" => {
                        request.traffic_coverage = Some(arg_from_str(value.unwrap_or("0.0"), err, "traffic-coverage", "number"));
                    },
                "web-property-id" => {
                        request.web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "optimization-type" => {
                        request.optimization_type = Some(value.unwrap_or("").to_string());
                    },
                "objective-metric" => {
                        request.objective_metric = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "equal-weighting" => {
                        request.equal_weighting = Some(arg_from_str(value.unwrap_or("false"), err, "equal-weighting", "boolean"));
                    },
                "account-id" => {
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet" => {
                        request.snippet = Some(value.unwrap_or("").to_string());
                    },
                "editable-in-ga-ui" => {
                        request.editable_in_ga_ui = Some(arg_from_str(value.unwrap_or("false"), err, "editable-in-ga-ui", "boolean"));
                    },
                "rewrite-variation-urls-as-original" => {
                        request.rewrite_variation_urls_as_original = Some(arg_from_str(value.unwrap_or("false"), err, "rewrite-variation-urls-as-original", "boolean"));
                    },
                "serving-framework" => {
                        request.serving_framework = Some(value.unwrap_or("").to_string());
                    },
                "status" => {
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "internal-web-property-id" => {
                        request.internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "winner-confidence-level" => {
                        request.winner_confidence_level = Some(arg_from_str(value.unwrap_or("0.0"), err, "winner-confidence-level", "number"));
                    },
                "start-time" => {
                        request.start_time = Some(value.unwrap_or("").to_string());
                    },
                "winner-found" => {
                        request.winner_found = Some(arg_from_str(value.unwrap_or("false"), err, "winner-found", "boolean"));
                    },
                "reason-experiment-ended" => {
                        request.reason_experiment_ended = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "minimum-experiment-length-in-days" => {
                        request.minimum_experiment_length_in_days = Some(arg_from_str(value.unwrap_or("-0"), err, "minimum-experiment-length-in-days", "integer"));
                    },
                "profile-id" => {
                        request.profile_id = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "end-time" => {
                        request_parent_link_init(&mut request);
                        request.end_time = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_parent_link_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "created", "description", "editable-in-ga-ui", "end-time", "equal-weighting", "href", "id", "internal-web-property-id", "kind", "minimum-experiment-length-in-days", "name", "objective-metric", "optimization-type", "parent-link", "profile-id", "reason-experiment-ended", "rewrite-variation-urls-as-original", "self-link", "serving-framework", "snippet", "start-time", "status", "traffic-coverage", "type", "updated", "web-property-id", "winner-confidence-level", "winner-found"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.management().experiments_patch(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""), opt.value_of("experiment-id").unwrap_or(""));
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

    fn _management_experiments_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Experiment::default();
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
            fn request_parent_link_init(request: &mut api::Experiment) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "traffic-coverage" => {
                        request.traffic_coverage = Some(arg_from_str(value.unwrap_or("0.0"), err, "traffic-coverage", "number"));
                    },
                "web-property-id" => {
                        request.web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "optimization-type" => {
                        request.optimization_type = Some(value.unwrap_or("").to_string());
                    },
                "objective-metric" => {
                        request.objective_metric = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "equal-weighting" => {
                        request.equal_weighting = Some(arg_from_str(value.unwrap_or("false"), err, "equal-weighting", "boolean"));
                    },
                "account-id" => {
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet" => {
                        request.snippet = Some(value.unwrap_or("").to_string());
                    },
                "editable-in-ga-ui" => {
                        request.editable_in_ga_ui = Some(arg_from_str(value.unwrap_or("false"), err, "editable-in-ga-ui", "boolean"));
                    },
                "rewrite-variation-urls-as-original" => {
                        request.rewrite_variation_urls_as_original = Some(arg_from_str(value.unwrap_or("false"), err, "rewrite-variation-urls-as-original", "boolean"));
                    },
                "serving-framework" => {
                        request.serving_framework = Some(value.unwrap_or("").to_string());
                    },
                "status" => {
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "internal-web-property-id" => {
                        request.internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "winner-confidence-level" => {
                        request.winner_confidence_level = Some(arg_from_str(value.unwrap_or("0.0"), err, "winner-confidence-level", "number"));
                    },
                "start-time" => {
                        request.start_time = Some(value.unwrap_or("").to_string());
                    },
                "winner-found" => {
                        request.winner_found = Some(arg_from_str(value.unwrap_or("false"), err, "winner-found", "boolean"));
                    },
                "reason-experiment-ended" => {
                        request.reason_experiment_ended = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "minimum-experiment-length-in-days" => {
                        request.minimum_experiment_length_in_days = Some(arg_from_str(value.unwrap_or("-0"), err, "minimum-experiment-length-in-days", "integer"));
                    },
                "profile-id" => {
                        request.profile_id = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "end-time" => {
                        request_parent_link_init(&mut request);
                        request.end_time = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_parent_link_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "created", "description", "editable-in-ga-ui", "end-time", "equal-weighting", "href", "id", "internal-web-property-id", "kind", "minimum-experiment-length-in-days", "name", "objective-metric", "optimization-type", "parent-link", "profile-id", "reason-experiment-ended", "rewrite-variation-urls-as-original", "self-link", "serving-framework", "snippet", "start-time", "status", "traffic-coverage", "type", "updated", "web-property-id", "winner-confidence-level", "winner-found"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.management().experiments_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""), opt.value_of("experiment-id").unwrap_or(""));
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

    fn _management_filters_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().filters_delete(opt.value_of("account-id").unwrap_or(""), opt.value_of("filter-id").unwrap_or(""));
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

    fn _management_filters_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().filters_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("filter-id").unwrap_or(""));
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

    fn _management_filters_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Filter::default();
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
            fn request_advanced_details_init(request: &mut api::Filter) {
                if request.advanced_details.is_none() {
                    request.advanced_details = Some(Default::default());
                }
            }
            
            fn request_exclude_details_init(request: &mut api::Filter) {
                if request.exclude_details.is_none() {
                    request.exclude_details = Some(Default::default());
                }
            }
            
            fn request_include_details_init(request: &mut api::Filter) {
                if request.include_details.is_none() {
                    request.include_details = Some(Default::default());
                }
            }
            
            fn request_lowercase_details_init(request: &mut api::Filter) {
                if request.lowercase_details.is_none() {
                    request.lowercase_details = Some(Default::default());
                }
            }
            
            fn request_parent_link_init(request: &mut api::Filter) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            fn request_search_and_replace_details_init(request: &mut api::Filter) {
                if request.search_and_replace_details.is_none() {
                    request.search_and_replace_details = Some(Default::default());
                }
            }
            
            fn request_uppercase_details_init(request: &mut api::Filter) {
                if request.uppercase_details.is_none() {
                    request.uppercase_details = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "uppercase-details.field" => {
                        request_uppercase_details_init(&mut request);
                        request.uppercase_details.as_mut().unwrap().field = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request_uppercase_details_init(&mut request);
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "advanced-details.extract-b" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().extract_b = Some(value.unwrap_or("").to_string());
                    },
                "advanced-details.override-output-field" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().override_output_field = Some(arg_from_str(value.unwrap_or("false"), err, "advanced-details.override-output-field", "boolean"));
                    },
                "advanced-details.field-a-required" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().field_a_required = Some(arg_from_str(value.unwrap_or("false"), err, "advanced-details.field-a-required", "boolean"));
                    },
                "advanced-details.output-constructor" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().output_constructor = Some(value.unwrap_or("").to_string());
                    },
                "advanced-details.field-b-required" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().field_b_required = Some(arg_from_str(value.unwrap_or("false"), err, "advanced-details.field-b-required", "boolean"));
                    },
                "advanced-details.case-sensitive" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().case_sensitive = Some(arg_from_str(value.unwrap_or("false"), err, "advanced-details.case-sensitive", "boolean"));
                    },
                "advanced-details.field-b" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().field_b = Some(value.unwrap_or("").to_string());
                    },
                "advanced-details.field-a" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().field_a = Some(value.unwrap_or("").to_string());
                    },
                "advanced-details.extract-a" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().extract_a = Some(value.unwrap_or("").to_string());
                    },
                "advanced-details.output-to-field" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().output_to_field = Some(value.unwrap_or("").to_string());
                    },
                "lowercase-details.field" => {
                        request_lowercase_details_init(&mut request);
                        request.lowercase_details.as_mut().unwrap().field = Some(value.unwrap_or("").to_string());
                    },
                "search-and-replace-details.case-sensitive" => {
                        request_search_and_replace_details_init(&mut request);
                        request.search_and_replace_details.as_mut().unwrap().case_sensitive = Some(arg_from_str(value.unwrap_or("false"), err, "search-and-replace-details.case-sensitive", "boolean"));
                    },
                "search-and-replace-details.search-string" => {
                        request_search_and_replace_details_init(&mut request);
                        request.search_and_replace_details.as_mut().unwrap().search_string = Some(value.unwrap_or("").to_string());
                    },
                "search-and-replace-details.replace-string" => {
                        request_search_and_replace_details_init(&mut request);
                        request.search_and_replace_details.as_mut().unwrap().replace_string = Some(value.unwrap_or("").to_string());
                    },
                "search-and-replace-details.field" => {
                        request_search_and_replace_details_init(&mut request);
                        request.search_and_replace_details.as_mut().unwrap().field = Some(value.unwrap_or("").to_string());
                    },
                "exclude-details.case-sensitive" => {
                        request_exclude_details_init(&mut request);
                        request.exclude_details.as_mut().unwrap().case_sensitive = Some(arg_from_str(value.unwrap_or("false"), err, "exclude-details.case-sensitive", "boolean"));
                    },
                "exclude-details.kind" => {
                        request_exclude_details_init(&mut request);
                        request.exclude_details.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "exclude-details.match-type" => {
                        request_exclude_details_init(&mut request);
                        request.exclude_details.as_mut().unwrap().match_type = Some(value.unwrap_or("").to_string());
                    },
                "exclude-details.expression-value" => {
                        request_exclude_details_init(&mut request);
                        request.exclude_details.as_mut().unwrap().expression_value = Some(value.unwrap_or("").to_string());
                    },
                "exclude-details.field" => {
                        request_exclude_details_init(&mut request);
                        request.exclude_details.as_mut().unwrap().field = Some(value.unwrap_or("").to_string());
                    },
                "include-details.case-sensitive" => {
                        request_include_details_init(&mut request);
                        request.include_details.as_mut().unwrap().case_sensitive = Some(arg_from_str(value.unwrap_or("false"), err, "include-details.case-sensitive", "boolean"));
                    },
                "include-details.kind" => {
                        request_include_details_init(&mut request);
                        request.include_details.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "include-details.match-type" => {
                        request_include_details_init(&mut request);
                        request.include_details.as_mut().unwrap().match_type = Some(value.unwrap_or("").to_string());
                    },
                "include-details.expression-value" => {
                        request_include_details_init(&mut request);
                        request.include_details.as_mut().unwrap().expression_value = Some(value.unwrap_or("").to_string());
                    },
                "include-details.field" => {
                        request_include_details_init(&mut request);
                        request.include_details.as_mut().unwrap().field = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_parent_link_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_parent_link_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_parent_link_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_parent_link_init(&mut request);
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "advanced-details", "case-sensitive", "created", "exclude-details", "expression-value", "extract-a", "extract-b", "field", "field-a", "field-a-required", "field-b", "field-b-required", "href", "id", "include-details", "kind", "lowercase-details", "match-type", "name", "output-constructor", "output-to-field", "override-output-field", "parent-link", "replace-string", "search-and-replace-details", "search-string", "self-link", "type", "updated", "uppercase-details"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.management().filters_insert(request, opt.value_of("account-id").unwrap_or(""));
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

    fn _management_filters_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().filters_list(opt.value_of("account-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
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
                                                Vec::new() + &self.gp + &["start-index", "max-results"]
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

    fn _management_filters_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Filter::default();
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
            fn request_advanced_details_init(request: &mut api::Filter) {
                if request.advanced_details.is_none() {
                    request.advanced_details = Some(Default::default());
                }
            }
            
            fn request_exclude_details_init(request: &mut api::Filter) {
                if request.exclude_details.is_none() {
                    request.exclude_details = Some(Default::default());
                }
            }
            
            fn request_include_details_init(request: &mut api::Filter) {
                if request.include_details.is_none() {
                    request.include_details = Some(Default::default());
                }
            }
            
            fn request_lowercase_details_init(request: &mut api::Filter) {
                if request.lowercase_details.is_none() {
                    request.lowercase_details = Some(Default::default());
                }
            }
            
            fn request_parent_link_init(request: &mut api::Filter) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            fn request_search_and_replace_details_init(request: &mut api::Filter) {
                if request.search_and_replace_details.is_none() {
                    request.search_and_replace_details = Some(Default::default());
                }
            }
            
            fn request_uppercase_details_init(request: &mut api::Filter) {
                if request.uppercase_details.is_none() {
                    request.uppercase_details = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "uppercase-details.field" => {
                        request_uppercase_details_init(&mut request);
                        request.uppercase_details.as_mut().unwrap().field = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request_uppercase_details_init(&mut request);
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "advanced-details.extract-b" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().extract_b = Some(value.unwrap_or("").to_string());
                    },
                "advanced-details.override-output-field" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().override_output_field = Some(arg_from_str(value.unwrap_or("false"), err, "advanced-details.override-output-field", "boolean"));
                    },
                "advanced-details.field-a-required" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().field_a_required = Some(arg_from_str(value.unwrap_or("false"), err, "advanced-details.field-a-required", "boolean"));
                    },
                "advanced-details.output-constructor" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().output_constructor = Some(value.unwrap_or("").to_string());
                    },
                "advanced-details.field-b-required" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().field_b_required = Some(arg_from_str(value.unwrap_or("false"), err, "advanced-details.field-b-required", "boolean"));
                    },
                "advanced-details.case-sensitive" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().case_sensitive = Some(arg_from_str(value.unwrap_or("false"), err, "advanced-details.case-sensitive", "boolean"));
                    },
                "advanced-details.field-b" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().field_b = Some(value.unwrap_or("").to_string());
                    },
                "advanced-details.field-a" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().field_a = Some(value.unwrap_or("").to_string());
                    },
                "advanced-details.extract-a" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().extract_a = Some(value.unwrap_or("").to_string());
                    },
                "advanced-details.output-to-field" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().output_to_field = Some(value.unwrap_or("").to_string());
                    },
                "lowercase-details.field" => {
                        request_lowercase_details_init(&mut request);
                        request.lowercase_details.as_mut().unwrap().field = Some(value.unwrap_or("").to_string());
                    },
                "search-and-replace-details.case-sensitive" => {
                        request_search_and_replace_details_init(&mut request);
                        request.search_and_replace_details.as_mut().unwrap().case_sensitive = Some(arg_from_str(value.unwrap_or("false"), err, "search-and-replace-details.case-sensitive", "boolean"));
                    },
                "search-and-replace-details.search-string" => {
                        request_search_and_replace_details_init(&mut request);
                        request.search_and_replace_details.as_mut().unwrap().search_string = Some(value.unwrap_or("").to_string());
                    },
                "search-and-replace-details.replace-string" => {
                        request_search_and_replace_details_init(&mut request);
                        request.search_and_replace_details.as_mut().unwrap().replace_string = Some(value.unwrap_or("").to_string());
                    },
                "search-and-replace-details.field" => {
                        request_search_and_replace_details_init(&mut request);
                        request.search_and_replace_details.as_mut().unwrap().field = Some(value.unwrap_or("").to_string());
                    },
                "exclude-details.case-sensitive" => {
                        request_exclude_details_init(&mut request);
                        request.exclude_details.as_mut().unwrap().case_sensitive = Some(arg_from_str(value.unwrap_or("false"), err, "exclude-details.case-sensitive", "boolean"));
                    },
                "exclude-details.kind" => {
                        request_exclude_details_init(&mut request);
                        request.exclude_details.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "exclude-details.match-type" => {
                        request_exclude_details_init(&mut request);
                        request.exclude_details.as_mut().unwrap().match_type = Some(value.unwrap_or("").to_string());
                    },
                "exclude-details.expression-value" => {
                        request_exclude_details_init(&mut request);
                        request.exclude_details.as_mut().unwrap().expression_value = Some(value.unwrap_or("").to_string());
                    },
                "exclude-details.field" => {
                        request_exclude_details_init(&mut request);
                        request.exclude_details.as_mut().unwrap().field = Some(value.unwrap_or("").to_string());
                    },
                "include-details.case-sensitive" => {
                        request_include_details_init(&mut request);
                        request.include_details.as_mut().unwrap().case_sensitive = Some(arg_from_str(value.unwrap_or("false"), err, "include-details.case-sensitive", "boolean"));
                    },
                "include-details.kind" => {
                        request_include_details_init(&mut request);
                        request.include_details.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "include-details.match-type" => {
                        request_include_details_init(&mut request);
                        request.include_details.as_mut().unwrap().match_type = Some(value.unwrap_or("").to_string());
                    },
                "include-details.expression-value" => {
                        request_include_details_init(&mut request);
                        request.include_details.as_mut().unwrap().expression_value = Some(value.unwrap_or("").to_string());
                    },
                "include-details.field" => {
                        request_include_details_init(&mut request);
                        request.include_details.as_mut().unwrap().field = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_parent_link_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_parent_link_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_parent_link_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_parent_link_init(&mut request);
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "advanced-details", "case-sensitive", "created", "exclude-details", "expression-value", "extract-a", "extract-b", "field", "field-a", "field-a-required", "field-b", "field-b-required", "href", "id", "include-details", "kind", "lowercase-details", "match-type", "name", "output-constructor", "output-to-field", "override-output-field", "parent-link", "replace-string", "search-and-replace-details", "search-string", "self-link", "type", "updated", "uppercase-details"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.management().filters_patch(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("filter-id").unwrap_or(""));
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

    fn _management_filters_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Filter::default();
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
            fn request_advanced_details_init(request: &mut api::Filter) {
                if request.advanced_details.is_none() {
                    request.advanced_details = Some(Default::default());
                }
            }
            
            fn request_exclude_details_init(request: &mut api::Filter) {
                if request.exclude_details.is_none() {
                    request.exclude_details = Some(Default::default());
                }
            }
            
            fn request_include_details_init(request: &mut api::Filter) {
                if request.include_details.is_none() {
                    request.include_details = Some(Default::default());
                }
            }
            
            fn request_lowercase_details_init(request: &mut api::Filter) {
                if request.lowercase_details.is_none() {
                    request.lowercase_details = Some(Default::default());
                }
            }
            
            fn request_parent_link_init(request: &mut api::Filter) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            fn request_search_and_replace_details_init(request: &mut api::Filter) {
                if request.search_and_replace_details.is_none() {
                    request.search_and_replace_details = Some(Default::default());
                }
            }
            
            fn request_uppercase_details_init(request: &mut api::Filter) {
                if request.uppercase_details.is_none() {
                    request.uppercase_details = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "uppercase-details.field" => {
                        request_uppercase_details_init(&mut request);
                        request.uppercase_details.as_mut().unwrap().field = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request_uppercase_details_init(&mut request);
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "advanced-details.extract-b" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().extract_b = Some(value.unwrap_or("").to_string());
                    },
                "advanced-details.override-output-field" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().override_output_field = Some(arg_from_str(value.unwrap_or("false"), err, "advanced-details.override-output-field", "boolean"));
                    },
                "advanced-details.field-a-required" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().field_a_required = Some(arg_from_str(value.unwrap_or("false"), err, "advanced-details.field-a-required", "boolean"));
                    },
                "advanced-details.output-constructor" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().output_constructor = Some(value.unwrap_or("").to_string());
                    },
                "advanced-details.field-b-required" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().field_b_required = Some(arg_from_str(value.unwrap_or("false"), err, "advanced-details.field-b-required", "boolean"));
                    },
                "advanced-details.case-sensitive" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().case_sensitive = Some(arg_from_str(value.unwrap_or("false"), err, "advanced-details.case-sensitive", "boolean"));
                    },
                "advanced-details.field-b" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().field_b = Some(value.unwrap_or("").to_string());
                    },
                "advanced-details.field-a" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().field_a = Some(value.unwrap_or("").to_string());
                    },
                "advanced-details.extract-a" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().extract_a = Some(value.unwrap_or("").to_string());
                    },
                "advanced-details.output-to-field" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().output_to_field = Some(value.unwrap_or("").to_string());
                    },
                "lowercase-details.field" => {
                        request_lowercase_details_init(&mut request);
                        request.lowercase_details.as_mut().unwrap().field = Some(value.unwrap_or("").to_string());
                    },
                "search-and-replace-details.case-sensitive" => {
                        request_search_and_replace_details_init(&mut request);
                        request.search_and_replace_details.as_mut().unwrap().case_sensitive = Some(arg_from_str(value.unwrap_or("false"), err, "search-and-replace-details.case-sensitive", "boolean"));
                    },
                "search-and-replace-details.search-string" => {
                        request_search_and_replace_details_init(&mut request);
                        request.search_and_replace_details.as_mut().unwrap().search_string = Some(value.unwrap_or("").to_string());
                    },
                "search-and-replace-details.replace-string" => {
                        request_search_and_replace_details_init(&mut request);
                        request.search_and_replace_details.as_mut().unwrap().replace_string = Some(value.unwrap_or("").to_string());
                    },
                "search-and-replace-details.field" => {
                        request_search_and_replace_details_init(&mut request);
                        request.search_and_replace_details.as_mut().unwrap().field = Some(value.unwrap_or("").to_string());
                    },
                "exclude-details.case-sensitive" => {
                        request_exclude_details_init(&mut request);
                        request.exclude_details.as_mut().unwrap().case_sensitive = Some(arg_from_str(value.unwrap_or("false"), err, "exclude-details.case-sensitive", "boolean"));
                    },
                "exclude-details.kind" => {
                        request_exclude_details_init(&mut request);
                        request.exclude_details.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "exclude-details.match-type" => {
                        request_exclude_details_init(&mut request);
                        request.exclude_details.as_mut().unwrap().match_type = Some(value.unwrap_or("").to_string());
                    },
                "exclude-details.expression-value" => {
                        request_exclude_details_init(&mut request);
                        request.exclude_details.as_mut().unwrap().expression_value = Some(value.unwrap_or("").to_string());
                    },
                "exclude-details.field" => {
                        request_exclude_details_init(&mut request);
                        request.exclude_details.as_mut().unwrap().field = Some(value.unwrap_or("").to_string());
                    },
                "include-details.case-sensitive" => {
                        request_include_details_init(&mut request);
                        request.include_details.as_mut().unwrap().case_sensitive = Some(arg_from_str(value.unwrap_or("false"), err, "include-details.case-sensitive", "boolean"));
                    },
                "include-details.kind" => {
                        request_include_details_init(&mut request);
                        request.include_details.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "include-details.match-type" => {
                        request_include_details_init(&mut request);
                        request.include_details.as_mut().unwrap().match_type = Some(value.unwrap_or("").to_string());
                    },
                "include-details.expression-value" => {
                        request_include_details_init(&mut request);
                        request.include_details.as_mut().unwrap().expression_value = Some(value.unwrap_or("").to_string());
                    },
                "include-details.field" => {
                        request_include_details_init(&mut request);
                        request.include_details.as_mut().unwrap().field = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_parent_link_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_parent_link_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_parent_link_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_parent_link_init(&mut request);
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "advanced-details", "case-sensitive", "created", "exclude-details", "expression-value", "extract-a", "extract-b", "field", "field-a", "field-a-required", "field-b", "field-b-required", "href", "id", "include-details", "kind", "lowercase-details", "match-type", "name", "output-constructor", "output-to-field", "override-output-field", "parent-link", "replace-string", "search-and-replace-details", "search-string", "self-link", "type", "updated", "uppercase-details"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.management().filters_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("filter-id").unwrap_or(""));
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

    fn _management_goals_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().goals_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""), opt.value_of("goal-id").unwrap_or(""));
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

    fn _management_goals_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Goal::default();
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
            fn request_event_details_init(request: &mut api::Goal) {
                if request.event_details.is_none() {
                    request.event_details = Some(Default::default());
                }
            }
            
            fn request_parent_link_init(request: &mut api::Goal) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            fn request_url_destination_details_init(request: &mut api::Goal) {
                if request.url_destination_details.is_none() {
                    request.url_destination_details = Some(Default::default());
                }
            }
            
            fn request_visit_num_pages_details_init(request: &mut api::Goal) {
                if request.visit_num_pages_details.is_none() {
                    request.visit_num_pages_details = Some(Default::default());
                }
            }
            
            fn request_visit_time_on_site_details_init(request: &mut api::Goal) {
                if request.visit_time_on_site_details.is_none() {
                    request.visit_time_on_site_details = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "visit-time-on-site-details.comparison-type" => {
                        request_visit_time_on_site_details_init(&mut request);
                        request.visit_time_on_site_details.as_mut().unwrap().comparison_type = Some(value.unwrap_or("").to_string());
                    },
                "visit-time-on-site-details.comparison-value" => {
                        request_visit_time_on_site_details_init(&mut request);
                        request.visit_time_on_site_details.as_mut().unwrap().comparison_value = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_visit_time_on_site_details_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request_visit_time_on_site_details_init(&mut request);
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "url-destination-details.url" => {
                        request_url_destination_details_init(&mut request);
                        request.url_destination_details.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "url-destination-details.case-sensitive" => {
                        request_url_destination_details_init(&mut request);
                        request.url_destination_details.as_mut().unwrap().case_sensitive = Some(arg_from_str(value.unwrap_or("false"), err, "url-destination-details.case-sensitive", "boolean"));
                    },
                "url-destination-details.match-type" => {
                        request_url_destination_details_init(&mut request);
                        request.url_destination_details.as_mut().unwrap().match_type = Some(value.unwrap_or("").to_string());
                    },
                "url-destination-details.first-step-required" => {
                        request_url_destination_details_init(&mut request);
                        request.url_destination_details.as_mut().unwrap().first_step_required = Some(arg_from_str(value.unwrap_or("false"), err, "url-destination-details.first-step-required", "boolean"));
                    },
                "kind" => {
                        request_url_destination_details_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "value" => {
                        request_url_destination_details_init(&mut request);
                        request.value = Some(arg_from_str(value.unwrap_or("0.0"), err, "value", "number"));
                    },
                "visit-num-pages-details.comparison-type" => {
                        request_visit_num_pages_details_init(&mut request);
                        request.visit_num_pages_details.as_mut().unwrap().comparison_type = Some(value.unwrap_or("").to_string());
                    },
                "visit-num-pages-details.comparison-value" => {
                        request_visit_num_pages_details_init(&mut request);
                        request.visit_num_pages_details.as_mut().unwrap().comparison_value = Some(value.unwrap_or("").to_string());
                    },
                "internal-web-property-id" => {
                        request_visit_num_pages_details_init(&mut request);
                        request.internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "event-details.use-event-value" => {
                        request_event_details_init(&mut request);
                        request.event_details.as_mut().unwrap().use_event_value = Some(arg_from_str(value.unwrap_or("false"), err, "event-details.use-event-value", "boolean"));
                    },
                "web-property-id" => {
                        request_event_details_init(&mut request);
                        request.web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "active" => {
                        request_event_details_init(&mut request);
                        request.active = Some(arg_from_str(value.unwrap_or("false"), err, "active", "boolean"));
                    },
                "profile-id" => {
                        request_event_details_init(&mut request);
                        request.profile_id = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_parent_link_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_parent_link_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_parent_link_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_parent_link_init(&mut request);
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "active", "case-sensitive", "comparison-type", "comparison-value", "created", "event-details", "first-step-required", "href", "id", "internal-web-property-id", "kind", "match-type", "name", "parent-link", "profile-id", "self-link", "type", "updated", "url", "url-destination-details", "use-event-value", "value", "visit-num-pages-details", "visit-time-on-site-details", "web-property-id"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.management().goals_insert(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""));
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

    fn _management_goals_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().goals_list(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
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
                                                Vec::new() + &self.gp + &["start-index", "max-results"]
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

    fn _management_goals_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Goal::default();
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
            fn request_event_details_init(request: &mut api::Goal) {
                if request.event_details.is_none() {
                    request.event_details = Some(Default::default());
                }
            }
            
            fn request_parent_link_init(request: &mut api::Goal) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            fn request_url_destination_details_init(request: &mut api::Goal) {
                if request.url_destination_details.is_none() {
                    request.url_destination_details = Some(Default::default());
                }
            }
            
            fn request_visit_num_pages_details_init(request: &mut api::Goal) {
                if request.visit_num_pages_details.is_none() {
                    request.visit_num_pages_details = Some(Default::default());
                }
            }
            
            fn request_visit_time_on_site_details_init(request: &mut api::Goal) {
                if request.visit_time_on_site_details.is_none() {
                    request.visit_time_on_site_details = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "visit-time-on-site-details.comparison-type" => {
                        request_visit_time_on_site_details_init(&mut request);
                        request.visit_time_on_site_details.as_mut().unwrap().comparison_type = Some(value.unwrap_or("").to_string());
                    },
                "visit-time-on-site-details.comparison-value" => {
                        request_visit_time_on_site_details_init(&mut request);
                        request.visit_time_on_site_details.as_mut().unwrap().comparison_value = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_visit_time_on_site_details_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request_visit_time_on_site_details_init(&mut request);
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "url-destination-details.url" => {
                        request_url_destination_details_init(&mut request);
                        request.url_destination_details.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "url-destination-details.case-sensitive" => {
                        request_url_destination_details_init(&mut request);
                        request.url_destination_details.as_mut().unwrap().case_sensitive = Some(arg_from_str(value.unwrap_or("false"), err, "url-destination-details.case-sensitive", "boolean"));
                    },
                "url-destination-details.match-type" => {
                        request_url_destination_details_init(&mut request);
                        request.url_destination_details.as_mut().unwrap().match_type = Some(value.unwrap_or("").to_string());
                    },
                "url-destination-details.first-step-required" => {
                        request_url_destination_details_init(&mut request);
                        request.url_destination_details.as_mut().unwrap().first_step_required = Some(arg_from_str(value.unwrap_or("false"), err, "url-destination-details.first-step-required", "boolean"));
                    },
                "kind" => {
                        request_url_destination_details_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "value" => {
                        request_url_destination_details_init(&mut request);
                        request.value = Some(arg_from_str(value.unwrap_or("0.0"), err, "value", "number"));
                    },
                "visit-num-pages-details.comparison-type" => {
                        request_visit_num_pages_details_init(&mut request);
                        request.visit_num_pages_details.as_mut().unwrap().comparison_type = Some(value.unwrap_or("").to_string());
                    },
                "visit-num-pages-details.comparison-value" => {
                        request_visit_num_pages_details_init(&mut request);
                        request.visit_num_pages_details.as_mut().unwrap().comparison_value = Some(value.unwrap_or("").to_string());
                    },
                "internal-web-property-id" => {
                        request_visit_num_pages_details_init(&mut request);
                        request.internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "event-details.use-event-value" => {
                        request_event_details_init(&mut request);
                        request.event_details.as_mut().unwrap().use_event_value = Some(arg_from_str(value.unwrap_or("false"), err, "event-details.use-event-value", "boolean"));
                    },
                "web-property-id" => {
                        request_event_details_init(&mut request);
                        request.web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "active" => {
                        request_event_details_init(&mut request);
                        request.active = Some(arg_from_str(value.unwrap_or("false"), err, "active", "boolean"));
                    },
                "profile-id" => {
                        request_event_details_init(&mut request);
                        request.profile_id = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_parent_link_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_parent_link_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_parent_link_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_parent_link_init(&mut request);
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "active", "case-sensitive", "comparison-type", "comparison-value", "created", "event-details", "first-step-required", "href", "id", "internal-web-property-id", "kind", "match-type", "name", "parent-link", "profile-id", "self-link", "type", "updated", "url", "url-destination-details", "use-event-value", "value", "visit-num-pages-details", "visit-time-on-site-details", "web-property-id"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.management().goals_patch(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""), opt.value_of("goal-id").unwrap_or(""));
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

    fn _management_goals_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Goal::default();
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
            fn request_event_details_init(request: &mut api::Goal) {
                if request.event_details.is_none() {
                    request.event_details = Some(Default::default());
                }
            }
            
            fn request_parent_link_init(request: &mut api::Goal) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            fn request_url_destination_details_init(request: &mut api::Goal) {
                if request.url_destination_details.is_none() {
                    request.url_destination_details = Some(Default::default());
                }
            }
            
            fn request_visit_num_pages_details_init(request: &mut api::Goal) {
                if request.visit_num_pages_details.is_none() {
                    request.visit_num_pages_details = Some(Default::default());
                }
            }
            
            fn request_visit_time_on_site_details_init(request: &mut api::Goal) {
                if request.visit_time_on_site_details.is_none() {
                    request.visit_time_on_site_details = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "visit-time-on-site-details.comparison-type" => {
                        request_visit_time_on_site_details_init(&mut request);
                        request.visit_time_on_site_details.as_mut().unwrap().comparison_type = Some(value.unwrap_or("").to_string());
                    },
                "visit-time-on-site-details.comparison-value" => {
                        request_visit_time_on_site_details_init(&mut request);
                        request.visit_time_on_site_details.as_mut().unwrap().comparison_value = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_visit_time_on_site_details_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request_visit_time_on_site_details_init(&mut request);
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "url-destination-details.url" => {
                        request_url_destination_details_init(&mut request);
                        request.url_destination_details.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "url-destination-details.case-sensitive" => {
                        request_url_destination_details_init(&mut request);
                        request.url_destination_details.as_mut().unwrap().case_sensitive = Some(arg_from_str(value.unwrap_or("false"), err, "url-destination-details.case-sensitive", "boolean"));
                    },
                "url-destination-details.match-type" => {
                        request_url_destination_details_init(&mut request);
                        request.url_destination_details.as_mut().unwrap().match_type = Some(value.unwrap_or("").to_string());
                    },
                "url-destination-details.first-step-required" => {
                        request_url_destination_details_init(&mut request);
                        request.url_destination_details.as_mut().unwrap().first_step_required = Some(arg_from_str(value.unwrap_or("false"), err, "url-destination-details.first-step-required", "boolean"));
                    },
                "kind" => {
                        request_url_destination_details_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "value" => {
                        request_url_destination_details_init(&mut request);
                        request.value = Some(arg_from_str(value.unwrap_or("0.0"), err, "value", "number"));
                    },
                "visit-num-pages-details.comparison-type" => {
                        request_visit_num_pages_details_init(&mut request);
                        request.visit_num_pages_details.as_mut().unwrap().comparison_type = Some(value.unwrap_or("").to_string());
                    },
                "visit-num-pages-details.comparison-value" => {
                        request_visit_num_pages_details_init(&mut request);
                        request.visit_num_pages_details.as_mut().unwrap().comparison_value = Some(value.unwrap_or("").to_string());
                    },
                "internal-web-property-id" => {
                        request_visit_num_pages_details_init(&mut request);
                        request.internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "event-details.use-event-value" => {
                        request_event_details_init(&mut request);
                        request.event_details.as_mut().unwrap().use_event_value = Some(arg_from_str(value.unwrap_or("false"), err, "event-details.use-event-value", "boolean"));
                    },
                "web-property-id" => {
                        request_event_details_init(&mut request);
                        request.web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "active" => {
                        request_event_details_init(&mut request);
                        request.active = Some(arg_from_str(value.unwrap_or("false"), err, "active", "boolean"));
                    },
                "profile-id" => {
                        request_event_details_init(&mut request);
                        request.profile_id = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_parent_link_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_parent_link_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_parent_link_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_parent_link_init(&mut request);
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "active", "case-sensitive", "comparison-type", "comparison-value", "created", "event-details", "first-step-required", "href", "id", "internal-web-property-id", "kind", "match-type", "name", "parent-link", "profile-id", "self-link", "type", "updated", "url", "url-destination-details", "use-event-value", "value", "visit-num-pages-details", "visit-time-on-site-details", "web-property-id"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.management().goals_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""), opt.value_of("goal-id").unwrap_or(""));
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

    fn _management_profile_filter_links_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().profile_filter_links_delete(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""), opt.value_of("link-id").unwrap_or(""));
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

    fn _management_profile_filter_links_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().profile_filter_links_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""), opt.value_of("link-id").unwrap_or(""));
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

    fn _management_profile_filter_links_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::ProfileFilterLink::default();
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
            fn request_filter_ref_init(request: &mut api::ProfileFilterLink) {
                if request.filter_ref.is_none() {
                    request.filter_ref = Some(Default::default());
                }
            }
            
            fn request_profile_ref_init(request: &mut api::ProfileFilterLink) {
                if request.profile_ref.is_none() {
                    request.profile_ref = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "rank" => {
                        request.rank = Some(arg_from_str(value.unwrap_or("-0"), err, "rank", "integer"));
                    },
                "filter-ref.kind" => {
                        request_filter_ref_init(&mut request);
                        request.filter_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "filter-ref.href" => {
                        request_filter_ref_init(&mut request);
                        request.filter_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "filter-ref.id" => {
                        request_filter_ref_init(&mut request);
                        request.filter_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "filter-ref.name" => {
                        request_filter_ref_init(&mut request);
                        request.filter_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "filter-ref.account-id" => {
                        request_filter_ref_init(&mut request);
                        request.filter_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.kind" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.name" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.internal-web-property-id" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.href" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.web-property-id" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.id" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.account-id" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_profile_ref_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_profile_ref_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "filter-ref", "href", "id", "internal-web-property-id", "kind", "name", "profile-ref", "rank", "self-link", "web-property-id"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.management().profile_filter_links_insert(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""));
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

    fn _management_profile_filter_links_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().profile_filter_links_list(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
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
                                                Vec::new() + &self.gp + &["start-index", "max-results"]
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

    fn _management_profile_filter_links_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::ProfileFilterLink::default();
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
            fn request_filter_ref_init(request: &mut api::ProfileFilterLink) {
                if request.filter_ref.is_none() {
                    request.filter_ref = Some(Default::default());
                }
            }
            
            fn request_profile_ref_init(request: &mut api::ProfileFilterLink) {
                if request.profile_ref.is_none() {
                    request.profile_ref = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "rank" => {
                        request.rank = Some(arg_from_str(value.unwrap_or("-0"), err, "rank", "integer"));
                    },
                "filter-ref.kind" => {
                        request_filter_ref_init(&mut request);
                        request.filter_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "filter-ref.href" => {
                        request_filter_ref_init(&mut request);
                        request.filter_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "filter-ref.id" => {
                        request_filter_ref_init(&mut request);
                        request.filter_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "filter-ref.name" => {
                        request_filter_ref_init(&mut request);
                        request.filter_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "filter-ref.account-id" => {
                        request_filter_ref_init(&mut request);
                        request.filter_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.kind" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.name" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.internal-web-property-id" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.href" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.web-property-id" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.id" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.account-id" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_profile_ref_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_profile_ref_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "filter-ref", "href", "id", "internal-web-property-id", "kind", "name", "profile-ref", "rank", "self-link", "web-property-id"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.management().profile_filter_links_patch(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""), opt.value_of("link-id").unwrap_or(""));
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

    fn _management_profile_filter_links_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::ProfileFilterLink::default();
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
            fn request_filter_ref_init(request: &mut api::ProfileFilterLink) {
                if request.filter_ref.is_none() {
                    request.filter_ref = Some(Default::default());
                }
            }
            
            fn request_profile_ref_init(request: &mut api::ProfileFilterLink) {
                if request.profile_ref.is_none() {
                    request.profile_ref = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "rank" => {
                        request.rank = Some(arg_from_str(value.unwrap_or("-0"), err, "rank", "integer"));
                    },
                "filter-ref.kind" => {
                        request_filter_ref_init(&mut request);
                        request.filter_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "filter-ref.href" => {
                        request_filter_ref_init(&mut request);
                        request.filter_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "filter-ref.id" => {
                        request_filter_ref_init(&mut request);
                        request.filter_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "filter-ref.name" => {
                        request_filter_ref_init(&mut request);
                        request.filter_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "filter-ref.account-id" => {
                        request_filter_ref_init(&mut request);
                        request.filter_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.kind" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.name" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.internal-web-property-id" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.href" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.web-property-id" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.id" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.account-id" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_profile_ref_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_profile_ref_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "filter-ref", "href", "id", "internal-web-property-id", "kind", "name", "profile-ref", "rank", "self-link", "web-property-id"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.management().profile_filter_links_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""), opt.value_of("link-id").unwrap_or(""));
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

    fn _management_profile_user_links_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().profile_user_links_delete(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""), opt.value_of("link-id").unwrap_or(""));
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

    fn _management_profile_user_links_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::EntityUserLink::default();
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
            fn request_entity_account_ref_init(request: &mut api::EntityUserLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().account_ref.is_none() {
                    request.entity.as_mut().unwrap().account_ref = Some(Default::default());
                }
            }
            
            fn request_entity_init(request: &mut api::EntityUserLink) {
                if request.entity.is_none() {
                    request.entity = Some(Default::default());
                }
            }
            
            fn request_entity_profile_ref_init(request: &mut api::EntityUserLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().profile_ref.is_none() {
                    request.entity.as_mut().unwrap().profile_ref = Some(Default::default());
                }
            }
            
            fn request_entity_web_property_ref_init(request: &mut api::EntityUserLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().web_property_ref.is_none() {
                    request.entity.as_mut().unwrap().web_property_ref = Some(Default::default());
                }
            }
            
            fn request_permissions_init(request: &mut api::EntityUserLink) {
                if request.permissions.is_none() {
                    request.permissions = Some(Default::default());
                }
            }
            
            fn request_user_ref_init(request: &mut api::EntityUserLink) {
                if request.user_ref.is_none() {
                    request.user_ref = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.kind" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.href" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.id" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.name" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.kind" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.name" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.internal-web-property-id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.href" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.web-property-id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.account-id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.kind" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.name" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.internal-web-property-id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.href" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.account-id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "user-ref.kind" => {
                        request_user_ref_init(&mut request);
                        request.user_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "user-ref.email" => {
                        request_user_ref_init(&mut request);
                        request.user_ref.as_mut().unwrap().email = Some(value.unwrap_or("").to_string());
                    },
                "user-ref.id" => {
                        request_user_ref_init(&mut request);
                        request.user_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_user_ref_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_user_ref_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "permissions.local" => {
                        request_permissions_init(&mut request);
                        if request.permissions.as_mut().unwrap().local.is_none() {
                           request.permissions.as_mut().unwrap().local = Some(Default::default());
                        }
                                        request.permissions.as_mut().unwrap().local.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "permissions.effective" => {
                        request_permissions_init(&mut request);
                        if request.permissions.as_mut().unwrap().effective.is_none() {
                           request.permissions.as_mut().unwrap().effective = Some(Default::default());
                        }
                                        request.permissions.as_mut().unwrap().effective.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "account-ref", "effective", "email", "entity", "href", "id", "internal-web-property-id", "kind", "local", "name", "permissions", "profile-ref", "self-link", "user-ref", "web-property-id", "web-property-ref"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.management().profile_user_links_insert(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""));
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

    fn _management_profile_user_links_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().profile_user_links_list(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
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
                                                Vec::new() + &self.gp + &["start-index", "max-results"]
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

    fn _management_profile_user_links_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::EntityUserLink::default();
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
            fn request_entity_account_ref_init(request: &mut api::EntityUserLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().account_ref.is_none() {
                    request.entity.as_mut().unwrap().account_ref = Some(Default::default());
                }
            }
            
            fn request_entity_init(request: &mut api::EntityUserLink) {
                if request.entity.is_none() {
                    request.entity = Some(Default::default());
                }
            }
            
            fn request_entity_profile_ref_init(request: &mut api::EntityUserLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().profile_ref.is_none() {
                    request.entity.as_mut().unwrap().profile_ref = Some(Default::default());
                }
            }
            
            fn request_entity_web_property_ref_init(request: &mut api::EntityUserLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().web_property_ref.is_none() {
                    request.entity.as_mut().unwrap().web_property_ref = Some(Default::default());
                }
            }
            
            fn request_permissions_init(request: &mut api::EntityUserLink) {
                if request.permissions.is_none() {
                    request.permissions = Some(Default::default());
                }
            }
            
            fn request_user_ref_init(request: &mut api::EntityUserLink) {
                if request.user_ref.is_none() {
                    request.user_ref = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.kind" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.href" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.id" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.name" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.kind" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.name" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.internal-web-property-id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.href" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.web-property-id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.account-id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.kind" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.name" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.internal-web-property-id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.href" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.account-id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "user-ref.kind" => {
                        request_user_ref_init(&mut request);
                        request.user_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "user-ref.email" => {
                        request_user_ref_init(&mut request);
                        request.user_ref.as_mut().unwrap().email = Some(value.unwrap_or("").to_string());
                    },
                "user-ref.id" => {
                        request_user_ref_init(&mut request);
                        request.user_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_user_ref_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_user_ref_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "permissions.local" => {
                        request_permissions_init(&mut request);
                        if request.permissions.as_mut().unwrap().local.is_none() {
                           request.permissions.as_mut().unwrap().local = Some(Default::default());
                        }
                                        request.permissions.as_mut().unwrap().local.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "permissions.effective" => {
                        request_permissions_init(&mut request);
                        if request.permissions.as_mut().unwrap().effective.is_none() {
                           request.permissions.as_mut().unwrap().effective = Some(Default::default());
                        }
                                        request.permissions.as_mut().unwrap().effective.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "account-ref", "effective", "email", "entity", "href", "id", "internal-web-property-id", "kind", "local", "name", "permissions", "profile-ref", "self-link", "user-ref", "web-property-id", "web-property-ref"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.management().profile_user_links_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""), opt.value_of("link-id").unwrap_or(""));
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

    fn _management_profiles_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().profiles_delete(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""));
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

    fn _management_profiles_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().profiles_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""));
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

    fn _management_profiles_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Profile::default();
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
            fn request_child_link_init(request: &mut api::Profile) {
                if request.child_link.is_none() {
                    request.child_link = Some(Default::default());
                }
            }
            
            fn request_parent_link_init(request: &mut api::Profile) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            fn request_permissions_init(request: &mut api::Profile) {
                if request.permissions.is_none() {
                    request.permissions = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "currency" => {
                        request.currency = Some(value.unwrap_or("").to_string());
                    },
                "e-commerce-tracking" => {
                        request.e_commerce_tracking = Some(arg_from_str(value.unwrap_or("false"), err, "e-commerce-tracking", "boolean"));
                    },
                "web-property-id" => {
                        request.web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "timezone" => {
                        request.timezone = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                "strip-site-search-category-parameters" => {
                        request.strip_site_search_category_parameters = Some(arg_from_str(value.unwrap_or("false"), err, "strip-site-search-category-parameters", "boolean"));
                    },
                "site-search-category-parameters" => {
                        request.site_search_category_parameters = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "exclude-query-parameters" => {
                        request.exclude_query_parameters = Some(value.unwrap_or("").to_string());
                    },
                "internal-web-property-id" => {
                        request.internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "child-link.href" => {
                        request_child_link_init(&mut request);
                        request.child_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "child-link.type" => {
                        request_child_link_init(&mut request);
                        request.child_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "enhanced-e-commerce-tracking" => {
                        request_child_link_init(&mut request);
                        request.enhanced_e_commerce_tracking = Some(arg_from_str(value.unwrap_or("false"), err, "enhanced-e-commerce-tracking", "boolean"));
                    },
                "permissions.effective" => {
                        request_permissions_init(&mut request);
                        if request.permissions.as_mut().unwrap().effective.is_none() {
                           request.permissions.as_mut().unwrap().effective = Some(Default::default());
                        }
                                        request.permissions.as_mut().unwrap().effective.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "default-page" => {
                        request_permissions_init(&mut request);
                        request.default_page = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_permissions_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "strip-site-search-query-parameters" => {
                        request_permissions_init(&mut request);
                        request.strip_site_search_query_parameters = Some(arg_from_str(value.unwrap_or("false"), err, "strip-site-search-query-parameters", "boolean"));
                    },
                "name" => {
                        request_permissions_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request_permissions_init(&mut request);
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "site-search-query-parameters" => {
                        request_permissions_init(&mut request);
                        request.site_search_query_parameters = Some(value.unwrap_or("").to_string());
                    },
                "website-url" => {
                        request_permissions_init(&mut request);
                        request.website_url = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_parent_link_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "child-link", "created", "currency", "default-page", "e-commerce-tracking", "effective", "enhanced-e-commerce-tracking", "exclude-query-parameters", "href", "id", "internal-web-property-id", "kind", "name", "parent-link", "permissions", "self-link", "site-search-category-parameters", "site-search-query-parameters", "strip-site-search-category-parameters", "strip-site-search-query-parameters", "timezone", "type", "updated", "web-property-id", "website-url"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.management().profiles_insert(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""));
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

    fn _management_profiles_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().profiles_list(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
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
                                                Vec::new() + &self.gp + &["start-index", "max-results"]
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

    fn _management_profiles_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Profile::default();
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
            fn request_child_link_init(request: &mut api::Profile) {
                if request.child_link.is_none() {
                    request.child_link = Some(Default::default());
                }
            }
            
            fn request_parent_link_init(request: &mut api::Profile) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            fn request_permissions_init(request: &mut api::Profile) {
                if request.permissions.is_none() {
                    request.permissions = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "currency" => {
                        request.currency = Some(value.unwrap_or("").to_string());
                    },
                "e-commerce-tracking" => {
                        request.e_commerce_tracking = Some(arg_from_str(value.unwrap_or("false"), err, "e-commerce-tracking", "boolean"));
                    },
                "web-property-id" => {
                        request.web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "timezone" => {
                        request.timezone = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                "strip-site-search-category-parameters" => {
                        request.strip_site_search_category_parameters = Some(arg_from_str(value.unwrap_or("false"), err, "strip-site-search-category-parameters", "boolean"));
                    },
                "site-search-category-parameters" => {
                        request.site_search_category_parameters = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "exclude-query-parameters" => {
                        request.exclude_query_parameters = Some(value.unwrap_or("").to_string());
                    },
                "internal-web-property-id" => {
                        request.internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "child-link.href" => {
                        request_child_link_init(&mut request);
                        request.child_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "child-link.type" => {
                        request_child_link_init(&mut request);
                        request.child_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "enhanced-e-commerce-tracking" => {
                        request_child_link_init(&mut request);
                        request.enhanced_e_commerce_tracking = Some(arg_from_str(value.unwrap_or("false"), err, "enhanced-e-commerce-tracking", "boolean"));
                    },
                "permissions.effective" => {
                        request_permissions_init(&mut request);
                        if request.permissions.as_mut().unwrap().effective.is_none() {
                           request.permissions.as_mut().unwrap().effective = Some(Default::default());
                        }
                                        request.permissions.as_mut().unwrap().effective.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "default-page" => {
                        request_permissions_init(&mut request);
                        request.default_page = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_permissions_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "strip-site-search-query-parameters" => {
                        request_permissions_init(&mut request);
                        request.strip_site_search_query_parameters = Some(arg_from_str(value.unwrap_or("false"), err, "strip-site-search-query-parameters", "boolean"));
                    },
                "name" => {
                        request_permissions_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request_permissions_init(&mut request);
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "site-search-query-parameters" => {
                        request_permissions_init(&mut request);
                        request.site_search_query_parameters = Some(value.unwrap_or("").to_string());
                    },
                "website-url" => {
                        request_permissions_init(&mut request);
                        request.website_url = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_parent_link_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "child-link", "created", "currency", "default-page", "e-commerce-tracking", "effective", "enhanced-e-commerce-tracking", "exclude-query-parameters", "href", "id", "internal-web-property-id", "kind", "name", "parent-link", "permissions", "self-link", "site-search-category-parameters", "site-search-query-parameters", "strip-site-search-category-parameters", "strip-site-search-query-parameters", "timezone", "type", "updated", "web-property-id", "website-url"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.management().profiles_patch(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""));
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

    fn _management_profiles_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Profile::default();
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
            fn request_child_link_init(request: &mut api::Profile) {
                if request.child_link.is_none() {
                    request.child_link = Some(Default::default());
                }
            }
            
            fn request_parent_link_init(request: &mut api::Profile) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            fn request_permissions_init(request: &mut api::Profile) {
                if request.permissions.is_none() {
                    request.permissions = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "currency" => {
                        request.currency = Some(value.unwrap_or("").to_string());
                    },
                "e-commerce-tracking" => {
                        request.e_commerce_tracking = Some(arg_from_str(value.unwrap_or("false"), err, "e-commerce-tracking", "boolean"));
                    },
                "web-property-id" => {
                        request.web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "timezone" => {
                        request.timezone = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                "strip-site-search-category-parameters" => {
                        request.strip_site_search_category_parameters = Some(arg_from_str(value.unwrap_or("false"), err, "strip-site-search-category-parameters", "boolean"));
                    },
                "site-search-category-parameters" => {
                        request.site_search_category_parameters = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "exclude-query-parameters" => {
                        request.exclude_query_parameters = Some(value.unwrap_or("").to_string());
                    },
                "internal-web-property-id" => {
                        request.internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "child-link.href" => {
                        request_child_link_init(&mut request);
                        request.child_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "child-link.type" => {
                        request_child_link_init(&mut request);
                        request.child_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "enhanced-e-commerce-tracking" => {
                        request_child_link_init(&mut request);
                        request.enhanced_e_commerce_tracking = Some(arg_from_str(value.unwrap_or("false"), err, "enhanced-e-commerce-tracking", "boolean"));
                    },
                "permissions.effective" => {
                        request_permissions_init(&mut request);
                        if request.permissions.as_mut().unwrap().effective.is_none() {
                           request.permissions.as_mut().unwrap().effective = Some(Default::default());
                        }
                                        request.permissions.as_mut().unwrap().effective.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "default-page" => {
                        request_permissions_init(&mut request);
                        request.default_page = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_permissions_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "strip-site-search-query-parameters" => {
                        request_permissions_init(&mut request);
                        request.strip_site_search_query_parameters = Some(arg_from_str(value.unwrap_or("false"), err, "strip-site-search-query-parameters", "boolean"));
                    },
                "name" => {
                        request_permissions_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request_permissions_init(&mut request);
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "site-search-query-parameters" => {
                        request_permissions_init(&mut request);
                        request.site_search_query_parameters = Some(value.unwrap_or("").to_string());
                    },
                "website-url" => {
                        request_permissions_init(&mut request);
                        request.website_url = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_parent_link_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "child-link", "created", "currency", "default-page", "e-commerce-tracking", "effective", "enhanced-e-commerce-tracking", "exclude-query-parameters", "href", "id", "internal-web-property-id", "kind", "name", "parent-link", "permissions", "self-link", "site-search-category-parameters", "site-search-query-parameters", "strip-site-search-category-parameters", "strip-site-search-query-parameters", "timezone", "type", "updated", "web-property-id", "website-url"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.management().profiles_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""));
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

    fn _management_segments_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().segments_list();
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
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
                                                Vec::new() + &self.gp + &["start-index", "max-results"]
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

    fn _management_unsampled_reports_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().unsampled_reports_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""), opt.value_of("unsampled-report-id").unwrap_or(""));
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

    fn _management_unsampled_reports_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::UnsampledReport::default();
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
            fn request_cloud_storage_download_details_init(request: &mut api::UnsampledReport) {
                if request.cloud_storage_download_details.is_none() {
                    request.cloud_storage_download_details = Some(Default::default());
                }
            }
            
            fn request_drive_download_details_init(request: &mut api::UnsampledReport) {
                if request.drive_download_details.is_none() {
                    request.drive_download_details = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "status" => {
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "cloud-storage-download-details.bucket-id" => {
                        request_cloud_storage_download_details_init(&mut request);
                        request.cloud_storage_download_details.as_mut().unwrap().bucket_id = Some(value.unwrap_or("").to_string());
                    },
                "cloud-storage-download-details.object-id" => {
                        request_cloud_storage_download_details_init(&mut request);
                        request.cloud_storage_download_details.as_mut().unwrap().object_id = Some(value.unwrap_or("").to_string());
                    },
                "download-type" => {
                        request_cloud_storage_download_details_init(&mut request);
                        request.download_type = Some(value.unwrap_or("").to_string());
                    },
                "dimensions" => {
                        request_cloud_storage_download_details_init(&mut request);
                        request.dimensions = Some(value.unwrap_or("").to_string());
                    },
                "start-date" => {
                        request_cloud_storage_download_details_init(&mut request);
                        request.start_date = Some(value.unwrap_or("").to_string());
                    },
                "end-date" => {
                        request_cloud_storage_download_details_init(&mut request);
                        request.end_date = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request_cloud_storage_download_details_init(&mut request);
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_cloud_storage_download_details_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "drive-download-details.document-id" => {
                        request_drive_download_details_init(&mut request);
                        request.drive_download_details.as_mut().unwrap().document_id = Some(value.unwrap_or("").to_string());
                    },
                "profile-id" => {
                        request_drive_download_details_init(&mut request);
                        request.profile_id = Some(value.unwrap_or("").to_string());
                    },
                "metrics" => {
                        request_drive_download_details_init(&mut request);
                        request.metrics = Some(value.unwrap_or("").to_string());
                    },
                "filters" => {
                        request_drive_download_details_init(&mut request);
                        request.filters = Some(value.unwrap_or("").to_string());
                    },
                "web-property-id" => {
                        request_drive_download_details_init(&mut request);
                        request.web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "title" => {
                        request_drive_download_details_init(&mut request);
                        request.title = Some(value.unwrap_or("").to_string());
                    },
                "segment" => {
                        request_drive_download_details_init(&mut request);
                        request.segment = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_drive_download_details_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_drive_download_details_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_drive_download_details_init(&mut request);
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "bucket-id", "cloud-storage-download-details", "created", "dimensions", "document-id", "download-type", "drive-download-details", "end-date", "filters", "id", "kind", "metrics", "object-id", "profile-id", "segment", "self-link", "start-date", "status", "title", "updated", "web-property-id"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.management().unsampled_reports_insert(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""));
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

    fn _management_unsampled_reports_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().unsampled_reports_list(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("profile-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
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
                                                Vec::new() + &self.gp + &["start-index", "max-results"]
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

    fn _management_uploads_delete_upload_data(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::AnalyticsDataimportDeleteUploadDataRequest::default();
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
            match &temp_cursor.to_string()[..] {
                "custom-data-import-uids" => {
                        if request.custom_data_import_uids.is_none() {
                           request.custom_data_import_uids = Some(Default::default());
                        }
                                        request.custom_data_import_uids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["custom-data-import-uids"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.management().uploads_delete_upload_data(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("custom-data-source-id").unwrap_or(""));
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

    fn _management_uploads_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().uploads_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("custom-data-source-id").unwrap_or(""), opt.value_of("upload-id").unwrap_or(""));
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

    fn _management_uploads_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().uploads_list(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("custom-data-source-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
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
                                                Vec::new() + &self.gp + &["start-index", "max-results"]
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

    fn _management_uploads_upload_data(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().uploads_upload_data(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("custom-data-source-id").unwrap_or(""));
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

    fn _management_web_property_ad_words_links_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().web_property_ad_words_links_delete(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("web-property-ad-words-link-id").unwrap_or(""));
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

    fn _management_web_property_ad_words_links_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().web_property_ad_words_links_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("web-property-ad-words-link-id").unwrap_or(""));
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

    fn _management_web_property_ad_words_links_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::EntityAdWordsLink::default();
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
            fn request_entity_init(request: &mut api::EntityAdWordsLink) {
                if request.entity.is_none() {
                    request.entity = Some(Default::default());
                }
            }
            
            fn request_entity_web_property_ref_init(request: &mut api::EntityAdWordsLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().web_property_ref.is_none() {
                    request.entity.as_mut().unwrap().web_property_ref = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "profile-ids" => {
                        if request.profile_ids.is_none() {
                           request.profile_ids = Some(Default::default());
                        }
                                        request.profile_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.kind" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.name" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.internal-web-property-id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.href" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.account-id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_entity_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_entity_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "entity", "href", "id", "internal-web-property-id", "kind", "name", "profile-ids", "self-link", "web-property-ref"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.management().web_property_ad_words_links_insert(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""));
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

    fn _management_web_property_ad_words_links_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().web_property_ad_words_links_list(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
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
                                                Vec::new() + &self.gp + &["start-index", "max-results"]
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

    fn _management_web_property_ad_words_links_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::EntityAdWordsLink::default();
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
            fn request_entity_init(request: &mut api::EntityAdWordsLink) {
                if request.entity.is_none() {
                    request.entity = Some(Default::default());
                }
            }
            
            fn request_entity_web_property_ref_init(request: &mut api::EntityAdWordsLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().web_property_ref.is_none() {
                    request.entity.as_mut().unwrap().web_property_ref = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "profile-ids" => {
                        if request.profile_ids.is_none() {
                           request.profile_ids = Some(Default::default());
                        }
                                        request.profile_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.kind" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.name" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.internal-web-property-id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.href" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.account-id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_entity_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_entity_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "entity", "href", "id", "internal-web-property-id", "kind", "name", "profile-ids", "self-link", "web-property-ref"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.management().web_property_ad_words_links_patch(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("web-property-ad-words-link-id").unwrap_or(""));
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

    fn _management_web_property_ad_words_links_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::EntityAdWordsLink::default();
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
            fn request_entity_init(request: &mut api::EntityAdWordsLink) {
                if request.entity.is_none() {
                    request.entity = Some(Default::default());
                }
            }
            
            fn request_entity_web_property_ref_init(request: &mut api::EntityAdWordsLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().web_property_ref.is_none() {
                    request.entity.as_mut().unwrap().web_property_ref = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "profile-ids" => {
                        if request.profile_ids.is_none() {
                           request.profile_ids = Some(Default::default());
                        }
                                        request.profile_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.kind" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.name" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.internal-web-property-id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.href" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.account-id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_entity_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_entity_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "entity", "href", "id", "internal-web-property-id", "kind", "name", "profile-ids", "self-link", "web-property-ref"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.management().web_property_ad_words_links_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("web-property-ad-words-link-id").unwrap_or(""));
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

    fn _management_webproperties_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().webproperties_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""));
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

    fn _management_webproperties_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Webproperty::default();
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
            fn request_child_link_init(request: &mut api::Webproperty) {
                if request.child_link.is_none() {
                    request.child_link = Some(Default::default());
                }
            }
            
            fn request_parent_link_init(request: &mut api::Webproperty) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            fn request_permissions_init(request: &mut api::Webproperty) {
                if request.permissions.is_none() {
                    request.permissions = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "website-url" => {
                        request.website_url = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "default-profile-id" => {
                        request.default_profile_id = Some(value.unwrap_or("").to_string());
                    },
                "level" => {
                        request.level = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "profile-count" => {
                        request.profile_count = Some(arg_from_str(value.unwrap_or("-0"), err, "profile-count", "integer"));
                    },
                "internal-web-property-id" => {
                        request.internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "child-link.href" => {
                        request_child_link_init(&mut request);
                        request.child_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "child-link.type" => {
                        request_child_link_init(&mut request);
                        request.child_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "industry-vertical" => {
                        request_child_link_init(&mut request);
                        request.industry_vertical = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "permissions.effective" => {
                        request_permissions_init(&mut request);
                        if request.permissions.as_mut().unwrap().effective.is_none() {
                           request.permissions.as_mut().unwrap().effective = Some(Default::default());
                        }
                                        request.permissions.as_mut().unwrap().effective.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_permissions_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_permissions_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_permissions_init(&mut request);
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "child-link", "created", "default-profile-id", "effective", "href", "id", "industry-vertical", "internal-web-property-id", "kind", "level", "name", "parent-link", "permissions", "profile-count", "self-link", "type", "updated", "website-url"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.management().webproperties_insert(request, opt.value_of("account-id").unwrap_or(""));
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

    fn _management_webproperties_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().webproperties_list(opt.value_of("account-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
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
                                                Vec::new() + &self.gp + &["start-index", "max-results"]
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

    fn _management_webproperties_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Webproperty::default();
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
            fn request_child_link_init(request: &mut api::Webproperty) {
                if request.child_link.is_none() {
                    request.child_link = Some(Default::default());
                }
            }
            
            fn request_parent_link_init(request: &mut api::Webproperty) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            fn request_permissions_init(request: &mut api::Webproperty) {
                if request.permissions.is_none() {
                    request.permissions = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "website-url" => {
                        request.website_url = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "default-profile-id" => {
                        request.default_profile_id = Some(value.unwrap_or("").to_string());
                    },
                "level" => {
                        request.level = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "profile-count" => {
                        request.profile_count = Some(arg_from_str(value.unwrap_or("-0"), err, "profile-count", "integer"));
                    },
                "internal-web-property-id" => {
                        request.internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "child-link.href" => {
                        request_child_link_init(&mut request);
                        request.child_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "child-link.type" => {
                        request_child_link_init(&mut request);
                        request.child_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "industry-vertical" => {
                        request_child_link_init(&mut request);
                        request.industry_vertical = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "permissions.effective" => {
                        request_permissions_init(&mut request);
                        if request.permissions.as_mut().unwrap().effective.is_none() {
                           request.permissions.as_mut().unwrap().effective = Some(Default::default());
                        }
                                        request.permissions.as_mut().unwrap().effective.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_permissions_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_permissions_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_permissions_init(&mut request);
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "child-link", "created", "default-profile-id", "effective", "href", "id", "industry-vertical", "internal-web-property-id", "kind", "level", "name", "parent-link", "permissions", "profile-count", "self-link", "type", "updated", "website-url"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.management().webproperties_patch(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""));
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

    fn _management_webproperties_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Webproperty::default();
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
            fn request_child_link_init(request: &mut api::Webproperty) {
                if request.child_link.is_none() {
                    request.child_link = Some(Default::default());
                }
            }
            
            fn request_parent_link_init(request: &mut api::Webproperty) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            fn request_permissions_init(request: &mut api::Webproperty) {
                if request.permissions.is_none() {
                    request.permissions = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "website-url" => {
                        request.website_url = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "default-profile-id" => {
                        request.default_profile_id = Some(value.unwrap_or("").to_string());
                    },
                "level" => {
                        request.level = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "profile-count" => {
                        request.profile_count = Some(arg_from_str(value.unwrap_or("-0"), err, "profile-count", "integer"));
                    },
                "internal-web-property-id" => {
                        request.internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "child-link.href" => {
                        request_child_link_init(&mut request);
                        request.child_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "child-link.type" => {
                        request_child_link_init(&mut request);
                        request.child_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "industry-vertical" => {
                        request_child_link_init(&mut request);
                        request.industry_vertical = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "permissions.effective" => {
                        request_permissions_init(&mut request);
                        if request.permissions.as_mut().unwrap().effective.is_none() {
                           request.permissions.as_mut().unwrap().effective = Some(Default::default());
                        }
                                        request.permissions.as_mut().unwrap().effective.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_permissions_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_permissions_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_permissions_init(&mut request);
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "child-link", "created", "default-profile-id", "effective", "href", "id", "industry-vertical", "internal-web-property-id", "kind", "level", "name", "parent-link", "permissions", "profile-count", "self-link", "type", "updated", "website-url"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.management().webproperties_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""));
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

    fn _management_webproperty_user_links_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().webproperty_user_links_delete(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("link-id").unwrap_or(""));
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

    fn _management_webproperty_user_links_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::EntityUserLink::default();
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
            fn request_entity_account_ref_init(request: &mut api::EntityUserLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().account_ref.is_none() {
                    request.entity.as_mut().unwrap().account_ref = Some(Default::default());
                }
            }
            
            fn request_entity_init(request: &mut api::EntityUserLink) {
                if request.entity.is_none() {
                    request.entity = Some(Default::default());
                }
            }
            
            fn request_entity_profile_ref_init(request: &mut api::EntityUserLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().profile_ref.is_none() {
                    request.entity.as_mut().unwrap().profile_ref = Some(Default::default());
                }
            }
            
            fn request_entity_web_property_ref_init(request: &mut api::EntityUserLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().web_property_ref.is_none() {
                    request.entity.as_mut().unwrap().web_property_ref = Some(Default::default());
                }
            }
            
            fn request_permissions_init(request: &mut api::EntityUserLink) {
                if request.permissions.is_none() {
                    request.permissions = Some(Default::default());
                }
            }
            
            fn request_user_ref_init(request: &mut api::EntityUserLink) {
                if request.user_ref.is_none() {
                    request.user_ref = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.kind" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.href" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.id" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.name" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.kind" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.name" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.internal-web-property-id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.href" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.web-property-id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.account-id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.kind" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.name" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.internal-web-property-id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.href" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.account-id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "user-ref.kind" => {
                        request_user_ref_init(&mut request);
                        request.user_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "user-ref.email" => {
                        request_user_ref_init(&mut request);
                        request.user_ref.as_mut().unwrap().email = Some(value.unwrap_or("").to_string());
                    },
                "user-ref.id" => {
                        request_user_ref_init(&mut request);
                        request.user_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_user_ref_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_user_ref_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "permissions.local" => {
                        request_permissions_init(&mut request);
                        if request.permissions.as_mut().unwrap().local.is_none() {
                           request.permissions.as_mut().unwrap().local = Some(Default::default());
                        }
                                        request.permissions.as_mut().unwrap().local.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "permissions.effective" => {
                        request_permissions_init(&mut request);
                        if request.permissions.as_mut().unwrap().effective.is_none() {
                           request.permissions.as_mut().unwrap().effective = Some(Default::default());
                        }
                                        request.permissions.as_mut().unwrap().effective.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "account-ref", "effective", "email", "entity", "href", "id", "internal-web-property-id", "kind", "local", "name", "permissions", "profile-ref", "self-link", "user-ref", "web-property-id", "web-property-ref"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.management().webproperty_user_links_insert(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""));
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

    fn _management_webproperty_user_links_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.management().webproperty_user_links_list(opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
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
                                                Vec::new() + &self.gp + &["start-index", "max-results"]
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

    fn _management_webproperty_user_links_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::EntityUserLink::default();
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
            fn request_entity_account_ref_init(request: &mut api::EntityUserLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().account_ref.is_none() {
                    request.entity.as_mut().unwrap().account_ref = Some(Default::default());
                }
            }
            
            fn request_entity_init(request: &mut api::EntityUserLink) {
                if request.entity.is_none() {
                    request.entity = Some(Default::default());
                }
            }
            
            fn request_entity_profile_ref_init(request: &mut api::EntityUserLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().profile_ref.is_none() {
                    request.entity.as_mut().unwrap().profile_ref = Some(Default::default());
                }
            }
            
            fn request_entity_web_property_ref_init(request: &mut api::EntityUserLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().web_property_ref.is_none() {
                    request.entity.as_mut().unwrap().web_property_ref = Some(Default::default());
                }
            }
            
            fn request_permissions_init(request: &mut api::EntityUserLink) {
                if request.permissions.is_none() {
                    request.permissions = Some(Default::default());
                }
            }
            
            fn request_user_ref_init(request: &mut api::EntityUserLink) {
                if request.user_ref.is_none() {
                    request.user_ref = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.kind" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.href" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.id" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.name" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.kind" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.name" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.internal-web-property-id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.href" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.web-property-id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.account-id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.kind" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.name" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.internal-web-property-id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.href" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.account-id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "user-ref.kind" => {
                        request_user_ref_init(&mut request);
                        request.user_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "user-ref.email" => {
                        request_user_ref_init(&mut request);
                        request.user_ref.as_mut().unwrap().email = Some(value.unwrap_or("").to_string());
                    },
                "user-ref.id" => {
                        request_user_ref_init(&mut request);
                        request.user_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_user_ref_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_user_ref_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "permissions.local" => {
                        request_permissions_init(&mut request);
                        if request.permissions.as_mut().unwrap().local.is_none() {
                           request.permissions.as_mut().unwrap().local = Some(Default::default());
                        }
                                        request.permissions.as_mut().unwrap().local.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "permissions.effective" => {
                        request_permissions_init(&mut request);
                        if request.permissions.as_mut().unwrap().effective.is_none() {
                           request.permissions.as_mut().unwrap().effective = Some(Default::default());
                        }
                                        request.permissions.as_mut().unwrap().effective.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "account-ref", "effective", "email", "entity", "href", "id", "internal-web-property-id", "kind", "local", "name", "permissions", "profile-ref", "self-link", "user-ref", "web-property-id", "web-property-ref"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.management().webproperty_user_links_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("web-property-id").unwrap_or(""), opt.value_of("link-id").unwrap_or(""));
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

    fn _metadata_columns_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.metadata().columns_list(opt.value_of("report-type").unwrap_or(""));
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

    fn _provisioning_create_account_ticket(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::AccountTicket::default();
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
            fn request_account_child_link_init(request: &mut api::AccountTicket) {
                request_account_init(request);
                if request.account.as_mut().unwrap().child_link.is_none() {
                    request.account.as_mut().unwrap().child_link = Some(Default::default());
                }
            }
            
            fn request_account_init(request: &mut api::AccountTicket) {
                if request.account.is_none() {
                    request.account = Some(Default::default());
                }
            }
            
            fn request_account_permissions_init(request: &mut api::AccountTicket) {
                request_account_init(request);
                if request.account.as_mut().unwrap().permissions.is_none() {
                    request.account.as_mut().unwrap().permissions = Some(Default::default());
                }
            }
            
            fn request_profile_child_link_init(request: &mut api::AccountTicket) {
                request_profile_init(request);
                if request.profile.as_mut().unwrap().child_link.is_none() {
                    request.profile.as_mut().unwrap().child_link = Some(Default::default());
                }
            }
            
            fn request_profile_init(request: &mut api::AccountTicket) {
                if request.profile.is_none() {
                    request.profile = Some(Default::default());
                }
            }
            
            fn request_profile_parent_link_init(request: &mut api::AccountTicket) {
                request_profile_init(request);
                if request.profile.as_mut().unwrap().parent_link.is_none() {
                    request.profile.as_mut().unwrap().parent_link = Some(Default::default());
                }
            }
            
            fn request_profile_permissions_init(request: &mut api::AccountTicket) {
                request_profile_init(request);
                if request.profile.as_mut().unwrap().permissions.is_none() {
                    request.profile.as_mut().unwrap().permissions = Some(Default::default());
                }
            }
            
            fn request_webproperty_child_link_init(request: &mut api::AccountTicket) {
                request_webproperty_init(request);
                if request.webproperty.as_mut().unwrap().child_link.is_none() {
                    request.webproperty.as_mut().unwrap().child_link = Some(Default::default());
                }
            }
            
            fn request_webproperty_init(request: &mut api::AccountTicket) {
                if request.webproperty.is_none() {
                    request.webproperty = Some(Default::default());
                }
            }
            
            fn request_webproperty_parent_link_init(request: &mut api::AccountTicket) {
                request_webproperty_init(request);
                if request.webproperty.as_mut().unwrap().parent_link.is_none() {
                    request.webproperty.as_mut().unwrap().parent_link = Some(Default::default());
                }
            }
            
            fn request_webproperty_permissions_init(request: &mut api::AccountTicket) {
                request_webproperty_init(request);
                if request.webproperty.as_mut().unwrap().permissions.is_none() {
                    request.webproperty.as_mut().unwrap().permissions = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "profile.currency" => {
                        request_profile_init(&mut request);
                        request.profile.as_mut().unwrap().currency = Some(value.unwrap_or("").to_string());
                    },
                "profile.e-commerce-tracking" => {
                        request_profile_init(&mut request);
                        request.profile.as_mut().unwrap().e_commerce_tracking = Some(arg_from_str(value.unwrap_or("false"), err, "profile.e-commerce-tracking", "boolean"));
                    },
                "profile.web-property-id" => {
                        request_profile_init(&mut request);
                        request.profile.as_mut().unwrap().web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "profile.timezone" => {
                        request_profile_init(&mut request);
                        request.profile.as_mut().unwrap().timezone = Some(value.unwrap_or("").to_string());
                    },
                "profile.id" => {
                        request_profile_init(&mut request);
                        request.profile.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "profile.account-id" => {
                        request_profile_init(&mut request);
                        request.profile.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "profile.strip-site-search-category-parameters" => {
                        request_profile_init(&mut request);
                        request.profile.as_mut().unwrap().strip_site_search_category_parameters = Some(arg_from_str(value.unwrap_or("false"), err, "profile.strip-site-search-category-parameters", "boolean"));
                    },
                "profile.site-search-category-parameters" => {
                        request_profile_init(&mut request);
                        request.profile.as_mut().unwrap().site_search_category_parameters = Some(value.unwrap_or("").to_string());
                    },
                "profile.type" => {
                        request_profile_init(&mut request);
                        request.profile.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "profile.updated" => {
                        request_profile_init(&mut request);
                        request.profile.as_mut().unwrap().updated = Some(value.unwrap_or("").to_string());
                    },
                "profile.exclude-query-parameters" => {
                        request_profile_init(&mut request);
                        request.profile.as_mut().unwrap().exclude_query_parameters = Some(value.unwrap_or("").to_string());
                    },
                "profile.internal-web-property-id" => {
                        request_profile_init(&mut request);
                        request.profile.as_mut().unwrap().internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "profile.child-link.href" => {
                        request_profile_child_link_init(&mut request);
                        request.profile.as_mut().unwrap().child_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "profile.child-link.type" => {
                        request_profile_child_link_init(&mut request);
                        request.profile.as_mut().unwrap().child_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "profile.enhanced-e-commerce-tracking" => {
                        request_profile_child_link_init(&mut request);
                        request.profile.as_mut().unwrap().enhanced_e_commerce_tracking = Some(arg_from_str(value.unwrap_or("false"), err, "profile.enhanced-e-commerce-tracking", "boolean"));
                    },
                "profile.permissions.effective" => {
                        request_profile_permissions_init(&mut request);
                        if request.profile.as_mut().unwrap().permissions.as_mut().unwrap().effective.is_none() {
                           request.profile.as_mut().unwrap().permissions.as_mut().unwrap().effective = Some(Default::default());
                        }
                                        request.profile.as_mut().unwrap().permissions.as_mut().unwrap().effective.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "profile.default-page" => {
                        request_profile_permissions_init(&mut request);
                        request.profile.as_mut().unwrap().default_page = Some(value.unwrap_or("").to_string());
                    },
                "profile.kind" => {
                        request_profile_permissions_init(&mut request);
                        request.profile.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "profile.strip-site-search-query-parameters" => {
                        request_profile_permissions_init(&mut request);
                        request.profile.as_mut().unwrap().strip_site_search_query_parameters = Some(arg_from_str(value.unwrap_or("false"), err, "profile.strip-site-search-query-parameters", "boolean"));
                    },
                "profile.name" => {
                        request_profile_permissions_init(&mut request);
                        request.profile.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "profile.created" => {
                        request_profile_permissions_init(&mut request);
                        request.profile.as_mut().unwrap().created = Some(value.unwrap_or("").to_string());
                    },
                "profile.site-search-query-parameters" => {
                        request_profile_permissions_init(&mut request);
                        request.profile.as_mut().unwrap().site_search_query_parameters = Some(value.unwrap_or("").to_string());
                    },
                "profile.website-url" => {
                        request_profile_permissions_init(&mut request);
                        request.profile.as_mut().unwrap().website_url = Some(value.unwrap_or("").to_string());
                    },
                "profile.parent-link.href" => {
                        request_profile_parent_link_init(&mut request);
                        request.profile.as_mut().unwrap().parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "profile.parent-link.type" => {
                        request_profile_parent_link_init(&mut request);
                        request.profile.as_mut().unwrap().parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "profile.self-link" => {
                        request_profile_parent_link_init(&mut request);
                        request.profile.as_mut().unwrap().self_link = Some(value.unwrap_or("").to_string());
                    },
                "account.kind" => {
                        request_account_init(&mut request);
                        request.account.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "account.name" => {
                        request_account_init(&mut request);
                        request.account.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "account.created" => {
                        request_account_init(&mut request);
                        request.account.as_mut().unwrap().created = Some(value.unwrap_or("").to_string());
                    },
                "account.updated" => {
                        request_account_init(&mut request);
                        request.account.as_mut().unwrap().updated = Some(value.unwrap_or("").to_string());
                    },
                "account.child-link.href" => {
                        request_account_child_link_init(&mut request);
                        request.account.as_mut().unwrap().child_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "account.child-link.type" => {
                        request_account_child_link_init(&mut request);
                        request.account.as_mut().unwrap().child_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "account.id" => {
                        request_account_child_link_init(&mut request);
                        request.account.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "account.self-link" => {
                        request_account_child_link_init(&mut request);
                        request.account.as_mut().unwrap().self_link = Some(value.unwrap_or("").to_string());
                    },
                "account.permissions.effective" => {
                        request_account_permissions_init(&mut request);
                        if request.account.as_mut().unwrap().permissions.as_mut().unwrap().effective.is_none() {
                           request.account.as_mut().unwrap().permissions.as_mut().unwrap().effective = Some(Default::default());
                        }
                                        request.account.as_mut().unwrap().permissions.as_mut().unwrap().effective.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "webproperty.website-url" => {
                        request_webproperty_init(&mut request);
                        request.webproperty.as_mut().unwrap().website_url = Some(value.unwrap_or("").to_string());
                    },
                "webproperty.updated" => {
                        request_webproperty_init(&mut request);
                        request.webproperty.as_mut().unwrap().updated = Some(value.unwrap_or("").to_string());
                    },
                "webproperty.name" => {
                        request_webproperty_init(&mut request);
                        request.webproperty.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "webproperty.created" => {
                        request_webproperty_init(&mut request);
                        request.webproperty.as_mut().unwrap().created = Some(value.unwrap_or("").to_string());
                    },
                "webproperty.default-profile-id" => {
                        request_webproperty_init(&mut request);
                        request.webproperty.as_mut().unwrap().default_profile_id = Some(value.unwrap_or("").to_string());
                    },
                "webproperty.level" => {
                        request_webproperty_init(&mut request);
                        request.webproperty.as_mut().unwrap().level = Some(value.unwrap_or("").to_string());
                    },
                "webproperty.kind" => {
                        request_webproperty_init(&mut request);
                        request.webproperty.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "webproperty.profile-count" => {
                        request_webproperty_init(&mut request);
                        request.webproperty.as_mut().unwrap().profile_count = Some(arg_from_str(value.unwrap_or("-0"), err, "webproperty.profile-count", "integer"));
                    },
                "webproperty.internal-web-property-id" => {
                        request_webproperty_init(&mut request);
                        request.webproperty.as_mut().unwrap().internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "webproperty.child-link.href" => {
                        request_webproperty_child_link_init(&mut request);
                        request.webproperty.as_mut().unwrap().child_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "webproperty.child-link.type" => {
                        request_webproperty_child_link_init(&mut request);
                        request.webproperty.as_mut().unwrap().child_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "webproperty.industry-vertical" => {
                        request_webproperty_child_link_init(&mut request);
                        request.webproperty.as_mut().unwrap().industry_vertical = Some(value.unwrap_or("").to_string());
                    },
                "webproperty.parent-link.href" => {
                        request_webproperty_parent_link_init(&mut request);
                        request.webproperty.as_mut().unwrap().parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "webproperty.parent-link.type" => {
                        request_webproperty_parent_link_init(&mut request);
                        request.webproperty.as_mut().unwrap().parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "webproperty.permissions.effective" => {
                        request_webproperty_permissions_init(&mut request);
                        if request.webproperty.as_mut().unwrap().permissions.as_mut().unwrap().effective.is_none() {
                           request.webproperty.as_mut().unwrap().permissions.as_mut().unwrap().effective = Some(Default::default());
                        }
                                        request.webproperty.as_mut().unwrap().permissions.as_mut().unwrap().effective.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "webproperty.id" => {
                        request_webproperty_permissions_init(&mut request);
                        request.webproperty.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "webproperty.self-link" => {
                        request_webproperty_permissions_init(&mut request);
                        request.webproperty.as_mut().unwrap().self_link = Some(value.unwrap_or("").to_string());
                    },
                "webproperty.account-id" => {
                        request_webproperty_permissions_init(&mut request);
                        request.webproperty.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_webproperty_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "redirect-uri" => {
                        request_webproperty_init(&mut request);
                        request.redirect_uri = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_webproperty_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account", "account-id", "child-link", "created", "currency", "default-page", "default-profile-id", "e-commerce-tracking", "effective", "enhanced-e-commerce-tracking", "exclude-query-parameters", "href", "id", "industry-vertical", "internal-web-property-id", "kind", "level", "name", "parent-link", "permissions", "profile", "profile-count", "redirect-uri", "self-link", "site-search-category-parameters", "site-search-query-parameters", "strip-site-search-category-parameters", "strip-site-search-query-parameters", "timezone", "type", "updated", "web-property-id", "webproperty", "website-url"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.provisioning().create_account_ticket(request);
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

    fn _doit(&self, dry_run: bool) -> Result<Result<(), DoitError>, Option<InvalidOptionsError>> {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Result<(), DoitError> = Ok(());
        let mut err_opt: Option<InvalidOptionsError> = None;
        match self.opt.subcommand() {
            ("data", Some(opt)) => {
                match opt.subcommand() {
                    ("ga-get", Some(opt)) => {
                        call_result = self._data_ga_get(opt, dry_run, &mut err);
                    },
                    ("mcf-get", Some(opt)) => {
                        call_result = self._data_mcf_get(opt, dry_run, &mut err);
                    },
                    ("realtime-get", Some(opt)) => {
                        call_result = self._data_realtime_get(opt, dry_run, &mut err);
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
                        call_result = self._management_account_summaries_list(opt, dry_run, &mut err);
                    },
                    ("account-user-links-delete", Some(opt)) => {
                        call_result = self._management_account_user_links_delete(opt, dry_run, &mut err);
                    },
                    ("account-user-links-insert", Some(opt)) => {
                        call_result = self._management_account_user_links_insert(opt, dry_run, &mut err);
                    },
                    ("account-user-links-list", Some(opt)) => {
                        call_result = self._management_account_user_links_list(opt, dry_run, &mut err);
                    },
                    ("account-user-links-update", Some(opt)) => {
                        call_result = self._management_account_user_links_update(opt, dry_run, &mut err);
                    },
                    ("accounts-list", Some(opt)) => {
                        call_result = self._management_accounts_list(opt, dry_run, &mut err);
                    },
                    ("custom-data-sources-list", Some(opt)) => {
                        call_result = self._management_custom_data_sources_list(opt, dry_run, &mut err);
                    },
                    ("custom-dimensions-get", Some(opt)) => {
                        call_result = self._management_custom_dimensions_get(opt, dry_run, &mut err);
                    },
                    ("custom-dimensions-insert", Some(opt)) => {
                        call_result = self._management_custom_dimensions_insert(opt, dry_run, &mut err);
                    },
                    ("custom-dimensions-list", Some(opt)) => {
                        call_result = self._management_custom_dimensions_list(opt, dry_run, &mut err);
                    },
                    ("custom-dimensions-patch", Some(opt)) => {
                        call_result = self._management_custom_dimensions_patch(opt, dry_run, &mut err);
                    },
                    ("custom-dimensions-update", Some(opt)) => {
                        call_result = self._management_custom_dimensions_update(opt, dry_run, &mut err);
                    },
                    ("custom-metrics-get", Some(opt)) => {
                        call_result = self._management_custom_metrics_get(opt, dry_run, &mut err);
                    },
                    ("custom-metrics-insert", Some(opt)) => {
                        call_result = self._management_custom_metrics_insert(opt, dry_run, &mut err);
                    },
                    ("custom-metrics-list", Some(opt)) => {
                        call_result = self._management_custom_metrics_list(opt, dry_run, &mut err);
                    },
                    ("custom-metrics-patch", Some(opt)) => {
                        call_result = self._management_custom_metrics_patch(opt, dry_run, &mut err);
                    },
                    ("custom-metrics-update", Some(opt)) => {
                        call_result = self._management_custom_metrics_update(opt, dry_run, &mut err);
                    },
                    ("experiments-delete", Some(opt)) => {
                        call_result = self._management_experiments_delete(opt, dry_run, &mut err);
                    },
                    ("experiments-get", Some(opt)) => {
                        call_result = self._management_experiments_get(opt, dry_run, &mut err);
                    },
                    ("experiments-insert", Some(opt)) => {
                        call_result = self._management_experiments_insert(opt, dry_run, &mut err);
                    },
                    ("experiments-list", Some(opt)) => {
                        call_result = self._management_experiments_list(opt, dry_run, &mut err);
                    },
                    ("experiments-patch", Some(opt)) => {
                        call_result = self._management_experiments_patch(opt, dry_run, &mut err);
                    },
                    ("experiments-update", Some(opt)) => {
                        call_result = self._management_experiments_update(opt, dry_run, &mut err);
                    },
                    ("filters-delete", Some(opt)) => {
                        call_result = self._management_filters_delete(opt, dry_run, &mut err);
                    },
                    ("filters-get", Some(opt)) => {
                        call_result = self._management_filters_get(opt, dry_run, &mut err);
                    },
                    ("filters-insert", Some(opt)) => {
                        call_result = self._management_filters_insert(opt, dry_run, &mut err);
                    },
                    ("filters-list", Some(opt)) => {
                        call_result = self._management_filters_list(opt, dry_run, &mut err);
                    },
                    ("filters-patch", Some(opt)) => {
                        call_result = self._management_filters_patch(opt, dry_run, &mut err);
                    },
                    ("filters-update", Some(opt)) => {
                        call_result = self._management_filters_update(opt, dry_run, &mut err);
                    },
                    ("goals-get", Some(opt)) => {
                        call_result = self._management_goals_get(opt, dry_run, &mut err);
                    },
                    ("goals-insert", Some(opt)) => {
                        call_result = self._management_goals_insert(opt, dry_run, &mut err);
                    },
                    ("goals-list", Some(opt)) => {
                        call_result = self._management_goals_list(opt, dry_run, &mut err);
                    },
                    ("goals-patch", Some(opt)) => {
                        call_result = self._management_goals_patch(opt, dry_run, &mut err);
                    },
                    ("goals-update", Some(opt)) => {
                        call_result = self._management_goals_update(opt, dry_run, &mut err);
                    },
                    ("profile-filter-links-delete", Some(opt)) => {
                        call_result = self._management_profile_filter_links_delete(opt, dry_run, &mut err);
                    },
                    ("profile-filter-links-get", Some(opt)) => {
                        call_result = self._management_profile_filter_links_get(opt, dry_run, &mut err);
                    },
                    ("profile-filter-links-insert", Some(opt)) => {
                        call_result = self._management_profile_filter_links_insert(opt, dry_run, &mut err);
                    },
                    ("profile-filter-links-list", Some(opt)) => {
                        call_result = self._management_profile_filter_links_list(opt, dry_run, &mut err);
                    },
                    ("profile-filter-links-patch", Some(opt)) => {
                        call_result = self._management_profile_filter_links_patch(opt, dry_run, &mut err);
                    },
                    ("profile-filter-links-update", Some(opt)) => {
                        call_result = self._management_profile_filter_links_update(opt, dry_run, &mut err);
                    },
                    ("profile-user-links-delete", Some(opt)) => {
                        call_result = self._management_profile_user_links_delete(opt, dry_run, &mut err);
                    },
                    ("profile-user-links-insert", Some(opt)) => {
                        call_result = self._management_profile_user_links_insert(opt, dry_run, &mut err);
                    },
                    ("profile-user-links-list", Some(opt)) => {
                        call_result = self._management_profile_user_links_list(opt, dry_run, &mut err);
                    },
                    ("profile-user-links-update", Some(opt)) => {
                        call_result = self._management_profile_user_links_update(opt, dry_run, &mut err);
                    },
                    ("profiles-delete", Some(opt)) => {
                        call_result = self._management_profiles_delete(opt, dry_run, &mut err);
                    },
                    ("profiles-get", Some(opt)) => {
                        call_result = self._management_profiles_get(opt, dry_run, &mut err);
                    },
                    ("profiles-insert", Some(opt)) => {
                        call_result = self._management_profiles_insert(opt, dry_run, &mut err);
                    },
                    ("profiles-list", Some(opt)) => {
                        call_result = self._management_profiles_list(opt, dry_run, &mut err);
                    },
                    ("profiles-patch", Some(opt)) => {
                        call_result = self._management_profiles_patch(opt, dry_run, &mut err);
                    },
                    ("profiles-update", Some(opt)) => {
                        call_result = self._management_profiles_update(opt, dry_run, &mut err);
                    },
                    ("segments-list", Some(opt)) => {
                        call_result = self._management_segments_list(opt, dry_run, &mut err);
                    },
                    ("unsampled-reports-get", Some(opt)) => {
                        call_result = self._management_unsampled_reports_get(opt, dry_run, &mut err);
                    },
                    ("unsampled-reports-insert", Some(opt)) => {
                        call_result = self._management_unsampled_reports_insert(opt, dry_run, &mut err);
                    },
                    ("unsampled-reports-list", Some(opt)) => {
                        call_result = self._management_unsampled_reports_list(opt, dry_run, &mut err);
                    },
                    ("uploads-delete-upload-data", Some(opt)) => {
                        call_result = self._management_uploads_delete_upload_data(opt, dry_run, &mut err);
                    },
                    ("uploads-get", Some(opt)) => {
                        call_result = self._management_uploads_get(opt, dry_run, &mut err);
                    },
                    ("uploads-list", Some(opt)) => {
                        call_result = self._management_uploads_list(opt, dry_run, &mut err);
                    },
                    ("uploads-upload-data", Some(opt)) => {
                        call_result = self._management_uploads_upload_data(opt, dry_run, &mut err);
                    },
                    ("web-property-ad-words-links-delete", Some(opt)) => {
                        call_result = self._management_web_property_ad_words_links_delete(opt, dry_run, &mut err);
                    },
                    ("web-property-ad-words-links-get", Some(opt)) => {
                        call_result = self._management_web_property_ad_words_links_get(opt, dry_run, &mut err);
                    },
                    ("web-property-ad-words-links-insert", Some(opt)) => {
                        call_result = self._management_web_property_ad_words_links_insert(opt, dry_run, &mut err);
                    },
                    ("web-property-ad-words-links-list", Some(opt)) => {
                        call_result = self._management_web_property_ad_words_links_list(opt, dry_run, &mut err);
                    },
                    ("web-property-ad-words-links-patch", Some(opt)) => {
                        call_result = self._management_web_property_ad_words_links_patch(opt, dry_run, &mut err);
                    },
                    ("web-property-ad-words-links-update", Some(opt)) => {
                        call_result = self._management_web_property_ad_words_links_update(opt, dry_run, &mut err);
                    },
                    ("webproperties-get", Some(opt)) => {
                        call_result = self._management_webproperties_get(opt, dry_run, &mut err);
                    },
                    ("webproperties-insert", Some(opt)) => {
                        call_result = self._management_webproperties_insert(opt, dry_run, &mut err);
                    },
                    ("webproperties-list", Some(opt)) => {
                        call_result = self._management_webproperties_list(opt, dry_run, &mut err);
                    },
                    ("webproperties-patch", Some(opt)) => {
                        call_result = self._management_webproperties_patch(opt, dry_run, &mut err);
                    },
                    ("webproperties-update", Some(opt)) => {
                        call_result = self._management_webproperties_update(opt, dry_run, &mut err);
                    },
                    ("webproperty-user-links-delete", Some(opt)) => {
                        call_result = self._management_webproperty_user_links_delete(opt, dry_run, &mut err);
                    },
                    ("webproperty-user-links-insert", Some(opt)) => {
                        call_result = self._management_webproperty_user_links_insert(opt, dry_run, &mut err);
                    },
                    ("webproperty-user-links-list", Some(opt)) => {
                        call_result = self._management_webproperty_user_links_list(opt, dry_run, &mut err);
                    },
                    ("webproperty-user-links-update", Some(opt)) => {
                        call_result = self._management_webproperty_user_links_update(opt, dry_run, &mut err);
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
                        call_result = self._metadata_columns_list(opt, dry_run, &mut err);
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
                        call_result = self._provisioning_create_account_ticket(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("provisioning".to_string()));
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

            match cmn::application_secret_from_directory(&config_dir, "analytics3-secret.json", 
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
                                          program_name: "analytics3",
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
            hub: api::Analytics::new(client, auth),
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
        
        ("management", "methods: 'account-summaries-list', 'account-user-links-delete', 'account-user-links-insert', 'account-user-links-list', 'account-user-links-update', 'accounts-list', 'custom-data-sources-list', 'custom-dimensions-get', 'custom-dimensions-insert', 'custom-dimensions-list', 'custom-dimensions-patch', 'custom-dimensions-update', 'custom-metrics-get', 'custom-metrics-insert', 'custom-metrics-list', 'custom-metrics-patch', 'custom-metrics-update', 'experiments-delete', 'experiments-get', 'experiments-insert', 'experiments-list', 'experiments-patch', 'experiments-update', 'filters-delete', 'filters-get', 'filters-insert', 'filters-list', 'filters-patch', 'filters-update', 'goals-get', 'goals-insert', 'goals-list', 'goals-patch', 'goals-update', 'profile-filter-links-delete', 'profile-filter-links-get', 'profile-filter-links-insert', 'profile-filter-links-list', 'profile-filter-links-patch', 'profile-filter-links-update', 'profile-user-links-delete', 'profile-user-links-insert', 'profile-user-links-list', 'profile-user-links-update', 'profiles-delete', 'profiles-get', 'profiles-insert', 'profiles-list', 'profiles-patch', 'profiles-update', 'segments-list', 'unsampled-reports-get', 'unsampled-reports-insert', 'unsampled-reports-list', 'uploads-delete-upload-data', 'uploads-get', 'uploads-list', 'uploads-upload-data', 'web-property-ad-words-links-delete', 'web-property-ad-words-links-get', 'web-property-ad-words-links-insert', 'web-property-ad-words-links-list', 'web-property-ad-words-links-patch', 'web-property-ad-words-links-update', 'webproperties-get', 'webproperties-insert', 'webproperties-list', 'webproperties-patch', 'webproperties-update', 'webproperty-user-links-delete', 'webproperty-user-links-insert', 'webproperty-user-links-list' and 'webproperty-user-links-update'", vec![
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
                    Some(r##"Returns a filters to which the user has access."##),
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
                    Some(r##"Updates an existing view (profile). This method supports patch semantics."##),
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
                    Some(r##"Updates an existing view (profile)."##),
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
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
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
            ("web-property-ad-words-links-delete",  
                    Some(r##"Deletes a web property-AdWords link."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_web-property-ad-words-links-delete",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"ID of the account which the given web property belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID to delete the AdWords link for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-ad-words-link-id"##),
                     None,
                     Some(r##"Web property AdWords link ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("web-property-ad-words-links-get",  
                    Some(r##"Returns a web property-AdWords link to which the user has access."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_web-property-ad-words-links-get",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"ID of the account which the given web property belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID to retrieve the AdWords link for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-ad-words-link-id"##),
                     None,
                     Some(r##"Web property-AdWords link ID."##),
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
                    Some(r##"Creates a webProperty-AdWords link."##),
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
                    Some(r##"Lists webProperty-AdWords links for a given web property."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_web-property-ad-words-links-list",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"ID of the account which the given web property belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID to retrieve the AdWords links for."##),
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
                    Some(r##"Updates an existing webProperty-AdWords link. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_web-property-ad-words-links-patch",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"ID of the account which the given web property belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID to retrieve the AdWords link for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-ad-words-link-id"##),
                     None,
                     Some(r##"Web property-AdWords link ID."##),
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
                    Some(r##"Updates an existing webProperty-AdWords link."##),
                    "Details at http://byron.github.io/google-apis-rs/google_analytics3_cli/management_web-property-ad-words-links-update",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"ID of the account which the given web property belongs to."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-id"##),
                     None,
                     Some(r##"Web property ID to retrieve the AdWords link for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"web-property-ad-words-link-id"##),
                     None,
                     Some(r##"Web property-AdWords link ID."##),
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
        
        ("provisioning", "methods: 'create-account-ticket'", vec![
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
            ]),
        
    ];
    
    let mut app = App::new("analytics3")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("0.2.0+20150305")
           .about("View and manage your Google Analytics data")
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