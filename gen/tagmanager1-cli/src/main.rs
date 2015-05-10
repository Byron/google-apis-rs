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
extern crate google_tagmanager1 as api;

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
    hub: api::TagManager<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n, 'a> Engine<'n, 'a> {
    fn _accounts_containers_create(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Container::default();
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
                "time-zone-id" => {
                        request.time_zone_id = Some(value.unwrap_or("").to_string());
                    },
                "enabled-built-in-variable" => {
                        if request.enabled_built_in_variable.is_none() {
                           request.enabled_built_in_variable = Some(Default::default());
                        }
                                        request.enabled_built_in_variable.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "time-zone-country-id" => {
                        request.time_zone_country_id = Some(arg_from_str(value.unwrap_or("-0"), err, "time-zone-country-id", "int64"));
                    },
                "public-id" => {
                        request.public_id = Some(value.unwrap_or("").to_string());
                    },
                "container-id" => {
                        request.container_id = Some(value.unwrap_or("").to_string());
                    },
                "domain-name" => {
                        if request.domain_name.is_none() {
                           request.domain_name = Some(Default::default());
                        }
                                        request.domain_name.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "notes" => {
                        request.notes = Some(value.unwrap_or("").to_string());
                    },
                "usage-context" => {
                        if request.usage_context.is_none() {
                           request.usage_context = Some(Default::default());
                        }
                                        request.usage_context.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "fingerprint" => {
                        request.fingerprint = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "container-id", "domain-name", "enabled-built-in-variable", "fingerprint", "name", "notes", "public-id", "time-zone-country-id", "time-zone-id", "usage-context"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.accounts().containers_create(request, opt.value_of("account-id").unwrap_or(""));
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

    fn _accounts_containers_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_delete(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""));
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

    fn _accounts_containers_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""));
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

    fn _accounts_containers_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_list(opt.value_of("account-id").unwrap_or(""));
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

    fn _accounts_containers_macros_create(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Macro::default();
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
                "schedule-start-ms" => {
                        request.schedule_start_ms = Some(value.unwrap_or("").to_string());
                    },
                "schedule-end-ms" => {
                        request.schedule_end_ms = Some(value.unwrap_or("").to_string());
                    },
                "macro-id" => {
                        request.macro_id = Some(value.unwrap_or("").to_string());
                    },
                "container-id" => {
                        request.container_id = Some(value.unwrap_or("").to_string());
                    },
                "notes" => {
                        request.notes = Some(value.unwrap_or("").to_string());
                    },
                "enabling-rule-id" => {
                        if request.enabling_rule_id.is_none() {
                           request.enabling_rule_id = Some(Default::default());
                        }
                                        request.enabling_rule_id.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "disabling-rule-id" => {
                        if request.disabling_rule_id.is_none() {
                           request.disabling_rule_id = Some(Default::default());
                        }
                                        request.disabling_rule_id.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "fingerprint" => {
                        request.fingerprint = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "container-id", "disabling-rule-id", "enabling-rule-id", "fingerprint", "macro-id", "name", "notes", "schedule-end-ms", "schedule-start-ms", "type"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.accounts().containers_macros_create(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""));
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

    fn _accounts_containers_macros_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_macros_delete(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("macro-id").unwrap_or(""));
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

    fn _accounts_containers_macros_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_macros_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("macro-id").unwrap_or(""));
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

    fn _accounts_containers_macros_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_macros_list(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""));
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

    fn _accounts_containers_macros_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Macro::default();
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
                "schedule-start-ms" => {
                        request.schedule_start_ms = Some(value.unwrap_or("").to_string());
                    },
                "schedule-end-ms" => {
                        request.schedule_end_ms = Some(value.unwrap_or("").to_string());
                    },
                "macro-id" => {
                        request.macro_id = Some(value.unwrap_or("").to_string());
                    },
                "container-id" => {
                        request.container_id = Some(value.unwrap_or("").to_string());
                    },
                "notes" => {
                        request.notes = Some(value.unwrap_or("").to_string());
                    },
                "enabling-rule-id" => {
                        if request.enabling_rule_id.is_none() {
                           request.enabling_rule_id = Some(Default::default());
                        }
                                        request.enabling_rule_id.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "disabling-rule-id" => {
                        if request.disabling_rule_id.is_none() {
                           request.disabling_rule_id = Some(Default::default());
                        }
                                        request.disabling_rule_id.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "fingerprint" => {
                        request.fingerprint = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "container-id", "disabling-rule-id", "enabling-rule-id", "fingerprint", "macro-id", "name", "notes", "schedule-end-ms", "schedule-start-ms", "type"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.accounts().containers_macros_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("macro-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["fingerprint"]
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

    fn _accounts_containers_rules_create(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Rule::default();
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
                "container-id" => {
                        request.container_id = Some(value.unwrap_or("").to_string());
                    },
                "rule-id" => {
                        request.rule_id = Some(value.unwrap_or("").to_string());
                    },
                "notes" => {
                        request.notes = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "fingerprint" => {
                        request.fingerprint = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "container-id", "fingerprint", "name", "notes", "rule-id"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.accounts().containers_rules_create(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""));
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

    fn _accounts_containers_rules_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_rules_delete(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("rule-id").unwrap_or(""));
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

    fn _accounts_containers_rules_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_rules_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("rule-id").unwrap_or(""));
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

    fn _accounts_containers_rules_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_rules_list(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""));
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

    fn _accounts_containers_rules_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Rule::default();
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
                "container-id" => {
                        request.container_id = Some(value.unwrap_or("").to_string());
                    },
                "rule-id" => {
                        request.rule_id = Some(value.unwrap_or("").to_string());
                    },
                "notes" => {
                        request.notes = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "fingerprint" => {
                        request.fingerprint = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "container-id", "fingerprint", "name", "notes", "rule-id"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.accounts().containers_rules_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("rule-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["fingerprint"]
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

    fn _accounts_containers_tags_create(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Tag::default();
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
            fn request_priority_init(request: &mut api::Tag) {
                if request.priority.is_none() {
                    request.priority = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "schedule-start-ms" => {
                        request.schedule_start_ms = Some(value.unwrap_or("").to_string());
                    },
                "schedule-end-ms" => {
                        request.schedule_end_ms = Some(value.unwrap_or("").to_string());
                    },
                "container-id" => {
                        request.container_id = Some(value.unwrap_or("").to_string());
                    },
                "blocking-trigger-id" => {
                        if request.blocking_trigger_id.is_none() {
                           request.blocking_trigger_id = Some(Default::default());
                        }
                                        request.blocking_trigger_id.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "tag-id" => {
                        request.tag_id = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "priority.type" => {
                        request_priority_init(&mut request);
                        request.priority.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "priority.value" => {
                        request_priority_init(&mut request);
                        request.priority.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "priority.key" => {
                        request_priority_init(&mut request);
                        request.priority.as_mut().unwrap().key = Some(value.unwrap_or("").to_string());
                    },
                "blocking-rule-id" => {
                        request_priority_init(&mut request);
                        if request.blocking_rule_id.is_none() {
                           request.blocking_rule_id = Some(Default::default());
                        }
                                        request.blocking_rule_id.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "live-only" => {
                        request_priority_init(&mut request);
                        request.live_only = Some(arg_from_str(value.unwrap_or("false"), err, "live-only", "boolean"));
                    },
                "fingerprint" => {
                        request_priority_init(&mut request);
                        request.fingerprint = Some(value.unwrap_or("").to_string());
                    },
                "firing-rule-id" => {
                        request_priority_init(&mut request);
                        if request.firing_rule_id.is_none() {
                           request.firing_rule_id = Some(Default::default());
                        }
                                        request.firing_rule_id.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "firing-trigger-id" => {
                        request_priority_init(&mut request);
                        if request.firing_trigger_id.is_none() {
                           request.firing_trigger_id = Some(Default::default());
                        }
                                        request.firing_trigger_id.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_priority_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "notes" => {
                        request_priority_init(&mut request);
                        request.notes = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_priority_init(&mut request);
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "blocking-rule-id", "blocking-trigger-id", "container-id", "fingerprint", "firing-rule-id", "firing-trigger-id", "key", "live-only", "name", "notes", "priority", "schedule-end-ms", "schedule-start-ms", "tag-id", "type", "value"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.accounts().containers_tags_create(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""));
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

    fn _accounts_containers_tags_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_tags_delete(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("tag-id").unwrap_or(""));
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

    fn _accounts_containers_tags_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_tags_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("tag-id").unwrap_or(""));
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

    fn _accounts_containers_tags_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_tags_list(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""));
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

    fn _accounts_containers_tags_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Tag::default();
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
            fn request_priority_init(request: &mut api::Tag) {
                if request.priority.is_none() {
                    request.priority = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "schedule-start-ms" => {
                        request.schedule_start_ms = Some(value.unwrap_or("").to_string());
                    },
                "schedule-end-ms" => {
                        request.schedule_end_ms = Some(value.unwrap_or("").to_string());
                    },
                "container-id" => {
                        request.container_id = Some(value.unwrap_or("").to_string());
                    },
                "blocking-trigger-id" => {
                        if request.blocking_trigger_id.is_none() {
                           request.blocking_trigger_id = Some(Default::default());
                        }
                                        request.blocking_trigger_id.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "tag-id" => {
                        request.tag_id = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "priority.type" => {
                        request_priority_init(&mut request);
                        request.priority.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "priority.value" => {
                        request_priority_init(&mut request);
                        request.priority.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "priority.key" => {
                        request_priority_init(&mut request);
                        request.priority.as_mut().unwrap().key = Some(value.unwrap_or("").to_string());
                    },
                "blocking-rule-id" => {
                        request_priority_init(&mut request);
                        if request.blocking_rule_id.is_none() {
                           request.blocking_rule_id = Some(Default::default());
                        }
                                        request.blocking_rule_id.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "live-only" => {
                        request_priority_init(&mut request);
                        request.live_only = Some(arg_from_str(value.unwrap_or("false"), err, "live-only", "boolean"));
                    },
                "fingerprint" => {
                        request_priority_init(&mut request);
                        request.fingerprint = Some(value.unwrap_or("").to_string());
                    },
                "firing-rule-id" => {
                        request_priority_init(&mut request);
                        if request.firing_rule_id.is_none() {
                           request.firing_rule_id = Some(Default::default());
                        }
                                        request.firing_rule_id.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "firing-trigger-id" => {
                        request_priority_init(&mut request);
                        if request.firing_trigger_id.is_none() {
                           request.firing_trigger_id = Some(Default::default());
                        }
                                        request.firing_trigger_id.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_priority_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "notes" => {
                        request_priority_init(&mut request);
                        request.notes = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_priority_init(&mut request);
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "blocking-rule-id", "blocking-trigger-id", "container-id", "fingerprint", "firing-rule-id", "firing-trigger-id", "key", "live-only", "name", "notes", "priority", "schedule-end-ms", "schedule-start-ms", "tag-id", "type", "value"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.accounts().containers_tags_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("tag-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["fingerprint"]
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

    fn _accounts_containers_triggers_create(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Trigger::default();
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
            fn request_check_validation_init(request: &mut api::Trigger) {
                if request.check_validation.is_none() {
                    request.check_validation = Some(Default::default());
                }
            }
            
            fn request_enable_all_videos_init(request: &mut api::Trigger) {
                if request.enable_all_videos.is_none() {
                    request.enable_all_videos = Some(Default::default());
                }
            }
            
            fn request_event_name_init(request: &mut api::Trigger) {
                if request.event_name.is_none() {
                    request.event_name = Some(Default::default());
                }
            }
            
            fn request_interval_init(request: &mut api::Trigger) {
                if request.interval.is_none() {
                    request.interval = Some(Default::default());
                }
            }
            
            fn request_limit_init(request: &mut api::Trigger) {
                if request.limit.is_none() {
                    request.limit = Some(Default::default());
                }
            }
            
            fn request_unique_trigger_id_init(request: &mut api::Trigger) {
                if request.unique_trigger_id.is_none() {
                    request.unique_trigger_id = Some(Default::default());
                }
            }
            
            fn request_video_percentage_list_init(request: &mut api::Trigger) {
                if request.video_percentage_list.is_none() {
                    request.video_percentage_list = Some(Default::default());
                }
            }
            
            fn request_wait_for_tags_init(request: &mut api::Trigger) {
                if request.wait_for_tags.is_none() {
                    request.wait_for_tags = Some(Default::default());
                }
            }
            
            fn request_wait_for_tags_timeout_init(request: &mut api::Trigger) {
                if request.wait_for_tags_timeout.is_none() {
                    request.wait_for_tags_timeout = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "video-percentage-list.type" => {
                        request_video_percentage_list_init(&mut request);
                        request.video_percentage_list.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "video-percentage-list.value" => {
                        request_video_percentage_list_init(&mut request);
                        request.video_percentage_list.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "video-percentage-list.key" => {
                        request_video_percentage_list_init(&mut request);
                        request.video_percentage_list.as_mut().unwrap().key = Some(value.unwrap_or("").to_string());
                    },
                "interval.type" => {
                        request_interval_init(&mut request);
                        request.interval.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "interval.value" => {
                        request_interval_init(&mut request);
                        request.interval.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "interval.key" => {
                        request_interval_init(&mut request);
                        request.interval.as_mut().unwrap().key = Some(value.unwrap_or("").to_string());
                    },
                "wait-for-tags.type" => {
                        request_wait_for_tags_init(&mut request);
                        request.wait_for_tags.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "wait-for-tags.value" => {
                        request_wait_for_tags_init(&mut request);
                        request.wait_for_tags.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "wait-for-tags.key" => {
                        request_wait_for_tags_init(&mut request);
                        request.wait_for_tags.as_mut().unwrap().key = Some(value.unwrap_or("").to_string());
                    },
                "container-id" => {
                        request_wait_for_tags_init(&mut request);
                        request.container_id = Some(value.unwrap_or("").to_string());
                    },
                "unique-trigger-id.type" => {
                        request_unique_trigger_id_init(&mut request);
                        request.unique_trigger_id.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "unique-trigger-id.value" => {
                        request_unique_trigger_id_init(&mut request);
                        request.unique_trigger_id.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "unique-trigger-id.key" => {
                        request_unique_trigger_id_init(&mut request);
                        request.unique_trigger_id.as_mut().unwrap().key = Some(value.unwrap_or("").to_string());
                    },
                "wait-for-tags-timeout.type" => {
                        request_wait_for_tags_timeout_init(&mut request);
                        request.wait_for_tags_timeout.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "wait-for-tags-timeout.value" => {
                        request_wait_for_tags_timeout_init(&mut request);
                        request.wait_for_tags_timeout.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "wait-for-tags-timeout.key" => {
                        request_wait_for_tags_timeout_init(&mut request);
                        request.wait_for_tags_timeout.as_mut().unwrap().key = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_wait_for_tags_timeout_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "event-name.type" => {
                        request_event_name_init(&mut request);
                        request.event_name.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "event-name.value" => {
                        request_event_name_init(&mut request);
                        request.event_name.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "event-name.key" => {
                        request_event_name_init(&mut request);
                        request.event_name.as_mut().unwrap().key = Some(value.unwrap_or("").to_string());
                    },
                "trigger-id" => {
                        request_event_name_init(&mut request);
                        request.trigger_id = Some(value.unwrap_or("").to_string());
                    },
                "limit.type" => {
                        request_limit_init(&mut request);
                        request.limit.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "limit.value" => {
                        request_limit_init(&mut request);
                        request.limit.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "limit.key" => {
                        request_limit_init(&mut request);
                        request.limit.as_mut().unwrap().key = Some(value.unwrap_or("").to_string());
                    },
                "check-validation.type" => {
                        request_check_validation_init(&mut request);
                        request.check_validation.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "check-validation.value" => {
                        request_check_validation_init(&mut request);
                        request.check_validation.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "check-validation.key" => {
                        request_check_validation_init(&mut request);
                        request.check_validation.as_mut().unwrap().key = Some(value.unwrap_or("").to_string());
                    },
                "fingerprint" => {
                        request_check_validation_init(&mut request);
                        request.fingerprint = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_check_validation_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "enable-all-videos.type" => {
                        request_enable_all_videos_init(&mut request);
                        request.enable_all_videos.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "enable-all-videos.value" => {
                        request_enable_all_videos_init(&mut request);
                        request.enable_all_videos.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "enable-all-videos.key" => {
                        request_enable_all_videos_init(&mut request);
                        request.enable_all_videos.as_mut().unwrap().key = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_enable_all_videos_init(&mut request);
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "check-validation", "container-id", "enable-all-videos", "event-name", "fingerprint", "interval", "key", "limit", "name", "trigger-id", "type", "unique-trigger-id", "value", "video-percentage-list", "wait-for-tags", "wait-for-tags-timeout"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.accounts().containers_triggers_create(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""));
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

    fn _accounts_containers_triggers_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_triggers_delete(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("trigger-id").unwrap_or(""));
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

    fn _accounts_containers_triggers_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_triggers_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("trigger-id").unwrap_or(""));
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

    fn _accounts_containers_triggers_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_triggers_list(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""));
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

    fn _accounts_containers_triggers_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Trigger::default();
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
            fn request_check_validation_init(request: &mut api::Trigger) {
                if request.check_validation.is_none() {
                    request.check_validation = Some(Default::default());
                }
            }
            
            fn request_enable_all_videos_init(request: &mut api::Trigger) {
                if request.enable_all_videos.is_none() {
                    request.enable_all_videos = Some(Default::default());
                }
            }
            
            fn request_event_name_init(request: &mut api::Trigger) {
                if request.event_name.is_none() {
                    request.event_name = Some(Default::default());
                }
            }
            
            fn request_interval_init(request: &mut api::Trigger) {
                if request.interval.is_none() {
                    request.interval = Some(Default::default());
                }
            }
            
            fn request_limit_init(request: &mut api::Trigger) {
                if request.limit.is_none() {
                    request.limit = Some(Default::default());
                }
            }
            
            fn request_unique_trigger_id_init(request: &mut api::Trigger) {
                if request.unique_trigger_id.is_none() {
                    request.unique_trigger_id = Some(Default::default());
                }
            }
            
            fn request_video_percentage_list_init(request: &mut api::Trigger) {
                if request.video_percentage_list.is_none() {
                    request.video_percentage_list = Some(Default::default());
                }
            }
            
            fn request_wait_for_tags_init(request: &mut api::Trigger) {
                if request.wait_for_tags.is_none() {
                    request.wait_for_tags = Some(Default::default());
                }
            }
            
            fn request_wait_for_tags_timeout_init(request: &mut api::Trigger) {
                if request.wait_for_tags_timeout.is_none() {
                    request.wait_for_tags_timeout = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "video-percentage-list.type" => {
                        request_video_percentage_list_init(&mut request);
                        request.video_percentage_list.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "video-percentage-list.value" => {
                        request_video_percentage_list_init(&mut request);
                        request.video_percentage_list.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "video-percentage-list.key" => {
                        request_video_percentage_list_init(&mut request);
                        request.video_percentage_list.as_mut().unwrap().key = Some(value.unwrap_or("").to_string());
                    },
                "interval.type" => {
                        request_interval_init(&mut request);
                        request.interval.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "interval.value" => {
                        request_interval_init(&mut request);
                        request.interval.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "interval.key" => {
                        request_interval_init(&mut request);
                        request.interval.as_mut().unwrap().key = Some(value.unwrap_or("").to_string());
                    },
                "wait-for-tags.type" => {
                        request_wait_for_tags_init(&mut request);
                        request.wait_for_tags.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "wait-for-tags.value" => {
                        request_wait_for_tags_init(&mut request);
                        request.wait_for_tags.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "wait-for-tags.key" => {
                        request_wait_for_tags_init(&mut request);
                        request.wait_for_tags.as_mut().unwrap().key = Some(value.unwrap_or("").to_string());
                    },
                "container-id" => {
                        request_wait_for_tags_init(&mut request);
                        request.container_id = Some(value.unwrap_or("").to_string());
                    },
                "unique-trigger-id.type" => {
                        request_unique_trigger_id_init(&mut request);
                        request.unique_trigger_id.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "unique-trigger-id.value" => {
                        request_unique_trigger_id_init(&mut request);
                        request.unique_trigger_id.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "unique-trigger-id.key" => {
                        request_unique_trigger_id_init(&mut request);
                        request.unique_trigger_id.as_mut().unwrap().key = Some(value.unwrap_or("").to_string());
                    },
                "wait-for-tags-timeout.type" => {
                        request_wait_for_tags_timeout_init(&mut request);
                        request.wait_for_tags_timeout.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "wait-for-tags-timeout.value" => {
                        request_wait_for_tags_timeout_init(&mut request);
                        request.wait_for_tags_timeout.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "wait-for-tags-timeout.key" => {
                        request_wait_for_tags_timeout_init(&mut request);
                        request.wait_for_tags_timeout.as_mut().unwrap().key = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_wait_for_tags_timeout_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "event-name.type" => {
                        request_event_name_init(&mut request);
                        request.event_name.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "event-name.value" => {
                        request_event_name_init(&mut request);
                        request.event_name.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "event-name.key" => {
                        request_event_name_init(&mut request);
                        request.event_name.as_mut().unwrap().key = Some(value.unwrap_or("").to_string());
                    },
                "trigger-id" => {
                        request_event_name_init(&mut request);
                        request.trigger_id = Some(value.unwrap_or("").to_string());
                    },
                "limit.type" => {
                        request_limit_init(&mut request);
                        request.limit.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "limit.value" => {
                        request_limit_init(&mut request);
                        request.limit.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "limit.key" => {
                        request_limit_init(&mut request);
                        request.limit.as_mut().unwrap().key = Some(value.unwrap_or("").to_string());
                    },
                "check-validation.type" => {
                        request_check_validation_init(&mut request);
                        request.check_validation.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "check-validation.value" => {
                        request_check_validation_init(&mut request);
                        request.check_validation.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "check-validation.key" => {
                        request_check_validation_init(&mut request);
                        request.check_validation.as_mut().unwrap().key = Some(value.unwrap_or("").to_string());
                    },
                "fingerprint" => {
                        request_check_validation_init(&mut request);
                        request.fingerprint = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_check_validation_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "enable-all-videos.type" => {
                        request_enable_all_videos_init(&mut request);
                        request.enable_all_videos.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "enable-all-videos.value" => {
                        request_enable_all_videos_init(&mut request);
                        request.enable_all_videos.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "enable-all-videos.key" => {
                        request_enable_all_videos_init(&mut request);
                        request.enable_all_videos.as_mut().unwrap().key = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_enable_all_videos_init(&mut request);
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "check-validation", "container-id", "enable-all-videos", "event-name", "fingerprint", "interval", "key", "limit", "name", "trigger-id", "type", "unique-trigger-id", "value", "video-percentage-list", "wait-for-tags", "wait-for-tags-timeout"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.accounts().containers_triggers_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("trigger-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["fingerprint"]
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

    fn _accounts_containers_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Container::default();
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
                "time-zone-id" => {
                        request.time_zone_id = Some(value.unwrap_or("").to_string());
                    },
                "enabled-built-in-variable" => {
                        if request.enabled_built_in_variable.is_none() {
                           request.enabled_built_in_variable = Some(Default::default());
                        }
                                        request.enabled_built_in_variable.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "time-zone-country-id" => {
                        request.time_zone_country_id = Some(arg_from_str(value.unwrap_or("-0"), err, "time-zone-country-id", "int64"));
                    },
                "public-id" => {
                        request.public_id = Some(value.unwrap_or("").to_string());
                    },
                "container-id" => {
                        request.container_id = Some(value.unwrap_or("").to_string());
                    },
                "domain-name" => {
                        if request.domain_name.is_none() {
                           request.domain_name = Some(Default::default());
                        }
                                        request.domain_name.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "notes" => {
                        request.notes = Some(value.unwrap_or("").to_string());
                    },
                "usage-context" => {
                        if request.usage_context.is_none() {
                           request.usage_context = Some(Default::default());
                        }
                                        request.usage_context.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "fingerprint" => {
                        request.fingerprint = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "container-id", "domain-name", "enabled-built-in-variable", "fingerprint", "name", "notes", "public-id", "time-zone-country-id", "time-zone-id", "usage-context"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.accounts().containers_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["fingerprint"]
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

    fn _accounts_containers_variables_create(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Variable::default();
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
                "schedule-start-ms" => {
                        request.schedule_start_ms = Some(value.unwrap_or("").to_string());
                    },
                "schedule-end-ms" => {
                        request.schedule_end_ms = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "variable-id" => {
                        request.variable_id = Some(value.unwrap_or("").to_string());
                    },
                "notes" => {
                        request.notes = Some(value.unwrap_or("").to_string());
                    },
                "enabling-trigger-id" => {
                        if request.enabling_trigger_id.is_none() {
                           request.enabling_trigger_id = Some(Default::default());
                        }
                                        request.enabling_trigger_id.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "fingerprint" => {
                        request.fingerprint = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "disabling-trigger-id" => {
                        if request.disabling_trigger_id.is_none() {
                           request.disabling_trigger_id = Some(Default::default());
                        }
                                        request.disabling_trigger_id.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "container-id" => {
                        request.container_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "container-id", "disabling-trigger-id", "enabling-trigger-id", "fingerprint", "name", "notes", "schedule-end-ms", "schedule-start-ms", "type", "variable-id"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.accounts().containers_variables_create(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""));
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

    fn _accounts_containers_variables_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_variables_delete(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("variable-id").unwrap_or(""));
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

    fn _accounts_containers_variables_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_variables_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("variable-id").unwrap_or(""));
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

    fn _accounts_containers_variables_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_variables_list(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""));
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

    fn _accounts_containers_variables_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Variable::default();
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
                "schedule-start-ms" => {
                        request.schedule_start_ms = Some(value.unwrap_or("").to_string());
                    },
                "schedule-end-ms" => {
                        request.schedule_end_ms = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "variable-id" => {
                        request.variable_id = Some(value.unwrap_or("").to_string());
                    },
                "notes" => {
                        request.notes = Some(value.unwrap_or("").to_string());
                    },
                "enabling-trigger-id" => {
                        if request.enabling_trigger_id.is_none() {
                           request.enabling_trigger_id = Some(Default::default());
                        }
                                        request.enabling_trigger_id.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "fingerprint" => {
                        request.fingerprint = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "disabling-trigger-id" => {
                        if request.disabling_trigger_id.is_none() {
                           request.disabling_trigger_id = Some(Default::default());
                        }
                                        request.disabling_trigger_id.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "container-id" => {
                        request.container_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "container-id", "disabling-trigger-id", "enabling-trigger-id", "fingerprint", "name", "notes", "schedule-end-ms", "schedule-start-ms", "type", "variable-id"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.accounts().containers_variables_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("variable-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["fingerprint"]
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

    fn _accounts_containers_versions_create(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::CreateContainerVersionRequestVersionOptions::default();
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
                "notes" => {
                        request.notes = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "quick-preview" => {
                        request.quick_preview = Some(arg_from_str(value.unwrap_or("false"), err, "quick-preview", "boolean"));
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["name", "notes", "quick-preview"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.accounts().containers_versions_create(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""));
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

    fn _accounts_containers_versions_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_versions_delete(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("container-version-id").unwrap_or(""));
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

    fn _accounts_containers_versions_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_versions_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("container-version-id").unwrap_or(""));
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

    fn _accounts_containers_versions_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_versions_list(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "headers" => {
                    call = call.headers(arg_from_str(value.unwrap_or("false"), err, "headers", "boolean"));
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
                                                Vec::new() + &self.gp + &["headers"]
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

    fn _accounts_containers_versions_publish(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_versions_publish(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("container-version-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["fingerprint"]
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

    fn _accounts_containers_versions_restore(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_versions_restore(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("container-version-id").unwrap_or(""));
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

    fn _accounts_containers_versions_undelete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().containers_versions_undelete(opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("container-version-id").unwrap_or(""));
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

    fn _accounts_containers_versions_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::ContainerVersion::default();
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
            fn request_container_init(request: &mut api::ContainerVersion) {
                if request.container.is_none() {
                    request.container = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "container.time-zone-id" => {
                        request_container_init(&mut request);
                        request.container.as_mut().unwrap().time_zone_id = Some(value.unwrap_or("").to_string());
                    },
                "container.enabled-built-in-variable" => {
                        request_container_init(&mut request);
                        if request.container.as_mut().unwrap().enabled_built_in_variable.is_none() {
                           request.container.as_mut().unwrap().enabled_built_in_variable = Some(Default::default());
                        }
                                        request.container.as_mut().unwrap().enabled_built_in_variable.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "container.time-zone-country-id" => {
                        request_container_init(&mut request);
                        request.container.as_mut().unwrap().time_zone_country_id = Some(arg_from_str(value.unwrap_or("-0"), err, "container.time-zone-country-id", "int64"));
                    },
                "container.public-id" => {
                        request_container_init(&mut request);
                        request.container.as_mut().unwrap().public_id = Some(value.unwrap_or("").to_string());
                    },
                "container.container-id" => {
                        request_container_init(&mut request);
                        request.container.as_mut().unwrap().container_id = Some(value.unwrap_or("").to_string());
                    },
                "container.domain-name" => {
                        request_container_init(&mut request);
                        if request.container.as_mut().unwrap().domain_name.is_none() {
                           request.container.as_mut().unwrap().domain_name = Some(Default::default());
                        }
                                        request.container.as_mut().unwrap().domain_name.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "container.notes" => {
                        request_container_init(&mut request);
                        request.container.as_mut().unwrap().notes = Some(value.unwrap_or("").to_string());
                    },
                "container.usage-context" => {
                        request_container_init(&mut request);
                        if request.container.as_mut().unwrap().usage_context.is_none() {
                           request.container.as_mut().unwrap().usage_context = Some(Default::default());
                        }
                                        request.container.as_mut().unwrap().usage_context.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "container.fingerprint" => {
                        request_container_init(&mut request);
                        request.container.as_mut().unwrap().fingerprint = Some(value.unwrap_or("").to_string());
                    },
                "container.account-id" => {
                        request_container_init(&mut request);
                        request.container.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "container.name" => {
                        request_container_init(&mut request);
                        request.container.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "container-id" => {
                        request_container_init(&mut request);
                        request.container_id = Some(value.unwrap_or("").to_string());
                    },
                "deleted" => {
                        request_container_init(&mut request);
                        request.deleted = Some(arg_from_str(value.unwrap_or("false"), err, "deleted", "boolean"));
                    },
                "notes" => {
                        request_container_init(&mut request);
                        request.notes = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_container_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "container-version-id" => {
                        request_container_init(&mut request);
                        request.container_version_id = Some(value.unwrap_or("").to_string());
                    },
                "fingerprint" => {
                        request_container_init(&mut request);
                        request.fingerprint = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_container_init(&mut request);
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "container", "container-id", "container-version-id", "deleted", "domain-name", "enabled-built-in-variable", "fingerprint", "name", "notes", "public-id", "time-zone-country-id", "time-zone-id", "usage-context"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.accounts().containers_versions_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("container-id").unwrap_or(""), opt.value_of("container-version-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["fingerprint"]
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

    fn _accounts_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().get(opt.value_of("account-id").unwrap_or(""));
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

    fn _accounts_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().list();
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

    fn _accounts_permissions_create(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::UserAccess::default();
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
            fn request_account_access_init(request: &mut api::UserAccess) {
                if request.account_access.is_none() {
                    request.account_access = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "account-access.permission" => {
                        request_account_access_init(&mut request);
                        if request.account_access.as_mut().unwrap().permission.is_none() {
                           request.account_access.as_mut().unwrap().permission = Some(Default::default());
                        }
                                        request.account_access.as_mut().unwrap().permission.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "email-address" => {
                        request_account_access_init(&mut request);
                        request.email_address = Some(value.unwrap_or("").to_string());
                    },
                "permission-id" => {
                        request_account_access_init(&mut request);
                        request.permission_id = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_account_access_init(&mut request);
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-access", "account-id", "email-address", "permission", "permission-id"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.accounts().permissions_create(request, opt.value_of("account-id").unwrap_or(""));
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

    fn _accounts_permissions_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().permissions_delete(opt.value_of("account-id").unwrap_or(""), opt.value_of("permission-id").unwrap_or(""));
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

    fn _accounts_permissions_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().permissions_get(opt.value_of("account-id").unwrap_or(""), opt.value_of("permission-id").unwrap_or(""));
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

    fn _accounts_permissions_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().permissions_list(opt.value_of("account-id").unwrap_or(""));
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

    fn _accounts_permissions_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::UserAccess::default();
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
            fn request_account_access_init(request: &mut api::UserAccess) {
                if request.account_access.is_none() {
                    request.account_access = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "account-access.permission" => {
                        request_account_access_init(&mut request);
                        if request.account_access.as_mut().unwrap().permission.is_none() {
                           request.account_access.as_mut().unwrap().permission = Some(Default::default());
                        }
                                        request.account_access.as_mut().unwrap().permission.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "email-address" => {
                        request_account_access_init(&mut request);
                        request.email_address = Some(value.unwrap_or("").to_string());
                    },
                "permission-id" => {
                        request_account_access_init(&mut request);
                        request.permission_id = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_account_access_init(&mut request);
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-access", "account-id", "email-address", "permission", "permission-id"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.accounts().permissions_update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("permission-id").unwrap_or(""));
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

    fn _accounts_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Account::default();
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
                "share-data" => {
                        request.share_data = Some(arg_from_str(value.unwrap_or("false"), err, "share-data", "boolean"));
                    },
                "fingerprint" => {
                        request.fingerprint = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "fingerprint", "name", "share-data"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.accounts().update(request, opt.value_of("account-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["fingerprint"]
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
            ("accounts", Some(opt)) => {
                match opt.subcommand() {
                    ("containers-create", Some(opt)) => {
                        call_result = self._accounts_containers_create(opt, dry_run, &mut err);
                    },
                    ("containers-delete", Some(opt)) => {
                        call_result = self._accounts_containers_delete(opt, dry_run, &mut err);
                    },
                    ("containers-get", Some(opt)) => {
                        call_result = self._accounts_containers_get(opt, dry_run, &mut err);
                    },
                    ("containers-list", Some(opt)) => {
                        call_result = self._accounts_containers_list(opt, dry_run, &mut err);
                    },
                    ("containers-macros-create", Some(opt)) => {
                        call_result = self._accounts_containers_macros_create(opt, dry_run, &mut err);
                    },
                    ("containers-macros-delete", Some(opt)) => {
                        call_result = self._accounts_containers_macros_delete(opt, dry_run, &mut err);
                    },
                    ("containers-macros-get", Some(opt)) => {
                        call_result = self._accounts_containers_macros_get(opt, dry_run, &mut err);
                    },
                    ("containers-macros-list", Some(opt)) => {
                        call_result = self._accounts_containers_macros_list(opt, dry_run, &mut err);
                    },
                    ("containers-macros-update", Some(opt)) => {
                        call_result = self._accounts_containers_macros_update(opt, dry_run, &mut err);
                    },
                    ("containers-rules-create", Some(opt)) => {
                        call_result = self._accounts_containers_rules_create(opt, dry_run, &mut err);
                    },
                    ("containers-rules-delete", Some(opt)) => {
                        call_result = self._accounts_containers_rules_delete(opt, dry_run, &mut err);
                    },
                    ("containers-rules-get", Some(opt)) => {
                        call_result = self._accounts_containers_rules_get(opt, dry_run, &mut err);
                    },
                    ("containers-rules-list", Some(opt)) => {
                        call_result = self._accounts_containers_rules_list(opt, dry_run, &mut err);
                    },
                    ("containers-rules-update", Some(opt)) => {
                        call_result = self._accounts_containers_rules_update(opt, dry_run, &mut err);
                    },
                    ("containers-tags-create", Some(opt)) => {
                        call_result = self._accounts_containers_tags_create(opt, dry_run, &mut err);
                    },
                    ("containers-tags-delete", Some(opt)) => {
                        call_result = self._accounts_containers_tags_delete(opt, dry_run, &mut err);
                    },
                    ("containers-tags-get", Some(opt)) => {
                        call_result = self._accounts_containers_tags_get(opt, dry_run, &mut err);
                    },
                    ("containers-tags-list", Some(opt)) => {
                        call_result = self._accounts_containers_tags_list(opt, dry_run, &mut err);
                    },
                    ("containers-tags-update", Some(opt)) => {
                        call_result = self._accounts_containers_tags_update(opt, dry_run, &mut err);
                    },
                    ("containers-triggers-create", Some(opt)) => {
                        call_result = self._accounts_containers_triggers_create(opt, dry_run, &mut err);
                    },
                    ("containers-triggers-delete", Some(opt)) => {
                        call_result = self._accounts_containers_triggers_delete(opt, dry_run, &mut err);
                    },
                    ("containers-triggers-get", Some(opt)) => {
                        call_result = self._accounts_containers_triggers_get(opt, dry_run, &mut err);
                    },
                    ("containers-triggers-list", Some(opt)) => {
                        call_result = self._accounts_containers_triggers_list(opt, dry_run, &mut err);
                    },
                    ("containers-triggers-update", Some(opt)) => {
                        call_result = self._accounts_containers_triggers_update(opt, dry_run, &mut err);
                    },
                    ("containers-update", Some(opt)) => {
                        call_result = self._accounts_containers_update(opt, dry_run, &mut err);
                    },
                    ("containers-variables-create", Some(opt)) => {
                        call_result = self._accounts_containers_variables_create(opt, dry_run, &mut err);
                    },
                    ("containers-variables-delete", Some(opt)) => {
                        call_result = self._accounts_containers_variables_delete(opt, dry_run, &mut err);
                    },
                    ("containers-variables-get", Some(opt)) => {
                        call_result = self._accounts_containers_variables_get(opt, dry_run, &mut err);
                    },
                    ("containers-variables-list", Some(opt)) => {
                        call_result = self._accounts_containers_variables_list(opt, dry_run, &mut err);
                    },
                    ("containers-variables-update", Some(opt)) => {
                        call_result = self._accounts_containers_variables_update(opt, dry_run, &mut err);
                    },
                    ("containers-versions-create", Some(opt)) => {
                        call_result = self._accounts_containers_versions_create(opt, dry_run, &mut err);
                    },
                    ("containers-versions-delete", Some(opt)) => {
                        call_result = self._accounts_containers_versions_delete(opt, dry_run, &mut err);
                    },
                    ("containers-versions-get", Some(opt)) => {
                        call_result = self._accounts_containers_versions_get(opt, dry_run, &mut err);
                    },
                    ("containers-versions-list", Some(opt)) => {
                        call_result = self._accounts_containers_versions_list(opt, dry_run, &mut err);
                    },
                    ("containers-versions-publish", Some(opt)) => {
                        call_result = self._accounts_containers_versions_publish(opt, dry_run, &mut err);
                    },
                    ("containers-versions-restore", Some(opt)) => {
                        call_result = self._accounts_containers_versions_restore(opt, dry_run, &mut err);
                    },
                    ("containers-versions-undelete", Some(opt)) => {
                        call_result = self._accounts_containers_versions_undelete(opt, dry_run, &mut err);
                    },
                    ("containers-versions-update", Some(opt)) => {
                        call_result = self._accounts_containers_versions_update(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._accounts_get(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._accounts_list(opt, dry_run, &mut err);
                    },
                    ("permissions-create", Some(opt)) => {
                        call_result = self._accounts_permissions_create(opt, dry_run, &mut err);
                    },
                    ("permissions-delete", Some(opt)) => {
                        call_result = self._accounts_permissions_delete(opt, dry_run, &mut err);
                    },
                    ("permissions-get", Some(opt)) => {
                        call_result = self._accounts_permissions_get(opt, dry_run, &mut err);
                    },
                    ("permissions-list", Some(opt)) => {
                        call_result = self._accounts_permissions_list(opt, dry_run, &mut err);
                    },
                    ("permissions-update", Some(opt)) => {
                        call_result = self._accounts_permissions_update(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._accounts_update(opt, dry_run, &mut err);
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
    fn new(opt: ArgMatches<'a, 'n>) -> Result<Engine<'a, 'n>, InvalidOptionsError> {
        let (config_dir, secret) = {
            let config_dir = match cmn::assure_config_dir_exists(opt.value_of("folder").unwrap_or("~/.google-service-cli")) {
                Err(e) => return Err(InvalidOptionsError::single(e, 3)),
                Ok(p) => p,
            };

            match cmn::application_secret_from_directory(&config_dir, "tagmanager1-secret.json", 
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
                                          program_name: "tagmanager1",
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
            hub: api::TagManager::new(client, auth),
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
        ("accounts", "methods: 'containers-create', 'containers-delete', 'containers-get', 'containers-list', 'containers-macros-create', 'containers-macros-delete', 'containers-macros-get', 'containers-macros-list', 'containers-macros-update', 'containers-rules-create', 'containers-rules-delete', 'containers-rules-get', 'containers-rules-list', 'containers-rules-update', 'containers-tags-create', 'containers-tags-delete', 'containers-tags-get', 'containers-tags-list', 'containers-tags-update', 'containers-triggers-create', 'containers-triggers-delete', 'containers-triggers-get', 'containers-triggers-list', 'containers-triggers-update', 'containers-update', 'containers-variables-create', 'containers-variables-delete', 'containers-variables-get', 'containers-variables-list', 'containers-variables-update', 'containers-versions-create', 'containers-versions-delete', 'containers-versions-get', 'containers-versions-list', 'containers-versions-publish', 'containers-versions-restore', 'containers-versions-undelete', 'containers-versions-update', 'get', 'list', 'permissions-create', 'permissions-delete', 'permissions-get', 'permissions-list', 'permissions-update' and 'update'", vec![
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
            ("containers-macros-create",  
                    Some(r##"Creates a GTM Macro."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-macros-create",
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
            ("containers-macros-delete",  
                    Some(r##"Deletes a GTM Macro."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-macros-delete",
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
        
                    (Some(r##"macro-id"##),
                     None,
                     Some(r##"The GTM Macro ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("containers-macros-get",  
                    Some(r##"Gets a GTM Macro."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-macros-get",
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
        
                    (Some(r##"macro-id"##),
                     None,
                     Some(r##"The GTM Macro ID."##),
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
            ("containers-macros-list",  
                    Some(r##"Lists all GTM Macros of a Container."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-macros-list",
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
            ("containers-macros-update",  
                    Some(r##"Updates a GTM Macro."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-macros-update",
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
        
                    (Some(r##"macro-id"##),
                     None,
                     Some(r##"The GTM Macro ID."##),
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
            ("containers-rules-create",  
                    Some(r##"Creates a GTM Rule."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-rules-create",
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
            ("containers-rules-delete",  
                    Some(r##"Deletes a GTM Rule."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-rules-delete",
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
        
                    (Some(r##"rule-id"##),
                     None,
                     Some(r##"The GTM Rule ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("containers-rules-get",  
                    Some(r##"Gets a GTM Rule."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-rules-get",
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
        
                    (Some(r##"rule-id"##),
                     None,
                     Some(r##"The GTM Rule ID."##),
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
            ("containers-rules-list",  
                    Some(r##"Lists all GTM Rules of a Container."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-rules-list",
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
            ("containers-rules-update",  
                    Some(r##"Updates a GTM Rule."##),
                    "Details at http://byron.github.io/google-apis-rs/google_tagmanager1_cli/accounts_containers-rules-update",
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
        
                    (Some(r##"rule-id"##),
                     None,
                     Some(r##"The GTM Rule ID."##),
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
                    Some(r##"Restores a Container Version. This will overwrite the container's current configuration (including its macros, rules and tags). The operation will not have any effect on the version that is being served (i.e. the published version)."##),
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
                     Some(r##"The GTM Account ID. @required tagmanager.accounts.permissions.list"##),
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
           .version("0.2.0+20150121")
           .about("API for accessing Tag Manager accounts and containers.")
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