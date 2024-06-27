// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_domains1_beta1::{api, Error, oauth2, client::chrono, FieldMask};


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
    hub: api::CloudDomains<S>,
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

    async fn _projects_locations_registrations_configure_contact_settings(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "contact-notices" => Some(("contactNotices", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "contact-settings.admin-contact.email" => Some(("contactSettings.adminContact.email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.admin-contact.fax-number" => Some(("contactSettings.adminContact.faxNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.admin-contact.phone-number" => Some(("contactSettings.adminContact.phoneNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.admin-contact.postal-address.address-lines" => Some(("contactSettings.adminContact.postalAddress.addressLines", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "contact-settings.admin-contact.postal-address.administrative-area" => Some(("contactSettings.adminContact.postalAddress.administrativeArea", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.admin-contact.postal-address.language-code" => Some(("contactSettings.adminContact.postalAddress.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.admin-contact.postal-address.locality" => Some(("contactSettings.adminContact.postalAddress.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.admin-contact.postal-address.organization" => Some(("contactSettings.adminContact.postalAddress.organization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.admin-contact.postal-address.postal-code" => Some(("contactSettings.adminContact.postalAddress.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.admin-contact.postal-address.recipients" => Some(("contactSettings.adminContact.postalAddress.recipients", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "contact-settings.admin-contact.postal-address.region-code" => Some(("contactSettings.adminContact.postalAddress.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.admin-contact.postal-address.revision" => Some(("contactSettings.adminContact.postalAddress.revision", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "contact-settings.admin-contact.postal-address.sorting-code" => Some(("contactSettings.adminContact.postalAddress.sortingCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.admin-contact.postal-address.sublocality" => Some(("contactSettings.adminContact.postalAddress.sublocality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.privacy" => Some(("contactSettings.privacy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.registrant-contact.email" => Some(("contactSettings.registrantContact.email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.registrant-contact.fax-number" => Some(("contactSettings.registrantContact.faxNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.registrant-contact.phone-number" => Some(("contactSettings.registrantContact.phoneNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.registrant-contact.postal-address.address-lines" => Some(("contactSettings.registrantContact.postalAddress.addressLines", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "contact-settings.registrant-contact.postal-address.administrative-area" => Some(("contactSettings.registrantContact.postalAddress.administrativeArea", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.registrant-contact.postal-address.language-code" => Some(("contactSettings.registrantContact.postalAddress.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.registrant-contact.postal-address.locality" => Some(("contactSettings.registrantContact.postalAddress.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.registrant-contact.postal-address.organization" => Some(("contactSettings.registrantContact.postalAddress.organization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.registrant-contact.postal-address.postal-code" => Some(("contactSettings.registrantContact.postalAddress.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.registrant-contact.postal-address.recipients" => Some(("contactSettings.registrantContact.postalAddress.recipients", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "contact-settings.registrant-contact.postal-address.region-code" => Some(("contactSettings.registrantContact.postalAddress.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.registrant-contact.postal-address.revision" => Some(("contactSettings.registrantContact.postalAddress.revision", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "contact-settings.registrant-contact.postal-address.sorting-code" => Some(("contactSettings.registrantContact.postalAddress.sortingCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.registrant-contact.postal-address.sublocality" => Some(("contactSettings.registrantContact.postalAddress.sublocality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.technical-contact.email" => Some(("contactSettings.technicalContact.email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.technical-contact.fax-number" => Some(("contactSettings.technicalContact.faxNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.technical-contact.phone-number" => Some(("contactSettings.technicalContact.phoneNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.technical-contact.postal-address.address-lines" => Some(("contactSettings.technicalContact.postalAddress.addressLines", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "contact-settings.technical-contact.postal-address.administrative-area" => Some(("contactSettings.technicalContact.postalAddress.administrativeArea", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.technical-contact.postal-address.language-code" => Some(("contactSettings.technicalContact.postalAddress.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.technical-contact.postal-address.locality" => Some(("contactSettings.technicalContact.postalAddress.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.technical-contact.postal-address.organization" => Some(("contactSettings.technicalContact.postalAddress.organization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.technical-contact.postal-address.postal-code" => Some(("contactSettings.technicalContact.postalAddress.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.technical-contact.postal-address.recipients" => Some(("contactSettings.technicalContact.postalAddress.recipients", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "contact-settings.technical-contact.postal-address.region-code" => Some(("contactSettings.technicalContact.postalAddress.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.technical-contact.postal-address.revision" => Some(("contactSettings.technicalContact.postalAddress.revision", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "contact-settings.technical-contact.postal-address.sorting-code" => Some(("contactSettings.technicalContact.postalAddress.sortingCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.technical-contact.postal-address.sublocality" => Some(("contactSettings.technicalContact.postalAddress.sublocality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-mask" => Some(("updateMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "validate-only" => Some(("validateOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["address-lines", "admin-contact", "administrative-area", "contact-notices", "contact-settings", "email", "fax-number", "language-code", "locality", "organization", "phone-number", "postal-address", "postal-code", "privacy", "recipients", "region-code", "registrant-contact", "revision", "sorting-code", "sublocality", "technical-contact", "update-mask", "validate-only"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ConfigureContactSettingsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_registrations_configure_contact_settings(request, opt.value_of("registration").unwrap_or(""));
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

    async fn _projects_locations_registrations_configure_dns_settings(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "dns-settings.custom-dns.name-servers" => Some(("dnsSettings.customDns.nameServers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "dns-settings.google-domains-dns.ds-state" => Some(("dnsSettings.googleDomainsDns.dsState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dns-settings.google-domains-dns.name-servers" => Some(("dnsSettings.googleDomainsDns.nameServers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "dns-settings.google-domains-redirects-data-available" => Some(("dnsSettings.googleDomainsRedirectsDataAvailable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update-mask" => Some(("updateMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "validate-only" => Some(("validateOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["custom-dns", "dns-settings", "ds-state", "google-domains-dns", "google-domains-redirects-data-available", "name-servers", "update-mask", "validate-only"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ConfigureDnsSettingsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_registrations_configure_dns_settings(request, opt.value_of("registration").unwrap_or(""));
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

    async fn _projects_locations_registrations_configure_management_settings(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "management-settings.effective-transfer-lock-state" => Some(("managementSettings.effectiveTransferLockState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "management-settings.preferred-renewal-method" => Some(("managementSettings.preferredRenewalMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "management-settings.renewal-method" => Some(("managementSettings.renewalMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "management-settings.transfer-lock-state" => Some(("managementSettings.transferLockState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-mask" => Some(("updateMask", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["effective-transfer-lock-state", "management-settings", "preferred-renewal-method", "renewal-method", "transfer-lock-state", "update-mask"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ConfigureManagementSettingsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_registrations_configure_management_settings(request, opt.value_of("registration").unwrap_or(""));
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

    async fn _projects_locations_registrations_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_registrations_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_registrations_export(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::ExportRegistrationRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_registrations_export(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_registrations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_registrations_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_registrations_get_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_registrations_get_iam_policy(opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_locations_registrations_import(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "domain-name" => Some(("domainName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["domain-name", "labels"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ImportDomainRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_registrations_import(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_registrations_initiate_push_transfer(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "tag" => Some(("tag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["tag"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::InitiatePushTransferRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_registrations_initiate_push_transfer(request, opt.value_of("registration").unwrap_or(""));
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

    async fn _projects_locations_registrations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_registrations_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_registrations_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "contact-settings.admin-contact.email" => Some(("contactSettings.adminContact.email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.admin-contact.fax-number" => Some(("contactSettings.adminContact.faxNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.admin-contact.phone-number" => Some(("contactSettings.adminContact.phoneNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.admin-contact.postal-address.address-lines" => Some(("contactSettings.adminContact.postalAddress.addressLines", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "contact-settings.admin-contact.postal-address.administrative-area" => Some(("contactSettings.adminContact.postalAddress.administrativeArea", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.admin-contact.postal-address.language-code" => Some(("contactSettings.adminContact.postalAddress.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.admin-contact.postal-address.locality" => Some(("contactSettings.adminContact.postalAddress.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.admin-contact.postal-address.organization" => Some(("contactSettings.adminContact.postalAddress.organization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.admin-contact.postal-address.postal-code" => Some(("contactSettings.adminContact.postalAddress.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.admin-contact.postal-address.recipients" => Some(("contactSettings.adminContact.postalAddress.recipients", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "contact-settings.admin-contact.postal-address.region-code" => Some(("contactSettings.adminContact.postalAddress.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.admin-contact.postal-address.revision" => Some(("contactSettings.adminContact.postalAddress.revision", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "contact-settings.admin-contact.postal-address.sorting-code" => Some(("contactSettings.adminContact.postalAddress.sortingCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.admin-contact.postal-address.sublocality" => Some(("contactSettings.adminContact.postalAddress.sublocality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.privacy" => Some(("contactSettings.privacy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.registrant-contact.email" => Some(("contactSettings.registrantContact.email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.registrant-contact.fax-number" => Some(("contactSettings.registrantContact.faxNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.registrant-contact.phone-number" => Some(("contactSettings.registrantContact.phoneNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.registrant-contact.postal-address.address-lines" => Some(("contactSettings.registrantContact.postalAddress.addressLines", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "contact-settings.registrant-contact.postal-address.administrative-area" => Some(("contactSettings.registrantContact.postalAddress.administrativeArea", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.registrant-contact.postal-address.language-code" => Some(("contactSettings.registrantContact.postalAddress.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.registrant-contact.postal-address.locality" => Some(("contactSettings.registrantContact.postalAddress.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.registrant-contact.postal-address.organization" => Some(("contactSettings.registrantContact.postalAddress.organization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.registrant-contact.postal-address.postal-code" => Some(("contactSettings.registrantContact.postalAddress.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.registrant-contact.postal-address.recipients" => Some(("contactSettings.registrantContact.postalAddress.recipients", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "contact-settings.registrant-contact.postal-address.region-code" => Some(("contactSettings.registrantContact.postalAddress.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.registrant-contact.postal-address.revision" => Some(("contactSettings.registrantContact.postalAddress.revision", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "contact-settings.registrant-contact.postal-address.sorting-code" => Some(("contactSettings.registrantContact.postalAddress.sortingCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.registrant-contact.postal-address.sublocality" => Some(("contactSettings.registrantContact.postalAddress.sublocality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.technical-contact.email" => Some(("contactSettings.technicalContact.email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.technical-contact.fax-number" => Some(("contactSettings.technicalContact.faxNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.technical-contact.phone-number" => Some(("contactSettings.technicalContact.phoneNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.technical-contact.postal-address.address-lines" => Some(("contactSettings.technicalContact.postalAddress.addressLines", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "contact-settings.technical-contact.postal-address.administrative-area" => Some(("contactSettings.technicalContact.postalAddress.administrativeArea", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.technical-contact.postal-address.language-code" => Some(("contactSettings.technicalContact.postalAddress.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.technical-contact.postal-address.locality" => Some(("contactSettings.technicalContact.postalAddress.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.technical-contact.postal-address.organization" => Some(("contactSettings.technicalContact.postalAddress.organization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.technical-contact.postal-address.postal-code" => Some(("contactSettings.technicalContact.postalAddress.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.technical-contact.postal-address.recipients" => Some(("contactSettings.technicalContact.postalAddress.recipients", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "contact-settings.technical-contact.postal-address.region-code" => Some(("contactSettings.technicalContact.postalAddress.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.technical-contact.postal-address.revision" => Some(("contactSettings.technicalContact.postalAddress.revision", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "contact-settings.technical-contact.postal-address.sorting-code" => Some(("contactSettings.technicalContact.postalAddress.sortingCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-settings.technical-contact.postal-address.sublocality" => Some(("contactSettings.technicalContact.postalAddress.sublocality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dns-settings.custom-dns.name-servers" => Some(("dnsSettings.customDns.nameServers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "dns-settings.google-domains-dns.ds-state" => Some(("dnsSettings.googleDomainsDns.dsState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "dns-settings.google-domains-dns.name-servers" => Some(("dnsSettings.googleDomainsDns.nameServers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "dns-settings.google-domains-redirects-data-available" => Some(("dnsSettings.googleDomainsRedirectsDataAvailable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "domain-name" => Some(("domainName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "domain-properties" => Some(("domainProperties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "expire-time" => Some(("expireTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "issues" => Some(("issues", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "management-settings.effective-transfer-lock-state" => Some(("managementSettings.effectiveTransferLockState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "management-settings.preferred-renewal-method" => Some(("managementSettings.preferredRenewalMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "management-settings.renewal-method" => Some(("managementSettings.renewalMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "management-settings.transfer-lock-state" => Some(("managementSettings.transferLockState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-contact-settings.admin-contact.email" => Some(("pendingContactSettings.adminContact.email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-contact-settings.admin-contact.fax-number" => Some(("pendingContactSettings.adminContact.faxNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-contact-settings.admin-contact.phone-number" => Some(("pendingContactSettings.adminContact.phoneNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-contact-settings.admin-contact.postal-address.address-lines" => Some(("pendingContactSettings.adminContact.postalAddress.addressLines", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "pending-contact-settings.admin-contact.postal-address.administrative-area" => Some(("pendingContactSettings.adminContact.postalAddress.administrativeArea", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-contact-settings.admin-contact.postal-address.language-code" => Some(("pendingContactSettings.adminContact.postalAddress.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-contact-settings.admin-contact.postal-address.locality" => Some(("pendingContactSettings.adminContact.postalAddress.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-contact-settings.admin-contact.postal-address.organization" => Some(("pendingContactSettings.adminContact.postalAddress.organization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-contact-settings.admin-contact.postal-address.postal-code" => Some(("pendingContactSettings.adminContact.postalAddress.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-contact-settings.admin-contact.postal-address.recipients" => Some(("pendingContactSettings.adminContact.postalAddress.recipients", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "pending-contact-settings.admin-contact.postal-address.region-code" => Some(("pendingContactSettings.adminContact.postalAddress.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-contact-settings.admin-contact.postal-address.revision" => Some(("pendingContactSettings.adminContact.postalAddress.revision", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "pending-contact-settings.admin-contact.postal-address.sorting-code" => Some(("pendingContactSettings.adminContact.postalAddress.sortingCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-contact-settings.admin-contact.postal-address.sublocality" => Some(("pendingContactSettings.adminContact.postalAddress.sublocality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-contact-settings.privacy" => Some(("pendingContactSettings.privacy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-contact-settings.registrant-contact.email" => Some(("pendingContactSettings.registrantContact.email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-contact-settings.registrant-contact.fax-number" => Some(("pendingContactSettings.registrantContact.faxNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-contact-settings.registrant-contact.phone-number" => Some(("pendingContactSettings.registrantContact.phoneNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-contact-settings.registrant-contact.postal-address.address-lines" => Some(("pendingContactSettings.registrantContact.postalAddress.addressLines", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "pending-contact-settings.registrant-contact.postal-address.administrative-area" => Some(("pendingContactSettings.registrantContact.postalAddress.administrativeArea", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-contact-settings.registrant-contact.postal-address.language-code" => Some(("pendingContactSettings.registrantContact.postalAddress.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-contact-settings.registrant-contact.postal-address.locality" => Some(("pendingContactSettings.registrantContact.postalAddress.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-contact-settings.registrant-contact.postal-address.organization" => Some(("pendingContactSettings.registrantContact.postalAddress.organization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-contact-settings.registrant-contact.postal-address.postal-code" => Some(("pendingContactSettings.registrantContact.postalAddress.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-contact-settings.registrant-contact.postal-address.recipients" => Some(("pendingContactSettings.registrantContact.postalAddress.recipients", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "pending-contact-settings.registrant-contact.postal-address.region-code" => Some(("pendingContactSettings.registrantContact.postalAddress.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-contact-settings.registrant-contact.postal-address.revision" => Some(("pendingContactSettings.registrantContact.postalAddress.revision", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "pending-contact-settings.registrant-contact.postal-address.sorting-code" => Some(("pendingContactSettings.registrantContact.postalAddress.sortingCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-contact-settings.registrant-contact.postal-address.sublocality" => Some(("pendingContactSettings.registrantContact.postalAddress.sublocality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-contact-settings.technical-contact.email" => Some(("pendingContactSettings.technicalContact.email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-contact-settings.technical-contact.fax-number" => Some(("pendingContactSettings.technicalContact.faxNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-contact-settings.technical-contact.phone-number" => Some(("pendingContactSettings.technicalContact.phoneNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-contact-settings.technical-contact.postal-address.address-lines" => Some(("pendingContactSettings.technicalContact.postalAddress.addressLines", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "pending-contact-settings.technical-contact.postal-address.administrative-area" => Some(("pendingContactSettings.technicalContact.postalAddress.administrativeArea", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-contact-settings.technical-contact.postal-address.language-code" => Some(("pendingContactSettings.technicalContact.postalAddress.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-contact-settings.technical-contact.postal-address.locality" => Some(("pendingContactSettings.technicalContact.postalAddress.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-contact-settings.technical-contact.postal-address.organization" => Some(("pendingContactSettings.technicalContact.postalAddress.organization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-contact-settings.technical-contact.postal-address.postal-code" => Some(("pendingContactSettings.technicalContact.postalAddress.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-contact-settings.technical-contact.postal-address.recipients" => Some(("pendingContactSettings.technicalContact.postalAddress.recipients", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "pending-contact-settings.technical-contact.postal-address.region-code" => Some(("pendingContactSettings.technicalContact.postalAddress.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-contact-settings.technical-contact.postal-address.revision" => Some(("pendingContactSettings.technicalContact.postalAddress.revision", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "pending-contact-settings.technical-contact.postal-address.sorting-code" => Some(("pendingContactSettings.technicalContact.postalAddress.sortingCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pending-contact-settings.technical-contact.postal-address.sublocality" => Some(("pendingContactSettings.technicalContact.postalAddress.sublocality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "register-failure-reason" => Some(("registerFailureReason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "supported-privacy" => Some(("supportedPrivacy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "transfer-failure-reason" => Some(("transferFailureReason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["address-lines", "admin-contact", "administrative-area", "contact-settings", "create-time", "custom-dns", "dns-settings", "domain-name", "domain-properties", "ds-state", "effective-transfer-lock-state", "email", "expire-time", "fax-number", "google-domains-dns", "google-domains-redirects-data-available", "issues", "labels", "language-code", "locality", "management-settings", "name", "name-servers", "organization", "pending-contact-settings", "phone-number", "postal-address", "postal-code", "preferred-renewal-method", "privacy", "recipients", "region-code", "register-failure-reason", "registrant-contact", "renewal-method", "revision", "sorting-code", "state", "sublocality", "supported-privacy", "technical-contact", "transfer-failure-reason", "transfer-lock-state"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Registration = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_registrations_patch(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_registrations_register(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "contact-notices" => Some(("contactNotices", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "domain-notices" => Some(("domainNotices", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "registration.contact-settings.admin-contact.email" => Some(("registration.contactSettings.adminContact.email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.admin-contact.fax-number" => Some(("registration.contactSettings.adminContact.faxNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.admin-contact.phone-number" => Some(("registration.contactSettings.adminContact.phoneNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.admin-contact.postal-address.address-lines" => Some(("registration.contactSettings.adminContact.postalAddress.addressLines", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "registration.contact-settings.admin-contact.postal-address.administrative-area" => Some(("registration.contactSettings.adminContact.postalAddress.administrativeArea", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.admin-contact.postal-address.language-code" => Some(("registration.contactSettings.adminContact.postalAddress.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.admin-contact.postal-address.locality" => Some(("registration.contactSettings.adminContact.postalAddress.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.admin-contact.postal-address.organization" => Some(("registration.contactSettings.adminContact.postalAddress.organization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.admin-contact.postal-address.postal-code" => Some(("registration.contactSettings.adminContact.postalAddress.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.admin-contact.postal-address.recipients" => Some(("registration.contactSettings.adminContact.postalAddress.recipients", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "registration.contact-settings.admin-contact.postal-address.region-code" => Some(("registration.contactSettings.adminContact.postalAddress.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.admin-contact.postal-address.revision" => Some(("registration.contactSettings.adminContact.postalAddress.revision", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "registration.contact-settings.admin-contact.postal-address.sorting-code" => Some(("registration.contactSettings.adminContact.postalAddress.sortingCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.admin-contact.postal-address.sublocality" => Some(("registration.contactSettings.adminContact.postalAddress.sublocality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.privacy" => Some(("registration.contactSettings.privacy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.registrant-contact.email" => Some(("registration.contactSettings.registrantContact.email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.registrant-contact.fax-number" => Some(("registration.contactSettings.registrantContact.faxNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.registrant-contact.phone-number" => Some(("registration.contactSettings.registrantContact.phoneNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.registrant-contact.postal-address.address-lines" => Some(("registration.contactSettings.registrantContact.postalAddress.addressLines", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "registration.contact-settings.registrant-contact.postal-address.administrative-area" => Some(("registration.contactSettings.registrantContact.postalAddress.administrativeArea", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.registrant-contact.postal-address.language-code" => Some(("registration.contactSettings.registrantContact.postalAddress.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.registrant-contact.postal-address.locality" => Some(("registration.contactSettings.registrantContact.postalAddress.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.registrant-contact.postal-address.organization" => Some(("registration.contactSettings.registrantContact.postalAddress.organization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.registrant-contact.postal-address.postal-code" => Some(("registration.contactSettings.registrantContact.postalAddress.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.registrant-contact.postal-address.recipients" => Some(("registration.contactSettings.registrantContact.postalAddress.recipients", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "registration.contact-settings.registrant-contact.postal-address.region-code" => Some(("registration.contactSettings.registrantContact.postalAddress.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.registrant-contact.postal-address.revision" => Some(("registration.contactSettings.registrantContact.postalAddress.revision", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "registration.contact-settings.registrant-contact.postal-address.sorting-code" => Some(("registration.contactSettings.registrantContact.postalAddress.sortingCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.registrant-contact.postal-address.sublocality" => Some(("registration.contactSettings.registrantContact.postalAddress.sublocality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.technical-contact.email" => Some(("registration.contactSettings.technicalContact.email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.technical-contact.fax-number" => Some(("registration.contactSettings.technicalContact.faxNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.technical-contact.phone-number" => Some(("registration.contactSettings.technicalContact.phoneNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.technical-contact.postal-address.address-lines" => Some(("registration.contactSettings.technicalContact.postalAddress.addressLines", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "registration.contact-settings.technical-contact.postal-address.administrative-area" => Some(("registration.contactSettings.technicalContact.postalAddress.administrativeArea", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.technical-contact.postal-address.language-code" => Some(("registration.contactSettings.technicalContact.postalAddress.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.technical-contact.postal-address.locality" => Some(("registration.contactSettings.technicalContact.postalAddress.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.technical-contact.postal-address.organization" => Some(("registration.contactSettings.technicalContact.postalAddress.organization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.technical-contact.postal-address.postal-code" => Some(("registration.contactSettings.technicalContact.postalAddress.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.technical-contact.postal-address.recipients" => Some(("registration.contactSettings.technicalContact.postalAddress.recipients", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "registration.contact-settings.technical-contact.postal-address.region-code" => Some(("registration.contactSettings.technicalContact.postalAddress.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.technical-contact.postal-address.revision" => Some(("registration.contactSettings.technicalContact.postalAddress.revision", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "registration.contact-settings.technical-contact.postal-address.sorting-code" => Some(("registration.contactSettings.technicalContact.postalAddress.sortingCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.technical-contact.postal-address.sublocality" => Some(("registration.contactSettings.technicalContact.postalAddress.sublocality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.create-time" => Some(("registration.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.dns-settings.custom-dns.name-servers" => Some(("registration.dnsSettings.customDns.nameServers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "registration.dns-settings.google-domains-dns.ds-state" => Some(("registration.dnsSettings.googleDomainsDns.dsState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.dns-settings.google-domains-dns.name-servers" => Some(("registration.dnsSettings.googleDomainsDns.nameServers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "registration.dns-settings.google-domains-redirects-data-available" => Some(("registration.dnsSettings.googleDomainsRedirectsDataAvailable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "registration.domain-name" => Some(("registration.domainName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.domain-properties" => Some(("registration.domainProperties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "registration.expire-time" => Some(("registration.expireTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.issues" => Some(("registration.issues", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "registration.labels" => Some(("registration.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "registration.management-settings.effective-transfer-lock-state" => Some(("registration.managementSettings.effectiveTransferLockState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.management-settings.preferred-renewal-method" => Some(("registration.managementSettings.preferredRenewalMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.management-settings.renewal-method" => Some(("registration.managementSettings.renewalMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.management-settings.transfer-lock-state" => Some(("registration.managementSettings.transferLockState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.name" => Some(("registration.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.admin-contact.email" => Some(("registration.pendingContactSettings.adminContact.email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.admin-contact.fax-number" => Some(("registration.pendingContactSettings.adminContact.faxNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.admin-contact.phone-number" => Some(("registration.pendingContactSettings.adminContact.phoneNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.admin-contact.postal-address.address-lines" => Some(("registration.pendingContactSettings.adminContact.postalAddress.addressLines", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "registration.pending-contact-settings.admin-contact.postal-address.administrative-area" => Some(("registration.pendingContactSettings.adminContact.postalAddress.administrativeArea", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.admin-contact.postal-address.language-code" => Some(("registration.pendingContactSettings.adminContact.postalAddress.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.admin-contact.postal-address.locality" => Some(("registration.pendingContactSettings.adminContact.postalAddress.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.admin-contact.postal-address.organization" => Some(("registration.pendingContactSettings.adminContact.postalAddress.organization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.admin-contact.postal-address.postal-code" => Some(("registration.pendingContactSettings.adminContact.postalAddress.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.admin-contact.postal-address.recipients" => Some(("registration.pendingContactSettings.adminContact.postalAddress.recipients", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "registration.pending-contact-settings.admin-contact.postal-address.region-code" => Some(("registration.pendingContactSettings.adminContact.postalAddress.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.admin-contact.postal-address.revision" => Some(("registration.pendingContactSettings.adminContact.postalAddress.revision", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.admin-contact.postal-address.sorting-code" => Some(("registration.pendingContactSettings.adminContact.postalAddress.sortingCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.admin-contact.postal-address.sublocality" => Some(("registration.pendingContactSettings.adminContact.postalAddress.sublocality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.privacy" => Some(("registration.pendingContactSettings.privacy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.registrant-contact.email" => Some(("registration.pendingContactSettings.registrantContact.email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.registrant-contact.fax-number" => Some(("registration.pendingContactSettings.registrantContact.faxNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.registrant-contact.phone-number" => Some(("registration.pendingContactSettings.registrantContact.phoneNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.registrant-contact.postal-address.address-lines" => Some(("registration.pendingContactSettings.registrantContact.postalAddress.addressLines", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "registration.pending-contact-settings.registrant-contact.postal-address.administrative-area" => Some(("registration.pendingContactSettings.registrantContact.postalAddress.administrativeArea", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.registrant-contact.postal-address.language-code" => Some(("registration.pendingContactSettings.registrantContact.postalAddress.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.registrant-contact.postal-address.locality" => Some(("registration.pendingContactSettings.registrantContact.postalAddress.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.registrant-contact.postal-address.organization" => Some(("registration.pendingContactSettings.registrantContact.postalAddress.organization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.registrant-contact.postal-address.postal-code" => Some(("registration.pendingContactSettings.registrantContact.postalAddress.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.registrant-contact.postal-address.recipients" => Some(("registration.pendingContactSettings.registrantContact.postalAddress.recipients", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "registration.pending-contact-settings.registrant-contact.postal-address.region-code" => Some(("registration.pendingContactSettings.registrantContact.postalAddress.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.registrant-contact.postal-address.revision" => Some(("registration.pendingContactSettings.registrantContact.postalAddress.revision", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.registrant-contact.postal-address.sorting-code" => Some(("registration.pendingContactSettings.registrantContact.postalAddress.sortingCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.registrant-contact.postal-address.sublocality" => Some(("registration.pendingContactSettings.registrantContact.postalAddress.sublocality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.technical-contact.email" => Some(("registration.pendingContactSettings.technicalContact.email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.technical-contact.fax-number" => Some(("registration.pendingContactSettings.technicalContact.faxNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.technical-contact.phone-number" => Some(("registration.pendingContactSettings.technicalContact.phoneNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.technical-contact.postal-address.address-lines" => Some(("registration.pendingContactSettings.technicalContact.postalAddress.addressLines", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "registration.pending-contact-settings.technical-contact.postal-address.administrative-area" => Some(("registration.pendingContactSettings.technicalContact.postalAddress.administrativeArea", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.technical-contact.postal-address.language-code" => Some(("registration.pendingContactSettings.technicalContact.postalAddress.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.technical-contact.postal-address.locality" => Some(("registration.pendingContactSettings.technicalContact.postalAddress.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.technical-contact.postal-address.organization" => Some(("registration.pendingContactSettings.technicalContact.postalAddress.organization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.technical-contact.postal-address.postal-code" => Some(("registration.pendingContactSettings.technicalContact.postalAddress.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.technical-contact.postal-address.recipients" => Some(("registration.pendingContactSettings.technicalContact.postalAddress.recipients", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "registration.pending-contact-settings.technical-contact.postal-address.region-code" => Some(("registration.pendingContactSettings.technicalContact.postalAddress.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.technical-contact.postal-address.revision" => Some(("registration.pendingContactSettings.technicalContact.postalAddress.revision", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.technical-contact.postal-address.sorting-code" => Some(("registration.pendingContactSettings.technicalContact.postalAddress.sortingCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.technical-contact.postal-address.sublocality" => Some(("registration.pendingContactSettings.technicalContact.postalAddress.sublocality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.register-failure-reason" => Some(("registration.registerFailureReason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.state" => Some(("registration.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.supported-privacy" => Some(("registration.supportedPrivacy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "registration.transfer-failure-reason" => Some(("registration.transferFailureReason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "validate-only" => Some(("validateOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "yearly-price.currency-code" => Some(("yearlyPrice.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "yearly-price.nanos" => Some(("yearlyPrice.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "yearly-price.units" => Some(("yearlyPrice.units", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["address-lines", "admin-contact", "administrative-area", "contact-notices", "contact-settings", "create-time", "currency-code", "custom-dns", "dns-settings", "domain-name", "domain-notices", "domain-properties", "ds-state", "effective-transfer-lock-state", "email", "expire-time", "fax-number", "google-domains-dns", "google-domains-redirects-data-available", "issues", "labels", "language-code", "locality", "management-settings", "name", "name-servers", "nanos", "organization", "pending-contact-settings", "phone-number", "postal-address", "postal-code", "preferred-renewal-method", "privacy", "recipients", "region-code", "register-failure-reason", "registrant-contact", "registration", "renewal-method", "revision", "sorting-code", "state", "sublocality", "supported-privacy", "technical-contact", "transfer-failure-reason", "transfer-lock-state", "units", "validate-only", "yearly-price"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::RegisterDomainRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_registrations_register(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_registrations_renew_domain(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "validate-only" => Some(("validateOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "yearly-price.currency-code" => Some(("yearlyPrice.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "yearly-price.nanos" => Some(("yearlyPrice.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "yearly-price.units" => Some(("yearlyPrice.units", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["currency-code", "nanos", "units", "validate-only", "yearly-price"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::RenewDomainRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_registrations_renew_domain(request, opt.value_of("registration").unwrap_or(""));
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

    async fn _projects_locations_registrations_reset_authorization_code(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::ResetAuthorizationCodeRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_registrations_reset_authorization_code(request, opt.value_of("registration").unwrap_or(""));
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

    async fn _projects_locations_registrations_retrieve_authorization_code(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_registrations_retrieve_authorization_code(opt.value_of("registration").unwrap_or(""));
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

    async fn _projects_locations_registrations_retrieve_google_domains_dns_records(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_registrations_retrieve_google_domains_dns_records(opt.value_of("registration").unwrap_or(""));
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

    async fn _projects_locations_registrations_retrieve_google_domains_forwarding_config(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_registrations_retrieve_google_domains_forwarding_config(opt.value_of("registration").unwrap_or(""));
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

    async fn _projects_locations_registrations_retrieve_importable_domains(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_registrations_retrieve_importable_domains(opt.value_of("location").unwrap_or(""));
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

    async fn _projects_locations_registrations_retrieve_register_parameters(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_registrations_retrieve_register_parameters(opt.value_of("location").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "domain-name" => {
                    call = call.domain_name(value.unwrap_or(""));
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
                                                                           v.extend(["domain-name"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _projects_locations_registrations_retrieve_transfer_parameters(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_registrations_retrieve_transfer_parameters(opt.value_of("location").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "domain-name" => {
                    call = call.domain_name(value.unwrap_or(""));
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
                                                                           v.extend(["domain-name"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _projects_locations_registrations_search_domains(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_registrations_search_domains(opt.value_of("location").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "query" => {
                    call = call.query(value.unwrap_or(""));
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
                                                                           v.extend(["query"].iter().map(|v|*v));
                                                                           v } ));
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

    async fn _projects_locations_registrations_set_iam_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.projects().locations_registrations_set_iam_policy(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_locations_registrations_test_iam_permissions(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.projects().locations_registrations_test_iam_permissions(request, opt.value_of("resource").unwrap_or(""));
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

    async fn _projects_locations_registrations_transfer(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "authorization-code.code" => Some(("authorizationCode.code", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "contact-notices" => Some(("contactNotices", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "registration.contact-settings.admin-contact.email" => Some(("registration.contactSettings.adminContact.email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.admin-contact.fax-number" => Some(("registration.contactSettings.adminContact.faxNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.admin-contact.phone-number" => Some(("registration.contactSettings.adminContact.phoneNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.admin-contact.postal-address.address-lines" => Some(("registration.contactSettings.adminContact.postalAddress.addressLines", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "registration.contact-settings.admin-contact.postal-address.administrative-area" => Some(("registration.contactSettings.adminContact.postalAddress.administrativeArea", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.admin-contact.postal-address.language-code" => Some(("registration.contactSettings.adminContact.postalAddress.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.admin-contact.postal-address.locality" => Some(("registration.contactSettings.adminContact.postalAddress.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.admin-contact.postal-address.organization" => Some(("registration.contactSettings.adminContact.postalAddress.organization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.admin-contact.postal-address.postal-code" => Some(("registration.contactSettings.adminContact.postalAddress.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.admin-contact.postal-address.recipients" => Some(("registration.contactSettings.adminContact.postalAddress.recipients", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "registration.contact-settings.admin-contact.postal-address.region-code" => Some(("registration.contactSettings.adminContact.postalAddress.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.admin-contact.postal-address.revision" => Some(("registration.contactSettings.adminContact.postalAddress.revision", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "registration.contact-settings.admin-contact.postal-address.sorting-code" => Some(("registration.contactSettings.adminContact.postalAddress.sortingCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.admin-contact.postal-address.sublocality" => Some(("registration.contactSettings.adminContact.postalAddress.sublocality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.privacy" => Some(("registration.contactSettings.privacy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.registrant-contact.email" => Some(("registration.contactSettings.registrantContact.email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.registrant-contact.fax-number" => Some(("registration.contactSettings.registrantContact.faxNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.registrant-contact.phone-number" => Some(("registration.contactSettings.registrantContact.phoneNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.registrant-contact.postal-address.address-lines" => Some(("registration.contactSettings.registrantContact.postalAddress.addressLines", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "registration.contact-settings.registrant-contact.postal-address.administrative-area" => Some(("registration.contactSettings.registrantContact.postalAddress.administrativeArea", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.registrant-contact.postal-address.language-code" => Some(("registration.contactSettings.registrantContact.postalAddress.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.registrant-contact.postal-address.locality" => Some(("registration.contactSettings.registrantContact.postalAddress.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.registrant-contact.postal-address.organization" => Some(("registration.contactSettings.registrantContact.postalAddress.organization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.registrant-contact.postal-address.postal-code" => Some(("registration.contactSettings.registrantContact.postalAddress.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.registrant-contact.postal-address.recipients" => Some(("registration.contactSettings.registrantContact.postalAddress.recipients", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "registration.contact-settings.registrant-contact.postal-address.region-code" => Some(("registration.contactSettings.registrantContact.postalAddress.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.registrant-contact.postal-address.revision" => Some(("registration.contactSettings.registrantContact.postalAddress.revision", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "registration.contact-settings.registrant-contact.postal-address.sorting-code" => Some(("registration.contactSettings.registrantContact.postalAddress.sortingCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.registrant-contact.postal-address.sublocality" => Some(("registration.contactSettings.registrantContact.postalAddress.sublocality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.technical-contact.email" => Some(("registration.contactSettings.technicalContact.email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.technical-contact.fax-number" => Some(("registration.contactSettings.technicalContact.faxNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.technical-contact.phone-number" => Some(("registration.contactSettings.technicalContact.phoneNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.technical-contact.postal-address.address-lines" => Some(("registration.contactSettings.technicalContact.postalAddress.addressLines", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "registration.contact-settings.technical-contact.postal-address.administrative-area" => Some(("registration.contactSettings.technicalContact.postalAddress.administrativeArea", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.technical-contact.postal-address.language-code" => Some(("registration.contactSettings.technicalContact.postalAddress.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.technical-contact.postal-address.locality" => Some(("registration.contactSettings.technicalContact.postalAddress.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.technical-contact.postal-address.organization" => Some(("registration.contactSettings.technicalContact.postalAddress.organization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.technical-contact.postal-address.postal-code" => Some(("registration.contactSettings.technicalContact.postalAddress.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.technical-contact.postal-address.recipients" => Some(("registration.contactSettings.technicalContact.postalAddress.recipients", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "registration.contact-settings.technical-contact.postal-address.region-code" => Some(("registration.contactSettings.technicalContact.postalAddress.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.technical-contact.postal-address.revision" => Some(("registration.contactSettings.technicalContact.postalAddress.revision", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "registration.contact-settings.technical-contact.postal-address.sorting-code" => Some(("registration.contactSettings.technicalContact.postalAddress.sortingCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.contact-settings.technical-contact.postal-address.sublocality" => Some(("registration.contactSettings.technicalContact.postalAddress.sublocality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.create-time" => Some(("registration.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.dns-settings.custom-dns.name-servers" => Some(("registration.dnsSettings.customDns.nameServers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "registration.dns-settings.google-domains-dns.ds-state" => Some(("registration.dnsSettings.googleDomainsDns.dsState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.dns-settings.google-domains-dns.name-servers" => Some(("registration.dnsSettings.googleDomainsDns.nameServers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "registration.dns-settings.google-domains-redirects-data-available" => Some(("registration.dnsSettings.googleDomainsRedirectsDataAvailable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "registration.domain-name" => Some(("registration.domainName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.domain-properties" => Some(("registration.domainProperties", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "registration.expire-time" => Some(("registration.expireTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.issues" => Some(("registration.issues", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "registration.labels" => Some(("registration.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "registration.management-settings.effective-transfer-lock-state" => Some(("registration.managementSettings.effectiveTransferLockState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.management-settings.preferred-renewal-method" => Some(("registration.managementSettings.preferredRenewalMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.management-settings.renewal-method" => Some(("registration.managementSettings.renewalMethod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.management-settings.transfer-lock-state" => Some(("registration.managementSettings.transferLockState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.name" => Some(("registration.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.admin-contact.email" => Some(("registration.pendingContactSettings.adminContact.email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.admin-contact.fax-number" => Some(("registration.pendingContactSettings.adminContact.faxNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.admin-contact.phone-number" => Some(("registration.pendingContactSettings.adminContact.phoneNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.admin-contact.postal-address.address-lines" => Some(("registration.pendingContactSettings.adminContact.postalAddress.addressLines", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "registration.pending-contact-settings.admin-contact.postal-address.administrative-area" => Some(("registration.pendingContactSettings.adminContact.postalAddress.administrativeArea", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.admin-contact.postal-address.language-code" => Some(("registration.pendingContactSettings.adminContact.postalAddress.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.admin-contact.postal-address.locality" => Some(("registration.pendingContactSettings.adminContact.postalAddress.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.admin-contact.postal-address.organization" => Some(("registration.pendingContactSettings.adminContact.postalAddress.organization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.admin-contact.postal-address.postal-code" => Some(("registration.pendingContactSettings.adminContact.postalAddress.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.admin-contact.postal-address.recipients" => Some(("registration.pendingContactSettings.adminContact.postalAddress.recipients", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "registration.pending-contact-settings.admin-contact.postal-address.region-code" => Some(("registration.pendingContactSettings.adminContact.postalAddress.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.admin-contact.postal-address.revision" => Some(("registration.pendingContactSettings.adminContact.postalAddress.revision", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.admin-contact.postal-address.sorting-code" => Some(("registration.pendingContactSettings.adminContact.postalAddress.sortingCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.admin-contact.postal-address.sublocality" => Some(("registration.pendingContactSettings.adminContact.postalAddress.sublocality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.privacy" => Some(("registration.pendingContactSettings.privacy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.registrant-contact.email" => Some(("registration.pendingContactSettings.registrantContact.email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.registrant-contact.fax-number" => Some(("registration.pendingContactSettings.registrantContact.faxNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.registrant-contact.phone-number" => Some(("registration.pendingContactSettings.registrantContact.phoneNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.registrant-contact.postal-address.address-lines" => Some(("registration.pendingContactSettings.registrantContact.postalAddress.addressLines", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "registration.pending-contact-settings.registrant-contact.postal-address.administrative-area" => Some(("registration.pendingContactSettings.registrantContact.postalAddress.administrativeArea", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.registrant-contact.postal-address.language-code" => Some(("registration.pendingContactSettings.registrantContact.postalAddress.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.registrant-contact.postal-address.locality" => Some(("registration.pendingContactSettings.registrantContact.postalAddress.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.registrant-contact.postal-address.organization" => Some(("registration.pendingContactSettings.registrantContact.postalAddress.organization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.registrant-contact.postal-address.postal-code" => Some(("registration.pendingContactSettings.registrantContact.postalAddress.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.registrant-contact.postal-address.recipients" => Some(("registration.pendingContactSettings.registrantContact.postalAddress.recipients", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "registration.pending-contact-settings.registrant-contact.postal-address.region-code" => Some(("registration.pendingContactSettings.registrantContact.postalAddress.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.registrant-contact.postal-address.revision" => Some(("registration.pendingContactSettings.registrantContact.postalAddress.revision", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.registrant-contact.postal-address.sorting-code" => Some(("registration.pendingContactSettings.registrantContact.postalAddress.sortingCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.registrant-contact.postal-address.sublocality" => Some(("registration.pendingContactSettings.registrantContact.postalAddress.sublocality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.technical-contact.email" => Some(("registration.pendingContactSettings.technicalContact.email", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.technical-contact.fax-number" => Some(("registration.pendingContactSettings.technicalContact.faxNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.technical-contact.phone-number" => Some(("registration.pendingContactSettings.technicalContact.phoneNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.technical-contact.postal-address.address-lines" => Some(("registration.pendingContactSettings.technicalContact.postalAddress.addressLines", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "registration.pending-contact-settings.technical-contact.postal-address.administrative-area" => Some(("registration.pendingContactSettings.technicalContact.postalAddress.administrativeArea", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.technical-contact.postal-address.language-code" => Some(("registration.pendingContactSettings.technicalContact.postalAddress.languageCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.technical-contact.postal-address.locality" => Some(("registration.pendingContactSettings.technicalContact.postalAddress.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.technical-contact.postal-address.organization" => Some(("registration.pendingContactSettings.technicalContact.postalAddress.organization", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.technical-contact.postal-address.postal-code" => Some(("registration.pendingContactSettings.technicalContact.postalAddress.postalCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.technical-contact.postal-address.recipients" => Some(("registration.pendingContactSettings.technicalContact.postalAddress.recipients", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "registration.pending-contact-settings.technical-contact.postal-address.region-code" => Some(("registration.pendingContactSettings.technicalContact.postalAddress.regionCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.technical-contact.postal-address.revision" => Some(("registration.pendingContactSettings.technicalContact.postalAddress.revision", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.technical-contact.postal-address.sorting-code" => Some(("registration.pendingContactSettings.technicalContact.postalAddress.sortingCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.pending-contact-settings.technical-contact.postal-address.sublocality" => Some(("registration.pendingContactSettings.technicalContact.postalAddress.sublocality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.register-failure-reason" => Some(("registration.registerFailureReason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.state" => Some(("registration.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "registration.supported-privacy" => Some(("registration.supportedPrivacy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "registration.transfer-failure-reason" => Some(("registration.transferFailureReason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "validate-only" => Some(("validateOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "yearly-price.currency-code" => Some(("yearlyPrice.currencyCode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "yearly-price.nanos" => Some(("yearlyPrice.nanos", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "yearly-price.units" => Some(("yearlyPrice.units", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["address-lines", "admin-contact", "administrative-area", "authorization-code", "code", "contact-notices", "contact-settings", "create-time", "currency-code", "custom-dns", "dns-settings", "domain-name", "domain-properties", "ds-state", "effective-transfer-lock-state", "email", "expire-time", "fax-number", "google-domains-dns", "google-domains-redirects-data-available", "issues", "labels", "language-code", "locality", "management-settings", "name", "name-servers", "nanos", "organization", "pending-contact-settings", "phone-number", "postal-address", "postal-code", "preferred-renewal-method", "privacy", "recipients", "region-code", "register-failure-reason", "registrant-contact", "registration", "renewal-method", "revision", "sorting-code", "state", "sublocality", "supported-privacy", "technical-contact", "transfer-failure-reason", "transfer-lock-state", "units", "validate-only", "yearly-price"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::TransferDomainRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_registrations_transfer(request, opt.value_of("parent").unwrap_or(""));
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
                    ("locations-get", Some(opt)) => {
                        call_result = self._projects_locations_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-list", Some(opt)) => {
                        call_result = self._projects_locations_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-operations-get", Some(opt)) => {
                        call_result = self._projects_locations_operations_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-operations-list", Some(opt)) => {
                        call_result = self._projects_locations_operations_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-registrations-configure-contact-settings", Some(opt)) => {
                        call_result = self._projects_locations_registrations_configure_contact_settings(opt, dry_run, &mut err).await;
                    },
                    ("locations-registrations-configure-dns-settings", Some(opt)) => {
                        call_result = self._projects_locations_registrations_configure_dns_settings(opt, dry_run, &mut err).await;
                    },
                    ("locations-registrations-configure-management-settings", Some(opt)) => {
                        call_result = self._projects_locations_registrations_configure_management_settings(opt, dry_run, &mut err).await;
                    },
                    ("locations-registrations-delete", Some(opt)) => {
                        call_result = self._projects_locations_registrations_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-registrations-export", Some(opt)) => {
                        call_result = self._projects_locations_registrations_export(opt, dry_run, &mut err).await;
                    },
                    ("locations-registrations-get", Some(opt)) => {
                        call_result = self._projects_locations_registrations_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-registrations-get-iam-policy", Some(opt)) => {
                        call_result = self._projects_locations_registrations_get_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("locations-registrations-import", Some(opt)) => {
                        call_result = self._projects_locations_registrations_import(opt, dry_run, &mut err).await;
                    },
                    ("locations-registrations-initiate-push-transfer", Some(opt)) => {
                        call_result = self._projects_locations_registrations_initiate_push_transfer(opt, dry_run, &mut err).await;
                    },
                    ("locations-registrations-list", Some(opt)) => {
                        call_result = self._projects_locations_registrations_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-registrations-patch", Some(opt)) => {
                        call_result = self._projects_locations_registrations_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-registrations-register", Some(opt)) => {
                        call_result = self._projects_locations_registrations_register(opt, dry_run, &mut err).await;
                    },
                    ("locations-registrations-renew-domain", Some(opt)) => {
                        call_result = self._projects_locations_registrations_renew_domain(opt, dry_run, &mut err).await;
                    },
                    ("locations-registrations-reset-authorization-code", Some(opt)) => {
                        call_result = self._projects_locations_registrations_reset_authorization_code(opt, dry_run, &mut err).await;
                    },
                    ("locations-registrations-retrieve-authorization-code", Some(opt)) => {
                        call_result = self._projects_locations_registrations_retrieve_authorization_code(opt, dry_run, &mut err).await;
                    },
                    ("locations-registrations-retrieve-google-domains-dns-records", Some(opt)) => {
                        call_result = self._projects_locations_registrations_retrieve_google_domains_dns_records(opt, dry_run, &mut err).await;
                    },
                    ("locations-registrations-retrieve-google-domains-forwarding-config", Some(opt)) => {
                        call_result = self._projects_locations_registrations_retrieve_google_domains_forwarding_config(opt, dry_run, &mut err).await;
                    },
                    ("locations-registrations-retrieve-importable-domains", Some(opt)) => {
                        call_result = self._projects_locations_registrations_retrieve_importable_domains(opt, dry_run, &mut err).await;
                    },
                    ("locations-registrations-retrieve-register-parameters", Some(opt)) => {
                        call_result = self._projects_locations_registrations_retrieve_register_parameters(opt, dry_run, &mut err).await;
                    },
                    ("locations-registrations-retrieve-transfer-parameters", Some(opt)) => {
                        call_result = self._projects_locations_registrations_retrieve_transfer_parameters(opt, dry_run, &mut err).await;
                    },
                    ("locations-registrations-search-domains", Some(opt)) => {
                        call_result = self._projects_locations_registrations_search_domains(opt, dry_run, &mut err).await;
                    },
                    ("locations-registrations-set-iam-policy", Some(opt)) => {
                        call_result = self._projects_locations_registrations_set_iam_policy(opt, dry_run, &mut err).await;
                    },
                    ("locations-registrations-test-iam-permissions", Some(opt)) => {
                        call_result = self._projects_locations_registrations_test_iam_permissions(opt, dry_run, &mut err).await;
                    },
                    ("locations-registrations-transfer", Some(opt)) => {
                        call_result = self._projects_locations_registrations_transfer(opt, dry_run, &mut err).await;
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

            match client::application_secret_from_directory(&config_dir, "domains1-beta1-secret.json",
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
        ).persist_tokens_to_disk(format!("{}/domains1-beta1", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::CloudDomains::new(client, auth),
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
        ("projects", "methods: 'locations-get', 'locations-list', 'locations-operations-get', 'locations-operations-list', 'locations-registrations-configure-contact-settings', 'locations-registrations-configure-dns-settings', 'locations-registrations-configure-management-settings', 'locations-registrations-delete', 'locations-registrations-export', 'locations-registrations-get', 'locations-registrations-get-iam-policy', 'locations-registrations-import', 'locations-registrations-initiate-push-transfer', 'locations-registrations-list', 'locations-registrations-patch', 'locations-registrations-register', 'locations-registrations-renew-domain', 'locations-registrations-reset-authorization-code', 'locations-registrations-retrieve-authorization-code', 'locations-registrations-retrieve-google-domains-dns-records', 'locations-registrations-retrieve-google-domains-forwarding-config', 'locations-registrations-retrieve-importable-domains', 'locations-registrations-retrieve-register-parameters', 'locations-registrations-retrieve-transfer-parameters', 'locations-registrations-search-domains', 'locations-registrations-set-iam-policy', 'locations-registrations-test-iam-permissions' and 'locations-registrations-transfer'", vec![
            ("locations-get",
                    Some(r##"Gets information about a location."##),
                    "Details at http://byron.github.io/google-apis-rs/google_domains1_beta1_cli/projects_locations-get",
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
                    "Details at http://byron.github.io/google-apis-rs/google_domains1_beta1_cli/projects_locations-list",
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
            ("locations-operations-get",
                    Some(r##"Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_domains1_beta1_cli/projects_locations-operations-get",
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
                    "Details at http://byron.github.io/google-apis-rs/google_domains1_beta1_cli/projects_locations-operations-list",
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
            ("locations-registrations-configure-contact-settings",
                    Some(r##"Updates a `Registration`'s contact settings. Some changes require confirmation by the domain's registrant contact . Caution: Please consider carefully any changes to contact privacy settings when changing from `REDACTED_CONTACT_DATA` to `PUBLIC_CONTACT_DATA.` There may be a delay in reflecting updates you make to registrant contact information such that any changes you make to contact privacy (including from `REDACTED_CONTACT_DATA` to `PUBLIC_CONTACT_DATA`) will be applied without delay but changes to registrant contact information may take a limited time to be publicized. This means that changes to contact privacy from `REDACTED_CONTACT_DATA` to `PUBLIC_CONTACT_DATA` may make the previous registrant contact data public until the modified registrant contact details are published."##),
                    "Details at http://byron.github.io/google-apis-rs/google_domains1_beta1_cli/projects_locations-registrations-configure-contact-settings",
                  vec![
                    (Some(r##"registration"##),
                     None,
                     Some(r##"Required. The name of the `Registration` whose contact settings are being updated, in the format `projects/*/locations/*/registrations/*`."##),
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
            ("locations-registrations-configure-dns-settings",
                    Some(r##"Updates a `Registration`'s DNS settings."##),
                    "Details at http://byron.github.io/google-apis-rs/google_domains1_beta1_cli/projects_locations-registrations-configure-dns-settings",
                  vec![
                    (Some(r##"registration"##),
                     None,
                     Some(r##"Required. The name of the `Registration` whose DNS settings are being updated, in the format `projects/*/locations/*/registrations/*`."##),
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
            ("locations-registrations-configure-management-settings",
                    Some(r##"Updates a `Registration`'s management settings."##),
                    "Details at http://byron.github.io/google-apis-rs/google_domains1_beta1_cli/projects_locations-registrations-configure-management-settings",
                  vec![
                    (Some(r##"registration"##),
                     None,
                     Some(r##"Required. The name of the `Registration` whose management settings are being updated, in the format `projects/*/locations/*/registrations/*`."##),
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
            ("locations-registrations-delete",
                    Some(r##"Deletes a `Registration` resource. This method works on any `Registration` resource using [Subscription or Commitment billing](/domains/pricing#billing-models), provided that the resource was created at least 1 day in the past. When an active registration is successfully deleted, you can continue to use the domain in [Google Domains](https://domains.google/) until it expires. The calling user becomes the domain's sole owner in Google Domains, and permissions for the domain are subsequently managed there. The domain does not renew automatically unless the new owner sets up billing in Google Domains. After January 2024 you will only be able to delete `Registration` resources when `state` is one of: `EXPORTED`, `EXPIRED`,`REGISTRATION_FAILED` or `TRANSFER_FAILED`. See [Cloud Domains feature deprecation](https://cloud.google.com/domains/docs/deprecations/feature-deprecations) for more details."##),
                    "Details at http://byron.github.io/google-apis-rs/google_domains1_beta1_cli/projects_locations-registrations-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the `Registration` to delete, in the format `projects/*/locations/*/registrations/*`."##),
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
            ("locations-registrations-export",
                    Some(r##"Deprecated: For more information, see [Cloud Domains feature deprecation](https://cloud.google.com/domains/docs/deprecations/feature-deprecations) Exports a `Registration` resource, such that it is no longer managed by Cloud Domains. When an active domain is successfully exported, you can continue to use the domain in [Google Domains](https://domains.google/) until it expires. The calling user becomes the domain's sole owner in Google Domains, and permissions for the domain are subsequently managed there. The domain does not renew automatically unless the new owner sets up billing in Google Domains."##),
                    "Details at http://byron.github.io/google-apis-rs/google_domains1_beta1_cli/projects_locations-registrations-export",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the `Registration` to export, in the format `projects/*/locations/*/registrations/*`."##),
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
            ("locations-registrations-get",
                    Some(r##"Gets the details of a `Registration` resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_domains1_beta1_cli/projects_locations-registrations-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the `Registration` to get, in the format `projects/*/locations/*/registrations/*`."##),
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
            ("locations-registrations-get-iam-policy",
                    Some(r##"Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set."##),
                    "Details at http://byron.github.io/google-apis-rs/google_domains1_beta1_cli/projects_locations-registrations-get-iam-policy",
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
            ("locations-registrations-import",
                    Some(r##"Deprecated: For more information, see [Cloud Domains feature deprecation](https://cloud.google.com/domains/docs/deprecations/feature-deprecations) Imports a domain name from [Google Domains](https://domains.google/) for use in Cloud Domains. To transfer a domain from another registrar, use the `TransferDomain` method instead. Since individual users can own domains in Google Domains, the calling user must have ownership permission on the domain."##),
                    "Details at http://byron.github.io/google-apis-rs/google_domains1_beta1_cli/projects_locations-registrations-import",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource of the Registration. Must be in the format `projects/*/locations/*`."##),
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
            ("locations-registrations-initiate-push-transfer",
                    Some(r##"Initiates the `Push Transfer` process to transfer the domain to another registrar. The process might complete instantly or might require confirmation or additional work. Check the emails sent to the email address of the registrant. The process is aborted after a timeout if it's not completed. This method is only supported for domains that have the `REQUIRE_PUSH_TRANSFER` property in the list of `domain_properties`. The domain must also be unlocked before it can be transferred to a different registrar. For more information, see [Transfer a registered domain to another registrar](https://cloud.google.com/domains/docs/transfer-domain-to-another-registrar)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_domains1_beta1_cli/projects_locations-registrations-initiate-push-transfer",
                  vec![
                    (Some(r##"registration"##),
                     None,
                     Some(r##"Required. The name of the `Registration` for which the push transfer is initiated, in the format `projects/*/locations/*/registrations/*`."##),
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
            ("locations-registrations-list",
                    Some(r##"Lists the `Registration` resources in a project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_domains1_beta1_cli/projects_locations-registrations-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The project and location from which to list `Registration`s, specified in the format `projects/*/locations/*`."##),
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
            ("locations-registrations-patch",
                    Some(r##"Updates select fields of a `Registration` resource, notably `labels`. To update other fields, use the appropriate custom update method: * To update management settings, see `ConfigureManagementSettings` * To update DNS configuration, see `ConfigureDnsSettings` * To update contact information, see `ConfigureContactSettings`"##),
                    "Details at http://byron.github.io/google-apis-rs/google_domains1_beta1_cli/projects_locations-registrations-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Output only. Name of the `Registration` resource, in the format `projects/*/locations/*/registrations/`."##),
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
            ("locations-registrations-register",
                    Some(r##"Registers a new domain name and creates a corresponding `Registration` resource. Call `RetrieveRegisterParameters` first to check availability of the domain name and determine parameters like price that are needed to build a call to this method. A successful call creates a `Registration` resource in state `REGISTRATION_PENDING`, which resolves to `ACTIVE` within 1-2 minutes, indicating that the domain was successfully registered. If the resource ends up in state `REGISTRATION_FAILED`, it indicates that the domain was not registered successfully, and you can safely delete the resource and retry registration."##),
                    "Details at http://byron.github.io/google-apis-rs/google_domains1_beta1_cli/projects_locations-registrations-register",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource of the `Registration`. Must be in the format `projects/*/locations/*`."##),
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
            ("locations-registrations-renew-domain",
                    Some(r##"Renews a recently expired domain. This method can only be called on domains that expired in the previous 30 days. After the renewal, the new expiration time of the domain is one year after the old expiration time and you are charged a `yearly_price` for the renewal."##),
                    "Details at http://byron.github.io/google-apis-rs/google_domains1_beta1_cli/projects_locations-registrations-renew-domain",
                  vec![
                    (Some(r##"registration"##),
                     None,
                     Some(r##"Required. The name of the `Registration` whish is being renewed, in the format `projects/*/locations/*/registrations/*`."##),
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
            ("locations-registrations-reset-authorization-code",
                    Some(r##"Resets the authorization code of the `Registration` to a new random string. You can call this method only after 60 days have elapsed since the initial domain registration. Domains that have the `REQUIRE_PUSH_TRANSFER` property in the list of `domain_properties` don't support authorization codes and must use the `InitiatePushTransfer` method to initiate the process to transfer the domain to a different registrar."##),
                    "Details at http://byron.github.io/google-apis-rs/google_domains1_beta1_cli/projects_locations-registrations-reset-authorization-code",
                  vec![
                    (Some(r##"registration"##),
                     None,
                     Some(r##"Required. The name of the `Registration` whose authorization code is being reset, in the format `projects/*/locations/*/registrations/*`."##),
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
            ("locations-registrations-retrieve-authorization-code",
                    Some(r##"Gets the authorization code of the `Registration` for the purpose of transferring the domain to another registrar. You can call this method only after 60 days have elapsed since the initial domain registration. Domains that have the `REQUIRE_PUSH_TRANSFER` property in the list of `domain_properties` don't support authorization codes and must use the `InitiatePushTransfer` method to initiate the process to transfer the domain to a different registrar."##),
                    "Details at http://byron.github.io/google-apis-rs/google_domains1_beta1_cli/projects_locations-registrations-retrieve-authorization-code",
                  vec![
                    (Some(r##"registration"##),
                     None,
                     Some(r##"Required. The name of the `Registration` whose authorization code is being retrieved, in the format `projects/*/locations/*/registrations/*`."##),
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
            ("locations-registrations-retrieve-google-domains-dns-records",
                    Some(r##"Lists the DNS records from the Google Domains DNS zone for domains that use the deprecated `google_domains_dns` in the `Registration`'s `dns_settings`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_domains1_beta1_cli/projects_locations-registrations-retrieve-google-domains-dns-records",
                  vec![
                    (Some(r##"registration"##),
                     None,
                     Some(r##"Required. The name of the `Registration` whose Google Domains DNS records details you are retrieving, in the format `projects/*/locations/*/registrations/*`."##),
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
            ("locations-registrations-retrieve-google-domains-forwarding-config",
                    Some(r##"Lists the deprecated domain and email forwarding configurations you set up in the deprecated Google Domains UI. The configuration is present only for domains with the `google_domains_redirects_data_available` set to `true` in the `Registration`'s `dns_settings`. A forwarding configuration might not work correctly if required DNS records are not present in the domain's authoritative DNS Zone."##),
                    "Details at http://byron.github.io/google-apis-rs/google_domains1_beta1_cli/projects_locations-registrations-retrieve-google-domains-forwarding-config",
                  vec![
                    (Some(r##"registration"##),
                     None,
                     Some(r##"Required. The name of the `Registration` whose Google Domains forwarding configuration details are being retrieved, in the format `projects/*/locations/*/registrations/*`."##),
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
            ("locations-registrations-retrieve-importable-domains",
                    Some(r##"Deprecated: For more information, see [Cloud Domains feature deprecation](https://cloud.google.com/domains/docs/deprecations/feature-deprecations) Lists domain names from [Google Domains](https://domains.google/) that can be imported to Cloud Domains using the `ImportDomain` method. Since individual users can own domains in Google Domains, the list of domains returned depends on the individual user making the call. Domains already managed by Cloud Domains are not returned."##),
                    "Details at http://byron.github.io/google-apis-rs/google_domains1_beta1_cli/projects_locations-registrations-retrieve-importable-domains",
                  vec![
                    (Some(r##"location"##),
                     None,
                     Some(r##"Required. The location. Must be in the format `projects/*/locations/*`."##),
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
            ("locations-registrations-retrieve-register-parameters",
                    Some(r##"Gets parameters needed to register a new domain name, including price and up-to-date availability. Use the returned values to call `RegisterDomain`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_domains1_beta1_cli/projects_locations-registrations-retrieve-register-parameters",
                  vec![
                    (Some(r##"location"##),
                     None,
                     Some(r##"Required. The location. Must be in the format `projects/*/locations/*`."##),
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
            ("locations-registrations-retrieve-transfer-parameters",
                    Some(r##"Deprecated: For more information, see [Cloud Domains feature deprecation](https://cloud.google.com/domains/docs/deprecations/feature-deprecations) Gets parameters needed to transfer a domain name from another registrar to Cloud Domains. For domains already managed by [Google Domains](https://domains.google/), use `ImportDomain` instead. Use the returned values to call `TransferDomain`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_domains1_beta1_cli/projects_locations-registrations-retrieve-transfer-parameters",
                  vec![
                    (Some(r##"location"##),
                     None,
                     Some(r##"Required. The location. Must be in the format `projects/*/locations/*`."##),
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
            ("locations-registrations-search-domains",
                    Some(r##"Searches for available domain names similar to the provided query. Availability results from this method are approximate; call `RetrieveRegisterParameters` on a domain before registering to confirm availability."##),
                    "Details at http://byron.github.io/google-apis-rs/google_domains1_beta1_cli/projects_locations-registrations-search-domains",
                  vec![
                    (Some(r##"location"##),
                     None,
                     Some(r##"Required. The location. Must be in the format `projects/*/locations/*`."##),
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
            ("locations-registrations-set-iam-policy",
                    Some(r##"Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors."##),
                    "Details at http://byron.github.io/google-apis-rs/google_domains1_beta1_cli/projects_locations-registrations-set-iam-policy",
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
            ("locations-registrations-test-iam-permissions",
                    Some(r##"Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning."##),
                    "Details at http://byron.github.io/google-apis-rs/google_domains1_beta1_cli/projects_locations-registrations-test-iam-permissions",
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
            ("locations-registrations-transfer",
                    Some(r##"Deprecated: For more information, see [Cloud Domains feature deprecation](https://cloud.google.com/domains/docs/deprecations/feature-deprecations) Transfers a domain name from another registrar to Cloud Domains. For domains already managed by [Google Domains](https://domains.google/), use `ImportDomain` instead. Before calling this method, go to the domain's current registrar to unlock the domain for transfer and retrieve the domain's transfer authorization code. Then call `RetrieveTransferParameters` to confirm that the domain is unlocked and to get values needed to build a call to this method. A successful call creates a `Registration` resource in state `TRANSFER_PENDING`. It can take several days to complete the transfer process. The registrant can often speed up this process by approving the transfer through the current registrar, either by clicking a link in an email from the registrar or by visiting the registrar's website. A few minutes after transfer approval, the resource transitions to state `ACTIVE`, indicating that the transfer was successful. If the transfer is rejected or the request expires without being approved, the resource can end up in state `TRANSFER_FAILED`. If transfer fails, you can safely delete the resource and retry the transfer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_domains1_beta1_cli/projects_locations-registrations-transfer",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource of the `Registration`. Must be in the format `projects/*/locations/*`."##),
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
    
    let mut app = App::new("domains1-beta1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+20240610")
           .about("Enables management and configuration of domain names.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_domains1_beta1_cli")
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
