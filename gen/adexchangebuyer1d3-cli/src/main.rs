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
extern crate google_adexchangebuyer1d3 as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  adexchangebuyer1d3 [options] accounts get <id> [-p <v>]... [-o <out>]
  adexchangebuyer1d3 [options] accounts list [-p <v>]... [-o <out>]
  adexchangebuyer1d3 [options] accounts patch <id> -r <kv>... [-p <v>]... [-o <out>]
  adexchangebuyer1d3 [options] accounts update <id> -r <kv>... [-p <v>]... [-o <out>]
  adexchangebuyer1d3 [options] billing-info get <account-id> [-p <v>]... [-o <out>]
  adexchangebuyer1d3 [options] billing-info list [-p <v>]... [-o <out>]
  adexchangebuyer1d3 [options] budget get <account-id> <billing-id> [-p <v>]... [-o <out>]
  adexchangebuyer1d3 [options] budget patch <account-id> <billing-id> -r <kv>... [-p <v>]... [-o <out>]
  adexchangebuyer1d3 [options] budget update <account-id> <billing-id> -r <kv>... [-p <v>]... [-o <out>]
  adexchangebuyer1d3 [options] creatives get <account-id> <buyer-creative-id> [-p <v>]... [-o <out>]
  adexchangebuyer1d3 [options] creatives insert -r <kv>... [-p <v>]... [-o <out>]
  adexchangebuyer1d3 [options] creatives list [-p <v>]... [-o <out>]
  adexchangebuyer1d3 [options] direct-deals get <id> [-p <v>]... [-o <out>]
  adexchangebuyer1d3 [options] direct-deals list [-p <v>]... [-o <out>]
  adexchangebuyer1d3 [options] performance-report list <account-id> <end-date-time> <start-date-time> [-p <v>]... [-o <out>]
  adexchangebuyer1d3 [options] pretargeting-config delete <account-id> <config-id> [-p <v>]...
  adexchangebuyer1d3 [options] pretargeting-config get <account-id> <config-id> [-p <v>]... [-o <out>]
  adexchangebuyer1d3 [options] pretargeting-config insert <account-id> -r <kv>... [-p <v>]... [-o <out>]
  adexchangebuyer1d3 [options] pretargeting-config list <account-id> [-p <v>]... [-o <out>]
  adexchangebuyer1d3 [options] pretargeting-config patch <account-id> <config-id> -r <kv>... [-p <v>]... [-o <out>]
  adexchangebuyer1d3 [options] pretargeting-config update <account-id> <config-id> -r <kv>... [-p <v>]... [-o <out>]
  adexchangebuyer1d3 --help

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
    hub: api::AdExchangeBuyer<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _accounts_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let id: i32 = arg_from_str(&self.opt.arg_id, err, "<id>", "integer");
        let mut call = self.hub.accounts().get(id);
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

    fn _accounts_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().list();
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

    fn _accounts_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Account::default();
        let id: i32 = arg_from_str(&self.opt.arg_id, err, "<id>", "integer");
        let mut call = self.hub.accounts().patch(&request, id);
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
                "maximum-total-qps" => {
                        request.maximum_total_qps = Some(arg_from_str(value.unwrap_or("-0"), err, "maximum-total-qps", "integer"));
                    },
                "maximum-active-creatives" => {
                        request.maximum_active_creatives = Some(arg_from_str(value.unwrap_or("-0"), err, "maximum-active-creatives", "integer"));
                    },
                "cookie-matching-nid" => {
                        request.cookie_matching_nid = Some(value.unwrap_or("").to_string());
                    },
                "number-active-creatives" => {
                        request.number_active_creatives = Some(arg_from_str(value.unwrap_or("-0"), err, "number-active-creatives", "integer"));
                    },
                "id" => {
                        request.id = Some(arg_from_str(value.unwrap_or("-0"), err, "id", "integer"));
                    },
                "cookie-matching-url" => {
                        request.cookie_matching_url = Some(value.unwrap_or("").to_string());
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

    fn _accounts_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Account::default();
        let id: i32 = arg_from_str(&self.opt.arg_id, err, "<id>", "integer");
        let mut call = self.hub.accounts().update(&request, id);
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
                "maximum-total-qps" => {
                        request.maximum_total_qps = Some(arg_from_str(value.unwrap_or("-0"), err, "maximum-total-qps", "integer"));
                    },
                "maximum-active-creatives" => {
                        request.maximum_active_creatives = Some(arg_from_str(value.unwrap_or("-0"), err, "maximum-active-creatives", "integer"));
                    },
                "cookie-matching-nid" => {
                        request.cookie_matching_nid = Some(value.unwrap_or("").to_string());
                    },
                "number-active-creatives" => {
                        request.number_active_creatives = Some(arg_from_str(value.unwrap_or("-0"), err, "number-active-creatives", "integer"));
                    },
                "id" => {
                        request.id = Some(arg_from_str(value.unwrap_or("-0"), err, "id", "integer"));
                    },
                "cookie-matching-url" => {
                        request.cookie_matching_url = Some(value.unwrap_or("").to_string());
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

    fn _billing_info_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let account_id: i32 = arg_from_str(&self.opt.arg_account_id, err, "<account-id>", "integer");
        let mut call = self.hub.billing_info().get(account_id);
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

    fn _billing_info_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.billing_info().list();
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

    fn _budget_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.budget().get(&self.opt.arg_account_id, &self.opt.arg_billing_id);
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

    fn _budget_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Budget::default();
        let mut call = self.hub.budget().patch(&request, &self.opt.arg_account_id, &self.opt.arg_billing_id);
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
                "budget-amount" => {
                        request.budget_amount = Some(value.unwrap_or("").to_string());
                    },
                "currency-code" => {
                        request.currency_code = Some(value.unwrap_or("").to_string());
                    },
                "billing-id" => {
                        request.billing_id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _budget_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Budget::default();
        let mut call = self.hub.budget().update(&request, &self.opt.arg_account_id, &self.opt.arg_billing_id);
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
                "budget-amount" => {
                        request.budget_amount = Some(value.unwrap_or("").to_string());
                    },
                "currency-code" => {
                        request.currency_code = Some(value.unwrap_or("").to_string());
                    },
                "billing-id" => {
                        request.billing_id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _creatives_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let account_id: i32 = arg_from_str(&self.opt.arg_account_id, err, "<account-id>", "integer");
        let mut call = self.hub.creatives().get(account_id, &self.opt.arg_buyer_creative_id);
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

    fn _creatives_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Creative::default();
        let mut call = self.hub.creatives().insert(&request);
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
            fn request_filtering_reasons_init(request: &mut api::Creative) {
                if request.filtering_reasons.is_none() {
                    request.filtering_reasons = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "product-categories" => {
                        if request.product_categories.is_none() {
                           request.product_categories = Some(Default::default());
                        }
                                        request.product_categories.as_mut().unwrap().push(arg_from_str(value.unwrap_or("-0"), err, "product-categories", "integer"));
                    },
                "status" => {
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "video-url" => {
                        request.video_url = Some(value.unwrap_or("").to_string());
                    },
                "agency-id" => {
                        request.agency_id = Some(value.unwrap_or("").to_string());
                    },
                "width" => {
                        request.width = Some(arg_from_str(value.unwrap_or("-0"), err, "width", "integer"));
                    },
                "attribute" => {
                        if request.attribute.is_none() {
                           request.attribute = Some(Default::default());
                        }
                                        request.attribute.as_mut().unwrap().push(arg_from_str(value.unwrap_or("-0"), err, "attribute", "integer"));
                    },
                "restricted-categories" => {
                        if request.restricted_categories.is_none() {
                           request.restricted_categories = Some(Default::default());
                        }
                                        request.restricted_categories.as_mut().unwrap().push(arg_from_str(value.unwrap_or("-0"), err, "restricted-categories", "integer"));
                    },
                "height" => {
                        request.height = Some(arg_from_str(value.unwrap_or("-0"), err, "height", "integer"));
                    },
                "advertiser-name" => {
                        request.advertiser_name = Some(value.unwrap_or("").to_string());
                    },
                "html-snippet" => {
                        request.html_snippet = Some(value.unwrap_or("").to_string());
                    },
                "advertiser-id" => {
                        if request.advertiser_id.is_none() {
                           request.advertiser_id = Some(Default::default());
                        }
                                        request.advertiser_id.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "buyer-creative-id" => {
                        request.buyer_creative_id = Some(value.unwrap_or("").to_string());
                    },
                "click-through-url" => {
                        if request.click_through_url.is_none() {
                           request.click_through_url = Some(Default::default());
                        }
                                        request.click_through_url.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "vendor-type" => {
                        if request.vendor_type.is_none() {
                           request.vendor_type = Some(Default::default());
                        }
                                        request.vendor_type.as_mut().unwrap().push(arg_from_str(value.unwrap_or("-0"), err, "vendor-type", "integer"));
                    },
                "filtering-reasons.date" => {
                        request_filtering_reasons_init(&mut request);
                        request.filtering_reasons.as_mut().unwrap().date = Some(value.unwrap_or("").to_string());
                    },
                "sensitive-categories" => {
                        request_filtering_reasons_init(&mut request);
                        if request.sensitive_categories.is_none() {
                           request.sensitive_categories = Some(Default::default());
                        }
                                        request.sensitive_categories.as_mut().unwrap().push(arg_from_str(value.unwrap_or("-0"), err, "sensitive-categories", "integer"));
                    },
                "account-id" => {
                        request_filtering_reasons_init(&mut request);
                        request.account_id = Some(arg_from_str(value.unwrap_or("-0"), err, "account-id", "integer"));
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

    fn _creatives_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.creatives().list();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "status-filter" => {
                    call = call.status_filter(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "buyer-creative-id" => {
                    call = call.add_buyer_creative_id(value.unwrap_or(""));
                },
                "account-id" => {
                    call = call.add_account_id(arg_from_str(value.unwrap_or("-0"), err, "account-id", "integer"));
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

    fn _direct_deals_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.direct_deals().get(&self.opt.arg_id);
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

    fn _direct_deals_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.direct_deals().list();
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

    fn _performance_report_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.performance_report().list(&self.opt.arg_account_id, &self.opt.arg_end_date_time, &self.opt.arg_start_date_time);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _pretargeting_config_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.pretargeting_config().delete(&self.opt.arg_account_id, &self.opt.arg_config_id);
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

    fn _pretargeting_config_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.pretargeting_config().get(&self.opt.arg_account_id, &self.opt.arg_config_id);
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

    fn _pretargeting_config_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::PretargetingConfig::default();
        let mut call = self.hub.pretargeting_config().insert(&request, &self.opt.arg_account_id);
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
                "billing-id" => {
                        request.billing_id = Some(value.unwrap_or("").to_string());
                    },
                "languages" => {
                        if request.languages.is_none() {
                           request.languages = Some(Default::default());
                        }
                                        request.languages.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "config-name" => {
                        request.config_name = Some(value.unwrap_or("").to_string());
                    },
                "excluded-geo-criteria-ids" => {
                        if request.excluded_geo_criteria_ids.is_none() {
                           request.excluded_geo_criteria_ids = Some(Default::default());
                        }
                                        request.excluded_geo_criteria_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "user-lists" => {
                        if request.user_lists.is_none() {
                           request.user_lists = Some(Default::default());
                        }
                                        request.user_lists.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "excluded-verticals" => {
                        if request.excluded_verticals.is_none() {
                           request.excluded_verticals = Some(Default::default());
                        }
                                        request.excluded_verticals.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "vendor-types" => {
                        if request.vendor_types.is_none() {
                           request.vendor_types = Some(Default::default());
                        }
                                        request.vendor_types.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "excluded-content-labels" => {
                        if request.excluded_content_labels.is_none() {
                           request.excluded_content_labels = Some(Default::default());
                        }
                                        request.excluded_content_labels.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "verticals" => {
                        if request.verticals.is_none() {
                           request.verticals = Some(Default::default());
                        }
                                        request.verticals.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "platforms" => {
                        if request.platforms.is_none() {
                           request.platforms = Some(Default::default());
                        }
                                        request.platforms.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "mobile-devices" => {
                        if request.mobile_devices.is_none() {
                           request.mobile_devices = Some(Default::default());
                        }
                                        request.mobile_devices.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "creative-type" => {
                        if request.creative_type.is_none() {
                           request.creative_type = Some(Default::default());
                        }
                                        request.creative_type.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "geo-criteria-ids" => {
                        if request.geo_criteria_ids.is_none() {
                           request.geo_criteria_ids = Some(Default::default());
                        }
                                        request.geo_criteria_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "mobile-operating-system-versions" => {
                        if request.mobile_operating_system_versions.is_none() {
                           request.mobile_operating_system_versions = Some(Default::default());
                        }
                                        request.mobile_operating_system_versions.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "mobile-carriers" => {
                        if request.mobile_carriers.is_none() {
                           request.mobile_carriers = Some(Default::default());
                        }
                                        request.mobile_carriers.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "config-id" => {
                        request.config_id = Some(value.unwrap_or("").to_string());
                    },
                "excluded-user-lists" => {
                        if request.excluded_user_lists.is_none() {
                           request.excluded_user_lists = Some(Default::default());
                        }
                                        request.excluded_user_lists.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "is-active" => {
                        request.is_active = Some(arg_from_str(value.unwrap_or("false"), err, "is-active", "boolean"));
                    },
                "supported-creative-attributes" => {
                        if request.supported_creative_attributes.is_none() {
                           request.supported_creative_attributes = Some(Default::default());
                        }
                                        request.supported_creative_attributes.as_mut().unwrap().push(value.unwrap_or("").to_string());
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

    fn _pretargeting_config_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.pretargeting_config().list(&self.opt.arg_account_id);
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

    fn _pretargeting_config_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::PretargetingConfig::default();
        let mut call = self.hub.pretargeting_config().patch(&request, &self.opt.arg_account_id, &self.opt.arg_config_id);
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
                "billing-id" => {
                        request.billing_id = Some(value.unwrap_or("").to_string());
                    },
                "languages" => {
                        if request.languages.is_none() {
                           request.languages = Some(Default::default());
                        }
                                        request.languages.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "config-name" => {
                        request.config_name = Some(value.unwrap_or("").to_string());
                    },
                "excluded-geo-criteria-ids" => {
                        if request.excluded_geo_criteria_ids.is_none() {
                           request.excluded_geo_criteria_ids = Some(Default::default());
                        }
                                        request.excluded_geo_criteria_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "user-lists" => {
                        if request.user_lists.is_none() {
                           request.user_lists = Some(Default::default());
                        }
                                        request.user_lists.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "excluded-verticals" => {
                        if request.excluded_verticals.is_none() {
                           request.excluded_verticals = Some(Default::default());
                        }
                                        request.excluded_verticals.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "vendor-types" => {
                        if request.vendor_types.is_none() {
                           request.vendor_types = Some(Default::default());
                        }
                                        request.vendor_types.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "excluded-content-labels" => {
                        if request.excluded_content_labels.is_none() {
                           request.excluded_content_labels = Some(Default::default());
                        }
                                        request.excluded_content_labels.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "verticals" => {
                        if request.verticals.is_none() {
                           request.verticals = Some(Default::default());
                        }
                                        request.verticals.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "platforms" => {
                        if request.platforms.is_none() {
                           request.platforms = Some(Default::default());
                        }
                                        request.platforms.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "mobile-devices" => {
                        if request.mobile_devices.is_none() {
                           request.mobile_devices = Some(Default::default());
                        }
                                        request.mobile_devices.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "creative-type" => {
                        if request.creative_type.is_none() {
                           request.creative_type = Some(Default::default());
                        }
                                        request.creative_type.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "geo-criteria-ids" => {
                        if request.geo_criteria_ids.is_none() {
                           request.geo_criteria_ids = Some(Default::default());
                        }
                                        request.geo_criteria_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "mobile-operating-system-versions" => {
                        if request.mobile_operating_system_versions.is_none() {
                           request.mobile_operating_system_versions = Some(Default::default());
                        }
                                        request.mobile_operating_system_versions.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "mobile-carriers" => {
                        if request.mobile_carriers.is_none() {
                           request.mobile_carriers = Some(Default::default());
                        }
                                        request.mobile_carriers.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "config-id" => {
                        request.config_id = Some(value.unwrap_or("").to_string());
                    },
                "excluded-user-lists" => {
                        if request.excluded_user_lists.is_none() {
                           request.excluded_user_lists = Some(Default::default());
                        }
                                        request.excluded_user_lists.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "is-active" => {
                        request.is_active = Some(arg_from_str(value.unwrap_or("false"), err, "is-active", "boolean"));
                    },
                "supported-creative-attributes" => {
                        if request.supported_creative_attributes.is_none() {
                           request.supported_creative_attributes = Some(Default::default());
                        }
                                        request.supported_creative_attributes.as_mut().unwrap().push(value.unwrap_or("").to_string());
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

    fn _pretargeting_config_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::PretargetingConfig::default();
        let mut call = self.hub.pretargeting_config().update(&request, &self.opt.arg_account_id, &self.opt.arg_config_id);
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
                "billing-id" => {
                        request.billing_id = Some(value.unwrap_or("").to_string());
                    },
                "languages" => {
                        if request.languages.is_none() {
                           request.languages = Some(Default::default());
                        }
                                        request.languages.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "config-name" => {
                        request.config_name = Some(value.unwrap_or("").to_string());
                    },
                "excluded-geo-criteria-ids" => {
                        if request.excluded_geo_criteria_ids.is_none() {
                           request.excluded_geo_criteria_ids = Some(Default::default());
                        }
                                        request.excluded_geo_criteria_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "user-lists" => {
                        if request.user_lists.is_none() {
                           request.user_lists = Some(Default::default());
                        }
                                        request.user_lists.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "excluded-verticals" => {
                        if request.excluded_verticals.is_none() {
                           request.excluded_verticals = Some(Default::default());
                        }
                                        request.excluded_verticals.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "vendor-types" => {
                        if request.vendor_types.is_none() {
                           request.vendor_types = Some(Default::default());
                        }
                                        request.vendor_types.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "excluded-content-labels" => {
                        if request.excluded_content_labels.is_none() {
                           request.excluded_content_labels = Some(Default::default());
                        }
                                        request.excluded_content_labels.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "verticals" => {
                        if request.verticals.is_none() {
                           request.verticals = Some(Default::default());
                        }
                                        request.verticals.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "platforms" => {
                        if request.platforms.is_none() {
                           request.platforms = Some(Default::default());
                        }
                                        request.platforms.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "mobile-devices" => {
                        if request.mobile_devices.is_none() {
                           request.mobile_devices = Some(Default::default());
                        }
                                        request.mobile_devices.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "creative-type" => {
                        if request.creative_type.is_none() {
                           request.creative_type = Some(Default::default());
                        }
                                        request.creative_type.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "geo-criteria-ids" => {
                        if request.geo_criteria_ids.is_none() {
                           request.geo_criteria_ids = Some(Default::default());
                        }
                                        request.geo_criteria_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "mobile-operating-system-versions" => {
                        if request.mobile_operating_system_versions.is_none() {
                           request.mobile_operating_system_versions = Some(Default::default());
                        }
                                        request.mobile_operating_system_versions.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "mobile-carriers" => {
                        if request.mobile_carriers.is_none() {
                           request.mobile_carriers = Some(Default::default());
                        }
                                        request.mobile_carriers.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "config-id" => {
                        request.config_id = Some(value.unwrap_or("").to_string());
                    },
                "excluded-user-lists" => {
                        if request.excluded_user_lists.is_none() {
                           request.excluded_user_lists = Some(Default::default());
                        }
                                        request.excluded_user_lists.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "is-active" => {
                        request.is_active = Some(arg_from_str(value.unwrap_or("false"), err, "is-active", "boolean"));
                    },
                "supported-creative-attributes" => {
                        if request.supported_creative_attributes.is_none() {
                           request.supported_creative_attributes = Some(Default::default());
                        }
                                        request.supported_creative_attributes.as_mut().unwrap().push(value.unwrap_or("").to_string());
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

    fn _doit(&self, dry_run: bool) -> (Option<api::Error>, Option<InvalidOptionsError>) {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Option<api::Error>;
        let mut err_opt: Option<InvalidOptionsError> = None;

        if self.opt.cmd_accounts {
            if self.opt.cmd_get {
                call_result = self._accounts_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._accounts_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._accounts_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._accounts_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_billing_info {
            if self.opt.cmd_get {
                call_result = self._billing_info_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._billing_info_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_budget {
            if self.opt.cmd_get {
                call_result = self._budget_get(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._budget_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._budget_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_creatives {
            if self.opt.cmd_get {
                call_result = self._creatives_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._creatives_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._creatives_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_direct_deals {
            if self.opt.cmd_get {
                call_result = self._direct_deals_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._direct_deals_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_performance_report {
            if self.opt.cmd_list {
                call_result = self._performance_report_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_pretargeting_config {
            if self.opt.cmd_delete {
                call_result = self._pretargeting_config_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._pretargeting_config_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._pretargeting_config_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._pretargeting_config_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._pretargeting_config_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._pretargeting_config_update(dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "adexchangebuyer1d3-secret.json", 
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
                                          program_name: "adexchangebuyer1d3",
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
            hub: api::AdExchangeBuyer::new(client, auth),
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