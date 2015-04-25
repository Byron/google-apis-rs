// DO NOT EDIT !
// This file was generated automatically from 'src/mako/cli/main.rs.mako'
// DO NOT EDIT !
#![feature(plugin, exit_status)]
#![plugin(docopt_macros)]
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

extern crate docopt;
extern crate yup_oauth2 as oauth2;
extern crate yup_hyper_mock as mock;
extern crate rustc_serialize;
extern crate serde;
extern crate hyper;
extern crate mime;
extern crate google_reseller1_sandbox as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  reseller1-sandbox [options] customers get <customer-id> [-p <v>]... [-o <out>]
  reseller1-sandbox [options] customers insert -r <kv>... [-p <v>]... [-o <out>]
  reseller1-sandbox [options] customers patch <customer-id> -r <kv>... [-p <v>]... [-o <out>]
  reseller1-sandbox [options] customers update <customer-id> -r <kv>... [-p <v>]... [-o <out>]
  reseller1-sandbox [options] subscriptions activate <customer-id> <subscription-id> [-p <v>]... [-o <out>]
  reseller1-sandbox [options] subscriptions change-plan <customer-id> <subscription-id> -r <kv>... [-p <v>]... [-o <out>]
  reseller1-sandbox [options] subscriptions change-renewal-settings <customer-id> <subscription-id> -r <kv>... [-p <v>]... [-o <out>]
  reseller1-sandbox [options] subscriptions change-seats <customer-id> <subscription-id> -r <kv>... [-p <v>]... [-o <out>]
  reseller1-sandbox [options] subscriptions delete <customer-id> <subscription-id> <deletion-type> [-p <v>]...
  reseller1-sandbox [options] subscriptions get <customer-id> <subscription-id> [-p <v>]... [-o <out>]
  reseller1-sandbox [options] subscriptions insert <customer-id> -r <kv>... [-p <v>]... [-o <out>]
  reseller1-sandbox [options] subscriptions list [-p <v>]... [-o <out>]
  reseller1-sandbox [options] subscriptions start-paid-service <customer-id> <subscription-id> [-p <v>]... [-o <out>]
  reseller1-sandbox [options] subscriptions suspend <customer-id> <subscription-id> [-p <v>]... [-o <out>]
  reseller1-sandbox --help

All documentation details can be found TODO: <URL to github.io docs here, see #51>

Configuration:
  --scope <url>  
            Specify the authentication a method should be executed in. Each scope requires
            the user to grant this application permission to use it.
            If unset, it defaults to the shortest scope url for a particular method.
  --config-dir <folder>
            A directory into which we will store our persistent data. Defaults to a user-writable
            directory that we will create during the first invocation.
            [default: ~/.google-service-cli]
  --debug
            Output all server communication to standard error. `tx` and `rx` are placed into 
            the same stream.
  --debug-auth
            Output all communication related to authentication to standard error. `tx` and `rx` are placed into 
            the same stream.
");

mod cmn;
use cmn::{InvalidOptionsError, CLIError, JsonTokenStorage, arg_from_str, writer_from_opts, parse_kv_arg, 
          input_file_from_opts, input_mime_from_opts, FieldCursor, FieldError};

use std::default::Default;
use std::str::FromStr;

use oauth2::{Authenticator, DefaultAuthenticatorDelegate};
use rustc_serialize::json;

struct Engine {
    opt: Options,
    hub: api::Reseller<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _customers_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.customers().get(&self.opt.arg_customer_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _customers_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Customer::default();
        let mut call = self.hub.customers().insert(&request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "customer-auth-token" => {
                    call = call.customer_auth_token(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        
        let mut field_name = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_postal_address_init(request: &mut api::Customer) {
                if request.postal_address.is_none() {
                    request.postal_address = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
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
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _customers_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Customer::default();
        let mut call = self.hub.customers().patch(&request, &self.opt.arg_customer_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        
        let mut field_name = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_postal_address_init(request: &mut api::Customer) {
                if request.postal_address.is_none() {
                    request.postal_address = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
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
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _customers_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Customer::default();
        let mut call = self.hub.customers().update(&request, &self.opt.arg_customer_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        
        let mut field_name = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_postal_address_init(request: &mut api::Customer) {
                if request.postal_address.is_none() {
                    request.postal_address = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
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
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _subscriptions_activate(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.subscriptions().activate(&self.opt.arg_customer_id, &self.opt.arg_subscription_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _subscriptions_change_plan(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::ChangePlanRequest::default();
        let mut call = self.hub.subscriptions().change_plan(&request, &self.opt.arg_customer_id, &self.opt.arg_subscription_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        
        let mut field_name = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_seats_init(request: &mut api::ChangePlanRequest) {
                if request.seats.is_none() {
                    request.seats = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
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
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _subscriptions_change_renewal_settings(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::RenewalSettings::default();
        let mut call = self.hub.subscriptions().change_renewal_settings(&request, &self.opt.arg_customer_id, &self.opt.arg_subscription_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        
        let mut field_name = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            match &field_name.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "renewal-type" => {
                        request.renewal_type = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _subscriptions_change_seats(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Seats::default();
        let mut call = self.hub.subscriptions().change_seats(&request, &self.opt.arg_customer_id, &self.opt.arg_subscription_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        
        let mut field_name = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            match &field_name.to_string()[..] {
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
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _subscriptions_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.subscriptions().delete(&self.opt.arg_customer_id, &self.opt.arg_subscription_id, &self.opt.arg_deletion_type);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok(mut response) => {
                    None
                }
            }
        }
    }

    fn _subscriptions_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.subscriptions().get(&self.opt.arg_customer_id, &self.opt.arg_subscription_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _subscriptions_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Subscription::default();
        let mut call = self.hub.subscriptions().insert(&request, &self.opt.arg_customer_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "customer-auth-token" => {
                    call = call.customer_auth_token(value.unwrap_or(""));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        
        let mut field_name = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
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
            
            match &field_name.to_string()[..] {
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
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _subscriptions_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.subscriptions().list();
        for parg in self.opt.arg_v.iter() {
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
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _subscriptions_start_paid_service(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.subscriptions().start_paid_service(&self.opt.arg_customer_id, &self.opt.arg_subscription_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _subscriptions_suspend(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.subscriptions().suspend(&self.opt.arg_customer_id, &self.opt.arg_subscription_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    let map = [
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
                        ("user-ip", "userIp"),
                    ];
                    call = call.param(map.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"))
                },
                _ => err.issues.push(CLIError::UnknownParameter(key.to_string())),
            }
        }
        let protocol = "standard-request";
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _doit(&self, dry_run: bool) -> (Option<api::Error>, Option<InvalidOptionsError>) {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Option<api::Error>;
        let mut err_opt: Option<InvalidOptionsError> = None;

        if self.opt.cmd_customers {
            if self.opt.cmd_get {
                call_result = self._customers_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._customers_insert(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._customers_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._customers_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_subscriptions {
            if self.opt.cmd_activate {
                call_result = self._subscriptions_activate(dry_run, &mut err);
            } else if self.opt.cmd_change_plan {
                call_result = self._subscriptions_change_plan(dry_run, &mut err);
            } else if self.opt.cmd_change_renewal_settings {
                call_result = self._subscriptions_change_renewal_settings(dry_run, &mut err);
            } else if self.opt.cmd_change_seats {
                call_result = self._subscriptions_change_seats(dry_run, &mut err);
            } else if self.opt.cmd_delete {
                call_result = self._subscriptions_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._subscriptions_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._subscriptions_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._subscriptions_list(dry_run, &mut err);
            } else if self.opt.cmd_start_paid_service {
                call_result = self._subscriptions_start_paid_service(dry_run, &mut err);
            } else if self.opt.cmd_suspend {
                call_result = self._subscriptions_suspend(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else {
            unreachable!();
        }

        if dry_run {
            if err.issues.len() > 0 {
                err_opt = Some(err);
            }
        }
        (call_result, err_opt)
    }

    // Please note that this call will fail if any part of the opt can't be handled
    fn new(opt: Options) -> Result<Engine, InvalidOptionsError> {
        let (config_dir, secret) = {
            let config_dir = match cmn::assure_config_dir_exists(&opt.flag_config_dir) {
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
                                        if opt.flag_debug_auth {
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
            if opt.flag_debug {
                hyper::Client::with_connector(mock::TeeConnector {
                        connector: hyper::net::HttpConnector(None) 
                    })
            } else {
                hyper::Client::new()
            };
        let engine = Engine {
            opt: opt,
            hub: api::Reseller::new(client, auth),
        };

        match engine._doit(true) {
            (_, Some(err)) => Err(err),
            _ => Ok(engine),
        }
    }

    // Execute the call with all the bells and whistles, informing the caller only if there was an error.
    // The absense of one indicates success.
    fn doit(&self) -> Option<api::Error> {
        self._doit(false).0
    }
}

fn main() {
    let opts: Options = Options::docopt().decode().unwrap_or_else(|e| e.exit());
    match Engine::new(opts) {
        Err(err) => {
            writeln!(io::stderr(), "{}", err).ok();
            env::set_exit_status(err.exit_code);
        },
        Ok(engine) => {
            if let Some(err) = engine.doit() {
                writeln!(io::stderr(), "{:?}", err).ok();
                writeln!(io::stderr(), "{}", err).ok();
                env::set_exit_status(1);
            }
        }
    }
}