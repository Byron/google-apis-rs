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
extern crate google_tagmanager1 as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  tagmanager1 [options] accounts containers-create <account-id> -r <kv>... [-p <v>]... [-o <out>]
  tagmanager1 [options] accounts containers-delete <account-id> <container-id> [-p <v>]...
  tagmanager1 [options] accounts containers-get <account-id> <container-id> [-p <v>]... [-o <out>]
  tagmanager1 [options] accounts containers-list <account-id> [-p <v>]... [-o <out>]
  tagmanager1 [options] accounts containers-macros-create <account-id> <container-id> -r <kv>... [-p <v>]... [-o <out>]
  tagmanager1 [options] accounts containers-macros-delete <account-id> <container-id> <macro-id> [-p <v>]...
  tagmanager1 [options] accounts containers-macros-get <account-id> <container-id> <macro-id> [-p <v>]... [-o <out>]
  tagmanager1 [options] accounts containers-macros-list <account-id> <container-id> [-p <v>]... [-o <out>]
  tagmanager1 [options] accounts containers-macros-update <account-id> <container-id> <macro-id> -r <kv>... [-p <v>]... [-o <out>]
  tagmanager1 [options] accounts containers-rules-create <account-id> <container-id> -r <kv>... [-p <v>]... [-o <out>]
  tagmanager1 [options] accounts containers-rules-delete <account-id> <container-id> <rule-id> [-p <v>]...
  tagmanager1 [options] accounts containers-rules-get <account-id> <container-id> <rule-id> [-p <v>]... [-o <out>]
  tagmanager1 [options] accounts containers-rules-list <account-id> <container-id> [-p <v>]... [-o <out>]
  tagmanager1 [options] accounts containers-rules-update <account-id> <container-id> <rule-id> -r <kv>... [-p <v>]... [-o <out>]
  tagmanager1 [options] accounts containers-tags-create <account-id> <container-id> -r <kv>... [-p <v>]... [-o <out>]
  tagmanager1 [options] accounts containers-tags-delete <account-id> <container-id> <tag-id> [-p <v>]...
  tagmanager1 [options] accounts containers-tags-get <account-id> <container-id> <tag-id> [-p <v>]... [-o <out>]
  tagmanager1 [options] accounts containers-tags-list <account-id> <container-id> [-p <v>]... [-o <out>]
  tagmanager1 [options] accounts containers-tags-update <account-id> <container-id> <tag-id> -r <kv>... [-p <v>]... [-o <out>]
  tagmanager1 [options] accounts containers-triggers-create <account-id> <container-id> -r <kv>... [-p <v>]... [-o <out>]
  tagmanager1 [options] accounts containers-triggers-delete <account-id> <container-id> <trigger-id> [-p <v>]...
  tagmanager1 [options] accounts containers-triggers-get <account-id> <container-id> <trigger-id> [-p <v>]... [-o <out>]
  tagmanager1 [options] accounts containers-triggers-list <account-id> <container-id> [-p <v>]... [-o <out>]
  tagmanager1 [options] accounts containers-triggers-update <account-id> <container-id> <trigger-id> -r <kv>... [-p <v>]... [-o <out>]
  tagmanager1 [options] accounts containers-update <account-id> <container-id> -r <kv>... [-p <v>]... [-o <out>]
  tagmanager1 [options] accounts containers-variables-create <account-id> <container-id> -r <kv>... [-p <v>]... [-o <out>]
  tagmanager1 [options] accounts containers-variables-delete <account-id> <container-id> <variable-id> [-p <v>]...
  tagmanager1 [options] accounts containers-variables-get <account-id> <container-id> <variable-id> [-p <v>]... [-o <out>]
  tagmanager1 [options] accounts containers-variables-list <account-id> <container-id> [-p <v>]... [-o <out>]
  tagmanager1 [options] accounts containers-variables-update <account-id> <container-id> <variable-id> -r <kv>... [-p <v>]... [-o <out>]
  tagmanager1 [options] accounts containers-versions-create <account-id> <container-id> -r <kv>... [-p <v>]... [-o <out>]
  tagmanager1 [options] accounts containers-versions-delete <account-id> <container-id> <container-version-id> [-p <v>]...
  tagmanager1 [options] accounts containers-versions-get <account-id> <container-id> <container-version-id> [-p <v>]... [-o <out>]
  tagmanager1 [options] accounts containers-versions-list <account-id> <container-id> [-p <v>]... [-o <out>]
  tagmanager1 [options] accounts containers-versions-publish <account-id> <container-id> <container-version-id> [-p <v>]... [-o <out>]
  tagmanager1 [options] accounts containers-versions-restore <account-id> <container-id> <container-version-id> [-p <v>]... [-o <out>]
  tagmanager1 [options] accounts containers-versions-undelete <account-id> <container-id> <container-version-id> [-p <v>]... [-o <out>]
  tagmanager1 [options] accounts containers-versions-update <account-id> <container-id> <container-version-id> -r <kv>... [-p <v>]... [-o <out>]
  tagmanager1 [options] accounts get <account-id> [-p <v>]... [-o <out>]
  tagmanager1 [options] accounts list [-p <v>]... [-o <out>]
  tagmanager1 [options] accounts permissions-create <account-id> -r <kv>... [-p <v>]... [-o <out>]
  tagmanager1 [options] accounts permissions-delete <account-id> <permission-id> [-p <v>]...
  tagmanager1 [options] accounts permissions-get <account-id> <permission-id> [-p <v>]... [-o <out>]
  tagmanager1 [options] accounts permissions-list <account-id> [-p <v>]... [-o <out>]
  tagmanager1 [options] accounts permissions-update <account-id> <permission-id> -r <kv>... [-p <v>]... [-o <out>]
  tagmanager1 [options] accounts update <account-id> -r <kv>... [-p <v>]... [-o <out>]
  tagmanager1 --help

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
    hub: api::TagManager<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _accounts_containers_create(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Container = Default::default();
        let mut call = self.hub.accounts().containers_create(&request, &self.opt.arg_account_id);
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
                "time-zone-id" => {
                        request.time_zone_id = Some(value.unwrap_or("").to_string());
                    },
                "enabled-built-in-variable" => {
                        if request.enabled_built_in_variable.is_none() {
                            request.enabled_built_in_variable = Some(Default::default());
                        }
                        request.enabled_built_in_variable.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "time-zone-country-id" => {
                        request.time_zone_country_id = Some(arg_from_str(value.unwrap_or("-0"), err, "time-zone-country-id", "int64"));
                    },
                "public-id" => {
                        request.public_id = Some(value.unwrap_or("").to_string());
                    },
                "container-id" => {
                        request.container_id = Some(value.unwrap_or("").to_string());
                    },
                "domain-name" => {
                        if request.domain_name.is_none() {
                            request.domain_name = Some(Default::default());
                        }
                        request.domain_name.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "notes" => {
                        request.notes = Some(value.unwrap_or("").to_string());
                    },
                "usage-context" => {
                        if request.usage_context.is_none() {
                            request.usage_context = Some(Default::default());
                        }
                        request.usage_context.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "fingerprint" => {
                        request.fingerprint = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
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

    fn _accounts_containers_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().containers_delete(&self.opt.arg_account_id, &self.opt.arg_container_id);
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

    fn _accounts_containers_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().containers_get(&self.opt.arg_account_id, &self.opt.arg_container_id);
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

    fn _accounts_containers_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().containers_list(&self.opt.arg_account_id);
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

    fn _accounts_containers_macros_create(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Macro = Default::default();
        let mut call = self.hub.accounts().containers_macros_create(&request, &self.opt.arg_account_id, &self.opt.arg_container_id);
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
                "schedule-start-ms" => {
                        request.schedule_start_ms = Some(value.unwrap_or("").to_string());
                    },
                "schedule-end-ms" => {
                        request.schedule_end_ms = Some(value.unwrap_or("").to_string());
                    },
                "macro-id" => {
                        request.macro_id = Some(value.unwrap_or("").to_string());
                    },
                "container-id" => {
                        request.container_id = Some(value.unwrap_or("").to_string());
                    },
                "notes" => {
                        request.notes = Some(value.unwrap_or("").to_string());
                    },
                "enabling-rule-id" => {
                        if request.enabling_rule_id.is_none() {
                            request.enabling_rule_id = Some(Default::default());
                        }
                        request.enabling_rule_id.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "disabling-rule-id" => {
                        if request.disabling_rule_id.is_none() {
                            request.disabling_rule_id = Some(Default::default());
                        }
                        request.disabling_rule_id.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "fingerprint" => {
                        request.fingerprint = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
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

    fn _accounts_containers_macros_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().containers_macros_delete(&self.opt.arg_account_id, &self.opt.arg_container_id, &self.opt.arg_macro_id);
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

    fn _accounts_containers_macros_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().containers_macros_get(&self.opt.arg_account_id, &self.opt.arg_container_id, &self.opt.arg_macro_id);
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

    fn _accounts_containers_macros_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().containers_macros_list(&self.opt.arg_account_id, &self.opt.arg_container_id);
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

    fn _accounts_containers_macros_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Macro = Default::default();
        let mut call = self.hub.accounts().containers_macros_update(&request, &self.opt.arg_account_id, &self.opt.arg_container_id, &self.opt.arg_macro_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
                "schedule-start-ms" => {
                        request.schedule_start_ms = Some(value.unwrap_or("").to_string());
                    },
                "schedule-end-ms" => {
                        request.schedule_end_ms = Some(value.unwrap_or("").to_string());
                    },
                "macro-id" => {
                        request.macro_id = Some(value.unwrap_or("").to_string());
                    },
                "container-id" => {
                        request.container_id = Some(value.unwrap_or("").to_string());
                    },
                "notes" => {
                        request.notes = Some(value.unwrap_or("").to_string());
                    },
                "enabling-rule-id" => {
                        if request.enabling_rule_id.is_none() {
                            request.enabling_rule_id = Some(Default::default());
                        }
                        request.enabling_rule_id.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "disabling-rule-id" => {
                        if request.disabling_rule_id.is_none() {
                            request.disabling_rule_id = Some(Default::default());
                        }
                        request.disabling_rule_id.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "fingerprint" => {
                        request.fingerprint = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
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

    fn _accounts_containers_rules_create(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Rule = Default::default();
        let mut call = self.hub.accounts().containers_rules_create(&request, &self.opt.arg_account_id, &self.opt.arg_container_id);
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
                "container-id" => {
                        request.container_id = Some(value.unwrap_or("").to_string());
                    },
                "rule-id" => {
                        request.rule_id = Some(value.unwrap_or("").to_string());
                    },
                "notes" => {
                        request.notes = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "fingerprint" => {
                        request.fingerprint = Some(value.unwrap_or("").to_string());
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

    fn _accounts_containers_rules_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().containers_rules_delete(&self.opt.arg_account_id, &self.opt.arg_container_id, &self.opt.arg_rule_id);
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

    fn _accounts_containers_rules_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().containers_rules_get(&self.opt.arg_account_id, &self.opt.arg_container_id, &self.opt.arg_rule_id);
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

    fn _accounts_containers_rules_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().containers_rules_list(&self.opt.arg_account_id, &self.opt.arg_container_id);
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

    fn _accounts_containers_rules_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Rule = Default::default();
        let mut call = self.hub.accounts().containers_rules_update(&request, &self.opt.arg_account_id, &self.opt.arg_container_id, &self.opt.arg_rule_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
                "container-id" => {
                        request.container_id = Some(value.unwrap_or("").to_string());
                    },
                "rule-id" => {
                        request.rule_id = Some(value.unwrap_or("").to_string());
                    },
                "notes" => {
                        request.notes = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "fingerprint" => {
                        request.fingerprint = Some(value.unwrap_or("").to_string());
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

    fn _accounts_containers_tags_create(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Tag = Default::default();
        let mut call = self.hub.accounts().containers_tags_create(&request, &self.opt.arg_account_id, &self.opt.arg_container_id);
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
            fn request_priority_init(request: &mut api::Tag) {
                if request.priority.is_none() {
                    request.priority = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "schedule-start-ms" => {
                        request.schedule_start_ms = Some(value.unwrap_or("").to_string());
                    },
                "schedule-end-ms" => {
                        request.schedule_end_ms = Some(value.unwrap_or("").to_string());
                    },
                "container-id" => {
                        request.container_id = Some(value.unwrap_or("").to_string());
                    },
                "blocking-trigger-id" => {
                        if request.blocking_trigger_id.is_none() {
                            request.blocking_trigger_id = Some(Default::default());
                        }
                        request.blocking_trigger_id.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "tag-id" => {
                        request.tag_id = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "priority.type" => {
                        request_priority_init(&mut request);
                        request.priority.as_mut().unwrap().type_ = value.unwrap_or("").to_string();
                    },
                "priority.value" => {
                        request_priority_init(&mut request);
                        request.priority.as_mut().unwrap().value = value.unwrap_or("").to_string();
                    },
                "priority.key" => {
                        request_priority_init(&mut request);
                        request.priority.as_mut().unwrap().key = value.unwrap_or("").to_string();
                    },
                "blocking-rule-id" => {
                        request_priority_init(&mut request);
                        if request.blocking_rule_id.is_none() {
                            request.blocking_rule_id = Some(Default::default());
                        }
                        request.blocking_rule_id.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "live-only" => {
                        request_priority_init(&mut request);
                        request.live_only = Some(arg_from_str(value.unwrap_or("false"), err, "live-only", "boolean"));
                    },
                "fingerprint" => {
                        request_priority_init(&mut request);
                        request.fingerprint = Some(value.unwrap_or("").to_string());
                    },
                "firing-rule-id" => {
                        request_priority_init(&mut request);
                        if request.firing_rule_id.is_none() {
                            request.firing_rule_id = Some(Default::default());
                        }
                        request.firing_rule_id.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "firing-trigger-id" => {
                        request_priority_init(&mut request);
                        if request.firing_trigger_id.is_none() {
                            request.firing_trigger_id = Some(Default::default());
                        }
                        request.firing_trigger_id.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_priority_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "notes" => {
                        request_priority_init(&mut request);
                        request.notes = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_priority_init(&mut request);
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

    fn _accounts_containers_tags_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().containers_tags_delete(&self.opt.arg_account_id, &self.opt.arg_container_id, &self.opt.arg_tag_id);
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

    fn _accounts_containers_tags_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().containers_tags_get(&self.opt.arg_account_id, &self.opt.arg_container_id, &self.opt.arg_tag_id);
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

    fn _accounts_containers_tags_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().containers_tags_list(&self.opt.arg_account_id, &self.opt.arg_container_id);
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

    fn _accounts_containers_tags_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Tag = Default::default();
        let mut call = self.hub.accounts().containers_tags_update(&request, &self.opt.arg_account_id, &self.opt.arg_container_id, &self.opt.arg_tag_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
            fn request_priority_init(request: &mut api::Tag) {
                if request.priority.is_none() {
                    request.priority = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "schedule-start-ms" => {
                        request.schedule_start_ms = Some(value.unwrap_or("").to_string());
                    },
                "schedule-end-ms" => {
                        request.schedule_end_ms = Some(value.unwrap_or("").to_string());
                    },
                "container-id" => {
                        request.container_id = Some(value.unwrap_or("").to_string());
                    },
                "blocking-trigger-id" => {
                        if request.blocking_trigger_id.is_none() {
                            request.blocking_trigger_id = Some(Default::default());
                        }
                        request.blocking_trigger_id.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "tag-id" => {
                        request.tag_id = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "priority.type" => {
                        request_priority_init(&mut request);
                        request.priority.as_mut().unwrap().type_ = value.unwrap_or("").to_string();
                    },
                "priority.value" => {
                        request_priority_init(&mut request);
                        request.priority.as_mut().unwrap().value = value.unwrap_or("").to_string();
                    },
                "priority.key" => {
                        request_priority_init(&mut request);
                        request.priority.as_mut().unwrap().key = value.unwrap_or("").to_string();
                    },
                "blocking-rule-id" => {
                        request_priority_init(&mut request);
                        if request.blocking_rule_id.is_none() {
                            request.blocking_rule_id = Some(Default::default());
                        }
                        request.blocking_rule_id.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "live-only" => {
                        request_priority_init(&mut request);
                        request.live_only = Some(arg_from_str(value.unwrap_or("false"), err, "live-only", "boolean"));
                    },
                "fingerprint" => {
                        request_priority_init(&mut request);
                        request.fingerprint = Some(value.unwrap_or("").to_string());
                    },
                "firing-rule-id" => {
                        request_priority_init(&mut request);
                        if request.firing_rule_id.is_none() {
                            request.firing_rule_id = Some(Default::default());
                        }
                        request.firing_rule_id.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "firing-trigger-id" => {
                        request_priority_init(&mut request);
                        if request.firing_trigger_id.is_none() {
                            request.firing_trigger_id = Some(Default::default());
                        }
                        request.firing_trigger_id.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_priority_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "notes" => {
                        request_priority_init(&mut request);
                        request.notes = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_priority_init(&mut request);
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

    fn _accounts_containers_triggers_create(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Trigger = Default::default();
        let mut call = self.hub.accounts().containers_triggers_create(&request, &self.opt.arg_account_id, &self.opt.arg_container_id);
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
            fn request_check_validation_init(request: &mut api::Trigger) {
                if request.check_validation.is_none() {
                    request.check_validation = Some(Default::default());
                }
            }
            
            fn request_enable_all_videos_init(request: &mut api::Trigger) {
                if request.enable_all_videos.is_none() {
                    request.enable_all_videos = Some(Default::default());
                }
            }
            
            fn request_event_name_init(request: &mut api::Trigger) {
                if request.event_name.is_none() {
                    request.event_name = Some(Default::default());
                }
            }
            
            fn request_interval_init(request: &mut api::Trigger) {
                if request.interval.is_none() {
                    request.interval = Some(Default::default());
                }
            }
            
            fn request_limit_init(request: &mut api::Trigger) {
                if request.limit.is_none() {
                    request.limit = Some(Default::default());
                }
            }
            
            fn request_unique_trigger_id_init(request: &mut api::Trigger) {
                if request.unique_trigger_id.is_none() {
                    request.unique_trigger_id = Some(Default::default());
                }
            }
            
            fn request_video_percentage_list_init(request: &mut api::Trigger) {
                if request.video_percentage_list.is_none() {
                    request.video_percentage_list = Some(Default::default());
                }
            }
            
            fn request_wait_for_tags_init(request: &mut api::Trigger) {
                if request.wait_for_tags.is_none() {
                    request.wait_for_tags = Some(Default::default());
                }
            }
            
            fn request_wait_for_tags_timeout_init(request: &mut api::Trigger) {
                if request.wait_for_tags_timeout.is_none() {
                    request.wait_for_tags_timeout = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "video-percentage-list.type" => {
                        request_video_percentage_list_init(&mut request);
                        request.video_percentage_list.as_mut().unwrap().type_ = value.unwrap_or("").to_string();
                    },
                "video-percentage-list.value" => {
                        request_video_percentage_list_init(&mut request);
                        request.video_percentage_list.as_mut().unwrap().value = value.unwrap_or("").to_string();
                    },
                "video-percentage-list.key" => {
                        request_video_percentage_list_init(&mut request);
                        request.video_percentage_list.as_mut().unwrap().key = value.unwrap_or("").to_string();
                    },
                "interval.type" => {
                        request_interval_init(&mut request);
                        request.interval.as_mut().unwrap().type_ = value.unwrap_or("").to_string();
                    },
                "interval.value" => {
                        request_interval_init(&mut request);
                        request.interval.as_mut().unwrap().value = value.unwrap_or("").to_string();
                    },
                "interval.key" => {
                        request_interval_init(&mut request);
                        request.interval.as_mut().unwrap().key = value.unwrap_or("").to_string();
                    },
                "wait-for-tags.type" => {
                        request_wait_for_tags_init(&mut request);
                        request.wait_for_tags.as_mut().unwrap().type_ = value.unwrap_or("").to_string();
                    },
                "wait-for-tags.value" => {
                        request_wait_for_tags_init(&mut request);
                        request.wait_for_tags.as_mut().unwrap().value = value.unwrap_or("").to_string();
                    },
                "wait-for-tags.key" => {
                        request_wait_for_tags_init(&mut request);
                        request.wait_for_tags.as_mut().unwrap().key = value.unwrap_or("").to_string();
                    },
                "container-id" => {
                        request_wait_for_tags_init(&mut request);
                        request.container_id = Some(value.unwrap_or("").to_string());
                    },
                "unique-trigger-id.type" => {
                        request_unique_trigger_id_init(&mut request);
                        request.unique_trigger_id.as_mut().unwrap().type_ = value.unwrap_or("").to_string();
                    },
                "unique-trigger-id.value" => {
                        request_unique_trigger_id_init(&mut request);
                        request.unique_trigger_id.as_mut().unwrap().value = value.unwrap_or("").to_string();
                    },
                "unique-trigger-id.key" => {
                        request_unique_trigger_id_init(&mut request);
                        request.unique_trigger_id.as_mut().unwrap().key = value.unwrap_or("").to_string();
                    },
                "wait-for-tags-timeout.type" => {
                        request_wait_for_tags_timeout_init(&mut request);
                        request.wait_for_tags_timeout.as_mut().unwrap().type_ = value.unwrap_or("").to_string();
                    },
                "wait-for-tags-timeout.value" => {
                        request_wait_for_tags_timeout_init(&mut request);
                        request.wait_for_tags_timeout.as_mut().unwrap().value = value.unwrap_or("").to_string();
                    },
                "wait-for-tags-timeout.key" => {
                        request_wait_for_tags_timeout_init(&mut request);
                        request.wait_for_tags_timeout.as_mut().unwrap().key = value.unwrap_or("").to_string();
                    },
                "name" => {
                        request_wait_for_tags_timeout_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "event-name.type" => {
                        request_event_name_init(&mut request);
                        request.event_name.as_mut().unwrap().type_ = value.unwrap_or("").to_string();
                    },
                "event-name.value" => {
                        request_event_name_init(&mut request);
                        request.event_name.as_mut().unwrap().value = value.unwrap_or("").to_string();
                    },
                "event-name.key" => {
                        request_event_name_init(&mut request);
                        request.event_name.as_mut().unwrap().key = value.unwrap_or("").to_string();
                    },
                "trigger-id" => {
                        request_event_name_init(&mut request);
                        request.trigger_id = Some(value.unwrap_or("").to_string());
                    },
                "limit.type" => {
                        request_limit_init(&mut request);
                        request.limit.as_mut().unwrap().type_ = value.unwrap_or("").to_string();
                    },
                "limit.value" => {
                        request_limit_init(&mut request);
                        request.limit.as_mut().unwrap().value = value.unwrap_or("").to_string();
                    },
                "limit.key" => {
                        request_limit_init(&mut request);
                        request.limit.as_mut().unwrap().key = value.unwrap_or("").to_string();
                    },
                "check-validation.type" => {
                        request_check_validation_init(&mut request);
                        request.check_validation.as_mut().unwrap().type_ = value.unwrap_or("").to_string();
                    },
                "check-validation.value" => {
                        request_check_validation_init(&mut request);
                        request.check_validation.as_mut().unwrap().value = value.unwrap_or("").to_string();
                    },
                "check-validation.key" => {
                        request_check_validation_init(&mut request);
                        request.check_validation.as_mut().unwrap().key = value.unwrap_or("").to_string();
                    },
                "fingerprint" => {
                        request_check_validation_init(&mut request);
                        request.fingerprint = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_check_validation_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "enable-all-videos.type" => {
                        request_enable_all_videos_init(&mut request);
                        request.enable_all_videos.as_mut().unwrap().type_ = value.unwrap_or("").to_string();
                    },
                "enable-all-videos.value" => {
                        request_enable_all_videos_init(&mut request);
                        request.enable_all_videos.as_mut().unwrap().value = value.unwrap_or("").to_string();
                    },
                "enable-all-videos.key" => {
                        request_enable_all_videos_init(&mut request);
                        request.enable_all_videos.as_mut().unwrap().key = value.unwrap_or("").to_string();
                    },
                "account-id" => {
                        request_enable_all_videos_init(&mut request);
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

    fn _accounts_containers_triggers_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().containers_triggers_delete(&self.opt.arg_account_id, &self.opt.arg_container_id, &self.opt.arg_trigger_id);
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

    fn _accounts_containers_triggers_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().containers_triggers_get(&self.opt.arg_account_id, &self.opt.arg_container_id, &self.opt.arg_trigger_id);
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

    fn _accounts_containers_triggers_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().containers_triggers_list(&self.opt.arg_account_id, &self.opt.arg_container_id);
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

    fn _accounts_containers_triggers_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Trigger = Default::default();
        let mut call = self.hub.accounts().containers_triggers_update(&request, &self.opt.arg_account_id, &self.opt.arg_container_id, &self.opt.arg_trigger_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
            fn request_check_validation_init(request: &mut api::Trigger) {
                if request.check_validation.is_none() {
                    request.check_validation = Some(Default::default());
                }
            }
            
            fn request_enable_all_videos_init(request: &mut api::Trigger) {
                if request.enable_all_videos.is_none() {
                    request.enable_all_videos = Some(Default::default());
                }
            }
            
            fn request_event_name_init(request: &mut api::Trigger) {
                if request.event_name.is_none() {
                    request.event_name = Some(Default::default());
                }
            }
            
            fn request_interval_init(request: &mut api::Trigger) {
                if request.interval.is_none() {
                    request.interval = Some(Default::default());
                }
            }
            
            fn request_limit_init(request: &mut api::Trigger) {
                if request.limit.is_none() {
                    request.limit = Some(Default::default());
                }
            }
            
            fn request_unique_trigger_id_init(request: &mut api::Trigger) {
                if request.unique_trigger_id.is_none() {
                    request.unique_trigger_id = Some(Default::default());
                }
            }
            
            fn request_video_percentage_list_init(request: &mut api::Trigger) {
                if request.video_percentage_list.is_none() {
                    request.video_percentage_list = Some(Default::default());
                }
            }
            
            fn request_wait_for_tags_init(request: &mut api::Trigger) {
                if request.wait_for_tags.is_none() {
                    request.wait_for_tags = Some(Default::default());
                }
            }
            
            fn request_wait_for_tags_timeout_init(request: &mut api::Trigger) {
                if request.wait_for_tags_timeout.is_none() {
                    request.wait_for_tags_timeout = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "video-percentage-list.type" => {
                        request_video_percentage_list_init(&mut request);
                        request.video_percentage_list.as_mut().unwrap().type_ = value.unwrap_or("").to_string();
                    },
                "video-percentage-list.value" => {
                        request_video_percentage_list_init(&mut request);
                        request.video_percentage_list.as_mut().unwrap().value = value.unwrap_or("").to_string();
                    },
                "video-percentage-list.key" => {
                        request_video_percentage_list_init(&mut request);
                        request.video_percentage_list.as_mut().unwrap().key = value.unwrap_or("").to_string();
                    },
                "interval.type" => {
                        request_interval_init(&mut request);
                        request.interval.as_mut().unwrap().type_ = value.unwrap_or("").to_string();
                    },
                "interval.value" => {
                        request_interval_init(&mut request);
                        request.interval.as_mut().unwrap().value = value.unwrap_or("").to_string();
                    },
                "interval.key" => {
                        request_interval_init(&mut request);
                        request.interval.as_mut().unwrap().key = value.unwrap_or("").to_string();
                    },
                "wait-for-tags.type" => {
                        request_wait_for_tags_init(&mut request);
                        request.wait_for_tags.as_mut().unwrap().type_ = value.unwrap_or("").to_string();
                    },
                "wait-for-tags.value" => {
                        request_wait_for_tags_init(&mut request);
                        request.wait_for_tags.as_mut().unwrap().value = value.unwrap_or("").to_string();
                    },
                "wait-for-tags.key" => {
                        request_wait_for_tags_init(&mut request);
                        request.wait_for_tags.as_mut().unwrap().key = value.unwrap_or("").to_string();
                    },
                "container-id" => {
                        request_wait_for_tags_init(&mut request);
                        request.container_id = Some(value.unwrap_or("").to_string());
                    },
                "unique-trigger-id.type" => {
                        request_unique_trigger_id_init(&mut request);
                        request.unique_trigger_id.as_mut().unwrap().type_ = value.unwrap_or("").to_string();
                    },
                "unique-trigger-id.value" => {
                        request_unique_trigger_id_init(&mut request);
                        request.unique_trigger_id.as_mut().unwrap().value = value.unwrap_or("").to_string();
                    },
                "unique-trigger-id.key" => {
                        request_unique_trigger_id_init(&mut request);
                        request.unique_trigger_id.as_mut().unwrap().key = value.unwrap_or("").to_string();
                    },
                "wait-for-tags-timeout.type" => {
                        request_wait_for_tags_timeout_init(&mut request);
                        request.wait_for_tags_timeout.as_mut().unwrap().type_ = value.unwrap_or("").to_string();
                    },
                "wait-for-tags-timeout.value" => {
                        request_wait_for_tags_timeout_init(&mut request);
                        request.wait_for_tags_timeout.as_mut().unwrap().value = value.unwrap_or("").to_string();
                    },
                "wait-for-tags-timeout.key" => {
                        request_wait_for_tags_timeout_init(&mut request);
                        request.wait_for_tags_timeout.as_mut().unwrap().key = value.unwrap_or("").to_string();
                    },
                "name" => {
                        request_wait_for_tags_timeout_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "event-name.type" => {
                        request_event_name_init(&mut request);
                        request.event_name.as_mut().unwrap().type_ = value.unwrap_or("").to_string();
                    },
                "event-name.value" => {
                        request_event_name_init(&mut request);
                        request.event_name.as_mut().unwrap().value = value.unwrap_or("").to_string();
                    },
                "event-name.key" => {
                        request_event_name_init(&mut request);
                        request.event_name.as_mut().unwrap().key = value.unwrap_or("").to_string();
                    },
                "trigger-id" => {
                        request_event_name_init(&mut request);
                        request.trigger_id = Some(value.unwrap_or("").to_string());
                    },
                "limit.type" => {
                        request_limit_init(&mut request);
                        request.limit.as_mut().unwrap().type_ = value.unwrap_or("").to_string();
                    },
                "limit.value" => {
                        request_limit_init(&mut request);
                        request.limit.as_mut().unwrap().value = value.unwrap_or("").to_string();
                    },
                "limit.key" => {
                        request_limit_init(&mut request);
                        request.limit.as_mut().unwrap().key = value.unwrap_or("").to_string();
                    },
                "check-validation.type" => {
                        request_check_validation_init(&mut request);
                        request.check_validation.as_mut().unwrap().type_ = value.unwrap_or("").to_string();
                    },
                "check-validation.value" => {
                        request_check_validation_init(&mut request);
                        request.check_validation.as_mut().unwrap().value = value.unwrap_or("").to_string();
                    },
                "check-validation.key" => {
                        request_check_validation_init(&mut request);
                        request.check_validation.as_mut().unwrap().key = value.unwrap_or("").to_string();
                    },
                "fingerprint" => {
                        request_check_validation_init(&mut request);
                        request.fingerprint = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_check_validation_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "enable-all-videos.type" => {
                        request_enable_all_videos_init(&mut request);
                        request.enable_all_videos.as_mut().unwrap().type_ = value.unwrap_or("").to_string();
                    },
                "enable-all-videos.value" => {
                        request_enable_all_videos_init(&mut request);
                        request.enable_all_videos.as_mut().unwrap().value = value.unwrap_or("").to_string();
                    },
                "enable-all-videos.key" => {
                        request_enable_all_videos_init(&mut request);
                        request.enable_all_videos.as_mut().unwrap().key = value.unwrap_or("").to_string();
                    },
                "account-id" => {
                        request_enable_all_videos_init(&mut request);
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

    fn _accounts_containers_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Container = Default::default();
        let mut call = self.hub.accounts().containers_update(&request, &self.opt.arg_account_id, &self.opt.arg_container_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
                "time-zone-id" => {
                        request.time_zone_id = Some(value.unwrap_or("").to_string());
                    },
                "enabled-built-in-variable" => {
                        if request.enabled_built_in_variable.is_none() {
                            request.enabled_built_in_variable = Some(Default::default());
                        }
                        request.enabled_built_in_variable.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "time-zone-country-id" => {
                        request.time_zone_country_id = Some(arg_from_str(value.unwrap_or("-0"), err, "time-zone-country-id", "int64"));
                    },
                "public-id" => {
                        request.public_id = Some(value.unwrap_or("").to_string());
                    },
                "container-id" => {
                        request.container_id = Some(value.unwrap_or("").to_string());
                    },
                "domain-name" => {
                        if request.domain_name.is_none() {
                            request.domain_name = Some(Default::default());
                        }
                        request.domain_name.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "notes" => {
                        request.notes = Some(value.unwrap_or("").to_string());
                    },
                "usage-context" => {
                        if request.usage_context.is_none() {
                            request.usage_context = Some(Default::default());
                        }
                        request.usage_context.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "fingerprint" => {
                        request.fingerprint = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
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

    fn _accounts_containers_variables_create(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Variable = Default::default();
        let mut call = self.hub.accounts().containers_variables_create(&request, &self.opt.arg_account_id, &self.opt.arg_container_id);
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
                "schedule-start-ms" => {
                        request.schedule_start_ms = Some(value.unwrap_or("").to_string());
                    },
                "schedule-end-ms" => {
                        request.schedule_end_ms = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "variable-id" => {
                        request.variable_id = Some(value.unwrap_or("").to_string());
                    },
                "notes" => {
                        request.notes = Some(value.unwrap_or("").to_string());
                    },
                "enabling-trigger-id" => {
                        if request.enabling_trigger_id.is_none() {
                            request.enabling_trigger_id = Some(Default::default());
                        }
                        request.enabling_trigger_id.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "fingerprint" => {
                        request.fingerprint = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "disabling-trigger-id" => {
                        if request.disabling_trigger_id.is_none() {
                            request.disabling_trigger_id = Some(Default::default());
                        }
                        request.disabling_trigger_id.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "container-id" => {
                        request.container_id = Some(value.unwrap_or("").to_string());
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

    fn _accounts_containers_variables_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().containers_variables_delete(&self.opt.arg_account_id, &self.opt.arg_container_id, &self.opt.arg_variable_id);
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

    fn _accounts_containers_variables_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().containers_variables_get(&self.opt.arg_account_id, &self.opt.arg_container_id, &self.opt.arg_variable_id);
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

    fn _accounts_containers_variables_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().containers_variables_list(&self.opt.arg_account_id, &self.opt.arg_container_id);
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

    fn _accounts_containers_variables_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Variable = Default::default();
        let mut call = self.hub.accounts().containers_variables_update(&request, &self.opt.arg_account_id, &self.opt.arg_container_id, &self.opt.arg_variable_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
                "schedule-start-ms" => {
                        request.schedule_start_ms = Some(value.unwrap_or("").to_string());
                    },
                "schedule-end-ms" => {
                        request.schedule_end_ms = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "variable-id" => {
                        request.variable_id = Some(value.unwrap_or("").to_string());
                    },
                "notes" => {
                        request.notes = Some(value.unwrap_or("").to_string());
                    },
                "enabling-trigger-id" => {
                        if request.enabling_trigger_id.is_none() {
                            request.enabling_trigger_id = Some(Default::default());
                        }
                        request.enabling_trigger_id.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "fingerprint" => {
                        request.fingerprint = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "disabling-trigger-id" => {
                        if request.disabling_trigger_id.is_none() {
                            request.disabling_trigger_id = Some(Default::default());
                        }
                        request.disabling_trigger_id.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "container-id" => {
                        request.container_id = Some(value.unwrap_or("").to_string());
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

    fn _accounts_containers_versions_create(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::CreateContainerVersionRequestVersionOptions = Default::default();
        let mut call = self.hub.accounts().containers_versions_create(&request, &self.opt.arg_account_id, &self.opt.arg_container_id);
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
                "notes" => {
                        request.notes = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "quick-preview" => {
                        request.quick_preview = Some(arg_from_str(value.unwrap_or("false"), err, "quick-preview", "boolean"));
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

    fn _accounts_containers_versions_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().containers_versions_delete(&self.opt.arg_account_id, &self.opt.arg_container_id, &self.opt.arg_container_version_id);
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

    fn _accounts_containers_versions_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().containers_versions_get(&self.opt.arg_account_id, &self.opt.arg_container_id, &self.opt.arg_container_version_id);
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

    fn _accounts_containers_versions_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().containers_versions_list(&self.opt.arg_account_id, &self.opt.arg_container_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "headers" => {
                    call = call.headers(arg_from_str(value.unwrap_or("false"), err, "headers", "boolean"));
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

    fn _accounts_containers_versions_publish(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().containers_versions_publish(&self.opt.arg_account_id, &self.opt.arg_container_id, &self.opt.arg_container_version_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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

    fn _accounts_containers_versions_restore(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().containers_versions_restore(&self.opt.arg_account_id, &self.opt.arg_container_id, &self.opt.arg_container_version_id);
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

    fn _accounts_containers_versions_undelete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().containers_versions_undelete(&self.opt.arg_account_id, &self.opt.arg_container_id, &self.opt.arg_container_version_id);
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

    fn _accounts_containers_versions_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::ContainerVersion = Default::default();
        let mut call = self.hub.accounts().containers_versions_update(&request, &self.opt.arg_account_id, &self.opt.arg_container_id, &self.opt.arg_container_version_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
            fn request_container_init(request: &mut api::ContainerVersion) {
                if request.container.is_none() {
                    request.container = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "container.time-zone-id" => {
                        request_container_init(&mut request);
                        request.container.as_mut().unwrap().time_zone_id = Some(value.unwrap_or("").to_string());
                    },
                "container.enabled-built-in-variable" => {
                        request_container_init(&mut request);
                        if request.container.as_mut().unwrap().enabled_built_in_variable.is_none() {
                            request.container.as_mut().unwrap().enabled_built_in_variable = Some(Default::default());
                        }
                        request.container.as_mut().unwrap().enabled_built_in_variable.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "container.time-zone-country-id" => {
                        request_container_init(&mut request);
                        request.container.as_mut().unwrap().time_zone_country_id = Some(arg_from_str(value.unwrap_or("-0"), err, "container.time-zone-country-id", "int64"));
                    },
                "container.public-id" => {
                        request_container_init(&mut request);
                        request.container.as_mut().unwrap().public_id = Some(value.unwrap_or("").to_string());
                    },
                "container.container-id" => {
                        request_container_init(&mut request);
                        request.container.as_mut().unwrap().container_id = Some(value.unwrap_or("").to_string());
                    },
                "container.domain-name" => {
                        request_container_init(&mut request);
                        if request.container.as_mut().unwrap().domain_name.is_none() {
                            request.container.as_mut().unwrap().domain_name = Some(Default::default());
                        }
                        request.container.as_mut().unwrap().domain_name.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "container.notes" => {
                        request_container_init(&mut request);
                        request.container.as_mut().unwrap().notes = Some(value.unwrap_or("").to_string());
                    },
                "container.usage-context" => {
                        request_container_init(&mut request);
                        if request.container.as_mut().unwrap().usage_context.is_none() {
                            request.container.as_mut().unwrap().usage_context = Some(Default::default());
                        }
                        request.container.as_mut().unwrap().usage_context.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "container.fingerprint" => {
                        request_container_init(&mut request);
                        request.container.as_mut().unwrap().fingerprint = Some(value.unwrap_or("").to_string());
                    },
                "container.account-id" => {
                        request_container_init(&mut request);
                        request.container.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "container.name" => {
                        request_container_init(&mut request);
                        request.container.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "container-id" => {
                        request_container_init(&mut request);
                        request.container_id = Some(value.unwrap_or("").to_string());
                    },
                "deleted" => {
                        request_container_init(&mut request);
                        request.deleted = Some(arg_from_str(value.unwrap_or("false"), err, "deleted", "boolean"));
                    },
                "notes" => {
                        request_container_init(&mut request);
                        request.notes = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_container_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "container-version-id" => {
                        request_container_init(&mut request);
                        request.container_version_id = Some(value.unwrap_or("").to_string());
                    },
                "fingerprint" => {
                        request_container_init(&mut request);
                        request.fingerprint = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_container_init(&mut request);
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

    fn _accounts_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().get(&self.opt.arg_account_id);
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

    fn _accounts_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().list();
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

    fn _accounts_permissions_create(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::UserAccess = Default::default();
        let mut call = self.hub.accounts().permissions_create(&request, &self.opt.arg_account_id);
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
            fn request_account_access_init(request: &mut api::UserAccess) {
                if request.account_access.is_none() {
                    request.account_access = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "account-access.permission" => {
                        request_account_access_init(&mut request);
                        request.account_access.as_mut().unwrap().permission.push(value.unwrap_or("").to_string());
                    },
                "email-address" => {
                        request_account_access_init(&mut request);
                        request.email_address = Some(value.unwrap_or("").to_string());
                    },
                "permission-id" => {
                        request_account_access_init(&mut request);
                        request.permission_id = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_account_access_init(&mut request);
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

    fn _accounts_permissions_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().permissions_delete(&self.opt.arg_account_id, &self.opt.arg_permission_id);
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

    fn _accounts_permissions_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().permissions_get(&self.opt.arg_account_id, &self.opt.arg_permission_id);
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

    fn _accounts_permissions_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().permissions_list(&self.opt.arg_account_id);
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

    fn _accounts_permissions_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::UserAccess = Default::default();
        let mut call = self.hub.accounts().permissions_update(&request, &self.opt.arg_account_id, &self.opt.arg_permission_id);
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
            fn request_account_access_init(request: &mut api::UserAccess) {
                if request.account_access.is_none() {
                    request.account_access = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "account-access.permission" => {
                        request_account_access_init(&mut request);
                        request.account_access.as_mut().unwrap().permission.push(value.unwrap_or("").to_string());
                    },
                "email-address" => {
                        request_account_access_init(&mut request);
                        request.email_address = Some(value.unwrap_or("").to_string());
                    },
                "permission-id" => {
                        request_account_access_init(&mut request);
                        request.permission_id = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_account_access_init(&mut request);
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

    fn _accounts_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Account = Default::default();
        let mut call = self.hub.accounts().update(&request, &self.opt.arg_account_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "fingerprint" => {
                    call = call.fingerprint(value.unwrap_or(""));
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
                "share-data" => {
                        request.share_data = Some(arg_from_str(value.unwrap_or("false"), err, "share-data", "boolean"));
                    },
                "fingerprint" => {
                        request.fingerprint = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
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

    fn _doit(&self, dry_run: bool) -> (Option<api::Error>, Option<InvalidOptionsError>) {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Option<api::Error>;
        let mut err_opt: Option<InvalidOptionsError> = None;

        if self.opt.cmd_accounts {
            if self.opt.cmd_containers_create {
                call_result = self._accounts_containers_create(dry_run, &mut err);
            } else if self.opt.cmd_containers_delete {
                call_result = self._accounts_containers_delete(dry_run, &mut err);
            } else if self.opt.cmd_containers_get {
                call_result = self._accounts_containers_get(dry_run, &mut err);
            } else if self.opt.cmd_containers_list {
                call_result = self._accounts_containers_list(dry_run, &mut err);
            } else if self.opt.cmd_containers_macros_create {
                call_result = self._accounts_containers_macros_create(dry_run, &mut err);
            } else if self.opt.cmd_containers_macros_delete {
                call_result = self._accounts_containers_macros_delete(dry_run, &mut err);
            } else if self.opt.cmd_containers_macros_get {
                call_result = self._accounts_containers_macros_get(dry_run, &mut err);
            } else if self.opt.cmd_containers_macros_list {
                call_result = self._accounts_containers_macros_list(dry_run, &mut err);
            } else if self.opt.cmd_containers_macros_update {
                call_result = self._accounts_containers_macros_update(dry_run, &mut err);
            } else if self.opt.cmd_containers_rules_create {
                call_result = self._accounts_containers_rules_create(dry_run, &mut err);
            } else if self.opt.cmd_containers_rules_delete {
                call_result = self._accounts_containers_rules_delete(dry_run, &mut err);
            } else if self.opt.cmd_containers_rules_get {
                call_result = self._accounts_containers_rules_get(dry_run, &mut err);
            } else if self.opt.cmd_containers_rules_list {
                call_result = self._accounts_containers_rules_list(dry_run, &mut err);
            } else if self.opt.cmd_containers_rules_update {
                call_result = self._accounts_containers_rules_update(dry_run, &mut err);
            } else if self.opt.cmd_containers_tags_create {
                call_result = self._accounts_containers_tags_create(dry_run, &mut err);
            } else if self.opt.cmd_containers_tags_delete {
                call_result = self._accounts_containers_tags_delete(dry_run, &mut err);
            } else if self.opt.cmd_containers_tags_get {
                call_result = self._accounts_containers_tags_get(dry_run, &mut err);
            } else if self.opt.cmd_containers_tags_list {
                call_result = self._accounts_containers_tags_list(dry_run, &mut err);
            } else if self.opt.cmd_containers_tags_update {
                call_result = self._accounts_containers_tags_update(dry_run, &mut err);
            } else if self.opt.cmd_containers_triggers_create {
                call_result = self._accounts_containers_triggers_create(dry_run, &mut err);
            } else if self.opt.cmd_containers_triggers_delete {
                call_result = self._accounts_containers_triggers_delete(dry_run, &mut err);
            } else if self.opt.cmd_containers_triggers_get {
                call_result = self._accounts_containers_triggers_get(dry_run, &mut err);
            } else if self.opt.cmd_containers_triggers_list {
                call_result = self._accounts_containers_triggers_list(dry_run, &mut err);
            } else if self.opt.cmd_containers_triggers_update {
                call_result = self._accounts_containers_triggers_update(dry_run, &mut err);
            } else if self.opt.cmd_containers_update {
                call_result = self._accounts_containers_update(dry_run, &mut err);
            } else if self.opt.cmd_containers_variables_create {
                call_result = self._accounts_containers_variables_create(dry_run, &mut err);
            } else if self.opt.cmd_containers_variables_delete {
                call_result = self._accounts_containers_variables_delete(dry_run, &mut err);
            } else if self.opt.cmd_containers_variables_get {
                call_result = self._accounts_containers_variables_get(dry_run, &mut err);
            } else if self.opt.cmd_containers_variables_list {
                call_result = self._accounts_containers_variables_list(dry_run, &mut err);
            } else if self.opt.cmd_containers_variables_update {
                call_result = self._accounts_containers_variables_update(dry_run, &mut err);
            } else if self.opt.cmd_containers_versions_create {
                call_result = self._accounts_containers_versions_create(dry_run, &mut err);
            } else if self.opt.cmd_containers_versions_delete {
                call_result = self._accounts_containers_versions_delete(dry_run, &mut err);
            } else if self.opt.cmd_containers_versions_get {
                call_result = self._accounts_containers_versions_get(dry_run, &mut err);
            } else if self.opt.cmd_containers_versions_list {
                call_result = self._accounts_containers_versions_list(dry_run, &mut err);
            } else if self.opt.cmd_containers_versions_publish {
                call_result = self._accounts_containers_versions_publish(dry_run, &mut err);
            } else if self.opt.cmd_containers_versions_restore {
                call_result = self._accounts_containers_versions_restore(dry_run, &mut err);
            } else if self.opt.cmd_containers_versions_undelete {
                call_result = self._accounts_containers_versions_undelete(dry_run, &mut err);
            } else if self.opt.cmd_containers_versions_update {
                call_result = self._accounts_containers_versions_update(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._accounts_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._accounts_list(dry_run, &mut err);
            } else if self.opt.cmd_permissions_create {
                call_result = self._accounts_permissions_create(dry_run, &mut err);
            } else if self.opt.cmd_permissions_delete {
                call_result = self._accounts_permissions_delete(dry_run, &mut err);
            } else if self.opt.cmd_permissions_get {
                call_result = self._accounts_permissions_get(dry_run, &mut err);
            } else if self.opt.cmd_permissions_list {
                call_result = self._accounts_permissions_list(dry_run, &mut err);
            } else if self.opt.cmd_permissions_update {
                call_result = self._accounts_permissions_update(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._accounts_update(dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "tagmanager1-secret.json") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                                      hyper::Client::new(),
                                      JsonTokenStorage {
                                        program_name: "tagmanager1",
                                        db_dir: config_dir.clone(),
                                      }, None);
        let engine = Engine {
            opt: opt,
            hub: api::TagManager::new(hyper::Client::new(), auth),
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