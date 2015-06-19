// DO NOT EDIT !
// This file was generated automatically from 'src/mako/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;
extern crate yup_oauth2 as oauth2;
extern crate yup_hyper_mock as mock;
extern crate serde;
extern crate hyper;
extern crate mime;
extern crate strsim;
extern crate google_sqladmin1_beta4 as api;

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
use serde::json;
use clap::ArgMatches;

enum DoitError {
    IoError(String, io::Error),
    ApiError(api::Error),
}

struct Engine<'n, 'a> {
    opt: ArgMatches<'n, 'a>,
    hub: api::SQLAdmin<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n, 'a> Engine<'n, 'a> {
    fn _backup_runs_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.backup_runs().get(opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""), opt.value_of("id").unwrap_or(""));
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
                                                                           v.extend([].iter().map(|v|*v));
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
                    Ok(())
                }
            }
        }
    }

    fn _backup_runs_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.backup_runs().list(opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
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
                    Ok(())
                }
            }
        }
    }

    fn _databases_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.databases().delete(opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""), opt.value_of("database").unwrap_or(""));
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
                                                                           v.extend([].iter().map(|v|*v));
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
                    Ok(())
                }
            }
        }
    }

    fn _databases_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.databases().get(opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""), opt.value_of("database").unwrap_or(""));
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
                                                                           v.extend([].iter().map(|v|*v));
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
                    Ok(())
                }
            }
        }
    }

    fn _databases_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
           
            let type_info = 
                match &temp_cursor.to_string()[..] {
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "charset" => Some(("charset", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project" => Some(("project", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance" => Some(("instance", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "collation" => Some(("collation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["charset", "collation", "etag", "instance", "kind", "name", "project", "self-link"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Database = json::value::from_value(object).unwrap();
        let mut call = self.hub.databases().insert(request, opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
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
                                                                           v.extend([].iter().map(|v|*v));
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
                    Ok(())
                }
            }
        }
    }

    fn _databases_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.databases().list(opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
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
                                                                           v.extend([].iter().map(|v|*v));
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
                    Ok(())
                }
            }
        }
    }

    fn _databases_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
           
            let type_info = 
                match &temp_cursor.to_string()[..] {
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "charset" => Some(("charset", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project" => Some(("project", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance" => Some(("instance", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "collation" => Some(("collation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["charset", "collation", "etag", "instance", "kind", "name", "project", "self-link"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Database = json::value::from_value(object).unwrap();
        let mut call = self.hub.databases().patch(request, opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""), opt.value_of("database").unwrap_or(""));
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
                                                                           v.extend([].iter().map(|v|*v));
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
                    Ok(())
                }
            }
        }
    }

    fn _databases_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
           
            let type_info = 
                match &temp_cursor.to_string()[..] {
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "charset" => Some(("charset", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project" => Some(("project", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance" => Some(("instance", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "collation" => Some(("collation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["charset", "collation", "etag", "instance", "kind", "name", "project", "self-link"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Database = json::value::from_value(object).unwrap();
        let mut call = self.hub.databases().update(request, opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""), opt.value_of("database").unwrap_or(""));
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
                                                                           v.extend([].iter().map(|v|*v));
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
                    Ok(())
                }
            }
        }
    }

    fn _flags_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.flags().list();
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
                                                                           v.extend([].iter().map(|v|*v));
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
                    Ok(())
                }
            }
        }
    }

    fn _instances_clone(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
           
            let type_info = 
                match &temp_cursor.to_string()[..] {
                    "clone-context.bin-log-coordinates.bin-log-position" => Some(("cloneContext.binLogCoordinates.binLogPosition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "clone-context.bin-log-coordinates.kind" => Some(("cloneContext.binLogCoordinates.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "clone-context.bin-log-coordinates.bin-log-file-name" => Some(("cloneContext.binLogCoordinates.binLogFileName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "clone-context.kind" => Some(("cloneContext.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "clone-context.destination-instance-name" => Some(("cloneContext.destinationInstanceName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["bin-log-coordinates", "bin-log-file-name", "bin-log-position", "clone-context", "destination-instance-name", "kind"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::InstancesCloneRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.instances().clone(request, opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
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
                                                                           v.extend([].iter().map(|v|*v));
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
                    Ok(())
                }
            }
        }
    }

    fn _instances_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.instances().delete(opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
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
                                                                           v.extend([].iter().map(|v|*v));
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
                    Ok(())
                }
            }
        }
    }

    fn _instances_export(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
           
            let type_info = 
                match &temp_cursor.to_string()[..] {
                    "export-context.kind" => Some(("exportContext.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "export-context.file-type" => Some(("exportContext.fileType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "export-context.uri" => Some(("exportContext.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "export-context.csv-export-options.select-query" => Some(("exportContext.csvExportOptions.selectQuery", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "export-context.databases" => Some(("exportContext.databases", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "export-context.sql-export-options.tables" => Some(("exportContext.sqlExportOptions.tables", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["csv-export-options", "databases", "export-context", "file-type", "kind", "select-query", "sql-export-options", "tables", "uri"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::InstancesExportRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.instances().export(request, opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
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
                                                                           v.extend([].iter().map(|v|*v));
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
                    Ok(())
                }
            }
        }
    }

    fn _instances_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.instances().get(opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
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
                                                                           v.extend([].iter().map(|v|*v));
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
                    Ok(())
                }
            }
        }
    }

    fn _instances_import(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
           
            let type_info = 
                match &temp_cursor.to_string()[..] {
                    "import-context.file-type" => Some(("importContext.fileType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "import-context.database" => Some(("importContext.database", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "import-context.uri" => Some(("importContext.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "import-context.kind" => Some(("importContext.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "import-context.csv-import-options.table" => Some(("importContext.csvImportOptions.table", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "import-context.csv-import-options.columns" => Some(("importContext.csvImportOptions.columns", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["columns", "csv-import-options", "database", "file-type", "import-context", "kind", "table", "uri"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::InstancesImportRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.instances().import(request, opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
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
                                                                           v.extend([].iter().map(|v|*v));
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
                    Ok(())
                }
            }
        }
    }

    fn _instances_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
           
            let type_info = 
                match &temp_cursor.to_string()[..] {
                    "on-premises-configuration.kind" => Some(("onPremisesConfiguration.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "on-premises-configuration.host-port" => Some(("onPremisesConfiguration.hostPort", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "max-disk-size" => Some(("maxDiskSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ipv6-address" => Some(("ipv6Address", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "server-ca-cert.cert-serial-number" => Some(("serverCaCert.certSerialNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "server-ca-cert.kind" => Some(("serverCaCert.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "server-ca-cert.sha1-fingerprint" => Some(("serverCaCert.sha1Fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "server-ca-cert.common-name" => Some(("serverCaCert.commonName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "server-ca-cert.instance" => Some(("serverCaCert.instance", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "server-ca-cert.cert" => Some(("serverCaCert.cert", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "server-ca-cert.expiration-time" => Some(("serverCaCert.expirationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "server-ca-cert.create-time" => Some(("serverCaCert.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "server-ca-cert.self-link" => Some(("serverCaCert.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replica-names" => Some(("replicaNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "project" => Some(("project", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "region" => Some(("region", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "settings.kind" => Some(("settings.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "settings.authorized-gae-applications" => Some(("settings.authorizedGaeApplications", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "settings.activation-policy" => Some(("settings.activationPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "settings.backup-configuration.kind" => Some(("settings.backupConfiguration.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "settings.backup-configuration.enabled" => Some(("settings.backupConfiguration.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "settings.backup-configuration.start-time" => Some(("settings.backupConfiguration.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "settings.backup-configuration.binary-log-enabled" => Some(("settings.backupConfiguration.binaryLogEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "settings.ip-configuration.ipv4-enabled" => Some(("settings.ipConfiguration.ipv4Enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "settings.ip-configuration.require-ssl" => Some(("settings.ipConfiguration.requireSsl", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "settings.tier" => Some(("settings.tier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "settings.database-replication-enabled" => Some(("settings.databaseReplicationEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "settings.replication-type" => Some(("settings.replicationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "settings.crash-safe-replication-enabled" => Some(("settings.crashSafeReplicationEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "settings.pricing-plan" => Some(("settings.pricingPlan", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "settings.settings-version" => Some(("settings.settingsVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "settings.location-preference.kind" => Some(("settings.locationPreference.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "settings.location-preference.zone" => Some(("settings.locationPreference.zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "settings.location-preference.follow-gae-application" => Some(("settings.locationPreference.followGaeApplication", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "master-instance-name" => Some(("masterInstanceName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "current-disk-size" => Some(("currentDiskSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-account-email-address" => Some(("serviceAccountEmailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replica-configuration.kind" => Some(("replicaConfiguration.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replica-configuration.mysql-replica-configuration.username" => Some(("replicaConfiguration.mysqlReplicaConfiguration.username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replica-configuration.mysql-replica-configuration.kind" => Some(("replicaConfiguration.mysqlReplicaConfiguration.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replica-configuration.mysql-replica-configuration.connect-retry-interval" => Some(("replicaConfiguration.mysqlReplicaConfiguration.connectRetryInterval", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "replica-configuration.mysql-replica-configuration.ssl-cipher" => Some(("replicaConfiguration.mysqlReplicaConfiguration.sslCipher", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replica-configuration.mysql-replica-configuration.ca-certificate" => Some(("replicaConfiguration.mysqlReplicaConfiguration.caCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replica-configuration.mysql-replica-configuration.client-certificate" => Some(("replicaConfiguration.mysqlReplicaConfiguration.clientCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replica-configuration.mysql-replica-configuration.master-heartbeat-period" => Some(("replicaConfiguration.mysqlReplicaConfiguration.masterHeartbeatPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replica-configuration.mysql-replica-configuration.verify-server-certificate" => Some(("replicaConfiguration.mysqlReplicaConfiguration.verifyServerCertificate", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "replica-configuration.mysql-replica-configuration.dump-file-path" => Some(("replicaConfiguration.mysqlReplicaConfiguration.dumpFilePath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replica-configuration.mysql-replica-configuration.password" => Some(("replicaConfiguration.mysqlReplicaConfiguration.password", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replica-configuration.mysql-replica-configuration.client-key" => Some(("replicaConfiguration.mysqlReplicaConfiguration.clientKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "database-version" => Some(("databaseVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-type" => Some(("instanceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["activation-policy", "authorized-gae-applications", "backup-configuration", "binary-log-enabled", "ca-certificate", "cert", "cert-serial-number", "client-certificate", "client-key", "common-name", "connect-retry-interval", "crash-safe-replication-enabled", "create-time", "current-disk-size", "database-replication-enabled", "database-version", "dump-file-path", "enabled", "etag", "expiration-time", "follow-gae-application", "host-port", "instance", "instance-type", "ip-configuration", "ipv4-enabled", "ipv6-address", "kind", "location-preference", "master-heartbeat-period", "master-instance-name", "max-disk-size", "mysql-replica-configuration", "name", "on-premises-configuration", "password", "pricing-plan", "project", "region", "replica-configuration", "replica-names", "replication-type", "require-ssl", "self-link", "server-ca-cert", "service-account-email-address", "settings", "settings-version", "sha1-fingerprint", "ssl-cipher", "start-time", "state", "tier", "username", "verify-server-certificate", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::DatabaseInstance = json::value::from_value(object).unwrap();
        let mut call = self.hub.instances().insert(request, opt.value_of("project").unwrap_or(""));
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
                                                                           v.extend([].iter().map(|v|*v));
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
                    Ok(())
                }
            }
        }
    }

    fn _instances_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.instances().list(opt.value_of("project").unwrap_or(""));
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
                    Ok(())
                }
            }
        }
    }

    fn _instances_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
           
            let type_info = 
                match &temp_cursor.to_string()[..] {
                    "on-premises-configuration.kind" => Some(("onPremisesConfiguration.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "on-premises-configuration.host-port" => Some(("onPremisesConfiguration.hostPort", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "max-disk-size" => Some(("maxDiskSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ipv6-address" => Some(("ipv6Address", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "server-ca-cert.cert-serial-number" => Some(("serverCaCert.certSerialNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "server-ca-cert.kind" => Some(("serverCaCert.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "server-ca-cert.sha1-fingerprint" => Some(("serverCaCert.sha1Fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "server-ca-cert.common-name" => Some(("serverCaCert.commonName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "server-ca-cert.instance" => Some(("serverCaCert.instance", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "server-ca-cert.cert" => Some(("serverCaCert.cert", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "server-ca-cert.expiration-time" => Some(("serverCaCert.expirationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "server-ca-cert.create-time" => Some(("serverCaCert.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "server-ca-cert.self-link" => Some(("serverCaCert.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replica-names" => Some(("replicaNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "project" => Some(("project", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "region" => Some(("region", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "settings.kind" => Some(("settings.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "settings.authorized-gae-applications" => Some(("settings.authorizedGaeApplications", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "settings.activation-policy" => Some(("settings.activationPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "settings.backup-configuration.kind" => Some(("settings.backupConfiguration.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "settings.backup-configuration.enabled" => Some(("settings.backupConfiguration.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "settings.backup-configuration.start-time" => Some(("settings.backupConfiguration.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "settings.backup-configuration.binary-log-enabled" => Some(("settings.backupConfiguration.binaryLogEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "settings.ip-configuration.ipv4-enabled" => Some(("settings.ipConfiguration.ipv4Enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "settings.ip-configuration.require-ssl" => Some(("settings.ipConfiguration.requireSsl", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "settings.tier" => Some(("settings.tier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "settings.database-replication-enabled" => Some(("settings.databaseReplicationEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "settings.replication-type" => Some(("settings.replicationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "settings.crash-safe-replication-enabled" => Some(("settings.crashSafeReplicationEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "settings.pricing-plan" => Some(("settings.pricingPlan", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "settings.settings-version" => Some(("settings.settingsVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "settings.location-preference.kind" => Some(("settings.locationPreference.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "settings.location-preference.zone" => Some(("settings.locationPreference.zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "settings.location-preference.follow-gae-application" => Some(("settings.locationPreference.followGaeApplication", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "master-instance-name" => Some(("masterInstanceName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "current-disk-size" => Some(("currentDiskSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-account-email-address" => Some(("serviceAccountEmailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replica-configuration.kind" => Some(("replicaConfiguration.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replica-configuration.mysql-replica-configuration.username" => Some(("replicaConfiguration.mysqlReplicaConfiguration.username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replica-configuration.mysql-replica-configuration.kind" => Some(("replicaConfiguration.mysqlReplicaConfiguration.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replica-configuration.mysql-replica-configuration.connect-retry-interval" => Some(("replicaConfiguration.mysqlReplicaConfiguration.connectRetryInterval", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "replica-configuration.mysql-replica-configuration.ssl-cipher" => Some(("replicaConfiguration.mysqlReplicaConfiguration.sslCipher", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replica-configuration.mysql-replica-configuration.ca-certificate" => Some(("replicaConfiguration.mysqlReplicaConfiguration.caCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replica-configuration.mysql-replica-configuration.client-certificate" => Some(("replicaConfiguration.mysqlReplicaConfiguration.clientCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replica-configuration.mysql-replica-configuration.master-heartbeat-period" => Some(("replicaConfiguration.mysqlReplicaConfiguration.masterHeartbeatPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replica-configuration.mysql-replica-configuration.verify-server-certificate" => Some(("replicaConfiguration.mysqlReplicaConfiguration.verifyServerCertificate", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "replica-configuration.mysql-replica-configuration.dump-file-path" => Some(("replicaConfiguration.mysqlReplicaConfiguration.dumpFilePath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replica-configuration.mysql-replica-configuration.password" => Some(("replicaConfiguration.mysqlReplicaConfiguration.password", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replica-configuration.mysql-replica-configuration.client-key" => Some(("replicaConfiguration.mysqlReplicaConfiguration.clientKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "database-version" => Some(("databaseVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-type" => Some(("instanceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["activation-policy", "authorized-gae-applications", "backup-configuration", "binary-log-enabled", "ca-certificate", "cert", "cert-serial-number", "client-certificate", "client-key", "common-name", "connect-retry-interval", "crash-safe-replication-enabled", "create-time", "current-disk-size", "database-replication-enabled", "database-version", "dump-file-path", "enabled", "etag", "expiration-time", "follow-gae-application", "host-port", "instance", "instance-type", "ip-configuration", "ipv4-enabled", "ipv6-address", "kind", "location-preference", "master-heartbeat-period", "master-instance-name", "max-disk-size", "mysql-replica-configuration", "name", "on-premises-configuration", "password", "pricing-plan", "project", "region", "replica-configuration", "replica-names", "replication-type", "require-ssl", "self-link", "server-ca-cert", "service-account-email-address", "settings", "settings-version", "sha1-fingerprint", "ssl-cipher", "start-time", "state", "tier", "username", "verify-server-certificate", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::DatabaseInstance = json::value::from_value(object).unwrap();
        let mut call = self.hub.instances().patch(request, opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
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
                                                                           v.extend([].iter().map(|v|*v));
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
                    Ok(())
                }
            }
        }
    }

    fn _instances_promote_replica(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.instances().promote_replica(opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
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
                                                                           v.extend([].iter().map(|v|*v));
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
                    Ok(())
                }
            }
        }
    }

    fn _instances_reset_ssl_config(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.instances().reset_ssl_config(opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
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
                                                                           v.extend([].iter().map(|v|*v));
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
                    Ok(())
                }
            }
        }
    }

    fn _instances_restart(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.instances().restart(opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
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
                                                                           v.extend([].iter().map(|v|*v));
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
                    Ok(())
                }
            }
        }
    }

    fn _instances_restore_backup(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
           
            let type_info = 
                match &temp_cursor.to_string()[..] {
                    "restore-backup-context.kind" => Some(("restoreBackupContext.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-backup-context.backup-run-id" => Some(("restoreBackupContext.backupRunId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["backup-run-id", "kind", "restore-backup-context"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::InstancesRestoreBackupRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.instances().restore_backup(request, opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
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
                                                                           v.extend([].iter().map(|v|*v));
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
                    Ok(())
                }
            }
        }
    }

    fn _instances_start_replica(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.instances().start_replica(opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
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
                                                                           v.extend([].iter().map(|v|*v));
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
                    Ok(())
                }
            }
        }
    }

    fn _instances_stop_replica(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.instances().stop_replica(opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
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
                                                                           v.extend([].iter().map(|v|*v));
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
                    Ok(())
                }
            }
        }
    }

    fn _instances_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
           
            let type_info = 
                match &temp_cursor.to_string()[..] {
                    "on-premises-configuration.kind" => Some(("onPremisesConfiguration.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "on-premises-configuration.host-port" => Some(("onPremisesConfiguration.hostPort", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "max-disk-size" => Some(("maxDiskSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ipv6-address" => Some(("ipv6Address", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "server-ca-cert.cert-serial-number" => Some(("serverCaCert.certSerialNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "server-ca-cert.kind" => Some(("serverCaCert.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "server-ca-cert.sha1-fingerprint" => Some(("serverCaCert.sha1Fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "server-ca-cert.common-name" => Some(("serverCaCert.commonName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "server-ca-cert.instance" => Some(("serverCaCert.instance", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "server-ca-cert.cert" => Some(("serverCaCert.cert", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "server-ca-cert.expiration-time" => Some(("serverCaCert.expirationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "server-ca-cert.create-time" => Some(("serverCaCert.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "server-ca-cert.self-link" => Some(("serverCaCert.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replica-names" => Some(("replicaNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "project" => Some(("project", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "region" => Some(("region", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "settings.kind" => Some(("settings.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "settings.authorized-gae-applications" => Some(("settings.authorizedGaeApplications", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "settings.activation-policy" => Some(("settings.activationPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "settings.backup-configuration.kind" => Some(("settings.backupConfiguration.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "settings.backup-configuration.enabled" => Some(("settings.backupConfiguration.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "settings.backup-configuration.start-time" => Some(("settings.backupConfiguration.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "settings.backup-configuration.binary-log-enabled" => Some(("settings.backupConfiguration.binaryLogEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "settings.ip-configuration.ipv4-enabled" => Some(("settings.ipConfiguration.ipv4Enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "settings.ip-configuration.require-ssl" => Some(("settings.ipConfiguration.requireSsl", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "settings.tier" => Some(("settings.tier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "settings.database-replication-enabled" => Some(("settings.databaseReplicationEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "settings.replication-type" => Some(("settings.replicationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "settings.crash-safe-replication-enabled" => Some(("settings.crashSafeReplicationEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "settings.pricing-plan" => Some(("settings.pricingPlan", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "settings.settings-version" => Some(("settings.settingsVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "settings.location-preference.kind" => Some(("settings.locationPreference.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "settings.location-preference.zone" => Some(("settings.locationPreference.zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "settings.location-preference.follow-gae-application" => Some(("settings.locationPreference.followGaeApplication", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "master-instance-name" => Some(("masterInstanceName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "current-disk-size" => Some(("currentDiskSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-account-email-address" => Some(("serviceAccountEmailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replica-configuration.kind" => Some(("replicaConfiguration.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replica-configuration.mysql-replica-configuration.username" => Some(("replicaConfiguration.mysqlReplicaConfiguration.username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replica-configuration.mysql-replica-configuration.kind" => Some(("replicaConfiguration.mysqlReplicaConfiguration.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replica-configuration.mysql-replica-configuration.connect-retry-interval" => Some(("replicaConfiguration.mysqlReplicaConfiguration.connectRetryInterval", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "replica-configuration.mysql-replica-configuration.ssl-cipher" => Some(("replicaConfiguration.mysqlReplicaConfiguration.sslCipher", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replica-configuration.mysql-replica-configuration.ca-certificate" => Some(("replicaConfiguration.mysqlReplicaConfiguration.caCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replica-configuration.mysql-replica-configuration.client-certificate" => Some(("replicaConfiguration.mysqlReplicaConfiguration.clientCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replica-configuration.mysql-replica-configuration.master-heartbeat-period" => Some(("replicaConfiguration.mysqlReplicaConfiguration.masterHeartbeatPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replica-configuration.mysql-replica-configuration.verify-server-certificate" => Some(("replicaConfiguration.mysqlReplicaConfiguration.verifyServerCertificate", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "replica-configuration.mysql-replica-configuration.dump-file-path" => Some(("replicaConfiguration.mysqlReplicaConfiguration.dumpFilePath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replica-configuration.mysql-replica-configuration.password" => Some(("replicaConfiguration.mysqlReplicaConfiguration.password", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "replica-configuration.mysql-replica-configuration.client-key" => Some(("replicaConfiguration.mysqlReplicaConfiguration.clientKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "database-version" => Some(("databaseVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-type" => Some(("instanceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["activation-policy", "authorized-gae-applications", "backup-configuration", "binary-log-enabled", "ca-certificate", "cert", "cert-serial-number", "client-certificate", "client-key", "common-name", "connect-retry-interval", "crash-safe-replication-enabled", "create-time", "current-disk-size", "database-replication-enabled", "database-version", "dump-file-path", "enabled", "etag", "expiration-time", "follow-gae-application", "host-port", "instance", "instance-type", "ip-configuration", "ipv4-enabled", "ipv6-address", "kind", "location-preference", "master-heartbeat-period", "master-instance-name", "max-disk-size", "mysql-replica-configuration", "name", "on-premises-configuration", "password", "pricing-plan", "project", "region", "replica-configuration", "replica-names", "replication-type", "require-ssl", "self-link", "server-ca-cert", "service-account-email-address", "settings", "settings-version", "sha1-fingerprint", "ssl-cipher", "start-time", "state", "tier", "username", "verify-server-certificate", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::DatabaseInstance = json::value::from_value(object).unwrap();
        let mut call = self.hub.instances().update(request, opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
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
                                                                           v.extend([].iter().map(|v|*v));
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
                    Ok(())
                }
            }
        }
    }

    fn _operations_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.operations().get(opt.value_of("project").unwrap_or(""), opt.value_of("operation").unwrap_or(""));
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
                                                                           v.extend([].iter().map(|v|*v));
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
                    Ok(())
                }
            }
        }
    }

    fn _operations_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.operations().list(opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
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
                    Ok(())
                }
            }
        }
    }

    fn _ssl_certs_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.ssl_certs().delete(opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""), opt.value_of("sha1-fingerprint").unwrap_or(""));
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
                                                                           v.extend([].iter().map(|v|*v));
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
                    Ok(())
                }
            }
        }
    }

    fn _ssl_certs_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.ssl_certs().get(opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""), opt.value_of("sha1-fingerprint").unwrap_or(""));
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
                                                                           v.extend([].iter().map(|v|*v));
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
                    Ok(())
                }
            }
        }
    }

    fn _ssl_certs_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
           
            let type_info = 
                match &temp_cursor.to_string()[..] {
                    "common-name" => Some(("commonName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["common-name"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SslCertsInsertRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.ssl_certs().insert(request, opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
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
                                                                           v.extend([].iter().map(|v|*v));
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
                    Ok(())
                }
            }
        }
    }

    fn _ssl_certs_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.ssl_certs().list(opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
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
                                                                           v.extend([].iter().map(|v|*v));
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
                    Ok(())
                }
            }
        }
    }

    fn _tiers_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.tiers().list(opt.value_of("project").unwrap_or(""));
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
                                                                           v.extend([].iter().map(|v|*v));
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
                    Ok(())
                }
            }
        }
    }

    fn _users_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.users().delete(opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""), opt.value_of("host").unwrap_or(""), opt.value_of("name").unwrap_or(""));
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
                                                                           v.extend([].iter().map(|v|*v));
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
                    Ok(())
                }
            }
        }
    }

    fn _users_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
           
            let type_info = 
                match &temp_cursor.to_string()[..] {
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project" => Some(("project", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance" => Some(("instance", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "host" => Some(("host", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "password" => Some(("password", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["etag", "host", "instance", "kind", "name", "password", "project"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::User = json::value::from_value(object).unwrap();
        let mut call = self.hub.users().insert(request, opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
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
                                                                           v.extend([].iter().map(|v|*v));
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
                    Ok(())
                }
            }
        }
    }

    fn _users_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.users().list(opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""));
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
                                                                           v.extend([].iter().map(|v|*v));
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
                    Ok(())
                }
            }
        }
    }

    fn _users_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
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
           
            let type_info = 
                match &temp_cursor.to_string()[..] {
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project" => Some(("project", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance" => Some(("instance", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "host" => Some(("host", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "password" => Some(("password", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["etag", "host", "instance", "kind", "name", "password", "project"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::User = json::value::from_value(object).unwrap();
        let mut call = self.hub.users().update(request, opt.value_of("project").unwrap_or(""), opt.value_of("instance").unwrap_or(""), opt.value_of("host").unwrap_or(""), opt.value_of("name").unwrap_or(""));
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
                                                                           v.extend([].iter().map(|v|*v));
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
            ("backup-runs", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._backup_runs_get(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._backup_runs_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("backup-runs".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("databases", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._databases_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._databases_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._databases_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._databases_list(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._databases_patch(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._databases_update(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("databases".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("flags", Some(opt)) => {
                match opt.subcommand() {
                    ("list", Some(opt)) => {
                        call_result = self._flags_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("flags".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("instances", Some(opt)) => {
                match opt.subcommand() {
                    ("clone", Some(opt)) => {
                        call_result = self._instances_clone(opt, dry_run, &mut err);
                    },
                    ("delete", Some(opt)) => {
                        call_result = self._instances_delete(opt, dry_run, &mut err);
                    },
                    ("export", Some(opt)) => {
                        call_result = self._instances_export(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._instances_get(opt, dry_run, &mut err);
                    },
                    ("import", Some(opt)) => {
                        call_result = self._instances_import(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._instances_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._instances_list(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._instances_patch(opt, dry_run, &mut err);
                    },
                    ("promote-replica", Some(opt)) => {
                        call_result = self._instances_promote_replica(opt, dry_run, &mut err);
                    },
                    ("reset-ssl-config", Some(opt)) => {
                        call_result = self._instances_reset_ssl_config(opt, dry_run, &mut err);
                    },
                    ("restart", Some(opt)) => {
                        call_result = self._instances_restart(opt, dry_run, &mut err);
                    },
                    ("restore-backup", Some(opt)) => {
                        call_result = self._instances_restore_backup(opt, dry_run, &mut err);
                    },
                    ("start-replica", Some(opt)) => {
                        call_result = self._instances_start_replica(opt, dry_run, &mut err);
                    },
                    ("stop-replica", Some(opt)) => {
                        call_result = self._instances_stop_replica(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._instances_update(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("instances".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("operations", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._operations_get(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._operations_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("operations".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("ssl-certs", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._ssl_certs_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._ssl_certs_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._ssl_certs_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._ssl_certs_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("ssl-certs".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("tiers", Some(opt)) => {
                match opt.subcommand() {
                    ("list", Some(opt)) => {
                        call_result = self._tiers_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("tiers".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("users", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._users_delete(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._users_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._users_list(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._users_update(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("users".to_string()));
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

            match cmn::application_secret_from_directory(&config_dir, "sqladmin1-beta4-secret.json", 
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
                                          program_name: "sqladmin1-beta4",
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
            hub: api::SQLAdmin::new(client, auth),
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
        ("backup-runs", "methods: 'get' and 'list'", vec![
            ("get",  
                    Some(r##"Retrieves a resource containing information about a backup run."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/backup-runs_get",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"id"##),
                     None,
                     Some(r##"The ID of this Backup Run."##),
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
                    Some(r##"Lists all backup runs associated with a given instance and configuration in the reverse chronological order of the enqueued time."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/backup-runs_list",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
        
        ("databases", "methods: 'delete', 'get', 'insert', 'list', 'patch' and 'update'", vec![
            ("delete",  
                    Some(r##"Deletes a resource containing information about a database inside a Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/databases_delete",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Database instance ID. This does not include the project ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"database"##),
                     None,
                     Some(r##"Name of the database to be deleted in the instance."##),
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
                    Some(r##"Retrieves a resource containing information about a database inside a Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/databases_get",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Database instance ID. This does not include the project ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"database"##),
                     None,
                     Some(r##"Name of the database in the instance."##),
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
                    Some(r##"Inserts a resource containing information about a database inside a Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/databases_insert",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Database instance ID. This does not include the project ID."##),
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
                    Some(r##"Lists databases in the specified Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/databases_list",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project for which to list Cloud SQL instances."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
                    Some(r##"Updates a resource containing information about a database inside a Cloud SQL instance. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/databases_patch",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Database instance ID. This does not include the project ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"database"##),
                     None,
                     Some(r##"Name of the database to be updated in the instance."##),
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
                    Some(r##"Updates a resource containing information about a database inside a Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/databases_update",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Database instance ID. This does not include the project ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"database"##),
                     None,
                     Some(r##"Name of the database to be updated in the instance."##),
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
        
        ("flags", "methods: 'list'", vec![
            ("list",  
                    Some(r##"List all available database flags for Google Cloud SQL instances."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/flags_list",
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
        
        ("instances", "methods: 'clone', 'delete', 'export', 'get', 'import', 'insert', 'list', 'patch', 'promote-replica', 'reset-ssl-config', 'restart', 'restore-backup', 'start-replica', 'stop-replica' and 'update'", vec![
            ("clone",  
                    Some(r##"Creates a Cloud SQL instance as a clone of the source instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/instances_clone",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the source as well as the clone Cloud SQL instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"The ID of the Cloud SQL instance to be cloned (source). This does not include the project ID."##),
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
            ("delete",  
                    Some(r##"Deletes a Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/instances_delete",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance to be deleted."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
            ("export",  
                    Some(r##"Exports data from a Cloud SQL instance to a Google Cloud Storage bucket as a MySQL dump file."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/instances_export",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance to be exported."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
                    Some(r##"Retrieves a resource containing information about a Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/instances_get",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Database instance ID. This does not include the project ID."##),
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
            ("import",  
                    Some(r##"Imports data into a Cloud SQL instance from a MySQL dump file in Google Cloud Storage."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/instances_import",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
                    Some(r##"Creates a new Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/instances_insert",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project to which the newly created Cloud SQL instances should belong."##),
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
                    Some(r##"Lists instances under a given project in the alphabetical order of the instance name."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/instances_list",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project for which to list Cloud SQL instances."##),
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
                    Some(r##"Updates settings of a Cloud SQL instance. Caution: This is not a partial update, so you must include values for all the settings that you want to retain. For partial updates, use patch.. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/instances_patch",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
            ("promote-replica",  
                    Some(r##"Promotes the read replica instance to be a stand-alone Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/instances_promote-replica",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"ID of the project that contains the read replica."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL read replica instance name."##),
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
            ("reset-ssl-config",  
                    Some(r##"Deletes all client certificates and generates a new server SSL certificate for the instance. The changes will not take effect until the instance is restarted. Existing instances without a server certificate will need to call this once to set a server certificate."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/instances_reset-ssl-config",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
            ("restart",  
                    Some(r##"Restarts a Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/instances_restart",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance to be restarted."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
            ("restore-backup",  
                    Some(r##"Restores a backup of a Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/instances_restore-backup",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
            ("start-replica",  
                    Some(r##"Starts the replication in the read replica instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/instances_start-replica",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"ID of the project that contains the read replica."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL read replica instance name."##),
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
            ("stop-replica",  
                    Some(r##"Stops the replication in the read replica instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/instances_stop-replica",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"ID of the project that contains the read replica."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL read replica instance name."##),
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
                    Some(r##"Updates settings of a Cloud SQL instance. Caution: This is not a partial update, so you must include values for all the settings that you want to retain. For partial updates, use patch."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/instances_update",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
        
        ("operations", "methods: 'get' and 'list'", vec![
            ("get",  
                    Some(r##"Retrieves an instance operation that has been performed on an instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/operations_get",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"operation"##),
                     None,
                     Some(r##"Instance operation ID."##),
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
                    Some(r##"Lists all instance operations that have been performed on the given Cloud SQL instance in the reverse chronological order of the start time."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/operations_list",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
        
        ("ssl-certs", "methods: 'delete', 'get', 'insert' and 'list'", vec![
            ("delete",  
                    Some(r##"Deletes the SSL certificate. The change will not take effect until the instance is restarted."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/ssl-certs_delete",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance to be deleted."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"sha1-fingerprint"##),
                     None,
                     Some(r##"Sha1 FingerPrint."##),
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
                    Some(r##"Retrieves a particular SSL certificate. Does not include the private key (required for usage). The private key must be saved from the response to initial creation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/ssl-certs_get",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"sha1-fingerprint"##),
                     None,
                     Some(r##"Sha1 FingerPrint."##),
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
                    Some(r##"Creates an SSL certificate and returns it along with the private key and server certificate authority. The new certificate will not be usable until the instance is restarted."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/ssl-certs_insert",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project to which the newly created Cloud SQL instances should belong."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
                    Some(r##"Lists all of the current SSL certificates for the instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/ssl-certs_list",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project for which to list Cloud SQL instances."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
        
        ("tiers", "methods: 'list'", vec![
            ("list",  
                    Some(r##"Lists all available service tiers for Google Cloud SQL, for example D1, D2. For related information, see Pricing."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/tiers_list",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project for which to list tiers."##),
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
        
        ("users", "methods: 'delete', 'insert', 'list' and 'update'", vec![
            ("delete",  
                    Some(r##"Deletes a user from a Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/users_delete",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Database instance ID. This does not include the project ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"host"##),
                     None,
                     Some(r##"Host of the user in the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"name"##),
                     None,
                     Some(r##"Name of the user in the instance."##),
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
                    Some(r##"Creates a new user in a Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/users_insert",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Database instance ID. This does not include the project ID."##),
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
                    Some(r##"Lists users in the specified Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/users_list",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Database instance ID. This does not include the project ID."##),
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
                    Some(r##"Updates an existing user in a Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli/users_update",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Database instance ID. This does not include the project ID."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"host"##),
                     None,
                     Some(r##"Host of the user in the instance."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"name"##),
                     None,
                     Some(r##"Name of the user in the instance."##),
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
    
    let mut app = App::new("sqladmin1-beta4")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("0.3.0+20150305")
           .about("API for Cloud SQL database instance management.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_sqladmin1_beta4_cli")
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
                            writeln!(io::stderr(), "{:?}", err).ok();
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