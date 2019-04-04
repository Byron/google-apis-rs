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
extern crate google_appengine1 as api;

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
    hub: api::Appengine<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n> Engine<'n> {
    fn _apps_authorized_certificates_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "domain-mappings-count" => Some(("domainMappingsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "visible-domain-mappings" => Some(("visibleDomainMappings", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "expire-time" => Some(("expireTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-raw-data.private-key" => Some(("certificateRawData.privateKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-raw-data.public-certificate" => Some(("certificateRawData.publicCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "managed-certificate.status" => Some(("managedCertificate.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "managed-certificate.last-renewal-time" => Some(("managedCertificate.lastRenewalTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "domain-names" => Some(("domainNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["certificate-raw-data", "display-name", "domain-mappings-count", "domain-names", "expire-time", "id", "last-renewal-time", "managed-certificate", "name", "private-key", "public-certificate", "status", "visible-domain-mappings"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::AuthorizedCertificate = json::value::from_value(object).unwrap();
        let mut call = self.hub.apps().authorized_certificates_create(request, opt.value_of("apps-id").unwrap_or(""));
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

    fn _apps_authorized_certificates_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().authorized_certificates_delete(opt.value_of("apps-id").unwrap_or(""), opt.value_of("authorized-certificates-id").unwrap_or(""));
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

    fn _apps_authorized_certificates_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().authorized_certificates_get(opt.value_of("apps-id").unwrap_or(""), opt.value_of("authorized-certificates-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "view" => {
                    call = call.view(value.unwrap_or(""));
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
                                                                           v.extend(["view"].iter().map(|v|*v));
                                                                           v } ));
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

    fn _apps_authorized_certificates_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().authorized_certificates_list(opt.value_of("apps-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "view" => {
                    call = call.view(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
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
                                                                           v.extend(["page-token", "page-size", "view"].iter().map(|v|*v));
                                                                           v } ));
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

    fn _apps_authorized_certificates_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "domain-mappings-count" => Some(("domainMappingsCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "visible-domain-mappings" => Some(("visibleDomainMappings", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "expire-time" => Some(("expireTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-raw-data.private-key" => Some(("certificateRawData.privateKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "certificate-raw-data.public-certificate" => Some(("certificateRawData.publicCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "managed-certificate.status" => Some(("managedCertificate.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "managed-certificate.last-renewal-time" => Some(("managedCertificate.lastRenewalTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "domain-names" => Some(("domainNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["certificate-raw-data", "display-name", "domain-mappings-count", "domain-names", "expire-time", "id", "last-renewal-time", "managed-certificate", "name", "private-key", "public-certificate", "status", "visible-domain-mappings"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::AuthorizedCertificate = json::value::from_value(object).unwrap();
        let mut call = self.hub.apps().authorized_certificates_patch(request, opt.value_of("apps-id").unwrap_or(""), opt.value_of("authorized-certificates-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(value.unwrap_or(""));
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

    fn _apps_authorized_domains_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().authorized_domains_list(opt.value_of("apps-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
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
                                                                           v.extend(["page-token", "page-size"].iter().map(|v|*v));
                                                                           v } ));
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

    fn _apps_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "default-hostname" => Some(("defaultHostname", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "code-bucket" => Some(("codeBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-bucket" => Some(("defaultBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-cookie-expiration" => Some(("defaultCookieExpiration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "iap.oauth2-client-id" => Some(("iap.oauth2ClientId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "iap.enabled" => Some(("iap.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "iap.oauth2-client-secret" => Some(("iap.oauth2ClientSecret", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "iap.oauth2-client-secret-sha256" => Some(("iap.oauth2ClientSecretSha256", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-id" => Some(("locationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "feature-settings.use-container-optimized-os" => Some(("featureSettings.useContainerOptimizedOs", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "feature-settings.split-health-checks" => Some(("featureSettings.splitHealthChecks", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "serving-status" => Some(("servingStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "auth-domain" => Some(("authDomain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gcr-domain" => Some(("gcrDomain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["auth-domain", "code-bucket", "default-bucket", "default-cookie-expiration", "default-hostname", "enabled", "feature-settings", "gcr-domain", "iap", "id", "location-id", "name", "oauth2-client-id", "oauth2-client-secret", "oauth2-client-secret-sha256", "serving-status", "split-health-checks", "use-container-optimized-os"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Application = json::value::from_value(object).unwrap();
        let mut call = self.hub.apps().create(request);
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

    fn _apps_domain_mappings_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "ssl-settings.certificate-id" => Some(("sslSettings.certificateId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ssl-settings.pending-managed-certificate-id" => Some(("sslSettings.pendingManagedCertificateId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ssl-settings.ssl-management-type" => Some(("sslSettings.sslManagementType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["certificate-id", "id", "name", "pending-managed-certificate-id", "ssl-management-type", "ssl-settings"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::DomainMapping = json::value::from_value(object).unwrap();
        let mut call = self.hub.apps().domain_mappings_create(request, opt.value_of("apps-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "override-strategy" => {
                    call = call.override_strategy(value.unwrap_or(""));
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
                                                                           v.extend(["override-strategy"].iter().map(|v|*v));
                                                                           v } ));
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

    fn _apps_domain_mappings_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().domain_mappings_delete(opt.value_of("apps-id").unwrap_or(""), opt.value_of("domain-mappings-id").unwrap_or(""));
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

    fn _apps_domain_mappings_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().domain_mappings_get(opt.value_of("apps-id").unwrap_or(""), opt.value_of("domain-mappings-id").unwrap_or(""));
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

    fn _apps_domain_mappings_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().domain_mappings_list(opt.value_of("apps-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
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
                                                                           v.extend(["page-token", "page-size"].iter().map(|v|*v));
                                                                           v } ));
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

    fn _apps_domain_mappings_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "ssl-settings.certificate-id" => Some(("sslSettings.certificateId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ssl-settings.pending-managed-certificate-id" => Some(("sslSettings.pendingManagedCertificateId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ssl-settings.ssl-management-type" => Some(("sslSettings.sslManagementType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["certificate-id", "id", "name", "pending-managed-certificate-id", "ssl-management-type", "ssl-settings"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::DomainMapping = json::value::from_value(object).unwrap();
        let mut call = self.hub.apps().domain_mappings_patch(request, opt.value_of("apps-id").unwrap_or(""), opt.value_of("domain-mappings-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(value.unwrap_or(""));
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

    fn _apps_firewall_ingress_rules_batch_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::BatchUpdateIngressRulesRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.apps().firewall_ingress_rules_batch_update(request, opt.value_of("apps-id").unwrap_or(""));
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

    fn _apps_firewall_ingress_rules_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "action" => Some(("action", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "priority" => Some(("priority", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-range" => Some(("sourceRange", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["action", "description", "priority", "source-range"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::FirewallRule = json::value::from_value(object).unwrap();
        let mut call = self.hub.apps().firewall_ingress_rules_create(request, opt.value_of("apps-id").unwrap_or(""));
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

    fn _apps_firewall_ingress_rules_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().firewall_ingress_rules_delete(opt.value_of("apps-id").unwrap_or(""), opt.value_of("ingress-rules-id").unwrap_or(""));
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

    fn _apps_firewall_ingress_rules_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().firewall_ingress_rules_get(opt.value_of("apps-id").unwrap_or(""), opt.value_of("ingress-rules-id").unwrap_or(""));
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

    fn _apps_firewall_ingress_rules_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().firewall_ingress_rules_list(opt.value_of("apps-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
                },
                "matching-address" => {
                    call = call.matching_address(value.unwrap_or(""));
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
                                                                           v.extend(["matching-address", "page-token", "page-size"].iter().map(|v|*v));
                                                                           v } ));
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

    fn _apps_firewall_ingress_rules_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "action" => Some(("action", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "priority" => Some(("priority", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "source-range" => Some(("sourceRange", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["action", "description", "priority", "source-range"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::FirewallRule = json::value::from_value(object).unwrap();
        let mut call = self.hub.apps().firewall_ingress_rules_patch(request, opt.value_of("apps-id").unwrap_or(""), opt.value_of("ingress-rules-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(value.unwrap_or(""));
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

    fn _apps_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().get(opt.value_of("apps-id").unwrap_or(""));
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

    fn _apps_locations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().locations_get(opt.value_of("apps-id").unwrap_or(""), opt.value_of("locations-id").unwrap_or(""));
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

    fn _apps_locations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().locations_list(opt.value_of("apps-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
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
                                                                           v.extend(["filter", "page-token", "page-size"].iter().map(|v|*v));
                                                                           v } ));
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

    fn _apps_operations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().operations_get(opt.value_of("apps-id").unwrap_or(""), opt.value_of("operations-id").unwrap_or(""));
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

    fn _apps_operations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().operations_list(opt.value_of("apps-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
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
                                                                           v.extend(["filter", "page-token", "page-size"].iter().map(|v|*v));
                                                                           v } ));
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

    fn _apps_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "default-hostname" => Some(("defaultHostname", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "code-bucket" => Some(("codeBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-bucket" => Some(("defaultBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-cookie-expiration" => Some(("defaultCookieExpiration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "iap.oauth2-client-id" => Some(("iap.oauth2ClientId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "iap.enabled" => Some(("iap.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "iap.oauth2-client-secret" => Some(("iap.oauth2ClientSecret", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "iap.oauth2-client-secret-sha256" => Some(("iap.oauth2ClientSecretSha256", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-id" => Some(("locationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "feature-settings.use-container-optimized-os" => Some(("featureSettings.useContainerOptimizedOs", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "feature-settings.split-health-checks" => Some(("featureSettings.splitHealthChecks", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "serving-status" => Some(("servingStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "auth-domain" => Some(("authDomain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "gcr-domain" => Some(("gcrDomain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["auth-domain", "code-bucket", "default-bucket", "default-cookie-expiration", "default-hostname", "enabled", "feature-settings", "gcr-domain", "iap", "id", "location-id", "name", "oauth2-client-id", "oauth2-client-secret", "oauth2-client-secret-sha256", "serving-status", "split-health-checks", "use-container-optimized-os"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Application = json::value::from_value(object).unwrap();
        let mut call = self.hub.apps().patch(request, opt.value_of("apps-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(value.unwrap_or(""));
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

    fn _apps_repair(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::RepairApplicationRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.apps().repair(request, opt.value_of("apps-id").unwrap_or(""));
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

    fn _apps_services_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().services_delete(opt.value_of("apps-id").unwrap_or(""), opt.value_of("services-id").unwrap_or(""));
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

    fn _apps_services_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().services_get(opt.value_of("apps-id").unwrap_or(""), opt.value_of("services-id").unwrap_or(""));
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

    fn _apps_services_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().services_list(opt.value_of("apps-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
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
                                                                           v.extend(["page-token", "page-size"].iter().map(|v|*v));
                                                                           v } ));
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

    fn _apps_services_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "split.shard-by" => Some(("split.shardBy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "split.allocations" => Some(("split.allocations", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Map })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["allocations", "id", "name", "shard-by", "split"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Service = json::value::from_value(object).unwrap();
        let mut call = self.hub.apps().services_patch(request, opt.value_of("apps-id").unwrap_or(""), opt.value_of("services-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(value.unwrap_or(""));
                },
                "migrate-traffic" => {
                    call = call.migrate_traffic(arg_from_str(value.unwrap_or("false"), err, "migrate-traffic", "boolean"));
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
                                                                           v.extend(["migrate-traffic", "update-mask"].iter().map(|v|*v));
                                                                           v } ));
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

    fn _apps_services_versions_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "endpoints-api-service.config-id" => Some(("endpointsApiService.configId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "endpoints-api-service.rollout-strategy" => Some(("endpointsApiService.rolloutStrategy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "endpoints-api-service.disable-trace-sampling" => Some(("endpointsApiService.disableTraceSampling", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "endpoints-api-service.name" => Some(("endpointsApiService.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "basic-scaling.idle-timeout" => Some(("basicScaling.idleTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "basic-scaling.max-instances" => Some(("basicScaling.maxInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "beta-settings" => Some(("betaSettings", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "vm" => Some(("vm", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "zones" => Some(("zones", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "instance-class" => Some(("instanceClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "readiness-check.check-interval" => Some(("readinessCheck.checkInterval", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "readiness-check.host" => Some(("readinessCheck.host", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "readiness-check.app-start-timeout" => Some(("readinessCheck.appStartTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "readiness-check.timeout" => Some(("readinessCheck.timeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "readiness-check.path" => Some(("readinessCheck.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "readiness-check.success-threshold" => Some(("readinessCheck.successThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "readiness-check.failure-threshold" => Some(("readinessCheck.failureThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "runtime-main-executable-path" => Some(("runtimeMainExecutablePath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-config.url" => Some(("apiConfig.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-config.security-level" => Some(("apiConfig.securityLevel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-config.auth-fail-action" => Some(("apiConfig.authFailAction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-config.login" => Some(("apiConfig.login", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-config.script" => Some(("apiConfig.script", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "network.instance-tag" => Some(("network.instanceTag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "network.forwarded-ports" => Some(("network.forwardedPorts", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "network.subnetwork-name" => Some(("network.subnetworkName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "network.name" => Some(("network.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "network.session-affinity" => Some(("network.sessionAffinity", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "health-check.restart-threshold" => Some(("healthCheck.restartThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "health-check.check-interval" => Some(("healthCheck.checkInterval", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-check.unhealthy-threshold" => Some(("healthCheck.unhealthyThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "health-check.healthy-threshold" => Some(("healthCheck.healthyThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "health-check.host" => Some(("healthCheck.host", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-check.timeout" => Some(("healthCheck.timeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-check.disable-health-check" => Some(("healthCheck.disableHealthCheck", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "default-expiration" => Some(("defaultExpiration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "version-url" => Some(("versionUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entrypoint.shell" => Some(("entrypoint.shell", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "env" => Some(("env", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "disk-usage-bytes" => Some(("diskUsageBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "automatic-scaling.standard-scheduler-settings.min-instances" => Some(("automaticScaling.standardSchedulerSettings.minInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.standard-scheduler-settings.target-cpu-utilization" => Some(("automaticScaling.standardSchedulerSettings.targetCpuUtilization", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "automatic-scaling.standard-scheduler-settings.target-throughput-utilization" => Some(("automaticScaling.standardSchedulerSettings.targetThroughputUtilization", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "automatic-scaling.standard-scheduler-settings.max-instances" => Some(("automaticScaling.standardSchedulerSettings.maxInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.min-pending-latency" => Some(("automaticScaling.minPendingLatency", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "automatic-scaling.network-utilization.target-received-packets-per-second" => Some(("automaticScaling.networkUtilization.targetReceivedPacketsPerSecond", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.network-utilization.target-sent-packets-per-second" => Some(("automaticScaling.networkUtilization.targetSentPacketsPerSecond", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.network-utilization.target-received-bytes-per-second" => Some(("automaticScaling.networkUtilization.targetReceivedBytesPerSecond", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.network-utilization.target-sent-bytes-per-second" => Some(("automaticScaling.networkUtilization.targetSentBytesPerSecond", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.min-idle-instances" => Some(("automaticScaling.minIdleInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.max-pending-latency" => Some(("automaticScaling.maxPendingLatency", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "automatic-scaling.max-idle-instances" => Some(("automaticScaling.maxIdleInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.disk-utilization.target-read-ops-per-second" => Some(("automaticScaling.diskUtilization.targetReadOpsPerSecond", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.disk-utilization.target-write-bytes-per-second" => Some(("automaticScaling.diskUtilization.targetWriteBytesPerSecond", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.disk-utilization.target-read-bytes-per-second" => Some(("automaticScaling.diskUtilization.targetReadBytesPerSecond", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.disk-utilization.target-write-ops-per-second" => Some(("automaticScaling.diskUtilization.targetWriteOpsPerSecond", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.request-utilization.target-concurrent-requests" => Some(("automaticScaling.requestUtilization.targetConcurrentRequests", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.request-utilization.target-request-count-per-second" => Some(("automaticScaling.requestUtilization.targetRequestCountPerSecond", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.cool-down-period" => Some(("automaticScaling.coolDownPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "automatic-scaling.max-total-instances" => Some(("automaticScaling.maxTotalInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.max-concurrent-requests" => Some(("automaticScaling.maxConcurrentRequests", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.min-total-instances" => Some(("automaticScaling.minTotalInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.cpu-utilization.target-utilization" => Some(("automaticScaling.cpuUtilization.targetUtilization", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "automatic-scaling.cpu-utilization.aggregation-window-length" => Some(("automaticScaling.cpuUtilization.aggregationWindowLength", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "env-variables" => Some(("envVariables", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "resources.disk-gb" => Some(("resources.diskGb", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "resources.cpu" => Some(("resources.cpu", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "resources.memory-gb" => Some(("resources.memoryGb", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "manual-scaling.instances" => Some(("manualScaling.instances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "vpc-access-connector.name" => Some(("vpcAccessConnector.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inbound-services" => Some(("inboundServices", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "runtime-channel" => Some(("runtimeChannel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created-by" => Some(("createdBy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment.cloud-build-options.app-yaml-path" => Some(("deployment.cloudBuildOptions.appYamlPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment.cloud-build-options.cloud-build-timeout" => Some(("deployment.cloudBuildOptions.cloudBuildTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment.container.image" => Some(("deployment.container.image", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment.zip.files-count" => Some(("deployment.zip.filesCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "deployment.zip.source-url" => Some(("deployment.zip.sourceUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nobuild-files-regex" => Some(("nobuildFilesRegex", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "threadsafe" => Some(("threadsafe", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "runtime-api-version" => Some(("runtimeApiVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "liveness-check.check-interval" => Some(("livenessCheck.checkInterval", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "liveness-check.initial-delay" => Some(("livenessCheck.initialDelay", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "liveness-check.host" => Some(("livenessCheck.host", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "liveness-check.timeout" => Some(("livenessCheck.timeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "liveness-check.path" => Some(("livenessCheck.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "liveness-check.success-threshold" => Some(("livenessCheck.successThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "liveness-check.failure-threshold" => Some(("livenessCheck.failureThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "serving-status" => Some(("servingStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime" => Some(("runtime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["aggregation-window-length", "api-config", "app-start-timeout", "app-yaml-path", "auth-fail-action", "automatic-scaling", "basic-scaling", "beta-settings", "check-interval", "cloud-build-options", "cloud-build-timeout", "config-id", "container", "cool-down-period", "cpu", "cpu-utilization", "create-time", "created-by", "default-expiration", "deployment", "disable-health-check", "disable-trace-sampling", "disk-gb", "disk-usage-bytes", "disk-utilization", "endpoints-api-service", "entrypoint", "env", "env-variables", "failure-threshold", "files-count", "forwarded-ports", "health-check", "healthy-threshold", "host", "id", "idle-timeout", "image", "inbound-services", "initial-delay", "instance-class", "instance-tag", "instances", "liveness-check", "login", "manual-scaling", "max-concurrent-requests", "max-idle-instances", "max-instances", "max-pending-latency", "max-total-instances", "memory-gb", "min-idle-instances", "min-instances", "min-pending-latency", "min-total-instances", "name", "network", "network-utilization", "nobuild-files-regex", "path", "readiness-check", "request-utilization", "resources", "restart-threshold", "rollout-strategy", "runtime", "runtime-api-version", "runtime-channel", "runtime-main-executable-path", "script", "security-level", "serving-status", "session-affinity", "shell", "source-url", "standard-scheduler-settings", "subnetwork-name", "success-threshold", "target-concurrent-requests", "target-cpu-utilization", "target-read-bytes-per-second", "target-read-ops-per-second", "target-received-bytes-per-second", "target-received-packets-per-second", "target-request-count-per-second", "target-sent-bytes-per-second", "target-sent-packets-per-second", "target-throughput-utilization", "target-utilization", "target-write-bytes-per-second", "target-write-ops-per-second", "threadsafe", "timeout", "unhealthy-threshold", "url", "version-url", "vm", "vpc-access-connector", "zip", "zones"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Version = json::value::from_value(object).unwrap();
        let mut call = self.hub.apps().services_versions_create(request, opt.value_of("apps-id").unwrap_or(""), opt.value_of("services-id").unwrap_or(""));
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

    fn _apps_services_versions_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().services_versions_delete(opt.value_of("apps-id").unwrap_or(""), opt.value_of("services-id").unwrap_or(""), opt.value_of("versions-id").unwrap_or(""));
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

    fn _apps_services_versions_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().services_versions_get(opt.value_of("apps-id").unwrap_or(""), opt.value_of("services-id").unwrap_or(""), opt.value_of("versions-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "view" => {
                    call = call.view(value.unwrap_or(""));
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
                                                                           v.extend(["view"].iter().map(|v|*v));
                                                                           v } ));
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

    fn _apps_services_versions_instances_debug(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "ssh-key" => Some(("sshKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["ssh-key"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::DebugInstanceRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.apps().services_versions_instances_debug(request, opt.value_of("apps-id").unwrap_or(""), opt.value_of("services-id").unwrap_or(""), opt.value_of("versions-id").unwrap_or(""), opt.value_of("instances-id").unwrap_or(""));
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

    fn _apps_services_versions_instances_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().services_versions_instances_delete(opt.value_of("apps-id").unwrap_or(""), opt.value_of("services-id").unwrap_or(""), opt.value_of("versions-id").unwrap_or(""), opt.value_of("instances-id").unwrap_or(""));
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

    fn _apps_services_versions_instances_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().services_versions_instances_get(opt.value_of("apps-id").unwrap_or(""), opt.value_of("services-id").unwrap_or(""), opt.value_of("versions-id").unwrap_or(""), opt.value_of("instances-id").unwrap_or(""));
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

    fn _apps_services_versions_instances_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().services_versions_instances_list(opt.value_of("apps-id").unwrap_or(""), opt.value_of("services-id").unwrap_or(""), opt.value_of("versions-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
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
                                                                           v.extend(["page-token", "page-size"].iter().map(|v|*v));
                                                                           v } ));
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

    fn _apps_services_versions_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().services_versions_list(opt.value_of("apps-id").unwrap_or(""), opt.value_of("services-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "view" => {
                    call = call.view(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
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
                                                                           v.extend(["page-token", "page-size", "view"].iter().map(|v|*v));
                                                                           v } ));
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

    fn _apps_services_versions_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "endpoints-api-service.config-id" => Some(("endpointsApiService.configId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "endpoints-api-service.rollout-strategy" => Some(("endpointsApiService.rolloutStrategy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "endpoints-api-service.disable-trace-sampling" => Some(("endpointsApiService.disableTraceSampling", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "endpoints-api-service.name" => Some(("endpointsApiService.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "basic-scaling.idle-timeout" => Some(("basicScaling.idleTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "basic-scaling.max-instances" => Some(("basicScaling.maxInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "beta-settings" => Some(("betaSettings", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "vm" => Some(("vm", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "zones" => Some(("zones", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "instance-class" => Some(("instanceClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "readiness-check.check-interval" => Some(("readinessCheck.checkInterval", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "readiness-check.host" => Some(("readinessCheck.host", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "readiness-check.app-start-timeout" => Some(("readinessCheck.appStartTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "readiness-check.timeout" => Some(("readinessCheck.timeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "readiness-check.path" => Some(("readinessCheck.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "readiness-check.success-threshold" => Some(("readinessCheck.successThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "readiness-check.failure-threshold" => Some(("readinessCheck.failureThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "runtime-main-executable-path" => Some(("runtimeMainExecutablePath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-config.url" => Some(("apiConfig.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-config.security-level" => Some(("apiConfig.securityLevel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-config.auth-fail-action" => Some(("apiConfig.authFailAction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-config.login" => Some(("apiConfig.login", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-config.script" => Some(("apiConfig.script", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "network.instance-tag" => Some(("network.instanceTag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "network.forwarded-ports" => Some(("network.forwardedPorts", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "network.subnetwork-name" => Some(("network.subnetworkName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "network.name" => Some(("network.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "network.session-affinity" => Some(("network.sessionAffinity", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "health-check.restart-threshold" => Some(("healthCheck.restartThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "health-check.check-interval" => Some(("healthCheck.checkInterval", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-check.unhealthy-threshold" => Some(("healthCheck.unhealthyThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "health-check.healthy-threshold" => Some(("healthCheck.healthyThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "health-check.host" => Some(("healthCheck.host", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-check.timeout" => Some(("healthCheck.timeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-check.disable-health-check" => Some(("healthCheck.disableHealthCheck", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "default-expiration" => Some(("defaultExpiration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "version-url" => Some(("versionUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "entrypoint.shell" => Some(("entrypoint.shell", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "env" => Some(("env", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "disk-usage-bytes" => Some(("diskUsageBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "automatic-scaling.standard-scheduler-settings.min-instances" => Some(("automaticScaling.standardSchedulerSettings.minInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.standard-scheduler-settings.target-cpu-utilization" => Some(("automaticScaling.standardSchedulerSettings.targetCpuUtilization", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "automatic-scaling.standard-scheduler-settings.target-throughput-utilization" => Some(("automaticScaling.standardSchedulerSettings.targetThroughputUtilization", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "automatic-scaling.standard-scheduler-settings.max-instances" => Some(("automaticScaling.standardSchedulerSettings.maxInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.min-pending-latency" => Some(("automaticScaling.minPendingLatency", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "automatic-scaling.network-utilization.target-received-packets-per-second" => Some(("automaticScaling.networkUtilization.targetReceivedPacketsPerSecond", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.network-utilization.target-sent-packets-per-second" => Some(("automaticScaling.networkUtilization.targetSentPacketsPerSecond", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.network-utilization.target-received-bytes-per-second" => Some(("automaticScaling.networkUtilization.targetReceivedBytesPerSecond", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.network-utilization.target-sent-bytes-per-second" => Some(("automaticScaling.networkUtilization.targetSentBytesPerSecond", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.min-idle-instances" => Some(("automaticScaling.minIdleInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.max-pending-latency" => Some(("automaticScaling.maxPendingLatency", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "automatic-scaling.max-idle-instances" => Some(("automaticScaling.maxIdleInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.disk-utilization.target-read-ops-per-second" => Some(("automaticScaling.diskUtilization.targetReadOpsPerSecond", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.disk-utilization.target-write-bytes-per-second" => Some(("automaticScaling.diskUtilization.targetWriteBytesPerSecond", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.disk-utilization.target-read-bytes-per-second" => Some(("automaticScaling.diskUtilization.targetReadBytesPerSecond", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.disk-utilization.target-write-ops-per-second" => Some(("automaticScaling.diskUtilization.targetWriteOpsPerSecond", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.request-utilization.target-concurrent-requests" => Some(("automaticScaling.requestUtilization.targetConcurrentRequests", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.request-utilization.target-request-count-per-second" => Some(("automaticScaling.requestUtilization.targetRequestCountPerSecond", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.cool-down-period" => Some(("automaticScaling.coolDownPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "automatic-scaling.max-total-instances" => Some(("automaticScaling.maxTotalInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.max-concurrent-requests" => Some(("automaticScaling.maxConcurrentRequests", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.min-total-instances" => Some(("automaticScaling.minTotalInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.cpu-utilization.target-utilization" => Some(("automaticScaling.cpuUtilization.targetUtilization", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "automatic-scaling.cpu-utilization.aggregation-window-length" => Some(("automaticScaling.cpuUtilization.aggregationWindowLength", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "env-variables" => Some(("envVariables", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "resources.disk-gb" => Some(("resources.diskGb", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "resources.cpu" => Some(("resources.cpu", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "resources.memory-gb" => Some(("resources.memoryGb", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "manual-scaling.instances" => Some(("manualScaling.instances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "vpc-access-connector.name" => Some(("vpcAccessConnector.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "inbound-services" => Some(("inboundServices", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "runtime-channel" => Some(("runtimeChannel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "created-by" => Some(("createdBy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment.cloud-build-options.app-yaml-path" => Some(("deployment.cloudBuildOptions.appYamlPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment.cloud-build-options.cloud-build-timeout" => Some(("deployment.cloudBuildOptions.cloudBuildTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment.container.image" => Some(("deployment.container.image", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment.zip.files-count" => Some(("deployment.zip.filesCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "deployment.zip.source-url" => Some(("deployment.zip.sourceUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nobuild-files-regex" => Some(("nobuildFilesRegex", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "threadsafe" => Some(("threadsafe", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "runtime-api-version" => Some(("runtimeApiVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "liveness-check.check-interval" => Some(("livenessCheck.checkInterval", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "liveness-check.initial-delay" => Some(("livenessCheck.initialDelay", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "liveness-check.host" => Some(("livenessCheck.host", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "liveness-check.timeout" => Some(("livenessCheck.timeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "liveness-check.path" => Some(("livenessCheck.path", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "liveness-check.success-threshold" => Some(("livenessCheck.successThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "liveness-check.failure-threshold" => Some(("livenessCheck.failureThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "serving-status" => Some(("servingStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime" => Some(("runtime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["aggregation-window-length", "api-config", "app-start-timeout", "app-yaml-path", "auth-fail-action", "automatic-scaling", "basic-scaling", "beta-settings", "check-interval", "cloud-build-options", "cloud-build-timeout", "config-id", "container", "cool-down-period", "cpu", "cpu-utilization", "create-time", "created-by", "default-expiration", "deployment", "disable-health-check", "disable-trace-sampling", "disk-gb", "disk-usage-bytes", "disk-utilization", "endpoints-api-service", "entrypoint", "env", "env-variables", "failure-threshold", "files-count", "forwarded-ports", "health-check", "healthy-threshold", "host", "id", "idle-timeout", "image", "inbound-services", "initial-delay", "instance-class", "instance-tag", "instances", "liveness-check", "login", "manual-scaling", "max-concurrent-requests", "max-idle-instances", "max-instances", "max-pending-latency", "max-total-instances", "memory-gb", "min-idle-instances", "min-instances", "min-pending-latency", "min-total-instances", "name", "network", "network-utilization", "nobuild-files-regex", "path", "readiness-check", "request-utilization", "resources", "restart-threshold", "rollout-strategy", "runtime", "runtime-api-version", "runtime-channel", "runtime-main-executable-path", "script", "security-level", "serving-status", "session-affinity", "shell", "source-url", "standard-scheduler-settings", "subnetwork-name", "success-threshold", "target-concurrent-requests", "target-cpu-utilization", "target-read-bytes-per-second", "target-read-ops-per-second", "target-received-bytes-per-second", "target-received-packets-per-second", "target-request-count-per-second", "target-sent-bytes-per-second", "target-sent-packets-per-second", "target-throughput-utilization", "target-utilization", "target-write-bytes-per-second", "target-write-ops-per-second", "threadsafe", "timeout", "unhealthy-threshold", "url", "version-url", "vm", "vpc-access-connector", "zip", "zones"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Version = json::value::from_value(object).unwrap();
        let mut call = self.hub.apps().services_versions_patch(request, opt.value_of("apps-id").unwrap_or(""), opt.value_of("services-id").unwrap_or(""), opt.value_of("versions-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(value.unwrap_or(""));
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
            ("apps", Some(opt)) => {
                match opt.subcommand() {
                    ("authorized-certificates-create", Some(opt)) => {
                        call_result = self._apps_authorized_certificates_create(opt, dry_run, &mut err);
                    },
                    ("authorized-certificates-delete", Some(opt)) => {
                        call_result = self._apps_authorized_certificates_delete(opt, dry_run, &mut err);
                    },
                    ("authorized-certificates-get", Some(opt)) => {
                        call_result = self._apps_authorized_certificates_get(opt, dry_run, &mut err);
                    },
                    ("authorized-certificates-list", Some(opt)) => {
                        call_result = self._apps_authorized_certificates_list(opt, dry_run, &mut err);
                    },
                    ("authorized-certificates-patch", Some(opt)) => {
                        call_result = self._apps_authorized_certificates_patch(opt, dry_run, &mut err);
                    },
                    ("authorized-domains-list", Some(opt)) => {
                        call_result = self._apps_authorized_domains_list(opt, dry_run, &mut err);
                    },
                    ("create", Some(opt)) => {
                        call_result = self._apps_create(opt, dry_run, &mut err);
                    },
                    ("domain-mappings-create", Some(opt)) => {
                        call_result = self._apps_domain_mappings_create(opt, dry_run, &mut err);
                    },
                    ("domain-mappings-delete", Some(opt)) => {
                        call_result = self._apps_domain_mappings_delete(opt, dry_run, &mut err);
                    },
                    ("domain-mappings-get", Some(opt)) => {
                        call_result = self._apps_domain_mappings_get(opt, dry_run, &mut err);
                    },
                    ("domain-mappings-list", Some(opt)) => {
                        call_result = self._apps_domain_mappings_list(opt, dry_run, &mut err);
                    },
                    ("domain-mappings-patch", Some(opt)) => {
                        call_result = self._apps_domain_mappings_patch(opt, dry_run, &mut err);
                    },
                    ("firewall-ingress-rules-batch-update", Some(opt)) => {
                        call_result = self._apps_firewall_ingress_rules_batch_update(opt, dry_run, &mut err);
                    },
                    ("firewall-ingress-rules-create", Some(opt)) => {
                        call_result = self._apps_firewall_ingress_rules_create(opt, dry_run, &mut err);
                    },
                    ("firewall-ingress-rules-delete", Some(opt)) => {
                        call_result = self._apps_firewall_ingress_rules_delete(opt, dry_run, &mut err);
                    },
                    ("firewall-ingress-rules-get", Some(opt)) => {
                        call_result = self._apps_firewall_ingress_rules_get(opt, dry_run, &mut err);
                    },
                    ("firewall-ingress-rules-list", Some(opt)) => {
                        call_result = self._apps_firewall_ingress_rules_list(opt, dry_run, &mut err);
                    },
                    ("firewall-ingress-rules-patch", Some(opt)) => {
                        call_result = self._apps_firewall_ingress_rules_patch(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._apps_get(opt, dry_run, &mut err);
                    },
                    ("locations-get", Some(opt)) => {
                        call_result = self._apps_locations_get(opt, dry_run, &mut err);
                    },
                    ("locations-list", Some(opt)) => {
                        call_result = self._apps_locations_list(opt, dry_run, &mut err);
                    },
                    ("operations-get", Some(opt)) => {
                        call_result = self._apps_operations_get(opt, dry_run, &mut err);
                    },
                    ("operations-list", Some(opt)) => {
                        call_result = self._apps_operations_list(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._apps_patch(opt, dry_run, &mut err);
                    },
                    ("repair", Some(opt)) => {
                        call_result = self._apps_repair(opt, dry_run, &mut err);
                    },
                    ("services-delete", Some(opt)) => {
                        call_result = self._apps_services_delete(opt, dry_run, &mut err);
                    },
                    ("services-get", Some(opt)) => {
                        call_result = self._apps_services_get(opt, dry_run, &mut err);
                    },
                    ("services-list", Some(opt)) => {
                        call_result = self._apps_services_list(opt, dry_run, &mut err);
                    },
                    ("services-patch", Some(opt)) => {
                        call_result = self._apps_services_patch(opt, dry_run, &mut err);
                    },
                    ("services-versions-create", Some(opt)) => {
                        call_result = self._apps_services_versions_create(opt, dry_run, &mut err);
                    },
                    ("services-versions-delete", Some(opt)) => {
                        call_result = self._apps_services_versions_delete(opt, dry_run, &mut err);
                    },
                    ("services-versions-get", Some(opt)) => {
                        call_result = self._apps_services_versions_get(opt, dry_run, &mut err);
                    },
                    ("services-versions-instances-debug", Some(opt)) => {
                        call_result = self._apps_services_versions_instances_debug(opt, dry_run, &mut err);
                    },
                    ("services-versions-instances-delete", Some(opt)) => {
                        call_result = self._apps_services_versions_instances_delete(opt, dry_run, &mut err);
                    },
                    ("services-versions-instances-get", Some(opt)) => {
                        call_result = self._apps_services_versions_instances_get(opt, dry_run, &mut err);
                    },
                    ("services-versions-instances-list", Some(opt)) => {
                        call_result = self._apps_services_versions_instances_list(opt, dry_run, &mut err);
                    },
                    ("services-versions-list", Some(opt)) => {
                        call_result = self._apps_services_versions_list(opt, dry_run, &mut err);
                    },
                    ("services-versions-patch", Some(opt)) => {
                        call_result = self._apps_services_versions_patch(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("apps".to_string()));
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

            match cmn::application_secret_from_directory(&config_dir, "appengine1-secret.json",
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
                                          program_name: "appengine1",
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
            hub: api::Appengine::new(client, auth),
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
        ("apps", "methods: 'authorized-certificates-create', 'authorized-certificates-delete', 'authorized-certificates-get', 'authorized-certificates-list', 'authorized-certificates-patch', 'authorized-domains-list', 'create', 'domain-mappings-create', 'domain-mappings-delete', 'domain-mappings-get', 'domain-mappings-list', 'domain-mappings-patch', 'firewall-ingress-rules-batch-update', 'firewall-ingress-rules-create', 'firewall-ingress-rules-delete', 'firewall-ingress-rules-get', 'firewall-ingress-rules-list', 'firewall-ingress-rules-patch', 'get', 'locations-get', 'locations-list', 'operations-get', 'operations-list', 'patch', 'repair', 'services-delete', 'services-get', 'services-list', 'services-patch', 'services-versions-create', 'services-versions-delete', 'services-versions-get', 'services-versions-instances-debug', 'services-versions-instances-delete', 'services-versions-instances-get', 'services-versions-instances-list', 'services-versions-list' and 'services-versions-patch'", vec![
            ("authorized-certificates-create",
                    Some(r##"Uploads the specified SSL certificate."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_authorized-certificates-create",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `parent`. Name of the parent Application resource. Example: apps/myapp."##),
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
            ("authorized-certificates-delete",
                    Some(r##"Deletes the specified SSL certificate."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_authorized-certificates-delete",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource to delete. Example: apps/myapp/authorizedCertificates/12345."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"authorized-certificates-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
            ("authorized-certificates-get",
                    Some(r##"Gets the specified SSL certificate."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_authorized-certificates-get",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource requested. Example: apps/myapp/authorizedCertificates/12345."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"authorized-certificates-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
            ("authorized-certificates-list",
                    Some(r##"Lists all SSL certificates the user is authorized to administer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_authorized-certificates-list",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `parent`. Name of the parent Application resource. Example: apps/myapp."##),
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
            ("authorized-certificates-patch",
                    Some(r##"Updates the specified SSL certificate. To renew a certificate and maintain its existing domain mappings, update certificate_data with a new certificate. The new certificate must be applicable to the same domains as the original certificate. The certificate display_name may also be updated."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_authorized-certificates-patch",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource to update. Example: apps/myapp/authorizedCertificates/12345."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"authorized-certificates-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
            ("authorized-domains-list",
                    Some(r##"Lists all domains the user is authorized to administer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_authorized-domains-list",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `parent`. Name of the parent Application resource. Example: apps/myapp."##),
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
            ("create",
                    Some(r##"Creates an App Engine application for a Google Cloud Platform project. Required fields:
        id - The ID of the target Cloud Platform project.
        location - The region (https://cloud.google.com/appengine/docs/locations) where you want the App Engine application located.For more information about App Engine applications, see Managing Projects, Applications, and Billing (https://cloud.google.com/appengine/docs/standard/python/console/)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_create",
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
            ("domain-mappings-create",
                    Some(r##"Maps a domain to an application. A user must be authorized to administer a domain in order to map it to an application. For a list of available authorized domains, see AuthorizedDomains.ListAuthorizedDomains."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_domain-mappings-create",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `parent`. Name of the parent Application resource. Example: apps/myapp."##),
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
            ("domain-mappings-delete",
                    Some(r##"Deletes the specified domain mapping. A user must be authorized to administer the associated domain in order to delete a DomainMapping resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_domain-mappings-delete",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource to delete. Example: apps/myapp/domainMappings/example.com."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"domain-mappings-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
            ("domain-mappings-get",
                    Some(r##"Gets the specified domain mapping."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_domain-mappings-get",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource requested. Example: apps/myapp/domainMappings/example.com."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"domain-mappings-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
            ("domain-mappings-list",
                    Some(r##"Lists the domain mappings on an application."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_domain-mappings-list",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `parent`. Name of the parent Application resource. Example: apps/myapp."##),
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
            ("domain-mappings-patch",
                    Some(r##"Updates the specified domain mapping. To map an SSL certificate to a domain mapping, update certificate_id to point to an AuthorizedCertificate resource. A user must be authorized to administer the associated domain in order to update a DomainMapping resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_domain-mappings-patch",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource to update. Example: apps/myapp/domainMappings/example.com."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"domain-mappings-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
            ("firewall-ingress-rules-batch-update",
                    Some(r##"Replaces the entire firewall ruleset in one bulk operation. This overrides and replaces the rules of an existing firewall with the new rules.If the final rule does not match traffic with the '*' wildcard IP range, then an "allow all" rule is explicitly added to the end of the list."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_firewall-ingress-rules-batch-update",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the Firewall collection to set. Example: apps/myapp/firewall/ingressRules."##),
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
            ("firewall-ingress-rules-create",
                    Some(r##"Creates a firewall rule for the application."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_firewall-ingress-rules-create",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `parent`. Name of the parent Firewall collection in which to create a new rule. Example: apps/myapp/firewall/ingressRules."##),
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
            ("firewall-ingress-rules-delete",
                    Some(r##"Deletes the specified firewall rule."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_firewall-ingress-rules-delete",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the Firewall resource to delete. Example: apps/myapp/firewall/ingressRules/100."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"ingress-rules-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
            ("firewall-ingress-rules-get",
                    Some(r##"Gets the specified firewall rule."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_firewall-ingress-rules-get",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the Firewall resource to retrieve. Example: apps/myapp/firewall/ingressRules/100."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"ingress-rules-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
            ("firewall-ingress-rules-list",
                    Some(r##"Lists the firewall rules of an application."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_firewall-ingress-rules-list",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `parent`. Name of the Firewall collection to retrieve. Example: apps/myapp/firewall/ingressRules."##),
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
            ("firewall-ingress-rules-patch",
                    Some(r##"Updates the specified firewall rule."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_firewall-ingress-rules-patch",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the Firewall resource to update. Example: apps/myapp/firewall/ingressRules/100."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"ingress-rules-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
                    Some(r##"Gets information about an application."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_get",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the Application resource to get. Example: apps/myapp."##),
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
                    Some(r##"Gets information about a location."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_locations-get",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Resource name for the location."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"locations-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_locations-list",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. The resource that owns the locations collection, if applicable."##),
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
            ("operations-get",
                    Some(r##"Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_operations-get",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. The name of the operation resource."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"operations-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
            ("operations-list",
                    Some(r##"Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns UNIMPLEMENTED.NOTE: the name binding allows API services to override the binding to use different resource name schemes, such as users/*/operations. To override the binding, API services can add a binding such as "/v1/{name=users/*}/operations" to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_operations-list",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. The name of the operation's parent resource."##),
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
                    Some(r##"Updates the specified Application resource. You can update the following fields:
        auth_domain - Google authentication domain for controlling user access to the application.
        default_cookie_expiration - Cookie expiration policy for the application."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_patch",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the Application resource to update. Example: apps/myapp."##),
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
            ("repair",
                    Some(r##"Recreates the required App Engine features for the specified App Engine application, for example a Cloud Storage bucket or App Engine service account. Use this method if you receive an error message about a missing feature, for example, Error retrieving the App Engine service account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_repair",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the application to repair. Example: apps/myapp"##),
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
            ("services-delete",
                    Some(r##"Deletes the specified service and all enclosed versions."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_services-delete",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource requested. Example: apps/myapp/services/default."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"services-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
            ("services-get",
                    Some(r##"Gets the current configuration of the specified service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_services-get",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource requested. Example: apps/myapp/services/default."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"services-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
            ("services-list",
                    Some(r##"Lists all the services in the application."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_services-list",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `parent`. Name of the parent Application resource. Example: apps/myapp."##),
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
            ("services-patch",
                    Some(r##"Updates the configuration of the specified service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_services-patch",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource to update. Example: apps/myapp/services/default."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"services-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
            ("services-versions-create",
                    Some(r##"Deploys code and resource files to a new version."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_services-versions-create",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `parent`. Name of the parent resource to create this version under. Example: apps/myapp/services/default."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"services-id"##),
                     None,
                     Some(r##"Part of `parent`. See documentation of `appsId`."##),
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
            ("services-versions-delete",
                    Some(r##"Deletes an existing Version resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_services-versions-delete",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource requested. Example: apps/myapp/services/default/versions/v1."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"services-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"versions-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
            ("services-versions-get",
                    Some(r##"Gets the specified Version resource. By default, only a BASIC_VIEW will be returned. Specify the FULL_VIEW parameter to get the full resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_services-versions-get",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource requested. Example: apps/myapp/services/default/versions/v1."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"services-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"versions-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
            ("services-versions-instances-debug",
                    Some(r##"Enables debugging on a VM instance. This allows you to use the SSH command to connect to the virtual machine where the instance lives. While in "debug mode", the instance continues to serve live traffic. You should delete the instance when you are done debugging and then allow the system to take over and determine if another instance should be started.Only applicable for instances in App Engine flexible environment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_services-versions-instances-debug",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource requested. Example: apps/myapp/services/default/versions/v1/instances/instance-1."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"services-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"versions-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instances-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
            ("services-versions-instances-delete",
                    Some(r##"Stops a running instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_services-versions-instances-delete",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource requested. Example: apps/myapp/services/default/versions/v1/instances/instance-1."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"services-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"versions-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instances-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
            ("services-versions-instances-get",
                    Some(r##"Gets instance information."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_services-versions-instances-get",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource requested. Example: apps/myapp/services/default/versions/v1/instances/instance-1."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"services-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"versions-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instances-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
            ("services-versions-instances-list",
                    Some(r##"Lists the instances of a version.Tip: To aggregate details about instances over time, see the Stackdriver Monitoring API (https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.timeSeries/list)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_services-versions-instances-list",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `parent`. Name of the parent Version resource. Example: apps/myapp/services/default/versions/v1."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"services-id"##),
                     None,
                     Some(r##"Part of `parent`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"versions-id"##),
                     None,
                     Some(r##"Part of `parent`. See documentation of `appsId`."##),
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
            ("services-versions-list",
                    Some(r##"Lists the versions of a service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_services-versions-list",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `parent`. Name of the parent Service resource. Example: apps/myapp/services/default."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"services-id"##),
                     None,
                     Some(r##"Part of `parent`. See documentation of `appsId`."##),
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
            ("services-versions-patch",
                    Some(r##"Updates the specified Version resource. You can specify the following fields depending on the App Engine environment and type of scaling that the version resource uses:Standard environment
        instance_class (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.instance_class)automatic scaling in the standard environment:
        automatic_scaling.min_idle_instances (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.automatic_scaling)
        automatic_scaling.max_idle_instances (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.automatic_scaling)
        automaticScaling.standard_scheduler_settings.max_instances (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#StandardSchedulerSettings)
        automaticScaling.standard_scheduler_settings.min_instances (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#StandardSchedulerSettings)
        automaticScaling.standard_scheduler_settings.target_cpu_utilization (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#StandardSchedulerSettings)
        automaticScaling.standard_scheduler_settings.target_throughput_utilization (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#StandardSchedulerSettings)basic scaling or manual scaling in the standard environment:
        serving_status (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.serving_status)Flexible environment
        serving_status (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.serving_status)automatic scaling in the flexible environment:
        automatic_scaling.min_total_instances (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.automatic_scaling)
        automatic_scaling.max_total_instances (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.automatic_scaling)
        automatic_scaling.cool_down_period_sec (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.automatic_scaling)
        automatic_scaling.cpu_utilization.target_utilization (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.automatic_scaling)"##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_services-versions-patch",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource to update. Example: apps/myapp/services/default/versions/1."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"services-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"versions-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
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
    
    let mut app = App::new("appengine1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("1.0.8+20190321")
           .about("Provisions and manages developers' App Engine applications.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_appengine1_cli")
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