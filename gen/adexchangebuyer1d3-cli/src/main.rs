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
extern crate google_adexchangebuyer1d3 as api;

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
    hub: api::AdExchangeBuyer<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n, 'a> Engine<'n, 'a> {
    fn _accounts_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let id: i32 = arg_from_str(&opt.value_of("id").unwrap_or(""), err, "<id>", "integer");
        let mut call = self.hub.accounts().get(id);
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

    fn _accounts_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "maximum-total-qps" => {
                        request.maximum_total_qps = Some(arg_from_str(value.unwrap_or("-0"), err, "maximum-total-qps", "integer"));
                    },
                "maximum-active-creatives" => {
                        request.maximum_active_creatives = Some(arg_from_str(value.unwrap_or("-0"), err, "maximum-active-creatives", "integer"));
                    },
                "cookie-matching-nid" => {
                        request.cookie_matching_nid = Some(value.unwrap_or("").to_string());
                    },
                "number-active-creatives" => {
                        request.number_active_creatives = Some(arg_from_str(value.unwrap_or("-0"), err, "number-active-creatives", "integer"));
                    },
                "id" => {
                        request.id = Some(arg_from_str(value.unwrap_or("-0"), err, "id", "integer"));
                    },
                "cookie-matching-url" => {
                        request.cookie_matching_url = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["cookie-matching-nid", "cookie-matching-url", "id", "kind", "maximum-active-creatives", "maximum-total-qps", "number-active-creatives"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let id: i32 = arg_from_str(&opt.value_of("id").unwrap_or(""), err, "<id>", "integer");
        let mut call = self.hub.accounts().patch(request, id);
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
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "maximum-total-qps" => {
                        request.maximum_total_qps = Some(arg_from_str(value.unwrap_or("-0"), err, "maximum-total-qps", "integer"));
                    },
                "maximum-active-creatives" => {
                        request.maximum_active_creatives = Some(arg_from_str(value.unwrap_or("-0"), err, "maximum-active-creatives", "integer"));
                    },
                "cookie-matching-nid" => {
                        request.cookie_matching_nid = Some(value.unwrap_or("").to_string());
                    },
                "number-active-creatives" => {
                        request.number_active_creatives = Some(arg_from_str(value.unwrap_or("-0"), err, "number-active-creatives", "integer"));
                    },
                "id" => {
                        request.id = Some(arg_from_str(value.unwrap_or("-0"), err, "id", "integer"));
                    },
                "cookie-matching-url" => {
                        request.cookie_matching_url = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["cookie-matching-nid", "cookie-matching-url", "id", "kind", "maximum-active-creatives", "maximum-total-qps", "number-active-creatives"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let id: i32 = arg_from_str(&opt.value_of("id").unwrap_or(""), err, "<id>", "integer");
        let mut call = self.hub.accounts().update(request, id);
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

    fn _billing_info_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let account_id: i32 = arg_from_str(&opt.value_of("account-id").unwrap_or(""), err, "<account-id>", "integer");
        let mut call = self.hub.billing_info().get(account_id);
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

    fn _billing_info_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.billing_info().list();
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

    fn _budget_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.budget().get(opt.value_of("account-id").unwrap_or(""), opt.value_of("billing-id").unwrap_or(""));
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

    fn _budget_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Budget::default();
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
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "budget-amount" => {
                        request.budget_amount = Some(value.unwrap_or("").to_string());
                    },
                "currency-code" => {
                        request.currency_code = Some(value.unwrap_or("").to_string());
                    },
                "billing-id" => {
                        request.billing_id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "billing-id", "budget-amount", "currency-code", "id", "kind"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.budget().patch(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("billing-id").unwrap_or(""));
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

    fn _budget_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Budget::default();
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
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "budget-amount" => {
                        request.budget_amount = Some(value.unwrap_or("").to_string());
                    },
                "currency-code" => {
                        request.currency_code = Some(value.unwrap_or("").to_string());
                    },
                "billing-id" => {
                        request.billing_id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "billing-id", "budget-amount", "currency-code", "id", "kind"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.budget().update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("billing-id").unwrap_or(""));
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

    fn _creatives_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let account_id: i32 = arg_from_str(&opt.value_of("account-id").unwrap_or(""), err, "<account-id>", "integer");
        let mut call = self.hub.creatives().get(account_id, opt.value_of("buyer-creative-id").unwrap_or(""));
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

    fn _creatives_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Creative::default();
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
            fn request_filtering_reasons_init(request: &mut api::Creative) {
                if request.filtering_reasons.is_none() {
                    request.filtering_reasons = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "product-categories" => {
                        if request.product_categories.is_none() {
                           request.product_categories = Some(Default::default());
                        }
                                        request.product_categories.as_mut().unwrap().push(arg_from_str(value.unwrap_or("-0"), err, "product-categories", "integer"));
                    },
                "status" => {
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "video-url" => {
                        request.video_url = Some(value.unwrap_or("").to_string());
                    },
                "agency-id" => {
                        request.agency_id = Some(value.unwrap_or("").to_string());
                    },
                "width" => {
                        request.width = Some(arg_from_str(value.unwrap_or("-0"), err, "width", "integer"));
                    },
                "attribute" => {
                        if request.attribute.is_none() {
                           request.attribute = Some(Default::default());
                        }
                                        request.attribute.as_mut().unwrap().push(arg_from_str(value.unwrap_or("-0"), err, "attribute", "integer"));
                    },
                "restricted-categories" => {
                        if request.restricted_categories.is_none() {
                           request.restricted_categories = Some(Default::default());
                        }
                                        request.restricted_categories.as_mut().unwrap().push(arg_from_str(value.unwrap_or("-0"), err, "restricted-categories", "integer"));
                    },
                "height" => {
                        request.height = Some(arg_from_str(value.unwrap_or("-0"), err, "height", "integer"));
                    },
                "advertiser-name" => {
                        request.advertiser_name = Some(value.unwrap_or("").to_string());
                    },
                "html-snippet" => {
                        request.html_snippet = Some(value.unwrap_or("").to_string());
                    },
                "advertiser-id" => {
                        if request.advertiser_id.is_none() {
                           request.advertiser_id = Some(Default::default());
                        }
                                        request.advertiser_id.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "buyer-creative-id" => {
                        request.buyer_creative_id = Some(value.unwrap_or("").to_string());
                    },
                "click-through-url" => {
                        if request.click_through_url.is_none() {
                           request.click_through_url = Some(Default::default());
                        }
                                        request.click_through_url.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "vendor-type" => {
                        if request.vendor_type.is_none() {
                           request.vendor_type = Some(Default::default());
                        }
                                        request.vendor_type.as_mut().unwrap().push(arg_from_str(value.unwrap_or("-0"), err, "vendor-type", "integer"));
                    },
                "filtering-reasons.date" => {
                        request_filtering_reasons_init(&mut request);
                        request.filtering_reasons.as_mut().unwrap().date = Some(value.unwrap_or("").to_string());
                    },
                "sensitive-categories" => {
                        request_filtering_reasons_init(&mut request);
                        if request.sensitive_categories.is_none() {
                           request.sensitive_categories = Some(Default::default());
                        }
                                        request.sensitive_categories.as_mut().unwrap().push(arg_from_str(value.unwrap_or("-0"), err, "sensitive-categories", "integer"));
                    },
                "account-id" => {
                        request_filtering_reasons_init(&mut request);
                        request.account_id = Some(arg_from_str(value.unwrap_or("-0"), err, "account-id", "integer"));
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["html-snippet", "account-id", "advertiser-id", "advertiser-name", "agency-id", "attribute", "buyer-creative-id", "click-through-url", "date", "filtering-reasons", "height", "kind", "product-categories", "restricted-categories", "sensitive-categories", "status", "vendor-type", "video-url", "width"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.creatives().insert(request);
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

    fn _creatives_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.creatives().list();
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "status-filter" => {
                    call = call.status_filter(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "buyer-creative-id" => {
                    call = call.add_buyer_creative_id(value.unwrap_or(""));
                },
                "account-id" => {
                    call = call.add_account_id(arg_from_str(value.unwrap_or("-0"), err, "account-id", "integer"));
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
                                                Vec::new() + &self.gp + &["status-filter", "page-token", "buyer-creative-id", "max-results", "account-id"]
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

    fn _direct_deals_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.direct_deals().get(opt.value_of("id").unwrap_or(""));
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

    fn _direct_deals_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.direct_deals().list();
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

    fn _performance_report_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.performance_report().list(opt.value_of("account-id").unwrap_or(""), opt.value_of("end-date-time").unwrap_or(""), opt.value_of("start-date-time").unwrap_or(""));
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

    fn _pretargeting_config_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.pretargeting_config().delete(opt.value_of("account-id").unwrap_or(""), opt.value_of("config-id").unwrap_or(""));
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

    fn _pretargeting_config_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.pretargeting_config().get(opt.value_of("account-id").unwrap_or(""), opt.value_of("config-id").unwrap_or(""));
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

    fn _pretargeting_config_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::PretargetingConfig::default();
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
                "billing-id" => {
                        request.billing_id = Some(value.unwrap_or("").to_string());
                    },
                "languages" => {
                        if request.languages.is_none() {
                           request.languages = Some(Default::default());
                        }
                                        request.languages.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "config-name" => {
                        request.config_name = Some(value.unwrap_or("").to_string());
                    },
                "excluded-geo-criteria-ids" => {
                        if request.excluded_geo_criteria_ids.is_none() {
                           request.excluded_geo_criteria_ids = Some(Default::default());
                        }
                                        request.excluded_geo_criteria_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "user-lists" => {
                        if request.user_lists.is_none() {
                           request.user_lists = Some(Default::default());
                        }
                                        request.user_lists.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "excluded-verticals" => {
                        if request.excluded_verticals.is_none() {
                           request.excluded_verticals = Some(Default::default());
                        }
                                        request.excluded_verticals.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "vendor-types" => {
                        if request.vendor_types.is_none() {
                           request.vendor_types = Some(Default::default());
                        }
                                        request.vendor_types.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "excluded-content-labels" => {
                        if request.excluded_content_labels.is_none() {
                           request.excluded_content_labels = Some(Default::default());
                        }
                                        request.excluded_content_labels.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "verticals" => {
                        if request.verticals.is_none() {
                           request.verticals = Some(Default::default());
                        }
                                        request.verticals.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "platforms" => {
                        if request.platforms.is_none() {
                           request.platforms = Some(Default::default());
                        }
                                        request.platforms.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "mobile-devices" => {
                        if request.mobile_devices.is_none() {
                           request.mobile_devices = Some(Default::default());
                        }
                                        request.mobile_devices.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "creative-type" => {
                        if request.creative_type.is_none() {
                           request.creative_type = Some(Default::default());
                        }
                                        request.creative_type.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "geo-criteria-ids" => {
                        if request.geo_criteria_ids.is_none() {
                           request.geo_criteria_ids = Some(Default::default());
                        }
                                        request.geo_criteria_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "mobile-operating-system-versions" => {
                        if request.mobile_operating_system_versions.is_none() {
                           request.mobile_operating_system_versions = Some(Default::default());
                        }
                                        request.mobile_operating_system_versions.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "mobile-carriers" => {
                        if request.mobile_carriers.is_none() {
                           request.mobile_carriers = Some(Default::default());
                        }
                                        request.mobile_carriers.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "config-id" => {
                        request.config_id = Some(value.unwrap_or("").to_string());
                    },
                "excluded-user-lists" => {
                        if request.excluded_user_lists.is_none() {
                           request.excluded_user_lists = Some(Default::default());
                        }
                                        request.excluded_user_lists.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "is-active" => {
                        request.is_active = Some(arg_from_str(value.unwrap_or("false"), err, "is-active", "boolean"));
                    },
                "supported-creative-attributes" => {
                        if request.supported_creative_attributes.is_none() {
                           request.supported_creative_attributes = Some(Default::default());
                        }
                                        request.supported_creative_attributes.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["billing-id", "config-id", "config-name", "creative-type", "excluded-content-labels", "excluded-geo-criteria-ids", "excluded-user-lists", "excluded-verticals", "geo-criteria-ids", "is-active", "kind", "languages", "mobile-carriers", "mobile-devices", "mobile-operating-system-versions", "platforms", "supported-creative-attributes", "user-lists", "vendor-types", "verticals"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.pretargeting_config().insert(request, opt.value_of("account-id").unwrap_or(""));
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

    fn _pretargeting_config_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.pretargeting_config().list(opt.value_of("account-id").unwrap_or(""));
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

    fn _pretargeting_config_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::PretargetingConfig::default();
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
                "billing-id" => {
                        request.billing_id = Some(value.unwrap_or("").to_string());
                    },
                "languages" => {
                        if request.languages.is_none() {
                           request.languages = Some(Default::default());
                        }
                                        request.languages.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "config-name" => {
                        request.config_name = Some(value.unwrap_or("").to_string());
                    },
                "excluded-geo-criteria-ids" => {
                        if request.excluded_geo_criteria_ids.is_none() {
                           request.excluded_geo_criteria_ids = Some(Default::default());
                        }
                                        request.excluded_geo_criteria_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "user-lists" => {
                        if request.user_lists.is_none() {
                           request.user_lists = Some(Default::default());
                        }
                                        request.user_lists.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "excluded-verticals" => {
                        if request.excluded_verticals.is_none() {
                           request.excluded_verticals = Some(Default::default());
                        }
                                        request.excluded_verticals.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "vendor-types" => {
                        if request.vendor_types.is_none() {
                           request.vendor_types = Some(Default::default());
                        }
                                        request.vendor_types.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "excluded-content-labels" => {
                        if request.excluded_content_labels.is_none() {
                           request.excluded_content_labels = Some(Default::default());
                        }
                                        request.excluded_content_labels.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "verticals" => {
                        if request.verticals.is_none() {
                           request.verticals = Some(Default::default());
                        }
                                        request.verticals.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "platforms" => {
                        if request.platforms.is_none() {
                           request.platforms = Some(Default::default());
                        }
                                        request.platforms.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "mobile-devices" => {
                        if request.mobile_devices.is_none() {
                           request.mobile_devices = Some(Default::default());
                        }
                                        request.mobile_devices.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "creative-type" => {
                        if request.creative_type.is_none() {
                           request.creative_type = Some(Default::default());
                        }
                                        request.creative_type.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "geo-criteria-ids" => {
                        if request.geo_criteria_ids.is_none() {
                           request.geo_criteria_ids = Some(Default::default());
                        }
                                        request.geo_criteria_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "mobile-operating-system-versions" => {
                        if request.mobile_operating_system_versions.is_none() {
                           request.mobile_operating_system_versions = Some(Default::default());
                        }
                                        request.mobile_operating_system_versions.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "mobile-carriers" => {
                        if request.mobile_carriers.is_none() {
                           request.mobile_carriers = Some(Default::default());
                        }
                                        request.mobile_carriers.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "config-id" => {
                        request.config_id = Some(value.unwrap_or("").to_string());
                    },
                "excluded-user-lists" => {
                        if request.excluded_user_lists.is_none() {
                           request.excluded_user_lists = Some(Default::default());
                        }
                                        request.excluded_user_lists.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "is-active" => {
                        request.is_active = Some(arg_from_str(value.unwrap_or("false"), err, "is-active", "boolean"));
                    },
                "supported-creative-attributes" => {
                        if request.supported_creative_attributes.is_none() {
                           request.supported_creative_attributes = Some(Default::default());
                        }
                                        request.supported_creative_attributes.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["billing-id", "config-id", "config-name", "creative-type", "excluded-content-labels", "excluded-geo-criteria-ids", "excluded-user-lists", "excluded-verticals", "geo-criteria-ids", "is-active", "kind", "languages", "mobile-carriers", "mobile-devices", "mobile-operating-system-versions", "platforms", "supported-creative-attributes", "user-lists", "vendor-types", "verticals"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.pretargeting_config().patch(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("config-id").unwrap_or(""));
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

    fn _pretargeting_config_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::PretargetingConfig::default();
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
                "billing-id" => {
                        request.billing_id = Some(value.unwrap_or("").to_string());
                    },
                "languages" => {
                        if request.languages.is_none() {
                           request.languages = Some(Default::default());
                        }
                                        request.languages.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "config-name" => {
                        request.config_name = Some(value.unwrap_or("").to_string());
                    },
                "excluded-geo-criteria-ids" => {
                        if request.excluded_geo_criteria_ids.is_none() {
                           request.excluded_geo_criteria_ids = Some(Default::default());
                        }
                                        request.excluded_geo_criteria_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "user-lists" => {
                        if request.user_lists.is_none() {
                           request.user_lists = Some(Default::default());
                        }
                                        request.user_lists.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "excluded-verticals" => {
                        if request.excluded_verticals.is_none() {
                           request.excluded_verticals = Some(Default::default());
                        }
                                        request.excluded_verticals.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "vendor-types" => {
                        if request.vendor_types.is_none() {
                           request.vendor_types = Some(Default::default());
                        }
                                        request.vendor_types.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "excluded-content-labels" => {
                        if request.excluded_content_labels.is_none() {
                           request.excluded_content_labels = Some(Default::default());
                        }
                                        request.excluded_content_labels.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "verticals" => {
                        if request.verticals.is_none() {
                           request.verticals = Some(Default::default());
                        }
                                        request.verticals.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "platforms" => {
                        if request.platforms.is_none() {
                           request.platforms = Some(Default::default());
                        }
                                        request.platforms.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "mobile-devices" => {
                        if request.mobile_devices.is_none() {
                           request.mobile_devices = Some(Default::default());
                        }
                                        request.mobile_devices.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "creative-type" => {
                        if request.creative_type.is_none() {
                           request.creative_type = Some(Default::default());
                        }
                                        request.creative_type.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "geo-criteria-ids" => {
                        if request.geo_criteria_ids.is_none() {
                           request.geo_criteria_ids = Some(Default::default());
                        }
                                        request.geo_criteria_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "mobile-operating-system-versions" => {
                        if request.mobile_operating_system_versions.is_none() {
                           request.mobile_operating_system_versions = Some(Default::default());
                        }
                                        request.mobile_operating_system_versions.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "mobile-carriers" => {
                        if request.mobile_carriers.is_none() {
                           request.mobile_carriers = Some(Default::default());
                        }
                                        request.mobile_carriers.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "config-id" => {
                        request.config_id = Some(value.unwrap_or("").to_string());
                    },
                "excluded-user-lists" => {
                        if request.excluded_user_lists.is_none() {
                           request.excluded_user_lists = Some(Default::default());
                        }
                                        request.excluded_user_lists.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "is-active" => {
                        request.is_active = Some(arg_from_str(value.unwrap_or("false"), err, "is-active", "boolean"));
                    },
                "supported-creative-attributes" => {
                        if request.supported_creative_attributes.is_none() {
                           request.supported_creative_attributes = Some(Default::default());
                        }
                                        request.supported_creative_attributes.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["billing-id", "config-id", "config-name", "creative-type", "excluded-content-labels", "excluded-geo-criteria-ids", "excluded-user-lists", "excluded-verticals", "geo-criteria-ids", "is-active", "kind", "languages", "mobile-carriers", "mobile-devices", "mobile-operating-system-versions", "platforms", "supported-creative-attributes", "user-lists", "vendor-types", "verticals"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.pretargeting_config().update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("config-id").unwrap_or(""));
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
            ("accounts", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._accounts_get(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._accounts_list(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._accounts_patch(opt, dry_run, &mut err);
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
            ("billing-info", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._billing_info_get(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._billing_info_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("billing-info".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("budget", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._budget_get(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._budget_patch(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._budget_update(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("budget".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("creatives", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._creatives_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._creatives_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._creatives_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("creatives".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("direct-deals", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._direct_deals_get(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._direct_deals_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("direct-deals".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("performance-report", Some(opt)) => {
                match opt.subcommand() {
                    ("list", Some(opt)) => {
                        call_result = self._performance_report_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("performance-report".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("pretargeting-config", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._pretargeting_config_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._pretargeting_config_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._pretargeting_config_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._pretargeting_config_list(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._pretargeting_config_patch(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._pretargeting_config_update(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("pretargeting-config".to_string()));
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

            match cmn::application_secret_from_directory(&config_dir, "adexchangebuyer1d3-secret.json", 
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
                                          program_name: "adexchangebuyer1d3",
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
            hub: api::AdExchangeBuyer::new(client, auth),
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
        ("accounts", "methods: 'get', 'list', 'patch' and 'update'", vec![
            ("get",  
                    Some(r##"Gets one account by ID."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d3_cli/accounts_get",
                  vec![
                    (Some(r##"id"##),
                     None,
                     Some(r##"The account id"##),
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
                    Some(r##"Retrieves the authenticated user's list of accounts."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d3_cli/accounts_list",
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
            ("patch",  
                    Some(r##"Updates an existing account. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d3_cli/accounts_patch",
                  vec![
                    (Some(r##"id"##),
                     None,
                     Some(r##"The account id"##),
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
                    Some(r##"Updates an existing account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d3_cli/accounts_update",
                  vec![
                    (Some(r##"id"##),
                     None,
                     Some(r##"The account id"##),
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
        
        ("billing-info", "methods: 'get' and 'list'", vec![
            ("get",  
                    Some(r##"Returns the billing information for one account specified by account ID."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d3_cli/billing-info_get",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The account id."##),
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
                    Some(r##"Retrieves a list of billing information for all accounts of the authenticated user."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d3_cli/billing-info_list",
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
        
        ("budget", "methods: 'get', 'patch' and 'update'", vec![
            ("get",  
                    Some(r##"Returns the budget information for the adgroup specified by the accountId and billingId."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d3_cli/budget_get",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The account id to get the budget information for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"billing-id"##),
                     None,
                     Some(r##"The billing id to get the budget information for."##),
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
                    Some(r##"Updates the budget amount for the budget of the adgroup specified by the accountId and billingId, with the budget amount in the request. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d3_cli/budget_patch",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The account id associated with the budget being updated."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"billing-id"##),
                     None,
                     Some(r##"The billing id associated with the budget being updated."##),
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
                    Some(r##"Updates the budget amount for the budget of the adgroup specified by the accountId and billingId, with the budget amount in the request."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d3_cli/budget_update",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The account id associated with the budget being updated."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"billing-id"##),
                     None,
                     Some(r##"The billing id associated with the budget being updated."##),
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
        
        ("creatives", "methods: 'get', 'insert' and 'list'", vec![
            ("get",  
                    Some(r##"Gets the status for a single creative. A creative will be available 30-40 minutes after submission."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d3_cli/creatives_get",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The id for the account that will serve this creative."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"buyer-creative-id"##),
                     None,
                     Some(r##"The buyer-specific id for this creative."##),
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
                    Some(r##"Submit a new creative."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d3_cli/creatives_insert",
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
            ("list",  
                    Some(r##"Retrieves a list of the authenticated user's active creatives. A creative will be available 30-40 minutes after submission."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d3_cli/creatives_list",
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
        
        ("direct-deals", "methods: 'get' and 'list'", vec![
            ("get",  
                    Some(r##"Gets one direct deal by ID."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d3_cli/direct-deals_get",
                  vec![
                    (Some(r##"id"##),
                     None,
                     Some(r##"The direct deal id"##),
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
                    Some(r##"Retrieves the authenticated user's list of direct deals."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d3_cli/direct-deals_list",
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
        
        ("performance-report", "methods: 'list'", vec![
            ("list",  
                    Some(r##"Retrieves the authenticated user's list of performance metrics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d3_cli/performance-report_list",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The account id to get the reports."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"end-date-time"##),
                     None,
                     Some(r##"The end time of the report in ISO 8601 timestamp format using UTC."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"start-date-time"##),
                     None,
                     Some(r##"The start time of the report in ISO 8601 timestamp format using UTC."##),
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
        
        ("pretargeting-config", "methods: 'delete', 'get', 'insert', 'list', 'patch' and 'update'", vec![
            ("delete",  
                    Some(r##"Deletes an existing pretargeting config."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d3_cli/pretargeting-config_delete",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The account id to delete the pretargeting config for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"config-id"##),
                     None,
                     Some(r##"The specific id of the configuration to delete."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",  
                    Some(r##"Gets a specific pretargeting configuration"##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d3_cli/pretargeting-config_get",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The account id to get the pretargeting config for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"config-id"##),
                     None,
                     Some(r##"The specific id of the configuration to retrieve."##),
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
                    Some(r##"Inserts a new pretargeting configuration."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d3_cli/pretargeting-config_insert",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The account id to insert the pretargeting config for."##),
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
                    Some(r##"Retrieves a list of the authenticated user's pretargeting configurations."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d3_cli/pretargeting-config_list",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The account id to get the pretargeting configs for."##),
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
                    Some(r##"Updates an existing pretargeting config. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d3_cli/pretargeting-config_patch",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The account id to update the pretargeting config for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"config-id"##),
                     None,
                     Some(r##"The specific id of the configuration to update."##),
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
                    Some(r##"Updates an existing pretargeting config."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d3_cli/pretargeting-config_update",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The account id to update the pretargeting config for."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"config-id"##),
                     None,
                     Some(r##"The specific id of the configuration to update."##),
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
    
    let mut app = App::new("adexchangebuyer1d3")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("0.2.0+20150326")
           .about("Accesses your bidding-account information, submits creatives for validation, finds available direct deals, and retrieves performance reports.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d3_cli")
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