// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_privateca1::{api, Error, oauth2, client::chrono, FieldMask};


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
    async fn _projects_locations_ca_pools_certificate_authorities_activate(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.projects().locations_ca_pools_certificate_authorities_activate(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_ca_pools_certificate_authorities_certificate_revocation_lists_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_ca_pools_certificate_authorities_certificate_revocation_lists_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_ca_pools_certificate_authorities_certificate_revocation_lists_get_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_ca_pools_certificate_authorities_certificate_revocation_lists_get_iam_policy(opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_locations_ca_pools_certificate_authorities_certificate_revocation_lists_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_ca_pools_certificate_authorities_certificate_revocation_lists_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_ca_pools_certificate_authorities_certificate_revocation_lists_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "revision-id" => Some(("revisionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sequence-number" => Some(("sequenceNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["access-url", "create-time", "labels", "name", "pem-crl", "revision-id", "sequence-number", "state", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CertificateRevocationList = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_ca_pools_certificate_authorities_certificate_revocation_lists_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_ca_pools_certificate_authorities_certificate_revocation_lists_set_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.projects().locations_ca_pools_certificate_authorities_certificate_revocation_lists_set_iam_policy(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_locations_ca_pools_certificate_authorities_certificate_revocation_lists_test_iam_permissions(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.projects().locations_ca_pools_certificate_authorities_certificate_revocation_lists_test_iam_permissions(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_locations_ca_pools_certificate_authorities_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "access-urls.crl-access-urls" => Some(("accessUrls.crlAccessUrls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.public-key.format" => Some(("config.publicKey.format", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.public-key.key" => Some(("config.publicKey.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.subject-config.subject.common-name" => Some(("config.subjectConfig.subject.commonName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                    "config.subject-key-id.key-id" => Some(("config.subjectKeyId.keyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.x509-config.aia-ocsp-servers" => Some(("config.x509Config.aiaOcspServers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.x509-config.ca-options.is-ca" => Some(("config.x509Config.caOptions.isCa", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.ca-options.max-issuer-path-length" => Some(("config.x509Config.caOptions.maxIssuerPathLength", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.base-key-usage.cert-sign" => Some(("config.x509Config.keyUsage.baseKeyUsage.certSign", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.base-key-usage.content-commitment" => Some(("config.x509Config.keyUsage.baseKeyUsage.contentCommitment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.base-key-usage.crl-sign" => Some(("config.x509Config.keyUsage.baseKeyUsage.crlSign", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.base-key-usage.data-encipherment" => Some(("config.x509Config.keyUsage.baseKeyUsage.dataEncipherment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.base-key-usage.decipher-only" => Some(("config.x509Config.keyUsage.baseKeyUsage.decipherOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.base-key-usage.digital-signature" => Some(("config.x509Config.keyUsage.baseKeyUsage.digitalSignature", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.base-key-usage.encipher-only" => Some(("config.x509Config.keyUsage.baseKeyUsage.encipherOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.base-key-usage.key-agreement" => Some(("config.x509Config.keyUsage.baseKeyUsage.keyAgreement", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.base-key-usage.key-encipherment" => Some(("config.x509Config.keyUsage.baseKeyUsage.keyEncipherment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.extended-key-usage.client-auth" => Some(("config.x509Config.keyUsage.extendedKeyUsage.clientAuth", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.extended-key-usage.code-signing" => Some(("config.x509Config.keyUsage.extendedKeyUsage.codeSigning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.extended-key-usage.email-protection" => Some(("config.x509Config.keyUsage.extendedKeyUsage.emailProtection", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.extended-key-usage.ocsp-signing" => Some(("config.x509Config.keyUsage.extendedKeyUsage.ocspSigning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.extended-key-usage.server-auth" => Some(("config.x509Config.keyUsage.extendedKeyUsage.serverAuth", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.extended-key-usage.time-stamping" => Some(("config.x509Config.keyUsage.extendedKeyUsage.timeStamping", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.name-constraints.critical" => Some(("config.x509Config.nameConstraints.critical", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.name-constraints.excluded-dns-names" => Some(("config.x509Config.nameConstraints.excludedDnsNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.x509-config.name-constraints.excluded-email-addresses" => Some(("config.x509Config.nameConstraints.excludedEmailAddresses", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.x509-config.name-constraints.excluded-ip-ranges" => Some(("config.x509Config.nameConstraints.excludedIpRanges", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.x509-config.name-constraints.excluded-uris" => Some(("config.x509Config.nameConstraints.excludedUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.x509-config.name-constraints.permitted-dns-names" => Some(("config.x509Config.nameConstraints.permittedDnsNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.x509-config.name-constraints.permitted-email-addresses" => Some(("config.x509Config.nameConstraints.permittedEmailAddresses", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.x509-config.name-constraints.permitted-ip-ranges" => Some(("config.x509Config.nameConstraints.permittedIpRanges", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.x509-config.name-constraints.permitted-uris" => Some(("config.x509Config.nameConstraints.permittedUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "delete-time" => Some(("deleteTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "expire-time" => Some(("expireTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gcs-bucket" => Some(("gcsBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "key-spec.algorithm" => Some(("keySpec.algorithm", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "key-spec.cloud-kms-key-version" => Some(("keySpec.cloudKmsKeyVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "lifetime" => Some(("lifetime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pem-ca-certificates" => Some(("pemCaCertificates", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "satisfies-pzi" => Some(("satisfiesPzi", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "satisfies-pzs" => Some(("satisfiesPzs", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "subordinate-config.certificate-authority" => Some(("subordinateConfig.certificateAuthority", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "subordinate-config.pem-issuer-chain.pem-certificates" => Some(("subordinateConfig.pemIssuerChain.pemCertificates", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "tier" => Some(("tier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["access-urls", "aia-ocsp-servers", "algorithm", "base-key-usage", "ca-certificate-access-url", "ca-options", "cert-sign", "certificate-authority", "client-auth", "cloud-kms-key-version", "code-signing", "common-name", "config", "content-commitment", "country-code", "create-time", "critical", "crl-access-urls", "crl-sign", "data-encipherment", "decipher-only", "delete-time", "digital-signature", "dns-names", "email-addresses", "email-protection", "encipher-only", "excluded-dns-names", "excluded-email-addresses", "excluded-ip-ranges", "excluded-uris", "expire-time", "extended-key-usage", "format", "gcs-bucket", "ip-addresses", "is-ca", "key", "key-agreement", "key-encipherment", "key-id", "key-spec", "key-usage", "labels", "lifetime", "locality", "max-issuer-path-length", "name", "name-constraints", "ocsp-signing", "organization", "organizational-unit", "pem-ca-certificates", "pem-certificates", "pem-issuer-chain", "permitted-dns-names", "permitted-email-addresses", "permitted-ip-ranges", "permitted-uris", "postal-code", "province", "public-key", "satisfies-pzi", "satisfies-pzs", "server-auth", "state", "street-address", "subject", "subject-alt-name", "subject-config", "subject-key-id", "subordinate-config", "tier", "time-stamping", "type", "update-time", "uris", "x509-config"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CertificateAuthority = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_ca_pools_certificate_authorities_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_ca_pools_certificate_authorities_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_ca_pools_certificate_authorities_delete(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "skip-grace-period" => {
                    call = call.skip_grace_period(        value.map(|v| arg_from_str(v, err, "skip-grace-period", "boolean")).unwrap_or(false));
                },
                "request-id" => {
                    call = call.request_id(value.unwrap_or(""));
                },
                "ignore-dependent-resources" => {
                    call = call.ignore_dependent_resources(        value.map(|v| arg_from_str(v, err, "ignore-dependent-resources", "boolean")).unwrap_or(false));
                },
                "ignore-active-certificates" => {
                    call = call.ignore_active_certificates(        value.map(|v| arg_from_str(v, err, "ignore-active-certificates", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["ignore-active-certificates", "ignore-dependent-resources", "request-id", "skip-grace-period"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _projects_locations_ca_pools_certificate_authorities_disable(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "ignore-dependent-resources" => Some(("ignoreDependentResources", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "request-id" => Some(("requestId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["ignore-dependent-resources", "request-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::DisableCertificateAuthorityRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_ca_pools_certificate_authorities_disable(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_ca_pools_certificate_authorities_enable(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.projects().locations_ca_pools_certificate_authorities_enable(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_ca_pools_certificate_authorities_fetch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_ca_pools_certificate_authorities_fetch(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_ca_pools_certificate_authorities_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_ca_pools_certificate_authorities_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_ca_pools_certificate_authorities_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_ca_pools_certificate_authorities_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_ca_pools_certificate_authorities_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "access-urls.crl-access-urls" => Some(("accessUrls.crlAccessUrls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.public-key.format" => Some(("config.publicKey.format", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.public-key.key" => Some(("config.publicKey.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.subject-config.subject.common-name" => Some(("config.subjectConfig.subject.commonName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                    "config.subject-key-id.key-id" => Some(("config.subjectKeyId.keyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.x509-config.aia-ocsp-servers" => Some(("config.x509Config.aiaOcspServers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.x509-config.ca-options.is-ca" => Some(("config.x509Config.caOptions.isCa", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.ca-options.max-issuer-path-length" => Some(("config.x509Config.caOptions.maxIssuerPathLength", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.base-key-usage.cert-sign" => Some(("config.x509Config.keyUsage.baseKeyUsage.certSign", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.base-key-usage.content-commitment" => Some(("config.x509Config.keyUsage.baseKeyUsage.contentCommitment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.base-key-usage.crl-sign" => Some(("config.x509Config.keyUsage.baseKeyUsage.crlSign", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.base-key-usage.data-encipherment" => Some(("config.x509Config.keyUsage.baseKeyUsage.dataEncipherment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.base-key-usage.decipher-only" => Some(("config.x509Config.keyUsage.baseKeyUsage.decipherOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.base-key-usage.digital-signature" => Some(("config.x509Config.keyUsage.baseKeyUsage.digitalSignature", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.base-key-usage.encipher-only" => Some(("config.x509Config.keyUsage.baseKeyUsage.encipherOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.base-key-usage.key-agreement" => Some(("config.x509Config.keyUsage.baseKeyUsage.keyAgreement", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.base-key-usage.key-encipherment" => Some(("config.x509Config.keyUsage.baseKeyUsage.keyEncipherment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.extended-key-usage.client-auth" => Some(("config.x509Config.keyUsage.extendedKeyUsage.clientAuth", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.extended-key-usage.code-signing" => Some(("config.x509Config.keyUsage.extendedKeyUsage.codeSigning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.extended-key-usage.email-protection" => Some(("config.x509Config.keyUsage.extendedKeyUsage.emailProtection", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.extended-key-usage.ocsp-signing" => Some(("config.x509Config.keyUsage.extendedKeyUsage.ocspSigning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.extended-key-usage.server-auth" => Some(("config.x509Config.keyUsage.extendedKeyUsage.serverAuth", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.extended-key-usage.time-stamping" => Some(("config.x509Config.keyUsage.extendedKeyUsage.timeStamping", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.name-constraints.critical" => Some(("config.x509Config.nameConstraints.critical", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.name-constraints.excluded-dns-names" => Some(("config.x509Config.nameConstraints.excludedDnsNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.x509-config.name-constraints.excluded-email-addresses" => Some(("config.x509Config.nameConstraints.excludedEmailAddresses", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.x509-config.name-constraints.excluded-ip-ranges" => Some(("config.x509Config.nameConstraints.excludedIpRanges", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.x509-config.name-constraints.excluded-uris" => Some(("config.x509Config.nameConstraints.excludedUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.x509-config.name-constraints.permitted-dns-names" => Some(("config.x509Config.nameConstraints.permittedDnsNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.x509-config.name-constraints.permitted-email-addresses" => Some(("config.x509Config.nameConstraints.permittedEmailAddresses", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.x509-config.name-constraints.permitted-ip-ranges" => Some(("config.x509Config.nameConstraints.permittedIpRanges", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.x509-config.name-constraints.permitted-uris" => Some(("config.x509Config.nameConstraints.permittedUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "delete-time" => Some(("deleteTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "expire-time" => Some(("expireTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gcs-bucket" => Some(("gcsBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "key-spec.algorithm" => Some(("keySpec.algorithm", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "key-spec.cloud-kms-key-version" => Some(("keySpec.cloudKmsKeyVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "lifetime" => Some(("lifetime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pem-ca-certificates" => Some(("pemCaCertificates", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "satisfies-pzi" => Some(("satisfiesPzi", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "satisfies-pzs" => Some(("satisfiesPzs", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "subordinate-config.certificate-authority" => Some(("subordinateConfig.certificateAuthority", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "subordinate-config.pem-issuer-chain.pem-certificates" => Some(("subordinateConfig.pemIssuerChain.pemCertificates", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "tier" => Some(("tier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["access-urls", "aia-ocsp-servers", "algorithm", "base-key-usage", "ca-certificate-access-url", "ca-options", "cert-sign", "certificate-authority", "client-auth", "cloud-kms-key-version", "code-signing", "common-name", "config", "content-commitment", "country-code", "create-time", "critical", "crl-access-urls", "crl-sign", "data-encipherment", "decipher-only", "delete-time", "digital-signature", "dns-names", "email-addresses", "email-protection", "encipher-only", "excluded-dns-names", "excluded-email-addresses", "excluded-ip-ranges", "excluded-uris", "expire-time", "extended-key-usage", "format", "gcs-bucket", "ip-addresses", "is-ca", "key", "key-agreement", "key-encipherment", "key-id", "key-spec", "key-usage", "labels", "lifetime", "locality", "max-issuer-path-length", "name", "name-constraints", "ocsp-signing", "organization", "organizational-unit", "pem-ca-certificates", "pem-certificates", "pem-issuer-chain", "permitted-dns-names", "permitted-email-addresses", "permitted-ip-ranges", "permitted-uris", "postal-code", "province", "public-key", "satisfies-pzi", "satisfies-pzs", "server-auth", "state", "street-address", "subject", "subject-alt-name", "subject-config", "subject-key-id", "subordinate-config", "tier", "time-stamping", "type", "update-time", "uris", "x509-config"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CertificateAuthority = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_ca_pools_certificate_authorities_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_ca_pools_certificate_authorities_undelete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::UndeleteCertificateAuthorityRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_ca_pools_certificate_authorities_undelete(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_ca_pools_certificates_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "certificate-description.crl-distribution-points" => Some(("certificateDescription.crlDistributionPoints", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-description.public-key.format" => Some(("certificateDescription.publicKey.format", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.public-key.key" => Some(("certificateDescription.publicKey.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.subject-description.hex-serial-number" => Some(("certificateDescription.subjectDescription.hexSerialNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.subject-description.lifetime" => Some(("certificateDescription.subjectDescription.lifetime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.subject-description.not-after-time" => Some(("certificateDescription.subjectDescription.notAfterTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.subject-description.not-before-time" => Some(("certificateDescription.subjectDescription.notBeforeTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.subject-description.subject.common-name" => Some(("certificateDescription.subjectDescription.subject.commonName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                    "certificate-description.x509-description.aia-ocsp-servers" => Some(("certificateDescription.x509Description.aiaOcspServers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-description.x509-description.ca-options.is-ca" => Some(("certificateDescription.x509Description.caOptions.isCa", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.x509-description.ca-options.max-issuer-path-length" => Some(("certificateDescription.x509Description.caOptions.maxIssuerPathLength", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "certificate-description.x509-description.key-usage.base-key-usage.cert-sign" => Some(("certificateDescription.x509Description.keyUsage.baseKeyUsage.certSign", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.x509-description.key-usage.base-key-usage.content-commitment" => Some(("certificateDescription.x509Description.keyUsage.baseKeyUsage.contentCommitment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.x509-description.key-usage.base-key-usage.crl-sign" => Some(("certificateDescription.x509Description.keyUsage.baseKeyUsage.crlSign", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.x509-description.key-usage.base-key-usage.data-encipherment" => Some(("certificateDescription.x509Description.keyUsage.baseKeyUsage.dataEncipherment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.x509-description.key-usage.base-key-usage.decipher-only" => Some(("certificateDescription.x509Description.keyUsage.baseKeyUsage.decipherOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.x509-description.key-usage.base-key-usage.digital-signature" => Some(("certificateDescription.x509Description.keyUsage.baseKeyUsage.digitalSignature", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.x509-description.key-usage.base-key-usage.encipher-only" => Some(("certificateDescription.x509Description.keyUsage.baseKeyUsage.encipherOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.x509-description.key-usage.base-key-usage.key-agreement" => Some(("certificateDescription.x509Description.keyUsage.baseKeyUsage.keyAgreement", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.x509-description.key-usage.base-key-usage.key-encipherment" => Some(("certificateDescription.x509Description.keyUsage.baseKeyUsage.keyEncipherment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.x509-description.key-usage.extended-key-usage.client-auth" => Some(("certificateDescription.x509Description.keyUsage.extendedKeyUsage.clientAuth", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.x509-description.key-usage.extended-key-usage.code-signing" => Some(("certificateDescription.x509Description.keyUsage.extendedKeyUsage.codeSigning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.x509-description.key-usage.extended-key-usage.email-protection" => Some(("certificateDescription.x509Description.keyUsage.extendedKeyUsage.emailProtection", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.x509-description.key-usage.extended-key-usage.ocsp-signing" => Some(("certificateDescription.x509Description.keyUsage.extendedKeyUsage.ocspSigning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.x509-description.key-usage.extended-key-usage.server-auth" => Some(("certificateDescription.x509Description.keyUsage.extendedKeyUsage.serverAuth", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.x509-description.key-usage.extended-key-usage.time-stamping" => Some(("certificateDescription.x509Description.keyUsage.extendedKeyUsage.timeStamping", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.x509-description.name-constraints.critical" => Some(("certificateDescription.x509Description.nameConstraints.critical", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.x509-description.name-constraints.excluded-dns-names" => Some(("certificateDescription.x509Description.nameConstraints.excludedDnsNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-description.x509-description.name-constraints.excluded-email-addresses" => Some(("certificateDescription.x509Description.nameConstraints.excludedEmailAddresses", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-description.x509-description.name-constraints.excluded-ip-ranges" => Some(("certificateDescription.x509Description.nameConstraints.excludedIpRanges", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-description.x509-description.name-constraints.excluded-uris" => Some(("certificateDescription.x509Description.nameConstraints.excludedUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-description.x509-description.name-constraints.permitted-dns-names" => Some(("certificateDescription.x509Description.nameConstraints.permittedDnsNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-description.x509-description.name-constraints.permitted-email-addresses" => Some(("certificateDescription.x509Description.nameConstraints.permittedEmailAddresses", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-description.x509-description.name-constraints.permitted-ip-ranges" => Some(("certificateDescription.x509Description.nameConstraints.permittedIpRanges", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-description.x509-description.name-constraints.permitted-uris" => Some(("certificateDescription.x509Description.nameConstraints.permittedUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-template" => Some(("certificateTemplate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.public-key.format" => Some(("config.publicKey.format", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.public-key.key" => Some(("config.publicKey.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.subject-config.subject.common-name" => Some(("config.subjectConfig.subject.commonName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                    "config.subject-key-id.key-id" => Some(("config.subjectKeyId.keyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.x509-config.aia-ocsp-servers" => Some(("config.x509Config.aiaOcspServers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.x509-config.ca-options.is-ca" => Some(("config.x509Config.caOptions.isCa", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.ca-options.max-issuer-path-length" => Some(("config.x509Config.caOptions.maxIssuerPathLength", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.base-key-usage.cert-sign" => Some(("config.x509Config.keyUsage.baseKeyUsage.certSign", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.base-key-usage.content-commitment" => Some(("config.x509Config.keyUsage.baseKeyUsage.contentCommitment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.base-key-usage.crl-sign" => Some(("config.x509Config.keyUsage.baseKeyUsage.crlSign", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.base-key-usage.data-encipherment" => Some(("config.x509Config.keyUsage.baseKeyUsage.dataEncipherment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.base-key-usage.decipher-only" => Some(("config.x509Config.keyUsage.baseKeyUsage.decipherOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.base-key-usage.digital-signature" => Some(("config.x509Config.keyUsage.baseKeyUsage.digitalSignature", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.base-key-usage.encipher-only" => Some(("config.x509Config.keyUsage.baseKeyUsage.encipherOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.base-key-usage.key-agreement" => Some(("config.x509Config.keyUsage.baseKeyUsage.keyAgreement", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.base-key-usage.key-encipherment" => Some(("config.x509Config.keyUsage.baseKeyUsage.keyEncipherment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.extended-key-usage.client-auth" => Some(("config.x509Config.keyUsage.extendedKeyUsage.clientAuth", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.extended-key-usage.code-signing" => Some(("config.x509Config.keyUsage.extendedKeyUsage.codeSigning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.extended-key-usage.email-protection" => Some(("config.x509Config.keyUsage.extendedKeyUsage.emailProtection", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.extended-key-usage.ocsp-signing" => Some(("config.x509Config.keyUsage.extendedKeyUsage.ocspSigning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.extended-key-usage.server-auth" => Some(("config.x509Config.keyUsage.extendedKeyUsage.serverAuth", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.extended-key-usage.time-stamping" => Some(("config.x509Config.keyUsage.extendedKeyUsage.timeStamping", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.name-constraints.critical" => Some(("config.x509Config.nameConstraints.critical", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.name-constraints.excluded-dns-names" => Some(("config.x509Config.nameConstraints.excludedDnsNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.x509-config.name-constraints.excluded-email-addresses" => Some(("config.x509Config.nameConstraints.excludedEmailAddresses", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.x509-config.name-constraints.excluded-ip-ranges" => Some(("config.x509Config.nameConstraints.excludedIpRanges", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.x509-config.name-constraints.excluded-uris" => Some(("config.x509Config.nameConstraints.excludedUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.x509-config.name-constraints.permitted-dns-names" => Some(("config.x509Config.nameConstraints.permittedDnsNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.x509-config.name-constraints.permitted-email-addresses" => Some(("config.x509Config.nameConstraints.permittedEmailAddresses", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.x509-config.name-constraints.permitted-ip-ranges" => Some(("config.x509Config.nameConstraints.permittedIpRanges", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.x509-config.name-constraints.permitted-uris" => Some(("config.x509Config.nameConstraints.permittedUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "issuer-certificate-authority" => Some(("issuerCertificateAuthority", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "lifetime" => Some(("lifetime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pem-certificate" => Some(("pemCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pem-certificate-chain" => Some(("pemCertificateChain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "pem-csr" => Some(("pemCsr", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "revocation-details.revocation-state" => Some(("revocationDetails.revocationState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "revocation-details.revocation-time" => Some(("revocationDetails.revocationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "subject-mode" => Some(("subjectMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["aia-issuing-certificate-urls", "aia-ocsp-servers", "authority-key-id", "base-key-usage", "ca-options", "cert-fingerprint", "cert-sign", "certificate-description", "certificate-template", "client-auth", "code-signing", "common-name", "config", "content-commitment", "country-code", "create-time", "critical", "crl-distribution-points", "crl-sign", "data-encipherment", "decipher-only", "digital-signature", "dns-names", "email-addresses", "email-protection", "encipher-only", "excluded-dns-names", "excluded-email-addresses", "excluded-ip-ranges", "excluded-uris", "extended-key-usage", "format", "hex-serial-number", "ip-addresses", "is-ca", "issuer-certificate-authority", "key", "key-agreement", "key-encipherment", "key-id", "key-usage", "labels", "lifetime", "locality", "max-issuer-path-length", "name", "name-constraints", "not-after-time", "not-before-time", "ocsp-signing", "organization", "organizational-unit", "pem-certificate", "pem-certificate-chain", "pem-csr", "permitted-dns-names", "permitted-email-addresses", "permitted-ip-ranges", "permitted-uris", "postal-code", "province", "public-key", "revocation-details", "revocation-state", "revocation-time", "server-auth", "sha256-hash", "street-address", "subject", "subject-alt-name", "subject-config", "subject-description", "subject-key-id", "subject-mode", "time-stamping", "update-time", "uris", "x509-config", "x509-description"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Certificate = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_ca_pools_certificates_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "validate-only" => {
                    call = call.validate_only(        value.map(|v| arg_from_str(v, err, "validate-only", "boolean")).unwrap_or(false));
                },
                "request-id" => {
                    call = call.request_id(value.unwrap_or(""));
                },
                "issuing-certificate-authority-id" => {
                    call = call.issuing_certificate_authority_id(value.unwrap_or(""));
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
                                                                           v.extend(["certificate-id", "issuing-certificate-authority-id", "request-id", "validate-only"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _projects_locations_ca_pools_certificates_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_ca_pools_certificates_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_ca_pools_certificates_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_ca_pools_certificates_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_ca_pools_certificates_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "certificate-description.crl-distribution-points" => Some(("certificateDescription.crlDistributionPoints", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-description.public-key.format" => Some(("certificateDescription.publicKey.format", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.public-key.key" => Some(("certificateDescription.publicKey.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.subject-description.hex-serial-number" => Some(("certificateDescription.subjectDescription.hexSerialNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.subject-description.lifetime" => Some(("certificateDescription.subjectDescription.lifetime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.subject-description.not-after-time" => Some(("certificateDescription.subjectDescription.notAfterTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.subject-description.not-before-time" => Some(("certificateDescription.subjectDescription.notBeforeTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-description.subject-description.subject.common-name" => Some(("certificateDescription.subjectDescription.subject.commonName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                    "certificate-description.x509-description.aia-ocsp-servers" => Some(("certificateDescription.x509Description.aiaOcspServers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-description.x509-description.ca-options.is-ca" => Some(("certificateDescription.x509Description.caOptions.isCa", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.x509-description.ca-options.max-issuer-path-length" => Some(("certificateDescription.x509Description.caOptions.maxIssuerPathLength", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "certificate-description.x509-description.key-usage.base-key-usage.cert-sign" => Some(("certificateDescription.x509Description.keyUsage.baseKeyUsage.certSign", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.x509-description.key-usage.base-key-usage.content-commitment" => Some(("certificateDescription.x509Description.keyUsage.baseKeyUsage.contentCommitment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.x509-description.key-usage.base-key-usage.crl-sign" => Some(("certificateDescription.x509Description.keyUsage.baseKeyUsage.crlSign", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.x509-description.key-usage.base-key-usage.data-encipherment" => Some(("certificateDescription.x509Description.keyUsage.baseKeyUsage.dataEncipherment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.x509-description.key-usage.base-key-usage.decipher-only" => Some(("certificateDescription.x509Description.keyUsage.baseKeyUsage.decipherOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.x509-description.key-usage.base-key-usage.digital-signature" => Some(("certificateDescription.x509Description.keyUsage.baseKeyUsage.digitalSignature", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.x509-description.key-usage.base-key-usage.encipher-only" => Some(("certificateDescription.x509Description.keyUsage.baseKeyUsage.encipherOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.x509-description.key-usage.base-key-usage.key-agreement" => Some(("certificateDescription.x509Description.keyUsage.baseKeyUsage.keyAgreement", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.x509-description.key-usage.base-key-usage.key-encipherment" => Some(("certificateDescription.x509Description.keyUsage.baseKeyUsage.keyEncipherment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.x509-description.key-usage.extended-key-usage.client-auth" => Some(("certificateDescription.x509Description.keyUsage.extendedKeyUsage.clientAuth", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.x509-description.key-usage.extended-key-usage.code-signing" => Some(("certificateDescription.x509Description.keyUsage.extendedKeyUsage.codeSigning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.x509-description.key-usage.extended-key-usage.email-protection" => Some(("certificateDescription.x509Description.keyUsage.extendedKeyUsage.emailProtection", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.x509-description.key-usage.extended-key-usage.ocsp-signing" => Some(("certificateDescription.x509Description.keyUsage.extendedKeyUsage.ocspSigning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.x509-description.key-usage.extended-key-usage.server-auth" => Some(("certificateDescription.x509Description.keyUsage.extendedKeyUsage.serverAuth", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.x509-description.key-usage.extended-key-usage.time-stamping" => Some(("certificateDescription.x509Description.keyUsage.extendedKeyUsage.timeStamping", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.x509-description.name-constraints.critical" => Some(("certificateDescription.x509Description.nameConstraints.critical", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "certificate-description.x509-description.name-constraints.excluded-dns-names" => Some(("certificateDescription.x509Description.nameConstraints.excludedDnsNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-description.x509-description.name-constraints.excluded-email-addresses" => Some(("certificateDescription.x509Description.nameConstraints.excludedEmailAddresses", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-description.x509-description.name-constraints.excluded-ip-ranges" => Some(("certificateDescription.x509Description.nameConstraints.excludedIpRanges", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-description.x509-description.name-constraints.excluded-uris" => Some(("certificateDescription.x509Description.nameConstraints.excludedUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-description.x509-description.name-constraints.permitted-dns-names" => Some(("certificateDescription.x509Description.nameConstraints.permittedDnsNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-description.x509-description.name-constraints.permitted-email-addresses" => Some(("certificateDescription.x509Description.nameConstraints.permittedEmailAddresses", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-description.x509-description.name-constraints.permitted-ip-ranges" => Some(("certificateDescription.x509Description.nameConstraints.permittedIpRanges", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-description.x509-description.name-constraints.permitted-uris" => Some(("certificateDescription.x509Description.nameConstraints.permittedUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "certificate-template" => Some(("certificateTemplate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.public-key.format" => Some(("config.publicKey.format", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.public-key.key" => Some(("config.publicKey.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.subject-config.subject.common-name" => Some(("config.subjectConfig.subject.commonName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                    "config.subject-key-id.key-id" => Some(("config.subjectKeyId.keyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.x509-config.aia-ocsp-servers" => Some(("config.x509Config.aiaOcspServers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.x509-config.ca-options.is-ca" => Some(("config.x509Config.caOptions.isCa", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.ca-options.max-issuer-path-length" => Some(("config.x509Config.caOptions.maxIssuerPathLength", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.base-key-usage.cert-sign" => Some(("config.x509Config.keyUsage.baseKeyUsage.certSign", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.base-key-usage.content-commitment" => Some(("config.x509Config.keyUsage.baseKeyUsage.contentCommitment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.base-key-usage.crl-sign" => Some(("config.x509Config.keyUsage.baseKeyUsage.crlSign", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.base-key-usage.data-encipherment" => Some(("config.x509Config.keyUsage.baseKeyUsage.dataEncipherment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.base-key-usage.decipher-only" => Some(("config.x509Config.keyUsage.baseKeyUsage.decipherOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.base-key-usage.digital-signature" => Some(("config.x509Config.keyUsage.baseKeyUsage.digitalSignature", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.base-key-usage.encipher-only" => Some(("config.x509Config.keyUsage.baseKeyUsage.encipherOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.base-key-usage.key-agreement" => Some(("config.x509Config.keyUsage.baseKeyUsage.keyAgreement", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.base-key-usage.key-encipherment" => Some(("config.x509Config.keyUsage.baseKeyUsage.keyEncipherment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.extended-key-usage.client-auth" => Some(("config.x509Config.keyUsage.extendedKeyUsage.clientAuth", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.extended-key-usage.code-signing" => Some(("config.x509Config.keyUsage.extendedKeyUsage.codeSigning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.extended-key-usage.email-protection" => Some(("config.x509Config.keyUsage.extendedKeyUsage.emailProtection", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.extended-key-usage.ocsp-signing" => Some(("config.x509Config.keyUsage.extendedKeyUsage.ocspSigning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.extended-key-usage.server-auth" => Some(("config.x509Config.keyUsage.extendedKeyUsage.serverAuth", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.key-usage.extended-key-usage.time-stamping" => Some(("config.x509Config.keyUsage.extendedKeyUsage.timeStamping", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.name-constraints.critical" => Some(("config.x509Config.nameConstraints.critical", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.x509-config.name-constraints.excluded-dns-names" => Some(("config.x509Config.nameConstraints.excludedDnsNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.x509-config.name-constraints.excluded-email-addresses" => Some(("config.x509Config.nameConstraints.excludedEmailAddresses", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.x509-config.name-constraints.excluded-ip-ranges" => Some(("config.x509Config.nameConstraints.excludedIpRanges", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.x509-config.name-constraints.excluded-uris" => Some(("config.x509Config.nameConstraints.excludedUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.x509-config.name-constraints.permitted-dns-names" => Some(("config.x509Config.nameConstraints.permittedDnsNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.x509-config.name-constraints.permitted-email-addresses" => Some(("config.x509Config.nameConstraints.permittedEmailAddresses", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.x509-config.name-constraints.permitted-ip-ranges" => Some(("config.x509Config.nameConstraints.permittedIpRanges", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.x509-config.name-constraints.permitted-uris" => Some(("config.x509Config.nameConstraints.permittedUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "issuer-certificate-authority" => Some(("issuerCertificateAuthority", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "lifetime" => Some(("lifetime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pem-certificate" => Some(("pemCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pem-certificate-chain" => Some(("pemCertificateChain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "pem-csr" => Some(("pemCsr", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "revocation-details.revocation-state" => Some(("revocationDetails.revocationState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "revocation-details.revocation-time" => Some(("revocationDetails.revocationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "subject-mode" => Some(("subjectMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["aia-issuing-certificate-urls", "aia-ocsp-servers", "authority-key-id", "base-key-usage", "ca-options", "cert-fingerprint", "cert-sign", "certificate-description", "certificate-template", "client-auth", "code-signing", "common-name", "config", "content-commitment", "country-code", "create-time", "critical", "crl-distribution-points", "crl-sign", "data-encipherment", "decipher-only", "digital-signature", "dns-names", "email-addresses", "email-protection", "encipher-only", "excluded-dns-names", "excluded-email-addresses", "excluded-ip-ranges", "excluded-uris", "extended-key-usage", "format", "hex-serial-number", "ip-addresses", "is-ca", "issuer-certificate-authority", "key", "key-agreement", "key-encipherment", "key-id", "key-usage", "labels", "lifetime", "locality", "max-issuer-path-length", "name", "name-constraints", "not-after-time", "not-before-time", "ocsp-signing", "organization", "organizational-unit", "pem-certificate", "pem-certificate-chain", "pem-csr", "permitted-dns-names", "permitted-email-addresses", "permitted-ip-ranges", "permitted-uris", "postal-code", "province", "public-key", "revocation-details", "revocation-state", "revocation-time", "server-auth", "sha256-hash", "street-address", "subject", "subject-alt-name", "subject-config", "subject-description", "subject-key-id", "subject-mode", "time-stamping", "update-time", "uris", "x509-config", "x509-description"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Certificate = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_ca_pools_certificates_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_ca_pools_certificates_revoke(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.projects().locations_ca_pools_certificates_revoke(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_ca_pools_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "issuance-policy.allowed-issuance-modes.allow-config-based-issuance" => Some(("issuancePolicy.allowedIssuanceModes.allowConfigBasedIssuance", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.allowed-issuance-modes.allow-csr-based-issuance" => Some(("issuancePolicy.allowedIssuanceModes.allowCsrBasedIssuance", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.baseline-values.aia-ocsp-servers" => Some(("issuancePolicy.baselineValues.aiaOcspServers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "issuance-policy.baseline-values.ca-options.is-ca" => Some(("issuancePolicy.baselineValues.caOptions.isCa", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.baseline-values.ca-options.max-issuer-path-length" => Some(("issuancePolicy.baselineValues.caOptions.maxIssuerPathLength", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "issuance-policy.baseline-values.key-usage.base-key-usage.cert-sign" => Some(("issuancePolicy.baselineValues.keyUsage.baseKeyUsage.certSign", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.baseline-values.key-usage.base-key-usage.content-commitment" => Some(("issuancePolicy.baselineValues.keyUsage.baseKeyUsage.contentCommitment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.baseline-values.key-usage.base-key-usage.crl-sign" => Some(("issuancePolicy.baselineValues.keyUsage.baseKeyUsage.crlSign", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.baseline-values.key-usage.base-key-usage.data-encipherment" => Some(("issuancePolicy.baselineValues.keyUsage.baseKeyUsage.dataEncipherment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.baseline-values.key-usage.base-key-usage.decipher-only" => Some(("issuancePolicy.baselineValues.keyUsage.baseKeyUsage.decipherOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.baseline-values.key-usage.base-key-usage.digital-signature" => Some(("issuancePolicy.baselineValues.keyUsage.baseKeyUsage.digitalSignature", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.baseline-values.key-usage.base-key-usage.encipher-only" => Some(("issuancePolicy.baselineValues.keyUsage.baseKeyUsage.encipherOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.baseline-values.key-usage.base-key-usage.key-agreement" => Some(("issuancePolicy.baselineValues.keyUsage.baseKeyUsage.keyAgreement", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.baseline-values.key-usage.base-key-usage.key-encipherment" => Some(("issuancePolicy.baselineValues.keyUsage.baseKeyUsage.keyEncipherment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.baseline-values.key-usage.extended-key-usage.client-auth" => Some(("issuancePolicy.baselineValues.keyUsage.extendedKeyUsage.clientAuth", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.baseline-values.key-usage.extended-key-usage.code-signing" => Some(("issuancePolicy.baselineValues.keyUsage.extendedKeyUsage.codeSigning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.baseline-values.key-usage.extended-key-usage.email-protection" => Some(("issuancePolicy.baselineValues.keyUsage.extendedKeyUsage.emailProtection", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.baseline-values.key-usage.extended-key-usage.ocsp-signing" => Some(("issuancePolicy.baselineValues.keyUsage.extendedKeyUsage.ocspSigning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.baseline-values.key-usage.extended-key-usage.server-auth" => Some(("issuancePolicy.baselineValues.keyUsage.extendedKeyUsage.serverAuth", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.baseline-values.key-usage.extended-key-usage.time-stamping" => Some(("issuancePolicy.baselineValues.keyUsage.extendedKeyUsage.timeStamping", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.baseline-values.name-constraints.critical" => Some(("issuancePolicy.baselineValues.nameConstraints.critical", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.baseline-values.name-constraints.excluded-dns-names" => Some(("issuancePolicy.baselineValues.nameConstraints.excludedDnsNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "issuance-policy.baseline-values.name-constraints.excluded-email-addresses" => Some(("issuancePolicy.baselineValues.nameConstraints.excludedEmailAddresses", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "issuance-policy.baseline-values.name-constraints.excluded-ip-ranges" => Some(("issuancePolicy.baselineValues.nameConstraints.excludedIpRanges", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "issuance-policy.baseline-values.name-constraints.excluded-uris" => Some(("issuancePolicy.baselineValues.nameConstraints.excludedUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "issuance-policy.baseline-values.name-constraints.permitted-dns-names" => Some(("issuancePolicy.baselineValues.nameConstraints.permittedDnsNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "issuance-policy.baseline-values.name-constraints.permitted-email-addresses" => Some(("issuancePolicy.baselineValues.nameConstraints.permittedEmailAddresses", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "issuance-policy.baseline-values.name-constraints.permitted-ip-ranges" => Some(("issuancePolicy.baselineValues.nameConstraints.permittedIpRanges", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "issuance-policy.baseline-values.name-constraints.permitted-uris" => Some(("issuancePolicy.baselineValues.nameConstraints.permittedUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "issuance-policy.identity-constraints.allow-subject-alt-names-passthrough" => Some(("issuancePolicy.identityConstraints.allowSubjectAltNamesPassthrough", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.identity-constraints.allow-subject-passthrough" => Some(("issuancePolicy.identityConstraints.allowSubjectPassthrough", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.identity-constraints.cel-expression.description" => Some(("issuancePolicy.identityConstraints.celExpression.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "issuance-policy.identity-constraints.cel-expression.expression" => Some(("issuancePolicy.identityConstraints.celExpression.expression", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "issuance-policy.identity-constraints.cel-expression.location" => Some(("issuancePolicy.identityConstraints.celExpression.location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "issuance-policy.identity-constraints.cel-expression.title" => Some(("issuancePolicy.identityConstraints.celExpression.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "issuance-policy.maximum-lifetime" => Some(("issuancePolicy.maximumLifetime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "issuance-policy.passthrough-extensions.known-extensions" => Some(("issuancePolicy.passthroughExtensions.knownExtensions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "publishing-options.encoding-format" => Some(("publishingOptions.encodingFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "publishing-options.publish-ca-cert" => Some(("publishingOptions.publishCaCert", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "publishing-options.publish-crl" => Some(("publishingOptions.publishCrl", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "tier" => Some(("tier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["aia-ocsp-servers", "allow-config-based-issuance", "allow-csr-based-issuance", "allow-subject-alt-names-passthrough", "allow-subject-passthrough", "allowed-issuance-modes", "base-key-usage", "baseline-values", "ca-options", "cel-expression", "cert-sign", "client-auth", "code-signing", "content-commitment", "critical", "crl-sign", "data-encipherment", "decipher-only", "description", "digital-signature", "email-protection", "encipher-only", "encoding-format", "excluded-dns-names", "excluded-email-addresses", "excluded-ip-ranges", "excluded-uris", "expression", "extended-key-usage", "identity-constraints", "is-ca", "issuance-policy", "key-agreement", "key-encipherment", "key-usage", "known-extensions", "labels", "location", "max-issuer-path-length", "maximum-lifetime", "name", "name-constraints", "ocsp-signing", "passthrough-extensions", "permitted-dns-names", "permitted-email-addresses", "permitted-ip-ranges", "permitted-uris", "publish-ca-cert", "publish-crl", "publishing-options", "server-auth", "tier", "time-stamping", "title"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CaPool = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_ca_pools_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "request-id" => {
                    call = call.request_id(value.unwrap_or(""));
                },
                "ca-pool-id" => {
                    call = call.ca_pool_id(value.unwrap_or(""));
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
                                                                           v.extend(["ca-pool-id", "request-id"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _projects_locations_ca_pools_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_ca_pools_delete(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "request-id" => {
                    call = call.request_id(value.unwrap_or(""));
                },
                "ignore-dependent-resources" => {
                    call = call.ignore_dependent_resources(        value.map(|v| arg_from_str(v, err, "ignore-dependent-resources", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["ignore-dependent-resources", "request-id"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _projects_locations_ca_pools_fetch_ca_certs(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::FetchCaCertsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_ca_pools_fetch_ca_certs(request, opt.value_of("ca-pool").unwrap_or(""));
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

    async fn _projects_locations_ca_pools_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_ca_pools_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_ca_pools_get_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_ca_pools_get_iam_policy(opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_locations_ca_pools_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_ca_pools_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_ca_pools_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "issuance-policy.allowed-issuance-modes.allow-config-based-issuance" => Some(("issuancePolicy.allowedIssuanceModes.allowConfigBasedIssuance", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.allowed-issuance-modes.allow-csr-based-issuance" => Some(("issuancePolicy.allowedIssuanceModes.allowCsrBasedIssuance", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.baseline-values.aia-ocsp-servers" => Some(("issuancePolicy.baselineValues.aiaOcspServers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "issuance-policy.baseline-values.ca-options.is-ca" => Some(("issuancePolicy.baselineValues.caOptions.isCa", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.baseline-values.ca-options.max-issuer-path-length" => Some(("issuancePolicy.baselineValues.caOptions.maxIssuerPathLength", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "issuance-policy.baseline-values.key-usage.base-key-usage.cert-sign" => Some(("issuancePolicy.baselineValues.keyUsage.baseKeyUsage.certSign", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.baseline-values.key-usage.base-key-usage.content-commitment" => Some(("issuancePolicy.baselineValues.keyUsage.baseKeyUsage.contentCommitment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.baseline-values.key-usage.base-key-usage.crl-sign" => Some(("issuancePolicy.baselineValues.keyUsage.baseKeyUsage.crlSign", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.baseline-values.key-usage.base-key-usage.data-encipherment" => Some(("issuancePolicy.baselineValues.keyUsage.baseKeyUsage.dataEncipherment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.baseline-values.key-usage.base-key-usage.decipher-only" => Some(("issuancePolicy.baselineValues.keyUsage.baseKeyUsage.decipherOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.baseline-values.key-usage.base-key-usage.digital-signature" => Some(("issuancePolicy.baselineValues.keyUsage.baseKeyUsage.digitalSignature", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.baseline-values.key-usage.base-key-usage.encipher-only" => Some(("issuancePolicy.baselineValues.keyUsage.baseKeyUsage.encipherOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.baseline-values.key-usage.base-key-usage.key-agreement" => Some(("issuancePolicy.baselineValues.keyUsage.baseKeyUsage.keyAgreement", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.baseline-values.key-usage.base-key-usage.key-encipherment" => Some(("issuancePolicy.baselineValues.keyUsage.baseKeyUsage.keyEncipherment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.baseline-values.key-usage.extended-key-usage.client-auth" => Some(("issuancePolicy.baselineValues.keyUsage.extendedKeyUsage.clientAuth", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.baseline-values.key-usage.extended-key-usage.code-signing" => Some(("issuancePolicy.baselineValues.keyUsage.extendedKeyUsage.codeSigning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.baseline-values.key-usage.extended-key-usage.email-protection" => Some(("issuancePolicy.baselineValues.keyUsage.extendedKeyUsage.emailProtection", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.baseline-values.key-usage.extended-key-usage.ocsp-signing" => Some(("issuancePolicy.baselineValues.keyUsage.extendedKeyUsage.ocspSigning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.baseline-values.key-usage.extended-key-usage.server-auth" => Some(("issuancePolicy.baselineValues.keyUsage.extendedKeyUsage.serverAuth", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.baseline-values.key-usage.extended-key-usage.time-stamping" => Some(("issuancePolicy.baselineValues.keyUsage.extendedKeyUsage.timeStamping", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.baseline-values.name-constraints.critical" => Some(("issuancePolicy.baselineValues.nameConstraints.critical", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.baseline-values.name-constraints.excluded-dns-names" => Some(("issuancePolicy.baselineValues.nameConstraints.excludedDnsNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "issuance-policy.baseline-values.name-constraints.excluded-email-addresses" => Some(("issuancePolicy.baselineValues.nameConstraints.excludedEmailAddresses", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "issuance-policy.baseline-values.name-constraints.excluded-ip-ranges" => Some(("issuancePolicy.baselineValues.nameConstraints.excludedIpRanges", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "issuance-policy.baseline-values.name-constraints.excluded-uris" => Some(("issuancePolicy.baselineValues.nameConstraints.excludedUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "issuance-policy.baseline-values.name-constraints.permitted-dns-names" => Some(("issuancePolicy.baselineValues.nameConstraints.permittedDnsNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "issuance-policy.baseline-values.name-constraints.permitted-email-addresses" => Some(("issuancePolicy.baselineValues.nameConstraints.permittedEmailAddresses", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "issuance-policy.baseline-values.name-constraints.permitted-ip-ranges" => Some(("issuancePolicy.baselineValues.nameConstraints.permittedIpRanges", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "issuance-policy.baseline-values.name-constraints.permitted-uris" => Some(("issuancePolicy.baselineValues.nameConstraints.permittedUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "issuance-policy.identity-constraints.allow-subject-alt-names-passthrough" => Some(("issuancePolicy.identityConstraints.allowSubjectAltNamesPassthrough", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.identity-constraints.allow-subject-passthrough" => Some(("issuancePolicy.identityConstraints.allowSubjectPassthrough", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "issuance-policy.identity-constraints.cel-expression.description" => Some(("issuancePolicy.identityConstraints.celExpression.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "issuance-policy.identity-constraints.cel-expression.expression" => Some(("issuancePolicy.identityConstraints.celExpression.expression", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "issuance-policy.identity-constraints.cel-expression.location" => Some(("issuancePolicy.identityConstraints.celExpression.location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "issuance-policy.identity-constraints.cel-expression.title" => Some(("issuancePolicy.identityConstraints.celExpression.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "issuance-policy.maximum-lifetime" => Some(("issuancePolicy.maximumLifetime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "issuance-policy.passthrough-extensions.known-extensions" => Some(("issuancePolicy.passthroughExtensions.knownExtensions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "publishing-options.encoding-format" => Some(("publishingOptions.encodingFormat", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "publishing-options.publish-ca-cert" => Some(("publishingOptions.publishCaCert", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "publishing-options.publish-crl" => Some(("publishingOptions.publishCrl", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "tier" => Some(("tier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["aia-ocsp-servers", "allow-config-based-issuance", "allow-csr-based-issuance", "allow-subject-alt-names-passthrough", "allow-subject-passthrough", "allowed-issuance-modes", "base-key-usage", "baseline-values", "ca-options", "cel-expression", "cert-sign", "client-auth", "code-signing", "content-commitment", "critical", "crl-sign", "data-encipherment", "decipher-only", "description", "digital-signature", "email-protection", "encipher-only", "encoding-format", "excluded-dns-names", "excluded-email-addresses", "excluded-ip-ranges", "excluded-uris", "expression", "extended-key-usage", "identity-constraints", "is-ca", "issuance-policy", "key-agreement", "key-encipherment", "key-usage", "known-extensions", "labels", "location", "max-issuer-path-length", "maximum-lifetime", "name", "name-constraints", "ocsp-signing", "passthrough-extensions", "permitted-dns-names", "permitted-email-addresses", "permitted-ip-ranges", "permitted-uris", "publish-ca-cert", "publish-crl", "publishing-options", "server-auth", "tier", "time-stamping", "title"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CaPool = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_ca_pools_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_ca_pools_set_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.projects().locations_ca_pools_set_iam_policy(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_locations_ca_pools_test_iam_permissions(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.projects().locations_ca_pools_test_iam_permissions(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_locations_certificate_templates_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "identity-constraints.allow-subject-alt-names-passthrough" => Some(("identityConstraints.allowSubjectAltNamesPassthrough", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "identity-constraints.allow-subject-passthrough" => Some(("identityConstraints.allowSubjectPassthrough", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "identity-constraints.cel-expression.description" => Some(("identityConstraints.celExpression.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "identity-constraints.cel-expression.expression" => Some(("identityConstraints.celExpression.expression", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "identity-constraints.cel-expression.location" => Some(("identityConstraints.celExpression.location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "identity-constraints.cel-expression.title" => Some(("identityConstraints.celExpression.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "maximum-lifetime" => Some(("maximumLifetime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "passthrough-extensions.known-extensions" => Some(("passthroughExtensions.knownExtensions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "predefined-values.aia-ocsp-servers" => Some(("predefinedValues.aiaOcspServers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "predefined-values.ca-options.is-ca" => Some(("predefinedValues.caOptions.isCa", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "predefined-values.ca-options.max-issuer-path-length" => Some(("predefinedValues.caOptions.maxIssuerPathLength", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "predefined-values.key-usage.base-key-usage.cert-sign" => Some(("predefinedValues.keyUsage.baseKeyUsage.certSign", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "predefined-values.key-usage.base-key-usage.content-commitment" => Some(("predefinedValues.keyUsage.baseKeyUsage.contentCommitment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "predefined-values.key-usage.base-key-usage.crl-sign" => Some(("predefinedValues.keyUsage.baseKeyUsage.crlSign", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "predefined-values.key-usage.base-key-usage.data-encipherment" => Some(("predefinedValues.keyUsage.baseKeyUsage.dataEncipherment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "predefined-values.key-usage.base-key-usage.decipher-only" => Some(("predefinedValues.keyUsage.baseKeyUsage.decipherOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "predefined-values.key-usage.base-key-usage.digital-signature" => Some(("predefinedValues.keyUsage.baseKeyUsage.digitalSignature", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "predefined-values.key-usage.base-key-usage.encipher-only" => Some(("predefinedValues.keyUsage.baseKeyUsage.encipherOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "predefined-values.key-usage.base-key-usage.key-agreement" => Some(("predefinedValues.keyUsage.baseKeyUsage.keyAgreement", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "predefined-values.key-usage.base-key-usage.key-encipherment" => Some(("predefinedValues.keyUsage.baseKeyUsage.keyEncipherment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "predefined-values.key-usage.extended-key-usage.client-auth" => Some(("predefinedValues.keyUsage.extendedKeyUsage.clientAuth", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "predefined-values.key-usage.extended-key-usage.code-signing" => Some(("predefinedValues.keyUsage.extendedKeyUsage.codeSigning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "predefined-values.key-usage.extended-key-usage.email-protection" => Some(("predefinedValues.keyUsage.extendedKeyUsage.emailProtection", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "predefined-values.key-usage.extended-key-usage.ocsp-signing" => Some(("predefinedValues.keyUsage.extendedKeyUsage.ocspSigning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "predefined-values.key-usage.extended-key-usage.server-auth" => Some(("predefinedValues.keyUsage.extendedKeyUsage.serverAuth", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "predefined-values.key-usage.extended-key-usage.time-stamping" => Some(("predefinedValues.keyUsage.extendedKeyUsage.timeStamping", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "predefined-values.name-constraints.critical" => Some(("predefinedValues.nameConstraints.critical", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "predefined-values.name-constraints.excluded-dns-names" => Some(("predefinedValues.nameConstraints.excludedDnsNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "predefined-values.name-constraints.excluded-email-addresses" => Some(("predefinedValues.nameConstraints.excludedEmailAddresses", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "predefined-values.name-constraints.excluded-ip-ranges" => Some(("predefinedValues.nameConstraints.excludedIpRanges", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "predefined-values.name-constraints.excluded-uris" => Some(("predefinedValues.nameConstraints.excludedUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "predefined-values.name-constraints.permitted-dns-names" => Some(("predefinedValues.nameConstraints.permittedDnsNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "predefined-values.name-constraints.permitted-email-addresses" => Some(("predefinedValues.nameConstraints.permittedEmailAddresses", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "predefined-values.name-constraints.permitted-ip-ranges" => Some(("predefinedValues.nameConstraints.permittedIpRanges", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "predefined-values.name-constraints.permitted-uris" => Some(("predefinedValues.nameConstraints.permittedUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["aia-ocsp-servers", "allow-subject-alt-names-passthrough", "allow-subject-passthrough", "base-key-usage", "ca-options", "cel-expression", "cert-sign", "client-auth", "code-signing", "content-commitment", "create-time", "critical", "crl-sign", "data-encipherment", "decipher-only", "description", "digital-signature", "email-protection", "encipher-only", "excluded-dns-names", "excluded-email-addresses", "excluded-ip-ranges", "excluded-uris", "expression", "extended-key-usage", "identity-constraints", "is-ca", "key-agreement", "key-encipherment", "key-usage", "known-extensions", "labels", "location", "max-issuer-path-length", "maximum-lifetime", "name", "name-constraints", "ocsp-signing", "passthrough-extensions", "permitted-dns-names", "permitted-email-addresses", "permitted-ip-ranges", "permitted-uris", "predefined-values", "server-auth", "time-stamping", "title", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CertificateTemplate = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_certificate_templates_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "request-id" => {
                    call = call.request_id(value.unwrap_or(""));
                },
                "certificate-template-id" => {
                    call = call.certificate_template_id(value.unwrap_or(""));
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
                                                                           v.extend(["certificate-template-id", "request-id"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _projects_locations_certificate_templates_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_certificate_templates_delete(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
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
                                                                           v.extend(["request-id"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _projects_locations_certificate_templates_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_certificate_templates_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_certificate_templates_get_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_certificate_templates_get_iam_policy(opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_locations_certificate_templates_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_certificate_templates_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_certificate_templates_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "identity-constraints.allow-subject-alt-names-passthrough" => Some(("identityConstraints.allowSubjectAltNamesPassthrough", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "identity-constraints.allow-subject-passthrough" => Some(("identityConstraints.allowSubjectPassthrough", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "identity-constraints.cel-expression.description" => Some(("identityConstraints.celExpression.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "identity-constraints.cel-expression.expression" => Some(("identityConstraints.celExpression.expression", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "identity-constraints.cel-expression.location" => Some(("identityConstraints.celExpression.location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "identity-constraints.cel-expression.title" => Some(("identityConstraints.celExpression.title", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "maximum-lifetime" => Some(("maximumLifetime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "passthrough-extensions.known-extensions" => Some(("passthroughExtensions.knownExtensions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "predefined-values.aia-ocsp-servers" => Some(("predefinedValues.aiaOcspServers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "predefined-values.ca-options.is-ca" => Some(("predefinedValues.caOptions.isCa", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "predefined-values.ca-options.max-issuer-path-length" => Some(("predefinedValues.caOptions.maxIssuerPathLength", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "predefined-values.key-usage.base-key-usage.cert-sign" => Some(("predefinedValues.keyUsage.baseKeyUsage.certSign", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "predefined-values.key-usage.base-key-usage.content-commitment" => Some(("predefinedValues.keyUsage.baseKeyUsage.contentCommitment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "predefined-values.key-usage.base-key-usage.crl-sign" => Some(("predefinedValues.keyUsage.baseKeyUsage.crlSign", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "predefined-values.key-usage.base-key-usage.data-encipherment" => Some(("predefinedValues.keyUsage.baseKeyUsage.dataEncipherment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "predefined-values.key-usage.base-key-usage.decipher-only" => Some(("predefinedValues.keyUsage.baseKeyUsage.decipherOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "predefined-values.key-usage.base-key-usage.digital-signature" => Some(("predefinedValues.keyUsage.baseKeyUsage.digitalSignature", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "predefined-values.key-usage.base-key-usage.encipher-only" => Some(("predefinedValues.keyUsage.baseKeyUsage.encipherOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "predefined-values.key-usage.base-key-usage.key-agreement" => Some(("predefinedValues.keyUsage.baseKeyUsage.keyAgreement", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "predefined-values.key-usage.base-key-usage.key-encipherment" => Some(("predefinedValues.keyUsage.baseKeyUsage.keyEncipherment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "predefined-values.key-usage.extended-key-usage.client-auth" => Some(("predefinedValues.keyUsage.extendedKeyUsage.clientAuth", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "predefined-values.key-usage.extended-key-usage.code-signing" => Some(("predefinedValues.keyUsage.extendedKeyUsage.codeSigning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "predefined-values.key-usage.extended-key-usage.email-protection" => Some(("predefinedValues.keyUsage.extendedKeyUsage.emailProtection", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "predefined-values.key-usage.extended-key-usage.ocsp-signing" => Some(("predefinedValues.keyUsage.extendedKeyUsage.ocspSigning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "predefined-values.key-usage.extended-key-usage.server-auth" => Some(("predefinedValues.keyUsage.extendedKeyUsage.serverAuth", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "predefined-values.key-usage.extended-key-usage.time-stamping" => Some(("predefinedValues.keyUsage.extendedKeyUsage.timeStamping", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "predefined-values.name-constraints.critical" => Some(("predefinedValues.nameConstraints.critical", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "predefined-values.name-constraints.excluded-dns-names" => Some(("predefinedValues.nameConstraints.excludedDnsNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "predefined-values.name-constraints.excluded-email-addresses" => Some(("predefinedValues.nameConstraints.excludedEmailAddresses", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "predefined-values.name-constraints.excluded-ip-ranges" => Some(("predefinedValues.nameConstraints.excludedIpRanges", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "predefined-values.name-constraints.excluded-uris" => Some(("predefinedValues.nameConstraints.excludedUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "predefined-values.name-constraints.permitted-dns-names" => Some(("predefinedValues.nameConstraints.permittedDnsNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "predefined-values.name-constraints.permitted-email-addresses" => Some(("predefinedValues.nameConstraints.permittedEmailAddresses", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "predefined-values.name-constraints.permitted-ip-ranges" => Some(("predefinedValues.nameConstraints.permittedIpRanges", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "predefined-values.name-constraints.permitted-uris" => Some(("predefinedValues.nameConstraints.permittedUris", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["aia-ocsp-servers", "allow-subject-alt-names-passthrough", "allow-subject-passthrough", "base-key-usage", "ca-options", "cel-expression", "cert-sign", "client-auth", "code-signing", "content-commitment", "create-time", "critical", "crl-sign", "data-encipherment", "decipher-only", "description", "digital-signature", "email-protection", "encipher-only", "excluded-dns-names", "excluded-email-addresses", "excluded-ip-ranges", "excluded-uris", "expression", "extended-key-usage", "identity-constraints", "is-ca", "key-agreement", "key-encipherment", "key-usage", "known-extensions", "labels", "location", "max-issuer-path-length", "maximum-lifetime", "name", "name-constraints", "ocsp-signing", "passthrough-extensions", "permitted-dns-names", "permitted-email-addresses", "permitted-ip-ranges", "permitted-uris", "predefined-values", "server-auth", "time-stamping", "title", "update-time"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CertificateTemplate = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_certificate_templates_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_certificate_templates_set_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.projects().locations_certificate_templates_set_iam_policy(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_locations_certificate_templates_test_iam_permissions(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.projects().locations_certificate_templates_test_iam_permissions(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _doit(&self, dry_run: bool) -> Result<Result<(), DoitError>, Option<InvalidOptionsError>> {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Result<(), DoitError> = Ok(());
        let mut err_opt: Option<InvalidOptionsError> = None;
        match self.opt.subcommand() {
            ("projects", Some(opt)) => {
                match opt.subcommand() {
                    ("locations-ca-pools-certificate-authorities-activate", Some(opt)) => {
                        call_result = self._projects_locations_ca_pools_certificate_authorities_activate(opt, dry_run, &mut err).await;
                    },
                    ("locations-ca-pools-certificate-authorities-certificate-revocation-lists-get", Some(opt)) => {
                        call_result = self._projects_locations_ca_pools_certificate_authorities_certificate_revocation_lists_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-ca-pools-certificate-authorities-certificate-revocation-lists-get-iam-policy", Some(opt)) => {
                        call_result = self._projects_locations_ca_pools_certificate_authorities_certificate_revocation_lists_get_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("locations-ca-pools-certificate-authorities-certificate-revocation-lists-list", Some(opt)) => {
                        call_result = self._projects_locations_ca_pools_certificate_authorities_certificate_revocation_lists_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-ca-pools-certificate-authorities-certificate-revocation-lists-patch", Some(opt)) => {
                        call_result = self._projects_locations_ca_pools_certificate_authorities_certificate_revocation_lists_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-ca-pools-certificate-authorities-certificate-revocation-lists-set-iam-policy", Some(opt)) => {
                        call_result = self._projects_locations_ca_pools_certificate_authorities_certificate_revocation_lists_set_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("locations-ca-pools-certificate-authorities-certificate-revocation-lists-test-iam-permissions", Some(opt)) => {
                        call_result = self._projects_locations_ca_pools_certificate_authorities_certificate_revocation_lists_test_iam_permissions(opt, dry_run, &mut err).await;
                    },
                    ("locations-ca-pools-certificate-authorities-create", Some(opt)) => {
                        call_result = self._projects_locations_ca_pools_certificate_authorities_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-ca-pools-certificate-authorities-delete", Some(opt)) => {
                        call_result = self._projects_locations_ca_pools_certificate_authorities_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-ca-pools-certificate-authorities-disable", Some(opt)) => {
                        call_result = self._projects_locations_ca_pools_certificate_authorities_disable(opt, dry_run, &mut err).await;
                    },
                    ("locations-ca-pools-certificate-authorities-enable", Some(opt)) => {
                        call_result = self._projects_locations_ca_pools_certificate_authorities_enable(opt, dry_run, &mut err).await;
                    },
                    ("locations-ca-pools-certificate-authorities-fetch", Some(opt)) => {
                        call_result = self._projects_locations_ca_pools_certificate_authorities_fetch(opt, dry_run, &mut err).await;
                    },
                    ("locations-ca-pools-certificate-authorities-get", Some(opt)) => {
                        call_result = self._projects_locations_ca_pools_certificate_authorities_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-ca-pools-certificate-authorities-list", Some(opt)) => {
                        call_result = self._projects_locations_ca_pools_certificate_authorities_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-ca-pools-certificate-authorities-patch", Some(opt)) => {
                        call_result = self._projects_locations_ca_pools_certificate_authorities_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-ca-pools-certificate-authorities-undelete", Some(opt)) => {
                        call_result = self._projects_locations_ca_pools_certificate_authorities_undelete(opt, dry_run, &mut err).await;
                    },
                    ("locations-ca-pools-certificates-create", Some(opt)) => {
                        call_result = self._projects_locations_ca_pools_certificates_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-ca-pools-certificates-get", Some(opt)) => {
                        call_result = self._projects_locations_ca_pools_certificates_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-ca-pools-certificates-list", Some(opt)) => {
                        call_result = self._projects_locations_ca_pools_certificates_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-ca-pools-certificates-patch", Some(opt)) => {
                        call_result = self._projects_locations_ca_pools_certificates_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-ca-pools-certificates-revoke", Some(opt)) => {
                        call_result = self._projects_locations_ca_pools_certificates_revoke(opt, dry_run, &mut err).await;
                    },
                    ("locations-ca-pools-create", Some(opt)) => {
                        call_result = self._projects_locations_ca_pools_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-ca-pools-delete", Some(opt)) => {
                        call_result = self._projects_locations_ca_pools_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-ca-pools-fetch-ca-certs", Some(opt)) => {
                        call_result = self._projects_locations_ca_pools_fetch_ca_certs(opt, dry_run, &mut err).await;
                    },
                    ("locations-ca-pools-get", Some(opt)) => {
                        call_result = self._projects_locations_ca_pools_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-ca-pools-get-iam-policy", Some(opt)) => {
                        call_result = self._projects_locations_ca_pools_get_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("locations-ca-pools-list", Some(opt)) => {
                        call_result = self._projects_locations_ca_pools_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-ca-pools-patch", Some(opt)) => {
                        call_result = self._projects_locations_ca_pools_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-ca-pools-set-iam-policy", Some(opt)) => {
                        call_result = self._projects_locations_ca_pools_set_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("locations-ca-pools-test-iam-permissions", Some(opt)) => {
                        call_result = self._projects_locations_ca_pools_test_iam_permissions(opt, dry_run, &mut err).await;
                    },
                    ("locations-certificate-templates-create", Some(opt)) => {
                        call_result = self._projects_locations_certificate_templates_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-certificate-templates-delete", Some(opt)) => {
                        call_result = self._projects_locations_certificate_templates_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-certificate-templates-get", Some(opt)) => {
                        call_result = self._projects_locations_certificate_templates_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-certificate-templates-get-iam-policy", Some(opt)) => {
                        call_result = self._projects_locations_certificate_templates_get_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("locations-certificate-templates-list", Some(opt)) => {
                        call_result = self._projects_locations_certificate_templates_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-certificate-templates-patch", Some(opt)) => {
                        call_result = self._projects_locations_certificate_templates_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-certificate-templates-set-iam-policy", Some(opt)) => {
                        call_result = self._projects_locations_certificate_templates_set_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("locations-certificate-templates-test-iam-permissions", Some(opt)) => {
                        call_result = self._projects_locations_certificate_templates_test_iam_permissions(opt, dry_run, &mut err).await;
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

            match client::application_secret_from_directory(&config_dir, "privateca1-secret.json",
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
        ).persist_tokens_to_disk(format!("{}/privateca1", config_dir)).build().await.unwrap();

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
        ("projects", "methods: 'locations-ca-pools-certificate-authorities-activate', 'locations-ca-pools-certificate-authorities-certificate-revocation-lists-get', 'locations-ca-pools-certificate-authorities-certificate-revocation-lists-get-iam-policy', 'locations-ca-pools-certificate-authorities-certificate-revocation-lists-list', 'locations-ca-pools-certificate-authorities-certificate-revocation-lists-patch', 'locations-ca-pools-certificate-authorities-certificate-revocation-lists-set-iam-policy', 'locations-ca-pools-certificate-authorities-certificate-revocation-lists-test-iam-permissions', 'locations-ca-pools-certificate-authorities-create', 'locations-ca-pools-certificate-authorities-delete', 'locations-ca-pools-certificate-authorities-disable', 'locations-ca-pools-certificate-authorities-enable', 'locations-ca-pools-certificate-authorities-fetch', 'locations-ca-pools-certificate-authorities-get', 'locations-ca-pools-certificate-authorities-list', 'locations-ca-pools-certificate-authorities-patch', 'locations-ca-pools-certificate-authorities-undelete', 'locations-ca-pools-certificates-create', 'locations-ca-pools-certificates-get', 'locations-ca-pools-certificates-list', 'locations-ca-pools-certificates-patch', 'locations-ca-pools-certificates-revoke', 'locations-ca-pools-create', 'locations-ca-pools-delete', 'locations-ca-pools-fetch-ca-certs', 'locations-ca-pools-get', 'locations-ca-pools-get-iam-policy', 'locations-ca-pools-list', 'locations-ca-pools-patch', 'locations-ca-pools-set-iam-policy', 'locations-ca-pools-test-iam-permissions', 'locations-certificate-templates-create', 'locations-certificate-templates-delete', 'locations-certificate-templates-get', 'locations-certificate-templates-get-iam-policy', 'locations-certificate-templates-list', 'locations-certificate-templates-patch', 'locations-certificate-templates-set-iam-policy', 'locations-certificate-templates-test-iam-permissions', 'locations-get', 'locations-list', 'locations-operations-cancel', 'locations-operations-delete', 'locations-operations-get' and 'locations-operations-list'", vec![
            ("locations-ca-pools-certificate-authorities-activate",
                    Some(r##"Activate a CertificateAuthority that is in state AWAITING_USER_ACTIVATION and is of type SUBORDINATE. After the parent Certificate Authority signs a certificate signing request from FetchCertificateAuthorityCsr, this method can complete the activation process."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-ca-pools-certificate-authorities-activate",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name for this CertificateAuthority in the format `projects/*/locations/*/caPools/*/certificateAuthorities/*`."##),
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
            ("locations-ca-pools-certificate-authorities-certificate-revocation-lists-get",
                    Some(r##"Returns a CertificateRevocationList."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-ca-pools-certificate-authorities-certificate-revocation-lists-get",
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
            ("locations-ca-pools-certificate-authorities-certificate-revocation-lists-get-iam-policy",
                    Some(r##"Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-ca-pools-certificate-authorities-certificate-revocation-lists-get-iam-policy",
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
            ("locations-ca-pools-certificate-authorities-certificate-revocation-lists-list",
                    Some(r##"Lists CertificateRevocationLists."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-ca-pools-certificate-authorities-certificate-revocation-lists-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The resource name of the location associated with the CertificateRevocationLists, in the format `projects/*/locations/*/caPools/*/certificateAuthorities/*`."##),
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
            ("locations-ca-pools-certificate-authorities-certificate-revocation-lists-patch",
                    Some(r##"Update a CertificateRevocationList."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-ca-pools-certificate-authorities-certificate-revocation-lists-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Output only. The resource name for this CertificateRevocationList in the format `projects/*/locations/*/caPools/*certificateAuthorities/*/ certificateRevocationLists/*`."##),
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
            ("locations-ca-pools-certificate-authorities-certificate-revocation-lists-set-iam-policy",
                    Some(r##"Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-ca-pools-certificate-authorities-certificate-revocation-lists-set-iam-policy",
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
            ("locations-ca-pools-certificate-authorities-certificate-revocation-lists-test-iam-permissions",
                    Some(r##"Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-ca-pools-certificate-authorities-certificate-revocation-lists-test-iam-permissions",
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
            ("locations-ca-pools-certificate-authorities-create",
                    Some(r##"Create a new CertificateAuthority in a given Project and Location."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-ca-pools-certificate-authorities-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The resource name of the CaPool associated with the CertificateAuthorities, in the format `projects/*/locations/*/caPools/*`."##),
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
            ("locations-ca-pools-certificate-authorities-delete",
                    Some(r##"Delete a CertificateAuthority."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-ca-pools-certificate-authorities-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name for this CertificateAuthority in the format `projects/*/locations/*/caPools/*/certificateAuthorities/*`."##),
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
            ("locations-ca-pools-certificate-authorities-disable",
                    Some(r##"Disable a CertificateAuthority."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-ca-pools-certificate-authorities-disable",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name for this CertificateAuthority in the format `projects/*/locations/*/caPools/*/certificateAuthorities/*`."##),
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
            ("locations-ca-pools-certificate-authorities-enable",
                    Some(r##"Enable a CertificateAuthority."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-ca-pools-certificate-authorities-enable",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name for this CertificateAuthority in the format `projects/*/locations/*/caPools/*/certificateAuthorities/*`."##),
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
            ("locations-ca-pools-certificate-authorities-fetch",
                    Some(r##"Fetch a certificate signing request (CSR) from a CertificateAuthority that is in state AWAITING_USER_ACTIVATION and is of type SUBORDINATE. The CSR must then be signed by the desired parent Certificate Authority, which could be another CertificateAuthority resource, or could be an on-prem certificate authority. See also ActivateCertificateAuthority."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-ca-pools-certificate-authorities-fetch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name for this CertificateAuthority in the format `projects/*/locations/*/caPools/*/certificateAuthorities/*`."##),
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
            ("locations-ca-pools-certificate-authorities-get",
                    Some(r##"Returns a CertificateAuthority."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-ca-pools-certificate-authorities-get",
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
            ("locations-ca-pools-certificate-authorities-list",
                    Some(r##"Lists CertificateAuthorities."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-ca-pools-certificate-authorities-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The resource name of the CaPool associated with the CertificateAuthorities, in the format `projects/*/locations/*/caPools/*`."##),
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
            ("locations-ca-pools-certificate-authorities-patch",
                    Some(r##"Update a CertificateAuthority."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-ca-pools-certificate-authorities-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Output only. The resource name for this CertificateAuthority in the format `projects/*/locations/*/caPools/*/certificateAuthorities/*`."##),
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
            ("locations-ca-pools-certificate-authorities-undelete",
                    Some(r##"Undelete a CertificateAuthority that has been deleted."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-ca-pools-certificate-authorities-undelete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name for this CertificateAuthority in the format `projects/*/locations/*/caPools/*/certificateAuthorities/*`."##),
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
            ("locations-ca-pools-certificates-create",
                    Some(r##"Create a new Certificate in a given Project, Location from a particular CaPool."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-ca-pools-certificates-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The resource name of the CaPool associated with the Certificate, in the format `projects/*/locations/*/caPools/*`."##),
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
            ("locations-ca-pools-certificates-get",
                    Some(r##"Returns a Certificate."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-ca-pools-certificates-get",
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
            ("locations-ca-pools-certificates-list",
                    Some(r##"Lists Certificates."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-ca-pools-certificates-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The resource name of the location associated with the Certificates, in the format `projects/*/locations/*/caPools/*`."##),
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
            ("locations-ca-pools-certificates-patch",
                    Some(r##"Update a Certificate. Currently, the only field you can update is the labels field."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-ca-pools-certificates-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Output only. The resource name for this Certificate in the format `projects/*/locations/*/caPools/*/certificates/*`."##),
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
            ("locations-ca-pools-certificates-revoke",
                    Some(r##"Revoke a Certificate."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-ca-pools-certificates-revoke",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name for this Certificate in the format `projects/*/locations/*/caPools/*/certificates/*`."##),
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
            ("locations-ca-pools-create",
                    Some(r##"Create a CaPool."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-ca-pools-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The resource name of the location associated with the CaPool, in the format `projects/*/locations/*`."##),
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
            ("locations-ca-pools-delete",
                    Some(r##"Delete a CaPool."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-ca-pools-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name for this CaPool in the format `projects/*/locations/*/caPools/*`."##),
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
            ("locations-ca-pools-fetch-ca-certs",
                    Some(r##"FetchCaCerts returns the current trust anchor for the CaPool. This will include CA certificate chains for all certificate authorities in the ENABLED, DISABLED, or STAGED states."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-ca-pools-fetch-ca-certs",
                  vec![
                    (Some(r##"ca-pool"##),
                     None,
                     Some(r##"Required. The resource name for the CaPool in the format `projects/*/locations/*/caPools/*`."##),
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
            ("locations-ca-pools-get",
                    Some(r##"Returns a CaPool."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-ca-pools-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the CaPool to get."##),
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
            ("locations-ca-pools-get-iam-policy",
                    Some(r##"Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-ca-pools-get-iam-policy",
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
            ("locations-ca-pools-list",
                    Some(r##"Lists CaPools."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-ca-pools-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The resource name of the location associated with the CaPools, in the format `projects/*/locations/*`."##),
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
            ("locations-ca-pools-patch",
                    Some(r##"Update a CaPool."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-ca-pools-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Output only. The resource name for this CaPool in the format `projects/*/locations/*/caPools/*`."##),
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
            ("locations-ca-pools-set-iam-policy",
                    Some(r##"Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-ca-pools-set-iam-policy",
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
            ("locations-ca-pools-test-iam-permissions",
                    Some(r##"Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-ca-pools-test-iam-permissions",
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
            ("locations-certificate-templates-create",
                    Some(r##"Create a new CertificateTemplate in a given Project and Location."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-certificate-templates-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The resource name of the location associated with the CertificateTemplate, in the format `projects/*/locations/*`."##),
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
            ("locations-certificate-templates-delete",
                    Some(r##"DeleteCertificateTemplate deletes a CertificateTemplate."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-certificate-templates-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name for this CertificateTemplate in the format `projects/*/locations/*/certificateTemplates/*`."##),
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
            ("locations-certificate-templates-get",
                    Some(r##"Returns a CertificateTemplate."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-certificate-templates-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the CertificateTemplate to get."##),
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
            ("locations-certificate-templates-get-iam-policy",
                    Some(r##"Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-certificate-templates-get-iam-policy",
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
            ("locations-certificate-templates-list",
                    Some(r##"Lists CertificateTemplates."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-certificate-templates-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The resource name of the location associated with the CertificateTemplates, in the format `projects/*/locations/*`."##),
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
            ("locations-certificate-templates-patch",
                    Some(r##"Update a CertificateTemplate."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-certificate-templates-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Output only. The resource name for this CertificateTemplate in the format `projects/*/locations/*/certificateTemplates/*`."##),
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
            ("locations-certificate-templates-set-iam-policy",
                    Some(r##"Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-certificate-templates-set-iam-policy",
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
            ("locations-certificate-templates-test-iam-permissions",
                    Some(r##"Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-certificate-templates-test-iam-permissions",
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
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-get",
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
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-list",
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
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-operations-cancel",
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
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-operations-delete",
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
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-operations-get",
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
                    Some(r##"Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_privateca1_cli/projects_locations-operations-list",
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
            ]),
        
    ];
    
    let mut app = App::new("privateca1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+20240605")
           .about("The Certificate Authority Service API is a highly-available, scalable service that enables you to simplify and automate the management of private certificate authorities (CAs) while staying in control of your private keys. ")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_privateca1_cli")
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
