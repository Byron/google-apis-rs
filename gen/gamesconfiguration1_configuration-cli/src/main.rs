// DO NOT EDIT !
// This file was generated automatically from 'src/mako/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;
extern crate yup_oauth2 as oauth2;
extern crate yup_hyper_mock as mock;
extern crate hyper_rustls;
extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate mime;
extern crate strsim;
extern crate google_gamesconfiguration1_configuration as api;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

mod cmn;

use cmn::{InvalidOptionsError, CLIError, JsonTokenStorage, arg_from_str, writer_from_opts, parse_kv_arg,
          input_file_from_opts, input_mime_from_opts, FieldCursor, FieldError, CallType, UploadProtocol,
          calltype_from_str, remove_json_null_values, ComplexType, JsonType, JsonTypeInfo};

use std::default::Default;
use std::str::FromStr;

use oauth2::{Authenticator, DefaultAuthenticatorDelegate, FlowType};
use serde_json as json;
use clap::ArgMatches;

enum DoitError {
    IoError(String, io::Error),
    ApiError(api::Error),
}

struct Engine<'n> {
    opt: ArgMatches<'n>,
    hub: api::GamesConfiguration<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n> Engine<'n> {
    fn _achievement_configurations_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.achievement_configurations().delete(opt.value_of("achievement-id").unwrap_or(""));
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

    fn _achievement_configurations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.achievement_configurations().get(opt.value_of("achievement-id").unwrap_or(""));
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
                CallType::Standard => call.doit(),
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

    fn _achievement_configurations_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "achievement-type" => Some(("achievementType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "steps-to-unlock" => Some(("stepsToUnlock", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "initial-state" => Some(("initialState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "token" => Some(("token", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.kind" => Some(("draft.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.description.kind" => Some(("draft.description.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.icon-url" => Some(("draft.iconUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.point-value" => Some(("draft.pointValue", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "draft.sort-rank" => Some(("draft.sortRank", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "draft.name.kind" => Some(("draft.name.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.kind" => Some(("published.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.description.kind" => Some(("published.description.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.icon-url" => Some(("published.iconUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.point-value" => Some(("published.pointValue", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "published.sort-rank" => Some(("published.sortRank", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "published.name.kind" => Some(("published.name.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["achievement-type", "description", "draft", "icon-url", "id", "initial-state", "kind", "name", "point-value", "published", "sort-rank", "steps-to-unlock", "token"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::AchievementConfiguration = json::value::from_value(object).unwrap();
        let mut call = self.hub.achievement_configurations().insert(request, opt.value_of("application-id").unwrap_or(""));
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
                CallType::Standard => call.doit(),
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

    fn _achievement_configurations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.achievement_configurations().list(opt.value_of("application-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _achievement_configurations_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "achievement-type" => Some(("achievementType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "steps-to-unlock" => Some(("stepsToUnlock", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "initial-state" => Some(("initialState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "token" => Some(("token", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.kind" => Some(("draft.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.description.kind" => Some(("draft.description.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.icon-url" => Some(("draft.iconUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.point-value" => Some(("draft.pointValue", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "draft.sort-rank" => Some(("draft.sortRank", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "draft.name.kind" => Some(("draft.name.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.kind" => Some(("published.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.description.kind" => Some(("published.description.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.icon-url" => Some(("published.iconUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.point-value" => Some(("published.pointValue", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "published.sort-rank" => Some(("published.sortRank", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "published.name.kind" => Some(("published.name.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["achievement-type", "description", "draft", "icon-url", "id", "initial-state", "kind", "name", "point-value", "published", "sort-rank", "steps-to-unlock", "token"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::AchievementConfiguration = json::value::from_value(object).unwrap();
        let mut call = self.hub.achievement_configurations().patch(request, opt.value_of("achievement-id").unwrap_or(""));
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
                CallType::Standard => call.doit(),
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

    fn _achievement_configurations_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "achievement-type" => Some(("achievementType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "steps-to-unlock" => Some(("stepsToUnlock", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "initial-state" => Some(("initialState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "token" => Some(("token", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.kind" => Some(("draft.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.description.kind" => Some(("draft.description.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.icon-url" => Some(("draft.iconUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.point-value" => Some(("draft.pointValue", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "draft.sort-rank" => Some(("draft.sortRank", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "draft.name.kind" => Some(("draft.name.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.kind" => Some(("published.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.description.kind" => Some(("published.description.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.icon-url" => Some(("published.iconUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.point-value" => Some(("published.pointValue", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "published.sort-rank" => Some(("published.sortRank", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "published.name.kind" => Some(("published.name.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["achievement-type", "description", "draft", "icon-url", "id", "initial-state", "kind", "name", "point-value", "published", "sort-rank", "steps-to-unlock", "token"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::AchievementConfiguration = json::value::from_value(object).unwrap();
        let mut call = self.hub.achievement_configurations().update(request, opt.value_of("achievement-id").unwrap_or(""));
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
                CallType::Standard => call.doit(),
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

    fn _image_configurations_upload(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.image_configurations().upload(opt.value_of("resource-id").unwrap_or(""), opt.value_of("image-type").unwrap_or(""));
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
        let vals = opt.values_of("mode").unwrap().collect::<Vec<&str>>();
        let protocol = calltype_from_str(vals[0], ["simple", "resumable"].iter().map(|&v| v.to_string()).collect(), err);
        let mut input_file = input_file_from_opts(vals[1], err);
        let mime_type = input_mime_from_opts(opt.value_of("mime").unwrap_or("application/octet-stream"), err);
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
                CallType::Upload(UploadProtocol::Simple) => call.upload(input_file.unwrap(), mime_type.unwrap()),
                CallType::Upload(UploadProtocol::Resumable) => call.upload_resumable(input_file.unwrap(), mime_type.unwrap()),
                CallType::Standard => unreachable!()
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

    fn _leaderboard_configurations_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.leaderboard_configurations().delete(opt.value_of("leaderboard-id").unwrap_or(""));
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

    fn _leaderboard_configurations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.leaderboard_configurations().get(opt.value_of("leaderboard-id").unwrap_or(""));
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
                CallType::Standard => call.doit(),
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

    fn _leaderboard_configurations_insert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "score-order" => Some(("scoreOrder", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "score-min" => Some(("scoreMin", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "token" => Some(("token", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "score-max" => Some(("scoreMax", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.score-format.currency-code" => Some(("published.scoreFormat.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.score-format.suffix.many.kind" => Some(("published.scoreFormat.suffix.many.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.score-format.suffix.two.kind" => Some(("published.scoreFormat.suffix.two.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.score-format.suffix.one.kind" => Some(("published.scoreFormat.suffix.one.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.score-format.suffix.few.kind" => Some(("published.scoreFormat.suffix.few.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.score-format.suffix.zero.kind" => Some(("published.scoreFormat.suffix.zero.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.score-format.suffix.other.kind" => Some(("published.scoreFormat.suffix.other.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.score-format.number-format-type" => Some(("published.scoreFormat.numberFormatType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.score-format.num-decimal-places" => Some(("published.scoreFormat.numDecimalPlaces", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "published.icon-url" => Some(("published.iconUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.kind" => Some(("published.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.name.kind" => Some(("published.name.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.sort-rank" => Some(("published.sortRank", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "draft.score-format.currency-code" => Some(("draft.scoreFormat.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.score-format.suffix.many.kind" => Some(("draft.scoreFormat.suffix.many.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.score-format.suffix.two.kind" => Some(("draft.scoreFormat.suffix.two.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.score-format.suffix.one.kind" => Some(("draft.scoreFormat.suffix.one.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.score-format.suffix.few.kind" => Some(("draft.scoreFormat.suffix.few.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.score-format.suffix.zero.kind" => Some(("draft.scoreFormat.suffix.zero.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.score-format.suffix.other.kind" => Some(("draft.scoreFormat.suffix.other.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.score-format.number-format-type" => Some(("draft.scoreFormat.numberFormatType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.score-format.num-decimal-places" => Some(("draft.scoreFormat.numDecimalPlaces", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "draft.icon-url" => Some(("draft.iconUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.kind" => Some(("draft.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.name.kind" => Some(("draft.name.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.sort-rank" => Some(("draft.sortRank", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["currency-code", "draft", "few", "icon-url", "id", "kind", "many", "name", "num-decimal-places", "number-format-type", "one", "other", "published", "score-format", "score-max", "score-min", "score-order", "sort-rank", "suffix", "token", "two", "zero"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::LeaderboardConfiguration = json::value::from_value(object).unwrap();
        let mut call = self.hub.leaderboard_configurations().insert(request, opt.value_of("application-id").unwrap_or(""));
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
                CallType::Standard => call.doit(),
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

    fn _leaderboard_configurations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.leaderboard_configurations().list(opt.value_of("application-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _leaderboard_configurations_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "score-order" => Some(("scoreOrder", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "score-min" => Some(("scoreMin", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "token" => Some(("token", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "score-max" => Some(("scoreMax", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.score-format.currency-code" => Some(("published.scoreFormat.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.score-format.suffix.many.kind" => Some(("published.scoreFormat.suffix.many.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.score-format.suffix.two.kind" => Some(("published.scoreFormat.suffix.two.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.score-format.suffix.one.kind" => Some(("published.scoreFormat.suffix.one.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.score-format.suffix.few.kind" => Some(("published.scoreFormat.suffix.few.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.score-format.suffix.zero.kind" => Some(("published.scoreFormat.suffix.zero.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.score-format.suffix.other.kind" => Some(("published.scoreFormat.suffix.other.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.score-format.number-format-type" => Some(("published.scoreFormat.numberFormatType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.score-format.num-decimal-places" => Some(("published.scoreFormat.numDecimalPlaces", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "published.icon-url" => Some(("published.iconUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.kind" => Some(("published.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.name.kind" => Some(("published.name.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.sort-rank" => Some(("published.sortRank", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "draft.score-format.currency-code" => Some(("draft.scoreFormat.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.score-format.suffix.many.kind" => Some(("draft.scoreFormat.suffix.many.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.score-format.suffix.two.kind" => Some(("draft.scoreFormat.suffix.two.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.score-format.suffix.one.kind" => Some(("draft.scoreFormat.suffix.one.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.score-format.suffix.few.kind" => Some(("draft.scoreFormat.suffix.few.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.score-format.suffix.zero.kind" => Some(("draft.scoreFormat.suffix.zero.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.score-format.suffix.other.kind" => Some(("draft.scoreFormat.suffix.other.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.score-format.number-format-type" => Some(("draft.scoreFormat.numberFormatType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.score-format.num-decimal-places" => Some(("draft.scoreFormat.numDecimalPlaces", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "draft.icon-url" => Some(("draft.iconUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.kind" => Some(("draft.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.name.kind" => Some(("draft.name.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.sort-rank" => Some(("draft.sortRank", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["currency-code", "draft", "few", "icon-url", "id", "kind", "many", "name", "num-decimal-places", "number-format-type", "one", "other", "published", "score-format", "score-max", "score-min", "score-order", "sort-rank", "suffix", "token", "two", "zero"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::LeaderboardConfiguration = json::value::from_value(object).unwrap();
        let mut call = self.hub.leaderboard_configurations().patch(request, opt.value_of("leaderboard-id").unwrap_or(""));
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
                CallType::Standard => call.doit(),
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

    fn _leaderboard_configurations_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "score-order" => Some(("scoreOrder", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "score-min" => Some(("scoreMin", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "token" => Some(("token", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "score-max" => Some(("scoreMax", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.score-format.currency-code" => Some(("published.scoreFormat.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.score-format.suffix.many.kind" => Some(("published.scoreFormat.suffix.many.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.score-format.suffix.two.kind" => Some(("published.scoreFormat.suffix.two.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.score-format.suffix.one.kind" => Some(("published.scoreFormat.suffix.one.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.score-format.suffix.few.kind" => Some(("published.scoreFormat.suffix.few.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.score-format.suffix.zero.kind" => Some(("published.scoreFormat.suffix.zero.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.score-format.suffix.other.kind" => Some(("published.scoreFormat.suffix.other.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.score-format.number-format-type" => Some(("published.scoreFormat.numberFormatType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.score-format.num-decimal-places" => Some(("published.scoreFormat.numDecimalPlaces", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "published.icon-url" => Some(("published.iconUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.kind" => Some(("published.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.name.kind" => Some(("published.name.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "published.sort-rank" => Some(("published.sortRank", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "draft.score-format.currency-code" => Some(("draft.scoreFormat.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.score-format.suffix.many.kind" => Some(("draft.scoreFormat.suffix.many.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.score-format.suffix.two.kind" => Some(("draft.scoreFormat.suffix.two.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.score-format.suffix.one.kind" => Some(("draft.scoreFormat.suffix.one.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.score-format.suffix.few.kind" => Some(("draft.scoreFormat.suffix.few.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.score-format.suffix.zero.kind" => Some(("draft.scoreFormat.suffix.zero.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.score-format.suffix.other.kind" => Some(("draft.scoreFormat.suffix.other.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.score-format.number-format-type" => Some(("draft.scoreFormat.numberFormatType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.score-format.num-decimal-places" => Some(("draft.scoreFormat.numDecimalPlaces", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "draft.icon-url" => Some(("draft.iconUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.kind" => Some(("draft.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.name.kind" => Some(("draft.name.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "draft.sort-rank" => Some(("draft.sortRank", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["currency-code", "draft", "few", "icon-url", "id", "kind", "many", "name", "num-decimal-places", "number-format-type", "one", "other", "published", "score-format", "score-max", "score-min", "score-order", "sort-rank", "suffix", "token", "two", "zero"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::LeaderboardConfiguration = json::value::from_value(object).unwrap();
        let mut call = self.hub.leaderboard_configurations().update(request, opt.value_of("leaderboard-id").unwrap_or(""));
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
                CallType::Standard => call.doit(),
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

    fn _doit(&self, dry_run: bool) -> Result<Result<(), DoitError>, Option<InvalidOptionsError>> {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Result<(), DoitError> = Ok(());
        let mut err_opt: Option<InvalidOptionsError> = None;
        match self.opt.subcommand() {
            ("achievement-configurations", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._achievement_configurations_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._achievement_configurations_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._achievement_configurations_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._achievement_configurations_list(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._achievement_configurations_patch(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._achievement_configurations_update(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("achievement-configurations".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("image-configurations", Some(opt)) => {
                match opt.subcommand() {
                    ("upload", Some(opt)) => {
                        call_result = self._image_configurations_upload(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("image-configurations".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("leaderboard-configurations", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._leaderboard_configurations_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._leaderboard_configurations_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._leaderboard_configurations_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._leaderboard_configurations_list(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._leaderboard_configurations_patch(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._leaderboard_configurations_update(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("leaderboard-configurations".to_string()));
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
    fn new(opt: ArgMatches<'n>) -> Result<Engine<'n>, InvalidOptionsError> {
        let (config_dir, secret) = {
            let config_dir = match cmn::assure_config_dir_exists(opt.value_of("folder").unwrap_or("~/.google-service-cli")) {
                Err(e) => return Err(InvalidOptionsError::single(e, 3)),
                Ok(p) => p,
            };

            match cmn::application_secret_from_directory(&config_dir, "gamesconfiguration1-configuration-secret.json",
                                                         "{\"installed\":{\"auth_uri\":\"https://accounts.google.com/o/oauth2/auth\",\"client_secret\":\"hCsslbCUyfehWMmbkG8vTYxG\",\"token_uri\":\"https://accounts.google.com/o/oauth2/token\",\"client_email\":\"\",\"redirect_uris\":[\"urn:ietf:wg:oauth:2.0:oob\",\"oob\"],\"client_x509_cert_url\":\"\",\"client_id\":\"620010449518-9ngf7o4dhs0dka470npqvor6dc5lqb9b.apps.googleusercontent.com\",\"auth_provider_x509_cert_url\":\"https://www.googleapis.com/oauth2/v1/certs\"}}") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let auth = Authenticator::new(  &secret, DefaultAuthenticatorDelegate,
                                        if opt.is_present("debug-auth") {
                                            hyper::Client::with_connector(mock::TeeConnector {
                                                    connector: hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())
                                                })
                                        } else {
                                            hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new()))
                                        },
                                        JsonTokenStorage {
                                          program_name: "gamesconfiguration1-configuration",
                                          db_dir: config_dir.clone(),
                                        }, Some(FlowType::InstalledRedirect(54324)));

        let client =
            if opt.is_present("debug") {
                hyper::Client::with_connector(mock::TeeConnector {
                        connector: hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())
                    })
            } else {
                hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new()))
            };
        let engine = Engine {
            opt: opt,
            hub: api::GamesConfiguration::new(client, auth),
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
    let upload_value_names = ["mode", "file"];
    let arg_data = [
        ("achievement-configurations", "methods: 'delete', 'get', 'insert', 'list', 'patch' and 'update'", vec![
            ("delete",
                    Some(r##"Delete the achievement configuration with the given ID."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gamesconfiguration1_configuration_cli/achievement-configurations_delete",
                  vec![
                    (Some(r##"achievement-id"##),
                     None,
                     Some(r##"The ID of the achievement used by this method."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",
                    Some(r##"Retrieves the metadata of the achievement configuration with the given ID."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gamesconfiguration1_configuration_cli/achievement-configurations_get",
                  vec![
                    (Some(r##"achievement-id"##),
                     None,
                     Some(r##"The ID of the achievement used by this method."##),
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
                    Some(r##"Insert a new achievement configuration in this application."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gamesconfiguration1_configuration_cli/achievement-configurations_insert",
                  vec![
                    (Some(r##"application-id"##),
                     None,
                     Some(r##"The application ID from the Google Play developer console."##),
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
                    Some(r##"Returns a list of the achievement configurations in this application."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gamesconfiguration1_configuration_cli/achievement-configurations_list",
                  vec![
                    (Some(r##"application-id"##),
                     None,
                     Some(r##"The application ID from the Google Play developer console."##),
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
                    Some(r##"Update the metadata of the achievement configuration with the given ID. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gamesconfiguration1_configuration_cli/achievement-configurations_patch",
                  vec![
                    (Some(r##"achievement-id"##),
                     None,
                     Some(r##"The ID of the achievement used by this method."##),
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
                    Some(r##"Update the metadata of the achievement configuration with the given ID."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gamesconfiguration1_configuration_cli/achievement-configurations_update",
                  vec![
                    (Some(r##"achievement-id"##),
                     None,
                     Some(r##"The ID of the achievement used by this method."##),
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
        
        ("image-configurations", "methods: 'upload'", vec![
            ("upload",
                    Some(r##"Uploads an image for a resource with the given ID and image type."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gamesconfiguration1_configuration_cli/image-configurations_upload",
                  vec![
                    (Some(r##"resource-id"##),
                     None,
                     Some(r##"The ID of the resource used by this method."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"image-type"##),
                     None,
                     Some(r##"Selects which image in a resource for this method."##),
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
            ]),
        
        ("leaderboard-configurations", "methods: 'delete', 'get', 'insert', 'list', 'patch' and 'update'", vec![
            ("delete",
                    Some(r##"Delete the leaderboard configuration with the given ID."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gamesconfiguration1_configuration_cli/leaderboard-configurations_delete",
                  vec![
                    (Some(r##"leaderboard-id"##),
                     None,
                     Some(r##"The ID of the leaderboard."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",
                    Some(r##"Retrieves the metadata of the leaderboard configuration with the given ID."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gamesconfiguration1_configuration_cli/leaderboard-configurations_get",
                  vec![
                    (Some(r##"leaderboard-id"##),
                     None,
                     Some(r##"The ID of the leaderboard."##),
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
                    Some(r##"Insert a new leaderboard configuration in this application."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gamesconfiguration1_configuration_cli/leaderboard-configurations_insert",
                  vec![
                    (Some(r##"application-id"##),
                     None,
                     Some(r##"The application ID from the Google Play developer console."##),
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
                    Some(r##"Returns a list of the leaderboard configurations in this application."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gamesconfiguration1_configuration_cli/leaderboard-configurations_list",
                  vec![
                    (Some(r##"application-id"##),
                     None,
                     Some(r##"The application ID from the Google Play developer console."##),
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
                    Some(r##"Update the metadata of the leaderboard configuration with the given ID. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gamesconfiguration1_configuration_cli/leaderboard-configurations_patch",
                  vec![
                    (Some(r##"leaderboard-id"##),
                     None,
                     Some(r##"The ID of the leaderboard."##),
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
                    Some(r##"Update the metadata of the leaderboard configuration with the given ID."##),
                    "Details at http://byron.github.io/google-apis-rs/google_gamesconfiguration1_configuration_cli/leaderboard-configurations_update",
                  vec![
                    (Some(r##"leaderboard-id"##),
                     None,
                     Some(r##"The ID of the leaderboard."##),
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
    
    let mut app = App::new("gamesconfiguration1-configuration")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("1.0.7+20181004")
           .about("The Publishing API for Google Play Game Services.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_gamesconfiguration1_configuration_cli")
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