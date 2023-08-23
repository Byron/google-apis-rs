// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_privateca1_beta1::{api, Error, oauth2, client::chrono, FieldMask};


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
    hub: api::CertificateAuthorityService<S>,
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
    async fn _projects_locations_certificate_authorities_activate(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "pem-ca-certificate" => Some(("pemCaCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-id" => Some(("requestId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "subordinate-config.certificate-authority" => Some(("subordinateConfig.certificateAuthority", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "subordinate-config.pem-issuer-chain.pem-certificates" => Some(("subordinateConfig.pemIssuerChain.pemCertificates", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["certificate-authority", "pem-ca-certificate", "pem-certificates", "pem-issuer-chain", "request-id", "subordinate-config"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ActivateCertificateAuthorityRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_certificate_authorities_activate(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_certificate_authorities_certificate_revocation_lists_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_certificate_authorities_certificate_revocation_lists_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_certificate_authorities_certificate_revocation_lists_get_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_certificate_authorities_certificate_revocation_lists_get_iam_policy(opt.value_of("resource").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "options-requested-policy-version" => {
                    call = call.options_requested_policy_version(        value.map(|v| arg_from_str(v, err, "options-requested-policy-version", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["options-requested-policy-version"].iter().map(|v|*v));
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

    async fn _projects_locations_certificate_authorities_certificate_revocation_lists_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_certificate_authorities_certificate_revocation_lists_list(opt.value_of("parent").unwrap_or(""));
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
                _ => {
                    let mut found = false;
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
                                                                           v.extend(["filter", "order-by", "page-size", "page-token"].iter().map(|v|*v));
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

    async fn _projects_locations_certificate_authorities_certificate_revocation_lists_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "access-url" => Some(("accessUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pem-crl" => Some(("pemCrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sequence-number" => Some(("sequenceNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["access-url", "create-time", "labels", "name", "pem-crl", "sequence-number", "state", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CertificateRevocationList = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_certificate_authorities_certificate_revocation_lists_patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
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
                                                                           v.extend(["request-id", "update-mask"].iter().map(|v|*v));
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

    async fn _projects_locations_certificate_authorities_certificate_revocation_lists_set_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "policy.etag" => Some(("policy.etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "policy.version" => Some(("policy.version", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update-mask" => Some(("updateMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["etag", "policy", "update-mask", "version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SetIamPolicyRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_certificate_authorities_certificate_revocation_lists_set_iam_policy(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_locations_certificate_authorities_certificate_revocation_lists_test_iam_permissions(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "permissions" => Some(("permissions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["permissions"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::TestIamPermissionsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_certificate_authorities_certificate_revocation_lists_test_iam_permissions(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_locations_certificate_authorities_certificates_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "certificate-description.aia-issuing-certificate-urls" => Some(("certificateDescription.aiaIssuingCertificateUrls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-description.authority-key-id.key-id" => Some(("certificateDescription.authorityKeyId.keyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.cert-fingerprint.sha256-hash" => Some(("certificateDescription.certFingerprint.sha256Hash", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.config-values.aia-ocsp-servers" => Some(("certificateDescription.configValues.aiaOcspServers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-description.config-values.ca-options.is-ca" => Some(("certificateDescription.configValues.caOptions.isCa", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.config-values.ca-options.max-issuer-path-length" => Some(("certificateDescription.configValues.caOptions.maxIssuerPathLength", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "certificate-description.config-values.key-usage.base-key-usage.cert-sign" => Some(("certificateDescription.configValues.keyUsage.baseKeyUsage.certSign", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.config-values.key-usage.base-key-usage.content-commitment" => Some(("certificateDescription.configValues.keyUsage.baseKeyUsage.contentCommitment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.config-values.key-usage.base-key-usage.crl-sign" => Some(("certificateDescription.configValues.keyUsage.baseKeyUsage.crlSign", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.config-values.key-usage.base-key-usage.data-encipherment" => Some(("certificateDescription.configValues.keyUsage.baseKeyUsage.dataEncipherment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.config-values.key-usage.base-key-usage.decipher-only" => Some(("certificateDescription.configValues.keyUsage.baseKeyUsage.decipherOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.config-values.key-usage.base-key-usage.digital-signature" => Some(("certificateDescription.configValues.keyUsage.baseKeyUsage.digitalSignature", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.config-values.key-usage.base-key-usage.encipher-only" => Some(("certificateDescription.configValues.keyUsage.baseKeyUsage.encipherOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.config-values.key-usage.base-key-usage.key-agreement" => Some(("certificateDescription.configValues.keyUsage.baseKeyUsage.keyAgreement", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.config-values.key-usage.base-key-usage.key-encipherment" => Some(("certificateDescription.configValues.keyUsage.baseKeyUsage.keyEncipherment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.config-values.key-usage.extended-key-usage.client-auth" => Some(("certificateDescription.configValues.keyUsage.extendedKeyUsage.clientAuth", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.config-values.key-usage.extended-key-usage.code-signing" => Some(("certificateDescription.configValues.keyUsage.extendedKeyUsage.codeSigning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.config-values.key-usage.extended-key-usage.email-protection" => Some(("certificateDescription.configValues.keyUsage.extendedKeyUsage.emailProtection", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.config-values.key-usage.extended-key-usage.ocsp-signing" => Some(("certificateDescription.configValues.keyUsage.extendedKeyUsage.ocspSigning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.config-values.key-usage.extended-key-usage.server-auth" => Some(("certificateDescription.configValues.keyUsage.extendedKeyUsage.serverAuth", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.config-values.key-usage.extended-key-usage.time-stamping" => Some(("certificateDescription.configValues.keyUsage.extendedKeyUsage.timeStamping", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.crl-distribution-points" => Some(("certificateDescription.crlDistributionPoints", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-description.public-key.key" => Some(("certificateDescription.publicKey.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.public-key.type" => Some(("certificateDescription.publicKey.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.subject-description.common-name" => Some(("certificateDescription.subjectDescription.commonName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.subject-description.hex-serial-number" => Some(("certificateDescription.subjectDescription.hexSerialNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.subject-description.lifetime" => Some(("certificateDescription.subjectDescription.lifetime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.subject-description.not-after-time" => Some(("certificateDescription.subjectDescription.notAfterTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.subject-description.not-before-time" => Some(("certificateDescription.subjectDescription.notBeforeTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.subject-description.subject.country-code" => Some(("certificateDescription.subjectDescription.subject.countryCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.subject-description.subject.locality" => Some(("certificateDescription.subjectDescription.subject.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.subject-description.subject.organization" => Some(("certificateDescription.subjectDescription.subject.organization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.subject-description.subject.organizational-unit" => Some(("certificateDescription.subjectDescription.subject.organizationalUnit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.subject-description.subject.postal-code" => Some(("certificateDescription.subjectDescription.subject.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.subject-description.subject.province" => Some(("certificateDescription.subjectDescription.subject.province", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.subject-description.subject.street-address" => Some(("certificateDescription.subjectDescription.subject.streetAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.subject-description.subject-alt-name.dns-names" => Some(("certificateDescription.subjectDescription.subjectAltName.dnsNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-description.subject-description.subject-alt-name.email-addresses" => Some(("certificateDescription.subjectDescription.subjectAltName.emailAddresses", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-description.subject-description.subject-alt-name.ip-addresses" => Some(("certificateDescription.subjectDescription.subjectAltName.ipAddresses", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-description.subject-description.subject-alt-name.uris" => Some(("certificateDescription.subjectDescription.subjectAltName.uris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-description.subject-key-id.key-id" => Some(("certificateDescription.subjectKeyId.keyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.public-key.key" => Some(("config.publicKey.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.public-key.type" => Some(("config.publicKey.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config" => Some(("config.reusableConfig.reusableConfig", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.aia-ocsp-servers" => Some(("config.reusableConfig.reusableConfigValues.aiaOcspServers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.reusable-config.reusable-config-values.ca-options.is-ca" => Some(("config.reusableConfig.reusableConfigValues.caOptions.isCa", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.ca-options.max-issuer-path-length" => Some(("config.reusableConfig.reusableConfigValues.caOptions.maxIssuerPathLength", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.base-key-usage.cert-sign" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.baseKeyUsage.certSign", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.base-key-usage.content-commitment" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.baseKeyUsage.contentCommitment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.base-key-usage.crl-sign" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.baseKeyUsage.crlSign", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.base-key-usage.data-encipherment" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.baseKeyUsage.dataEncipherment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.base-key-usage.decipher-only" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.baseKeyUsage.decipherOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.base-key-usage.digital-signature" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.baseKeyUsage.digitalSignature", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.base-key-usage.encipher-only" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.baseKeyUsage.encipherOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.base-key-usage.key-agreement" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.baseKeyUsage.keyAgreement", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.base-key-usage.key-encipherment" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.baseKeyUsage.keyEncipherment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.extended-key-usage.client-auth" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.extendedKeyUsage.clientAuth", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.extended-key-usage.code-signing" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.extendedKeyUsage.codeSigning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.extended-key-usage.email-protection" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.extendedKeyUsage.emailProtection", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.extended-key-usage.ocsp-signing" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.extendedKeyUsage.ocspSigning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.extended-key-usage.server-auth" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.extendedKeyUsage.serverAuth", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.extended-key-usage.time-stamping" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.extendedKeyUsage.timeStamping", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.subject-config.common-name" => Some(("config.subjectConfig.commonName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.subject-config.subject.country-code" => Some(("config.subjectConfig.subject.countryCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.subject-config.subject.locality" => Some(("config.subjectConfig.subject.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.subject-config.subject.organization" => Some(("config.subjectConfig.subject.organization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.subject-config.subject.organizational-unit" => Some(("config.subjectConfig.subject.organizationalUnit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.subject-config.subject.postal-code" => Some(("config.subjectConfig.subject.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.subject-config.subject.province" => Some(("config.subjectConfig.subject.province", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.subject-config.subject.street-address" => Some(("config.subjectConfig.subject.streetAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.subject-config.subject-alt-name.dns-names" => Some(("config.subjectConfig.subjectAltName.dnsNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.subject-config.subject-alt-name.email-addresses" => Some(("config.subjectConfig.subjectAltName.emailAddresses", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.subject-config.subject-alt-name.ip-addresses" => Some(("config.subjectConfig.subjectAltName.ipAddresses", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.subject-config.subject-alt-name.uris" => Some(("config.subjectConfig.subjectAltName.uris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "lifetime" => Some(("lifetime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pem-certificate" => Some(("pemCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pem-certificate-chain" => Some(("pemCertificateChain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "pem-csr" => Some(("pemCsr", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "revocation-details.revocation-state" => Some(("revocationDetails.revocationState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "revocation-details.revocation-time" => Some(("revocationDetails.revocationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["aia-issuing-certificate-urls", "aia-ocsp-servers", "authority-key-id", "base-key-usage", "ca-options", "cert-fingerprint", "cert-sign", "certificate-description", "client-auth", "code-signing", "common-name", "config", "config-values", "content-commitment", "country-code", "create-time", "crl-distribution-points", "crl-sign", "data-encipherment", "decipher-only", "digital-signature", "dns-names", "email-addresses", "email-protection", "encipher-only", "extended-key-usage", "hex-serial-number", "ip-addresses", "is-ca", "key", "key-agreement", "key-encipherment", "key-id", "key-usage", "labels", "lifetime", "locality", "max-issuer-path-length", "name", "not-after-time", "not-before-time", "ocsp-signing", "organization", "organizational-unit", "pem-certificate", "pem-certificate-chain", "pem-csr", "postal-code", "province", "public-key", "reusable-config", "reusable-config-values", "revocation-details", "revocation-state", "revocation-time", "server-auth", "sha256-hash", "street-address", "subject", "subject-alt-name", "subject-config", "subject-description", "subject-key-id", "time-stamping", "type", "update-time", "uris"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Certificate = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_certificate_authorities_certificates_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "request-id" => {
                    call = call.request_id(value.unwrap_or(""));
                },
                "certificate-id" => {
                    call = call.certificate_id(value.unwrap_or(""));
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
                                                                           v.extend(["certificate-id", "request-id"].iter().map(|v|*v));
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

    async fn _projects_locations_certificate_authorities_certificates_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_certificate_authorities_certificates_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_certificate_authorities_certificates_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_certificate_authorities_certificates_list(opt.value_of("parent").unwrap_or(""));
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
                _ => {
                    let mut found = false;
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
                                                                           v.extend(["filter", "order-by", "page-size", "page-token"].iter().map(|v|*v));
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

    async fn _projects_locations_certificate_authorities_certificates_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "certificate-description.aia-issuing-certificate-urls" => Some(("certificateDescription.aiaIssuingCertificateUrls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-description.authority-key-id.key-id" => Some(("certificateDescription.authorityKeyId.keyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.cert-fingerprint.sha256-hash" => Some(("certificateDescription.certFingerprint.sha256Hash", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.config-values.aia-ocsp-servers" => Some(("certificateDescription.configValues.aiaOcspServers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-description.config-values.ca-options.is-ca" => Some(("certificateDescription.configValues.caOptions.isCa", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.config-values.ca-options.max-issuer-path-length" => Some(("certificateDescription.configValues.caOptions.maxIssuerPathLength", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "certificate-description.config-values.key-usage.base-key-usage.cert-sign" => Some(("certificateDescription.configValues.keyUsage.baseKeyUsage.certSign", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.config-values.key-usage.base-key-usage.content-commitment" => Some(("certificateDescription.configValues.keyUsage.baseKeyUsage.contentCommitment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.config-values.key-usage.base-key-usage.crl-sign" => Some(("certificateDescription.configValues.keyUsage.baseKeyUsage.crlSign", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.config-values.key-usage.base-key-usage.data-encipherment" => Some(("certificateDescription.configValues.keyUsage.baseKeyUsage.dataEncipherment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.config-values.key-usage.base-key-usage.decipher-only" => Some(("certificateDescription.configValues.keyUsage.baseKeyUsage.decipherOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.config-values.key-usage.base-key-usage.digital-signature" => Some(("certificateDescription.configValues.keyUsage.baseKeyUsage.digitalSignature", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.config-values.key-usage.base-key-usage.encipher-only" => Some(("certificateDescription.configValues.keyUsage.baseKeyUsage.encipherOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.config-values.key-usage.base-key-usage.key-agreement" => Some(("certificateDescription.configValues.keyUsage.baseKeyUsage.keyAgreement", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.config-values.key-usage.base-key-usage.key-encipherment" => Some(("certificateDescription.configValues.keyUsage.baseKeyUsage.keyEncipherment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.config-values.key-usage.extended-key-usage.client-auth" => Some(("certificateDescription.configValues.keyUsage.extendedKeyUsage.clientAuth", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.config-values.key-usage.extended-key-usage.code-signing" => Some(("certificateDescription.configValues.keyUsage.extendedKeyUsage.codeSigning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.config-values.key-usage.extended-key-usage.email-protection" => Some(("certificateDescription.configValues.keyUsage.extendedKeyUsage.emailProtection", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.config-values.key-usage.extended-key-usage.ocsp-signing" => Some(("certificateDescription.configValues.keyUsage.extendedKeyUsage.ocspSigning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.config-values.key-usage.extended-key-usage.server-auth" => Some(("certificateDescription.configValues.keyUsage.extendedKeyUsage.serverAuth", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.config-values.key-usage.extended-key-usage.time-stamping" => Some(("certificateDescription.configValues.keyUsage.extendedKeyUsage.timeStamping", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.crl-distribution-points" => Some(("certificateDescription.crlDistributionPoints", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-description.public-key.key" => Some(("certificateDescription.publicKey.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.public-key.type" => Some(("certificateDescription.publicKey.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.subject-description.common-name" => Some(("certificateDescription.subjectDescription.commonName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.subject-description.hex-serial-number" => Some(("certificateDescription.subjectDescription.hexSerialNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.subject-description.lifetime" => Some(("certificateDescription.subjectDescription.lifetime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.subject-description.not-after-time" => Some(("certificateDescription.subjectDescription.notAfterTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.subject-description.not-before-time" => Some(("certificateDescription.subjectDescription.notBeforeTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.subject-description.subject.country-code" => Some(("certificateDescription.subjectDescription.subject.countryCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.subject-description.subject.locality" => Some(("certificateDescription.subjectDescription.subject.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.subject-description.subject.organization" => Some(("certificateDescription.subjectDescription.subject.organization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.subject-description.subject.organizational-unit" => Some(("certificateDescription.subjectDescription.subject.organizationalUnit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.subject-description.subject.postal-code" => Some(("certificateDescription.subjectDescription.subject.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.subject-description.subject.province" => Some(("certificateDescription.subjectDescription.subject.province", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.subject-description.subject.street-address" => Some(("certificateDescription.subjectDescription.subject.streetAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.subject-description.subject-alt-name.dns-names" => Some(("certificateDescription.subjectDescription.subjectAltName.dnsNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-description.subject-description.subject-alt-name.email-addresses" => Some(("certificateDescription.subjectDescription.subjectAltName.emailAddresses", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-description.subject-description.subject-alt-name.ip-addresses" => Some(("certificateDescription.subjectDescription.subjectAltName.ipAddresses", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-description.subject-description.subject-alt-name.uris" => Some(("certificateDescription.subjectDescription.subjectAltName.uris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-description.subject-key-id.key-id" => Some(("certificateDescription.subjectKeyId.keyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.public-key.key" => Some(("config.publicKey.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.public-key.type" => Some(("config.publicKey.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config" => Some(("config.reusableConfig.reusableConfig", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.aia-ocsp-servers" => Some(("config.reusableConfig.reusableConfigValues.aiaOcspServers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.reusable-config.reusable-config-values.ca-options.is-ca" => Some(("config.reusableConfig.reusableConfigValues.caOptions.isCa", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.ca-options.max-issuer-path-length" => Some(("config.reusableConfig.reusableConfigValues.caOptions.maxIssuerPathLength", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.base-key-usage.cert-sign" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.baseKeyUsage.certSign", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.base-key-usage.content-commitment" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.baseKeyUsage.contentCommitment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.base-key-usage.crl-sign" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.baseKeyUsage.crlSign", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.base-key-usage.data-encipherment" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.baseKeyUsage.dataEncipherment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.base-key-usage.decipher-only" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.baseKeyUsage.decipherOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.base-key-usage.digital-signature" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.baseKeyUsage.digitalSignature", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.base-key-usage.encipher-only" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.baseKeyUsage.encipherOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.base-key-usage.key-agreement" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.baseKeyUsage.keyAgreement", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.base-key-usage.key-encipherment" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.baseKeyUsage.keyEncipherment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.extended-key-usage.client-auth" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.extendedKeyUsage.clientAuth", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.extended-key-usage.code-signing" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.extendedKeyUsage.codeSigning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.extended-key-usage.email-protection" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.extendedKeyUsage.emailProtection", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.extended-key-usage.ocsp-signing" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.extendedKeyUsage.ocspSigning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.extended-key-usage.server-auth" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.extendedKeyUsage.serverAuth", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.extended-key-usage.time-stamping" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.extendedKeyUsage.timeStamping", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.subject-config.common-name" => Some(("config.subjectConfig.commonName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.subject-config.subject.country-code" => Some(("config.subjectConfig.subject.countryCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.subject-config.subject.locality" => Some(("config.subjectConfig.subject.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.subject-config.subject.organization" => Some(("config.subjectConfig.subject.organization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.subject-config.subject.organizational-unit" => Some(("config.subjectConfig.subject.organizationalUnit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.subject-config.subject.postal-code" => Some(("config.subjectConfig.subject.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.subject-config.subject.province" => Some(("config.subjectConfig.subject.province", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.subject-config.subject.street-address" => Some(("config.subjectConfig.subject.streetAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.subject-config.subject-alt-name.dns-names" => Some(("config.subjectConfig.subjectAltName.dnsNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.subject-config.subject-alt-name.email-addresses" => Some(("config.subjectConfig.subjectAltName.emailAddresses", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.subject-config.subject-alt-name.ip-addresses" => Some(("config.subjectConfig.subjectAltName.ipAddresses", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.subject-config.subject-alt-name.uris" => Some(("config.subjectConfig.subjectAltName.uris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "lifetime" => Some(("lifetime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pem-certificate" => Some(("pemCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pem-certificate-chain" => Some(("pemCertificateChain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "pem-csr" => Some(("pemCsr", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "revocation-details.revocation-state" => Some(("revocationDetails.revocationState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "revocation-details.revocation-time" => Some(("revocationDetails.revocationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["aia-issuing-certificate-urls", "aia-ocsp-servers", "authority-key-id", "base-key-usage", "ca-options", "cert-fingerprint", "cert-sign", "certificate-description", "client-auth", "code-signing", "common-name", "config", "config-values", "content-commitment", "country-code", "create-time", "crl-distribution-points", "crl-sign", "data-encipherment", "decipher-only", "digital-signature", "dns-names", "email-addresses", "email-protection", "encipher-only", "extended-key-usage", "hex-serial-number", "ip-addresses", "is-ca", "key", "key-agreement", "key-encipherment", "key-id", "key-usage", "labels", "lifetime", "locality", "max-issuer-path-length", "name", "not-after-time", "not-before-time", "ocsp-signing", "organization", "organizational-unit", "pem-certificate", "pem-certificate-chain", "pem-csr", "postal-code", "province", "public-key", "reusable-config", "reusable-config-values", "revocation-details", "revocation-state", "revocation-time", "server-auth", "sha256-hash", "street-address", "subject", "subject-alt-name", "subject-config", "subject-description", "subject-key-id", "time-stamping", "type", "update-time", "uris"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Certificate = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_certificate_authorities_certificates_patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
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
                                                                           v.extend(["request-id", "update-mask"].iter().map(|v|*v));
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

    async fn _projects_locations_certificate_authorities_certificates_revoke(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "reason" => Some(("reason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-id" => Some(("requestId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["reason", "request-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::RevokeCertificateRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_certificate_authorities_certificates_revoke(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_certificate_authorities_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "access-urls.ca-certificate-access-url" => Some(("accessUrls.caCertificateAccessUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "access-urls.crl-access-url" => Some(("accessUrls.crlAccessUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-policy.allowed-common-names" => Some(("certificatePolicy.allowedCommonNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-policy.allowed-issuance-modes.allow-config-based-issuance" => Some(("certificatePolicy.allowedIssuanceModes.allowConfigBasedIssuance", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-policy.allowed-issuance-modes.allow-csr-based-issuance" => Some(("certificatePolicy.allowedIssuanceModes.allowCsrBasedIssuance", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-policy.allowed-sans.allow-custom-sans" => Some(("certificatePolicy.allowedSans.allowCustomSans", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-policy.allowed-sans.allow-globbing-dns-wildcards" => Some(("certificatePolicy.allowedSans.allowGlobbingDnsWildcards", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-policy.allowed-sans.allowed-dns-names" => Some(("certificatePolicy.allowedSans.allowedDnsNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-policy.allowed-sans.allowed-email-addresses" => Some(("certificatePolicy.allowedSans.allowedEmailAddresses", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-policy.allowed-sans.allowed-ips" => Some(("certificatePolicy.allowedSans.allowedIps", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-policy.allowed-sans.allowed-uris" => Some(("certificatePolicy.allowedSans.allowedUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-policy.maximum-lifetime" => Some(("certificatePolicy.maximumLifetime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-policy.overwrite-config-values.reusable-config" => Some(("certificatePolicy.overwriteConfigValues.reusableConfig", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-policy.overwrite-config-values.reusable-config-values.aia-ocsp-servers" => Some(("certificatePolicy.overwriteConfigValues.reusableConfigValues.aiaOcspServers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-policy.overwrite-config-values.reusable-config-values.ca-options.is-ca" => Some(("certificatePolicy.overwriteConfigValues.reusableConfigValues.caOptions.isCa", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-policy.overwrite-config-values.reusable-config-values.ca-options.max-issuer-path-length" => Some(("certificatePolicy.overwriteConfigValues.reusableConfigValues.caOptions.maxIssuerPathLength", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "certificate-policy.overwrite-config-values.reusable-config-values.key-usage.base-key-usage.cert-sign" => Some(("certificatePolicy.overwriteConfigValues.reusableConfigValues.keyUsage.baseKeyUsage.certSign", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-policy.overwrite-config-values.reusable-config-values.key-usage.base-key-usage.content-commitment" => Some(("certificatePolicy.overwriteConfigValues.reusableConfigValues.keyUsage.baseKeyUsage.contentCommitment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-policy.overwrite-config-values.reusable-config-values.key-usage.base-key-usage.crl-sign" => Some(("certificatePolicy.overwriteConfigValues.reusableConfigValues.keyUsage.baseKeyUsage.crlSign", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-policy.overwrite-config-values.reusable-config-values.key-usage.base-key-usage.data-encipherment" => Some(("certificatePolicy.overwriteConfigValues.reusableConfigValues.keyUsage.baseKeyUsage.dataEncipherment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-policy.overwrite-config-values.reusable-config-values.key-usage.base-key-usage.decipher-only" => Some(("certificatePolicy.overwriteConfigValues.reusableConfigValues.keyUsage.baseKeyUsage.decipherOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-policy.overwrite-config-values.reusable-config-values.key-usage.base-key-usage.digital-signature" => Some(("certificatePolicy.overwriteConfigValues.reusableConfigValues.keyUsage.baseKeyUsage.digitalSignature", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-policy.overwrite-config-values.reusable-config-values.key-usage.base-key-usage.encipher-only" => Some(("certificatePolicy.overwriteConfigValues.reusableConfigValues.keyUsage.baseKeyUsage.encipherOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-policy.overwrite-config-values.reusable-config-values.key-usage.base-key-usage.key-agreement" => Some(("certificatePolicy.overwriteConfigValues.reusableConfigValues.keyUsage.baseKeyUsage.keyAgreement", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-policy.overwrite-config-values.reusable-config-values.key-usage.base-key-usage.key-encipherment" => Some(("certificatePolicy.overwriteConfigValues.reusableConfigValues.keyUsage.baseKeyUsage.keyEncipherment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-policy.overwrite-config-values.reusable-config-values.key-usage.extended-key-usage.client-auth" => Some(("certificatePolicy.overwriteConfigValues.reusableConfigValues.keyUsage.extendedKeyUsage.clientAuth", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-policy.overwrite-config-values.reusable-config-values.key-usage.extended-key-usage.code-signing" => Some(("certificatePolicy.overwriteConfigValues.reusableConfigValues.keyUsage.extendedKeyUsage.codeSigning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-policy.overwrite-config-values.reusable-config-values.key-usage.extended-key-usage.email-protection" => Some(("certificatePolicy.overwriteConfigValues.reusableConfigValues.keyUsage.extendedKeyUsage.emailProtection", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-policy.overwrite-config-values.reusable-config-values.key-usage.extended-key-usage.ocsp-signing" => Some(("certificatePolicy.overwriteConfigValues.reusableConfigValues.keyUsage.extendedKeyUsage.ocspSigning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-policy.overwrite-config-values.reusable-config-values.key-usage.extended-key-usage.server-auth" => Some(("certificatePolicy.overwriteConfigValues.reusableConfigValues.keyUsage.extendedKeyUsage.serverAuth", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-policy.overwrite-config-values.reusable-config-values.key-usage.extended-key-usage.time-stamping" => Some(("certificatePolicy.overwriteConfigValues.reusableConfigValues.keyUsage.extendedKeyUsage.timeStamping", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.public-key.key" => Some(("config.publicKey.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.public-key.type" => Some(("config.publicKey.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config" => Some(("config.reusableConfig.reusableConfig", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.aia-ocsp-servers" => Some(("config.reusableConfig.reusableConfigValues.aiaOcspServers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.reusable-config.reusable-config-values.ca-options.is-ca" => Some(("config.reusableConfig.reusableConfigValues.caOptions.isCa", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.ca-options.max-issuer-path-length" => Some(("config.reusableConfig.reusableConfigValues.caOptions.maxIssuerPathLength", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.base-key-usage.cert-sign" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.baseKeyUsage.certSign", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.base-key-usage.content-commitment" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.baseKeyUsage.contentCommitment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.base-key-usage.crl-sign" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.baseKeyUsage.crlSign", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.base-key-usage.data-encipherment" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.baseKeyUsage.dataEncipherment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.base-key-usage.decipher-only" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.baseKeyUsage.decipherOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.base-key-usage.digital-signature" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.baseKeyUsage.digitalSignature", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.base-key-usage.encipher-only" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.baseKeyUsage.encipherOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.base-key-usage.key-agreement" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.baseKeyUsage.keyAgreement", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.base-key-usage.key-encipherment" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.baseKeyUsage.keyEncipherment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.extended-key-usage.client-auth" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.extendedKeyUsage.clientAuth", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.extended-key-usage.code-signing" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.extendedKeyUsage.codeSigning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.extended-key-usage.email-protection" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.extendedKeyUsage.emailProtection", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.extended-key-usage.ocsp-signing" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.extendedKeyUsage.ocspSigning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.extended-key-usage.server-auth" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.extendedKeyUsage.serverAuth", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.extended-key-usage.time-stamping" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.extendedKeyUsage.timeStamping", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.subject-config.common-name" => Some(("config.subjectConfig.commonName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.subject-config.subject.country-code" => Some(("config.subjectConfig.subject.countryCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.subject-config.subject.locality" => Some(("config.subjectConfig.subject.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.subject-config.subject.organization" => Some(("config.subjectConfig.subject.organization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.subject-config.subject.organizational-unit" => Some(("config.subjectConfig.subject.organizationalUnit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.subject-config.subject.postal-code" => Some(("config.subjectConfig.subject.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.subject-config.subject.province" => Some(("config.subjectConfig.subject.province", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.subject-config.subject.street-address" => Some(("config.subjectConfig.subject.streetAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.subject-config.subject-alt-name.dns-names" => Some(("config.subjectConfig.subjectAltName.dnsNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.subject-config.subject-alt-name.email-addresses" => Some(("config.subjectConfig.subjectAltName.emailAddresses", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.subject-config.subject-alt-name.ip-addresses" => Some(("config.subjectConfig.subjectAltName.ipAddresses", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.subject-config.subject-alt-name.uris" => Some(("config.subjectConfig.subjectAltName.uris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "delete-time" => Some(("deleteTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gcs-bucket" => Some(("gcsBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "issuing-options.include-ca-cert-url" => Some(("issuingOptions.includeCaCertUrl", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuing-options.include-crl-access-url" => Some(("issuingOptions.includeCrlAccessUrl", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "key-spec.algorithm" => Some(("keySpec.algorithm", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "key-spec.cloud-kms-key-version" => Some(("keySpec.cloudKmsKeyVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "lifetime" => Some(("lifetime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pem-ca-certificates" => Some(("pemCaCertificates", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "subordinate-config.certificate-authority" => Some(("subordinateConfig.certificateAuthority", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "subordinate-config.pem-issuer-chain.pem-certificates" => Some(("subordinateConfig.pemIssuerChain.pemCertificates", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "tier" => Some(("tier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["access-urls", "aia-ocsp-servers", "algorithm", "allow-config-based-issuance", "allow-csr-based-issuance", "allow-custom-sans", "allow-globbing-dns-wildcards", "allowed-common-names", "allowed-dns-names", "allowed-email-addresses", "allowed-ips", "allowed-issuance-modes", "allowed-sans", "allowed-uris", "base-key-usage", "ca-certificate-access-url", "ca-options", "cert-sign", "certificate-authority", "certificate-policy", "client-auth", "cloud-kms-key-version", "code-signing", "common-name", "config", "content-commitment", "country-code", "create-time", "crl-access-url", "crl-sign", "data-encipherment", "decipher-only", "delete-time", "digital-signature", "dns-names", "email-addresses", "email-protection", "encipher-only", "extended-key-usage", "gcs-bucket", "include-ca-cert-url", "include-crl-access-url", "ip-addresses", "is-ca", "issuing-options", "key", "key-agreement", "key-encipherment", "key-spec", "key-usage", "labels", "lifetime", "locality", "max-issuer-path-length", "maximum-lifetime", "name", "ocsp-signing", "organization", "organizational-unit", "overwrite-config-values", "pem-ca-certificates", "pem-certificates", "pem-issuer-chain", "postal-code", "province", "public-key", "reusable-config", "reusable-config-values", "server-auth", "state", "street-address", "subject", "subject-alt-name", "subject-config", "subordinate-config", "tier", "time-stamping", "type", "update-time", "uris"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CertificateAuthority = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_certificate_authorities_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "request-id" => {
                    call = call.request_id(value.unwrap_or(""));
                },
                "certificate-authority-id" => {
                    call = call.certificate_authority_id(value.unwrap_or(""));
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
                                                                           v.extend(["certificate-authority-id", "request-id"].iter().map(|v|*v));
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

    async fn _projects_locations_certificate_authorities_disable(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "request-id" => Some(("requestId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["request-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::DisableCertificateAuthorityRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_certificate_authorities_disable(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_certificate_authorities_enable(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "request-id" => Some(("requestId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["request-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::EnableCertificateAuthorityRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_certificate_authorities_enable(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_certificate_authorities_fetch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_certificate_authorities_fetch(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_certificate_authorities_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_certificate_authorities_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_certificate_authorities_get_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_certificate_authorities_get_iam_policy(opt.value_of("resource").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "options-requested-policy-version" => {
                    call = call.options_requested_policy_version(        value.map(|v| arg_from_str(v, err, "options-requested-policy-version", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["options-requested-policy-version"].iter().map(|v|*v));
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

    async fn _projects_locations_certificate_authorities_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_certificate_authorities_list(opt.value_of("parent").unwrap_or(""));
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
                _ => {
                    let mut found = false;
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
                                                                           v.extend(["filter", "order-by", "page-size", "page-token"].iter().map(|v|*v));
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

    async fn _projects_locations_certificate_authorities_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "access-urls.ca-certificate-access-url" => Some(("accessUrls.caCertificateAccessUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "access-urls.crl-access-url" => Some(("accessUrls.crlAccessUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-policy.allowed-common-names" => Some(("certificatePolicy.allowedCommonNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-policy.allowed-issuance-modes.allow-config-based-issuance" => Some(("certificatePolicy.allowedIssuanceModes.allowConfigBasedIssuance", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-policy.allowed-issuance-modes.allow-csr-based-issuance" => Some(("certificatePolicy.allowedIssuanceModes.allowCsrBasedIssuance", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-policy.allowed-sans.allow-custom-sans" => Some(("certificatePolicy.allowedSans.allowCustomSans", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-policy.allowed-sans.allow-globbing-dns-wildcards" => Some(("certificatePolicy.allowedSans.allowGlobbingDnsWildcards", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-policy.allowed-sans.allowed-dns-names" => Some(("certificatePolicy.allowedSans.allowedDnsNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-policy.allowed-sans.allowed-email-addresses" => Some(("certificatePolicy.allowedSans.allowedEmailAddresses", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-policy.allowed-sans.allowed-ips" => Some(("certificatePolicy.allowedSans.allowedIps", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-policy.allowed-sans.allowed-uris" => Some(("certificatePolicy.allowedSans.allowedUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-policy.maximum-lifetime" => Some(("certificatePolicy.maximumLifetime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-policy.overwrite-config-values.reusable-config" => Some(("certificatePolicy.overwriteConfigValues.reusableConfig", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-policy.overwrite-config-values.reusable-config-values.aia-ocsp-servers" => Some(("certificatePolicy.overwriteConfigValues.reusableConfigValues.aiaOcspServers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-policy.overwrite-config-values.reusable-config-values.ca-options.is-ca" => Some(("certificatePolicy.overwriteConfigValues.reusableConfigValues.caOptions.isCa", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-policy.overwrite-config-values.reusable-config-values.ca-options.max-issuer-path-length" => Some(("certificatePolicy.overwriteConfigValues.reusableConfigValues.caOptions.maxIssuerPathLength", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "certificate-policy.overwrite-config-values.reusable-config-values.key-usage.base-key-usage.cert-sign" => Some(("certificatePolicy.overwriteConfigValues.reusableConfigValues.keyUsage.baseKeyUsage.certSign", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-policy.overwrite-config-values.reusable-config-values.key-usage.base-key-usage.content-commitment" => Some(("certificatePolicy.overwriteConfigValues.reusableConfigValues.keyUsage.baseKeyUsage.contentCommitment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-policy.overwrite-config-values.reusable-config-values.key-usage.base-key-usage.crl-sign" => Some(("certificatePolicy.overwriteConfigValues.reusableConfigValues.keyUsage.baseKeyUsage.crlSign", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-policy.overwrite-config-values.reusable-config-values.key-usage.base-key-usage.data-encipherment" => Some(("certificatePolicy.overwriteConfigValues.reusableConfigValues.keyUsage.baseKeyUsage.dataEncipherment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-policy.overwrite-config-values.reusable-config-values.key-usage.base-key-usage.decipher-only" => Some(("certificatePolicy.overwriteConfigValues.reusableConfigValues.keyUsage.baseKeyUsage.decipherOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-policy.overwrite-config-values.reusable-config-values.key-usage.base-key-usage.digital-signature" => Some(("certificatePolicy.overwriteConfigValues.reusableConfigValues.keyUsage.baseKeyUsage.digitalSignature", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-policy.overwrite-config-values.reusable-config-values.key-usage.base-key-usage.encipher-only" => Some(("certificatePolicy.overwriteConfigValues.reusableConfigValues.keyUsage.baseKeyUsage.encipherOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-policy.overwrite-config-values.reusable-config-values.key-usage.base-key-usage.key-agreement" => Some(("certificatePolicy.overwriteConfigValues.reusableConfigValues.keyUsage.baseKeyUsage.keyAgreement", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-policy.overwrite-config-values.reusable-config-values.key-usage.base-key-usage.key-encipherment" => Some(("certificatePolicy.overwriteConfigValues.reusableConfigValues.keyUsage.baseKeyUsage.keyEncipherment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-policy.overwrite-config-values.reusable-config-values.key-usage.extended-key-usage.client-auth" => Some(("certificatePolicy.overwriteConfigValues.reusableConfigValues.keyUsage.extendedKeyUsage.clientAuth", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-policy.overwrite-config-values.reusable-config-values.key-usage.extended-key-usage.code-signing" => Some(("certificatePolicy.overwriteConfigValues.reusableConfigValues.keyUsage.extendedKeyUsage.codeSigning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-policy.overwrite-config-values.reusable-config-values.key-usage.extended-key-usage.email-protection" => Some(("certificatePolicy.overwriteConfigValues.reusableConfigValues.keyUsage.extendedKeyUsage.emailProtection", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-policy.overwrite-config-values.reusable-config-values.key-usage.extended-key-usage.ocsp-signing" => Some(("certificatePolicy.overwriteConfigValues.reusableConfigValues.keyUsage.extendedKeyUsage.ocspSigning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-policy.overwrite-config-values.reusable-config-values.key-usage.extended-key-usage.server-auth" => Some(("certificatePolicy.overwriteConfigValues.reusableConfigValues.keyUsage.extendedKeyUsage.serverAuth", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-policy.overwrite-config-values.reusable-config-values.key-usage.extended-key-usage.time-stamping" => Some(("certificatePolicy.overwriteConfigValues.reusableConfigValues.keyUsage.extendedKeyUsage.timeStamping", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.public-key.key" => Some(("config.publicKey.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.public-key.type" => Some(("config.publicKey.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config" => Some(("config.reusableConfig.reusableConfig", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.aia-ocsp-servers" => Some(("config.reusableConfig.reusableConfigValues.aiaOcspServers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.reusable-config.reusable-config-values.ca-options.is-ca" => Some(("config.reusableConfig.reusableConfigValues.caOptions.isCa", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.ca-options.max-issuer-path-length" => Some(("config.reusableConfig.reusableConfigValues.caOptions.maxIssuerPathLength", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.base-key-usage.cert-sign" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.baseKeyUsage.certSign", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.base-key-usage.content-commitment" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.baseKeyUsage.contentCommitment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.base-key-usage.crl-sign" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.baseKeyUsage.crlSign", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.base-key-usage.data-encipherment" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.baseKeyUsage.dataEncipherment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.base-key-usage.decipher-only" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.baseKeyUsage.decipherOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.base-key-usage.digital-signature" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.baseKeyUsage.digitalSignature", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.base-key-usage.encipher-only" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.baseKeyUsage.encipherOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.base-key-usage.key-agreement" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.baseKeyUsage.keyAgreement", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.base-key-usage.key-encipherment" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.baseKeyUsage.keyEncipherment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.extended-key-usage.client-auth" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.extendedKeyUsage.clientAuth", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.extended-key-usage.code-signing" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.extendedKeyUsage.codeSigning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.extended-key-usage.email-protection" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.extendedKeyUsage.emailProtection", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.extended-key-usage.ocsp-signing" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.extendedKeyUsage.ocspSigning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.extended-key-usage.server-auth" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.extendedKeyUsage.serverAuth", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.reusable-config.reusable-config-values.key-usage.extended-key-usage.time-stamping" => Some(("config.reusableConfig.reusableConfigValues.keyUsage.extendedKeyUsage.timeStamping", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.subject-config.common-name" => Some(("config.subjectConfig.commonName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.subject-config.subject.country-code" => Some(("config.subjectConfig.subject.countryCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.subject-config.subject.locality" => Some(("config.subjectConfig.subject.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.subject-config.subject.organization" => Some(("config.subjectConfig.subject.organization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.subject-config.subject.organizational-unit" => Some(("config.subjectConfig.subject.organizationalUnit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.subject-config.subject.postal-code" => Some(("config.subjectConfig.subject.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.subject-config.subject.province" => Some(("config.subjectConfig.subject.province", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.subject-config.subject.street-address" => Some(("config.subjectConfig.subject.streetAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.subject-config.subject-alt-name.dns-names" => Some(("config.subjectConfig.subjectAltName.dnsNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.subject-config.subject-alt-name.email-addresses" => Some(("config.subjectConfig.subjectAltName.emailAddresses", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.subject-config.subject-alt-name.ip-addresses" => Some(("config.subjectConfig.subjectAltName.ipAddresses", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.subject-config.subject-alt-name.uris" => Some(("config.subjectConfig.subjectAltName.uris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "delete-time" => Some(("deleteTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gcs-bucket" => Some(("gcsBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "issuing-options.include-ca-cert-url" => Some(("issuingOptions.includeCaCertUrl", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuing-options.include-crl-access-url" => Some(("issuingOptions.includeCrlAccessUrl", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "key-spec.algorithm" => Some(("keySpec.algorithm", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "key-spec.cloud-kms-key-version" => Some(("keySpec.cloudKmsKeyVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "lifetime" => Some(("lifetime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pem-ca-certificates" => Some(("pemCaCertificates", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "subordinate-config.certificate-authority" => Some(("subordinateConfig.certificateAuthority", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "subordinate-config.pem-issuer-chain.pem-certificates" => Some(("subordinateConfig.pemIssuerChain.pemCertificates", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "tier" => Some(("tier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["access-urls", "aia-ocsp-servers", "algorithm", "allow-config-based-issuance", "allow-csr-based-issuance", "allow-custom-sans", "allow-globbing-dns-wildcards", "allowed-common-names", "allowed-dns-names", "allowed-email-addresses", "allowed-ips", "allowed-issuance-modes", "allowed-sans", "allowed-uris", "base-key-usage", "ca-certificate-access-url", "ca-options", "cert-sign", "certificate-authority", "certificate-policy", "client-auth", "cloud-kms-key-version", "code-signing", "common-name", "config", "content-commitment", "country-code", "create-time", "crl-access-url", "crl-sign", "data-encipherment", "decipher-only", "delete-time", "digital-signature", "dns-names", "email-addresses", "email-protection", "encipher-only", "extended-key-usage", "gcs-bucket", "include-ca-cert-url", "include-crl-access-url", "ip-addresses", "is-ca", "issuing-options", "key", "key-agreement", "key-encipherment", "key-spec", "key-usage", "labels", "lifetime", "locality", "max-issuer-path-length", "maximum-lifetime", "name", "ocsp-signing", "organization", "organizational-unit", "overwrite-config-values", "pem-ca-certificates", "pem-certificates", "pem-issuer-chain", "postal-code", "province", "public-key", "reusable-config", "reusable-config-values", "server-auth", "state", "street-address", "subject", "subject-alt-name", "subject-config", "subordinate-config", "tier", "time-stamping", "type", "update-time", "uris"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CertificateAuthority = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_certificate_authorities_patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
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
                                                                           v.extend(["request-id", "update-mask"].iter().map(|v|*v));
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

    async fn _projects_locations_certificate_authorities_restore(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "request-id" => Some(("requestId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["request-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::RestoreCertificateAuthorityRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_certificate_authorities_restore(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_certificate_authorities_schedule_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "ignore-active-certificates" => Some(("ignoreActiveCertificates", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "request-id" => Some(("requestId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["ignore-active-certificates", "request-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ScheduleDeleteCertificateAuthorityRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_certificate_authorities_schedule_delete(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_certificate_authorities_set_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "policy.etag" => Some(("policy.etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "policy.version" => Some(("policy.version", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update-mask" => Some(("updateMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["etag", "policy", "update-mask", "version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SetIamPolicyRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_certificate_authorities_set_iam_policy(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_locations_certificate_authorities_test_iam_permissions(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "permissions" => Some(("permissions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["permissions"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::TestIamPermissionsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_certificate_authorities_test_iam_permissions(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_locations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_list(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["filter", "page-size", "page-token"].iter().map(|v|*v));
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

    async fn _projects_locations_operations_cancel(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::CancelOperationRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_operations_cancel(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_operations_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_operations_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_operations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_operations_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_operations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_operations_list(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["filter", "page-size", "page-token"].iter().map(|v|*v));
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

    async fn _projects_locations_reusable_configs_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_reusable_configs_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_reusable_configs_get_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_reusable_configs_get_iam_policy(opt.value_of("resource").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "options-requested-policy-version" => {
                    call = call.options_requested_policy_version(        value.map(|v| arg_from_str(v, err, "options-requested-policy-version", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["options-requested-policy-version"].iter().map(|v|*v));
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

    async fn _projects_locations_reusable_configs_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_reusable_configs_list(opt.value_of("parent").unwrap_or(""));
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
                _ => {
                    let mut found = false;
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
                                                                           v.extend(["filter", "order-by", "page-size", "page-token"].iter().map(|v|*v));
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

    async fn _projects_locations_reusable_configs_set_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "policy.etag" => Some(("policy.etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "policy.version" => Some(("policy.version", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update-mask" => Some(("updateMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["etag", "policy", "update-mask", "version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SetIamPolicyRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_reusable_configs_set_iam_policy(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_locations_reusable_configs_test_iam_permissions(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "permissions" => Some(("permissions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["permissions"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::TestIamPermissionsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_reusable_configs_test_iam_permissions(request, opt.value_of("resource").unwrap_or(""));
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
            ("projects", Some(opt)) => {
                match opt.subcommand() {
                    ("locations-certificate-authorities-activate", Some(opt)) => {
                        call_result = self._projects_locations_certificate_authorities_activate(opt, dry_run, &mut err).await;
                    },
                    ("locations-certificate-authorities-certificate-revocation-lists-get", Some(opt)) => {
                        call_result = self._projects_locations_certificate_authorities_certificate_revocation_lists_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-certificate-authorities-certificate-revocation-lists-get-iam-policy", Some(opt)) => {
                        call_result = self._projects_locations_certificate_authorities_certificate_revocation_lists_get_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("locations-certificate-authorities-certificate-revocation-lists-list", Some(opt)) => {
                        call_result = self._projects_locations_certificate_authorities_certificate_revocation_lists_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-certificate-authorities-certificate-revocation-lists-patch", Some(opt)) => {
                        call_result = self._projects_locations_certificate_authorities_certificate_revocation_lists_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-certificate-authorities-certificate-revocation-lists-set-iam-policy", Some(opt)) => {
                        call_result = self._projects_locations_certificate_authorities_certificate_revocation_lists_set_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("locations-certificate-authorities-certificate-revocation-lists-test-iam-permissions", Some(opt)) => {
                        call_result = self._projects_locations_certificate_authorities_certificate_revocation_lists_test_iam_permissions(opt, dry_run, &mut err).await;
                    },
                    ("locations-certificate-authorities-certificates-create", Some(opt)) => {
                        call_result = self._projects_locations_certificate_authorities_certificates_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-certificate-authorities-certificates-get", Some(opt)) => {
                        call_result = self._projects_locations_certificate_authorities_certificates_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-certificate-authorities-certificates-list", Some(opt)) => {
                        call_result = self._projects_locations_certificate_authorities_certificates_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-certificate-authorities-certificates-patch", Some(opt)) => {
                        call_result = self._projects_locations_certificate_authorities_certificates_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-certificate-authorities-certificates-revoke", Some(opt)) => {
                        call_result = self._projects_locations_certificate_authorities_certificates_revoke(opt, dry_run, &mut err).await;
                    },
                    ("locations-certificate-authorities-create", Some(opt)) => {
                        call_result = self._projects_locations_certificate_authorities_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-certificate-authorities-disable", Some(opt)) => {
                        call_result = self._projects_locations_certificate_authorities_disable(opt, dry_run, &mut err).await;
                    },
                    ("locations-certificate-authorities-enable", Some(opt)) => {
                        call_result = self._projects_locations_certificate_authorities_enable(opt, dry_run, &mut err).await;
                    },
                    ("locations-certificate-authorities-fetch", Some(opt)) => {
                        call_result = self._projects_locations_certificate_authorities_fetch(opt, dry_run, &mut err).await;
                    },
                    ("locations-certificate-authorities-get", Some(opt)) => {
                        call_result = self._projects_locations_certificate_authorities_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-certificate-authorities-get-iam-policy", Some(opt)) => {
                        call_result = self._projects_locations_certificate_authorities_get_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("locations-certificate-authorities-list", Some(opt)) => {
                        call_result = self._projects_locations_certificate_authorities_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-certificate-authorities-patch", Some(opt)) => {
                        call_result = self._projects_locations_certificate_authorities_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-certificate-authorities-restore", Some(opt)) => {
                        call_result = self._projects_locations_certificate_authorities_restore(opt, dry_run, &mut err).await;
                    },
                    ("locations-certificate-authorities-schedule-delete", Some(opt)) => {
                        call_result = self._projects_locations_certificate_authorities_schedule_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-certificate-authorities-set-iam-policy", Some(opt)) => {
                        call_result = self._projects_locations_certificate_authorities_set_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("locations-certificate-authorities-test-iam-permissions", Some(opt)) => {
                        call_result = self._projects_locations_certificate_authorities_test_iam_permissions(opt, dry_run, &mut err).await;
                    },
                    ("locations-get", Some(opt)) => {
                        call_result = self._projects_locations_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-list", Some(opt)) => {
                        call_result = self._projects_locations_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-operations-cancel", Some(opt)) => {
                        call_result = self._projects_locations_operations_cancel(opt, dry_run, &mut err).await;
                    },
                    ("locations-operations-delete", Some(opt)) => {
                        call_result = self._projects_locations_operations_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-operations-get", Some(opt)) => {
                        call_result = self._projects_locations_operations_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-operations-list", Some(opt)) => {
                        call_result = self._projects_locations_operations_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-reusable-configs-get", Some(opt)) => {
                        call_result = self._projects_locations_reusable_configs_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-reusable-configs-get-iam-policy", Some(opt)) => {
                        call_result = self._projects_locations_reusable_configs_get_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("locations-reusable-configs-list", Some(opt)) => {
                        call_result = self._projects_locations_reusable_configs_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-reusable-configs-set-iam-policy", Some(opt)) => {
                        call_result = self._projects_locations_reusable_configs_set_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("locations-reusable-configs-test-iam-permissions", Some(opt)) => {
                        call_result = self._projects_locations_reusable_configs_test_iam_permissions(opt, dry_run, &mut err).await;
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
    async fn new(opt: ArgMatches<'n>, connector: S) -> Result<Engine<'n, S>, InvalidOptionsError> {
        let (config_dir, secret) = {
            let config_dir = match client::assure_config_dir_exists(opt.value_of("folder").unwrap_or("~/.google-service-cli")) {
                Err(e) => return Err(InvalidOptionsError::single(e, 3)),
                Ok(p) => p,
            };

            match client::application_secret_from_directory(&config_dir, "privateca1-beta1-secret.json",
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
        ).persist_tokens_to_disk(format!("{}/privateca1-beta1", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::CertificateAuthorityService::new(client, auth),
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
        ("projects", "methods: 'locations-certificate-authorities-activate', 'locations-certificate-authorities-certificate-revocation-lists-get', 'locations-certificate-authorities-certificate-revocation-lists-get-iam-policy', 'locations-certificate-authorities-certificate-revocation-lists-list', 'locations-certificate-authorities-certificate-revocation-lists-patch', 'locations-certificate-authorities-certificate-revocation-lists-set-iam-policy', 'locations-certificate-authorities-certificate-revocation-lists-test-iam-permissions', 'locations-certificate-authorities-certificates-create', 'locations-certificate-authorities-certificates-get', 'locations-certificate-authorities-certificates-list', 'locations-certificate-authorities-certificates-patch', 'locations-certificate-authorities-certificates-revoke', 'locations-certificate-authorities-create', 'locations-certificate-authorities-disable', 'locations-certificate-authorities-enable', 'locations-certificate-authorities-fetch', 'locations-certificate-authorities-get', 'locations-certificate-authorities-get-iam-policy', 'locations-certificate-authorities-list', 'locations-certificate-authorities-patch', 'locations-certificate-authorities-restore', 'locations-certificate-authorities-schedule-delete', 'locations-certificate-authorities-set-iam-policy', 'locations-certificate-authorities-test-iam-permissions', 'locations-get', 'locations-list', 'locations-operations-cancel', 'locations-operations-delete', 'locations-operations-get', 'locations-operations-list', 'locations-reusable-configs-get', 'locations-reusable-configs-get-iam-policy', 'locations-reusable-configs-list', 'locations-reusable-configs-set-iam-policy' and 'locations-reusable-configs-test-iam-permissions'", vec![
            ("locations-certificate-authorities-activate",
                    Some(r##"Activate a CertificateAuthority that is in state PENDING_ACTIVATION and is of type SUBORDINATE. After the parent Certificate Authority signs a certificate signing request from FetchCertificateAuthorityCsr, this method can complete the activation process."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_beta1_cli/projects_locations-certificate-authorities-activate",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name for this CertificateAuthority in the format `projects/*/locations/*/certificateAuthorities/*`."##),
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
            ("locations-certificate-authorities-certificate-revocation-lists-get",
                    Some(r##"Returns a CertificateRevocationList."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_beta1_cli/projects_locations-certificate-authorities-certificate-revocation-lists-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the CertificateRevocationList to get."##),
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
            ("locations-certificate-authorities-certificate-revocation-lists-get-iam-policy",
                    Some(r##"Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_beta1_cli/projects_locations-certificate-authorities-certificate-revocation-lists-get-iam-policy",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
            ("locations-certificate-authorities-certificate-revocation-lists-list",
                    Some(r##"Lists CertificateRevocationLists."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_beta1_cli/projects_locations-certificate-authorities-certificate-revocation-lists-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The resource name of the location associated with the CertificateRevocationLists, in the format `projects/*/locations/*/certificateauthorities/*`."##),
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
            ("locations-certificate-authorities-certificate-revocation-lists-patch",
                    Some(r##"Update a CertificateRevocationList."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_beta1_cli/projects_locations-certificate-authorities-certificate-revocation-lists-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Output only. The resource path for this CertificateRevocationList in the format `projects/*/locations/*/certificateAuthorities/*/ certificateRevocationLists/*`."##),
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
            ("locations-certificate-authorities-certificate-revocation-lists-set-iam-policy",
                    Some(r##"Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_beta1_cli/projects_locations-certificate-authorities-certificate-revocation-lists-set-iam-policy",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
            ("locations-certificate-authorities-certificate-revocation-lists-test-iam-permissions",
                    Some(r##"Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_beta1_cli/projects_locations-certificate-authorities-certificate-revocation-lists-test-iam-permissions",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
            ("locations-certificate-authorities-certificates-create",
                    Some(r##"Create a new Certificate in a given Project, Location from a particular CertificateAuthority."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_beta1_cli/projects_locations-certificate-authorities-certificates-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The resource name of the location and CertificateAuthority associated with the Certificate, in the format `projects/*/locations/*/certificateAuthorities/*`."##),
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
            ("locations-certificate-authorities-certificates-get",
                    Some(r##"Returns a Certificate."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_beta1_cli/projects_locations-certificate-authorities-certificates-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the Certificate to get."##),
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
            ("locations-certificate-authorities-certificates-list",
                    Some(r##"Lists Certificates."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_beta1_cli/projects_locations-certificate-authorities-certificates-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The resource name of the location associated with the Certificates, in the format `projects/*/locations/*/certificateauthorities/*`."##),
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
            ("locations-certificate-authorities-certificates-patch",
                    Some(r##"Update a Certificate. Currently, the only field you can update is the labels field."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_beta1_cli/projects_locations-certificate-authorities-certificates-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Output only. The resource path for this Certificate in the format `projects/*/locations/*/certificateAuthorities/*/certificates/*`."##),
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
            ("locations-certificate-authorities-certificates-revoke",
                    Some(r##"Revoke a Certificate."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_beta1_cli/projects_locations-certificate-authorities-certificates-revoke",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name for this Certificate in the format `projects/*/locations/*/certificateAuthorities/*/certificates/*`."##),
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
            ("locations-certificate-authorities-create",
                    Some(r##"Create a new CertificateAuthority in a given Project and Location."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_beta1_cli/projects_locations-certificate-authorities-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The resource name of the location associated with the CertificateAuthorities, in the format `projects/*/locations/*`."##),
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
            ("locations-certificate-authorities-disable",
                    Some(r##"Disable a CertificateAuthority."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_beta1_cli/projects_locations-certificate-authorities-disable",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name for this CertificateAuthority in the format `projects/*/locations/*/certificateAuthorities/*`."##),
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
            ("locations-certificate-authorities-enable",
                    Some(r##"Enable a CertificateAuthority."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_beta1_cli/projects_locations-certificate-authorities-enable",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name for this CertificateAuthority in the format `projects/*/locations/*/certificateAuthorities/*`."##),
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
            ("locations-certificate-authorities-fetch",
                    Some(r##"Fetch a certificate signing request (CSR) from a CertificateAuthority that is in state PENDING_ACTIVATION and is of type SUBORDINATE. The CSR must then be signed by the desired parent Certificate Authority, which could be another CertificateAuthority resource, or could be an on-prem certificate authority. See also ActivateCertificateAuthority."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_beta1_cli/projects_locations-certificate-authorities-fetch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name for this CertificateAuthority in the format `projects/*/locations/*/certificateAuthorities/*`."##),
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
            ("locations-certificate-authorities-get",
                    Some(r##"Returns a CertificateAuthority."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_beta1_cli/projects_locations-certificate-authorities-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the CertificateAuthority to get."##),
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
            ("locations-certificate-authorities-get-iam-policy",
                    Some(r##"Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_beta1_cli/projects_locations-certificate-authorities-get-iam-policy",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
            ("locations-certificate-authorities-list",
                    Some(r##"Lists CertificateAuthorities."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_beta1_cli/projects_locations-certificate-authorities-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The resource name of the location associated with the CertificateAuthorities, in the format `projects/*/locations/*`."##),
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
            ("locations-certificate-authorities-patch",
                    Some(r##"Update a CertificateAuthority."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_beta1_cli/projects_locations-certificate-authorities-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Output only. The resource name for this CertificateAuthority in the format `projects/*/locations/*/certificateAuthorities/*`."##),
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
            ("locations-certificate-authorities-restore",
                    Some(r##"Restore a CertificateAuthority that is scheduled for deletion."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_beta1_cli/projects_locations-certificate-authorities-restore",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name for this CertificateAuthority in the format `projects/*/locations/*/certificateAuthorities/*`."##),
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
            ("locations-certificate-authorities-schedule-delete",
                    Some(r##"Schedule a CertificateAuthority for deletion."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_beta1_cli/projects_locations-certificate-authorities-schedule-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name for this CertificateAuthority in the format `projects/*/locations/*/certificateAuthorities/*`."##),
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
            ("locations-certificate-authorities-set-iam-policy",
                    Some(r##"Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_beta1_cli/projects_locations-certificate-authorities-set-iam-policy",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
            ("locations-certificate-authorities-test-iam-permissions",
                    Some(r##"Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_beta1_cli/projects_locations-certificate-authorities-test-iam-permissions",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
            ("locations-get",
                    Some(r##"Gets information about a location."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_beta1_cli/projects_locations-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Resource name for the location."##),
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
                    Some(r##"Lists information about the supported locations for this service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_beta1_cli/projects_locations-list",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The resource that owns the locations collection, if applicable."##),
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
            ("locations-operations-cancel",
                    Some(r##"Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_beta1_cli/projects_locations-operations-cancel",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the operation resource to be cancelled."##),
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
            ("locations-operations-delete",
                    Some(r##"Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_beta1_cli/projects_locations-operations-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the operation resource to be deleted."##),
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
            ("locations-operations-get",
                    Some(r##"Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_beta1_cli/projects_locations-operations-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the operation resource."##),
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
            ("locations-operations-list",
                    Some(r##"Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `"/v1/{name=users/*}/operations"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_beta1_cli/projects_locations-operations-list",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the operation's parent resource."##),
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
            ("locations-reusable-configs-get",
                    Some(r##"Returns a ReusableConfig."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_beta1_cli/projects_locations-reusable-configs-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the ReusableConfigs to get."##),
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
            ("locations-reusable-configs-get-iam-policy",
                    Some(r##"Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_beta1_cli/projects_locations-reusable-configs-get-iam-policy",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
            ("locations-reusable-configs-list",
                    Some(r##"Lists ReusableConfigs."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_beta1_cli/projects_locations-reusable-configs-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The resource name of the location associated with the ReusableConfigs, in the format `projects/*/locations/*`."##),
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
            ("locations-reusable-configs-set-iam-policy",
                    Some(r##"Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_beta1_cli/projects_locations-reusable-configs-set-iam-policy",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
            ("locations-reusable-configs-test-iam-permissions",
                    Some(r##"Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_beta1_cli/projects_locations-reusable-configs-test-iam-permissions",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
    
    let mut app = App::new("privateca1-beta1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.3+20230105")
           .about("The Certificate Authority Service API is a highly-available, scalable service that enables you to simplify and automate the management of private certificate authorities (CAs) while staying in control of your private keys. ")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_privateca1_beta1_cli")
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
