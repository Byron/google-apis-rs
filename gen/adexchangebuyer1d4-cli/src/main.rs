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
extern crate google_adexchangebuyer1d4 as api;

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

    fn _accounts_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "maximum-total-qps" => Some(("maximumTotalQps", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "maximum-active-creatives" => Some(("maximumActiveCreatives", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cookie-matching-nid" => Some(("cookieMatchingNid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "number-active-creatives" => Some(("numberActiveCreatives", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cookie-matching-url" => Some(("cookieMatchingUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cookie-matching-nid", "cookie-matching-url", "id", "kind", "maximum-active-creatives", "maximum-total-qps", "number-active-creatives"]);
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

    fn _accounts_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "maximum-total-qps" => Some(("maximumTotalQps", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "maximum-active-creatives" => Some(("maximumActiveCreatives", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cookie-matching-nid" => Some(("cookieMatchingNid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "number-active-creatives" => Some(("numberActiveCreatives", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cookie-matching-url" => Some(("cookieMatchingUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cookie-matching-nid", "cookie-matching-url", "id", "kind", "maximum-active-creatives", "maximum-total-qps", "number-active-creatives"]);
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

    fn _budget_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "budget-amount" => Some(("budgetAmount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "currency-code" => Some(("currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "billing-id" => Some(("billingId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "account-id" => Some(("accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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

    fn _budget_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "budget-amount" => Some(("budgetAmount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "currency-code" => Some(("currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "billing-id" => Some(("billingId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "account-id" => Some(("accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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

    fn _clientaccess_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let sponsor_account_id: i32 = arg_from_str(&opt.value_of("sponsor-account-id").unwrap_or(""), err, "<sponsor-account-id>", "integer");
        let mut call = self.hub.clientaccess().delete(opt.value_of("client-account-id").unwrap_or(""), sponsor_account_id);
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

    fn _clientaccess_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let sponsor_account_id: i32 = arg_from_str(&opt.value_of("sponsor-account-id").unwrap_or(""), err, "<sponsor-account-id>", "integer");
        let mut call = self.hub.clientaccess().get(opt.value_of("client-account-id").unwrap_or(""), sponsor_account_id);
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

    fn _clientaccess_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "client-account-id" => Some(("clientAccountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "capabilities" => Some(("capabilities", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["capabilities", "client-account-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ClientAccessCapabilities = json::value::from_value(object).unwrap();
        let mut call = self.hub.clientaccess().insert(request);
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "sponsor-account-id" => {
                    call = call.sponsor_account_id(arg_from_str(value.unwrap_or("-0"), err, "sponsor-account-id", "integer"));
                },
                "client-account-id" => {
                    call = call.client_account_id(value.unwrap_or(""));
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
                                                                           v.extend(["client-account-id", "sponsor-account-id"].iter().map(|v|*v));
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

    fn _clientaccess_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "sponsor-account-id" => Some(("sponsorAccountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["sponsor-account-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ListClientAccessCapabilitiesRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.clientaccess().list(request);
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

    fn _clientaccess_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "client-account-id" => Some(("clientAccountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "capabilities" => Some(("capabilities", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["capabilities", "client-account-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ClientAccessCapabilities = json::value::from_value(object).unwrap();
        let sponsor_account_id: i32 = arg_from_str(&opt.value_of("sponsor-account-id").unwrap_or(""), err, "<sponsor-account-id>", "integer");
        let mut call = self.hub.clientaccess().patch(request, opt.value_of("client-account-id").unwrap_or(""), sponsor_account_id);
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

    fn _clientaccess_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "client-account-id" => Some(("clientAccountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "capabilities" => Some(("capabilities", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["capabilities", "client-account-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ClientAccessCapabilities = json::value::from_value(object).unwrap();
        let sponsor_account_id: i32 = arg_from_str(&opt.value_of("sponsor-account-id").unwrap_or(""), err, "<sponsor-account-id>", "integer");
        let mut call = self.hub.clientaccess().update(request, opt.value_of("client-account-id").unwrap_or(""), sponsor_account_id);
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

    fn _creatives_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "api-upload-timestamp" => Some(("api_upload_timestamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attribute" => Some(("attribute", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Vec })),
                    "height" => Some(("height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "advertiser-name" => Some(("advertiserName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "html-snippet" => Some(("HTMLSnippet", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "open-auction-status" => Some(("openAuctionStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "buyer-creative-id" => Some(("buyerCreativeId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "impression-tracking-url" => Some(("impressionTrackingUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "account-id" => Some(("accountId", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "video-url" => Some(("videoURL", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "click-through-url" => Some(("clickThroughUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "width" => Some(("width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "native-ad.body" => Some(("nativeAd.body", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native-ad.advertiser" => Some(("nativeAd.advertiser", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native-ad.store" => Some(("nativeAd.store", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native-ad.headline" => Some(("nativeAd.headline", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native-ad.image.url" => Some(("nativeAd.image.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native-ad.image.width" => Some(("nativeAd.image.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "native-ad.image.height" => Some(("nativeAd.image.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "native-ad.star-rating" => Some(("nativeAd.starRating", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "native-ad.call-to-action" => Some(("nativeAd.callToAction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native-ad.logo.url" => Some(("nativeAd.logo.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native-ad.logo.width" => Some(("nativeAd.logo.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "native-ad.logo.height" => Some(("nativeAd.logo.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "native-ad.app-icon.url" => Some(("nativeAd.appIcon.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native-ad.app-icon.width" => Some(("nativeAd.appIcon.width", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "native-ad.app-icon.height" => Some(("nativeAd.appIcon.height", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "native-ad.impression-tracking-url" => Some(("nativeAd.impressionTrackingUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "native-ad.price" => Some(("nativeAd.price", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "native-ad.click-tracking-url" => Some(("nativeAd.clickTrackingUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "version" => Some(("version", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "vendor-type" => Some(("vendorType", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Vec })),
                    "sensitive-categories" => Some(("sensitiveCategories", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Vec })),
                    "product-categories" => Some(("productCategories", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Vec })),
                    "agency-id" => Some(("agencyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deals-status" => Some(("dealsStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "advertiser-id" => Some(("advertiserId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restricted-categories" => Some(("restrictedCategories", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Vec })),
                    "filtering-reasons.date" => Some(("filteringReasons.date", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["html-snippet", "account-id", "advertiser", "advertiser-id", "advertiser-name", "agency-id", "api-upload-timestamp", "app-icon", "attribute", "body", "buyer-creative-id", "call-to-action", "click-through-url", "click-tracking-url", "date", "deals-status", "filtering-reasons", "headline", "height", "image", "impression-tracking-url", "kind", "logo", "native-ad", "open-auction-status", "price", "product-categories", "restricted-categories", "sensitive-categories", "star-rating", "store", "url", "vendor-type", "version", "video-url", "width"]);
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

    fn _creatives_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.creatives().list();
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "open-auction-status-filter" => {
                    call = call.open_auction_status_filter(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "deals-status-filter" => {
                    call = call.deals_status_filter(value.unwrap_or(""));
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
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["open-auction-status-filter", "max-results", "page-token", "buyer-creative-id", "deals-status-filter", "account-id"].iter().map(|v|*v));
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

    fn _deals_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "include-private-auctions" => Some(("includePrivateAuctions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["include-private-auctions"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GetFinalizedNegotiationByExternalDealIdRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.deals().get(request, opt.value_of("deal-id").unwrap_or(""));
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

    fn _marketplacedeals_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "order-revision-number" => Some(("orderRevisionNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-action" => Some(("updateAction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deal-ids" => Some(("dealIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["deal-ids", "order-revision-number", "update-action"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::DeleteOrderDealsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.marketplacedeals().delete(request, opt.value_of("order-id").unwrap_or(""));
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

    fn _marketplacedeals_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "order-revision-number" => Some(("orderRevisionNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-action" => Some(("updateAction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["order-revision-number", "update-action"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::AddOrderDealsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.marketplacedeals().insert(request, opt.value_of("order-id").unwrap_or(""));
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

    fn _marketplacedeals_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.marketplacedeals().list(opt.value_of("order-id").unwrap_or(""));
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

    fn _marketplacedeals_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "order-revision-number" => Some(("orderRevisionNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "order.order-id" => Some(("order.orderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "order.kind" => Some(("order.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "order.name" => Some(("order.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "order.revision-time-ms" => Some(("order.revisionTimeMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "order.buyer-private-data.reference-id" => Some(("order.buyerPrivateData.referenceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "order.buyer-private-data.reference-payload" => Some(("order.buyerPrivateData.referencePayload", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "order.billed-buyer.account-id" => Some(("order.billedBuyer.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "order.originator-role" => Some(("order.originatorRole", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "order.seller.sub-account-id" => Some(("order.seller.subAccountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "order.seller.account-id" => Some(("order.seller.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "order.last-updater-role" => Some(("order.lastUpdaterRole", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "order.order-state" => Some(("order.orderState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "order.has-seller-signed-off" => Some(("order.hasSellerSignedOff", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "order.is-renegotiating" => Some(("order.isRenegotiating", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "order.buyer.account-id" => Some(("order.buyer.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "order.has-buyer-signed-off" => Some(("order.hasBuyerSignedOff", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "order.is-setup-complete" => Some(("order.isSetupComplete", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "order.revision-number" => Some(("order.revisionNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "order.last-updater-or-commentor-role" => Some(("order.lastUpdaterOrCommentorRole", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-action" => Some(("updateAction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "billed-buyer", "buyer", "buyer-private-data", "has-buyer-signed-off", "has-seller-signed-off", "is-renegotiating", "is-setup-complete", "kind", "last-updater-or-commentor-role", "last-updater-role", "name", "order", "order-id", "order-revision-number", "order-state", "originator-role", "reference-id", "reference-payload", "revision-number", "revision-time-ms", "seller", "sub-account-id", "update-action"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::EditAllOrderDealsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.marketplacedeals().update(request, opt.value_of("order-id").unwrap_or(""));
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

    fn _marketplacenotes_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.marketplacenotes().insert(request, opt.value_of("order-id").unwrap_or(""));
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

    fn _marketplacenotes_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.marketplacenotes().list(opt.value_of("order-id").unwrap_or(""));
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

    fn _marketplaceoffers_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.marketplaceoffers().get(opt.value_of("offer-id").unwrap_or(""));
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

    fn _marketplaceoffers_search(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.marketplaceoffers().search();
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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

    fn _marketplaceorders_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.marketplaceorders().get(opt.value_of("order-id").unwrap_or(""));
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

    fn _marketplaceorders_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.marketplaceorders().insert(request);
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

    fn _marketplaceorders_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "order-id" => Some(("orderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "revision-time-ms" => Some(("revisionTimeMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "buyer-private-data.reference-id" => Some(("buyerPrivateData.referenceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "buyer-private-data.reference-payload" => Some(("buyerPrivateData.referencePayload", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "billed-buyer.account-id" => Some(("billedBuyer.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "originator-role" => Some(("originatorRole", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "seller.sub-account-id" => Some(("seller.subAccountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "seller.account-id" => Some(("seller.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-updater-role" => Some(("lastUpdaterRole", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "order-state" => Some(("orderState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "has-seller-signed-off" => Some(("hasSellerSignedOff", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "is-renegotiating" => Some(("isRenegotiating", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "buyer.account-id" => Some(("buyer.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "has-buyer-signed-off" => Some(("hasBuyerSignedOff", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "is-setup-complete" => Some(("isSetupComplete", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "revision-number" => Some(("revisionNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-updater-or-commentor-role" => Some(("lastUpdaterOrCommentorRole", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "billed-buyer", "buyer", "buyer-private-data", "has-buyer-signed-off", "has-seller-signed-off", "is-renegotiating", "is-setup-complete", "kind", "last-updater-or-commentor-role", "last-updater-role", "name", "order-id", "order-state", "originator-role", "reference-id", "reference-payload", "revision-number", "revision-time-ms", "seller", "sub-account-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::MarketplaceOrder = json::value::from_value(object).unwrap();
        let mut call = self.hub.marketplaceorders().patch(request, opt.value_of("order-id").unwrap_or(""), opt.value_of("revision-number").unwrap_or(""), opt.value_of("update-action").unwrap_or(""));
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

    fn _marketplaceorders_search(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.marketplaceorders().search();
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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

    fn _marketplaceorders_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "order-id" => Some(("orderId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "revision-time-ms" => Some(("revisionTimeMs", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "buyer-private-data.reference-id" => Some(("buyerPrivateData.referenceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "buyer-private-data.reference-payload" => Some(("buyerPrivateData.referencePayload", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "billed-buyer.account-id" => Some(("billedBuyer.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "originator-role" => Some(("originatorRole", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "seller.sub-account-id" => Some(("seller.subAccountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "seller.account-id" => Some(("seller.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-updater-role" => Some(("lastUpdaterRole", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "order-state" => Some(("orderState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "has-seller-signed-off" => Some(("hasSellerSignedOff", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "is-renegotiating" => Some(("isRenegotiating", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "buyer.account-id" => Some(("buyer.accountId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "has-buyer-signed-off" => Some(("hasBuyerSignedOff", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "is-setup-complete" => Some(("isSetupComplete", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "revision-number" => Some(("revisionNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "last-updater-or-commentor-role" => Some(("lastUpdaterOrCommentorRole", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "billed-buyer", "buyer", "buyer-private-data", "has-buyer-signed-off", "has-seller-signed-off", "is-renegotiating", "is-setup-complete", "kind", "last-updater-or-commentor-role", "last-updater-role", "name", "order-id", "order-state", "originator-role", "reference-id", "reference-payload", "revision-number", "revision-time-ms", "seller", "sub-account-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::MarketplaceOrder = json::value::from_value(object).unwrap();
        let mut call = self.hub.marketplaceorders().update(request, opt.value_of("order-id").unwrap_or(""), opt.value_of("revision-number").unwrap_or(""), opt.value_of("update-action").unwrap_or(""));
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

    fn _negotiationrounds_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.finalize-automatically" => Some(("terms.finalizeAutomatically", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "terms.inventory-segment-targeting.positive-icm-interests" => Some(("terms.inventorySegmentTargeting.positiveIcmInterests", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.positive-inventory-slots" => Some(("terms.inventorySegmentTargeting.positiveInventorySlots", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.negative-site-urls" => Some(("terms.inventorySegmentTargeting.negativeSiteUrls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.positive-icm-brands" => Some(("terms.inventorySegmentTargeting.positiveIcmBrands", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.negative-xfp-placements" => Some(("terms.inventorySegmentTargeting.negativeXfpPlacements", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.positive-video-ad-position-segments" => Some(("terms.inventorySegmentTargeting.positiveVideoAdPositionSegments", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.negative-icm-interests" => Some(("terms.inventorySegmentTargeting.negativeIcmInterests", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.negative-audience-segments" => Some(("terms.inventorySegmentTargeting.negativeAudienceSegments", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.positive-operating-systems" => Some(("terms.inventorySegmentTargeting.positiveOperatingSystems", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.negative-operating-systems" => Some(("terms.inventorySegmentTargeting.negativeOperatingSystems", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.negative-icm-brands" => Some(("terms.inventorySegmentTargeting.negativeIcmBrands", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.positive-locations" => Some(("terms.inventorySegmentTargeting.positiveLocations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.negative-mobile-apps" => Some(("terms.inventorySegmentTargeting.negativeMobileApps", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.positive-device-categories" => Some(("terms.inventorySegmentTargeting.positiveDeviceCategories", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.negative-inventory-slots" => Some(("terms.inventorySegmentTargeting.negativeInventorySlots", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.negative-video-ad-position-segments" => Some(("terms.inventorySegmentTargeting.negativeVideoAdPositionSegments", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.negative-video-duration-segments" => Some(("terms.inventorySegmentTargeting.negativeVideoDurationSegments", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.positive-sizes" => Some(("terms.inventorySegmentTargeting.positiveSizes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.positive-xfp-ad-slots" => Some(("terms.inventorySegmentTargeting.positiveXfpAdSlots", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.positive-site-urls" => Some(("terms.inventorySegmentTargeting.positiveSiteUrls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.positive-mobile-apps" => Some(("terms.inventorySegmentTargeting.positiveMobileApps", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.negative-sizes" => Some(("terms.inventorySegmentTargeting.negativeSizes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.positive-operating-system-versions" => Some(("terms.inventorySegmentTargeting.positiveOperatingSystemVersions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.negative-locations" => Some(("terms.inventorySegmentTargeting.negativeLocations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.negative-operating-system-versions" => Some(("terms.inventorySegmentTargeting.negativeOperatingSystemVersions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.positive-xfp-placements" => Some(("terms.inventorySegmentTargeting.positiveXfpPlacements", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.negative-xfp-ad-slots" => Some(("terms.inventorySegmentTargeting.negativeXfpAdSlots", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.positive-video-duration-segments" => Some(("terms.inventorySegmentTargeting.positiveVideoDurationSegments", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.positive-audience-segments" => Some(("terms.inventorySegmentTargeting.positiveAudienceSegments", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.negative-device-categories" => Some(("terms.inventorySegmentTargeting.negativeDeviceCategories", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.positive-ad-type-segments" => Some(("terms.inventorySegmentTargeting.positiveAdTypeSegments", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.negative-ad-type-segments" => Some(("terms.inventorySegmentTargeting.negativeAdTypeSegments", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.end-date.time-zone-id" => Some(("terms.endDate.timeZoneId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.end-date.hour" => Some(("terms.endDate.hour", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "terms.end-date.month" => Some(("terms.endDate.month", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "terms.end-date.second" => Some(("terms.endDate.second", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "terms.end-date.year" => Some(("terms.endDate.year", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "terms.end-date.day" => Some(("terms.endDate.day", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "terms.end-date.minute" => Some(("terms.endDate.minute", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "terms.terms-attributes" => Some(("terms.termsAttributes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.start-date.time-zone-id" => Some(("terms.startDate.timeZoneId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.start-date.hour" => Some(("terms.startDate.hour", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "terms.start-date.month" => Some(("terms.startDate.month", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "terms.start-date.second" => Some(("terms.startDate.second", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "terms.start-date.year" => Some(("terms.startDate.year", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "terms.start-date.day" => Some(("terms.startDate.day", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "terms.start-date.minute" => Some(("terms.startDate.minute", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "terms.buyer-billing-type" => Some(("terms.buyerBillingType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.estimated-impressions-per-day" => Some(("terms.estimatedImpressionsPerDay", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.monetizer-type" => Some(("terms.monetizerType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.target-by-deal-id" => Some(("terms.targetByDealId", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "terms.minimum-spend-micros" => Some(("terms.minimumSpendMicros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.targeting-all-ad-slots" => Some(("terms.targetingAllAdSlots", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "terms.creative-review-policy" => Some(("terms.creativeReviewPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.audience-segment-description" => Some(("terms.audienceSegmentDescription", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.deal-premium.micros" => Some(("terms.dealPremium.micros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.deal-premium.currency-code" => Some(("terms.dealPremium.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.creative-blocking-level" => Some(("terms.creativeBlockingLevel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.audience-segment.num-cookies" => Some(("terms.audienceSegment.numCookies", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.audience-segment.description" => Some(("terms.audienceSegment.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.audience-segment.name" => Some(("terms.audienceSegment.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.audience-segment.id" => Some(("terms.audienceSegment.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.description" => Some(("terms.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.billing-terms" => Some(("terms.billingTerms", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.is-reservation" => Some(("terms.isReservation", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "terms.semi-transparent" => Some(("terms.semiTransparent", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "terms.minimum-true-looks" => Some(("terms.minimumTrueLooks", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.cpm.micros" => Some(("terms.cpm.micros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.cpm.currency-code" => Some(("terms.cpm.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.descriptive-name" => Some(("terms.descriptiveName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.urls" => Some(("terms.urls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.estimated-spend.micros" => Some(("terms.estimatedSpend.micros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.estimated-spend.currency-code" => Some(("terms.estimatedSpend.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "round-number" => Some(("roundNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "notes" => Some(("notes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "originator-role" => Some(("originatorRole", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "negotiation-id" => Some(("negotiationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "edit-history.created-time-stamp" => Some(("editHistory.createdTimeStamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "edit-history.created-by-login-name" => Some(("editHistory.createdByLoginName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "edit-history.last-updated-by-login-name" => Some(("editHistory.lastUpdatedByLoginName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "edit-history.last-update-time-stamp" => Some(("editHistory.lastUpdateTimeStamp", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dbm-partner-id" => Some(("dbmPartnerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "action" => Some(("action", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["action", "audience-segment", "audience-segment-description", "billing-terms", "buyer-billing-type", "cpm", "created-by-login-name", "created-time-stamp", "creative-blocking-level", "creative-review-policy", "currency-code", "day", "dbm-partner-id", "deal-premium", "description", "descriptive-name", "edit-history", "end-date", "estimated-impressions-per-day", "estimated-spend", "finalize-automatically", "hour", "id", "inventory-segment-targeting", "is-reservation", "kind", "last-update-time-stamp", "last-updated-by-login-name", "micros", "minimum-spend-micros", "minimum-true-looks", "minute", "monetizer-type", "month", "name", "negative-ad-type-segments", "negative-audience-segments", "negative-device-categories", "negative-icm-brands", "negative-icm-interests", "negative-inventory-slots", "negative-locations", "negative-mobile-apps", "negative-operating-system-versions", "negative-operating-systems", "negative-site-urls", "negative-sizes", "negative-video-ad-position-segments", "negative-video-duration-segments", "negative-xfp-ad-slots", "negative-xfp-placements", "negotiation-id", "notes", "num-cookies", "originator-role", "positive-ad-type-segments", "positive-audience-segments", "positive-device-categories", "positive-icm-brands", "positive-icm-interests", "positive-inventory-slots", "positive-locations", "positive-mobile-apps", "positive-operating-system-versions", "positive-operating-systems", "positive-site-urls", "positive-sizes", "positive-video-ad-position-segments", "positive-video-duration-segments", "positive-xfp-ad-slots", "positive-xfp-placements", "round-number", "second", "semi-transparent", "start-date", "target-by-deal-id", "targeting-all-ad-slots", "terms", "terms-attributes", "time-zone-id", "urls", "year"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::NegotiationRoundDto = json::value::from_value(object).unwrap();
        let mut call = self.hub.negotiationrounds().insert(request, opt.value_of("negotiation-id").unwrap_or(""));
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

    fn _negotiations_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "include-private-auctions" => Some(("includePrivateAuctions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["include-private-auctions"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GetNegotiationByIdRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.negotiations().get(request, opt.value_of("negotiation-id").unwrap_or(""));
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

    fn _negotiations_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "status" => Some(("status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "buyer-email-contacts" => Some(("buyerEmailContacts", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "label-names" => Some(("labelNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "billed-buyer.buyer.enabled-for-preferred-deals" => Some(("billedBuyer.buyer.enabledForPreferredDeals", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "billed-buyer.buyer.display-name" => Some(("billedBuyer.buyer.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "billed-buyer.buyer.customer-id" => Some(("billedBuyer.buyer.customerId", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "billed-buyer.buyer.enabled-for-interest-targeting-deals" => Some(("billedBuyer.buyer.enabledForInterestTargetingDeals", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "billed-buyer.buyer.sponsor-account-id" => Some(("billedBuyer.buyer.sponsorAccountId", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "billed-buyer.buyer.id" => Some(("billedBuyer.buyer.id", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "billed-buyer.buyer.account-id" => Some(("billedBuyer.buyer.accountId", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "billed-buyer.web-property.enabled-for-preferred-deals" => Some(("billedBuyer.webProperty.enabledForPreferredDeals", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "billed-buyer.web-property.name" => Some(("billedBuyer.webProperty.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "billed-buyer.web-property.syndication-product" => Some(("billedBuyer.webProperty.syndicationProduct", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "billed-buyer.web-property.allow-interest-targeted-ads" => Some(("billedBuyer.webProperty.allowInterestTargetedAds", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "billed-buyer.web-property.site-urls" => Some(("billedBuyer.webProperty.siteUrls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "billed-buyer.web-property.property-code" => Some(("billedBuyer.webProperty.propertyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "billed-buyer.web-property.id" => Some(("billedBuyer.webProperty.id", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "billed-buyer.customer-id" => Some(("billedBuyer.customerId", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "billed-buyer.name" => Some(("billedBuyer.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "billed-buyer.buyer-seller-role" => Some(("billedBuyer.buyerSellerRole", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "offer-id" => Some(("offerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "seller.buyer.enabled-for-preferred-deals" => Some(("seller.buyer.enabledForPreferredDeals", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "seller.buyer.display-name" => Some(("seller.buyer.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "seller.buyer.customer-id" => Some(("seller.buyer.customerId", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "seller.buyer.enabled-for-interest-targeting-deals" => Some(("seller.buyer.enabledForInterestTargetingDeals", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "seller.buyer.sponsor-account-id" => Some(("seller.buyer.sponsorAccountId", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "seller.buyer.id" => Some(("seller.buyer.id", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "seller.buyer.account-id" => Some(("seller.buyer.accountId", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "seller.web-property.enabled-for-preferred-deals" => Some(("seller.webProperty.enabledForPreferredDeals", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "seller.web-property.name" => Some(("seller.webProperty.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "seller.web-property.syndication-product" => Some(("seller.webProperty.syndicationProduct", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "seller.web-property.allow-interest-targeted-ads" => Some(("seller.webProperty.allowInterestTargetedAds", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "seller.web-property.site-urls" => Some(("seller.webProperty.siteUrls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "seller.web-property.property-code" => Some(("seller.webProperty.propertyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "seller.web-property.id" => Some(("seller.webProperty.id", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "seller.customer-id" => Some(("seller.customerId", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "seller.name" => Some(("seller.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "seller.buyer-seller-role" => Some(("seller.buyerSellerRole", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "negotiation-id" => Some(("negotiationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "negotiation-state" => Some(("negotiationState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "buyer.buyer.enabled-for-preferred-deals" => Some(("buyer.buyer.enabledForPreferredDeals", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "buyer.buyer.display-name" => Some(("buyer.buyer.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "buyer.buyer.customer-id" => Some(("buyer.buyer.customerId", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "buyer.buyer.enabled-for-interest-targeting-deals" => Some(("buyer.buyer.enabledForInterestTargetingDeals", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "buyer.buyer.sponsor-account-id" => Some(("buyer.buyer.sponsorAccountId", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "buyer.buyer.id" => Some(("buyer.buyer.id", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "buyer.buyer.account-id" => Some(("buyer.buyer.accountId", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "buyer.web-property.enabled-for-preferred-deals" => Some(("buyer.webProperty.enabledForPreferredDeals", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "buyer.web-property.name" => Some(("buyer.webProperty.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "buyer.web-property.syndication-product" => Some(("buyer.webProperty.syndicationProduct", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "buyer.web-property.allow-interest-targeted-ads" => Some(("buyer.webProperty.allowInterestTargetedAds", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "buyer.web-property.site-urls" => Some(("buyer.webProperty.siteUrls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "buyer.web-property.property-code" => Some(("buyer.webProperty.propertyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "buyer.web-property.id" => Some(("buyer.webProperty.id", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "buyer.customer-id" => Some(("buyer.customerId", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "buyer.name" => Some(("buyer.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "buyer.buyer-seller-role" => Some(("buyer.buyerSellerRole", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "stats.revenue.micros" => Some(("stats.revenue.micros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "stats.revenue.currency-code" => Some(("stats.revenue.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "stats.bids" => Some(("stats.bids", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "stats.impressions" => Some(("stats.impressions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "stats.requests" => Some(("stats.requests", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "stats.good-bids" => Some(("stats.goodBids", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "stats.spend.micros" => Some(("stats.spend.micros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "stats.spend.currency-code" => Some(("stats.spend.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "external-deal-id" => Some(("externalDealId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "seller-email-contacts" => Some(("sellerEmailContacts", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "deal-type" => Some(("dealType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "allow-interest-targeted-ads", "bids", "billed-buyer", "buyer", "buyer-email-contacts", "buyer-seller-role", "currency-code", "customer-id", "deal-type", "display-name", "enabled-for-interest-targeting-deals", "enabled-for-preferred-deals", "external-deal-id", "good-bids", "id", "impressions", "kind", "label-names", "micros", "name", "negotiation-id", "negotiation-state", "offer-id", "property-code", "requests", "revenue", "seller", "seller-email-contacts", "site-urls", "spend", "sponsor-account-id", "stats", "status", "syndication-product", "web-property"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::NegotiationDto = json::value::from_value(object).unwrap();
        let mut call = self.hub.negotiations().insert(request);
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

    fn _negotiations_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "since-timestamp-millis" => Some(("sinceTimestampMillis", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "include-private-auctions" => Some(("includePrivateAuctions", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "finalized" => Some(("finalized", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["finalized", "include-private-auctions", "since-timestamp-millis"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GetNegotiationsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.negotiations().list(request);
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

    fn _offers_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.offers().get(opt.value_of("offer-id").unwrap_or(""));
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

    fn _offers_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "status" => Some(("status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "label-names" => Some(("labelNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "point-of-contact" => Some(("pointOfContact", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "is-open" => Some(("isOpen", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "creator.buyer.enabled-for-preferred-deals" => Some(("creator.buyer.enabledForPreferredDeals", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "creator.buyer.display-name" => Some(("creator.buyer.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creator.buyer.customer-id" => Some(("creator.buyer.customerId", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "creator.buyer.enabled-for-interest-targeting-deals" => Some(("creator.buyer.enabledForInterestTargetingDeals", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "creator.buyer.sponsor-account-id" => Some(("creator.buyer.sponsorAccountId", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "creator.buyer.id" => Some(("creator.buyer.id", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "creator.buyer.account-id" => Some(("creator.buyer.accountId", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "creator.web-property.enabled-for-preferred-deals" => Some(("creator.webProperty.enabledForPreferredDeals", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "creator.web-property.name" => Some(("creator.webProperty.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creator.web-property.syndication-product" => Some(("creator.webProperty.syndicationProduct", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creator.web-property.allow-interest-targeted-ads" => Some(("creator.webProperty.allowInterestTargetedAds", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "creator.web-property.site-urls" => Some(("creator.webProperty.siteUrls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "creator.web-property.property-code" => Some(("creator.webProperty.propertyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creator.web-property.id" => Some(("creator.webProperty.id", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "creator.customer-id" => Some(("creator.customerId", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "creator.name" => Some(("creator.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "creator.buyer-seller-role" => Some(("creator.buyerSellerRole", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "billed-buyer.buyer.enabled-for-preferred-deals" => Some(("billedBuyer.buyer.enabledForPreferredDeals", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "billed-buyer.buyer.display-name" => Some(("billedBuyer.buyer.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "billed-buyer.buyer.customer-id" => Some(("billedBuyer.buyer.customerId", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "billed-buyer.buyer.enabled-for-interest-targeting-deals" => Some(("billedBuyer.buyer.enabledForInterestTargetingDeals", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "billed-buyer.buyer.sponsor-account-id" => Some(("billedBuyer.buyer.sponsorAccountId", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "billed-buyer.buyer.id" => Some(("billedBuyer.buyer.id", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "billed-buyer.buyer.account-id" => Some(("billedBuyer.buyer.accountId", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "billed-buyer.web-property.enabled-for-preferred-deals" => Some(("billedBuyer.webProperty.enabledForPreferredDeals", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "billed-buyer.web-property.name" => Some(("billedBuyer.webProperty.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "billed-buyer.web-property.syndication-product" => Some(("billedBuyer.webProperty.syndicationProduct", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "billed-buyer.web-property.allow-interest-targeted-ads" => Some(("billedBuyer.webProperty.allowInterestTargetedAds", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "billed-buyer.web-property.site-urls" => Some(("billedBuyer.webProperty.siteUrls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "billed-buyer.web-property.property-code" => Some(("billedBuyer.webProperty.propertyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "billed-buyer.web-property.id" => Some(("billedBuyer.webProperty.id", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "billed-buyer.customer-id" => Some(("billedBuyer.customerId", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "billed-buyer.name" => Some(("billedBuyer.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "billed-buyer.buyer-seller-role" => Some(("billedBuyer.buyerSellerRole", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "offer-id" => Some(("offerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "offer-state" => Some(("offerState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "anonymous" => Some(("anonymous", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "terms.finalize-automatically" => Some(("terms.finalizeAutomatically", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "terms.inventory-segment-targeting.positive-icm-interests" => Some(("terms.inventorySegmentTargeting.positiveIcmInterests", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.positive-inventory-slots" => Some(("terms.inventorySegmentTargeting.positiveInventorySlots", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.negative-site-urls" => Some(("terms.inventorySegmentTargeting.negativeSiteUrls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.positive-icm-brands" => Some(("terms.inventorySegmentTargeting.positiveIcmBrands", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.negative-xfp-placements" => Some(("terms.inventorySegmentTargeting.negativeXfpPlacements", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.positive-video-ad-position-segments" => Some(("terms.inventorySegmentTargeting.positiveVideoAdPositionSegments", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.negative-icm-interests" => Some(("terms.inventorySegmentTargeting.negativeIcmInterests", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.negative-audience-segments" => Some(("terms.inventorySegmentTargeting.negativeAudienceSegments", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.positive-operating-systems" => Some(("terms.inventorySegmentTargeting.positiveOperatingSystems", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.negative-operating-systems" => Some(("terms.inventorySegmentTargeting.negativeOperatingSystems", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.negative-icm-brands" => Some(("terms.inventorySegmentTargeting.negativeIcmBrands", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.positive-locations" => Some(("terms.inventorySegmentTargeting.positiveLocations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.negative-mobile-apps" => Some(("terms.inventorySegmentTargeting.negativeMobileApps", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.positive-device-categories" => Some(("terms.inventorySegmentTargeting.positiveDeviceCategories", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.negative-inventory-slots" => Some(("terms.inventorySegmentTargeting.negativeInventorySlots", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.negative-video-ad-position-segments" => Some(("terms.inventorySegmentTargeting.negativeVideoAdPositionSegments", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.negative-video-duration-segments" => Some(("terms.inventorySegmentTargeting.negativeVideoDurationSegments", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.positive-sizes" => Some(("terms.inventorySegmentTargeting.positiveSizes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.positive-xfp-ad-slots" => Some(("terms.inventorySegmentTargeting.positiveXfpAdSlots", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.positive-site-urls" => Some(("terms.inventorySegmentTargeting.positiveSiteUrls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.positive-mobile-apps" => Some(("terms.inventorySegmentTargeting.positiveMobileApps", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.negative-sizes" => Some(("terms.inventorySegmentTargeting.negativeSizes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.positive-operating-system-versions" => Some(("terms.inventorySegmentTargeting.positiveOperatingSystemVersions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.negative-locations" => Some(("terms.inventorySegmentTargeting.negativeLocations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.negative-operating-system-versions" => Some(("terms.inventorySegmentTargeting.negativeOperatingSystemVersions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.positive-xfp-placements" => Some(("terms.inventorySegmentTargeting.positiveXfpPlacements", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.negative-xfp-ad-slots" => Some(("terms.inventorySegmentTargeting.negativeXfpAdSlots", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.positive-video-duration-segments" => Some(("terms.inventorySegmentTargeting.positiveVideoDurationSegments", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.positive-audience-segments" => Some(("terms.inventorySegmentTargeting.positiveAudienceSegments", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.negative-device-categories" => Some(("terms.inventorySegmentTargeting.negativeDeviceCategories", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.positive-ad-type-segments" => Some(("terms.inventorySegmentTargeting.positiveAdTypeSegments", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.inventory-segment-targeting.negative-ad-type-segments" => Some(("terms.inventorySegmentTargeting.negativeAdTypeSegments", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.end-date.time-zone-id" => Some(("terms.endDate.timeZoneId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.end-date.hour" => Some(("terms.endDate.hour", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "terms.end-date.month" => Some(("terms.endDate.month", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "terms.end-date.second" => Some(("terms.endDate.second", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "terms.end-date.year" => Some(("terms.endDate.year", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "terms.end-date.day" => Some(("terms.endDate.day", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "terms.end-date.minute" => Some(("terms.endDate.minute", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "terms.terms-attributes" => Some(("terms.termsAttributes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.start-date.time-zone-id" => Some(("terms.startDate.timeZoneId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.start-date.hour" => Some(("terms.startDate.hour", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "terms.start-date.month" => Some(("terms.startDate.month", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "terms.start-date.second" => Some(("terms.startDate.second", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "terms.start-date.year" => Some(("terms.startDate.year", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "terms.start-date.day" => Some(("terms.startDate.day", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "terms.start-date.minute" => Some(("terms.startDate.minute", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "terms.buyer-billing-type" => Some(("terms.buyerBillingType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.estimated-impressions-per-day" => Some(("terms.estimatedImpressionsPerDay", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.monetizer-type" => Some(("terms.monetizerType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.target-by-deal-id" => Some(("terms.targetByDealId", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "terms.minimum-spend-micros" => Some(("terms.minimumSpendMicros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.targeting-all-ad-slots" => Some(("terms.targetingAllAdSlots", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "terms.creative-review-policy" => Some(("terms.creativeReviewPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.audience-segment-description" => Some(("terms.audienceSegmentDescription", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.deal-premium.micros" => Some(("terms.dealPremium.micros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.deal-premium.currency-code" => Some(("terms.dealPremium.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.creative-blocking-level" => Some(("terms.creativeBlockingLevel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.audience-segment.num-cookies" => Some(("terms.audienceSegment.numCookies", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.audience-segment.description" => Some(("terms.audienceSegment.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.audience-segment.name" => Some(("terms.audienceSegment.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.audience-segment.id" => Some(("terms.audienceSegment.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.description" => Some(("terms.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.billing-terms" => Some(("terms.billingTerms", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.is-reservation" => Some(("terms.isReservation", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "terms.semi-transparent" => Some(("terms.semiTransparent", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "terms.minimum-true-looks" => Some(("terms.minimumTrueLooks", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.cpm.micros" => Some(("terms.cpm.micros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.cpm.currency-code" => Some(("terms.cpm.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.descriptive-name" => Some(("terms.descriptiveName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.urls" => Some(("terms.urls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "terms.estimated-spend.micros" => Some(("terms.estimatedSpend.micros", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "terms.estimated-spend.currency-code" => Some(("terms.estimatedSpend.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "email-contacts" => Some(("emailContacts", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-id", "allow-interest-targeted-ads", "anonymous", "audience-segment", "audience-segment-description", "billed-buyer", "billing-terms", "buyer", "buyer-billing-type", "buyer-seller-role", "cpm", "creative-blocking-level", "creative-review-policy", "creator", "currency-code", "customer-id", "day", "deal-premium", "description", "descriptive-name", "display-name", "email-contacts", "enabled-for-interest-targeting-deals", "enabled-for-preferred-deals", "end-date", "estimated-impressions-per-day", "estimated-spend", "finalize-automatically", "hour", "id", "inventory-segment-targeting", "is-open", "is-reservation", "kind", "label-names", "micros", "minimum-spend-micros", "minimum-true-looks", "minute", "monetizer-type", "month", "name", "negative-ad-type-segments", "negative-audience-segments", "negative-device-categories", "negative-icm-brands", "negative-icm-interests", "negative-inventory-slots", "negative-locations", "negative-mobile-apps", "negative-operating-system-versions", "negative-operating-systems", "negative-site-urls", "negative-sizes", "negative-video-ad-position-segments", "negative-video-duration-segments", "negative-xfp-ad-slots", "negative-xfp-placements", "num-cookies", "offer-id", "offer-state", "point-of-contact", "positive-ad-type-segments", "positive-audience-segments", "positive-device-categories", "positive-icm-brands", "positive-icm-interests", "positive-inventory-slots", "positive-locations", "positive-mobile-apps", "positive-operating-system-versions", "positive-operating-systems", "positive-site-urls", "positive-sizes", "positive-video-ad-position-segments", "positive-video-duration-segments", "positive-xfp-ad-slots", "positive-xfp-placements", "property-code", "second", "semi-transparent", "site-urls", "sponsor-account-id", "start-date", "status", "syndication-product", "target-by-deal-id", "targeting-all-ad-slots", "terms", "terms-attributes", "time-zone-id", "urls", "web-property", "year"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::OfferDto = json::value::from_value(object).unwrap();
        let mut call = self.hub.offers().insert(request);
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

    fn _offers_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "since-timestamp-millis" => Some(("sinceTimestampMillis", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["since-timestamp-millis"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ListOffersRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.offers().list(request);
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

    fn _pretargeting_config_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "billing-id" => Some(("billingId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "languages" => Some(("languages", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config-name" => Some(("configName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "excluded-geo-criteria-ids" => Some(("excludedGeoCriteriaIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "user-lists" => Some(("userLists", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "excluded-verticals" => Some(("excludedVerticals", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "vendor-types" => Some(("vendorTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "excluded-content-labels" => Some(("excludedContentLabels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "verticals" => Some(("verticals", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "platforms" => Some(("platforms", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "mobile-devices" => Some(("mobileDevices", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "creative-type" => Some(("creativeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "geo-criteria-ids" => Some(("geoCriteriaIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "mobile-operating-system-versions" => Some(("mobileOperatingSystemVersions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "mobile-carriers" => Some(("mobileCarriers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config-id" => Some(("configId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "excluded-user-lists" => Some(("excludedUserLists", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "is-active" => Some(("isActive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "supported-creative-attributes" => Some(("supportedCreativeAttributes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["billing-id", "config-id", "config-name", "creative-type", "excluded-content-labels", "excluded-geo-criteria-ids", "excluded-user-lists", "excluded-verticals", "geo-criteria-ids", "is-active", "kind", "languages", "mobile-carriers", "mobile-devices", "mobile-operating-system-versions", "platforms", "supported-creative-attributes", "user-lists", "vendor-types", "verticals"]);
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

    fn _pretargeting_config_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "billing-id" => Some(("billingId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "languages" => Some(("languages", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config-name" => Some(("configName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "excluded-geo-criteria-ids" => Some(("excludedGeoCriteriaIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "user-lists" => Some(("userLists", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "excluded-verticals" => Some(("excludedVerticals", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "vendor-types" => Some(("vendorTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "excluded-content-labels" => Some(("excludedContentLabels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "verticals" => Some(("verticals", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "platforms" => Some(("platforms", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "mobile-devices" => Some(("mobileDevices", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "creative-type" => Some(("creativeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "geo-criteria-ids" => Some(("geoCriteriaIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "mobile-operating-system-versions" => Some(("mobileOperatingSystemVersions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "mobile-carriers" => Some(("mobileCarriers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config-id" => Some(("configId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "excluded-user-lists" => Some(("excludedUserLists", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "is-active" => Some(("isActive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "supported-creative-attributes" => Some(("supportedCreativeAttributes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["billing-id", "config-id", "config-name", "creative-type", "excluded-content-labels", "excluded-geo-criteria-ids", "excluded-user-lists", "excluded-verticals", "geo-criteria-ids", "is-active", "kind", "languages", "mobile-carriers", "mobile-devices", "mobile-operating-system-versions", "platforms", "supported-creative-attributes", "user-lists", "vendor-types", "verticals"]);
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

    fn _pretargeting_config_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "billing-id" => Some(("billingId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "languages" => Some(("languages", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config-name" => Some(("configName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "excluded-geo-criteria-ids" => Some(("excludedGeoCriteriaIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "user-lists" => Some(("userLists", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "excluded-verticals" => Some(("excludedVerticals", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "vendor-types" => Some(("vendorTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "excluded-content-labels" => Some(("excludedContentLabels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "verticals" => Some(("verticals", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "platforms" => Some(("platforms", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "mobile-devices" => Some(("mobileDevices", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "creative-type" => Some(("creativeType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "geo-criteria-ids" => Some(("geoCriteriaIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "mobile-operating-system-versions" => Some(("mobileOperatingSystemVersions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "mobile-carriers" => Some(("mobileCarriers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config-id" => Some(("configId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "excluded-user-lists" => Some(("excludedUserLists", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "is-active" => Some(("isActive", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "supported-creative-attributes" => Some(("supportedCreativeAttributes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["billing-id", "config-id", "config-name", "creative-type", "excluded-content-labels", "excluded-geo-criteria-ids", "excluded-user-lists", "excluded-verticals", "geo-criteria-ids", "is-active", "kind", "languages", "mobile-carriers", "mobile-devices", "mobile-operating-system-versions", "platforms", "supported-creative-attributes", "user-lists", "vendor-types", "verticals"]);
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
            ("clientaccess", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._clientaccess_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._clientaccess_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._clientaccess_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._clientaccess_list(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._clientaccess_patch(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._clientaccess_update(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("clientaccess".to_string()));
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
            ("deals", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._deals_get(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("deals".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("marketplacedeals", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._marketplacedeals_delete(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._marketplacedeals_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._marketplacedeals_list(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._marketplacedeals_update(opt, dry_run, &mut err);
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
                        call_result = self._marketplacenotes_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._marketplacenotes_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("marketplacenotes".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("marketplaceoffers", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._marketplaceoffers_get(opt, dry_run, &mut err);
                    },
                    ("search", Some(opt)) => {
                        call_result = self._marketplaceoffers_search(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("marketplaceoffers".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("marketplaceorders", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._marketplaceorders_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._marketplaceorders_insert(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._marketplaceorders_patch(opt, dry_run, &mut err);
                    },
                    ("search", Some(opt)) => {
                        call_result = self._marketplaceorders_search(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._marketplaceorders_update(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("marketplaceorders".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("negotiationrounds", Some(opt)) => {
                match opt.subcommand() {
                    ("insert", Some(opt)) => {
                        call_result = self._negotiationrounds_insert(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("negotiationrounds".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("negotiations", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._negotiations_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._negotiations_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._negotiations_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("negotiations".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("offers", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._offers_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._offers_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._offers_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("offers".to_string()));
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

            match cmn::application_secret_from_directory(&config_dir, "adexchangebuyer1d4-secret.json", 
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
                                          program_name: "adexchangebuyer1d4",
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
        
        ("clientaccess", "methods: 'delete', 'get', 'insert', 'list', 'patch' and 'update'", vec![
            ("delete",  
                    None,
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/clientaccess_delete",
                  vec![
                    (Some(r##"client-account-id"##),
                     None,
                     None,
                     Some(true),
                     Some(false)),
        
                    (Some(r##"sponsor-account-id"##),
                     None,
                     None,
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",  
                    None,
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/clientaccess_get",
                  vec![
                    (Some(r##"client-account-id"##),
                     None,
                     None,
                     Some(true),
                     Some(false)),
        
                    (Some(r##"sponsor-account-id"##),
                     None,
                     None,
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
                    None,
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/clientaccess_insert",
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
                    None,
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/clientaccess_list",
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
                    None,
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/clientaccess_patch",
                  vec![
                    (Some(r##"client-account-id"##),
                     None,
                     None,
                     Some(true),
                     Some(false)),
        
                    (Some(r##"sponsor-account-id"##),
                     None,
                     None,
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
                    None,
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/clientaccess_update",
                  vec![
                    (Some(r##"client-account-id"##),
                     None,
                     None,
                     Some(true),
                     Some(false)),
        
                    (Some(r##"sponsor-account-id"##),
                     None,
                     None,
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
            ]),
        
        ("deals", "methods: 'get'", vec![
            ("get",  
                    Some(r##"Gets the requested deal."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/deals_get",
                  vec![
                    (Some(r##"deal-id"##),
                     None,
                     None,
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
        
        ("marketplacedeals", "methods: 'delete', 'insert', 'list' and 'update'", vec![
            ("delete",  
                    Some(r##"Delete the specified deals from the order"##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/marketplacedeals_delete",
                  vec![
                    (Some(r##"order-id"##),
                     None,
                     Some(r##"The orderId to delete deals from."##),
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
                    Some(r##"Add new deals for the specified order"##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/marketplacedeals_insert",
                  vec![
                    (Some(r##"order-id"##),
                     None,
                     Some(r##"OrderId for which deals need to be added."##),
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
                    Some(r##"List all the deals for a given order"##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/marketplacedeals_list",
                  vec![
                    (Some(r##"order-id"##),
                     None,
                     Some(r##"The orderId to get deals for."##),
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
                    Some(r##"Replaces all the deals in the order with the passed in deals"##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/marketplacedeals_update",
                  vec![
                    (Some(r##"order-id"##),
                     None,
                     Some(r##"The orderId to edit deals on."##),
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
                    Some(r##"Add notes to the order"##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/marketplacenotes_insert",
                  vec![
                    (Some(r##"order-id"##),
                     None,
                     Some(r##"The orderId to add notes for."##),
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
                    Some(r##"Get all the notes associated with an order"##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/marketplacenotes_list",
                  vec![
                    (Some(r##"order-id"##),
                     None,
                     Some(r##"The orderId to get notes for."##),
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
        
        ("marketplaceoffers", "methods: 'get' and 'search'", vec![
            ("get",  
                    Some(r##"Gets the requested negotiation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/marketplaceoffers_get",
                  vec![
                    (Some(r##"offer-id"##),
                     None,
                     Some(r##"The offerId for the offer to get the head revision for."##),
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
                    Some(r##"Gets the requested negotiation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/marketplaceoffers_search",
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
        
        ("marketplaceorders", "methods: 'get', 'insert', 'patch', 'search' and 'update'", vec![
            ("get",  
                    Some(r##"Get an order given its id"##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/marketplaceorders_get",
                  vec![
                    (Some(r##"order-id"##),
                     None,
                     Some(r##"Id of the order to retrieve."##),
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
                    Some(r##"Create the given list of orders"##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/marketplaceorders_insert",
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
                    Some(r##"Update the given order. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/marketplaceorders_patch",
                  vec![
                    (Some(r##"order-id"##),
                     None,
                     Some(r##"The order id to update."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"revision-number"##),
                     None,
                     Some(r##"The last known revision number to update. If the head revision in the marketplace database has since changed, an error will be thrown. The caller should then fetch the lastest order at head revision and retry the update at that revision."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"update-action"##),
                     None,
                     Some(r##"The proposed action to take on the order."##),
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
                    Some(r##"Search for orders using pql query"##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/marketplaceorders_search",
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
            ("update",  
                    Some(r##"Update the given order"##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/marketplaceorders_update",
                  vec![
                    (Some(r##"order-id"##),
                     None,
                     Some(r##"The order id to update."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"revision-number"##),
                     None,
                     Some(r##"The last known revision number to update. If the head revision in the marketplace database has since changed, an error will be thrown. The caller should then fetch the lastest order at head revision and retry the update at that revision."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"update-action"##),
                     None,
                     Some(r##"The proposed action to take on the order."##),
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
        
        ("negotiationrounds", "methods: 'insert'", vec![
            ("insert",  
                    Some(r##"Adds the requested negotiationRound to the requested negotiation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/negotiationrounds_insert",
                  vec![
                    (Some(r##"negotiation-id"##),
                     None,
                     None,
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
        
        ("negotiations", "methods: 'get', 'insert' and 'list'", vec![
            ("get",  
                    Some(r##"Gets the requested negotiation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/negotiations_get",
                  vec![
                    (Some(r##"negotiation-id"##),
                     None,
                     None,
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
                    Some(r##"Creates or updates the requested negotiation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/negotiations_insert",
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
                    Some(r##"Lists all negotiations the authenticated user has access to."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/negotiations_list",
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
        
        ("offers", "methods: 'get', 'insert' and 'list'", vec![
            ("get",  
                    Some(r##"Gets the requested offer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/offers_get",
                  vec![
                    (Some(r##"offer-id"##),
                     None,
                     None,
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
                    Some(r##"Creates or updates the requested offer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/offers_insert",
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
                    Some(r##"Lists all offers the authenticated user has access to."##),
                    "Details at http://byron.github.io/google-apis-rs/google_adexchangebuyer1d4_cli/offers_list",
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
        
    ];
    
    let mut app = App::new("adexchangebuyer1d4")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("0.3.2+20150909")
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