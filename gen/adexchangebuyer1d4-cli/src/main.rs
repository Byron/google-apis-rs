// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_adexchangebuyer1d4::{api, Error, oauth2, client::chrono, FieldMask};


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
    hub: api::AdExchangeBuyer<S>,
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
    async fn _accounts_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let id: i32 = arg_from_str(&opt.value_of("id").unwrap_or(""), err, "<id>", "integer");
        let mut call = self.hub.accounts().get(id);
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

    async fn _accounts_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().list();
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

    async fn _accounts_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "apply-pretargeting-to-non-guaranteed-deals" => Some(("applyPretargetingToNonGuaranteedDeals", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cookie-matching-nid" => Some(("cookieMatchingNid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cookie-matching-url" => Some(("cookieMatchingUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "maximum-active-creatives" => Some(("maximumActiveCreatives", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "maximum-total-qps" => Some(("maximumTotalQps", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "number-active-creatives" => Some(("numberActiveCreatives", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["apply-pretargeting-to-non-guaranteed-deals", "cookie-matching-nid", "cookie-matching-url", "id", "kind", "maximum-active-creatives", "maximum-total-qps", "number-active-creatives"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Account = json::value::from_value(object).unwrap();
        let id: i32 = arg_from_str(&opt.value_of("id").unwrap_or(""), err, "<id>", "integer");
        let mut call = self.hub.accounts().patch(request, id);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "confirm-unsafe-account-change" => {
                    call = call.confirm_unsafe_account_change(        value.map(|v| arg_from_str(v, err, "confirm-unsafe-account-change", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["confirm-unsafe-account-change"].iter().map(|v|*v));
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

    async fn _accounts_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "apply-pretargeting-to-non-guaranteed-deals" => Some(("applyPretargetingToNonGuaranteedDeals", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cookie-matching-nid" => Some(("cookieMatchingNid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cookie-matching-url" => Some(("cookieMatchingUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "maximum-active-creatives" => Some(("maximumActiveCreatives", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "maximum-total-qps" => Some(("maximumTotalQps", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "number-active-creatives" => Some(("numberActiveCreatives", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["apply-pretargeting-to-non-guaranteed-deals", "cookie-matching-nid", "cookie-matching-url", "id", "kind", "maximum-active-creatives", "maximum-total-qps", "number-active-creatives"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Account = json::value::from_value(object).unwrap();
        let id: i32 = arg_from_str(&opt.value_of("id").unwrap_or(""), err, "<id>", "integer");
        let mut call = self.hub.accounts().update(request, id);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "confirm-unsafe-account-change" => {
                    call = call.confirm_unsafe_account_change(        value.map(|v| arg_from_str(v, err, "confirm-unsafe-account-change", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["confirm-unsafe-account-change"].iter().map(|v|*v));
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

    async fn _billing_info_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let account_id: i32 = arg_from_str(&opt.value_of("account-id").unwrap_or(""), err, "<account-id>", "integer");
        let mut call = self.hub.billing_info().get(account_id);
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

    async fn _billing_info_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.billing_info().list();
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

    async fn _budget_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.budget().get(opt.value_of("account-id").unwrap_or(""), opt.value_of("billing-id").unwrap_or(""));
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

    async fn _budget_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "account-id" => Some(("accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "billing-id" => Some(("billingId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "budget-amount" => Some(("budgetAmount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "currency-code" => Some(("currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "billing-id", "budget-amount", "currency-code", "id", "kind"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Budget = json::value::from_value(object).unwrap();
        let mut call = self.hub.budget().patch(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("billing-id").unwrap_or(""));
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

    async fn _budget_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "account-id" => Some(("accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "billing-id" => Some(("billingId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "budget-amount" => Some(("budgetAmount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "currency-code" => Some(("currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "billing-id", "budget-amount", "currency-code", "id", "kind"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Budget = json::value::from_value(object).unwrap();
        let mut call = self.hub.budget().update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("billing-id").unwrap_or(""));
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

    async fn _creatives_add_deal(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let account_id: i32 = arg_from_str(&opt.value_of("account-id").unwrap_or(""), err, "<account-id>", "integer");
        let mut call = self.hub.creatives().add_deal(account_id, opt.value_of("buyer-creative-id").unwrap_or(""), opt.value_of("deal-id").unwrap_or(""));
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
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _creatives_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let account_id: i32 = arg_from_str(&opt.value_of("account-id").unwrap_or(""), err, "<account-id>", "integer");
        let mut call = self.hub.creatives().get(account_id, opt.value_of("buyer-creative-id").unwrap_or(""));
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

    async fn _creatives_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "html-snippet" => Some(("HTMLSnippet", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "account-id" => Some(("accountId", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "ad-choices-destination-url" => Some(("adChoicesDestinationUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ad-technology-providers.detected-provider-ids" => Some(("adTechnologyProviders.detectedProviderIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "ad-technology-providers.has-unidentified-provider" => Some(("adTechnologyProviders.hasUnidentifiedProvider", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "advertiser-id" => Some(("advertiserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "advertiser-name" => Some(("advertiserName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "agency-id" => Some(("agencyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-upload-timestamp" => Some(("apiUploadTimestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attribute" => Some(("attribute", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Vec })),
                    "buyer-creative-id" => Some(("buyerCreativeId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "click-through-url" => Some(("clickThroughUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "creative-status-identity-type" => Some(("creativeStatusIdentityType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deals-status" => Some(("dealsStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "detected-domains" => Some(("detectedDomains", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "filtering-reasons.date" => Some(("filteringReasons.date", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "height" => Some(("height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "impression-tracking-url" => Some(("impressionTrackingUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "languages" => Some(("languages", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "native-ad.advertiser" => Some(("nativeAd.advertiser", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native-ad.app-icon.height" => Some(("nativeAd.appIcon.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "native-ad.app-icon.url" => Some(("nativeAd.appIcon.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native-ad.app-icon.width" => Some(("nativeAd.appIcon.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "native-ad.body" => Some(("nativeAd.body", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native-ad.call-to-action" => Some(("nativeAd.callToAction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native-ad.click-link-url" => Some(("nativeAd.clickLinkUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native-ad.click-tracking-url" => Some(("nativeAd.clickTrackingUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native-ad.headline" => Some(("nativeAd.headline", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native-ad.image.height" => Some(("nativeAd.image.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "native-ad.image.url" => Some(("nativeAd.image.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native-ad.image.width" => Some(("nativeAd.image.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "native-ad.impression-tracking-url" => Some(("nativeAd.impressionTrackingUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "native-ad.logo.height" => Some(("nativeAd.logo.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "native-ad.logo.url" => Some(("nativeAd.logo.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native-ad.logo.width" => Some(("nativeAd.logo.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "native-ad.price" => Some(("nativeAd.price", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native-ad.star-rating" => Some(("nativeAd.starRating", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "native-ad.video-url" => Some(("nativeAd.videoURL", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "open-auction-status" => Some(("openAuctionStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "product-categories" => Some(("productCategories", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Vec })),
                    "restricted-categories" => Some(("restrictedCategories", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Vec })),
                    "sensitive-categories" => Some(("sensitiveCategories", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Vec })),
                    "vendor-type" => Some(("vendorType", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Vec })),
                    "version" => Some(("version", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "video-url" => Some(("videoURL", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "video-vast-xml" => Some(("videoVastXML", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "width" => Some(("width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["html-snippet", "account-id", "ad-choices-destination-url", "ad-technology-providers", "advertiser", "advertiser-id", "advertiser-name", "agency-id", "api-upload-timestamp", "app-icon", "attribute", "body", "buyer-creative-id", "call-to-action", "click-link-url", "click-through-url", "click-tracking-url", "creative-status-identity-type", "date", "deals-status", "detected-domains", "detected-provider-ids", "filtering-reasons", "has-unidentified-provider", "headline", "height", "image", "impression-tracking-url", "kind", "languages", "logo", "native-ad", "open-auction-status", "price", "product-categories", "restricted-categories", "sensitive-categories", "star-rating", "url", "vendor-type", "version", "video-url", "video-vast-xml", "width"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Creative = json::value::from_value(object).unwrap();
        let mut call = self.hub.creatives().insert(request);
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

    async fn _creatives_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.creatives().list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "open-auction-status-filter" => {
                    call = call.open_auction_status_filter(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "uint32")).unwrap_or(0));
                },
                "deals-status-filter" => {
                    call = call.deals_status_filter(value.unwrap_or(""));
                },
                "buyer-creative-id" => {
                    call = call.add_buyer_creative_id(value.unwrap_or(""));
                },
                "account-id" => {
                    call = call.add_account_id(        value.map(|v| arg_from_str(v, err, "account-id", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["account-id", "buyer-creative-id", "deals-status-filter", "max-results", "open-auction-status-filter", "page-token"].iter().map(|v|*v));
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

    async fn _creatives_list_deals(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let account_id: i32 = arg_from_str(&opt.value_of("account-id").unwrap_or(""), err, "<account-id>", "integer");
        let mut call = self.hub.creatives().list_deals(account_id, opt.value_of("buyer-creative-id").unwrap_or(""));
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

    async fn _creatives_remove_deal(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let account_id: i32 = arg_from_str(&opt.value_of("account-id").unwrap_or(""), err, "<account-id>", "integer");
        let mut call = self.hub.creatives().remove_deal(account_id, opt.value_of("buyer-creative-id").unwrap_or(""), opt.value_of("deal-id").unwrap_or(""));
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
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _marketplacedeals_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "deal-ids" => Some(("dealIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "proposal-revision-number" => Some(("proposalRevisionNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-action" => Some(("updateAction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["deal-ids", "proposal-revision-number", "update-action"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::DeleteOrderDealsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.marketplacedeals().delete(request, opt.value_of("proposal-id").unwrap_or(""));
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

    async fn _marketplacedeals_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "proposal-revision-number" => Some(("proposalRevisionNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-action" => Some(("updateAction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["proposal-revision-number", "update-action"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::AddOrderDealsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.marketplacedeals().insert(request, opt.value_of("proposal-id").unwrap_or(""));
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

    async fn _marketplacedeals_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.marketplacedeals().list(opt.value_of("proposal-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "pql-query" => {
                    call = call.pql_query(value.unwrap_or(""));
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
                                                                           v.extend(["pql-query"].iter().map(|v|*v));
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

    async fn _marketplacedeals_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "proposal.billed-buyer.account-id" => Some(("proposal.billedBuyer.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "proposal.buyer.account-id" => Some(("proposal.buyer.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "proposal.buyer-private-data.reference-id" => Some(("proposal.buyerPrivateData.referenceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "proposal.buyer-private-data.reference-payload" => Some(("proposal.buyerPrivateData.referencePayload", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "proposal.dbm-advertiser-ids" => Some(("proposal.dbmAdvertiserIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "proposal.has-buyer-signed-off" => Some(("proposal.hasBuyerSignedOff", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "proposal.has-seller-signed-off" => Some(("proposal.hasSellerSignedOff", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "proposal.inventory-source" => Some(("proposal.inventorySource", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "proposal.is-renegotiating" => Some(("proposal.isRenegotiating", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "proposal.is-setup-complete" => Some(("proposal.isSetupComplete", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "proposal.kind" => Some(("proposal.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "proposal.last-updater-or-commentor-role" => Some(("proposal.lastUpdaterOrCommentorRole", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "proposal.name" => Some(("proposal.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "proposal.negotiation-id" => Some(("proposal.negotiationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "proposal.originator-role" => Some(("proposal.originatorRole", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "proposal.private-auction-id" => Some(("proposal.privateAuctionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "proposal.proposal-id" => Some(("proposal.proposalId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "proposal.proposal-state" => Some(("proposal.proposalState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "proposal.revision-number" => Some(("proposal.revisionNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "proposal.revision-time-ms" => Some(("proposal.revisionTimeMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "proposal.seller.account-id" => Some(("proposal.seller.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "proposal.seller.sub-account-id" => Some(("proposal.seller.subAccountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "proposal-revision-number" => Some(("proposalRevisionNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-action" => Some(("updateAction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "billed-buyer", "buyer", "buyer-private-data", "dbm-advertiser-ids", "has-buyer-signed-off", "has-seller-signed-off", "inventory-source", "is-renegotiating", "is-setup-complete", "kind", "last-updater-or-commentor-role", "name", "negotiation-id", "originator-role", "private-auction-id", "proposal", "proposal-id", "proposal-revision-number", "proposal-state", "reference-id", "reference-payload", "revision-number", "revision-time-ms", "seller", "sub-account-id", "update-action"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::EditAllOrderDealsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.marketplacedeals().update(request, opt.value_of("proposal-id").unwrap_or(""));
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

    async fn _marketplacenotes_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::AddOrderNotesRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.marketplacenotes().insert(request, opt.value_of("proposal-id").unwrap_or(""));
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

    async fn _marketplacenotes_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.marketplacenotes().list(opt.value_of("proposal-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "pql-query" => {
                    call = call.pql_query(value.unwrap_or(""));
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
                                                                           v.extend(["pql-query"].iter().map(|v|*v));
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

    async fn _marketplaceprivateauction_updateproposal(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "external-deal-id" => Some(("externalDealId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "note.creator-role" => Some(("note.creatorRole", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "note.deal-id" => Some(("note.dealId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "note.kind" => Some(("note.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "note.note" => Some(("note.note", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "note.note-id" => Some(("note.noteId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "note.proposal-id" => Some(("note.proposalId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "note.proposal-revision-number" => Some(("note.proposalRevisionNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "note.timestamp-ms" => Some(("note.timestampMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "proposal-revision-number" => Some(("proposalRevisionNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-action" => Some(("updateAction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["creator-role", "deal-id", "external-deal-id", "kind", "note", "note-id", "proposal-id", "proposal-revision-number", "timestamp-ms", "update-action"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::UpdatePrivateAuctionProposalRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.marketplaceprivateauction().updateproposal(request, opt.value_of("private-auction-id").unwrap_or(""));
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
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _performance_report_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.performance_report().list(opt.value_of("account-id").unwrap_or(""), opt.value_of("end-date-time").unwrap_or(""), opt.value_of("start-date-time").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(        value.map(|v| arg_from_str(v, err, "max-results", "uint32")).unwrap_or(0));
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
                                                                           v.extend(["max-results", "page-token"].iter().map(|v|*v));
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

    async fn _pretargeting_config_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.pretargeting_config().delete(opt.value_of("account-id").unwrap_or(""), opt.value_of("config-id").unwrap_or(""));
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
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _pretargeting_config_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.pretargeting_config().get(opt.value_of("account-id").unwrap_or(""), opt.value_of("config-id").unwrap_or(""));
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

    async fn _pretargeting_config_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "billing-id" => Some(("billingId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config-id" => Some(("configId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config-name" => Some(("configName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creative-type" => Some(("creativeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "excluded-content-labels" => Some(("excludedContentLabels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "excluded-geo-criteria-ids" => Some(("excludedGeoCriteriaIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "excluded-user-lists" => Some(("excludedUserLists", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "excluded-verticals" => Some(("excludedVerticals", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "geo-criteria-ids" => Some(("geoCriteriaIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "is-active" => Some(("isActive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "languages" => Some(("languages", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "maximum-qps" => Some(("maximumQps", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "minimum-viewability-decile" => Some(("minimumViewabilityDecile", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "mobile-carriers" => Some(("mobileCarriers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "mobile-devices" => Some(("mobileDevices", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "mobile-operating-system-versions" => Some(("mobileOperatingSystemVersions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "platforms" => Some(("platforms", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "supported-creative-attributes" => Some(("supportedCreativeAttributes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "user-identifier-data-required" => Some(("userIdentifierDataRequired", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "user-lists" => Some(("userLists", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "vendor-types" => Some(("vendorTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "verticals" => Some(("verticals", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["billing-id", "config-id", "config-name", "creative-type", "excluded-content-labels", "excluded-geo-criteria-ids", "excluded-user-lists", "excluded-verticals", "geo-criteria-ids", "is-active", "kind", "languages", "maximum-qps", "minimum-viewability-decile", "mobile-carriers", "mobile-devices", "mobile-operating-system-versions", "platforms", "supported-creative-attributes", "user-identifier-data-required", "user-lists", "vendor-types", "verticals"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::PretargetingConfig = json::value::from_value(object).unwrap();
        let mut call = self.hub.pretargeting_config().insert(request, opt.value_of("account-id").unwrap_or(""));
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

    async fn _pretargeting_config_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.pretargeting_config().list(opt.value_of("account-id").unwrap_or(""));
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

    async fn _pretargeting_config_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "billing-id" => Some(("billingId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config-id" => Some(("configId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config-name" => Some(("configName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creative-type" => Some(("creativeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "excluded-content-labels" => Some(("excludedContentLabels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "excluded-geo-criteria-ids" => Some(("excludedGeoCriteriaIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "excluded-user-lists" => Some(("excludedUserLists", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "excluded-verticals" => Some(("excludedVerticals", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "geo-criteria-ids" => Some(("geoCriteriaIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "is-active" => Some(("isActive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "languages" => Some(("languages", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "maximum-qps" => Some(("maximumQps", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "minimum-viewability-decile" => Some(("minimumViewabilityDecile", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "mobile-carriers" => Some(("mobileCarriers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "mobile-devices" => Some(("mobileDevices", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "mobile-operating-system-versions" => Some(("mobileOperatingSystemVersions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "platforms" => Some(("platforms", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "supported-creative-attributes" => Some(("supportedCreativeAttributes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "user-identifier-data-required" => Some(("userIdentifierDataRequired", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "user-lists" => Some(("userLists", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "vendor-types" => Some(("vendorTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "verticals" => Some(("verticals", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["billing-id", "config-id", "config-name", "creative-type", "excluded-content-labels", "excluded-geo-criteria-ids", "excluded-user-lists", "excluded-verticals", "geo-criteria-ids", "is-active", "kind", "languages", "maximum-qps", "minimum-viewability-decile", "mobile-carriers", "mobile-devices", "mobile-operating-system-versions", "platforms", "supported-creative-attributes", "user-identifier-data-required", "user-lists", "vendor-types", "verticals"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::PretargetingConfig = json::value::from_value(object).unwrap();
        let mut call = self.hub.pretargeting_config().patch(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("config-id").unwrap_or(""));
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

    async fn _pretargeting_config_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "billing-id" => Some(("billingId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config-id" => Some(("configId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config-name" => Some(("configName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creative-type" => Some(("creativeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "excluded-content-labels" => Some(("excludedContentLabels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "excluded-geo-criteria-ids" => Some(("excludedGeoCriteriaIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "excluded-user-lists" => Some(("excludedUserLists", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "excluded-verticals" => Some(("excludedVerticals", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "geo-criteria-ids" => Some(("geoCriteriaIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "is-active" => Some(("isActive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "languages" => Some(("languages", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "maximum-qps" => Some(("maximumQps", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "minimum-viewability-decile" => Some(("minimumViewabilityDecile", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "mobile-carriers" => Some(("mobileCarriers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "mobile-devices" => Some(("mobileDevices", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "mobile-operating-system-versions" => Some(("mobileOperatingSystemVersions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "platforms" => Some(("platforms", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "supported-creative-attributes" => Some(("supportedCreativeAttributes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "user-identifier-data-required" => Some(("userIdentifierDataRequired", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "user-lists" => Some(("userLists", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "vendor-types" => Some(("vendorTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "verticals" => Some(("verticals", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["billing-id", "config-id", "config-name", "creative-type", "excluded-content-labels", "excluded-geo-criteria-ids", "excluded-user-lists", "excluded-verticals", "geo-criteria-ids", "is-active", "kind", "languages", "maximum-qps", "minimum-viewability-decile", "mobile-carriers", "mobile-devices", "mobile-operating-system-versions", "platforms", "supported-creative-attributes", "user-identifier-data-required", "user-lists", "vendor-types", "verticals"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::PretargetingConfig = json::value::from_value(object).unwrap();
        let mut call = self.hub.pretargeting_config().update(request, opt.value_of("account-id").unwrap_or(""), opt.value_of("config-id").unwrap_or(""));
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

    async fn _products_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.products().get(opt.value_of("product-id").unwrap_or(""));
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

    async fn _products_search(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.products().search();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "pql-query" => {
                    call = call.pql_query(value.unwrap_or(""));
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
                                                                           v.extend(["pql-query"].iter().map(|v|*v));
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

    async fn _proposals_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.proposals().get(opt.value_of("proposal-id").unwrap_or(""));
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

    async fn _proposals_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "web-property-code" => Some(("webPropertyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["web-property-code"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CreateOrdersRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.proposals().insert(request);
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

    async fn _proposals_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "billed-buyer.account-id" => Some(("billedBuyer.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "buyer.account-id" => Some(("buyer.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "buyer-private-data.reference-id" => Some(("buyerPrivateData.referenceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "buyer-private-data.reference-payload" => Some(("buyerPrivateData.referencePayload", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dbm-advertiser-ids" => Some(("dbmAdvertiserIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "has-buyer-signed-off" => Some(("hasBuyerSignedOff", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "has-seller-signed-off" => Some(("hasSellerSignedOff", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inventory-source" => Some(("inventorySource", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "is-renegotiating" => Some(("isRenegotiating", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "is-setup-complete" => Some(("isSetupComplete", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-updater-or-commentor-role" => Some(("lastUpdaterOrCommentorRole", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "negotiation-id" => Some(("negotiationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "originator-role" => Some(("originatorRole", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "private-auction-id" => Some(("privateAuctionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "proposal-id" => Some(("proposalId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "proposal-state" => Some(("proposalState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "revision-number" => Some(("revisionNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "revision-time-ms" => Some(("revisionTimeMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "seller.account-id" => Some(("seller.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "seller.sub-account-id" => Some(("seller.subAccountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "billed-buyer", "buyer", "buyer-private-data", "dbm-advertiser-ids", "has-buyer-signed-off", "has-seller-signed-off", "inventory-source", "is-renegotiating", "is-setup-complete", "kind", "last-updater-or-commentor-role", "name", "negotiation-id", "originator-role", "private-auction-id", "proposal-id", "proposal-state", "reference-id", "reference-payload", "revision-number", "revision-time-ms", "seller", "sub-account-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Proposal = json::value::from_value(object).unwrap();
        let mut call = self.hub.proposals().patch(request, opt.value_of("proposal-id").unwrap_or(""), opt.value_of("revision-number").unwrap_or(""), opt.value_of("update-action").unwrap_or(""));
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

    async fn _proposals_search(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.proposals().search();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "pql-query" => {
                    call = call.pql_query(value.unwrap_or(""));
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
                                                                           v.extend(["pql-query"].iter().map(|v|*v));
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

    async fn _proposals_setupcomplete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.proposals().setupcomplete(opt.value_of("proposal-id").unwrap_or(""));
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
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    async fn _proposals_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "billed-buyer.account-id" => Some(("billedBuyer.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "buyer.account-id" => Some(("buyer.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "buyer-private-data.reference-id" => Some(("buyerPrivateData.referenceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "buyer-private-data.reference-payload" => Some(("buyerPrivateData.referencePayload", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dbm-advertiser-ids" => Some(("dbmAdvertiserIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "has-buyer-signed-off" => Some(("hasBuyerSignedOff", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "has-seller-signed-off" => Some(("hasSellerSignedOff", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inventory-source" => Some(("inventorySource", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "is-renegotiating" => Some(("isRenegotiating", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "is-setup-complete" => Some(("isSetupComplete", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-updater-or-commentor-role" => Some(("lastUpdaterOrCommentorRole", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "negotiation-id" => Some(("negotiationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "originator-role" => Some(("originatorRole", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "private-auction-id" => Some(("privateAuctionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "proposal-id" => Some(("proposalId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "proposal-state" => Some(("proposalState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "revision-number" => Some(("revisionNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "revision-time-ms" => Some(("revisionTimeMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "seller.account-id" => Some(("seller.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "seller.sub-account-id" => Some(("seller.subAccountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "billed-buyer", "buyer", "buyer-private-data", "dbm-advertiser-ids", "has-buyer-signed-off", "has-seller-signed-off", "inventory-source", "is-renegotiating", "is-setup-complete", "kind", "last-updater-or-commentor-role", "name", "negotiation-id", "originator-role", "private-auction-id", "proposal-id", "proposal-state", "reference-id", "reference-payload", "revision-number", "revision-time-ms", "seller", "sub-account-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Proposal = json::value::from_value(object).unwrap();
        let mut call = self.hub.proposals().update(request, opt.value_of("proposal-id").unwrap_or(""), opt.value_of("revision-number").unwrap_or(""), opt.value_of("update-action").unwrap_or(""));
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

    async fn _pubprofiles_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let account_id: i32 = arg_from_str(&opt.value_of("account-id").unwrap_or(""), err, "<account-id>", "integer");
        let mut call = self.hub.pubprofiles().list(account_id);
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
            ("accounts", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._accounts_get(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._accounts_list(opt, dry_run, &mut err).await;
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._accounts_patch(opt, dry_run, &mut err).await;
                    },
                    ("update", Some(opt)) => {
                        call_result = self._accounts_update(opt, dry_run, &mut err).await;
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
                        call_result = self._billing_info_get(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._billing_info_list(opt, dry_run, &mut err).await;
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
                        call_result = self._budget_get(opt, dry_run, &mut err).await;
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._budget_patch(opt, dry_run, &mut err).await;
                    },
                    ("update", Some(opt)) => {
                        call_result = self._budget_update(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("budget".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("creatives", Some(opt)) => {
                match opt.subcommand() {
                    ("add-deal", Some(opt)) => {
                        call_result = self._creatives_add_deal(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._creatives_get(opt, dry_run, &mut err).await;
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._creatives_insert(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._creatives_list(opt, dry_run, &mut err).await;
                    },
                    ("list-deals", Some(opt)) => {
                        call_result = self._creatives_list_deals(opt, dry_run, &mut err).await;
                    },
                    ("remove-deal", Some(opt)) => {
                        call_result = self._creatives_remove_deal(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("creatives".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("marketplacedeals", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._marketplacedeals_delete(opt, dry_run, &mut err).await;
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._marketplacedeals_insert(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._marketplacedeals_list(opt, dry_run, &mut err).await;
                    },
                    ("update", Some(opt)) => {
                        call_result = self._marketplacedeals_update(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("marketplacedeals".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("marketplacenotes", Some(opt)) => {
                match opt.subcommand() {
                    ("insert", Some(opt)) => {
                        call_result = self._marketplacenotes_insert(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._marketplacenotes_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("marketplacenotes".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("marketplaceprivateauction", Some(opt)) => {
                match opt.subcommand() {
                    ("updateproposal", Some(opt)) => {
                        call_result = self._marketplaceprivateauction_updateproposal(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("marketplaceprivateauction".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("performance-report", Some(opt)) => {
                match opt.subcommand() {
                    ("list", Some(opt)) => {
                        call_result = self._performance_report_list(opt, dry_run, &mut err).await;
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
                        call_result = self._pretargeting_config_delete(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._pretargeting_config_get(opt, dry_run, &mut err).await;
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._pretargeting_config_insert(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._pretargeting_config_list(opt, dry_run, &mut err).await;
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._pretargeting_config_patch(opt, dry_run, &mut err).await;
                    },
                    ("update", Some(opt)) => {
                        call_result = self._pretargeting_config_update(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("pretargeting-config".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("products", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._products_get(opt, dry_run, &mut err).await;
                    },
                    ("search", Some(opt)) => {
                        call_result = self._products_search(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("products".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("proposals", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._proposals_get(opt, dry_run, &mut err).await;
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._proposals_insert(opt, dry_run, &mut err).await;
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._proposals_patch(opt, dry_run, &mut err).await;
                    },
                    ("search", Some(opt)) => {
                        call_result = self._proposals_search(opt, dry_run, &mut err).await;
                    },
                    ("setupcomplete", Some(opt)) => {
                        call_result = self._proposals_setupcomplete(opt, dry_run, &mut err).await;
                    },
                    ("update", Some(opt)) => {
                        call_result = self._proposals_update(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("proposals".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("pubprofiles", Some(opt)) => {
                match opt.subcommand() {
                    ("list", Some(opt)) => {
                        call_result = self._pubprofiles_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("pubprofiles".to_string()));
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

            match client::application_secret_from_directory(&config_dir, "adexchangebuyer1d4-secret.json",
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
        ).persist_tokens_to_disk(format!("{}/adexchangebuyer1d4", config_dir)).build().await.unwrap();

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
        ("accounts", "methods: 'get', 'list', 'patch' and 'update'", vec![
            ("get",
                    Some(r##"Gets one account by ID."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/accounts_get",
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
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/accounts_list",
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
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/accounts_patch",
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
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/accounts_update",
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
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/billing-info_get",
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
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/billing-info_list",
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
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/budget_get",
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
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/budget_patch",
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
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/budget_update",
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
        
        ("creatives", "methods: 'add-deal', 'get', 'insert', 'list', 'list-deals' and 'remove-deal'", vec![
            ("add-deal",
                    Some(r##"Add a deal id association for the creative."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/creatives_add-deal",
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
        
                    (Some(r##"deal-id"##),
                     None,
                     Some(r##"The id of the deal id to associate with this creative."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",
                    Some(r##"Gets the status for a single creative. A creative will be available 30-40 minutes after submission."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/creatives_get",
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
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/creatives_insert",
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
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/creatives_list",
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
            ("list-deals",
                    Some(r##"Lists the external deal ids associated with the creative."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/creatives_list-deals",
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
            ("remove-deal",
                    Some(r##"Remove a deal id associated with the creative."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/creatives_remove-deal",
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
        
                    (Some(r##"deal-id"##),
                     None,
                     Some(r##"The id of the deal id to disassociate with this creative."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ]),
        
        ("marketplacedeals", "methods: 'delete', 'insert', 'list' and 'update'", vec![
            ("delete",
                    Some(r##"Delete the specified deals from the proposal"##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/marketplacedeals_delete",
                  vec![
                    (Some(r##"proposal-id"##),
                     None,
                     Some(r##"The proposalId to delete deals from."##),
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
            ("insert",
                    Some(r##"Add new deals for the specified proposal"##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/marketplacedeals_insert",
                  vec![
                    (Some(r##"proposal-id"##),
                     None,
                     Some(r##"proposalId for which deals need to be added."##),
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
                    Some(r##"List all the deals for a given proposal"##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/marketplacedeals_list",
                  vec![
                    (Some(r##"proposal-id"##),
                     None,
                     Some(r##"The proposalId to get deals for. To search across all proposals specify order_id = '-' as part of the URL."##),
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
            ("update",
                    Some(r##"Replaces all the deals in the proposal with the passed in deals"##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/marketplacedeals_update",
                  vec![
                    (Some(r##"proposal-id"##),
                     None,
                     Some(r##"The proposalId to edit deals on."##),
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
        
        ("marketplacenotes", "methods: 'insert' and 'list'", vec![
            ("insert",
                    Some(r##"Add notes to the proposal"##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/marketplacenotes_insert",
                  vec![
                    (Some(r##"proposal-id"##),
                     None,
                     Some(r##"The proposalId to add notes for."##),
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
                    Some(r##"Get all the notes associated with a proposal"##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/marketplacenotes_list",
                  vec![
                    (Some(r##"proposal-id"##),
                     None,
                     Some(r##"The proposalId to get notes for. To search across all proposals specify order_id = '-' as part of the URL."##),
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
        
        ("marketplaceprivateauction", "methods: 'updateproposal'", vec![
            ("updateproposal",
                    Some(r##"Update a given private auction proposal"##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/marketplaceprivateauction_updateproposal",
                  vec![
                    (Some(r##"private-auction-id"##),
                     None,
                     Some(r##"The private auction id to be updated."##),
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
            ]),
        
        ("performance-report", "methods: 'list'", vec![
            ("list",
                    Some(r##"Retrieves the authenticated user's list of performance metrics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/performance-report_list",
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
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/pretargeting-config_delete",
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
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/pretargeting-config_get",
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
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/pretargeting-config_insert",
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
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/pretargeting-config_list",
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
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/pretargeting-config_patch",
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
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/pretargeting-config_update",
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
        
        ("products", "methods: 'get' and 'search'", vec![
            ("get",
                    Some(r##"Gets the requested product by id."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/products_get",
                  vec![
                    (Some(r##"product-id"##),
                     None,
                     Some(r##"The id for the product to get the head revision for."##),
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
                    Some(r##"Gets the requested product."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/products_search",
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
        
        ("proposals", "methods: 'get', 'insert', 'patch', 'search', 'setupcomplete' and 'update'", vec![
            ("get",
                    Some(r##"Get a proposal given its id"##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/proposals_get",
                  vec![
                    (Some(r##"proposal-id"##),
                     None,
                     Some(r##"Id of the proposal to retrieve."##),
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
                    Some(r##"Create the given list of proposals"##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/proposals_insert",
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
            ("patch",
                    Some(r##"Update the given proposal. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/proposals_patch",
                  vec![
                    (Some(r##"proposal-id"##),
                     None,
                     Some(r##"The proposal id to update."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"revision-number"##),
                     None,
                     Some(r##"The last known revision number to update. If the head revision in the marketplace database has since changed, an error will be thrown. The caller should then fetch the latest proposal at head revision and retry the update at that revision."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"update-action"##),
                     None,
                     Some(r##"The proposed action to take on the proposal. This field is required and it must be set when updating a proposal."##),
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
            ("search",
                    Some(r##"Search for proposals using pql query"##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/proposals_search",
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
            ("setupcomplete",
                    Some(r##"Update the given proposal to indicate that setup has been completed."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/proposals_setupcomplete",
                  vec![
                    (Some(r##"proposal-id"##),
                     None,
                     Some(r##"The proposal id for which the setup is complete"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("update",
                    Some(r##"Update the given proposal"##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/proposals_update",
                  vec![
                    (Some(r##"proposal-id"##),
                     None,
                     Some(r##"The proposal id to update."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"revision-number"##),
                     None,
                     Some(r##"The last known revision number to update. If the head revision in the marketplace database has since changed, an error will be thrown. The caller should then fetch the latest proposal at head revision and retry the update at that revision."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"update-action"##),
                     None,
                     Some(r##"The proposed action to take on the proposal. This field is required and it must be set when updating a proposal."##),
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
        
        ("pubprofiles", "methods: 'list'", vec![
            ("list",
                    Some(r##"Gets the requested publisher profile(s) by publisher accountId."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/pubprofiles_list",
                  vec![
                    (Some(r##"account-id"##),
                     None,
                     Some(r##"The accountId of the publisher to get profiles for."##),
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
    
    let mut app = App::new("adexchangebuyer1d4")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+20210330")
           .about("Accesses your bidding-account information, submits creatives for validation, finds available direct deals, and retrieves performance reports.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli")
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
