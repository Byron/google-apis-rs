// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_mybusiness4::{api, Error, oauth2, client::chrono, FieldMask};


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
    hub: api::MyBusiness<S>,
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
    async fn _accounts_admins_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "admin-name" => Some(("adminName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-invitation" => Some(("pendingInvitation", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "role" => Some(("role", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["admin-name", "name", "pending-invitation", "role"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Admin = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().admins_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _accounts_admins_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().admins_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _accounts_admins_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().admins_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _accounts_admins_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "admin-name" => Some(("adminName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-invitation" => Some(("pendingInvitation", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "role" => Some(("role", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["admin-name", "name", "pending-invitation", "role"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Admin = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().admins_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _accounts_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "account-name" => Some(("accountName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "account-number" => Some(("accountNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "organization-info.phone-number" => Some(("organizationInfo.phoneNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "organization-info.postal-address.address-lines" => Some(("organizationInfo.postalAddress.addressLines", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "organization-info.postal-address.administrative-area" => Some(("organizationInfo.postalAddress.administrativeArea", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "organization-info.postal-address.language-code" => Some(("organizationInfo.postalAddress.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "organization-info.postal-address.locality" => Some(("organizationInfo.postalAddress.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "organization-info.postal-address.organization" => Some(("organizationInfo.postalAddress.organization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "organization-info.postal-address.postal-code" => Some(("organizationInfo.postalAddress.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "organization-info.postal-address.recipients" => Some(("organizationInfo.postalAddress.recipients", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "organization-info.postal-address.region-code" => Some(("organizationInfo.postalAddress.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "organization-info.postal-address.revision" => Some(("organizationInfo.postalAddress.revision", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "organization-info.postal-address.sorting-code" => Some(("organizationInfo.postalAddress.sortingCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "organization-info.postal-address.sublocality" => Some(("organizationInfo.postalAddress.sublocality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "organization-info.registered-domain" => Some(("organizationInfo.registeredDomain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "permission-level" => Some(("permissionLevel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "role" => Some(("role", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state.status" => Some(("state.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state.vetted-status" => Some(("state.vettedStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-name", "account-number", "address-lines", "administrative-area", "language-code", "locality", "name", "organization", "organization-info", "permission-level", "phone-number", "postal-address", "postal-code", "recipients", "region-code", "registered-domain", "revision", "role", "sorting-code", "state", "status", "sublocality", "type", "vetted-status"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Account = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().create(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "primary-owner" => {
                    call = call.primary_owner(value.unwrap_or(""));
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
                                                                           v.extend(["primary-owner"].iter().map(|v|*v));
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

    async fn _accounts_delete_notifications(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().delete_notifications(opt.value_of("name").unwrap_or(""));
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

    async fn _accounts_generate_account_number(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::GenerateAccountNumberRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().generate_account_number(request, opt.value_of("name").unwrap_or(""));
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

    async fn _accounts_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().get(opt.value_of("name").unwrap_or(""));
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

    async fn _accounts_get_notifications(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().get_notifications(opt.value_of("name").unwrap_or(""));
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

    async fn _accounts_invitations_accept(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::AcceptInvitationRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().invitations_accept(request, opt.value_of("name").unwrap_or(""));
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

    async fn _accounts_invitations_decline(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::DeclineInvitationRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().invitations_decline(request, opt.value_of("name").unwrap_or(""));
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

    async fn _accounts_invitations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().invitations_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "target-type" => {
                    call = call.target_type(value.unwrap_or(""));
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
                                                                           v.extend(["target-type"].iter().map(|v|*v));
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
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "name" => {
                    call = call.name(value.unwrap_or(""));
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
                                                                           v.extend(["filter", "name", "page-size", "page-token"].iter().map(|v|*v));
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

    async fn _accounts_list_recommend_google_locations(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().list_recommend_google_locations(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["page-size", "page-token"].iter().map(|v|*v));
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

    async fn _accounts_locations_admins_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "admin-name" => Some(("adminName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-invitation" => Some(("pendingInvitation", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "role" => Some(("role", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["admin-name", "name", "pending-invitation", "role"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Admin = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().locations_admins_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _accounts_locations_admins_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().locations_admins_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _accounts_locations_admins_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().locations_admins_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _accounts_locations_admins_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "admin-name" => Some(("adminName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-invitation" => Some(("pendingInvitation", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "role" => Some(("role", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["admin-name", "name", "pending-invitation", "role"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Admin = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().locations_admins_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _accounts_locations_associate(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "place-id" => Some(("placeId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["place-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::AssociateLocationRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().locations_associate(request, opt.value_of("name").unwrap_or(""));
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

    async fn _accounts_locations_batch_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "location-names" => Some(("locationNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["location-names"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::BatchGetLocationsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().locations_batch_get(request, opt.value_of("name").unwrap_or(""));
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

    async fn _accounts_locations_batch_get_reviews(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "ignore-rating-only-reviews" => Some(("ignoreRatingOnlyReviews", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location-names" => Some(("locationNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "order-by" => Some(("orderBy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "page-size" => Some(("pageSize", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "page-token" => Some(("pageToken", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["ignore-rating-only-reviews", "location-names", "order-by", "page-size", "page-token"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::BatchGetReviewsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().locations_batch_get_reviews(request, opt.value_of("name").unwrap_or(""));
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

    async fn _accounts_locations_clear_association(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::ClearLocationAssociationRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().locations_clear_association(request, opt.value_of("name").unwrap_or(""));
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

    async fn _accounts_locations_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "ad-words-location-extensions.ad-phone" => Some(("adWordsLocationExtensions.adPhone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "additional-phones" => Some(("additionalPhones", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "address.address-lines" => Some(("address.addressLines", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "address.administrative-area" => Some(("address.administrativeArea", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "address.language-code" => Some(("address.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "address.locality" => Some(("address.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "address.organization" => Some(("address.organization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "address.postal-code" => Some(("address.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "address.recipients" => Some(("address.recipients", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "address.region-code" => Some(("address.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "address.revision" => Some(("address.revision", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "address.sorting-code" => Some(("address.sortingCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "address.sublocality" => Some(("address.sublocality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "language-code" => Some(("languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "latlng.latitude" => Some(("latlng.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "latlng.longitude" => Some(("latlng.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "location-key.explicit-no-place-id" => Some(("locationKey.explicitNoPlaceId", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location-key.place-id" => Some(("locationKey.placeId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-key.plus-page-id" => Some(("locationKey.plusPageId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-key.request-id" => Some(("locationKey.requestId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-name" => Some(("locationName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-state.can-delete" => Some(("locationState.canDelete", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location-state.can-update" => Some(("locationState.canUpdate", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location-state.has-pending-edits" => Some(("locationState.hasPendingEdits", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location-state.has-pending-verification" => Some(("locationState.hasPendingVerification", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location-state.is-disabled" => Some(("locationState.isDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location-state.is-disconnected" => Some(("locationState.isDisconnected", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location-state.is-duplicate" => Some(("locationState.isDuplicate", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location-state.is-google-updated" => Some(("locationState.isGoogleUpdated", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location-state.is-local-post-api-disabled" => Some(("locationState.isLocalPostApiDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location-state.is-pending-review" => Some(("locationState.isPendingReview", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location-state.is-published" => Some(("locationState.isPublished", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location-state.is-suspended" => Some(("locationState.isSuspended", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location-state.is-verified" => Some(("locationState.isVerified", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location-state.needs-reverification" => Some(("locationState.needsReverification", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "metadata.duplicate.access" => Some(("metadata.duplicate.access", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.duplicate.location-name" => Some(("metadata.duplicate.locationName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.duplicate.place-id" => Some(("metadata.duplicate.placeId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.maps-url" => Some(("metadata.mapsUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.new-review-url" => Some(("metadata.newReviewUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "open-info.can-reopen" => Some(("openInfo.canReopen", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "open-info.opening-date.day" => Some(("openInfo.openingDate.day", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "open-info.opening-date.month" => Some(("openInfo.openingDate.month", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "open-info.opening-date.year" => Some(("openInfo.openingDate.year", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "open-info.status" => Some(("openInfo.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "primary-category.category-id" => Some(("primaryCategory.categoryId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "primary-category.display-name" => Some(("primaryCategory.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "primary-phone" => Some(("primaryPhone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile.description" => Some(("profile.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "relationship-data.parent-chain" => Some(("relationshipData.parentChain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-area.business-type" => Some(("serviceArea.businessType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-area.radius.latlng.latitude" => Some(("serviceArea.radius.latlng.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "service-area.radius.latlng.longitude" => Some(("serviceArea.radius.latlng.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "service-area.radius.radius-km" => Some(("serviceArea.radius.radiusKm", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "store-code" => Some(("storeCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "website-url" => Some(("websiteUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["access", "ad-phone", "ad-words-location-extensions", "additional-phones", "address", "address-lines", "administrative-area", "business-type", "can-delete", "can-reopen", "can-update", "category-id", "day", "description", "display-name", "duplicate", "explicit-no-place-id", "has-pending-edits", "has-pending-verification", "is-disabled", "is-disconnected", "is-duplicate", "is-google-updated", "is-local-post-api-disabled", "is-pending-review", "is-published", "is-suspended", "is-verified", "labels", "language-code", "latitude", "latlng", "locality", "location-key", "location-name", "location-state", "longitude", "maps-url", "metadata", "month", "name", "needs-reverification", "new-review-url", "open-info", "opening-date", "organization", "parent-chain", "place-id", "plus-page-id", "postal-code", "primary-category", "primary-phone", "profile", "radius", "radius-km", "recipients", "region-code", "relationship-data", "request-id", "revision", "service-area", "sorting-code", "status", "store-code", "sublocality", "website-url", "year"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Location = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().locations_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "validate-only" => {
                    call = call.validate_only(        value.map(|v| arg_from_str(v, err, "validate-only", "boolean")).unwrap_or(false));
                },
                "request-id" => {
                    call = call.request_id(value.unwrap_or(""));
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
                                                                           v.extend(["request-id", "validate-only"].iter().map(|v|*v));
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

    async fn _accounts_locations_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().locations_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _accounts_locations_fetch_verification_options(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "context.address.address-lines" => Some(("context.address.addressLines", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "context.address.administrative-area" => Some(("context.address.administrativeArea", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "context.address.language-code" => Some(("context.address.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "context.address.locality" => Some(("context.address.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "context.address.organization" => Some(("context.address.organization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "context.address.postal-code" => Some(("context.address.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "context.address.recipients" => Some(("context.address.recipients", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "context.address.region-code" => Some(("context.address.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "context.address.revision" => Some(("context.address.revision", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "context.address.sorting-code" => Some(("context.address.sortingCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "context.address.sublocality" => Some(("context.address.sublocality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "language-code" => Some(("languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["address", "address-lines", "administrative-area", "context", "language-code", "locality", "organization", "postal-code", "recipients", "region-code", "revision", "sorting-code", "sublocality"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::FetchVerificationOptionsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().locations_fetch_verification_options(request, opt.value_of("name").unwrap_or(""));
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

    async fn _accounts_locations_find_matches(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "language-code" => Some(("languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "max-cache-duration" => Some(("maxCacheDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "num-results" => Some(("numResults", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["language-code", "max-cache-duration", "num-results"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::FindMatchingLocationsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().locations_find_matches(request, opt.value_of("name").unwrap_or(""));
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

    async fn _accounts_locations_followers_get_metadata(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().locations_followers_get_metadata(opt.value_of("name").unwrap_or(""));
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

    async fn _accounts_locations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().locations_get(opt.value_of("name").unwrap_or(""));
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

    async fn _accounts_locations_get_google_updated(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().locations_get_google_updated(opt.value_of("name").unwrap_or(""));
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

    async fn _accounts_locations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().locations_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
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
                                                                           v.extend(["filter", "language-code", "order-by", "page-size", "page-token"].iter().map(|v|*v));
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

    async fn _accounts_locations_local_posts_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "alert-type" => Some(("alertType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "call-to-action.action-type" => Some(("callToAction.actionType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "call-to-action.url" => Some(("callToAction.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event.schedule.end-date.day" => Some(("event.schedule.endDate.day", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "event.schedule.end-date.month" => Some(("event.schedule.endDate.month", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "event.schedule.end-date.year" => Some(("event.schedule.endDate.year", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "event.schedule.end-time.hours" => Some(("event.schedule.endTime.hours", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "event.schedule.end-time.minutes" => Some(("event.schedule.endTime.minutes", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "event.schedule.end-time.nanos" => Some(("event.schedule.endTime.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "event.schedule.end-time.seconds" => Some(("event.schedule.endTime.seconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "event.schedule.start-date.day" => Some(("event.schedule.startDate.day", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "event.schedule.start-date.month" => Some(("event.schedule.startDate.month", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "event.schedule.start-date.year" => Some(("event.schedule.startDate.year", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "event.schedule.start-time.hours" => Some(("event.schedule.startTime.hours", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "event.schedule.start-time.minutes" => Some(("event.schedule.startTime.minutes", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "event.schedule.start-time.nanos" => Some(("event.schedule.startTime.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "event.schedule.start-time.seconds" => Some(("event.schedule.startTime.seconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "event.title" => Some(("event.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "language-code" => Some(("languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "offer.coupon-code" => Some(("offer.couponCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "offer.redeem-online-url" => Some(("offer.redeemOnlineUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "offer.terms-conditions" => Some(("offer.termsConditions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "search-url" => Some(("searchUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "summary" => Some(("summary", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "topic-type" => Some(("topicType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["action-type", "alert-type", "call-to-action", "coupon-code", "create-time", "day", "end-date", "end-time", "event", "hours", "language-code", "minutes", "month", "name", "nanos", "offer", "redeem-online-url", "schedule", "search-url", "seconds", "start-date", "start-time", "state", "summary", "terms-conditions", "title", "topic-type", "update-time", "url", "year"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::LocalPost = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().locations_local_posts_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _accounts_locations_local_posts_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().locations_local_posts_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _accounts_locations_local_posts_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().locations_local_posts_get(opt.value_of("name").unwrap_or(""));
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

    async fn _accounts_locations_local_posts_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().locations_local_posts_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["page-size", "page-token"].iter().map(|v|*v));
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

    async fn _accounts_locations_local_posts_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "alert-type" => Some(("alertType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "call-to-action.action-type" => Some(("callToAction.actionType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "call-to-action.url" => Some(("callToAction.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "event.schedule.end-date.day" => Some(("event.schedule.endDate.day", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "event.schedule.end-date.month" => Some(("event.schedule.endDate.month", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "event.schedule.end-date.year" => Some(("event.schedule.endDate.year", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "event.schedule.end-time.hours" => Some(("event.schedule.endTime.hours", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "event.schedule.end-time.minutes" => Some(("event.schedule.endTime.minutes", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "event.schedule.end-time.nanos" => Some(("event.schedule.endTime.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "event.schedule.end-time.seconds" => Some(("event.schedule.endTime.seconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "event.schedule.start-date.day" => Some(("event.schedule.startDate.day", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "event.schedule.start-date.month" => Some(("event.schedule.startDate.month", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "event.schedule.start-date.year" => Some(("event.schedule.startDate.year", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "event.schedule.start-time.hours" => Some(("event.schedule.startTime.hours", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "event.schedule.start-time.minutes" => Some(("event.schedule.startTime.minutes", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "event.schedule.start-time.nanos" => Some(("event.schedule.startTime.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "event.schedule.start-time.seconds" => Some(("event.schedule.startTime.seconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "event.title" => Some(("event.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "language-code" => Some(("languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "offer.coupon-code" => Some(("offer.couponCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "offer.redeem-online-url" => Some(("offer.redeemOnlineUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "offer.terms-conditions" => Some(("offer.termsConditions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "search-url" => Some(("searchUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "summary" => Some(("summary", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "topic-type" => Some(("topicType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["action-type", "alert-type", "call-to-action", "coupon-code", "create-time", "day", "end-date", "end-time", "event", "hours", "language-code", "minutes", "month", "name", "nanos", "offer", "redeem-online-url", "schedule", "search-url", "seconds", "start-date", "start-time", "state", "summary", "terms-conditions", "title", "topic-type", "update-time", "url", "year"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::LocalPost = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().locations_local_posts_patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
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
                                                                           v.extend(["update-mask"].iter().map(|v|*v));
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

    async fn _accounts_locations_local_posts_report_insights(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "basic-request.time-range.end-time" => Some(("basicRequest.timeRange.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "basic-request.time-range.start-time" => Some(("basicRequest.timeRange.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "local-post-names" => Some(("localPostNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["basic-request", "end-time", "local-post-names", "start-time", "time-range"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ReportLocalPostInsightsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().locations_local_posts_report_insights(request, opt.value_of("name").unwrap_or(""));
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

    async fn _accounts_locations_media_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "attribution.profile-name" => Some(("attribution.profileName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attribution.profile-photo-url" => Some(("attribution.profilePhotoUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attribution.profile-url" => Some(("attribution.profileUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attribution.takedown-url" => Some(("attribution.takedownUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "data-ref.resource-name" => Some(("dataRef.resourceName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dimensions.height-pixels" => Some(("dimensions.heightPixels", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "dimensions.width-pixels" => Some(("dimensions.widthPixels", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "google-url" => Some(("googleUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "insights.view-count" => Some(("insights.viewCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "location-association.category" => Some(("locationAssociation.category", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-association.price-list-item-id" => Some(("locationAssociation.priceListItemId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "media-format" => Some(("mediaFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-url" => Some(("sourceUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "thumbnail-url" => Some(("thumbnailUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["attribution", "category", "create-time", "data-ref", "description", "dimensions", "google-url", "height-pixels", "insights", "location-association", "media-format", "name", "price-list-item-id", "profile-name", "profile-photo-url", "profile-url", "resource-name", "source-url", "takedown-url", "thumbnail-url", "view-count", "width-pixels"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::MediaItem = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().locations_media_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _accounts_locations_media_customers_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().locations_media_customers_get(opt.value_of("name").unwrap_or(""));
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

    async fn _accounts_locations_media_customers_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().locations_media_customers_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["page-size", "page-token"].iter().map(|v|*v));
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

    async fn _accounts_locations_media_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().locations_media_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _accounts_locations_media_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().locations_media_get(opt.value_of("name").unwrap_or(""));
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

    async fn _accounts_locations_media_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().locations_media_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["page-size", "page-token"].iter().map(|v|*v));
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

    async fn _accounts_locations_media_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "attribution.profile-name" => Some(("attribution.profileName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attribution.profile-photo-url" => Some(("attribution.profilePhotoUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attribution.profile-url" => Some(("attribution.profileUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "attribution.takedown-url" => Some(("attribution.takedownUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "data-ref.resource-name" => Some(("dataRef.resourceName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dimensions.height-pixels" => Some(("dimensions.heightPixels", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "dimensions.width-pixels" => Some(("dimensions.widthPixels", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "google-url" => Some(("googleUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "insights.view-count" => Some(("insights.viewCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "location-association.category" => Some(("locationAssociation.category", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-association.price-list-item-id" => Some(("locationAssociation.priceListItemId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "media-format" => Some(("mediaFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-url" => Some(("sourceUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "thumbnail-url" => Some(("thumbnailUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["attribution", "category", "create-time", "data-ref", "description", "dimensions", "google-url", "height-pixels", "insights", "location-association", "media-format", "name", "price-list-item-id", "profile-name", "profile-photo-url", "profile-url", "resource-name", "source-url", "takedown-url", "thumbnail-url", "view-count", "width-pixels"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::MediaItem = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().locations_media_patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
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
                                                                           v.extend(["update-mask"].iter().map(|v|*v));
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

    async fn _accounts_locations_media_start_upload(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::StartUploadMediaItemDataRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().locations_media_start_upload(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _accounts_locations_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "ad-words-location-extensions.ad-phone" => Some(("adWordsLocationExtensions.adPhone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "additional-phones" => Some(("additionalPhones", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "address.address-lines" => Some(("address.addressLines", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "address.administrative-area" => Some(("address.administrativeArea", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "address.language-code" => Some(("address.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "address.locality" => Some(("address.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "address.organization" => Some(("address.organization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "address.postal-code" => Some(("address.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "address.recipients" => Some(("address.recipients", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "address.region-code" => Some(("address.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "address.revision" => Some(("address.revision", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "address.sorting-code" => Some(("address.sortingCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "address.sublocality" => Some(("address.sublocality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "language-code" => Some(("languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "latlng.latitude" => Some(("latlng.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "latlng.longitude" => Some(("latlng.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "location-key.explicit-no-place-id" => Some(("locationKey.explicitNoPlaceId", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location-key.place-id" => Some(("locationKey.placeId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-key.plus-page-id" => Some(("locationKey.plusPageId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-key.request-id" => Some(("locationKey.requestId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-name" => Some(("locationName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-state.can-delete" => Some(("locationState.canDelete", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location-state.can-update" => Some(("locationState.canUpdate", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location-state.has-pending-edits" => Some(("locationState.hasPendingEdits", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location-state.has-pending-verification" => Some(("locationState.hasPendingVerification", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location-state.is-disabled" => Some(("locationState.isDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location-state.is-disconnected" => Some(("locationState.isDisconnected", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location-state.is-duplicate" => Some(("locationState.isDuplicate", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location-state.is-google-updated" => Some(("locationState.isGoogleUpdated", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location-state.is-local-post-api-disabled" => Some(("locationState.isLocalPostApiDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location-state.is-pending-review" => Some(("locationState.isPendingReview", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location-state.is-published" => Some(("locationState.isPublished", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location-state.is-suspended" => Some(("locationState.isSuspended", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location-state.is-verified" => Some(("locationState.isVerified", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location-state.needs-reverification" => Some(("locationState.needsReverification", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "metadata.duplicate.access" => Some(("metadata.duplicate.access", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.duplicate.location-name" => Some(("metadata.duplicate.locationName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.duplicate.place-id" => Some(("metadata.duplicate.placeId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.maps-url" => Some(("metadata.mapsUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "metadata.new-review-url" => Some(("metadata.newReviewUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "open-info.can-reopen" => Some(("openInfo.canReopen", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "open-info.opening-date.day" => Some(("openInfo.openingDate.day", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "open-info.opening-date.month" => Some(("openInfo.openingDate.month", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "open-info.opening-date.year" => Some(("openInfo.openingDate.year", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "open-info.status" => Some(("openInfo.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "primary-category.category-id" => Some(("primaryCategory.categoryId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "primary-category.display-name" => Some(("primaryCategory.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "primary-phone" => Some(("primaryPhone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "profile.description" => Some(("profile.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "relationship-data.parent-chain" => Some(("relationshipData.parentChain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-area.business-type" => Some(("serviceArea.businessType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "service-area.radius.latlng.latitude" => Some(("serviceArea.radius.latlng.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "service-area.radius.latlng.longitude" => Some(("serviceArea.radius.latlng.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "service-area.radius.radius-km" => Some(("serviceArea.radius.radiusKm", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "store-code" => Some(("storeCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "website-url" => Some(("websiteUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["access", "ad-phone", "ad-words-location-extensions", "additional-phones", "address", "address-lines", "administrative-area", "business-type", "can-delete", "can-reopen", "can-update", "category-id", "day", "description", "display-name", "duplicate", "explicit-no-place-id", "has-pending-edits", "has-pending-verification", "is-disabled", "is-disconnected", "is-duplicate", "is-google-updated", "is-local-post-api-disabled", "is-pending-review", "is-published", "is-suspended", "is-verified", "labels", "language-code", "latitude", "latlng", "locality", "location-key", "location-name", "location-state", "longitude", "maps-url", "metadata", "month", "name", "needs-reverification", "new-review-url", "open-info", "opening-date", "organization", "parent-chain", "place-id", "plus-page-id", "postal-code", "primary-category", "primary-phone", "profile", "radius", "radius-km", "recipients", "region-code", "relationship-data", "request-id", "revision", "service-area", "sorting-code", "status", "store-code", "sublocality", "website-url", "year"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Location = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().locations_patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "validate-only" => {
                    call = call.validate_only(        value.map(|v| arg_from_str(v, err, "validate-only", "boolean")).unwrap_or(false));
                },
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
                },
                "attribute-mask" => {
                    call = call.attribute_mask(        value.map(|v| arg_from_str(v, err, "attribute-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
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
                                                                           v.extend(["attribute-mask", "update-mask", "validate-only"].iter().map(|v|*v));
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

    async fn _accounts_locations_questions_answers_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().locations_questions_answers_delete(opt.value_of("parent").unwrap_or(""));
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

    async fn _accounts_locations_questions_answers_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().locations_questions_answers_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["order-by", "page-size", "page-token"].iter().map(|v|*v));
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

    async fn _accounts_locations_questions_answers_upsert(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "answer.author.display-name" => Some(("answer.author.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "answer.author.profile-photo-url" => Some(("answer.author.profilePhotoUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "answer.author.type" => Some(("answer.author.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "answer.create-time" => Some(("answer.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "answer.name" => Some(("answer.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "answer.text" => Some(("answer.text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "answer.update-time" => Some(("answer.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "answer.upvote-count" => Some(("answer.upvoteCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["answer", "author", "create-time", "display-name", "name", "profile-photo-url", "text", "type", "update-time", "upvote-count"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::UpsertAnswerRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().locations_questions_answers_upsert(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _accounts_locations_questions_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "author.display-name" => Some(("author.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.profile-photo-url" => Some(("author.profilePhotoUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.type" => Some(("author.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "text" => Some(("text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "total-answer-count" => Some(("totalAnswerCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upvote-count" => Some(("upvoteCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["author", "create-time", "display-name", "name", "profile-photo-url", "text", "total-answer-count", "type", "update-time", "upvote-count"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Question = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().locations_questions_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _accounts_locations_questions_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().locations_questions_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _accounts_locations_questions_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().locations_questions_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                "answers-per-question" => {
                    call = call.answers_per_question(        value.map(|v| arg_from_str(v, err, "answers-per-question", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["answers-per-question", "filter", "order-by", "page-size", "page-token"].iter().map(|v|*v));
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

    async fn _accounts_locations_questions_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "author.display-name" => Some(("author.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.profile-photo-url" => Some(("author.profilePhotoUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "author.type" => Some(("author.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "text" => Some(("text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "total-answer-count" => Some(("totalAnswerCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upvote-count" => Some(("upvoteCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["author", "create-time", "display-name", "name", "profile-photo-url", "text", "total-answer-count", "type", "update-time", "upvote-count"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Question = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().locations_questions_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _accounts_locations_report_insights(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "basic-request.time-range.end-time" => Some(("basicRequest.timeRange.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "basic-request.time-range.start-time" => Some(("basicRequest.timeRange.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "driving-directions-request.language-code" => Some(("drivingDirectionsRequest.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "driving-directions-request.num-days" => Some(("drivingDirectionsRequest.numDays", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-names" => Some(("locationNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["basic-request", "driving-directions-request", "end-time", "language-code", "location-names", "num-days", "start-time", "time-range"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ReportLocationInsightsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().locations_report_insights(request, opt.value_of("name").unwrap_or(""));
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

    async fn _accounts_locations_reviews_delete_reply(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().locations_reviews_delete_reply(opt.value_of("name").unwrap_or(""));
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

    async fn _accounts_locations_reviews_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().locations_reviews_get(opt.value_of("name").unwrap_or(""));
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

    async fn _accounts_locations_reviews_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().locations_reviews_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["order-by", "page-size", "page-token"].iter().map(|v|*v));
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

    async fn _accounts_locations_reviews_update_reply(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "comment" => Some(("comment", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["comment", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ReviewReply = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().locations_reviews_update_reply(request, opt.value_of("name").unwrap_or(""));
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

    async fn _accounts_locations_transfer(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "to-account" => Some(("toAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["to-account"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::TransferLocationRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().locations_transfer(request, opt.value_of("name").unwrap_or(""));
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

    async fn _accounts_locations_verifications_complete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "pin" => Some(("pin", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["pin"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CompleteVerificationRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().locations_verifications_complete(request, opt.value_of("name").unwrap_or(""));
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

    async fn _accounts_locations_verifications_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.accounts().locations_verifications_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["page-size", "page-token"].iter().map(|v|*v));
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

    async fn _accounts_locations_verify(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "address-input.mailer-contact-name" => Some(("addressInput.mailerContactName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "context.address.address-lines" => Some(("context.address.addressLines", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "context.address.administrative-area" => Some(("context.address.administrativeArea", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "context.address.language-code" => Some(("context.address.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "context.address.locality" => Some(("context.address.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "context.address.organization" => Some(("context.address.organization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "context.address.postal-code" => Some(("context.address.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "context.address.recipients" => Some(("context.address.recipients", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "context.address.region-code" => Some(("context.address.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "context.address.revision" => Some(("context.address.revision", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "context.address.sorting-code" => Some(("context.address.sortingCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "context.address.sublocality" => Some(("context.address.sublocality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "email-input.email-address" => Some(("emailInput.emailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "language-code" => Some(("languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "method" => Some(("method", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "phone-input.phone-number" => Some(("phoneInput.phoneNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "vetted-partner-input.token.token-string" => Some(("vettedPartnerInput.token.tokenString", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["address", "address-input", "address-lines", "administrative-area", "context", "email-address", "email-input", "language-code", "locality", "mailer-contact-name", "method", "organization", "phone-input", "phone-number", "postal-code", "recipients", "region-code", "revision", "sorting-code", "sublocality", "token", "token-string", "vetted-partner-input"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::VerifyLocationRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().locations_verify(request, opt.value_of("name").unwrap_or(""));
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
                    "account-name" => Some(("accountName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "account-number" => Some(("accountNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "organization-info.phone-number" => Some(("organizationInfo.phoneNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "organization-info.postal-address.address-lines" => Some(("organizationInfo.postalAddress.addressLines", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "organization-info.postal-address.administrative-area" => Some(("organizationInfo.postalAddress.administrativeArea", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "organization-info.postal-address.language-code" => Some(("organizationInfo.postalAddress.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "organization-info.postal-address.locality" => Some(("organizationInfo.postalAddress.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "organization-info.postal-address.organization" => Some(("organizationInfo.postalAddress.organization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "organization-info.postal-address.postal-code" => Some(("organizationInfo.postalAddress.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "organization-info.postal-address.recipients" => Some(("organizationInfo.postalAddress.recipients", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "organization-info.postal-address.region-code" => Some(("organizationInfo.postalAddress.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "organization-info.postal-address.revision" => Some(("organizationInfo.postalAddress.revision", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "organization-info.postal-address.sorting-code" => Some(("organizationInfo.postalAddress.sortingCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "organization-info.postal-address.sublocality" => Some(("organizationInfo.postalAddress.sublocality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "organization-info.registered-domain" => Some(("organizationInfo.registeredDomain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "permission-level" => Some(("permissionLevel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "role" => Some(("role", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state.status" => Some(("state.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state.vetted-status" => Some(("state.vettedStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["account-name", "account-number", "address-lines", "administrative-area", "language-code", "locality", "name", "organization", "organization-info", "permission-level", "phone-number", "postal-address", "postal-code", "recipients", "region-code", "registered-domain", "revision", "role", "sorting-code", "state", "status", "sublocality", "type", "vetted-status"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Account = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().update(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "validate-only" => {
                    call = call.validate_only(        value.map(|v| arg_from_str(v, err, "validate-only", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["validate-only"].iter().map(|v|*v));
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

    async fn _accounts_update_notifications(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "notification-types" => Some(("notificationTypes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "topic-name" => Some(("topicName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["name", "notification-types", "topic-name"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Notifications = json::value::from_value(object).unwrap();
        let mut call = self.hub.accounts().update_notifications(request, opt.value_of("name").unwrap_or(""));
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

    async fn _attributes_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.attributes().list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "name" => {
                    call = call.name(value.unwrap_or(""));
                },
                "language-code" => {
                    call = call.language_code(value.unwrap_or(""));
                },
                "country" => {
                    call = call.country(value.unwrap_or(""));
                },
                "category-id" => {
                    call = call.category_id(value.unwrap_or(""));
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
                                                                           v.extend(["category-id", "country", "language-code", "name", "page-size", "page-token"].iter().map(|v|*v));
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

    async fn _categories_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.categories().list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "search-term" => {
                    call = call.search_term(value.unwrap_or(""));
                },
                "region-code" => {
                    call = call.region_code(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "language-code" => {
                    call = call.language_code(value.unwrap_or(""));
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
                                                                           v.extend(["language-code", "page-size", "page-token", "region-code", "search-term"].iter().map(|v|*v));
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

    async fn _chains_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.chains().get(opt.value_of("name").unwrap_or(""));
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

    async fn _chains_search(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.chains().search();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "result-count" => {
                    call = call.result_count(        value.map(|v| arg_from_str(v, err, "result-count", "int32")).unwrap_or(-0));
                },
                "chain-display-name" => {
                    call = call.chain_display_name(value.unwrap_or(""));
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
                                                                           v.extend(["chain-display-name", "result-count"].iter().map(|v|*v));
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

    async fn _google_locations_report(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "location-group-name" => Some(("locationGroupName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "report-reason-bad-location" => Some(("reportReasonBadLocation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "report-reason-bad-recommendation" => Some(("reportReasonBadRecommendation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "report-reason-elaboration" => Some(("reportReasonElaboration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "report-reason-language-code" => Some(("reportReasonLanguageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["location-group-name", "report-reason-bad-location", "report-reason-bad-recommendation", "report-reason-elaboration", "report-reason-language-code"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ReportGoogleLocationRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.google_locations().report(request, opt.value_of("name").unwrap_or(""));
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

    async fn _google_locations_search(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "location.ad-words-location-extensions.ad-phone" => Some(("location.adWordsLocationExtensions.adPhone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.additional-phones" => Some(("location.additionalPhones", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "location.address.address-lines" => Some(("location.address.addressLines", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "location.address.administrative-area" => Some(("location.address.administrativeArea", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.address.language-code" => Some(("location.address.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.address.locality" => Some(("location.address.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.address.organization" => Some(("location.address.organization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.address.postal-code" => Some(("location.address.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.address.recipients" => Some(("location.address.recipients", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "location.address.region-code" => Some(("location.address.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.address.revision" => Some(("location.address.revision", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "location.address.sorting-code" => Some(("location.address.sortingCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.address.sublocality" => Some(("location.address.sublocality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.labels" => Some(("location.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "location.language-code" => Some(("location.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.latlng.latitude" => Some(("location.latlng.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "location.latlng.longitude" => Some(("location.latlng.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "location.location-key.explicit-no-place-id" => Some(("location.locationKey.explicitNoPlaceId", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.location-key.place-id" => Some(("location.locationKey.placeId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.location-key.plus-page-id" => Some(("location.locationKey.plusPageId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.location-key.request-id" => Some(("location.locationKey.requestId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.location-name" => Some(("location.locationName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.location-state.can-delete" => Some(("location.locationState.canDelete", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.location-state.can-update" => Some(("location.locationState.canUpdate", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.location-state.has-pending-edits" => Some(("location.locationState.hasPendingEdits", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.location-state.has-pending-verification" => Some(("location.locationState.hasPendingVerification", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.location-state.is-disabled" => Some(("location.locationState.isDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.location-state.is-disconnected" => Some(("location.locationState.isDisconnected", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.location-state.is-duplicate" => Some(("location.locationState.isDuplicate", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.location-state.is-google-updated" => Some(("location.locationState.isGoogleUpdated", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.location-state.is-local-post-api-disabled" => Some(("location.locationState.isLocalPostApiDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.location-state.is-pending-review" => Some(("location.locationState.isPendingReview", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.location-state.is-published" => Some(("location.locationState.isPublished", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.location-state.is-suspended" => Some(("location.locationState.isSuspended", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.location-state.is-verified" => Some(("location.locationState.isVerified", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.location-state.needs-reverification" => Some(("location.locationState.needsReverification", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.metadata.duplicate.access" => Some(("location.metadata.duplicate.access", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.metadata.duplicate.location-name" => Some(("location.metadata.duplicate.locationName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.metadata.duplicate.place-id" => Some(("location.metadata.duplicate.placeId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.metadata.maps-url" => Some(("location.metadata.mapsUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.metadata.new-review-url" => Some(("location.metadata.newReviewUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.name" => Some(("location.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.open-info.can-reopen" => Some(("location.openInfo.canReopen", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.open-info.opening-date.day" => Some(("location.openInfo.openingDate.day", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "location.open-info.opening-date.month" => Some(("location.openInfo.openingDate.month", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "location.open-info.opening-date.year" => Some(("location.openInfo.openingDate.year", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "location.open-info.status" => Some(("location.openInfo.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.primary-category.category-id" => Some(("location.primaryCategory.categoryId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.primary-category.display-name" => Some(("location.primaryCategory.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.primary-phone" => Some(("location.primaryPhone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.profile.description" => Some(("location.profile.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.relationship-data.parent-chain" => Some(("location.relationshipData.parentChain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.service-area.business-type" => Some(("location.serviceArea.businessType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.service-area.radius.latlng.latitude" => Some(("location.serviceArea.radius.latlng.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "location.service-area.radius.latlng.longitude" => Some(("location.serviceArea.radius.latlng.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "location.service-area.radius.radius-km" => Some(("location.serviceArea.radius.radiusKm", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "location.store-code" => Some(("location.storeCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.website-url" => Some(("location.websiteUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "query" => Some(("query", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "result-count" => Some(("resultCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["access", "ad-phone", "ad-words-location-extensions", "additional-phones", "address", "address-lines", "administrative-area", "business-type", "can-delete", "can-reopen", "can-update", "category-id", "day", "description", "display-name", "duplicate", "explicit-no-place-id", "has-pending-edits", "has-pending-verification", "is-disabled", "is-disconnected", "is-duplicate", "is-google-updated", "is-local-post-api-disabled", "is-pending-review", "is-published", "is-suspended", "is-verified", "labels", "language-code", "latitude", "latlng", "locality", "location", "location-key", "location-name", "location-state", "longitude", "maps-url", "metadata", "month", "name", "needs-reverification", "new-review-url", "open-info", "opening-date", "organization", "parent-chain", "place-id", "plus-page-id", "postal-code", "primary-category", "primary-phone", "profile", "query", "radius", "radius-km", "recipients", "region-code", "relationship-data", "request-id", "result-count", "revision", "service-area", "sorting-code", "status", "store-code", "sublocality", "website-url", "year"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SearchGoogleLocationsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.google_locations().search(request);
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

    async fn _verification_tokens_generate(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "location.ad-words-location-extensions.ad-phone" => Some(("location.adWordsLocationExtensions.adPhone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.additional-phones" => Some(("location.additionalPhones", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "location.address.address-lines" => Some(("location.address.addressLines", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "location.address.administrative-area" => Some(("location.address.administrativeArea", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.address.language-code" => Some(("location.address.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.address.locality" => Some(("location.address.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.address.organization" => Some(("location.address.organization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.address.postal-code" => Some(("location.address.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.address.recipients" => Some(("location.address.recipients", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "location.address.region-code" => Some(("location.address.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.address.revision" => Some(("location.address.revision", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "location.address.sorting-code" => Some(("location.address.sortingCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.address.sublocality" => Some(("location.address.sublocality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.labels" => Some(("location.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "location.language-code" => Some(("location.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.latlng.latitude" => Some(("location.latlng.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "location.latlng.longitude" => Some(("location.latlng.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "location.location-key.explicit-no-place-id" => Some(("location.locationKey.explicitNoPlaceId", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.location-key.place-id" => Some(("location.locationKey.placeId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.location-key.plus-page-id" => Some(("location.locationKey.plusPageId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.location-key.request-id" => Some(("location.locationKey.requestId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.location-name" => Some(("location.locationName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.location-state.can-delete" => Some(("location.locationState.canDelete", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.location-state.can-update" => Some(("location.locationState.canUpdate", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.location-state.has-pending-edits" => Some(("location.locationState.hasPendingEdits", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.location-state.has-pending-verification" => Some(("location.locationState.hasPendingVerification", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.location-state.is-disabled" => Some(("location.locationState.isDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.location-state.is-disconnected" => Some(("location.locationState.isDisconnected", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.location-state.is-duplicate" => Some(("location.locationState.isDuplicate", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.location-state.is-google-updated" => Some(("location.locationState.isGoogleUpdated", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.location-state.is-local-post-api-disabled" => Some(("location.locationState.isLocalPostApiDisabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.location-state.is-pending-review" => Some(("location.locationState.isPendingReview", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.location-state.is-published" => Some(("location.locationState.isPublished", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.location-state.is-suspended" => Some(("location.locationState.isSuspended", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.location-state.is-verified" => Some(("location.locationState.isVerified", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.location-state.needs-reverification" => Some(("location.locationState.needsReverification", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.metadata.duplicate.access" => Some(("location.metadata.duplicate.access", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.metadata.duplicate.location-name" => Some(("location.metadata.duplicate.locationName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.metadata.duplicate.place-id" => Some(("location.metadata.duplicate.placeId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.metadata.maps-url" => Some(("location.metadata.mapsUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.metadata.new-review-url" => Some(("location.metadata.newReviewUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.name" => Some(("location.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.open-info.can-reopen" => Some(("location.openInfo.canReopen", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "location.open-info.opening-date.day" => Some(("location.openInfo.openingDate.day", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "location.open-info.opening-date.month" => Some(("location.openInfo.openingDate.month", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "location.open-info.opening-date.year" => Some(("location.openInfo.openingDate.year", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "location.open-info.status" => Some(("location.openInfo.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.primary-category.category-id" => Some(("location.primaryCategory.categoryId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.primary-category.display-name" => Some(("location.primaryCategory.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.primary-phone" => Some(("location.primaryPhone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.profile.description" => Some(("location.profile.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.relationship-data.parent-chain" => Some(("location.relationshipData.parentChain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.service-area.business-type" => Some(("location.serviceArea.businessType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.service-area.radius.latlng.latitude" => Some(("location.serviceArea.radius.latlng.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "location.service-area.radius.latlng.longitude" => Some(("location.serviceArea.radius.latlng.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "location.service-area.radius.radius-km" => Some(("location.serviceArea.radius.radiusKm", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "location.store-code" => Some(("location.storeCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.website-url" => Some(("location.websiteUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["access", "ad-phone", "ad-words-location-extensions", "additional-phones", "address", "address-lines", "administrative-area", "business-type", "can-delete", "can-reopen", "can-update", "category-id", "day", "description", "display-name", "duplicate", "explicit-no-place-id", "has-pending-edits", "has-pending-verification", "is-disabled", "is-disconnected", "is-duplicate", "is-google-updated", "is-local-post-api-disabled", "is-pending-review", "is-published", "is-suspended", "is-verified", "labels", "language-code", "latitude", "latlng", "locality", "location", "location-key", "location-name", "location-state", "longitude", "maps-url", "metadata", "month", "name", "needs-reverification", "new-review-url", "open-info", "opening-date", "organization", "parent-chain", "place-id", "plus-page-id", "postal-code", "primary-category", "primary-phone", "profile", "radius", "radius-km", "recipients", "region-code", "relationship-data", "request-id", "revision", "service-area", "sorting-code", "status", "store-code", "sublocality", "website-url", "year"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::GenerateVerificationTokenRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.verification_tokens().generate(request);
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
                    ("admins-create", Some(opt)) => {
                        call_result = self._accounts_admins_create(opt, dry_run, &mut err).await;
                    },
                    ("admins-delete", Some(opt)) => {
                        call_result = self._accounts_admins_delete(opt, dry_run, &mut err).await;
                    },
                    ("admins-list", Some(opt)) => {
                        call_result = self._accounts_admins_list(opt, dry_run, &mut err).await;
                    },
                    ("admins-patch", Some(opt)) => {
                        call_result = self._accounts_admins_patch(opt, dry_run, &mut err).await;
                    },
                    ("create", Some(opt)) => {
                        call_result = self._accounts_create(opt, dry_run, &mut err).await;
                    },
                    ("delete-notifications", Some(opt)) => {
                        call_result = self._accounts_delete_notifications(opt, dry_run, &mut err).await;
                    },
                    ("generate-account-number", Some(opt)) => {
                        call_result = self._accounts_generate_account_number(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._accounts_get(opt, dry_run, &mut err).await;
                    },
                    ("get-notifications", Some(opt)) => {
                        call_result = self._accounts_get_notifications(opt, dry_run, &mut err).await;
                    },
                    ("invitations-accept", Some(opt)) => {
                        call_result = self._accounts_invitations_accept(opt, dry_run, &mut err).await;
                    },
                    ("invitations-decline", Some(opt)) => {
                        call_result = self._accounts_invitations_decline(opt, dry_run, &mut err).await;
                    },
                    ("invitations-list", Some(opt)) => {
                        call_result = self._accounts_invitations_list(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._accounts_list(opt, dry_run, &mut err).await;
                    },
                    ("list-recommend-google-locations", Some(opt)) => {
                        call_result = self._accounts_list_recommend_google_locations(opt, dry_run, &mut err).await;
                    },
                    ("locations-admins-create", Some(opt)) => {
                        call_result = self._accounts_locations_admins_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-admins-delete", Some(opt)) => {
                        call_result = self._accounts_locations_admins_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-admins-list", Some(opt)) => {
                        call_result = self._accounts_locations_admins_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-admins-patch", Some(opt)) => {
                        call_result = self._accounts_locations_admins_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-associate", Some(opt)) => {
                        call_result = self._accounts_locations_associate(opt, dry_run, &mut err).await;
                    },
                    ("locations-batch-get", Some(opt)) => {
                        call_result = self._accounts_locations_batch_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-batch-get-reviews", Some(opt)) => {
                        call_result = self._accounts_locations_batch_get_reviews(opt, dry_run, &mut err).await;
                    },
                    ("locations-clear-association", Some(opt)) => {
                        call_result = self._accounts_locations_clear_association(opt, dry_run, &mut err).await;
                    },
                    ("locations-create", Some(opt)) => {
                        call_result = self._accounts_locations_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-delete", Some(opt)) => {
                        call_result = self._accounts_locations_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-fetch-verification-options", Some(opt)) => {
                        call_result = self._accounts_locations_fetch_verification_options(opt, dry_run, &mut err).await;
                    },
                    ("locations-find-matches", Some(opt)) => {
                        call_result = self._accounts_locations_find_matches(opt, dry_run, &mut err).await;
                    },
                    ("locations-followers-get-metadata", Some(opt)) => {
                        call_result = self._accounts_locations_followers_get_metadata(opt, dry_run, &mut err).await;
                    },
                    ("locations-get", Some(opt)) => {
                        call_result = self._accounts_locations_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-get-google-updated", Some(opt)) => {
                        call_result = self._accounts_locations_get_google_updated(opt, dry_run, &mut err).await;
                    },
                    ("locations-list", Some(opt)) => {
                        call_result = self._accounts_locations_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-local-posts-create", Some(opt)) => {
                        call_result = self._accounts_locations_local_posts_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-local-posts-delete", Some(opt)) => {
                        call_result = self._accounts_locations_local_posts_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-local-posts-get", Some(opt)) => {
                        call_result = self._accounts_locations_local_posts_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-local-posts-list", Some(opt)) => {
                        call_result = self._accounts_locations_local_posts_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-local-posts-patch", Some(opt)) => {
                        call_result = self._accounts_locations_local_posts_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-local-posts-report-insights", Some(opt)) => {
                        call_result = self._accounts_locations_local_posts_report_insights(opt, dry_run, &mut err).await;
                    },
                    ("locations-media-create", Some(opt)) => {
                        call_result = self._accounts_locations_media_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-media-customers-get", Some(opt)) => {
                        call_result = self._accounts_locations_media_customers_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-media-customers-list", Some(opt)) => {
                        call_result = self._accounts_locations_media_customers_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-media-delete", Some(opt)) => {
                        call_result = self._accounts_locations_media_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-media-get", Some(opt)) => {
                        call_result = self._accounts_locations_media_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-media-list", Some(opt)) => {
                        call_result = self._accounts_locations_media_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-media-patch", Some(opt)) => {
                        call_result = self._accounts_locations_media_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-media-start-upload", Some(opt)) => {
                        call_result = self._accounts_locations_media_start_upload(opt, dry_run, &mut err).await;
                    },
                    ("locations-patch", Some(opt)) => {
                        call_result = self._accounts_locations_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-questions-answers-delete", Some(opt)) => {
                        call_result = self._accounts_locations_questions_answers_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-questions-answers-list", Some(opt)) => {
                        call_result = self._accounts_locations_questions_answers_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-questions-answers-upsert", Some(opt)) => {
                        call_result = self._accounts_locations_questions_answers_upsert(opt, dry_run, &mut err).await;
                    },
                    ("locations-questions-create", Some(opt)) => {
                        call_result = self._accounts_locations_questions_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-questions-delete", Some(opt)) => {
                        call_result = self._accounts_locations_questions_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-questions-list", Some(opt)) => {
                        call_result = self._accounts_locations_questions_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-questions-patch", Some(opt)) => {
                        call_result = self._accounts_locations_questions_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-report-insights", Some(opt)) => {
                        call_result = self._accounts_locations_report_insights(opt, dry_run, &mut err).await;
                    },
                    ("locations-reviews-delete-reply", Some(opt)) => {
                        call_result = self._accounts_locations_reviews_delete_reply(opt, dry_run, &mut err).await;
                    },
                    ("locations-reviews-get", Some(opt)) => {
                        call_result = self._accounts_locations_reviews_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-reviews-list", Some(opt)) => {
                        call_result = self._accounts_locations_reviews_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-reviews-update-reply", Some(opt)) => {
                        call_result = self._accounts_locations_reviews_update_reply(opt, dry_run, &mut err).await;
                    },
                    ("locations-transfer", Some(opt)) => {
                        call_result = self._accounts_locations_transfer(opt, dry_run, &mut err).await;
                    },
                    ("locations-verifications-complete", Some(opt)) => {
                        call_result = self._accounts_locations_verifications_complete(opt, dry_run, &mut err).await;
                    },
                    ("locations-verifications-list", Some(opt)) => {
                        call_result = self._accounts_locations_verifications_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-verify", Some(opt)) => {
                        call_result = self._accounts_locations_verify(opt, dry_run, &mut err).await;
                    },
                    ("update", Some(opt)) => {
                        call_result = self._accounts_update(opt, dry_run, &mut err).await;
                    },
                    ("update-notifications", Some(opt)) => {
                        call_result = self._accounts_update_notifications(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("accounts".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("attributes", Some(opt)) => {
                match opt.subcommand() {
                    ("list", Some(opt)) => {
                        call_result = self._attributes_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("attributes".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("categories", Some(opt)) => {
                match opt.subcommand() {
                    ("list", Some(opt)) => {
                        call_result = self._categories_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("categories".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("chains", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._chains_get(opt, dry_run, &mut err).await;
                    },
                    ("search", Some(opt)) => {
                        call_result = self._chains_search(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("chains".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("google-locations", Some(opt)) => {
                match opt.subcommand() {
                    ("report", Some(opt)) => {
                        call_result = self._google_locations_report(opt, dry_run, &mut err).await;
                    },
                    ("search", Some(opt)) => {
                        call_result = self._google_locations_search(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("google-locations".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("verification-tokens", Some(opt)) => {
                match opt.subcommand() {
                    ("generate", Some(opt)) => {
                        call_result = self._verification_tokens_generate(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("verification-tokens".to_string()));
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

            match client::application_secret_from_directory(&config_dir, "mybusiness4-secret.json",
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
        ).persist_tokens_to_disk(format!("{}/mybusiness4", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::MyBusiness::new(client, auth),
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
        ("accounts", "methods: 'admins-create', 'admins-delete', 'admins-list', 'admins-patch', 'create', 'delete-notifications', 'generate-account-number', 'get', 'get-notifications', 'invitations-accept', 'invitations-decline', 'invitations-list', 'list', 'list-recommend-google-locations', 'locations-admins-create', 'locations-admins-delete', 'locations-admins-list', 'locations-admins-patch', 'locations-associate', 'locations-batch-get', 'locations-batch-get-reviews', 'locations-clear-association', 'locations-create', 'locations-delete', 'locations-fetch-verification-options', 'locations-find-matches', 'locations-followers-get-metadata', 'locations-get', 'locations-get-google-updated', 'locations-list', 'locations-local-posts-create', 'locations-local-posts-delete', 'locations-local-posts-get', 'locations-local-posts-list', 'locations-local-posts-patch', 'locations-local-posts-report-insights', 'locations-media-create', 'locations-media-customers-get', 'locations-media-customers-list', 'locations-media-delete', 'locations-media-get', 'locations-media-list', 'locations-media-patch', 'locations-media-start-upload', 'locations-patch', 'locations-questions-answers-delete', 'locations-questions-answers-list', 'locations-questions-answers-upsert', 'locations-questions-create', 'locations-questions-delete', 'locations-questions-list', 'locations-questions-patch', 'locations-report-insights', 'locations-reviews-delete-reply', 'locations-reviews-get', 'locations-reviews-list', 'locations-reviews-update-reply', 'locations-transfer', 'locations-verifications-complete', 'locations-verifications-list', 'locations-verify', 'update' and 'update-notifications'", vec![
            ("admins-create",
                    Some(r##"Invites the specified user to become an administrator for the specified
        account. The invitee must accept the invitation in order to be granted
        access to the account. See AcceptInvitation to programmatically accept an
        invitation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_admins-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The resource name of the account this admin is created for."##),
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
            ("admins-delete",
                    Some(r##"Removes the specified admin from the specified account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_admins-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The resource name of the admin to remove from the account."##),
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
            ("admins-list",
                    Some(r##"Lists the admins for the specified account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_admins-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The name of the account from which to retrieve a list of admins."##),
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
            ("admins-patch",
                    Some(r##"Updates the Admin for the specified Account Admin. Only the AdminRole of
        the Admin can be updated."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_admins-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The resource name of the admin to update."##),
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
            ("create",
                    Some(r##"Creates an account with the specified name and type under the given parent.
        <ul>
        <li> Personal accounts and Organizations cannot be created. </li>
        <li> User Groups cannot be created with a Personal account as primary
        owner. </li>
        <li> Location Groups cannot be created with a primary owner of a
        Personal account if the Personal account is in an Organization. </li>
        <li> Location Groups cannot own Location Groups. </li>
        </ul>"##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_create",
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
            ("delete-notifications",
                    Some(r##"Clears the pubsub notification settings for the account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_delete-notifications",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The resource name for the notification settings to be cleared."##),
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
            ("generate-account-number",
                    Some(r##"Generates an account number for this account. The account number is not
        provisioned when an account is created. Use this request to create an
        account number when it is required."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_generate-account-number",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the account to generate an account number for."##),
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
                    Some(r##"Gets the specified account. Returns `NOT_FOUND` if the
        account does not exist or if the caller does not have access rights to it."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the account to fetch."##),
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
            ("get-notifications",
                    Some(r##"Returns the pubsub notification settings for the account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_get-notifications",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The notification settings resource name."##),
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
            ("invitations-accept",
                    Some(r##"Accepts the specified invitation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_invitations-accept",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the invitation that is being accepted."##),
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
            ("invitations-decline",
                    Some(r##"Declines the specified invitation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_invitations-decline",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the account invitation that is being declined."##),
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
            ("invitations-list",
                    Some(r##"Lists pending invitations for the specified account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_invitations-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The name of the account from which the list of invitations is being
        retrieved."##),
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
                    Some(r##"Lists all of the accounts for the authenticated user. This includes all
        accounts that the user owns, as well as any accounts for which the user
        has management rights."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_list",
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
            ("list-recommend-google-locations",
                    Some(r##"List all the GoogleLocations that
        have been recommended to the specified GMB account.
        Recommendations are provided for personal accounts and location groups
        only, requests for all other account types will result in an error.
        The recommendations for location groups are based on the locations in that
        group.
        
        The recommendations for personal accounts are based on all of
        the locations that the user has access to on Google My Business (which
        includes locations they can access through location groups), and is a
        superset of all recommendations generated for the user."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_list-recommend-google-locations",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Name of the account resource to fetch recommended Google locations for."##),
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
            ("locations-admins-create",
                    Some(r##"Invites the specified user to become an administrator for the specified
        location. The invitee must accept the invitation in order to be granted
        access to the location. See AcceptInvitation to programmatically accept an
        invitation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-admins-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The resource name of the location this admin is created for."##),
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
            ("locations-admins-delete",
                    Some(r##"Removes the specified admin as a manager of the specified location."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-admins-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The resource name of the admin to remove from the location."##),
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
            ("locations-admins-list",
                    Some(r##"Lists all of the admins for the specified location."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-admins-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The name of the location to list admins of."##),
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
            ("locations-admins-patch",
                    Some(r##"Updates the Admin for the specified Location Admin. Only the AdminRole of
        the Admin can be updated."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-admins-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The resource name of the admin to update."##),
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
            ("locations-associate",
                    Some(r##"Associates a location to a place ID. Any previous association is
        overwritten. This operation is only valid if the location is unverified.
        The association must be valid, that is, it appears in the list of
        `FindMatchingLocations`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-associate",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The resource name of the location to associate."##),
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
            ("locations-batch-get",
                    Some(r##"Gets all of the specified locations in the given account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-batch-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the account from which to fetch locations."##),
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
            ("locations-batch-get-reviews",
                    Some(r##"Returns the paginated list of reviews for all specified locations.
        This operation is only valid if the specified locations are verified.
        <aside class="note"><b>Note:</b> Reviews are limited
        to a batch size of 200 `location_names` per call.</aside>"##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-batch-get-reviews",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the account from which to retrieve a list of reviews across
        multiple locations."##),
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
            ("locations-clear-association",
                    Some(r##"Clears an association between a location and its place ID. This
        operation is only valid if the location is unverified."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-clear-association",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The resource name of the location to disassociate."##),
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
            ("locations-create",
                    Some(r##"Creates a new location owned by the
        specified account, and returns it."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The name of the account in which to create this location."##),
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
            ("locations-delete",
                    Some(r##"Deletes a location.
        
        <aside class="note"><b>Note:</b> If this location cannot be deleted using
        the API as marked in the
        LocationState, use the [Google My
        Business](https://business.google.com/manage/) website.
        
        Returns `NOT_FOUND` if the location does not exist."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the location to delete."##),
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
            ("locations-fetch-verification-options",
                    Some(r##"Reports all eligible verification options for a location in a specific
        language."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-fetch-verification-options",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Resource name of the location to verify."##),
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
            ("locations-find-matches",
                    Some(r##"Finds all of the possible locations that are a match to the specified
        location. This operation is only valid if the location is unverified."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-find-matches",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The resource name of the location to find matches for."##),
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
            ("locations-followers-get-metadata",
                    Some(r##"Get the followers settings for a location.
        
        NOT_FOUND is returned if either the account or the location doesn't exist.
        PRECONDITION_FAILED is returned if the location is not verified nor
        connected to Maps."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-followers-get-metadata",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The resource name of the location's followers metadata.
        accounts/{account_id}/locations/{location_id}/followers/metadata"##),
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
            ("locations-get",
                    Some(r##"Gets the specified location. Returns `NOT_FOUND` if the
        location does not exist."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the location to fetch."##),
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
            ("locations-get-google-updated",
                    Some(r##"Gets the Google-updated version of the specified location.
        Returns `NOT_FOUND` if the location does not exist."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-get-google-updated",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the location to fetch."##),
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
            ("locations-list",
                    Some(r##"Lists the locations for the specified account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The name of the account to fetch locations from. If the Account is of AccountType PERSONAL, only Locations that are
        directly owned by the Account are returned, otherwise it will return all
        accessible locations from the Account, either directly or indirectly."##),
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
            ("locations-local-posts-create",
                    Some(r##"Creates a new local post associated with the specified location, and
        returns it."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-local-posts-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The name of the location in which to create this local post."##),
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
            ("locations-local-posts-delete",
                    Some(r##"Deletes a local post.
        Returns `NOT_FOUND` if the local post does not exist."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-local-posts-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the local post to delete."##),
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
            ("locations-local-posts-get",
                    Some(r##"Gets the specified local post.
        Returns `NOT_FOUND` if the local post does not exist."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-local-posts-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the local post to fetch."##),
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
            ("locations-local-posts-list",
                    Some(r##"Returns a list of local posts associated with a location."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-local-posts-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The name of the location whose local posts will be listed."##),
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
            ("locations-local-posts-patch",
                    Some(r##"Updates the specified local post and returns the updated local post."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-local-posts-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the local post to update."##),
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
            ("locations-local-posts-report-insights",
                    Some(r##"Returns insights for a set of local posts associated with a single listing.
        Which metrics and how they are reported are options specified in the
        request proto.
        <aside class="note"><b>Note:</b> Insight reports are limited
        to 100 `local_post_names` per call.</aside>"##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-local-posts-report-insights",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the location for which to fetch insights."##),
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
            ("locations-media-create",
                    Some(r##"Creates a new media item for the location."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-media-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The resource name of the location where this media item will be created."##),
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
            ("locations-media-customers-get",
                    Some(r##"Returns metadata for the requested customer media item."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-media-customers-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The resource name of the requested customer media item."##),
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
            ("locations-media-customers-list",
                    Some(r##"Returns a list of media items associated with a location that have been
        contributed by customers."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-media-customers-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The name of the location whose customer media items will be listed."##),
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
            ("locations-media-delete",
                    Some(r##"Deletes the specified media item."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-media-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the media item to be deleted."##),
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
            ("locations-media-get",
                    Some(r##"Returns metadata for the requested media item."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-media-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the requested media item."##),
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
            ("locations-media-list",
                    Some(r##"Returns a list of media items associated with a location."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-media-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The name of the location whose media items will be listed."##),
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
            ("locations-media-patch",
                    Some(r##"Updates metadata of the specified media item.
        This can only be used to update the Category of a media item,
        with the exception that the new category cannot be COVER or PROFILE."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-media-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the media item to be updated."##),
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
            ("locations-media-start-upload",
                    Some(r##"Generates a `MediaItemDataRef` for media item uploading."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-media-start-upload",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The resource name of the location this media item is to be added to."##),
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
            ("locations-patch",
                    Some(r##"Updates the specified location.
        
        Photos are only allowed on a location that has a Google+ page.
        
        Returns `NOT_FOUND` if the location does not exist."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the location to update."##),
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
            ("locations-questions-answers-delete",
                    Some(r##"Deletes the answer written by the current user to a question."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-questions-answers-delete",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The name of the question to delete an answer for."##),
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
            ("locations-questions-answers-list",
                    Some(r##"Returns the paginated list of answers for a specified question."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-questions-answers-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The name of the question to fetch answers for."##),
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
            ("locations-questions-answers-upsert",
                    Some(r##"Creates an answer or updates the existing answer written by the user for
        the specified question. A user can only create one answer per question."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-questions-answers-upsert",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The name of the question to write an answer for."##),
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
            ("locations-questions-create",
                    Some(r##"Adds a question for the specified location."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-questions-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The name of the location to write a question for."##),
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
            ("locations-questions-delete",
                    Some(r##"Deletes a specific question written by the current user."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-questions-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the question to delete."##),
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
            ("locations-questions-list",
                    Some(r##"Returns the paginated list of questions and some of its answers for a
        specified location."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-questions-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The name of the location to fetch questions for."##),
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
            ("locations-questions-patch",
                    Some(r##"Updates a specific question written by the current user."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-questions-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the question to update."##),
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
            ("locations-report-insights",
                    Some(r##"Returns a report containing insights on one or more metrics by location.
        
        <aside class="note"><b>Note:</b> Insight reports are limited
        to a batch size of 10 `location_names` per call.</aside>"##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-report-insights",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The account resource name."##),
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
            ("locations-reviews-delete-reply",
                    Some(r##"Deletes the response to the specified review.
        This operation is only valid if the specified location is verified."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-reviews-delete-reply",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the review reply to delete."##),
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
            ("locations-reviews-get",
                    Some(r##"Returns the specified review.
        This operation is only valid if the specified location is verified.
        Returns `NOT_FOUND` if the review does not exist, or has been deleted."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-reviews-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the review to fetch."##),
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
            ("locations-reviews-list",
                    Some(r##"Returns the paginated list of reviews for the specified location.
        This operation is only valid if the specified location is verified."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-reviews-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The name of the location to fetch reviews for."##),
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
            ("locations-reviews-update-reply",
                    Some(r##"Updates the reply to the specified review.
        A reply is created if one does not exist.
        This operation is only valid if the specified location is verified."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-reviews-update-reply",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the review to respond to."##),
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
            ("locations-transfer",
                    Some(r##"Moves a location from an account that the user owns to another account
        that the same user administers. The user must be an owner of the account
        the location is currently associated with and must also be at least a
        manager of the destination account. Returns the Location with its new
        resource name."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-transfer",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the location to transfer."##),
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
            ("locations-verifications-complete",
                    Some(r##"Completes a `PENDING` verification.
        
        It is only necessary for non `AUTO` verification methods. `AUTO`
        verification request is instantly `VERIFIED` upon creation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-verifications-complete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Resource name of the verification to complete."##),
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
            ("locations-verifications-list",
                    Some(r##"List verifications of a location, ordered by create time."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-verifications-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Resource name of the location that verification requests belong to."##),
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
            ("locations-verify",
                    Some(r##"Starts the verification process for a location."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_locations-verify",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Resource name of the location to verify."##),
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
                    Some(r##"Updates the specified business account. Personal accounts cannot be
        updated using this method.
        <aside class="note"><b>Note:</b> The only editable field for an account is
        `account_name`.
        Any other fields passed in (such as `type` or `role`) are
        ignored.</aside>"##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_update",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the account to update."##),
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
            ("update-notifications",
                    Some(r##"Sets the pubsub notification settings for the account informing My Business
        which topic to send pubsub notifications for:
        
        - New reviews for locations administered by the account.
        - Updated reviews for locations administered by the account.
        - New `GoogleUpdates` for locations administered by the account.
        
        An account will only have one notification settings resource, and only one
        pubsub topic can be set."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/accounts_update-notifications",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The notification settings resource name."##),
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
        
        ("attributes", "methods: 'list'", vec![
            ("list",
                    Some(r##"Returns the list of available attributes that would be available for a
        location with the given primary category and country."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/attributes_list",
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
        
        ("categories", "methods: 'list'", vec![
            ("list",
                    Some(r##"Returns a list of business categories. Search will match the category name
        but not the category ID.
        
        <aside class="note"><b>Note:</b> Search only matches the front of
        a category name (that is, 'food' may return 'Food Court' but not 'Fast Food
        Restaurant').</aside>"##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/categories_list",
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
        
        ("chains", "methods: 'get' and 'search'", vec![
            ("get",
                    Some(r##"Gets the specified chain. Returns `NOT_FOUND` if the
        chain does not exist."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/chains_get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The chain's resource name, in the format `chains/{chain_place_id}`."##),
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
                    Some(r##"Searches the chain based on chain name."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/chains_search",
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
        
        ("google-locations", "methods: 'report' and 'search'", vec![
            ("report",
                    Some(r##"Report a GoogleLocation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/google-locations_report",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Resource name of a [GoogleLocation], in the format
        `googleLocations/{googleLocationId}`."##),
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
                    Some(r##"Search all of the possible locations that are a match to the specified
        request."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/google-locations_search",
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
        
        ("verification-tokens", "methods: 'generate'", vec![
            ("generate",
                    Some(r##"Generates a token for the provided location data as a vetted partner.
        
        Throws PERMISSION_DENIED if the caller is not a vetted partner account.
        Throws FAILED_PRECONDITION if the caller's VettedStatus is INVALID."##),
                    "Details at http://byron.github.io/google-apis-rs/google_mybusiness4_cli/verification-tokens_generate",
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
    
    let mut app = App::new("mybusiness4")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+0")
           .about("The Google My Business API provides an interface for managing business location information on Google.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_mybusiness4_cli")
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
