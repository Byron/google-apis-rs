// DO NOT EDIT !
// This file was generated automatically from 'src/mako/cli/main.rs.mako'
// DO NOT EDIT !
#![feature(plugin, exit_status)]
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;
extern crate yup_oauth2 as oauth2;
extern crate yup_hyper_mock as mock;
extern crate serde;
extern crate hyper;
extern crate mime;
extern crate strsim;
extern crate google_reseller1_sandbox as api;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

mod cmn;

use cmn::{InvalidOptionsError, CLIError, JsonTokenStorage, arg_from_str, writer_from_opts, parse_kv_arg, 
          input_file_from_opts, input_mime_from_opts, FieldCursor, FieldError, CallType, UploadProtocol,
          calltype_from_str, remove_json_null_values};

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
    hub: api::Reseller<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n, 'a> Engine<'n, 'a> {
    fn _customers_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.customers().get(opt.value_of("customer-id").unwrap_or(""));
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _customers_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Customer::default();
        let mut field_cursor = FieldCursor::default();
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
            fn request_postal_address_init(request: &mut api::Customer) {
                if request.postal_address.is_none() {
                    request.postal_address = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "customer-domain" => {
                        request.customer_domain = Some(value.unwrap_or("").to_string());
                    },
                "alternate-email" => {
                        request.alternate_email = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "resource-ui-url" => {
                        request.resource_ui_url = Some(value.unwrap_or("").to_string());
                    },
                "phone-number" => {
                        request.phone_number = Some(value.unwrap_or("").to_string());
                    },
                "postal-address.kind" => {
                        request_postal_address_init(&mut request);
                        request.postal_address.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "postal-address.organization-name" => {
                        request_postal_address_init(&mut request);
                        request.postal_address.as_mut().unwrap().organization_name = Some(value.unwrap_or("").to_string());
                    },
                "postal-address.country-code" => {
                        request_postal_address_init(&mut request);
                        request.postal_address.as_mut().unwrap().country_code = Some(value.unwrap_or("").to_string());
                    },
                "postal-address.locality" => {
                        request_postal_address_init(&mut request);
                        request.postal_address.as_mut().unwrap().locality = Some(value.unwrap_or("").to_string());
                    },
                "postal-address.region" => {
                        request_postal_address_init(&mut request);
                        request.postal_address.as_mut().unwrap().region = Some(value.unwrap_or("").to_string());
                    },
                "postal-address.address-line2" => {
                        request_postal_address_init(&mut request);
                        request.postal_address.as_mut().unwrap().address_line2 = Some(value.unwrap_or("").to_string());
                    },
                "postal-address.address-line3" => {
                        request_postal_address_init(&mut request);
                        request.postal_address.as_mut().unwrap().address_line3 = Some(value.unwrap_or("").to_string());
                    },
                "postal-address.contact-name" => {
                        request_postal_address_init(&mut request);
                        request.postal_address.as_mut().unwrap().contact_name = Some(value.unwrap_or("").to_string());
                    },
                "postal-address.address-line1" => {
                        request_postal_address_init(&mut request);
                        request.postal_address.as_mut().unwrap().address_line1 = Some(value.unwrap_or("").to_string());
                    },
                "postal-address.postal-code" => {
                        request_postal_address_init(&mut request);
                        request.postal_address.as_mut().unwrap().postal_code = Some(value.unwrap_or("").to_string());
                    },
                "customer-id" => {
                        request_postal_address_init(&mut request);
                        request.customer_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["address-line1", "address-line2", "address-line3", "alternate-email", "contact-name", "country-code", "customer-domain", "customer-id", "kind", "locality", "organization-name", "phone-number", "postal-address", "postal-code", "region", "resource-ui-url"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.customers().insert(request);
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "customer-auth-token" => {
                    call = call.customer_auth_token(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["customer-auth-token"]
                                                            ));
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _customers_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Customer::default();
        let mut field_cursor = FieldCursor::default();
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
            fn request_postal_address_init(request: &mut api::Customer) {
                if request.postal_address.is_none() {
                    request.postal_address = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "customer-domain" => {
                        request.customer_domain = Some(value.unwrap_or("").to_string());
                    },
                "alternate-email" => {
                        request.alternate_email = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "resource-ui-url" => {
                        request.resource_ui_url = Some(value.unwrap_or("").to_string());
                    },
                "phone-number" => {
                        request.phone_number = Some(value.unwrap_or("").to_string());
                    },
                "postal-address.kind" => {
                        request_postal_address_init(&mut request);
                        request.postal_address.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "postal-address.organization-name" => {
                        request_postal_address_init(&mut request);
                        request.postal_address.as_mut().unwrap().organization_name = Some(value.unwrap_or("").to_string());
                    },
                "postal-address.country-code" => {
                        request_postal_address_init(&mut request);
                        request.postal_address.as_mut().unwrap().country_code = Some(value.unwrap_or("").to_string());
                    },
                "postal-address.locality" => {
                        request_postal_address_init(&mut request);
                        request.postal_address.as_mut().unwrap().locality = Some(value.unwrap_or("").to_string());
                    },
                "postal-address.region" => {
                        request_postal_address_init(&mut request);
                        request.postal_address.as_mut().unwrap().region = Some(value.unwrap_or("").to_string());
                    },
                "postal-address.address-line2" => {
                        request_postal_address_init(&mut request);
                        request.postal_address.as_mut().unwrap().address_line2 = Some(value.unwrap_or("").to_string());
                    },
                "postal-address.address-line3" => {
                        request_postal_address_init(&mut request);
                        request.postal_address.as_mut().unwrap().address_line3 = Some(value.unwrap_or("").to_string());
                    },
                "postal-address.contact-name" => {
                        request_postal_address_init(&mut request);
                        request.postal_address.as_mut().unwrap().contact_name = Some(value.unwrap_or("").to_string());
                    },
                "postal-address.address-line1" => {
                        request_postal_address_init(&mut request);
                        request.postal_address.as_mut().unwrap().address_line1 = Some(value.unwrap_or("").to_string());
                    },
                "postal-address.postal-code" => {
                        request_postal_address_init(&mut request);
                        request.postal_address.as_mut().unwrap().postal_code = Some(value.unwrap_or("").to_string());
                    },
                "customer-id" => {
                        request_postal_address_init(&mut request);
                        request.customer_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["address-line1", "address-line2", "address-line3", "alternate-email", "contact-name", "country-code", "customer-domain", "customer-id", "kind", "locality", "organization-name", "phone-number", "postal-address", "postal-code", "region", "resource-ui-url"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.customers().patch(request, opt.value_of("customer-id").unwrap_or(""));
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _customers_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Customer::default();
        let mut field_cursor = FieldCursor::default();
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
            fn request_postal_address_init(request: &mut api::Customer) {
                if request.postal_address.is_none() {
                    request.postal_address = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "customer-domain" => {
                        request.customer_domain = Some(value.unwrap_or("").to_string());
                    },
                "alternate-email" => {
                        request.alternate_email = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "resource-ui-url" => {
                        request.resource_ui_url = Some(value.unwrap_or("").to_string());
                    },
                "phone-number" => {
                        request.phone_number = Some(value.unwrap_or("").to_string());
                    },
                "postal-address.kind" => {
                        request_postal_address_init(&mut request);
                        request.postal_address.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "postal-address.organization-name" => {
                        request_postal_address_init(&mut request);
                        request.postal_address.as_mut().unwrap().organization_name = Some(value.unwrap_or("").to_string());
                    },
                "postal-address.country-code" => {
                        request_postal_address_init(&mut request);
                        request.postal_address.as_mut().unwrap().country_code = Some(value.unwrap_or("").to_string());
                    },
                "postal-address.locality" => {
                        request_postal_address_init(&mut request);
                        request.postal_address.as_mut().unwrap().locality = Some(value.unwrap_or("").to_string());
                    },
                "postal-address.region" => {
                        request_postal_address_init(&mut request);
                        request.postal_address.as_mut().unwrap().region = Some(value.unwrap_or("").to_string());
                    },
                "postal-address.address-line2" => {
                        request_postal_address_init(&mut request);
                        request.postal_address.as_mut().unwrap().address_line2 = Some(value.unwrap_or("").to_string());
                    },
                "postal-address.address-line3" => {
                        request_postal_address_init(&mut request);
                        request.postal_address.as_mut().unwrap().address_line3 = Some(value.unwrap_or("").to_string());
                    },
                "postal-address.contact-name" => {
                        request_postal_address_init(&mut request);
                        request.postal_address.as_mut().unwrap().contact_name = Some(value.unwrap_or("").to_string());
                    },
                "postal-address.address-line1" => {
                        request_postal_address_init(&mut request);
                        request.postal_address.as_mut().unwrap().address_line1 = Some(value.unwrap_or("").to_string());
                    },
                "postal-address.postal-code" => {
                        request_postal_address_init(&mut request);
                        request.postal_address.as_mut().unwrap().postal_code = Some(value.unwrap_or("").to_string());
                    },
                "customer-id" => {
                        request_postal_address_init(&mut request);
                        request.customer_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["address-line1", "address-line2", "address-line3", "alternate-email", "contact-name", "country-code", "customer-domain", "customer-id", "kind", "locality", "organization-name", "phone-number", "postal-address", "postal-code", "region", "resource-ui-url"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.customers().update(request, opt.value_of("customer-id").unwrap_or(""));
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _subscriptions_activate(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.subscriptions().activate(opt.value_of("customer-id").unwrap_or(""), opt.value_of("subscription-id").unwrap_or(""));
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _subscriptions_change_plan(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::ChangePlanRequest::default();
        let mut field_cursor = FieldCursor::default();
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
            fn request_seats_init(request: &mut api::ChangePlanRequest) {
                if request.seats.is_none() {
                    request.seats = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "plan-name" => {
                        request.plan_name = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "seats.kind" => {
                        request_seats_init(&mut request);
                        request.seats.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "seats.number-of-seats" => {
                        request_seats_init(&mut request);
                        request.seats.as_mut().unwrap().number_of_seats = Some(arg_from_str(value.unwrap_or("-0"), err, "seats.number-of-seats", "integer"));
                    },
                "seats.maximum-number-of-seats" => {
                        request_seats_init(&mut request);
                        request.seats.as_mut().unwrap().maximum_number_of_seats = Some(arg_from_str(value.unwrap_or("-0"), err, "seats.maximum-number-of-seats", "integer"));
                    },
                "seats.licensed-number-of-seats" => {
                        request_seats_init(&mut request);
                        request.seats.as_mut().unwrap().licensed_number_of_seats = Some(arg_from_str(value.unwrap_or("-0"), err, "seats.licensed-number-of-seats", "integer"));
                    },
                "purchase-order-id" => {
                        request_seats_init(&mut request);
                        request.purchase_order_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["kind", "licensed-number-of-seats", "maximum-number-of-seats", "number-of-seats", "plan-name", "purchase-order-id", "seats"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.subscriptions().change_plan(request, opt.value_of("customer-id").unwrap_or(""), opt.value_of("subscription-id").unwrap_or(""));
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _subscriptions_change_renewal_settings(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::RenewalSettings::default();
        let mut field_cursor = FieldCursor::default();
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
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "renewal-type" => {
                        request.renewal_type = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["kind", "renewal-type"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.subscriptions().change_renewal_settings(request, opt.value_of("customer-id").unwrap_or(""), opt.value_of("subscription-id").unwrap_or(""));
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _subscriptions_change_seats(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Seats::default();
        let mut field_cursor = FieldCursor::default();
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
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "number-of-seats" => {
                        request.number_of_seats = Some(arg_from_str(value.unwrap_or("-0"), err, "number-of-seats", "integer"));
                    },
                "maximum-number-of-seats" => {
                        request.maximum_number_of_seats = Some(arg_from_str(value.unwrap_or("-0"), err, "maximum-number-of-seats", "integer"));
                    },
                "licensed-number-of-seats" => {
                        request.licensed_number_of_seats = Some(arg_from_str(value.unwrap_or("-0"), err, "licensed-number-of-seats", "integer"));
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["kind", "licensed-number-of-seats", "maximum-number-of-seats", "number-of-seats"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.subscriptions().change_seats(request, opt.value_of("customer-id").unwrap_or(""), opt.value_of("subscription-id").unwrap_or(""));
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _subscriptions_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.subscriptions().delete(opt.value_of("customer-id").unwrap_or(""), opt.value_of("subscription-id").unwrap_or(""), opt.value_of("deletion-type").unwrap_or(""));
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
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

    fn _subscriptions_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.subscriptions().get(opt.value_of("customer-id").unwrap_or(""), opt.value_of("subscription-id").unwrap_or(""));
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _subscriptions_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Subscription::default();
        let mut field_cursor = FieldCursor::default();
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
            fn request_plan_commitment_interval_init(request: &mut api::Subscription) {
                request_plan_init(request);
                if request.plan.as_mut().unwrap().commitment_interval.is_none() {
                    request.plan.as_mut().unwrap().commitment_interval = Some(Default::default());
                }
            }
            
            fn request_plan_init(request: &mut api::Subscription) {
                if request.plan.is_none() {
                    request.plan = Some(Default::default());
                }
            }
            
            fn request_renewal_settings_init(request: &mut api::Subscription) {
                if request.renewal_settings.is_none() {
                    request.renewal_settings = Some(Default::default());
                }
            }
            
            fn request_seats_init(request: &mut api::Subscription) {
                if request.seats.is_none() {
                    request.seats = Some(Default::default());
                }
            }
            
            fn request_transfer_info_init(request: &mut api::Subscription) {
                if request.transfer_info.is_none() {
                    request.transfer_info = Some(Default::default());
                }
            }
            
            fn request_trial_settings_init(request: &mut api::Subscription) {
                if request.trial_settings.is_none() {
                    request.trial_settings = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "renewal-settings.kind" => {
                        request_renewal_settings_init(&mut request);
                        request.renewal_settings.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "renewal-settings.renewal-type" => {
                        request_renewal_settings_init(&mut request);
                        request.renewal_settings.as_mut().unwrap().renewal_type = Some(value.unwrap_or("").to_string());
                    },
                "sku-id" => {
                        request_renewal_settings_init(&mut request);
                        request.sku_id = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_renewal_settings_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "trial-settings.trial-end-time" => {
                        request_trial_settings_init(&mut request);
                        request.trial_settings.as_mut().unwrap().trial_end_time = Some(value.unwrap_or("").to_string());
                    },
                "trial-settings.is-in-trial" => {
                        request_trial_settings_init(&mut request);
                        request.trial_settings.as_mut().unwrap().is_in_trial = Some(arg_from_str(value.unwrap_or("false"), err, "trial-settings.is-in-trial", "boolean"));
                    },
                "transfer-info.transferability-expiration-time" => {
                        request_transfer_info_init(&mut request);
                        request.transfer_info.as_mut().unwrap().transferability_expiration_time = Some(value.unwrap_or("").to_string());
                    },
                "transfer-info.minimum-transferable-seats" => {
                        request_transfer_info_init(&mut request);
                        request.transfer_info.as_mut().unwrap().minimum_transferable_seats = Some(arg_from_str(value.unwrap_or("-0"), err, "transfer-info.minimum-transferable-seats", "integer"));
                    },
                "resource-ui-url" => {
                        request_transfer_info_init(&mut request);
                        request.resource_ui_url = Some(value.unwrap_or("").to_string());
                    },
                "seats.kind" => {
                        request_seats_init(&mut request);
                        request.seats.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "seats.number-of-seats" => {
                        request_seats_init(&mut request);
                        request.seats.as_mut().unwrap().number_of_seats = Some(arg_from_str(value.unwrap_or("-0"), err, "seats.number-of-seats", "integer"));
                    },
                "seats.maximum-number-of-seats" => {
                        request_seats_init(&mut request);
                        request.seats.as_mut().unwrap().maximum_number_of_seats = Some(arg_from_str(value.unwrap_or("-0"), err, "seats.maximum-number-of-seats", "integer"));
                    },
                "seats.licensed-number-of-seats" => {
                        request_seats_init(&mut request);
                        request.seats.as_mut().unwrap().licensed_number_of_seats = Some(arg_from_str(value.unwrap_or("-0"), err, "seats.licensed-number-of-seats", "integer"));
                    },
                "creation-time" => {
                        request_seats_init(&mut request);
                        request.creation_time = Some(value.unwrap_or("").to_string());
                    },
                "status" => {
                        request_seats_init(&mut request);
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "plan.plan-name" => {
                        request_plan_init(&mut request);
                        request.plan.as_mut().unwrap().plan_name = Some(value.unwrap_or("").to_string());
                    },
                "plan.commitment-interval.end-time" => {
                        request_plan_commitment_interval_init(&mut request);
                        request.plan.as_mut().unwrap().commitment_interval.as_mut().unwrap().end_time = Some(value.unwrap_or("").to_string());
                    },
                "plan.commitment-interval.start-time" => {
                        request_plan_commitment_interval_init(&mut request);
                        request.plan.as_mut().unwrap().commitment_interval.as_mut().unwrap().start_time = Some(value.unwrap_or("").to_string());
                    },
                "plan.is-commitment-plan" => {
                        request_plan_commitment_interval_init(&mut request);
                        request.plan.as_mut().unwrap().is_commitment_plan = Some(arg_from_str(value.unwrap_or("false"), err, "plan.is-commitment-plan", "boolean"));
                    },
                "purchase-order-id" => {
                        request_plan_init(&mut request);
                        request.purchase_order_id = Some(value.unwrap_or("").to_string());
                    },
                "subscription-id" => {
                        request_plan_init(&mut request);
                        request.subscription_id = Some(value.unwrap_or("").to_string());
                    },
                "billing-method" => {
                        request_plan_init(&mut request);
                        request.billing_method = Some(value.unwrap_or("").to_string());
                    },
                "customer-id" => {
                        request_plan_init(&mut request);
                        request.customer_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["billing-method", "commitment-interval", "creation-time", "customer-id", "end-time", "is-commitment-plan", "is-in-trial", "kind", "licensed-number-of-seats", "maximum-number-of-seats", "minimum-transferable-seats", "number-of-seats", "plan", "plan-name", "purchase-order-id", "renewal-settings", "renewal-type", "resource-ui-url", "seats", "sku-id", "start-time", "status", "subscription-id", "transfer-info", "transferability-expiration-time", "trial-end-time", "trial-settings"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.subscriptions().insert(request, opt.value_of("customer-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "customer-auth-token" => {
                    call = call.customer_auth_token(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["customer-auth-token"]
                                                            ));
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _subscriptions_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.subscriptions().list();
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "customer-name-prefix" => {
                    call = call.customer_name_prefix(value.unwrap_or(""));
                },
                "customer-id" => {
                    call = call.customer_id(value.unwrap_or(""));
                },
                "customer-auth-token" => {
                    call = call.customer_auth_token(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["customer-auth-token", "page-token", "customer-id", "max-results", "customer-name-prefix"]
                                                            ));
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _subscriptions_start_paid_service(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.subscriptions().start_paid_service(opt.value_of("customer-id").unwrap_or(""), opt.value_of("subscription-id").unwrap_or(""));
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _subscriptions_suspend(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.subscriptions().suspend(opt.value_of("customer-id").unwrap_or(""), opt.value_of("subscription-id").unwrap_or(""));
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
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
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
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
            ("customers", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._customers_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._customers_insert(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._customers_patch(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._customers_update(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("customers".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("subscriptions", Some(opt)) => {
                match opt.subcommand() {
                    ("activate", Some(opt)) => {
                        call_result = self._subscriptions_activate(opt, dry_run, &mut err);
                    },
                    ("change-plan", Some(opt)) => {
                        call_result = self._subscriptions_change_plan(opt, dry_run, &mut err);
                    },
                    ("change-renewal-settings", Some(opt)) => {
                        call_result = self._subscriptions_change_renewal_settings(opt, dry_run, &mut err);
                    },
                    ("change-seats", Some(opt)) => {
                        call_result = self._subscriptions_change_seats(opt, dry_run, &mut err);
                    },
                    ("delete", Some(opt)) => {
                        call_result = self._subscriptions_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._subscriptions_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._subscriptions_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._subscriptions_list(opt, dry_run, &mut err);
                    },
                    ("start-paid-service", Some(opt)) => {
                        call_result = self._subscriptions_start_paid_service(opt, dry_run, &mut err);
                    },
                    ("suspend", Some(opt)) => {
                        call_result = self._subscriptions_suspend(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("subscriptions".to_string()));
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

            match cmn::application_secret_from_directory(&config_dir, "reseller1-sandbox-secret.json", 
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
                                          program_name: "reseller1-sandbox",
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
            hub: api::Reseller::new(client, auth),
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
    let arg_data = [
        ("customers", "methods: 'get', 'insert', 'patch' and 'update'", vec![
            ("get",  
                    Some(r##"Gets a customer resource if one exists and is owned by the reseller."##),
                    "Details at http://byron.github.io/google-apis-rs/google_reseller1_sandbox_cli/customers_get",
                  vec![
                    (Some(r##"customer-id"##),
                     None,
                     Some(r##"Id of the Customer"##),
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
                    Some(r##"Creates a customer resource if one does not already exist."##),
                    "Details at http://byron.github.io/google-apis-rs/google_reseller1_sandbox_cli/customers_insert",
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
                    Some(r##"Update a customer resource if one it exists and is owned by the reseller. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_reseller1_sandbox_cli/customers_patch",
                  vec![
                    (Some(r##"customer-id"##),
                     None,
                     Some(r##"Id of the Customer"##),
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
                    Some(r##"Update a customer resource if one it exists and is owned by the reseller."##),
                    "Details at http://byron.github.io/google-apis-rs/google_reseller1_sandbox_cli/customers_update",
                  vec![
                    (Some(r##"customer-id"##),
                     None,
                     Some(r##"Id of the Customer"##),
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
        
        ("subscriptions", "methods: 'activate', 'change-plan', 'change-renewal-settings', 'change-seats', 'delete', 'get', 'insert', 'list', 'start-paid-service' and 'suspend'", vec![
            ("activate",  
                    Some(r##"Activates a subscription previously suspended by the reseller"##),
                    "Details at http://byron.github.io/google-apis-rs/google_reseller1_sandbox_cli/subscriptions_activate",
                  vec![
                    (Some(r##"customer-id"##),
                     None,
                     Some(r##"Id of the Customer"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"subscription-id"##),
                     None,
                     Some(r##"Id of the subscription, which is unique for a customer"##),
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
            ("change-plan",  
                    Some(r##"Changes the plan of a subscription"##),
                    "Details at http://byron.github.io/google-apis-rs/google_reseller1_sandbox_cli/subscriptions_change-plan",
                  vec![
                    (Some(r##"customer-id"##),
                     None,
                     Some(r##"Id of the Customer"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"subscription-id"##),
                     None,
                     Some(r##"Id of the subscription, which is unique for a customer"##),
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
            ("change-renewal-settings",  
                    Some(r##"Changes the renewal settings of a subscription"##),
                    "Details at http://byron.github.io/google-apis-rs/google_reseller1_sandbox_cli/subscriptions_change-renewal-settings",
                  vec![
                    (Some(r##"customer-id"##),
                     None,
                     Some(r##"Id of the Customer"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"subscription-id"##),
                     None,
                     Some(r##"Id of the subscription, which is unique for a customer"##),
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
            ("change-seats",  
                    Some(r##"Changes the seats configuration of a subscription"##),
                    "Details at http://byron.github.io/google-apis-rs/google_reseller1_sandbox_cli/subscriptions_change-seats",
                  vec![
                    (Some(r##"customer-id"##),
                     None,
                     Some(r##"Id of the Customer"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"subscription-id"##),
                     None,
                     Some(r##"Id of the subscription, which is unique for a customer"##),
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
                    Some(r##"Cancels/Downgrades a subscription."##),
                    "Details at http://byron.github.io/google-apis-rs/google_reseller1_sandbox_cli/subscriptions_delete",
                  vec![
                    (Some(r##"customer-id"##),
                     None,
                     Some(r##"Id of the Customer"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"subscription-id"##),
                     None,
                     Some(r##"Id of the subscription, which is unique for a customer"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"deletion-type"##),
                     None,
                     Some(r##"Whether the subscription is to be fully cancelled or downgraded"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",  
                    Some(r##"Gets a subscription of the customer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_reseller1_sandbox_cli/subscriptions_get",
                  vec![
                    (Some(r##"customer-id"##),
                     None,
                     Some(r##"Id of the Customer"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"subscription-id"##),
                     None,
                     Some(r##"Id of the subscription, which is unique for a customer"##),
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
                    Some(r##"Creates/Transfers a subscription for the customer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_reseller1_sandbox_cli/subscriptions_insert",
                  vec![
                    (Some(r##"customer-id"##),
                     None,
                     Some(r##"Id of the Customer"##),
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
                    Some(r##"Lists subscriptions of a reseller, optionally filtered by a customer name prefix."##),
                    "Details at http://byron.github.io/google-apis-rs/google_reseller1_sandbox_cli/subscriptions_list",
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
            ("start-paid-service",  
                    Some(r##"Starts paid service of a trial subscription"##),
                    "Details at http://byron.github.io/google-apis-rs/google_reseller1_sandbox_cli/subscriptions_start-paid-service",
                  vec![
                    (Some(r##"customer-id"##),
                     None,
                     Some(r##"Id of the Customer"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"subscription-id"##),
                     None,
                     Some(r##"Id of the subscription, which is unique for a customer"##),
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
            ("suspend",  
                    Some(r##"Suspends an active subscription"##),
                    "Details at http://byron.github.io/google-apis-rs/google_reseller1_sandbox_cli/subscriptions_suspend",
                  vec![
                    (Some(r##"customer-id"##),
                     None,
                     Some(r##"Id of the Customer"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"subscription-id"##),
                     None,
                     Some(r##"Id of the subscription, which is unique for a customer"##),
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
    
    let mut app = App::new("reseller1-sandbox")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("0.2.0+20141112")
           .about("Lets you create and manage your customers and their subscriptions.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_reseller1_sandbox_cli")
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
                       let mut arg = Arg::with_name(arg_name_str);
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
            env::set_exit_status(err.exit_code);
            writeln!(io::stderr(), "{}", err).ok();
        },
        Ok(engine) => {
            if let Err(doit_err) = engine.doit() {
                env::set_exit_status(1);
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
}