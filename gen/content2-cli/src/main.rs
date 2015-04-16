// DO NOT EDIT !
// This file was generated automatically from 'src/mako/cli/main.rs.mako'
// DO NOT EDIT !
#![feature(plugin, exit_status)]
#![plugin(docopt_macros)]
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

extern crate docopt;
extern crate yup_oauth2 as oauth2;
extern crate rustc_serialize;
extern crate serde;
extern crate hyper;
extern crate mime;
extern crate google_content2 as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  content2 [options] accounts authinfo [-p <v>]... [-o <out>]
  content2 [options] accounts custombatch -r <kv>... [-p <v>]... [-o <out>]
  content2 [options] accounts delete <merchant-id> <account-id> [-p <v>]...
  content2 [options] accounts get <merchant-id> <account-id> [-p <v>]... [-o <out>]
  content2 [options] accounts insert <merchant-id> -r <kv>... [-p <v>]... [-o <out>]
  content2 [options] accounts list <merchant-id> [-p <v>]... [-o <out>]
  content2 [options] accounts patch <merchant-id> <account-id> -r <kv>... [-p <v>]... [-o <out>]
  content2 [options] accounts update <merchant-id> <account-id> -r <kv>... [-p <v>]... [-o <out>]
  content2 [options] accountshipping custombatch -r <kv>... [-p <v>]... [-o <out>]
  content2 [options] accountshipping get <merchant-id> <account-id> [-p <v>]... [-o <out>]
  content2 [options] accountshipping list <merchant-id> [-p <v>]... [-o <out>]
  content2 [options] accountshipping patch <merchant-id> <account-id> -r <kv>... [-p <v>]... [-o <out>]
  content2 [options] accountshipping update <merchant-id> <account-id> -r <kv>... [-p <v>]... [-o <out>]
  content2 [options] accountstatuses custombatch -r <kv>... [-p <v>]... [-o <out>]
  content2 [options] accountstatuses get <merchant-id> <account-id> [-p <v>]... [-o <out>]
  content2 [options] accountstatuses list <merchant-id> [-p <v>]... [-o <out>]
  content2 [options] accounttax custombatch -r <kv>... [-p <v>]... [-o <out>]
  content2 [options] accounttax get <merchant-id> <account-id> [-p <v>]... [-o <out>]
  content2 [options] accounttax list <merchant-id> [-p <v>]... [-o <out>]
  content2 [options] accounttax patch <merchant-id> <account-id> -r <kv>... [-p <v>]... [-o <out>]
  content2 [options] accounttax update <merchant-id> <account-id> -r <kv>... [-p <v>]... [-o <out>]
  content2 [options] datafeeds custombatch -r <kv>... [-p <v>]... [-o <out>]
  content2 [options] datafeeds delete <merchant-id> <datafeed-id> [-p <v>]...
  content2 [options] datafeeds get <merchant-id> <datafeed-id> [-p <v>]... [-o <out>]
  content2 [options] datafeeds insert <merchant-id> -r <kv>... [-p <v>]... [-o <out>]
  content2 [options] datafeeds list <merchant-id> [-p <v>]... [-o <out>]
  content2 [options] datafeeds patch <merchant-id> <datafeed-id> -r <kv>... [-p <v>]... [-o <out>]
  content2 [options] datafeeds update <merchant-id> <datafeed-id> -r <kv>... [-p <v>]... [-o <out>]
  content2 [options] datafeedstatuses custombatch -r <kv>... [-p <v>]... [-o <out>]
  content2 [options] datafeedstatuses get <merchant-id> <datafeed-id> [-p <v>]... [-o <out>]
  content2 [options] datafeedstatuses list <merchant-id> [-p <v>]... [-o <out>]
  content2 [options] inventory custombatch -r <kv>... [-p <v>]... [-o <out>]
  content2 [options] inventory set <merchant-id> <store-code> <product-id> -r <kv>... [-p <v>]... [-o <out>]
  content2 [options] products custombatch -r <kv>... [-p <v>]... [-o <out>]
  content2 [options] products delete <merchant-id> <product-id> [-p <v>]...
  content2 [options] products get <merchant-id> <product-id> [-p <v>]... [-o <out>]
  content2 [options] products insert <merchant-id> -r <kv>... [-p <v>]... [-o <out>]
  content2 [options] products list <merchant-id> [-p <v>]... [-o <out>]
  content2 [options] productstatuses custombatch -r <kv>... [-p <v>]... [-o <out>]
  content2 [options] productstatuses get <merchant-id> <product-id> [-p <v>]... [-o <out>]
  content2 [options] productstatuses list <merchant-id> [-p <v>]... [-o <out>]
  content2 --help

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
    hub: api::ShoppingContent<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _accounts_authinfo(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().authinfo();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _accounts_custombatch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::AccountsCustomBatchRequest = Default::default();
        let mut call = self.hub.accounts().custombatch(&request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            match &field_name.to_string()[..] {
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _accounts_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().delete(&self.opt.arg_merchant_id, &self.opt.arg_account_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    None
                }
            }
        }
    }

    fn _accounts_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().get(&self.opt.arg_merchant_id, &self.opt.arg_account_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _accounts_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Account = Default::default();
        let mut call = self.hub.accounts().insert(&request, &self.opt.arg_merchant_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            match &field_name.to_string()[..] {
                "reviews-url" => {
                        request.reviews_url = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "adult-content" => {
                        request.adult_content = Some(arg_from_str(value.unwrap_or("false"), err, "adult-content", "boolean"));
                    },
                "website-url" => {
                        request.website_url = Some(value.unwrap_or("").to_string());
                    },
                "seller-id" => {
                        request.seller_id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _accounts_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().list(&self.opt.arg_merchant_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _accounts_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Account = Default::default();
        let mut call = self.hub.accounts().patch(&request, &self.opt.arg_merchant_id, &self.opt.arg_account_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            match &field_name.to_string()[..] {
                "reviews-url" => {
                        request.reviews_url = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "adult-content" => {
                        request.adult_content = Some(arg_from_str(value.unwrap_or("false"), err, "adult-content", "boolean"));
                    },
                "website-url" => {
                        request.website_url = Some(value.unwrap_or("").to_string());
                    },
                "seller-id" => {
                        request.seller_id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _accounts_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Account = Default::default();
        let mut call = self.hub.accounts().update(&request, &self.opt.arg_merchant_id, &self.opt.arg_account_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            match &field_name.to_string()[..] {
                "reviews-url" => {
                        request.reviews_url = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "adult-content" => {
                        request.adult_content = Some(arg_from_str(value.unwrap_or("false"), err, "adult-content", "boolean"));
                    },
                "website-url" => {
                        request.website_url = Some(value.unwrap_or("").to_string());
                    },
                "seller-id" => {
                        request.seller_id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _accountshipping_custombatch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::AccountshippingCustomBatchRequest = Default::default();
        let mut call = self.hub.accountshipping().custombatch(&request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            match &field_name.to_string()[..] {
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _accountshipping_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accountshipping().get(&self.opt.arg_merchant_id, &self.opt.arg_account_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _accountshipping_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accountshipping().list(&self.opt.arg_merchant_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _accountshipping_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::AccountShipping = Default::default();
        let mut call = self.hub.accountshipping().patch(&request, &self.opt.arg_merchant_id, &self.opt.arg_account_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            match &field_name.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request.account_id = Some(value.unwrap_or("").to_string());
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _accountshipping_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::AccountShipping = Default::default();
        let mut call = self.hub.accountshipping().update(&request, &self.opt.arg_merchant_id, &self.opt.arg_account_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            match &field_name.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request.account_id = Some(value.unwrap_or("").to_string());
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _accountstatuses_custombatch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::AccountstatusesCustomBatchRequest = Default::default();
        let mut call = self.hub.accountstatuses().custombatch(&request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            match &field_name.to_string()[..] {
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _accountstatuses_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accountstatuses().get(&self.opt.arg_merchant_id, &self.opt.arg_account_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _accountstatuses_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accountstatuses().list(&self.opt.arg_merchant_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _accounttax_custombatch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::AccounttaxCustomBatchRequest = Default::default();
        let mut call = self.hub.accounttax().custombatch(&request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            match &field_name.to_string()[..] {
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _accounttax_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounttax().get(&self.opt.arg_merchant_id, &self.opt.arg_account_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _accounttax_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounttax().list(&self.opt.arg_merchant_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _accounttax_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::AccountTax = Default::default();
        let mut call = self.hub.accounttax().patch(&request, &self.opt.arg_merchant_id, &self.opt.arg_account_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            match &field_name.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request.account_id = Some(value.unwrap_or("").to_string());
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _accounttax_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::AccountTax = Default::default();
        let mut call = self.hub.accounttax().update(&request, &self.opt.arg_merchant_id, &self.opt.arg_account_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            match &field_name.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request.account_id = Some(value.unwrap_or("").to_string());
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _datafeeds_custombatch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::DatafeedsCustomBatchRequest = Default::default();
        let mut call = self.hub.datafeeds().custombatch(&request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            match &field_name.to_string()[..] {
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _datafeeds_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.datafeeds().delete(&self.opt.arg_merchant_id, &self.opt.arg_datafeed_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    None
                }
            }
        }
    }

    fn _datafeeds_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.datafeeds().get(&self.opt.arg_merchant_id, &self.opt.arg_datafeed_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _datafeeds_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Datafeed = Default::default();
        let mut call = self.hub.datafeeds().insert(&request, &self.opt.arg_merchant_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_fetch_schedule_init(request: &mut api::Datafeed) {
                if request.fetch_schedule.is_none() {
                    request.fetch_schedule = Some(Default::default());
                }
            }
            
            fn request_format_init(request: &mut api::Datafeed) {
                if request.format.is_none() {
                    request.format = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "content-type" => {
                        request.content_type = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "content-language" => {
                        request.content_language = Some(value.unwrap_or("").to_string());
                    },
                "format.file-encoding" => {
                        request_format_init(&mut request);
                        request.format.as_mut().unwrap().file_encoding = value.unwrap_or("").to_string();
                    },
                "format.quoting-mode" => {
                        request_format_init(&mut request);
                        request.format.as_mut().unwrap().quoting_mode = value.unwrap_or("").to_string();
                    },
                "format.column-delimiter" => {
                        request_format_init(&mut request);
                        request.format.as_mut().unwrap().column_delimiter = value.unwrap_or("").to_string();
                    },
                "fetch-schedule.username" => {
                        request_fetch_schedule_init(&mut request);
                        request.fetch_schedule.as_mut().unwrap().username = value.unwrap_or("").to_string();
                    },
                "fetch-schedule.hour" => {
                        request_fetch_schedule_init(&mut request);
                        request.fetch_schedule.as_mut().unwrap().hour = arg_from_str(value.unwrap_or("-0"), err, "fetch-schedule.hour", "integer");
                    },
                "fetch-schedule.fetch-url" => {
                        request_fetch_schedule_init(&mut request);
                        request.fetch_schedule.as_mut().unwrap().fetch_url = value.unwrap_or("").to_string();
                    },
                "fetch-schedule.weekday" => {
                        request_fetch_schedule_init(&mut request);
                        request.fetch_schedule.as_mut().unwrap().weekday = value.unwrap_or("").to_string();
                    },
                "fetch-schedule.time-zone" => {
                        request_fetch_schedule_init(&mut request);
                        request.fetch_schedule.as_mut().unwrap().time_zone = value.unwrap_or("").to_string();
                    },
                "fetch-schedule.password" => {
                        request_fetch_schedule_init(&mut request);
                        request.fetch_schedule.as_mut().unwrap().password = value.unwrap_or("").to_string();
                    },
                "fetch-schedule.day-of-month" => {
                        request_fetch_schedule_init(&mut request);
                        request.fetch_schedule.as_mut().unwrap().day_of_month = arg_from_str(value.unwrap_or("-0"), err, "fetch-schedule.day-of-month", "integer");
                    },
                "target-country" => {
                        request_fetch_schedule_init(&mut request);
                        request.target_country = Some(arg_from_str(value.unwrap_or("-0"), err, "target-country", "int64"));
                    },
                "file-name" => {
                        request_fetch_schedule_init(&mut request);
                        request.file_name = Some(value.unwrap_or("").to_string());
                    },
                "intended-destinations" => {
                        request_fetch_schedule_init(&mut request);
                        if request.intended_destinations.is_none() {
                            request.intended_destinations = Some(Default::default());
                        }
                        request.intended_destinations.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_fetch_schedule_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "attribute-language" => {
                        request_fetch_schedule_init(&mut request);
                        request.attribute_language = Some(value.unwrap_or("").to_string());
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _datafeeds_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.datafeeds().list(&self.opt.arg_merchant_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _datafeeds_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Datafeed = Default::default();
        let mut call = self.hub.datafeeds().patch(&request, &self.opt.arg_merchant_id, &self.opt.arg_datafeed_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_fetch_schedule_init(request: &mut api::Datafeed) {
                if request.fetch_schedule.is_none() {
                    request.fetch_schedule = Some(Default::default());
                }
            }
            
            fn request_format_init(request: &mut api::Datafeed) {
                if request.format.is_none() {
                    request.format = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "content-type" => {
                        request.content_type = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "content-language" => {
                        request.content_language = Some(value.unwrap_or("").to_string());
                    },
                "format.file-encoding" => {
                        request_format_init(&mut request);
                        request.format.as_mut().unwrap().file_encoding = value.unwrap_or("").to_string();
                    },
                "format.quoting-mode" => {
                        request_format_init(&mut request);
                        request.format.as_mut().unwrap().quoting_mode = value.unwrap_or("").to_string();
                    },
                "format.column-delimiter" => {
                        request_format_init(&mut request);
                        request.format.as_mut().unwrap().column_delimiter = value.unwrap_or("").to_string();
                    },
                "fetch-schedule.username" => {
                        request_fetch_schedule_init(&mut request);
                        request.fetch_schedule.as_mut().unwrap().username = value.unwrap_or("").to_string();
                    },
                "fetch-schedule.hour" => {
                        request_fetch_schedule_init(&mut request);
                        request.fetch_schedule.as_mut().unwrap().hour = arg_from_str(value.unwrap_or("-0"), err, "fetch-schedule.hour", "integer");
                    },
                "fetch-schedule.fetch-url" => {
                        request_fetch_schedule_init(&mut request);
                        request.fetch_schedule.as_mut().unwrap().fetch_url = value.unwrap_or("").to_string();
                    },
                "fetch-schedule.weekday" => {
                        request_fetch_schedule_init(&mut request);
                        request.fetch_schedule.as_mut().unwrap().weekday = value.unwrap_or("").to_string();
                    },
                "fetch-schedule.time-zone" => {
                        request_fetch_schedule_init(&mut request);
                        request.fetch_schedule.as_mut().unwrap().time_zone = value.unwrap_or("").to_string();
                    },
                "fetch-schedule.password" => {
                        request_fetch_schedule_init(&mut request);
                        request.fetch_schedule.as_mut().unwrap().password = value.unwrap_or("").to_string();
                    },
                "fetch-schedule.day-of-month" => {
                        request_fetch_schedule_init(&mut request);
                        request.fetch_schedule.as_mut().unwrap().day_of_month = arg_from_str(value.unwrap_or("-0"), err, "fetch-schedule.day-of-month", "integer");
                    },
                "target-country" => {
                        request_fetch_schedule_init(&mut request);
                        request.target_country = Some(arg_from_str(value.unwrap_or("-0"), err, "target-country", "int64"));
                    },
                "file-name" => {
                        request_fetch_schedule_init(&mut request);
                        request.file_name = Some(value.unwrap_or("").to_string());
                    },
                "intended-destinations" => {
                        request_fetch_schedule_init(&mut request);
                        if request.intended_destinations.is_none() {
                            request.intended_destinations = Some(Default::default());
                        }
                        request.intended_destinations.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_fetch_schedule_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "attribute-language" => {
                        request_fetch_schedule_init(&mut request);
                        request.attribute_language = Some(value.unwrap_or("").to_string());
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _datafeeds_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Datafeed = Default::default();
        let mut call = self.hub.datafeeds().update(&request, &self.opt.arg_merchant_id, &self.opt.arg_datafeed_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_fetch_schedule_init(request: &mut api::Datafeed) {
                if request.fetch_schedule.is_none() {
                    request.fetch_schedule = Some(Default::default());
                }
            }
            
            fn request_format_init(request: &mut api::Datafeed) {
                if request.format.is_none() {
                    request.format = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "content-type" => {
                        request.content_type = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "content-language" => {
                        request.content_language = Some(value.unwrap_or("").to_string());
                    },
                "format.file-encoding" => {
                        request_format_init(&mut request);
                        request.format.as_mut().unwrap().file_encoding = value.unwrap_or("").to_string();
                    },
                "format.quoting-mode" => {
                        request_format_init(&mut request);
                        request.format.as_mut().unwrap().quoting_mode = value.unwrap_or("").to_string();
                    },
                "format.column-delimiter" => {
                        request_format_init(&mut request);
                        request.format.as_mut().unwrap().column_delimiter = value.unwrap_or("").to_string();
                    },
                "fetch-schedule.username" => {
                        request_fetch_schedule_init(&mut request);
                        request.fetch_schedule.as_mut().unwrap().username = value.unwrap_or("").to_string();
                    },
                "fetch-schedule.hour" => {
                        request_fetch_schedule_init(&mut request);
                        request.fetch_schedule.as_mut().unwrap().hour = arg_from_str(value.unwrap_or("-0"), err, "fetch-schedule.hour", "integer");
                    },
                "fetch-schedule.fetch-url" => {
                        request_fetch_schedule_init(&mut request);
                        request.fetch_schedule.as_mut().unwrap().fetch_url = value.unwrap_or("").to_string();
                    },
                "fetch-schedule.weekday" => {
                        request_fetch_schedule_init(&mut request);
                        request.fetch_schedule.as_mut().unwrap().weekday = value.unwrap_or("").to_string();
                    },
                "fetch-schedule.time-zone" => {
                        request_fetch_schedule_init(&mut request);
                        request.fetch_schedule.as_mut().unwrap().time_zone = value.unwrap_or("").to_string();
                    },
                "fetch-schedule.password" => {
                        request_fetch_schedule_init(&mut request);
                        request.fetch_schedule.as_mut().unwrap().password = value.unwrap_or("").to_string();
                    },
                "fetch-schedule.day-of-month" => {
                        request_fetch_schedule_init(&mut request);
                        request.fetch_schedule.as_mut().unwrap().day_of_month = arg_from_str(value.unwrap_or("-0"), err, "fetch-schedule.day-of-month", "integer");
                    },
                "target-country" => {
                        request_fetch_schedule_init(&mut request);
                        request.target_country = Some(arg_from_str(value.unwrap_or("-0"), err, "target-country", "int64"));
                    },
                "file-name" => {
                        request_fetch_schedule_init(&mut request);
                        request.file_name = Some(value.unwrap_or("").to_string());
                    },
                "intended-destinations" => {
                        request_fetch_schedule_init(&mut request);
                        if request.intended_destinations.is_none() {
                            request.intended_destinations = Some(Default::default());
                        }
                        request.intended_destinations.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_fetch_schedule_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "attribute-language" => {
                        request_fetch_schedule_init(&mut request);
                        request.attribute_language = Some(value.unwrap_or("").to_string());
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _datafeedstatuses_custombatch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::DatafeedstatusesCustomBatchRequest = Default::default();
        let mut call = self.hub.datafeedstatuses().custombatch(&request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            match &field_name.to_string()[..] {
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _datafeedstatuses_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.datafeedstatuses().get(&self.opt.arg_merchant_id, &self.opt.arg_datafeed_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _datafeedstatuses_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.datafeedstatuses().list(&self.opt.arg_merchant_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _inventory_custombatch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::InventoryCustomBatchRequest = Default::default();
        let mut call = self.hub.inventory().custombatch(&request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            match &field_name.to_string()[..] {
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _inventory_set(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::InventorySetRequest = Default::default();
        let mut call = self.hub.inventory().set(&request, &self.opt.arg_merchant_id, &self.opt.arg_store_code, &self.opt.arg_product_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_price_init(request: &mut api::InventorySetRequest) {
                if request.price.is_none() {
                    request.price = Some(Default::default());
                }
            }
            
            fn request_sale_price_init(request: &mut api::InventorySetRequest) {
                if request.sale_price.is_none() {
                    request.sale_price = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "sale-price-effective-date" => {
                        request.sale_price_effective_date = Some(value.unwrap_or("").to_string());
                    },
                "price.currency" => {
                        request_price_init(&mut request);
                        request.price.as_mut().unwrap().currency = value.unwrap_or("").to_string();
                    },
                "price.value" => {
                        request_price_init(&mut request);
                        request.price.as_mut().unwrap().value = value.unwrap_or("").to_string();
                    },
                "sale-price.currency" => {
                        request_sale_price_init(&mut request);
                        request.sale_price.as_mut().unwrap().currency = value.unwrap_or("").to_string();
                    },
                "sale-price.value" => {
                        request_sale_price_init(&mut request);
                        request.sale_price.as_mut().unwrap().value = value.unwrap_or("").to_string();
                    },
                "availability" => {
                        request_sale_price_init(&mut request);
                        request.availability = Some(value.unwrap_or("").to_string());
                    },
                "quantity" => {
                        request_sale_price_init(&mut request);
                        request.quantity = Some(arg_from_str(value.unwrap_or("-0"), err, "quantity", "integer"));
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _products_custombatch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::ProductsCustomBatchRequest = Default::default();
        let mut call = self.hub.products().custombatch(&request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "dry-run" => {
                    call = call.dry_run(arg_from_str(value.unwrap_or("false"), err, "dry-run", "boolean"));
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
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            match &field_name.to_string()[..] {
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _products_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.products().delete(&self.opt.arg_merchant_id, &self.opt.arg_product_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "dry-run" => {
                    call = call.dry_run(arg_from_str(value.unwrap_or("false"), err, "dry-run", "boolean"));
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
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok(mut response) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    None
                }
            }
        }
    }

    fn _products_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.products().get(&self.opt.arg_merchant_id, &self.opt.arg_product_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _products_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Product = Default::default();
        let mut call = self.hub.products().insert(&request, &self.opt.arg_merchant_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "dry-run" => {
                    call = call.dry_run(arg_from_str(value.unwrap_or("false"), err, "dry-run", "boolean"));
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
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_installment_init(request: &mut api::Product) {
                if request.installment.is_none() {
                    request.installment = Some(Default::default());
                }
            }
            
            fn request_loyalty_points_init(request: &mut api::Product) {
                if request.loyalty_points.is_none() {
                    request.loyalty_points = Some(Default::default());
                }
            }
            
            fn request_price_init(request: &mut api::Product) {
                if request.price.is_none() {
                    request.price = Some(Default::default());
                }
            }
            
            fn request_sale_price_init(request: &mut api::Product) {
                if request.sale_price.is_none() {
                    request.sale_price = Some(Default::default());
                }
            }
            
            fn request_shipping_height_init(request: &mut api::Product) {
                if request.shipping_height.is_none() {
                    request.shipping_height = Some(Default::default());
                }
            }
            
            fn request_shipping_length_init(request: &mut api::Product) {
                if request.shipping_length.is_none() {
                    request.shipping_length = Some(Default::default());
                }
            }
            
            fn request_shipping_weight_init(request: &mut api::Product) {
                if request.shipping_weight.is_none() {
                    request.shipping_weight = Some(Default::default());
                }
            }
            
            fn request_shipping_width_init(request: &mut api::Product) {
                if request.shipping_width.is_none() {
                    request.shipping_width = Some(Default::default());
                }
            }
            
            fn request_unit_pricing_base_measure_init(request: &mut api::Product) {
                if request.unit_pricing_base_measure.is_none() {
                    request.unit_pricing_base_measure = Some(Default::default());
                }
            }
            
            fn request_unit_pricing_measure_init(request: &mut api::Product) {
                if request.unit_pricing_measure.is_none() {
                    request.unit_pricing_measure = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "display-ads-title" => {
                        request.display_ads_title = Some(value.unwrap_or("").to_string());
                    },
                "color" => {
                        request.color = Some(value.unwrap_or("").to_string());
                    },
                "additional-image-links" => {
                        if request.additional_image_links.is_none() {
                            request.additional_image_links = Some(Default::default());
                        }
                        request.additional_image_links.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "adwords-labels" => {
                        if request.adwords_labels.is_none() {
                            request.adwords_labels = Some(Default::default());
                        }
                        request.adwords_labels.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "item-group-id" => {
                        request.item_group_id = Some(value.unwrap_or("").to_string());
                    },
                "gtin" => {
                        request.gtin = Some(value.unwrap_or("").to_string());
                    },
                "expiration-date" => {
                        request.expiration_date = Some(value.unwrap_or("").to_string());
                    },
                "google-product-category" => {
                        request.google_product_category = Some(value.unwrap_or("").to_string());
                    },
                "multipack" => {
                        request.multipack = Some(value.unwrap_or("").to_string());
                    },
                "display-ads-id" => {
                        request.display_ads_id = Some(value.unwrap_or("").to_string());
                    },
                "display-ads-value" => {
                        request.display_ads_value = Some(arg_from_str(value.unwrap_or("0.0"), err, "display-ads-value", "number"));
                    },
                "availability" => {
                        request.availability = Some(value.unwrap_or("").to_string());
                    },
                "adwords-grouping" => {
                        request.adwords_grouping = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "target-country" => {
                        request.target_country = Some(arg_from_str(value.unwrap_or("-0"), err, "target-country", "int64"));
                    },
                "size-type" => {
                        request.size_type = Some(value.unwrap_or("").to_string());
                    },
                "offer-id" => {
                        request.offer_id = Some(value.unwrap_or("").to_string());
                    },
                "title" => {
                        request.title = Some(value.unwrap_or("").to_string());
                    },
                "pattern" => {
                        request.pattern = Some(value.unwrap_or("").to_string());
                    },
                "unit-pricing-measure.value" => {
                        request_unit_pricing_measure_init(&mut request);
                        request.unit_pricing_measure.as_mut().unwrap().value = arg_from_str(value.unwrap_or("0.0"), err, "unit-pricing-measure.value", "number");
                    },
                "unit-pricing-measure.unit" => {
                        request_unit_pricing_measure_init(&mut request);
                        request.unit_pricing_measure.as_mut().unwrap().unit = value.unwrap_or("").to_string();
                    },
                "validated-destinations" => {
                        request_unit_pricing_measure_init(&mut request);
                        if request.validated_destinations.is_none() {
                            request.validated_destinations = Some(Default::default());
                        }
                        request.validated_destinations.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_unit_pricing_measure_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "online-only" => {
                        request_unit_pricing_measure_init(&mut request);
                        request.online_only = Some(arg_from_str(value.unwrap_or("false"), err, "online-only", "boolean"));
                    },
                "is-bundle" => {
                        request_unit_pricing_measure_init(&mut request);
                        request.is_bundle = Some(arg_from_str(value.unwrap_or("false"), err, "is-bundle", "boolean"));
                    },
                "mobile-link" => {
                        request_unit_pricing_measure_init(&mut request);
                        request.mobile_link = Some(value.unwrap_or("").to_string());
                    },
                "price.currency" => {
                        request_price_init(&mut request);
                        request.price.as_mut().unwrap().currency = value.unwrap_or("").to_string();
                    },
                "price.value" => {
                        request_price_init(&mut request);
                        request.price.as_mut().unwrap().value = value.unwrap_or("").to_string();
                    },
                "channel" => {
                        request_price_init(&mut request);
                        request.channel = Some(value.unwrap_or("").to_string());
                    },
                "loyalty-points.ratio" => {
                        request_loyalty_points_init(&mut request);
                        request.loyalty_points.as_mut().unwrap().ratio = arg_from_str(value.unwrap_or("0.0"), err, "loyalty-points.ratio", "number");
                    },
                "loyalty-points.name" => {
                        request_loyalty_points_init(&mut request);
                        request.loyalty_points.as_mut().unwrap().name = value.unwrap_or("").to_string();
                    },
                "loyalty-points.points-value" => {
                        request_loyalty_points_init(&mut request);
                        request.loyalty_points.as_mut().unwrap().points_value = value.unwrap_or("").to_string();
                    },
                "shipping-height.value" => {
                        request_shipping_height_init(&mut request);
                        request.shipping_height.as_mut().unwrap().value = arg_from_str(value.unwrap_or("0.0"), err, "shipping-height.value", "number");
                    },
                "shipping-height.unit" => {
                        request_shipping_height_init(&mut request);
                        request.shipping_height.as_mut().unwrap().unit = value.unwrap_or("").to_string();
                    },
                "content-language" => {
                        request_shipping_height_init(&mut request);
                        request.content_language = Some(value.unwrap_or("").to_string());
                    },
                "mpn" => {
                        request_shipping_height_init(&mut request);
                        request.mpn = Some(value.unwrap_or("").to_string());
                    },
                "sale-price-effective-date" => {
                        request_shipping_height_init(&mut request);
                        request.sale_price_effective_date = Some(value.unwrap_or("").to_string());
                    },
                "brand" => {
                        request_shipping_height_init(&mut request);
                        request.brand = Some(value.unwrap_or("").to_string());
                    },
                "material" => {
                        request_shipping_height_init(&mut request);
                        request.material = Some(value.unwrap_or("").to_string());
                    },
                "availability-date" => {
                        request_shipping_height_init(&mut request);
                        request.availability_date = Some(value.unwrap_or("").to_string());
                    },
                "shipping-length.value" => {
                        request_shipping_length_init(&mut request);
                        request.shipping_length.as_mut().unwrap().value = arg_from_str(value.unwrap_or("0.0"), err, "shipping-length.value", "number");
                    },
                "shipping-length.unit" => {
                        request_shipping_length_init(&mut request);
                        request.shipping_length.as_mut().unwrap().unit = value.unwrap_or("").to_string();
                    },
                "link" => {
                        request_shipping_length_init(&mut request);
                        request.link = Some(value.unwrap_or("").to_string());
                    },
                "adult" => {
                        request_shipping_length_init(&mut request);
                        request.adult = Some(arg_from_str(value.unwrap_or("false"), err, "adult", "boolean"));
                    },
                "display-ads-link" => {
                        request_shipping_length_init(&mut request);
                        request.display_ads_link = Some(value.unwrap_or("").to_string());
                    },
                "energy-efficiency-class" => {
                        request_shipping_length_init(&mut request);
                        request.energy_efficiency_class = Some(value.unwrap_or("").to_string());
                    },
                "size-system" => {
                        request_shipping_length_init(&mut request);
                        request.size_system = Some(value.unwrap_or("").to_string());
                    },
                "custom-label4" => {
                        request_shipping_length_init(&mut request);
                        request.custom_label4 = Some(value.unwrap_or("").to_string());
                    },
                "custom-label3" => {
                        request_shipping_length_init(&mut request);
                        request.custom_label3 = Some(value.unwrap_or("").to_string());
                    },
                "custom-label2" => {
                        request_shipping_length_init(&mut request);
                        request.custom_label2 = Some(value.unwrap_or("").to_string());
                    },
                "condition" => {
                        request_shipping_length_init(&mut request);
                        request.condition = Some(value.unwrap_or("").to_string());
                    },
                "custom-label0" => {
                        request_shipping_length_init(&mut request);
                        request.custom_label0 = Some(value.unwrap_or("").to_string());
                    },
                "shipping-label" => {
                        request_shipping_length_init(&mut request);
                        request.shipping_label = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_shipping_length_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "unit-pricing-base-measure.value" => {
                        request_unit_pricing_base_measure_init(&mut request);
                        request.unit_pricing_base_measure.as_mut().unwrap().value = value.unwrap_or("").to_string();
                    },
                "unit-pricing-base-measure.unit" => {
                        request_unit_pricing_base_measure_init(&mut request);
                        request.unit_pricing_base_measure.as_mut().unwrap().unit = value.unwrap_or("").to_string();
                    },
                "installment.amount.currency" => {
                        request_installment_init(&mut request);
                        request.installment.as_mut().unwrap().amount.currency = value.unwrap_or("").to_string();
                    },
                "installment.amount.value" => {
                        request_installment_init(&mut request);
                        request.installment.as_mut().unwrap().amount.value = value.unwrap_or("").to_string();
                    },
                "installment.months" => {
                        request_installment_init(&mut request);
                        request.installment.as_mut().unwrap().months = value.unwrap_or("").to_string();
                    },
                "sizes" => {
                        request_installment_init(&mut request);
                        if request.sizes.is_none() {
                            request.sizes = Some(Default::default());
                        }
                        request.sizes.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "gender" => {
                        request_installment_init(&mut request);
                        request.gender = Some(value.unwrap_or("").to_string());
                    },
                "shipping-width.value" => {
                        request_shipping_width_init(&mut request);
                        request.shipping_width.as_mut().unwrap().value = arg_from_str(value.unwrap_or("0.0"), err, "shipping-width.value", "number");
                    },
                "shipping-width.unit" => {
                        request_shipping_width_init(&mut request);
                        request.shipping_width.as_mut().unwrap().unit = value.unwrap_or("").to_string();
                    },
                "shipping-weight.value" => {
                        request_shipping_weight_init(&mut request);
                        request.shipping_weight.as_mut().unwrap().value = arg_from_str(value.unwrap_or("0.0"), err, "shipping-weight.value", "number");
                    },
                "shipping-weight.unit" => {
                        request_shipping_weight_init(&mut request);
                        request.shipping_weight.as_mut().unwrap().unit = value.unwrap_or("").to_string();
                    },
                "identifier-exists" => {
                        request_shipping_weight_init(&mut request);
                        request.identifier_exists = Some(arg_from_str(value.unwrap_or("false"), err, "identifier-exists", "boolean"));
                    },
                "image-link" => {
                        request_shipping_weight_init(&mut request);
                        request.image_link = Some(value.unwrap_or("").to_string());
                    },
                "sale-price.currency" => {
                        request_sale_price_init(&mut request);
                        request.sale_price.as_mut().unwrap().currency = value.unwrap_or("").to_string();
                    },
                "sale-price.value" => {
                        request_sale_price_init(&mut request);
                        request.sale_price.as_mut().unwrap().value = value.unwrap_or("").to_string();
                    },
                "product-type" => {
                        request_sale_price_init(&mut request);
                        request.product_type = Some(value.unwrap_or("").to_string());
                    },
                "display-ads-similar-ids" => {
                        request_sale_price_init(&mut request);
                        if request.display_ads_similar_ids.is_none() {
                            request.display_ads_similar_ids = Some(Default::default());
                        }
                        request.display_ads_similar_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "custom-label1" => {
                        request_sale_price_init(&mut request);
                        request.custom_label1 = Some(value.unwrap_or("").to_string());
                    },
                "age-group" => {
                        request_sale_price_init(&mut request);
                        request.age_group = Some(value.unwrap_or("").to_string());
                    },
                "adwords-redirect" => {
                        request_sale_price_init(&mut request);
                        request.adwords_redirect = Some(value.unwrap_or("").to_string());
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _products_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.products().list(&self.opt.arg_merchant_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _productstatuses_custombatch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::ProductstatusesCustomBatchRequest = Default::default();
        let mut call = self.hub.productstatuses().custombatch(&request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            match &field_name.to_string()[..] {
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _productstatuses_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.productstatuses().get(&self.opt.arg_merchant_id, &self.opt.arg_product_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _productstatuses_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.productstatuses().list(&self.opt.arg_merchant_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _doit(&self, dry_run: bool) -> (Option<api::Error>, Option<InvalidOptionsError>) {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Option<api::Error>;
        let mut err_opt: Option<InvalidOptionsError> = None;

        if self.opt.cmd_accounts {
            if self.opt.cmd_authinfo {
                call_result = self._accounts_authinfo(dry_run, &mut err);
            } else if self.opt.cmd_custombatch {
                call_result = self._accounts_custombatch(dry_run, &mut err);
            } else if self.opt.cmd_delete {
                call_result = self._accounts_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._accounts_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._accounts_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._accounts_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._accounts_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._accounts_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_accountshipping {
            if self.opt.cmd_custombatch {
                call_result = self._accountshipping_custombatch(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._accountshipping_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._accountshipping_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._accountshipping_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._accountshipping_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_accountstatuses {
            if self.opt.cmd_custombatch {
                call_result = self._accountstatuses_custombatch(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._accountstatuses_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._accountstatuses_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_accounttax {
            if self.opt.cmd_custombatch {
                call_result = self._accounttax_custombatch(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._accounttax_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._accounttax_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._accounttax_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._accounttax_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_datafeeds {
            if self.opt.cmd_custombatch {
                call_result = self._datafeeds_custombatch(dry_run, &mut err);
            } else if self.opt.cmd_delete {
                call_result = self._datafeeds_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._datafeeds_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._datafeeds_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._datafeeds_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._datafeeds_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._datafeeds_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_datafeedstatuses {
            if self.opt.cmd_custombatch {
                call_result = self._datafeedstatuses_custombatch(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._datafeedstatuses_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._datafeedstatuses_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_inventory {
            if self.opt.cmd_custombatch {
                call_result = self._inventory_custombatch(dry_run, &mut err);
            } else if self.opt.cmd_set {
                call_result = self._inventory_set(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_products {
            if self.opt.cmd_custombatch {
                call_result = self._products_custombatch(dry_run, &mut err);
            } else if self.opt.cmd_delete {
                call_result = self._products_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._products_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._products_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._products_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_productstatuses {
            if self.opt.cmd_custombatch {
                call_result = self._productstatuses_custombatch(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._productstatuses_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._productstatuses_list(dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "content2-secret.json") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                                      hyper::Client::new(),
                                      JsonTokenStorage {
                                        program_name: "content2",
                                        db_dir: config_dir.clone(),
                                      }, None);
        let engine = Engine {
            opt: opt,
            hub: api::ShoppingContent::new(hyper::Client::new(), auth),
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
            write!(io::stderr(), "{}", err).ok();
            env::set_exit_status(err.exit_code);
        },
        Ok(engine) => {
            if let Some(err) = engine.doit() {
                write!(io::stderr(), "{}", err).ok();
                env::set_exit_status(1);
            }
        }
    }
}