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
extern crate google_dlp2 as api;

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
    hub: api::DLP<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n> Engine<'n> {
    fn _info_types_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.info_types().list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "language-code" => {
                    call = call.language_code(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
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
                                                                           v.extend(["filter", "language-code"].iter().map(|v|*v));
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

    fn _organizations_deidentify_templates_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "deidentify-template.update-time" => Some(("deidentifyTemplate.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.display-name" => Some(("deidentifyTemplate.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.description" => Some(("deidentifyTemplate.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.name" => Some(("deidentifyTemplate.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.create-time" => Some(("deidentifyTemplate.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "template-id" => Some(("templateId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["create-time", "deidentify-template", "description", "display-name", "name", "template-id", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2CreateDeidentifyTemplateRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.organizations().deidentify_templates_create(request, opt.value_of("parent").unwrap_or(""));
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

    fn _organizations_deidentify_templates_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().deidentify_templates_delete(opt.value_of("name").unwrap_or(""));
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

    fn _organizations_deidentify_templates_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().deidentify_templates_get(opt.value_of("name").unwrap_or(""));
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

    fn _organizations_deidentify_templates_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().deidentify_templates_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
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
                                                                           v.extend(["order-by", "page-token", "page-size"].iter().map(|v|*v));
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

    fn _organizations_deidentify_templates_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "deidentify-template.update-time" => Some(("deidentifyTemplate.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.display-name" => Some(("deidentifyTemplate.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.description" => Some(("deidentifyTemplate.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.name" => Some(("deidentifyTemplate.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.create-time" => Some(("deidentifyTemplate.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-mask" => Some(("updateMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["create-time", "deidentify-template", "description", "display-name", "name", "update-mask", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2UpdateDeidentifyTemplateRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.organizations().deidentify_templates_patch(request, opt.value_of("name").unwrap_or(""));
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

    fn _organizations_inspect_templates_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "inspect-template.update-time" => Some(("inspectTemplate.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.display-name" => Some(("inspectTemplate.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.description" => Some(("inspectTemplate.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.exclude-info-types" => Some(("inspectTemplate.inspectConfig.excludeInfoTypes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.content-options" => Some(("inspectTemplate.inspectConfig.contentOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inspect-template.inspect-config.include-quote" => Some(("inspectTemplate.inspectConfig.includeQuote", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.limits.max-findings-per-request" => Some(("inspectTemplate.inspectConfig.limits.maxFindingsPerRequest", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.limits.max-findings-per-item" => Some(("inspectTemplate.inspectConfig.limits.maxFindingsPerItem", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.min-likelihood" => Some(("inspectTemplate.inspectConfig.minLikelihood", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.create-time" => Some(("inspectTemplate.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.name" => Some(("inspectTemplate.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "template-id" => Some(("templateId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["content-options", "create-time", "description", "display-name", "exclude-info-types", "include-quote", "inspect-config", "inspect-template", "limits", "max-findings-per-item", "max-findings-per-request", "min-likelihood", "name", "template-id", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2CreateInspectTemplateRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.organizations().inspect_templates_create(request, opt.value_of("parent").unwrap_or(""));
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

    fn _organizations_inspect_templates_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().inspect_templates_delete(opt.value_of("name").unwrap_or(""));
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

    fn _organizations_inspect_templates_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().inspect_templates_get(opt.value_of("name").unwrap_or(""));
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

    fn _organizations_inspect_templates_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().inspect_templates_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
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
                                                                           v.extend(["order-by", "page-token", "page-size"].iter().map(|v|*v));
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

    fn _organizations_inspect_templates_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "update-mask" => Some(("updateMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.update-time" => Some(("inspectTemplate.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.display-name" => Some(("inspectTemplate.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.description" => Some(("inspectTemplate.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.exclude-info-types" => Some(("inspectTemplate.inspectConfig.excludeInfoTypes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.content-options" => Some(("inspectTemplate.inspectConfig.contentOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inspect-template.inspect-config.include-quote" => Some(("inspectTemplate.inspectConfig.includeQuote", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.limits.max-findings-per-request" => Some(("inspectTemplate.inspectConfig.limits.maxFindingsPerRequest", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.limits.max-findings-per-item" => Some(("inspectTemplate.inspectConfig.limits.maxFindingsPerItem", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.min-likelihood" => Some(("inspectTemplate.inspectConfig.minLikelihood", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.create-time" => Some(("inspectTemplate.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.name" => Some(("inspectTemplate.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["content-options", "create-time", "description", "display-name", "exclude-info-types", "include-quote", "inspect-config", "inspect-template", "limits", "max-findings-per-item", "max-findings-per-request", "min-likelihood", "name", "update-mask", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2UpdateInspectTemplateRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.organizations().inspect_templates_patch(request, opt.value_of("name").unwrap_or(""));
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

    fn _organizations_stored_info_types_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "stored-info-type-id" => Some(("storedInfoTypeId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.display-name" => Some(("config.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.description" => Some(("config.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.output-path.path" => Some(("config.largeCustomDictionary.outputPath.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.field.name" => Some(("config.largeCustomDictionary.bigQueryField.field.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.table.project-id" => Some(("config.largeCustomDictionary.bigQueryField.table.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.table.table-id" => Some(("config.largeCustomDictionary.bigQueryField.table.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.table.dataset-id" => Some(("config.largeCustomDictionary.bigQueryField.table.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.cloud-storage-file-set.url" => Some(("config.largeCustomDictionary.cloudStorageFileSet.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["big-query-field", "cloud-storage-file-set", "config", "dataset-id", "description", "display-name", "field", "large-custom-dictionary", "name", "output-path", "path", "project-id", "stored-info-type-id", "table", "table-id", "url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2CreateStoredInfoTypeRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.organizations().stored_info_types_create(request, opt.value_of("parent").unwrap_or(""));
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

    fn _organizations_stored_info_types_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().stored_info_types_delete(opt.value_of("name").unwrap_or(""));
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

    fn _organizations_stored_info_types_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().stored_info_types_get(opt.value_of("name").unwrap_or(""));
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

    fn _organizations_stored_info_types_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.organizations().stored_info_types_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
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
                                                                           v.extend(["order-by", "page-token", "page-size"].iter().map(|v|*v));
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

    fn _organizations_stored_info_types_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "config.display-name" => Some(("config.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.description" => Some(("config.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.output-path.path" => Some(("config.largeCustomDictionary.outputPath.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.field.name" => Some(("config.largeCustomDictionary.bigQueryField.field.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.table.project-id" => Some(("config.largeCustomDictionary.bigQueryField.table.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.table.table-id" => Some(("config.largeCustomDictionary.bigQueryField.table.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.table.dataset-id" => Some(("config.largeCustomDictionary.bigQueryField.table.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.cloud-storage-file-set.url" => Some(("config.largeCustomDictionary.cloudStorageFileSet.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-mask" => Some(("updateMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["big-query-field", "cloud-storage-file-set", "config", "dataset-id", "description", "display-name", "field", "large-custom-dictionary", "name", "output-path", "path", "project-id", "table", "table-id", "update-mask", "url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2UpdateStoredInfoTypeRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.organizations().stored_info_types_patch(request, opt.value_of("name").unwrap_or(""));
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

    fn _projects_content_deidentify(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "deidentify-template-name" => Some(("deidentifyTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "item.byte-item.type" => Some(("item.byteItem.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "item.byte-item.data" => Some(("item.byteItem.data", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "item.value" => Some(("item.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-config.exclude-info-types" => Some(("inspectConfig.excludeInfoTypes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-config.content-options" => Some(("inspectConfig.contentOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inspect-config.include-quote" => Some(("inspectConfig.includeQuote", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-config.limits.max-findings-per-request" => Some(("inspectConfig.limits.maxFindingsPerRequest", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-config.limits.max-findings-per-item" => Some(("inspectConfig.limits.maxFindingsPerItem", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-config.min-likelihood" => Some(("inspectConfig.minLikelihood", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template-name" => Some(("inspectTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["byte-item", "content-options", "data", "deidentify-template-name", "exclude-info-types", "include-quote", "inspect-config", "inspect-template-name", "item", "limits", "max-findings-per-item", "max-findings-per-request", "min-likelihood", "type", "value"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2DeidentifyContentRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().content_deidentify(request, opt.value_of("parent").unwrap_or(""));
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

    fn _projects_content_inspect(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "item.byte-item.type" => Some(("item.byteItem.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "item.byte-item.data" => Some(("item.byteItem.data", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "item.value" => Some(("item.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-config.exclude-info-types" => Some(("inspectConfig.excludeInfoTypes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-config.content-options" => Some(("inspectConfig.contentOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inspect-config.include-quote" => Some(("inspectConfig.includeQuote", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-config.limits.max-findings-per-request" => Some(("inspectConfig.limits.maxFindingsPerRequest", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-config.limits.max-findings-per-item" => Some(("inspectConfig.limits.maxFindingsPerItem", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-config.min-likelihood" => Some(("inspectConfig.minLikelihood", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template-name" => Some(("inspectTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["byte-item", "content-options", "data", "exclude-info-types", "include-quote", "inspect-config", "inspect-template-name", "item", "limits", "max-findings-per-item", "max-findings-per-request", "min-likelihood", "type", "value"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2InspectContentRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().content_inspect(request, opt.value_of("parent").unwrap_or(""));
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

    fn _projects_content_reidentify(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "item.byte-item.type" => Some(("item.byteItem.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "item.byte-item.data" => Some(("item.byteItem.data", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "item.value" => Some(("item.value", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "reidentify-template-name" => Some(("reidentifyTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-config.exclude-info-types" => Some(("inspectConfig.excludeInfoTypes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-config.content-options" => Some(("inspectConfig.contentOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inspect-config.include-quote" => Some(("inspectConfig.includeQuote", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-config.limits.max-findings-per-request" => Some(("inspectConfig.limits.maxFindingsPerRequest", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-config.limits.max-findings-per-item" => Some(("inspectConfig.limits.maxFindingsPerItem", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-config.min-likelihood" => Some(("inspectConfig.minLikelihood", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template-name" => Some(("inspectTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["byte-item", "content-options", "data", "exclude-info-types", "include-quote", "inspect-config", "inspect-template-name", "item", "limits", "max-findings-per-item", "max-findings-per-request", "min-likelihood", "reidentify-template-name", "type", "value"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2ReidentifyContentRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().content_reidentify(request, opt.value_of("parent").unwrap_or(""));
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

    fn _projects_deidentify_templates_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "deidentify-template.update-time" => Some(("deidentifyTemplate.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.display-name" => Some(("deidentifyTemplate.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.description" => Some(("deidentifyTemplate.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.name" => Some(("deidentifyTemplate.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.create-time" => Some(("deidentifyTemplate.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "template-id" => Some(("templateId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["create-time", "deidentify-template", "description", "display-name", "name", "template-id", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2CreateDeidentifyTemplateRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().deidentify_templates_create(request, opt.value_of("parent").unwrap_or(""));
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

    fn _projects_deidentify_templates_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().deidentify_templates_delete(opt.value_of("name").unwrap_or(""));
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

    fn _projects_deidentify_templates_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().deidentify_templates_get(opt.value_of("name").unwrap_or(""));
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

    fn _projects_deidentify_templates_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().deidentify_templates_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
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
                                                                           v.extend(["order-by", "page-token", "page-size"].iter().map(|v|*v));
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

    fn _projects_deidentify_templates_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "deidentify-template.update-time" => Some(("deidentifyTemplate.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.display-name" => Some(("deidentifyTemplate.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.description" => Some(("deidentifyTemplate.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.name" => Some(("deidentifyTemplate.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deidentify-template.create-time" => Some(("deidentifyTemplate.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-mask" => Some(("updateMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["create-time", "deidentify-template", "description", "display-name", "name", "update-mask", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2UpdateDeidentifyTemplateRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().deidentify_templates_patch(request, opt.value_of("name").unwrap_or(""));
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

    fn _projects_dlp_jobs_cancel(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::GooglePrivacyDlpV2CancelDlpJobRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().dlp_jobs_cancel(request, opt.value_of("name").unwrap_or(""));
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

    fn _projects_dlp_jobs_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "risk-job.privacy-metric.numerical-stats-config.field.name" => Some(("riskJob.privacyMetric.numericalStatsConfig.field.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "risk-job.privacy-metric.k-map-estimation-config.region-code" => Some(("riskJob.privacyMetric.kMapEstimationConfig.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "risk-job.privacy-metric.l-diversity-config.sensitive-attribute.name" => Some(("riskJob.privacyMetric.lDiversityConfig.sensitiveAttribute.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "risk-job.privacy-metric.delta-presence-estimation-config.region-code" => Some(("riskJob.privacyMetric.deltaPresenceEstimationConfig.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "risk-job.privacy-metric.categorical-stats-config.field.name" => Some(("riskJob.privacyMetric.categoricalStatsConfig.field.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "risk-job.privacy-metric.k-anonymity-config.entity-id.field.name" => Some(("riskJob.privacyMetric.kAnonymityConfig.entityId.field.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "risk-job.source-table.project-id" => Some(("riskJob.sourceTable.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "risk-job.source-table.table-id" => Some(("riskJob.sourceTable.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "risk-job.source-table.dataset-id" => Some(("riskJob.sourceTable.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.timespan-config.timestamp-field.name" => Some(("inspectJob.storageConfig.timespanConfig.timestampField.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.timespan-config.end-time" => Some(("inspectJob.storageConfig.timespanConfig.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.timespan-config.start-time" => Some(("inspectJob.storageConfig.timespanConfig.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.timespan-config.enable-auto-population-of-timespan-config" => Some(("inspectJob.storageConfig.timespanConfig.enableAutoPopulationOfTimespanConfig", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.big-query-options.rows-limit" => Some(("inspectJob.storageConfig.bigQueryOptions.rowsLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.big-query-options.sample-method" => Some(("inspectJob.storageConfig.bigQueryOptions.sampleMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.big-query-options.table-reference.project-id" => Some(("inspectJob.storageConfig.bigQueryOptions.tableReference.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.big-query-options.table-reference.table-id" => Some(("inspectJob.storageConfig.bigQueryOptions.tableReference.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.big-query-options.table-reference.dataset-id" => Some(("inspectJob.storageConfig.bigQueryOptions.tableReference.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.big-query-options.rows-limit-percent" => Some(("inspectJob.storageConfig.bigQueryOptions.rowsLimitPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.datastore-options.partition-id.project-id" => Some(("inspectJob.storageConfig.datastoreOptions.partitionId.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.datastore-options.partition-id.namespace-id" => Some(("inspectJob.storageConfig.datastoreOptions.partitionId.namespaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.datastore-options.kind.name" => Some(("inspectJob.storageConfig.datastoreOptions.kind.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.cloud-storage-options.bytes-limit-per-file" => Some(("inspectJob.storageConfig.cloudStorageOptions.bytesLimitPerFile", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.cloud-storage-options.sample-method" => Some(("inspectJob.storageConfig.cloudStorageOptions.sampleMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.cloud-storage-options.file-set.url" => Some(("inspectJob.storageConfig.cloudStorageOptions.fileSet.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.cloud-storage-options.file-set.regex-file-set.exclude-regex" => Some(("inspectJob.storageConfig.cloudStorageOptions.fileSet.regexFileSet.excludeRegex", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inspect-job.storage-config.cloud-storage-options.file-set.regex-file-set.bucket-name" => Some(("inspectJob.storageConfig.cloudStorageOptions.fileSet.regexFileSet.bucketName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.cloud-storage-options.file-set.regex-file-set.include-regex" => Some(("inspectJob.storageConfig.cloudStorageOptions.fileSet.regexFileSet.includeRegex", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inspect-job.storage-config.cloud-storage-options.bytes-limit-per-file-percent" => Some(("inspectJob.storageConfig.cloudStorageOptions.bytesLimitPerFilePercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.cloud-storage-options.files-limit-percent" => Some(("inspectJob.storageConfig.cloudStorageOptions.filesLimitPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-job.storage-config.cloud-storage-options.file-types" => Some(("inspectJob.storageConfig.cloudStorageOptions.fileTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inspect-job.inspect-config.exclude-info-types" => Some(("inspectJob.inspectConfig.excludeInfoTypes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-job.inspect-config.content-options" => Some(("inspectJob.inspectConfig.contentOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inspect-job.inspect-config.include-quote" => Some(("inspectJob.inspectConfig.includeQuote", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-job.inspect-config.limits.max-findings-per-request" => Some(("inspectJob.inspectConfig.limits.maxFindingsPerRequest", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-job.inspect-config.limits.max-findings-per-item" => Some(("inspectJob.inspectConfig.limits.maxFindingsPerItem", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-job.inspect-config.min-likelihood" => Some(("inspectJob.inspectConfig.minLikelihood", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-job.inspect-template-name" => Some(("inspectJob.inspectTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-id" => Some(("jobId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["big-query-options", "bucket-name", "bytes-limit-per-file", "bytes-limit-per-file-percent", "categorical-stats-config", "cloud-storage-options", "content-options", "dataset-id", "datastore-options", "delta-presence-estimation-config", "enable-auto-population-of-timespan-config", "end-time", "entity-id", "exclude-info-types", "exclude-regex", "field", "file-set", "file-types", "files-limit-percent", "include-quote", "include-regex", "inspect-config", "inspect-job", "inspect-template-name", "job-id", "k-anonymity-config", "k-map-estimation-config", "kind", "l-diversity-config", "limits", "max-findings-per-item", "max-findings-per-request", "min-likelihood", "name", "namespace-id", "numerical-stats-config", "partition-id", "privacy-metric", "project-id", "regex-file-set", "region-code", "risk-job", "rows-limit", "rows-limit-percent", "sample-method", "sensitive-attribute", "source-table", "start-time", "storage-config", "table-id", "table-reference", "timespan-config", "timestamp-field", "url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2CreateDlpJobRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().dlp_jobs_create(request, opt.value_of("parent").unwrap_or(""));
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

    fn _projects_dlp_jobs_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().dlp_jobs_delete(opt.value_of("name").unwrap_or(""));
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

    fn _projects_dlp_jobs_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().dlp_jobs_get(opt.value_of("name").unwrap_or(""));
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

    fn _projects_dlp_jobs_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().dlp_jobs_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "type" => {
                    call = call.type_(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
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
                                                                           v.extend(["order-by", "page-token", "type", "filter", "page-size"].iter().map(|v|*v));
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

    fn _projects_image_redact(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "byte-item.type" => Some(("byteItem.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "byte-item.data" => Some(("byteItem.data", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-config.exclude-info-types" => Some(("inspectConfig.excludeInfoTypes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-config.content-options" => Some(("inspectConfig.contentOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inspect-config.include-quote" => Some(("inspectConfig.includeQuote", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-config.limits.max-findings-per-request" => Some(("inspectConfig.limits.maxFindingsPerRequest", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-config.limits.max-findings-per-item" => Some(("inspectConfig.limits.maxFindingsPerItem", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-config.min-likelihood" => Some(("inspectConfig.minLikelihood", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "include-findings" => Some(("includeFindings", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["byte-item", "content-options", "data", "exclude-info-types", "include-findings", "include-quote", "inspect-config", "limits", "max-findings-per-item", "max-findings-per-request", "min-likelihood", "type"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2RedactImageRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().image_redact(request, opt.value_of("parent").unwrap_or(""));
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

    fn _projects_inspect_templates_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "inspect-template.update-time" => Some(("inspectTemplate.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.display-name" => Some(("inspectTemplate.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.description" => Some(("inspectTemplate.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.exclude-info-types" => Some(("inspectTemplate.inspectConfig.excludeInfoTypes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.content-options" => Some(("inspectTemplate.inspectConfig.contentOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inspect-template.inspect-config.include-quote" => Some(("inspectTemplate.inspectConfig.includeQuote", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.limits.max-findings-per-request" => Some(("inspectTemplate.inspectConfig.limits.maxFindingsPerRequest", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.limits.max-findings-per-item" => Some(("inspectTemplate.inspectConfig.limits.maxFindingsPerItem", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.min-likelihood" => Some(("inspectTemplate.inspectConfig.minLikelihood", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.create-time" => Some(("inspectTemplate.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.name" => Some(("inspectTemplate.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "template-id" => Some(("templateId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["content-options", "create-time", "description", "display-name", "exclude-info-types", "include-quote", "inspect-config", "inspect-template", "limits", "max-findings-per-item", "max-findings-per-request", "min-likelihood", "name", "template-id", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2CreateInspectTemplateRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().inspect_templates_create(request, opt.value_of("parent").unwrap_or(""));
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

    fn _projects_inspect_templates_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().inspect_templates_delete(opt.value_of("name").unwrap_or(""));
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

    fn _projects_inspect_templates_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().inspect_templates_get(opt.value_of("name").unwrap_or(""));
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

    fn _projects_inspect_templates_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().inspect_templates_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
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
                                                                           v.extend(["order-by", "page-token", "page-size"].iter().map(|v|*v));
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

    fn _projects_inspect_templates_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "update-mask" => Some(("updateMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.update-time" => Some(("inspectTemplate.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.display-name" => Some(("inspectTemplate.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.description" => Some(("inspectTemplate.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.exclude-info-types" => Some(("inspectTemplate.inspectConfig.excludeInfoTypes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.content-options" => Some(("inspectTemplate.inspectConfig.contentOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "inspect-template.inspect-config.include-quote" => Some(("inspectTemplate.inspectConfig.includeQuote", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.limits.max-findings-per-request" => Some(("inspectTemplate.inspectConfig.limits.maxFindingsPerRequest", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.limits.max-findings-per-item" => Some(("inspectTemplate.inspectConfig.limits.maxFindingsPerItem", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inspect-template.inspect-config.min-likelihood" => Some(("inspectTemplate.inspectConfig.minLikelihood", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.create-time" => Some(("inspectTemplate.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inspect-template.name" => Some(("inspectTemplate.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["content-options", "create-time", "description", "display-name", "exclude-info-types", "include-quote", "inspect-config", "inspect-template", "limits", "max-findings-per-item", "max-findings-per-request", "min-likelihood", "name", "update-mask", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2UpdateInspectTemplateRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().inspect_templates_patch(request, opt.value_of("name").unwrap_or(""));
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

    fn _projects_job_triggers_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "trigger-id" => Some(("triggerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.status" => Some(("jobTrigger.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.update-time" => Some(("jobTrigger.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.display-name" => Some(("jobTrigger.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.description" => Some(("jobTrigger.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.timespan-config.timestamp-field.name" => Some(("jobTrigger.inspectJob.storageConfig.timespanConfig.timestampField.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.timespan-config.end-time" => Some(("jobTrigger.inspectJob.storageConfig.timespanConfig.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.timespan-config.start-time" => Some(("jobTrigger.inspectJob.storageConfig.timespanConfig.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.timespan-config.enable-auto-population-of-timespan-config" => Some(("jobTrigger.inspectJob.storageConfig.timespanConfig.enableAutoPopulationOfTimespanConfig", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.rows-limit" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.rowsLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.sample-method" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.sampleMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.table-reference.project-id" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.tableReference.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.table-reference.table-id" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.tableReference.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.table-reference.dataset-id" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.tableReference.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.rows-limit-percent" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.rowsLimitPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.datastore-options.partition-id.project-id" => Some(("jobTrigger.inspectJob.storageConfig.datastoreOptions.partitionId.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.datastore-options.partition-id.namespace-id" => Some(("jobTrigger.inspectJob.storageConfig.datastoreOptions.partitionId.namespaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.datastore-options.kind.name" => Some(("jobTrigger.inspectJob.storageConfig.datastoreOptions.kind.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.bytes-limit-per-file" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.bytesLimitPerFile", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.sample-method" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.sampleMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.file-set.url" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.fileSet.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.file-set.regex-file-set.exclude-regex" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.fileSet.regexFileSet.excludeRegex", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.file-set.regex-file-set.bucket-name" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.fileSet.regexFileSet.bucketName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.file-set.regex-file-set.include-regex" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.fileSet.regexFileSet.includeRegex", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.bytes-limit-per-file-percent" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.bytesLimitPerFilePercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.files-limit-percent" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.filesLimitPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.file-types" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.fileTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-trigger.inspect-job.inspect-config.exclude-info-types" => Some(("jobTrigger.inspectJob.inspectConfig.excludeInfoTypes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-config.content-options" => Some(("jobTrigger.inspectJob.inspectConfig.contentOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-trigger.inspect-job.inspect-config.include-quote" => Some(("jobTrigger.inspectJob.inspectConfig.includeQuote", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-config.limits.max-findings-per-request" => Some(("jobTrigger.inspectJob.inspectConfig.limits.maxFindingsPerRequest", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-config.limits.max-findings-per-item" => Some(("jobTrigger.inspectJob.inspectConfig.limits.maxFindingsPerItem", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-config.min-likelihood" => Some(("jobTrigger.inspectJob.inspectConfig.minLikelihood", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-template-name" => Some(("jobTrigger.inspectJob.inspectTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.last-run-time" => Some(("jobTrigger.lastRunTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.create-time" => Some(("jobTrigger.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.name" => Some(("jobTrigger.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["big-query-options", "bucket-name", "bytes-limit-per-file", "bytes-limit-per-file-percent", "cloud-storage-options", "content-options", "create-time", "dataset-id", "datastore-options", "description", "display-name", "enable-auto-population-of-timespan-config", "end-time", "exclude-info-types", "exclude-regex", "file-set", "file-types", "files-limit-percent", "include-quote", "include-regex", "inspect-config", "inspect-job", "inspect-template-name", "job-trigger", "kind", "last-run-time", "limits", "max-findings-per-item", "max-findings-per-request", "min-likelihood", "name", "namespace-id", "partition-id", "project-id", "regex-file-set", "rows-limit", "rows-limit-percent", "sample-method", "start-time", "status", "storage-config", "table-id", "table-reference", "timespan-config", "timestamp-field", "trigger-id", "update-time", "url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2CreateJobTriggerRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().job_triggers_create(request, opt.value_of("parent").unwrap_or(""));
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

    fn _projects_job_triggers_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().job_triggers_delete(opt.value_of("name").unwrap_or(""));
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

    fn _projects_job_triggers_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().job_triggers_get(opt.value_of("name").unwrap_or(""));
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

    fn _projects_job_triggers_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().job_triggers_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
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
                                                                           v.extend(["order-by", "page-token", "page-size"].iter().map(|v|*v));
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

    fn _projects_job_triggers_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "job-trigger.status" => Some(("jobTrigger.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.update-time" => Some(("jobTrigger.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.display-name" => Some(("jobTrigger.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.description" => Some(("jobTrigger.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.timespan-config.timestamp-field.name" => Some(("jobTrigger.inspectJob.storageConfig.timespanConfig.timestampField.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.timespan-config.end-time" => Some(("jobTrigger.inspectJob.storageConfig.timespanConfig.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.timespan-config.start-time" => Some(("jobTrigger.inspectJob.storageConfig.timespanConfig.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.timespan-config.enable-auto-population-of-timespan-config" => Some(("jobTrigger.inspectJob.storageConfig.timespanConfig.enableAutoPopulationOfTimespanConfig", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.rows-limit" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.rowsLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.sample-method" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.sampleMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.table-reference.project-id" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.tableReference.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.table-reference.table-id" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.tableReference.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.table-reference.dataset-id" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.tableReference.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.big-query-options.rows-limit-percent" => Some(("jobTrigger.inspectJob.storageConfig.bigQueryOptions.rowsLimitPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.datastore-options.partition-id.project-id" => Some(("jobTrigger.inspectJob.storageConfig.datastoreOptions.partitionId.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.datastore-options.partition-id.namespace-id" => Some(("jobTrigger.inspectJob.storageConfig.datastoreOptions.partitionId.namespaceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.datastore-options.kind.name" => Some(("jobTrigger.inspectJob.storageConfig.datastoreOptions.kind.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.bytes-limit-per-file" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.bytesLimitPerFile", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.sample-method" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.sampleMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.file-set.url" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.fileSet.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.file-set.regex-file-set.exclude-regex" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.fileSet.regexFileSet.excludeRegex", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.file-set.regex-file-set.bucket-name" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.fileSet.regexFileSet.bucketName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.file-set.regex-file-set.include-regex" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.fileSet.regexFileSet.includeRegex", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.bytes-limit-per-file-percent" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.bytesLimitPerFilePercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.files-limit-percent" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.filesLimitPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.storage-config.cloud-storage-options.file-types" => Some(("jobTrigger.inspectJob.storageConfig.cloudStorageOptions.fileTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-trigger.inspect-job.inspect-config.exclude-info-types" => Some(("jobTrigger.inspectJob.inspectConfig.excludeInfoTypes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-config.content-options" => Some(("jobTrigger.inspectJob.inspectConfig.contentOptions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "job-trigger.inspect-job.inspect-config.include-quote" => Some(("jobTrigger.inspectJob.inspectConfig.includeQuote", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-config.limits.max-findings-per-request" => Some(("jobTrigger.inspectJob.inspectConfig.limits.maxFindingsPerRequest", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-config.limits.max-findings-per-item" => Some(("jobTrigger.inspectJob.inspectConfig.limits.maxFindingsPerItem", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-config.min-likelihood" => Some(("jobTrigger.inspectJob.inspectConfig.minLikelihood", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.inspect-job.inspect-template-name" => Some(("jobTrigger.inspectJob.inspectTemplateName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.last-run-time" => Some(("jobTrigger.lastRunTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.create-time" => Some(("jobTrigger.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "job-trigger.name" => Some(("jobTrigger.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-mask" => Some(("updateMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["big-query-options", "bucket-name", "bytes-limit-per-file", "bytes-limit-per-file-percent", "cloud-storage-options", "content-options", "create-time", "dataset-id", "datastore-options", "description", "display-name", "enable-auto-population-of-timespan-config", "end-time", "exclude-info-types", "exclude-regex", "file-set", "file-types", "files-limit-percent", "include-quote", "include-regex", "inspect-config", "inspect-job", "inspect-template-name", "job-trigger", "kind", "last-run-time", "limits", "max-findings-per-item", "max-findings-per-request", "min-likelihood", "name", "namespace-id", "partition-id", "project-id", "regex-file-set", "rows-limit", "rows-limit-percent", "sample-method", "start-time", "status", "storage-config", "table-id", "table-reference", "timespan-config", "timestamp-field", "update-mask", "update-time", "url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2UpdateJobTriggerRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().job_triggers_patch(request, opt.value_of("name").unwrap_or(""));
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

    fn _projects_stored_info_types_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "stored-info-type-id" => Some(("storedInfoTypeId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.display-name" => Some(("config.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.description" => Some(("config.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.output-path.path" => Some(("config.largeCustomDictionary.outputPath.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.field.name" => Some(("config.largeCustomDictionary.bigQueryField.field.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.table.project-id" => Some(("config.largeCustomDictionary.bigQueryField.table.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.table.table-id" => Some(("config.largeCustomDictionary.bigQueryField.table.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.table.dataset-id" => Some(("config.largeCustomDictionary.bigQueryField.table.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.cloud-storage-file-set.url" => Some(("config.largeCustomDictionary.cloudStorageFileSet.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["big-query-field", "cloud-storage-file-set", "config", "dataset-id", "description", "display-name", "field", "large-custom-dictionary", "name", "output-path", "path", "project-id", "stored-info-type-id", "table", "table-id", "url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2CreateStoredInfoTypeRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().stored_info_types_create(request, opt.value_of("parent").unwrap_or(""));
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

    fn _projects_stored_info_types_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().stored_info_types_delete(opt.value_of("name").unwrap_or(""));
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

    fn _projects_stored_info_types_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().stored_info_types_get(opt.value_of("name").unwrap_or(""));
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

    fn _projects_stored_info_types_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().stored_info_types_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
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
                                                                           v.extend(["order-by", "page-token", "page-size"].iter().map(|v|*v));
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

    fn _projects_stored_info_types_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "config.display-name" => Some(("config.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.description" => Some(("config.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.output-path.path" => Some(("config.largeCustomDictionary.outputPath.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.field.name" => Some(("config.largeCustomDictionary.bigQueryField.field.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.table.project-id" => Some(("config.largeCustomDictionary.bigQueryField.table.projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.table.table-id" => Some(("config.largeCustomDictionary.bigQueryField.table.tableId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.big-query-field.table.dataset-id" => Some(("config.largeCustomDictionary.bigQueryField.table.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.large-custom-dictionary.cloud-storage-file-set.url" => Some(("config.largeCustomDictionary.cloudStorageFileSet.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-mask" => Some(("updateMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["big-query-field", "cloud-storage-file-set", "config", "dataset-id", "description", "display-name", "field", "large-custom-dictionary", "name", "output-path", "path", "project-id", "table", "table-id", "update-mask", "url"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GooglePrivacyDlpV2UpdateStoredInfoTypeRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().stored_info_types_patch(request, opt.value_of("name").unwrap_or(""));
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
            ("info-types", Some(opt)) => {
                match opt.subcommand() {
                    ("list", Some(opt)) => {
                        call_result = self._info_types_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("info-types".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("organizations", Some(opt)) => {
                match opt.subcommand() {
                    ("deidentify-templates-create", Some(opt)) => {
                        call_result = self._organizations_deidentify_templates_create(opt, dry_run, &mut err);
                    },
                    ("deidentify-templates-delete", Some(opt)) => {
                        call_result = self._organizations_deidentify_templates_delete(opt, dry_run, &mut err);
                    },
                    ("deidentify-templates-get", Some(opt)) => {
                        call_result = self._organizations_deidentify_templates_get(opt, dry_run, &mut err);
                    },
                    ("deidentify-templates-list", Some(opt)) => {
                        call_result = self._organizations_deidentify_templates_list(opt, dry_run, &mut err);
                    },
                    ("deidentify-templates-patch", Some(opt)) => {
                        call_result = self._organizations_deidentify_templates_patch(opt, dry_run, &mut err);
                    },
                    ("inspect-templates-create", Some(opt)) => {
                        call_result = self._organizations_inspect_templates_create(opt, dry_run, &mut err);
                    },
                    ("inspect-templates-delete", Some(opt)) => {
                        call_result = self._organizations_inspect_templates_delete(opt, dry_run, &mut err);
                    },
                    ("inspect-templates-get", Some(opt)) => {
                        call_result = self._organizations_inspect_templates_get(opt, dry_run, &mut err);
                    },
                    ("inspect-templates-list", Some(opt)) => {
                        call_result = self._organizations_inspect_templates_list(opt, dry_run, &mut err);
                    },
                    ("inspect-templates-patch", Some(opt)) => {
                        call_result = self._organizations_inspect_templates_patch(opt, dry_run, &mut err);
                    },
                    ("stored-info-types-create", Some(opt)) => {
                        call_result = self._organizations_stored_info_types_create(opt, dry_run, &mut err);
                    },
                    ("stored-info-types-delete", Some(opt)) => {
                        call_result = self._organizations_stored_info_types_delete(opt, dry_run, &mut err);
                    },
                    ("stored-info-types-get", Some(opt)) => {
                        call_result = self._organizations_stored_info_types_get(opt, dry_run, &mut err);
                    },
                    ("stored-info-types-list", Some(opt)) => {
                        call_result = self._organizations_stored_info_types_list(opt, dry_run, &mut err);
                    },
                    ("stored-info-types-patch", Some(opt)) => {
                        call_result = self._organizations_stored_info_types_patch(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("organizations".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("projects", Some(opt)) => {
                match opt.subcommand() {
                    ("content-deidentify", Some(opt)) => {
                        call_result = self._projects_content_deidentify(opt, dry_run, &mut err);
                    },
                    ("content-inspect", Some(opt)) => {
                        call_result = self._projects_content_inspect(opt, dry_run, &mut err);
                    },
                    ("content-reidentify", Some(opt)) => {
                        call_result = self._projects_content_reidentify(opt, dry_run, &mut err);
                    },
                    ("deidentify-templates-create", Some(opt)) => {
                        call_result = self._projects_deidentify_templates_create(opt, dry_run, &mut err);
                    },
                    ("deidentify-templates-delete", Some(opt)) => {
                        call_result = self._projects_deidentify_templates_delete(opt, dry_run, &mut err);
                    },
                    ("deidentify-templates-get", Some(opt)) => {
                        call_result = self._projects_deidentify_templates_get(opt, dry_run, &mut err);
                    },
                    ("deidentify-templates-list", Some(opt)) => {
                        call_result = self._projects_deidentify_templates_list(opt, dry_run, &mut err);
                    },
                    ("deidentify-templates-patch", Some(opt)) => {
                        call_result = self._projects_deidentify_templates_patch(opt, dry_run, &mut err);
                    },
                    ("dlp-jobs-cancel", Some(opt)) => {
                        call_result = self._projects_dlp_jobs_cancel(opt, dry_run, &mut err);
                    },
                    ("dlp-jobs-create", Some(opt)) => {
                        call_result = self._projects_dlp_jobs_create(opt, dry_run, &mut err);
                    },
                    ("dlp-jobs-delete", Some(opt)) => {
                        call_result = self._projects_dlp_jobs_delete(opt, dry_run, &mut err);
                    },
                    ("dlp-jobs-get", Some(opt)) => {
                        call_result = self._projects_dlp_jobs_get(opt, dry_run, &mut err);
                    },
                    ("dlp-jobs-list", Some(opt)) => {
                        call_result = self._projects_dlp_jobs_list(opt, dry_run, &mut err);
                    },
                    ("image-redact", Some(opt)) => {
                        call_result = self._projects_image_redact(opt, dry_run, &mut err);
                    },
                    ("inspect-templates-create", Some(opt)) => {
                        call_result = self._projects_inspect_templates_create(opt, dry_run, &mut err);
                    },
                    ("inspect-templates-delete", Some(opt)) => {
                        call_result = self._projects_inspect_templates_delete(opt, dry_run, &mut err);
                    },
                    ("inspect-templates-get", Some(opt)) => {
                        call_result = self._projects_inspect_templates_get(opt, dry_run, &mut err);
                    },
                    ("inspect-templates-list", Some(opt)) => {
                        call_result = self._projects_inspect_templates_list(opt, dry_run, &mut err);
                    },
                    ("inspect-templates-patch", Some(opt)) => {
                        call_result = self._projects_inspect_templates_patch(opt, dry_run, &mut err);
                    },
                    ("job-triggers-create", Some(opt)) => {
                        call_result = self._projects_job_triggers_create(opt, dry_run, &mut err);
                    },
                    ("job-triggers-delete", Some(opt)) => {
                        call_result = self._projects_job_triggers_delete(opt, dry_run, &mut err);
                    },
                    ("job-triggers-get", Some(opt)) => {
                        call_result = self._projects_job_triggers_get(opt, dry_run, &mut err);
                    },
                    ("job-triggers-list", Some(opt)) => {
                        call_result = self._projects_job_triggers_list(opt, dry_run, &mut err);
                    },
                    ("job-triggers-patch", Some(opt)) => {
                        call_result = self._projects_job_triggers_patch(opt, dry_run, &mut err);
                    },
                    ("stored-info-types-create", Some(opt)) => {
                        call_result = self._projects_stored_info_types_create(opt, dry_run, &mut err);
                    },
                    ("stored-info-types-delete", Some(opt)) => {
                        call_result = self._projects_stored_info_types_delete(opt, dry_run, &mut err);
                    },
                    ("stored-info-types-get", Some(opt)) => {
                        call_result = self._projects_stored_info_types_get(opt, dry_run, &mut err);
                    },
                    ("stored-info-types-list", Some(opt)) => {
                        call_result = self._projects_stored_info_types_list(opt, dry_run, &mut err);
                    },
                    ("stored-info-types-patch", Some(opt)) => {
                        call_result = self._projects_stored_info_types_patch(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("projects".to_string()));
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

            match cmn::application_secret_from_directory(&config_dir, "dlp2-secret.json",
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
                                          program_name: "dlp2",
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
            hub: api::DLP::new(client, auth),
            gp: vec!["$-xgafv", "access-token", "alt", "callback", "fields", "key", "oauth-token", "pretty-print", "quota-user", "upload-type", "upload-protocol"],
            gpm: vec![
                    ("$-xgafv", "$.xgafv"),
                    ("access-token", "access_token"),
                    ("oauth-token", "oauth_token"),
                    ("pretty-print", "prettyPrint"),
                    ("quota-user", "quotaUser"),
                    ("upload-type", "uploadType"),
                    ("upload-protocol", "upload_protocol"),
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
        ("info-types", "methods: 'list'", vec![
            ("list",
                    Some(r##"Returns a list of the sensitive information types that the DLP API
        supports. See https://cloud.google.com/dlp/docs/infotypes-reference to
        learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/info-types_list",
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
        
        ("organizations", "methods: 'deidentify-templates-create', 'deidentify-templates-delete', 'deidentify-templates-get', 'deidentify-templates-list', 'deidentify-templates-patch', 'inspect-templates-create', 'inspect-templates-delete', 'inspect-templates-get', 'inspect-templates-list', 'inspect-templates-patch', 'stored-info-types-create', 'stored-info-types-delete', 'stored-info-types-get', 'stored-info-types-list' and 'stored-info-types-patch'", vec![
            ("deidentify-templates-create",
                    Some(r##"Creates a DeidentifyTemplate for re-using frequently used configuration
        for de-identifying content, images, and storage.
        See https://cloud.google.com/dlp/docs/creating-templates-deid to learn
        more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_deidentify-templates-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The parent resource name, for example projects/my-project-id or
        organizations/my-org-id."##),
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
            ("deidentify-templates-delete",
                    Some(r##"Deletes a DeidentifyTemplate.
        See https://cloud.google.com/dlp/docs/creating-templates-deid to learn
        more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_deidentify-templates-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Resource name of the organization and deidentify template to be deleted,
        for example `organizations/433245324/deidentifyTemplates/432452342` or
        projects/project-id/deidentifyTemplates/432452342."##),
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
            ("deidentify-templates-get",
                    Some(r##"Gets a DeidentifyTemplate.
        See https://cloud.google.com/dlp/docs/creating-templates-deid to learn
        more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_deidentify-templates-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Resource name of the organization and deidentify template to be read, for
        example `organizations/433245324/deidentifyTemplates/432452342` or
        projects/project-id/deidentifyTemplates/432452342."##),
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
            ("deidentify-templates-list",
                    Some(r##"Lists DeidentifyTemplates.
        See https://cloud.google.com/dlp/docs/creating-templates-deid to learn
        more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_deidentify-templates-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The parent resource name, for example projects/my-project-id or
        organizations/my-org-id."##),
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
            ("deidentify-templates-patch",
                    Some(r##"Updates the DeidentifyTemplate.
        See https://cloud.google.com/dlp/docs/creating-templates-deid to learn
        more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_deidentify-templates-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Resource name of organization and deidentify template to be updated, for
        example `organizations/433245324/deidentifyTemplates/432452342` or
        projects/project-id/deidentifyTemplates/432452342."##),
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
            ("inspect-templates-create",
                    Some(r##"Creates an InspectTemplate for re-using frequently used configuration
        for inspecting content, images, and storage.
        See https://cloud.google.com/dlp/docs/creating-templates to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_inspect-templates-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The parent resource name, for example projects/my-project-id or
        organizations/my-org-id."##),
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
            ("inspect-templates-delete",
                    Some(r##"Deletes an InspectTemplate.
        See https://cloud.google.com/dlp/docs/creating-templates to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_inspect-templates-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Resource name of the organization and inspectTemplate to be deleted, for
        example `organizations/433245324/inspectTemplates/432452342` or
        projects/project-id/inspectTemplates/432452342."##),
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
            ("inspect-templates-get",
                    Some(r##"Gets an InspectTemplate.
        See https://cloud.google.com/dlp/docs/creating-templates to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_inspect-templates-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Resource name of the organization and inspectTemplate to be read, for
        example `organizations/433245324/inspectTemplates/432452342` or
        projects/project-id/inspectTemplates/432452342."##),
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
            ("inspect-templates-list",
                    Some(r##"Lists InspectTemplates.
        See https://cloud.google.com/dlp/docs/creating-templates to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_inspect-templates-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The parent resource name, for example projects/my-project-id or
        organizations/my-org-id."##),
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
            ("inspect-templates-patch",
                    Some(r##"Updates the InspectTemplate.
        See https://cloud.google.com/dlp/docs/creating-templates to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_inspect-templates-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Resource name of organization and inspectTemplate to be updated, for
        example `organizations/433245324/inspectTemplates/432452342` or
        projects/project-id/inspectTemplates/432452342."##),
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
            ("stored-info-types-create",
                    Some(r##"Creates a pre-built stored infoType to be used for inspection.
        See https://cloud.google.com/dlp/docs/creating-stored-infotypes to
        learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_stored-info-types-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The parent resource name, for example projects/my-project-id or
        organizations/my-org-id."##),
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
            ("stored-info-types-delete",
                    Some(r##"Deletes a stored infoType.
        See https://cloud.google.com/dlp/docs/creating-stored-infotypes to
        learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_stored-info-types-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Resource name of the organization and storedInfoType to be deleted, for
        example `organizations/433245324/storedInfoTypes/432452342` or
        projects/project-id/storedInfoTypes/432452342."##),
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
            ("stored-info-types-get",
                    Some(r##"Gets a stored infoType.
        See https://cloud.google.com/dlp/docs/creating-stored-infotypes to
        learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_stored-info-types-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Resource name of the organization and storedInfoType to be read, for
        example `organizations/433245324/storedInfoTypes/432452342` or
        projects/project-id/storedInfoTypes/432452342."##),
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
            ("stored-info-types-list",
                    Some(r##"Lists stored infoTypes.
        See https://cloud.google.com/dlp/docs/creating-stored-infotypes to
        learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_stored-info-types-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The parent resource name, for example projects/my-project-id or
        organizations/my-org-id."##),
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
            ("stored-info-types-patch",
                    Some(r##"Updates the stored infoType by creating a new version. The existing version
        will continue to be used until the new version is ready.
        See https://cloud.google.com/dlp/docs/creating-stored-infotypes to
        learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/organizations_stored-info-types-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Resource name of organization and storedInfoType to be updated, for
        example `organizations/433245324/storedInfoTypes/432452342` or
        projects/project-id/storedInfoTypes/432452342."##),
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
        
        ("projects", "methods: 'content-deidentify', 'content-inspect', 'content-reidentify', 'deidentify-templates-create', 'deidentify-templates-delete', 'deidentify-templates-get', 'deidentify-templates-list', 'deidentify-templates-patch', 'dlp-jobs-cancel', 'dlp-jobs-create', 'dlp-jobs-delete', 'dlp-jobs-get', 'dlp-jobs-list', 'image-redact', 'inspect-templates-create', 'inspect-templates-delete', 'inspect-templates-get', 'inspect-templates-list', 'inspect-templates-patch', 'job-triggers-create', 'job-triggers-delete', 'job-triggers-get', 'job-triggers-list', 'job-triggers-patch', 'stored-info-types-create', 'stored-info-types-delete', 'stored-info-types-get', 'stored-info-types-list' and 'stored-info-types-patch'", vec![
            ("content-deidentify",
                    Some(r##"De-identifies potentially sensitive info from a ContentItem.
        This method has limits on input size and output size.
        See https://cloud.google.com/dlp/docs/deidentify-sensitive-data to
        learn more.
        
        When no InfoTypes or CustomInfoTypes are specified in this request, the
        system will automatically choose what detectors to run. By default this may
        be all types, but may change over time as detectors are updated."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_content-deidentify",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The parent resource name, for example projects/my-project-id."##),
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
            ("content-inspect",
                    Some(r##"Finds potentially sensitive info in content.
        This method has limits on input size, processing time, and output size.
        
        When no InfoTypes or CustomInfoTypes are specified in this request, the
        system will automatically choose what detectors to run. By default this may
        be all types, but may change over time as detectors are updated.
        
        For how to guides, see https://cloud.google.com/dlp/docs/inspecting-images
        and https://cloud.google.com/dlp/docs/inspecting-text,"##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_content-inspect",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The parent resource name, for example projects/my-project-id."##),
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
            ("content-reidentify",
                    Some(r##"Re-identifies content that has been de-identified.
        See
        https://cloud.google.com/dlp/docs/pseudonymization#re-identification_in_free_text_code_example
        to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_content-reidentify",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The parent resource name."##),
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
            ("deidentify-templates-create",
                    Some(r##"Creates a DeidentifyTemplate for re-using frequently used configuration
        for de-identifying content, images, and storage.
        See https://cloud.google.com/dlp/docs/creating-templates-deid to learn
        more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_deidentify-templates-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The parent resource name, for example projects/my-project-id or
        organizations/my-org-id."##),
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
            ("deidentify-templates-delete",
                    Some(r##"Deletes a DeidentifyTemplate.
        See https://cloud.google.com/dlp/docs/creating-templates-deid to learn
        more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_deidentify-templates-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Resource name of the organization and deidentify template to be deleted,
        for example `organizations/433245324/deidentifyTemplates/432452342` or
        projects/project-id/deidentifyTemplates/432452342."##),
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
            ("deidentify-templates-get",
                    Some(r##"Gets a DeidentifyTemplate.
        See https://cloud.google.com/dlp/docs/creating-templates-deid to learn
        more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_deidentify-templates-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Resource name of the organization and deidentify template to be read, for
        example `organizations/433245324/deidentifyTemplates/432452342` or
        projects/project-id/deidentifyTemplates/432452342."##),
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
            ("deidentify-templates-list",
                    Some(r##"Lists DeidentifyTemplates.
        See https://cloud.google.com/dlp/docs/creating-templates-deid to learn
        more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_deidentify-templates-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The parent resource name, for example projects/my-project-id or
        organizations/my-org-id."##),
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
            ("deidentify-templates-patch",
                    Some(r##"Updates the DeidentifyTemplate.
        See https://cloud.google.com/dlp/docs/creating-templates-deid to learn
        more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_deidentify-templates-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Resource name of organization and deidentify template to be updated, for
        example `organizations/433245324/deidentifyTemplates/432452342` or
        projects/project-id/deidentifyTemplates/432452342."##),
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
            ("dlp-jobs-cancel",
                    Some(r##"Starts asynchronous cancellation on a long-running DlpJob. The server
        makes a best effort to cancel the DlpJob, but success is not
        guaranteed.
        See https://cloud.google.com/dlp/docs/inspecting-storage and
        https://cloud.google.com/dlp/docs/compute-risk-analysis to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_dlp-jobs-cancel",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the DlpJob resource to be cancelled."##),
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
            ("dlp-jobs-create",
                    Some(r##"Creates a new job to inspect storage or calculate risk metrics.
        See https://cloud.google.com/dlp/docs/inspecting-storage and
        https://cloud.google.com/dlp/docs/compute-risk-analysis to learn more.
        
        When no InfoTypes or CustomInfoTypes are specified in inspect jobs, the
        system will automatically choose what detectors to run. By default this may
        be all types, but may change over time as detectors are updated."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_dlp-jobs-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The parent resource name, for example projects/my-project-id."##),
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
            ("dlp-jobs-delete",
                    Some(r##"Deletes a long-running DlpJob. This method indicates that the client is
        no longer interested in the DlpJob result. The job will be cancelled if
        possible.
        See https://cloud.google.com/dlp/docs/inspecting-storage and
        https://cloud.google.com/dlp/docs/compute-risk-analysis to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_dlp-jobs-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the DlpJob resource to be deleted."##),
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
            ("dlp-jobs-get",
                    Some(r##"Gets the latest state of a long-running DlpJob.
        See https://cloud.google.com/dlp/docs/inspecting-storage and
        https://cloud.google.com/dlp/docs/compute-risk-analysis to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_dlp-jobs-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the DlpJob resource."##),
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
            ("dlp-jobs-list",
                    Some(r##"Lists DlpJobs that match the specified filter in the request.
        See https://cloud.google.com/dlp/docs/inspecting-storage and
        https://cloud.google.com/dlp/docs/compute-risk-analysis to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_dlp-jobs-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The parent resource name, for example projects/my-project-id."##),
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
            ("image-redact",
                    Some(r##"Redacts potentially sensitive info from an image.
        This method has limits on input size, processing time, and output size.
        See https://cloud.google.com/dlp/docs/redacting-sensitive-data-images to
        learn more.
        
        When no InfoTypes or CustomInfoTypes are specified in this request, the
        system will automatically choose what detectors to run. By default this may
        be all types, but may change over time as detectors are updated."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_image-redact",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The parent resource name, for example projects/my-project-id."##),
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
            ("inspect-templates-create",
                    Some(r##"Creates an InspectTemplate for re-using frequently used configuration
        for inspecting content, images, and storage.
        See https://cloud.google.com/dlp/docs/creating-templates to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_inspect-templates-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The parent resource name, for example projects/my-project-id or
        organizations/my-org-id."##),
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
            ("inspect-templates-delete",
                    Some(r##"Deletes an InspectTemplate.
        See https://cloud.google.com/dlp/docs/creating-templates to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_inspect-templates-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Resource name of the organization and inspectTemplate to be deleted, for
        example `organizations/433245324/inspectTemplates/432452342` or
        projects/project-id/inspectTemplates/432452342."##),
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
            ("inspect-templates-get",
                    Some(r##"Gets an InspectTemplate.
        See https://cloud.google.com/dlp/docs/creating-templates to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_inspect-templates-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Resource name of the organization and inspectTemplate to be read, for
        example `organizations/433245324/inspectTemplates/432452342` or
        projects/project-id/inspectTemplates/432452342."##),
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
            ("inspect-templates-list",
                    Some(r##"Lists InspectTemplates.
        See https://cloud.google.com/dlp/docs/creating-templates to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_inspect-templates-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The parent resource name, for example projects/my-project-id or
        organizations/my-org-id."##),
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
            ("inspect-templates-patch",
                    Some(r##"Updates the InspectTemplate.
        See https://cloud.google.com/dlp/docs/creating-templates to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_inspect-templates-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Resource name of organization and inspectTemplate to be updated, for
        example `organizations/433245324/inspectTemplates/432452342` or
        projects/project-id/inspectTemplates/432452342."##),
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
            ("job-triggers-create",
                    Some(r##"Creates a job trigger to run DLP actions such as scanning storage for
        sensitive information on a set schedule.
        See https://cloud.google.com/dlp/docs/creating-job-triggers to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_job-triggers-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The parent resource name, for example projects/my-project-id."##),
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
            ("job-triggers-delete",
                    Some(r##"Deletes a job trigger.
        See https://cloud.google.com/dlp/docs/creating-job-triggers to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_job-triggers-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Resource name of the project and the triggeredJob, for example
        `projects/dlp-test-project/jobTriggers/53234423`."##),
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
            ("job-triggers-get",
                    Some(r##"Gets a job trigger.
        See https://cloud.google.com/dlp/docs/creating-job-triggers to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_job-triggers-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Resource name of the project and the triggeredJob, for example
        `projects/dlp-test-project/jobTriggers/53234423`."##),
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
            ("job-triggers-list",
                    Some(r##"Lists job triggers.
        See https://cloud.google.com/dlp/docs/creating-job-triggers to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_job-triggers-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The parent resource name, for example `projects/my-project-id`."##),
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
            ("job-triggers-patch",
                    Some(r##"Updates a job trigger.
        See https://cloud.google.com/dlp/docs/creating-job-triggers to learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_job-triggers-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Resource name of the project and the triggeredJob, for example
        `projects/dlp-test-project/jobTriggers/53234423`."##),
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
            ("stored-info-types-create",
                    Some(r##"Creates a pre-built stored infoType to be used for inspection.
        See https://cloud.google.com/dlp/docs/creating-stored-infotypes to
        learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_stored-info-types-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The parent resource name, for example projects/my-project-id or
        organizations/my-org-id."##),
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
            ("stored-info-types-delete",
                    Some(r##"Deletes a stored infoType.
        See https://cloud.google.com/dlp/docs/creating-stored-infotypes to
        learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_stored-info-types-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Resource name of the organization and storedInfoType to be deleted, for
        example `organizations/433245324/storedInfoTypes/432452342` or
        projects/project-id/storedInfoTypes/432452342."##),
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
            ("stored-info-types-get",
                    Some(r##"Gets a stored infoType.
        See https://cloud.google.com/dlp/docs/creating-stored-infotypes to
        learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_stored-info-types-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Resource name of the organization and storedInfoType to be read, for
        example `organizations/433245324/storedInfoTypes/432452342` or
        projects/project-id/storedInfoTypes/432452342."##),
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
            ("stored-info-types-list",
                    Some(r##"Lists stored infoTypes.
        See https://cloud.google.com/dlp/docs/creating-stored-infotypes to
        learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_stored-info-types-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The parent resource name, for example projects/my-project-id or
        organizations/my-org-id."##),
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
            ("stored-info-types-patch",
                    Some(r##"Updates the stored infoType by creating a new version. The existing version
        will continue to be used until the new version is ready.
        See https://cloud.google.com/dlp/docs/creating-stored-infotypes to
        learn more."##),
                    "Details at http://byron.github.io/google-apis-rs/google_dlp2_cli/projects_stored-info-types-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Resource name of organization and storedInfoType to be updated, for
        example `organizations/433245324/storedInfoTypes/432452342` or
        projects/project-id/storedInfoTypes/432452342."##),
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
    
    let mut app = App::new("dlp2")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("1.0.7+20181009")
           .about("Provides methods for detection, risk analysis, and de-identification of privacy-sensitive fragments in text, images, and Google Cloud Platform storage repositories.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_dlp2_cli")
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