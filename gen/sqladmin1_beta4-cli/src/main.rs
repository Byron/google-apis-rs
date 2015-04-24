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
extern crate google_sqladmin1_beta4 as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  sqladmin1-beta4 [options] backup-runs get <project> <instance> <id> [-p <v>]... [-o <out>]
  sqladmin1-beta4 [options] backup-runs list <project> <instance> [-p <v>]... [-o <out>]
  sqladmin1-beta4 [options] databases delete <project> <instance> <database> [-p <v>]... [-o <out>]
  sqladmin1-beta4 [options] databases get <project> <instance> <database> [-p <v>]... [-o <out>]
  sqladmin1-beta4 [options] databases insert <project> <instance> -r <kv>... [-p <v>]... [-o <out>]
  sqladmin1-beta4 [options] databases list <project> <instance> [-p <v>]... [-o <out>]
  sqladmin1-beta4 [options] databases patch <project> <instance> <database> -r <kv>... [-p <v>]... [-o <out>]
  sqladmin1-beta4 [options] databases update <project> <instance> <database> -r <kv>... [-p <v>]... [-o <out>]
  sqladmin1-beta4 [options] flags list [-p <v>]... [-o <out>]
  sqladmin1-beta4 [options] instances clone <project> <instance> -r <kv>... [-p <v>]... [-o <out>]
  sqladmin1-beta4 [options] instances delete <project> <instance> [-p <v>]... [-o <out>]
  sqladmin1-beta4 [options] instances export <project> <instance> -r <kv>... [-p <v>]... [-o <out>]
  sqladmin1-beta4 [options] instances get <project> <instance> [-p <v>]... [-o <out>]
  sqladmin1-beta4 [options] instances import <project> <instance> -r <kv>... [-p <v>]... [-o <out>]
  sqladmin1-beta4 [options] instances insert <project> -r <kv>... [-p <v>]... [-o <out>]
  sqladmin1-beta4 [options] instances list <project> [-p <v>]... [-o <out>]
  sqladmin1-beta4 [options] instances patch <project> <instance> -r <kv>... [-p <v>]... [-o <out>]
  sqladmin1-beta4 [options] instances promote-replica <project> <instance> [-p <v>]... [-o <out>]
  sqladmin1-beta4 [options] instances reset-ssl-config <project> <instance> [-p <v>]... [-o <out>]
  sqladmin1-beta4 [options] instances restart <project> <instance> [-p <v>]... [-o <out>]
  sqladmin1-beta4 [options] instances restore-backup <project> <instance> -r <kv>... [-p <v>]... [-o <out>]
  sqladmin1-beta4 [options] instances start-replica <project> <instance> [-p <v>]... [-o <out>]
  sqladmin1-beta4 [options] instances stop-replica <project> <instance> [-p <v>]... [-o <out>]
  sqladmin1-beta4 [options] instances update <project> <instance> -r <kv>... [-p <v>]... [-o <out>]
  sqladmin1-beta4 [options] operations get <project> <operation> [-p <v>]... [-o <out>]
  sqladmin1-beta4 [options] operations list <project> <instance> [-p <v>]... [-o <out>]
  sqladmin1-beta4 [options] ssl-certs delete <project> <instance> <sha1-fingerprint> [-p <v>]... [-o <out>]
  sqladmin1-beta4 [options] ssl-certs get <project> <instance> <sha1-fingerprint> [-p <v>]... [-o <out>]
  sqladmin1-beta4 [options] ssl-certs insert <project> <instance> -r <kv>... [-p <v>]... [-o <out>]
  sqladmin1-beta4 [options] ssl-certs list <project> <instance> [-p <v>]... [-o <out>]
  sqladmin1-beta4 [options] tiers list <project> [-p <v>]... [-o <out>]
  sqladmin1-beta4 [options] users delete <project> <instance> <host> <name> [-p <v>]... [-o <out>]
  sqladmin1-beta4 [options] users insert <project> <instance> -r <kv>... [-p <v>]... [-o <out>]
  sqladmin1-beta4 [options] users list <project> <instance> [-p <v>]... [-o <out>]
  sqladmin1-beta4 [options] users update <project> <instance> <host> <name> -r <kv>... [-p <v>]... [-o <out>]
  sqladmin1-beta4 --help

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
    hub: api::SQLAdmin<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _backup_runs_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.backup_runs().get(&self.opt.arg_project, &self.opt.arg_instance, &self.opt.arg_id);
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

    fn _backup_runs_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.backup_runs().list(&self.opt.arg_project, &self.opt.arg_instance);
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

    fn _databases_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.databases().delete(&self.opt.arg_project, &self.opt.arg_instance, &self.opt.arg_database);
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

    fn _databases_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.databases().get(&self.opt.arg_project, &self.opt.arg_instance, &self.opt.arg_database);
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

    fn _databases_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Database::default();
        let mut call = self.hub.databases().insert(&request, &self.opt.arg_project, &self.opt.arg_instance);
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
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "charset" => {
                        request.charset = Some(value.unwrap_or("").to_string());
                    },
                "project" => {
                        request.project = Some(value.unwrap_or("").to_string());
                    },
                "instance" => {
                        request.instance = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "collation" => {
                        request.collation = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request.self_link = Some(value.unwrap_or("").to_string());
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

    fn _databases_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.databases().list(&self.opt.arg_project, &self.opt.arg_instance);
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

    fn _databases_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Database::default();
        let mut call = self.hub.databases().patch(&request, &self.opt.arg_project, &self.opt.arg_instance, &self.opt.arg_database);
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
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "charset" => {
                        request.charset = Some(value.unwrap_or("").to_string());
                    },
                "project" => {
                        request.project = Some(value.unwrap_or("").to_string());
                    },
                "instance" => {
                        request.instance = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "collation" => {
                        request.collation = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request.self_link = Some(value.unwrap_or("").to_string());
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

    fn _databases_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Database::default();
        let mut call = self.hub.databases().update(&request, &self.opt.arg_project, &self.opt.arg_instance, &self.opt.arg_database);
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
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "charset" => {
                        request.charset = Some(value.unwrap_or("").to_string());
                    },
                "project" => {
                        request.project = Some(value.unwrap_or("").to_string());
                    },
                "instance" => {
                        request.instance = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "collation" => {
                        request.collation = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request.self_link = Some(value.unwrap_or("").to_string());
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

    fn _flags_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.flags().list();
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

    fn _instances_clone(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::InstancesCloneRequest::default();
        let mut call = self.hub.instances().clone(&request, &self.opt.arg_project, &self.opt.arg_instance);
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
            fn request_clone_context_bin_log_coordinates_init(request: &mut api::InstancesCloneRequest) {
                request_clone_context_init(request);
                if request.clone_context.as_mut().unwrap().bin_log_coordinates.is_none() {
                    request.clone_context.as_mut().unwrap().bin_log_coordinates = Some(Default::default());
                }
            }
            
            fn request_clone_context_init(request: &mut api::InstancesCloneRequest) {
                if request.clone_context.is_none() {
                    request.clone_context = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "clone-context.bin-log-coordinates.bin-log-position" => {
                        request_clone_context_bin_log_coordinates_init(&mut request);
                        request.clone_context.as_mut().unwrap().bin_log_coordinates.as_mut().unwrap().bin_log_position = Some(value.unwrap_or("").to_string());
                    },
                "clone-context.bin-log-coordinates.kind" => {
                        request_clone_context_bin_log_coordinates_init(&mut request);
                        request.clone_context.as_mut().unwrap().bin_log_coordinates.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "clone-context.bin-log-coordinates.bin-log-file-name" => {
                        request_clone_context_bin_log_coordinates_init(&mut request);
                        request.clone_context.as_mut().unwrap().bin_log_coordinates.as_mut().unwrap().bin_log_file_name = Some(value.unwrap_or("").to_string());
                    },
                "clone-context.kind" => {
                        request_clone_context_bin_log_coordinates_init(&mut request);
                        request.clone_context.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "clone-context.destination-instance-name" => {
                        request_clone_context_bin_log_coordinates_init(&mut request);
                        request.clone_context.as_mut().unwrap().destination_instance_name = Some(value.unwrap_or("").to_string());
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

    fn _instances_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.instances().delete(&self.opt.arg_project, &self.opt.arg_instance);
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

    fn _instances_export(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::InstancesExportRequest::default();
        let mut call = self.hub.instances().export(&request, &self.opt.arg_project, &self.opt.arg_instance);
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
            fn request_export_context_csv_export_options_init(request: &mut api::InstancesExportRequest) {
                request_export_context_init(request);
                if request.export_context.as_mut().unwrap().csv_export_options.is_none() {
                    request.export_context.as_mut().unwrap().csv_export_options = Some(Default::default());
                }
            }
            
            fn request_export_context_init(request: &mut api::InstancesExportRequest) {
                if request.export_context.is_none() {
                    request.export_context = Some(Default::default());
                }
            }
            
            fn request_export_context_sql_export_options_init(request: &mut api::InstancesExportRequest) {
                request_export_context_init(request);
                if request.export_context.as_mut().unwrap().sql_export_options.is_none() {
                    request.export_context.as_mut().unwrap().sql_export_options = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "export-context.kind" => {
                        request_export_context_init(&mut request);
                        request.export_context.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "export-context.file-type" => {
                        request_export_context_init(&mut request);
                        request.export_context.as_mut().unwrap().file_type = Some(value.unwrap_or("").to_string());
                    },
                "export-context.uri" => {
                        request_export_context_init(&mut request);
                        request.export_context.as_mut().unwrap().uri = Some(value.unwrap_or("").to_string());
                    },
                "export-context.csv-export-options.select-query" => {
                        request_export_context_csv_export_options_init(&mut request);
                        request.export_context.as_mut().unwrap().csv_export_options.as_mut().unwrap().select_query = Some(value.unwrap_or("").to_string());
                    },
                "export-context.databases" => {
                        request_export_context_csv_export_options_init(&mut request);
                        if request.export_context.as_mut().unwrap().databases.is_none() {
                           request.export_context.as_mut().unwrap().databases = Some(Default::default());
                        }
                                        request.export_context.as_mut().unwrap().databases.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "export-context.sql-export-options.tables" => {
                        request_export_context_sql_export_options_init(&mut request);
                        if request.export_context.as_mut().unwrap().sql_export_options.as_mut().unwrap().tables.is_none() {
                           request.export_context.as_mut().unwrap().sql_export_options.as_mut().unwrap().tables = Some(Default::default());
                        }
                                        request.export_context.as_mut().unwrap().sql_export_options.as_mut().unwrap().tables.as_mut().unwrap().push(value.unwrap_or("").to_string());
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

    fn _instances_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.instances().get(&self.opt.arg_project, &self.opt.arg_instance);
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

    fn _instances_import(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::InstancesImportRequest::default();
        let mut call = self.hub.instances().import(&request, &self.opt.arg_project, &self.opt.arg_instance);
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
            fn request_import_context_csv_import_options_init(request: &mut api::InstancesImportRequest) {
                request_import_context_init(request);
                if request.import_context.as_mut().unwrap().csv_import_options.is_none() {
                    request.import_context.as_mut().unwrap().csv_import_options = Some(Default::default());
                }
            }
            
            fn request_import_context_init(request: &mut api::InstancesImportRequest) {
                if request.import_context.is_none() {
                    request.import_context = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "import-context.file-type" => {
                        request_import_context_init(&mut request);
                        request.import_context.as_mut().unwrap().file_type = Some(value.unwrap_or("").to_string());
                    },
                "import-context.database" => {
                        request_import_context_init(&mut request);
                        request.import_context.as_mut().unwrap().database = Some(value.unwrap_or("").to_string());
                    },
                "import-context.uri" => {
                        request_import_context_init(&mut request);
                        request.import_context.as_mut().unwrap().uri = Some(value.unwrap_or("").to_string());
                    },
                "import-context.kind" => {
                        request_import_context_init(&mut request);
                        request.import_context.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "import-context.csv-import-options.table" => {
                        request_import_context_csv_import_options_init(&mut request);
                        request.import_context.as_mut().unwrap().csv_import_options.as_mut().unwrap().table = Some(value.unwrap_or("").to_string());
                    },
                "import-context.csv-import-options.columns" => {
                        request_import_context_csv_import_options_init(&mut request);
                        if request.import_context.as_mut().unwrap().csv_import_options.as_mut().unwrap().columns.is_none() {
                           request.import_context.as_mut().unwrap().csv_import_options.as_mut().unwrap().columns = Some(Default::default());
                        }
                                        request.import_context.as_mut().unwrap().csv_import_options.as_mut().unwrap().columns.as_mut().unwrap().push(value.unwrap_or("").to_string());
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

    fn _instances_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::DatabaseInstance::default();
        let mut call = self.hub.instances().insert(&request, &self.opt.arg_project);
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
            fn request_on_premises_configuration_init(request: &mut api::DatabaseInstance) {
                if request.on_premises_configuration.is_none() {
                    request.on_premises_configuration = Some(Default::default());
                }
            }
            
            fn request_replica_configuration_init(request: &mut api::DatabaseInstance) {
                if request.replica_configuration.is_none() {
                    request.replica_configuration = Some(Default::default());
                }
            }
            
            fn request_replica_configuration_mysql_replica_configuration_init(request: &mut api::DatabaseInstance) {
                request_replica_configuration_init(request);
                if request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.is_none() {
                    request.replica_configuration.as_mut().unwrap().mysql_replica_configuration = Some(Default::default());
                }
            }
            
            fn request_server_ca_cert_init(request: &mut api::DatabaseInstance) {
                if request.server_ca_cert.is_none() {
                    request.server_ca_cert = Some(Default::default());
                }
            }
            
            fn request_settings_backup_configuration_init(request: &mut api::DatabaseInstance) {
                request_settings_init(request);
                if request.settings.as_mut().unwrap().backup_configuration.is_none() {
                    request.settings.as_mut().unwrap().backup_configuration = Some(Default::default());
                }
            }
            
            fn request_settings_init(request: &mut api::DatabaseInstance) {
                if request.settings.is_none() {
                    request.settings = Some(Default::default());
                }
            }
            
            fn request_settings_ip_configuration_init(request: &mut api::DatabaseInstance) {
                request_settings_init(request);
                if request.settings.as_mut().unwrap().ip_configuration.is_none() {
                    request.settings.as_mut().unwrap().ip_configuration = Some(Default::default());
                }
            }
            
            fn request_settings_location_preference_init(request: &mut api::DatabaseInstance) {
                request_settings_init(request);
                if request.settings.as_mut().unwrap().location_preference.is_none() {
                    request.settings.as_mut().unwrap().location_preference = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "on-premises-configuration.kind" => {
                        request_on_premises_configuration_init(&mut request);
                        request.on_premises_configuration.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "on-premises-configuration.host-port" => {
                        request_on_premises_configuration_init(&mut request);
                        request.on_premises_configuration.as_mut().unwrap().host_port = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_on_premises_configuration_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "max-disk-size" => {
                        request_on_premises_configuration_init(&mut request);
                        request.max_disk_size = Some(value.unwrap_or("").to_string());
                    },
                "ipv6-address" => {
                        request_on_premises_configuration_init(&mut request);
                        request.ipv6_address = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.cert-serial-number" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().cert_serial_number = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.kind" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.sha1-fingerprint" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().sha1_fingerprint = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.common-name" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().common_name = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.instance" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().instance = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.cert" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().cert = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.expiration-time" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().expiration_time = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.create-time" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().create_time = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.self-link" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().self_link = Some(value.unwrap_or("").to_string());
                    },
                "replica-names" => {
                        request_server_ca_cert_init(&mut request);
                        if request.replica_names.is_none() {
                           request.replica_names = Some(Default::default());
                        }
                                        request.replica_names.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "project" => {
                        request_server_ca_cert_init(&mut request);
                        request.project = Some(value.unwrap_or("").to_string());
                    },
                "region" => {
                        request_server_ca_cert_init(&mut request);
                        request.region = Some(value.unwrap_or("").to_string());
                    },
                "settings.kind" => {
                        request_settings_init(&mut request);
                        request.settings.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "settings.authorized-gae-applications" => {
                        request_settings_init(&mut request);
                        if request.settings.as_mut().unwrap().authorized_gae_applications.is_none() {
                           request.settings.as_mut().unwrap().authorized_gae_applications = Some(Default::default());
                        }
                                        request.settings.as_mut().unwrap().authorized_gae_applications.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "settings.activation-policy" => {
                        request_settings_init(&mut request);
                        request.settings.as_mut().unwrap().activation_policy = Some(value.unwrap_or("").to_string());
                    },
                "settings.backup-configuration.kind" => {
                        request_settings_backup_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().backup_configuration.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "settings.backup-configuration.enabled" => {
                        request_settings_backup_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().backup_configuration.as_mut().unwrap().enabled = Some(arg_from_str(value.unwrap_or("false"), err, "settings.backup-configuration.enabled", "boolean"));
                    },
                "settings.backup-configuration.start-time" => {
                        request_settings_backup_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().backup_configuration.as_mut().unwrap().start_time = Some(value.unwrap_or("").to_string());
                    },
                "settings.backup-configuration.binary-log-enabled" => {
                        request_settings_backup_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().backup_configuration.as_mut().unwrap().binary_log_enabled = Some(arg_from_str(value.unwrap_or("false"), err, "settings.backup-configuration.binary-log-enabled", "boolean"));
                    },
                "settings.ip-configuration.ipv4-enabled" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().ip_configuration.as_mut().unwrap().ipv4_enabled = Some(arg_from_str(value.unwrap_or("false"), err, "settings.ip-configuration.ipv4-enabled", "boolean"));
                    },
                "settings.ip-configuration.require-ssl" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().ip_configuration.as_mut().unwrap().require_ssl = Some(arg_from_str(value.unwrap_or("false"), err, "settings.ip-configuration.require-ssl", "boolean"));
                    },
                "settings.tier" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().tier = Some(value.unwrap_or("").to_string());
                    },
                "settings.database-replication-enabled" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().database_replication_enabled = Some(arg_from_str(value.unwrap_or("false"), err, "settings.database-replication-enabled", "boolean"));
                    },
                "settings.replication-type" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().replication_type = Some(value.unwrap_or("").to_string());
                    },
                "settings.crash-safe-replication-enabled" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().crash_safe_replication_enabled = Some(arg_from_str(value.unwrap_or("false"), err, "settings.crash-safe-replication-enabled", "boolean"));
                    },
                "settings.pricing-plan" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().pricing_plan = Some(value.unwrap_or("").to_string());
                    },
                "settings.settings-version" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().settings_version = Some(value.unwrap_or("").to_string());
                    },
                "settings.location-preference.kind" => {
                        request_settings_location_preference_init(&mut request);
                        request.settings.as_mut().unwrap().location_preference.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "settings.location-preference.zone" => {
                        request_settings_location_preference_init(&mut request);
                        request.settings.as_mut().unwrap().location_preference.as_mut().unwrap().zone = Some(value.unwrap_or("").to_string());
                    },
                "settings.location-preference.follow-gae-application" => {
                        request_settings_location_preference_init(&mut request);
                        request.settings.as_mut().unwrap().location_preference.as_mut().unwrap().follow_gae_application = Some(value.unwrap_or("").to_string());
                    },
                "master-instance-name" => {
                        request_settings_init(&mut request);
                        request.master_instance_name = Some(value.unwrap_or("").to_string());
                    },
                "current-disk-size" => {
                        request_settings_init(&mut request);
                        request.current_disk_size = Some(value.unwrap_or("").to_string());
                    },
                "state" => {
                        request_settings_init(&mut request);
                        request.state = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_settings_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "service-account-email-address" => {
                        request_settings_init(&mut request);
                        request.service_account_email_address = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.kind" => {
                        request_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.username" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().username = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.kind" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.connect-retry-interval" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().connect_retry_interval = Some(arg_from_str(value.unwrap_or("-0"), err, "replica-configuration.mysql-replica-configuration.connect-retry-interval", "integer"));
                    },
                "replica-configuration.mysql-replica-configuration.ssl-cipher" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().ssl_cipher = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.ca-certificate" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().ca_certificate = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.client-certificate" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().client_certificate = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.master-heartbeat-period" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().master_heartbeat_period = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.verify-server-certificate" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().verify_server_certificate = Some(arg_from_str(value.unwrap_or("false"), err, "replica-configuration.mysql-replica-configuration.verify-server-certificate", "boolean"));
                    },
                "replica-configuration.mysql-replica-configuration.dump-file-path" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().dump_file_path = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.password" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().password = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.client-key" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().client_key = Some(value.unwrap_or("").to_string());
                    },
                "database-version" => {
                        request_replica_configuration_init(&mut request);
                        request.database_version = Some(value.unwrap_or("").to_string());
                    },
                "instance-type" => {
                        request_replica_configuration_init(&mut request);
                        request.instance_type = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_replica_configuration_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_replica_configuration_init(&mut request);
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _instances_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.instances().list(&self.opt.arg_project);
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

    fn _instances_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::DatabaseInstance::default();
        let mut call = self.hub.instances().patch(&request, &self.opt.arg_project, &self.opt.arg_instance);
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
            fn request_on_premises_configuration_init(request: &mut api::DatabaseInstance) {
                if request.on_premises_configuration.is_none() {
                    request.on_premises_configuration = Some(Default::default());
                }
            }
            
            fn request_replica_configuration_init(request: &mut api::DatabaseInstance) {
                if request.replica_configuration.is_none() {
                    request.replica_configuration = Some(Default::default());
                }
            }
            
            fn request_replica_configuration_mysql_replica_configuration_init(request: &mut api::DatabaseInstance) {
                request_replica_configuration_init(request);
                if request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.is_none() {
                    request.replica_configuration.as_mut().unwrap().mysql_replica_configuration = Some(Default::default());
                }
            }
            
            fn request_server_ca_cert_init(request: &mut api::DatabaseInstance) {
                if request.server_ca_cert.is_none() {
                    request.server_ca_cert = Some(Default::default());
                }
            }
            
            fn request_settings_backup_configuration_init(request: &mut api::DatabaseInstance) {
                request_settings_init(request);
                if request.settings.as_mut().unwrap().backup_configuration.is_none() {
                    request.settings.as_mut().unwrap().backup_configuration = Some(Default::default());
                }
            }
            
            fn request_settings_init(request: &mut api::DatabaseInstance) {
                if request.settings.is_none() {
                    request.settings = Some(Default::default());
                }
            }
            
            fn request_settings_ip_configuration_init(request: &mut api::DatabaseInstance) {
                request_settings_init(request);
                if request.settings.as_mut().unwrap().ip_configuration.is_none() {
                    request.settings.as_mut().unwrap().ip_configuration = Some(Default::default());
                }
            }
            
            fn request_settings_location_preference_init(request: &mut api::DatabaseInstance) {
                request_settings_init(request);
                if request.settings.as_mut().unwrap().location_preference.is_none() {
                    request.settings.as_mut().unwrap().location_preference = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "on-premises-configuration.kind" => {
                        request_on_premises_configuration_init(&mut request);
                        request.on_premises_configuration.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "on-premises-configuration.host-port" => {
                        request_on_premises_configuration_init(&mut request);
                        request.on_premises_configuration.as_mut().unwrap().host_port = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_on_premises_configuration_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "max-disk-size" => {
                        request_on_premises_configuration_init(&mut request);
                        request.max_disk_size = Some(value.unwrap_or("").to_string());
                    },
                "ipv6-address" => {
                        request_on_premises_configuration_init(&mut request);
                        request.ipv6_address = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.cert-serial-number" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().cert_serial_number = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.kind" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.sha1-fingerprint" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().sha1_fingerprint = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.common-name" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().common_name = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.instance" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().instance = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.cert" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().cert = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.expiration-time" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().expiration_time = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.create-time" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().create_time = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.self-link" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().self_link = Some(value.unwrap_or("").to_string());
                    },
                "replica-names" => {
                        request_server_ca_cert_init(&mut request);
                        if request.replica_names.is_none() {
                           request.replica_names = Some(Default::default());
                        }
                                        request.replica_names.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "project" => {
                        request_server_ca_cert_init(&mut request);
                        request.project = Some(value.unwrap_or("").to_string());
                    },
                "region" => {
                        request_server_ca_cert_init(&mut request);
                        request.region = Some(value.unwrap_or("").to_string());
                    },
                "settings.kind" => {
                        request_settings_init(&mut request);
                        request.settings.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "settings.authorized-gae-applications" => {
                        request_settings_init(&mut request);
                        if request.settings.as_mut().unwrap().authorized_gae_applications.is_none() {
                           request.settings.as_mut().unwrap().authorized_gae_applications = Some(Default::default());
                        }
                                        request.settings.as_mut().unwrap().authorized_gae_applications.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "settings.activation-policy" => {
                        request_settings_init(&mut request);
                        request.settings.as_mut().unwrap().activation_policy = Some(value.unwrap_or("").to_string());
                    },
                "settings.backup-configuration.kind" => {
                        request_settings_backup_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().backup_configuration.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "settings.backup-configuration.enabled" => {
                        request_settings_backup_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().backup_configuration.as_mut().unwrap().enabled = Some(arg_from_str(value.unwrap_or("false"), err, "settings.backup-configuration.enabled", "boolean"));
                    },
                "settings.backup-configuration.start-time" => {
                        request_settings_backup_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().backup_configuration.as_mut().unwrap().start_time = Some(value.unwrap_or("").to_string());
                    },
                "settings.backup-configuration.binary-log-enabled" => {
                        request_settings_backup_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().backup_configuration.as_mut().unwrap().binary_log_enabled = Some(arg_from_str(value.unwrap_or("false"), err, "settings.backup-configuration.binary-log-enabled", "boolean"));
                    },
                "settings.ip-configuration.ipv4-enabled" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().ip_configuration.as_mut().unwrap().ipv4_enabled = Some(arg_from_str(value.unwrap_or("false"), err, "settings.ip-configuration.ipv4-enabled", "boolean"));
                    },
                "settings.ip-configuration.require-ssl" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().ip_configuration.as_mut().unwrap().require_ssl = Some(arg_from_str(value.unwrap_or("false"), err, "settings.ip-configuration.require-ssl", "boolean"));
                    },
                "settings.tier" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().tier = Some(value.unwrap_or("").to_string());
                    },
                "settings.database-replication-enabled" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().database_replication_enabled = Some(arg_from_str(value.unwrap_or("false"), err, "settings.database-replication-enabled", "boolean"));
                    },
                "settings.replication-type" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().replication_type = Some(value.unwrap_or("").to_string());
                    },
                "settings.crash-safe-replication-enabled" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().crash_safe_replication_enabled = Some(arg_from_str(value.unwrap_or("false"), err, "settings.crash-safe-replication-enabled", "boolean"));
                    },
                "settings.pricing-plan" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().pricing_plan = Some(value.unwrap_or("").to_string());
                    },
                "settings.settings-version" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().settings_version = Some(value.unwrap_or("").to_string());
                    },
                "settings.location-preference.kind" => {
                        request_settings_location_preference_init(&mut request);
                        request.settings.as_mut().unwrap().location_preference.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "settings.location-preference.zone" => {
                        request_settings_location_preference_init(&mut request);
                        request.settings.as_mut().unwrap().location_preference.as_mut().unwrap().zone = Some(value.unwrap_or("").to_string());
                    },
                "settings.location-preference.follow-gae-application" => {
                        request_settings_location_preference_init(&mut request);
                        request.settings.as_mut().unwrap().location_preference.as_mut().unwrap().follow_gae_application = Some(value.unwrap_or("").to_string());
                    },
                "master-instance-name" => {
                        request_settings_init(&mut request);
                        request.master_instance_name = Some(value.unwrap_or("").to_string());
                    },
                "current-disk-size" => {
                        request_settings_init(&mut request);
                        request.current_disk_size = Some(value.unwrap_or("").to_string());
                    },
                "state" => {
                        request_settings_init(&mut request);
                        request.state = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_settings_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "service-account-email-address" => {
                        request_settings_init(&mut request);
                        request.service_account_email_address = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.kind" => {
                        request_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.username" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().username = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.kind" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.connect-retry-interval" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().connect_retry_interval = Some(arg_from_str(value.unwrap_or("-0"), err, "replica-configuration.mysql-replica-configuration.connect-retry-interval", "integer"));
                    },
                "replica-configuration.mysql-replica-configuration.ssl-cipher" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().ssl_cipher = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.ca-certificate" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().ca_certificate = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.client-certificate" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().client_certificate = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.master-heartbeat-period" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().master_heartbeat_period = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.verify-server-certificate" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().verify_server_certificate = Some(arg_from_str(value.unwrap_or("false"), err, "replica-configuration.mysql-replica-configuration.verify-server-certificate", "boolean"));
                    },
                "replica-configuration.mysql-replica-configuration.dump-file-path" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().dump_file_path = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.password" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().password = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.client-key" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().client_key = Some(value.unwrap_or("").to_string());
                    },
                "database-version" => {
                        request_replica_configuration_init(&mut request);
                        request.database_version = Some(value.unwrap_or("").to_string());
                    },
                "instance-type" => {
                        request_replica_configuration_init(&mut request);
                        request.instance_type = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_replica_configuration_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_replica_configuration_init(&mut request);
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _instances_promote_replica(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.instances().promote_replica(&self.opt.arg_project, &self.opt.arg_instance);
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

    fn _instances_reset_ssl_config(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.instances().reset_ssl_config(&self.opt.arg_project, &self.opt.arg_instance);
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

    fn _instances_restart(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.instances().restart(&self.opt.arg_project, &self.opt.arg_instance);
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

    fn _instances_restore_backup(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::InstancesRestoreBackupRequest::default();
        let mut call = self.hub.instances().restore_backup(&request, &self.opt.arg_project, &self.opt.arg_instance);
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
            fn request_restore_backup_context_init(request: &mut api::InstancesRestoreBackupRequest) {
                if request.restore_backup_context.is_none() {
                    request.restore_backup_context = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "restore-backup-context.kind" => {
                        request_restore_backup_context_init(&mut request);
                        request.restore_backup_context.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "restore-backup-context.backup-run-id" => {
                        request_restore_backup_context_init(&mut request);
                        request.restore_backup_context.as_mut().unwrap().backup_run_id = Some(value.unwrap_or("").to_string());
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

    fn _instances_start_replica(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.instances().start_replica(&self.opt.arg_project, &self.opt.arg_instance);
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

    fn _instances_stop_replica(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.instances().stop_replica(&self.opt.arg_project, &self.opt.arg_instance);
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

    fn _instances_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::DatabaseInstance::default();
        let mut call = self.hub.instances().update(&request, &self.opt.arg_project, &self.opt.arg_instance);
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
            fn request_on_premises_configuration_init(request: &mut api::DatabaseInstance) {
                if request.on_premises_configuration.is_none() {
                    request.on_premises_configuration = Some(Default::default());
                }
            }
            
            fn request_replica_configuration_init(request: &mut api::DatabaseInstance) {
                if request.replica_configuration.is_none() {
                    request.replica_configuration = Some(Default::default());
                }
            }
            
            fn request_replica_configuration_mysql_replica_configuration_init(request: &mut api::DatabaseInstance) {
                request_replica_configuration_init(request);
                if request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.is_none() {
                    request.replica_configuration.as_mut().unwrap().mysql_replica_configuration = Some(Default::default());
                }
            }
            
            fn request_server_ca_cert_init(request: &mut api::DatabaseInstance) {
                if request.server_ca_cert.is_none() {
                    request.server_ca_cert = Some(Default::default());
                }
            }
            
            fn request_settings_backup_configuration_init(request: &mut api::DatabaseInstance) {
                request_settings_init(request);
                if request.settings.as_mut().unwrap().backup_configuration.is_none() {
                    request.settings.as_mut().unwrap().backup_configuration = Some(Default::default());
                }
            }
            
            fn request_settings_init(request: &mut api::DatabaseInstance) {
                if request.settings.is_none() {
                    request.settings = Some(Default::default());
                }
            }
            
            fn request_settings_ip_configuration_init(request: &mut api::DatabaseInstance) {
                request_settings_init(request);
                if request.settings.as_mut().unwrap().ip_configuration.is_none() {
                    request.settings.as_mut().unwrap().ip_configuration = Some(Default::default());
                }
            }
            
            fn request_settings_location_preference_init(request: &mut api::DatabaseInstance) {
                request_settings_init(request);
                if request.settings.as_mut().unwrap().location_preference.is_none() {
                    request.settings.as_mut().unwrap().location_preference = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "on-premises-configuration.kind" => {
                        request_on_premises_configuration_init(&mut request);
                        request.on_premises_configuration.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "on-premises-configuration.host-port" => {
                        request_on_premises_configuration_init(&mut request);
                        request.on_premises_configuration.as_mut().unwrap().host_port = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_on_premises_configuration_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "max-disk-size" => {
                        request_on_premises_configuration_init(&mut request);
                        request.max_disk_size = Some(value.unwrap_or("").to_string());
                    },
                "ipv6-address" => {
                        request_on_premises_configuration_init(&mut request);
                        request.ipv6_address = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.cert-serial-number" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().cert_serial_number = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.kind" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.sha1-fingerprint" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().sha1_fingerprint = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.common-name" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().common_name = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.instance" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().instance = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.cert" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().cert = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.expiration-time" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().expiration_time = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.create-time" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().create_time = Some(value.unwrap_or("").to_string());
                    },
                "server-ca-cert.self-link" => {
                        request_server_ca_cert_init(&mut request);
                        request.server_ca_cert.as_mut().unwrap().self_link = Some(value.unwrap_or("").to_string());
                    },
                "replica-names" => {
                        request_server_ca_cert_init(&mut request);
                        if request.replica_names.is_none() {
                           request.replica_names = Some(Default::default());
                        }
                                        request.replica_names.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "project" => {
                        request_server_ca_cert_init(&mut request);
                        request.project = Some(value.unwrap_or("").to_string());
                    },
                "region" => {
                        request_server_ca_cert_init(&mut request);
                        request.region = Some(value.unwrap_or("").to_string());
                    },
                "settings.kind" => {
                        request_settings_init(&mut request);
                        request.settings.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "settings.authorized-gae-applications" => {
                        request_settings_init(&mut request);
                        if request.settings.as_mut().unwrap().authorized_gae_applications.is_none() {
                           request.settings.as_mut().unwrap().authorized_gae_applications = Some(Default::default());
                        }
                                        request.settings.as_mut().unwrap().authorized_gae_applications.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "settings.activation-policy" => {
                        request_settings_init(&mut request);
                        request.settings.as_mut().unwrap().activation_policy = Some(value.unwrap_or("").to_string());
                    },
                "settings.backup-configuration.kind" => {
                        request_settings_backup_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().backup_configuration.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "settings.backup-configuration.enabled" => {
                        request_settings_backup_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().backup_configuration.as_mut().unwrap().enabled = Some(arg_from_str(value.unwrap_or("false"), err, "settings.backup-configuration.enabled", "boolean"));
                    },
                "settings.backup-configuration.start-time" => {
                        request_settings_backup_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().backup_configuration.as_mut().unwrap().start_time = Some(value.unwrap_or("").to_string());
                    },
                "settings.backup-configuration.binary-log-enabled" => {
                        request_settings_backup_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().backup_configuration.as_mut().unwrap().binary_log_enabled = Some(arg_from_str(value.unwrap_or("false"), err, "settings.backup-configuration.binary-log-enabled", "boolean"));
                    },
                "settings.ip-configuration.ipv4-enabled" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().ip_configuration.as_mut().unwrap().ipv4_enabled = Some(arg_from_str(value.unwrap_or("false"), err, "settings.ip-configuration.ipv4-enabled", "boolean"));
                    },
                "settings.ip-configuration.require-ssl" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().ip_configuration.as_mut().unwrap().require_ssl = Some(arg_from_str(value.unwrap_or("false"), err, "settings.ip-configuration.require-ssl", "boolean"));
                    },
                "settings.tier" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().tier = Some(value.unwrap_or("").to_string());
                    },
                "settings.database-replication-enabled" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().database_replication_enabled = Some(arg_from_str(value.unwrap_or("false"), err, "settings.database-replication-enabled", "boolean"));
                    },
                "settings.replication-type" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().replication_type = Some(value.unwrap_or("").to_string());
                    },
                "settings.crash-safe-replication-enabled" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().crash_safe_replication_enabled = Some(arg_from_str(value.unwrap_or("false"), err, "settings.crash-safe-replication-enabled", "boolean"));
                    },
                "settings.pricing-plan" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().pricing_plan = Some(value.unwrap_or("").to_string());
                    },
                "settings.settings-version" => {
                        request_settings_ip_configuration_init(&mut request);
                        request.settings.as_mut().unwrap().settings_version = Some(value.unwrap_or("").to_string());
                    },
                "settings.location-preference.kind" => {
                        request_settings_location_preference_init(&mut request);
                        request.settings.as_mut().unwrap().location_preference.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "settings.location-preference.zone" => {
                        request_settings_location_preference_init(&mut request);
                        request.settings.as_mut().unwrap().location_preference.as_mut().unwrap().zone = Some(value.unwrap_or("").to_string());
                    },
                "settings.location-preference.follow-gae-application" => {
                        request_settings_location_preference_init(&mut request);
                        request.settings.as_mut().unwrap().location_preference.as_mut().unwrap().follow_gae_application = Some(value.unwrap_or("").to_string());
                    },
                "master-instance-name" => {
                        request_settings_init(&mut request);
                        request.master_instance_name = Some(value.unwrap_or("").to_string());
                    },
                "current-disk-size" => {
                        request_settings_init(&mut request);
                        request.current_disk_size = Some(value.unwrap_or("").to_string());
                    },
                "state" => {
                        request_settings_init(&mut request);
                        request.state = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_settings_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "service-account-email-address" => {
                        request_settings_init(&mut request);
                        request.service_account_email_address = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.kind" => {
                        request_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.username" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().username = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.kind" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.connect-retry-interval" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().connect_retry_interval = Some(arg_from_str(value.unwrap_or("-0"), err, "replica-configuration.mysql-replica-configuration.connect-retry-interval", "integer"));
                    },
                "replica-configuration.mysql-replica-configuration.ssl-cipher" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().ssl_cipher = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.ca-certificate" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().ca_certificate = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.client-certificate" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().client_certificate = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.master-heartbeat-period" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().master_heartbeat_period = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.verify-server-certificate" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().verify_server_certificate = Some(arg_from_str(value.unwrap_or("false"), err, "replica-configuration.mysql-replica-configuration.verify-server-certificate", "boolean"));
                    },
                "replica-configuration.mysql-replica-configuration.dump-file-path" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().dump_file_path = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.password" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().password = Some(value.unwrap_or("").to_string());
                    },
                "replica-configuration.mysql-replica-configuration.client-key" => {
                        request_replica_configuration_mysql_replica_configuration_init(&mut request);
                        request.replica_configuration.as_mut().unwrap().mysql_replica_configuration.as_mut().unwrap().client_key = Some(value.unwrap_or("").to_string());
                    },
                "database-version" => {
                        request_replica_configuration_init(&mut request);
                        request.database_version = Some(value.unwrap_or("").to_string());
                    },
                "instance-type" => {
                        request_replica_configuration_init(&mut request);
                        request.instance_type = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_replica_configuration_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_replica_configuration_init(&mut request);
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _operations_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.operations().get(&self.opt.arg_project, &self.opt.arg_operation);
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

    fn _operations_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.operations().list(&self.opt.arg_project, &self.opt.arg_instance);
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

    fn _ssl_certs_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.ssl_certs().delete(&self.opt.arg_project, &self.opt.arg_instance, &self.opt.arg_sha1_fingerprint);
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

    fn _ssl_certs_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.ssl_certs().get(&self.opt.arg_project, &self.opt.arg_instance, &self.opt.arg_sha1_fingerprint);
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

    fn _ssl_certs_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::SslCertsInsertRequest::default();
        let mut call = self.hub.ssl_certs().insert(&request, &self.opt.arg_project, &self.opt.arg_instance);
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
                "common-name" => {
                        request.common_name = Some(value.unwrap_or("").to_string());
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

    fn _ssl_certs_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.ssl_certs().list(&self.opt.arg_project, &self.opt.arg_instance);
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

    fn _tiers_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.tiers().list(&self.opt.arg_project);
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

    fn _users_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.users().delete(&self.opt.arg_project, &self.opt.arg_instance, &self.opt.arg_host, &self.opt.arg_name);
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

    fn _users_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::User::default();
        let mut call = self.hub.users().insert(&request, &self.opt.arg_project, &self.opt.arg_instance);
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
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "project" => {
                        request.project = Some(value.unwrap_or("").to_string());
                    },
                "instance" => {
                        request.instance = Some(value.unwrap_or("").to_string());
                    },
                "host" => {
                        request.host = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "password" => {
                        request.password = Some(value.unwrap_or("").to_string());
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

    fn _users_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.users().list(&self.opt.arg_project, &self.opt.arg_instance);
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

    fn _users_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::User::default();
        let mut call = self.hub.users().update(&request, &self.opt.arg_project, &self.opt.arg_instance, &self.opt.arg_host, &self.opt.arg_name);
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
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "project" => {
                        request.project = Some(value.unwrap_or("").to_string());
                    },
                "instance" => {
                        request.instance = Some(value.unwrap_or("").to_string());
                    },
                "host" => {
                        request.host = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "password" => {
                        request.password = Some(value.unwrap_or("").to_string());
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

        if self.opt.cmd_backup_runs {
            if self.opt.cmd_get {
                call_result = self._backup_runs_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._backup_runs_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_databases {
            if self.opt.cmd_delete {
                call_result = self._databases_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._databases_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._databases_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._databases_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._databases_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._databases_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_flags {
            if self.opt.cmd_list {
                call_result = self._flags_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_instances {
            if self.opt.cmd_clone {
                call_result = self._instances_clone(dry_run, &mut err);
            } else if self.opt.cmd_delete {
                call_result = self._instances_delete(dry_run, &mut err);
            } else if self.opt.cmd_export {
                call_result = self._instances_export(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._instances_get(dry_run, &mut err);
            } else if self.opt.cmd_import {
                call_result = self._instances_import(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._instances_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._instances_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._instances_patch(dry_run, &mut err);
            } else if self.opt.cmd_promote_replica {
                call_result = self._instances_promote_replica(dry_run, &mut err);
            } else if self.opt.cmd_reset_ssl_config {
                call_result = self._instances_reset_ssl_config(dry_run, &mut err);
            } else if self.opt.cmd_restart {
                call_result = self._instances_restart(dry_run, &mut err);
            } else if self.opt.cmd_restore_backup {
                call_result = self._instances_restore_backup(dry_run, &mut err);
            } else if self.opt.cmd_start_replica {
                call_result = self._instances_start_replica(dry_run, &mut err);
            } else if self.opt.cmd_stop_replica {
                call_result = self._instances_stop_replica(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._instances_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_operations {
            if self.opt.cmd_get {
                call_result = self._operations_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._operations_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_ssl_certs {
            if self.opt.cmd_delete {
                call_result = self._ssl_certs_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._ssl_certs_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._ssl_certs_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._ssl_certs_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_tiers {
            if self.opt.cmd_list {
                call_result = self._tiers_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_users {
            if self.opt.cmd_delete {
                call_result = self._users_delete(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._users_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._users_list(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._users_update(dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "sqladmin1-beta4-secret.json", 
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
                                          program_name: "sqladmin1-beta4",
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
            hub: api::SQLAdmin::new(client, auth),
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