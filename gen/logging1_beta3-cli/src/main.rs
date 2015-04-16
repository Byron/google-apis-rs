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
extern crate google_logging1_beta3 as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  logging1-beta3 [options] projects log-services-indexes-list <projects-id> <log-services-id> [-p <v>]... [-o <out>]
  logging1-beta3 [options] projects log-services-list <projects-id> [-p <v>]... [-o <out>]
  logging1-beta3 [options] projects log-services-sinks-create <projects-id> <log-services-id> -r <kv>... [-p <v>]... [-o <out>]
  logging1-beta3 [options] projects log-services-sinks-delete <projects-id> <log-services-id> <sinks-id> [-p <v>]... [-o <out>]
  logging1-beta3 [options] projects log-services-sinks-get <projects-id> <log-services-id> <sinks-id> [-p <v>]... [-o <out>]
  logging1-beta3 [options] projects log-services-sinks-list <projects-id> <log-services-id> [-p <v>]... [-o <out>]
  logging1-beta3 [options] projects log-services-sinks-update <projects-id> <log-services-id> <sinks-id> -r <kv>... [-p <v>]... [-o <out>]
  logging1-beta3 [options] projects logs-delete <projects-id> <logs-id> [-p <v>]... [-o <out>]
  logging1-beta3 [options] projects logs-entries-write <projects-id> <logs-id> -r <kv>... [-p <v>]... [-o <out>]
  logging1-beta3 [options] projects logs-list <projects-id> [-p <v>]... [-o <out>]
  logging1-beta3 [options] projects logs-sinks-create <projects-id> <logs-id> -r <kv>... [-p <v>]... [-o <out>]
  logging1-beta3 [options] projects logs-sinks-delete <projects-id> <logs-id> <sinks-id> [-p <v>]... [-o <out>]
  logging1-beta3 [options] projects logs-sinks-get <projects-id> <logs-id> <sinks-id> [-p <v>]... [-o <out>]
  logging1-beta3 [options] projects logs-sinks-list <projects-id> <logs-id> [-p <v>]... [-o <out>]
  logging1-beta3 [options] projects logs-sinks-update <projects-id> <logs-id> <sinks-id> -r <kv>... [-p <v>]... [-o <out>]
  logging1-beta3 --help

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
    hub: api::Logging<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _projects_log_services_indexes_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.projects().log_services_indexes_list(&self.opt.arg_projects_id, &self.opt.arg_log_services_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
                },
                "log" => {
                    call = call.log(value.unwrap_or(""));
                },
                "index-prefix" => {
                    call = call.index_prefix(value.unwrap_or(""));
                },
                "depth" => {
                    call = call.depth(arg_from_str(value.unwrap_or("-0"), err, "depth", "integer"));
                },
                "$-xgafv"
                |"access-token"
                |"alt"
                |"bearer-token"
                |"callback"
                |"fields"
                |"key"
                |"oauth-token"
                |"pp"
                |"pretty-print"
                |"quota-user" => {
                    let map = [
                        ("$-xgafv", "$.xgafv"),
                        ("access-token", "access_token"),
                        ("bearer-token", "bearer_token"),
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
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

    fn _projects_log_services_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.projects().log_services_list(&self.opt.arg_projects_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
                },
                "log" => {
                    call = call.log(value.unwrap_or(""));
                },
                "$-xgafv"
                |"access-token"
                |"alt"
                |"bearer-token"
                |"callback"
                |"fields"
                |"key"
                |"oauth-token"
                |"pp"
                |"pretty-print"
                |"quota-user" => {
                    let map = [
                        ("$-xgafv", "$.xgafv"),
                        ("access-token", "access_token"),
                        ("bearer-token", "bearer_token"),
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
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

    fn _projects_log_services_sinks_create(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::LogSink = Default::default();
        let mut call = self.hub.projects().log_services_sinks_create(&request, &self.opt.arg_projects_id, &self.opt.arg_log_services_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "$-xgafv"
                |"access-token"
                |"alt"
                |"bearer-token"
                |"callback"
                |"fields"
                |"key"
                |"oauth-token"
                |"pp"
                |"pretty-print"
                |"quota-user" => {
                    let map = [
                        ("$-xgafv", "$.xgafv"),
                        ("access-token", "access_token"),
                        ("bearer-token", "bearer_token"),
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
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
                "destination" => {
                        request.destination = Some(value.unwrap_or("").to_string());
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

    fn _projects_log_services_sinks_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.projects().log_services_sinks_delete(&self.opt.arg_projects_id, &self.opt.arg_log_services_id, &self.opt.arg_sinks_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "$-xgafv"
                |"access-token"
                |"alt"
                |"bearer-token"
                |"callback"
                |"fields"
                |"key"
                |"oauth-token"
                |"pp"
                |"pretty-print"
                |"quota-user" => {
                    let map = [
                        ("$-xgafv", "$.xgafv"),
                        ("access-token", "access_token"),
                        ("bearer-token", "bearer_token"),
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
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

    fn _projects_log_services_sinks_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.projects().log_services_sinks_get(&self.opt.arg_projects_id, &self.opt.arg_log_services_id, &self.opt.arg_sinks_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "$-xgafv"
                |"access-token"
                |"alt"
                |"bearer-token"
                |"callback"
                |"fields"
                |"key"
                |"oauth-token"
                |"pp"
                |"pretty-print"
                |"quota-user" => {
                    let map = [
                        ("$-xgafv", "$.xgafv"),
                        ("access-token", "access_token"),
                        ("bearer-token", "bearer_token"),
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
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

    fn _projects_log_services_sinks_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.projects().log_services_sinks_list(&self.opt.arg_projects_id, &self.opt.arg_log_services_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "$-xgafv"
                |"access-token"
                |"alt"
                |"bearer-token"
                |"callback"
                |"fields"
                |"key"
                |"oauth-token"
                |"pp"
                |"pretty-print"
                |"quota-user" => {
                    let map = [
                        ("$-xgafv", "$.xgafv"),
                        ("access-token", "access_token"),
                        ("bearer-token", "bearer_token"),
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
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

    fn _projects_log_services_sinks_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::LogSink = Default::default();
        let mut call = self.hub.projects().log_services_sinks_update(&request, &self.opt.arg_projects_id, &self.opt.arg_log_services_id, &self.opt.arg_sinks_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "$-xgafv"
                |"access-token"
                |"alt"
                |"bearer-token"
                |"callback"
                |"fields"
                |"key"
                |"oauth-token"
                |"pp"
                |"pretty-print"
                |"quota-user" => {
                    let map = [
                        ("$-xgafv", "$.xgafv"),
                        ("access-token", "access_token"),
                        ("bearer-token", "bearer_token"),
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
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
                "destination" => {
                        request.destination = Some(value.unwrap_or("").to_string());
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

    fn _projects_logs_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.projects().logs_delete(&self.opt.arg_projects_id, &self.opt.arg_logs_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "$-xgafv"
                |"access-token"
                |"alt"
                |"bearer-token"
                |"callback"
                |"fields"
                |"key"
                |"oauth-token"
                |"pp"
                |"pretty-print"
                |"quota-user" => {
                    let map = [
                        ("$-xgafv", "$.xgafv"),
                        ("access-token", "access_token"),
                        ("bearer-token", "bearer_token"),
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
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

    fn _projects_logs_entries_write(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::WriteLogEntriesRequest = Default::default();
        let mut call = self.hub.projects().logs_entries_write(&request, &self.opt.arg_projects_id, &self.opt.arg_logs_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "$-xgafv"
                |"access-token"
                |"alt"
                |"bearer-token"
                |"callback"
                |"fields"
                |"key"
                |"oauth-token"
                |"pp"
                |"pretty-print"
                |"quota-user" => {
                    let map = [
                        ("$-xgafv", "$.xgafv"),
                        ("access-token", "access_token"),
                        ("bearer-token", "bearer_token"),
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
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
                "common-labels" => {
                        if request.common_labels.is_none() {
                            request.common_labels = Some(Default::default());
                        }
                        request.common_labels.as_mut().unwrap().push(value.unwrap_or("").to_string());
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

    fn _projects_logs_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.projects().logs_list(&self.opt.arg_projects_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "service-name" => {
                    call = call.service_name(value.unwrap_or(""));
                },
                "service-index-prefix" => {
                    call = call.service_index_prefix(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
                },
                "$-xgafv"
                |"access-token"
                |"alt"
                |"bearer-token"
                |"callback"
                |"fields"
                |"key"
                |"oauth-token"
                |"pp"
                |"pretty-print"
                |"quota-user" => {
                    let map = [
                        ("$-xgafv", "$.xgafv"),
                        ("access-token", "access_token"),
                        ("bearer-token", "bearer_token"),
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
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

    fn _projects_logs_sinks_create(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::LogSink = Default::default();
        let mut call = self.hub.projects().logs_sinks_create(&request, &self.opt.arg_projects_id, &self.opt.arg_logs_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "$-xgafv"
                |"access-token"
                |"alt"
                |"bearer-token"
                |"callback"
                |"fields"
                |"key"
                |"oauth-token"
                |"pp"
                |"pretty-print"
                |"quota-user" => {
                    let map = [
                        ("$-xgafv", "$.xgafv"),
                        ("access-token", "access_token"),
                        ("bearer-token", "bearer_token"),
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
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
                "destination" => {
                        request.destination = Some(value.unwrap_or("").to_string());
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

    fn _projects_logs_sinks_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.projects().logs_sinks_delete(&self.opt.arg_projects_id, &self.opt.arg_logs_id, &self.opt.arg_sinks_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "$-xgafv"
                |"access-token"
                |"alt"
                |"bearer-token"
                |"callback"
                |"fields"
                |"key"
                |"oauth-token"
                |"pp"
                |"pretty-print"
                |"quota-user" => {
                    let map = [
                        ("$-xgafv", "$.xgafv"),
                        ("access-token", "access_token"),
                        ("bearer-token", "bearer_token"),
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
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

    fn _projects_logs_sinks_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.projects().logs_sinks_get(&self.opt.arg_projects_id, &self.opt.arg_logs_id, &self.opt.arg_sinks_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "$-xgafv"
                |"access-token"
                |"alt"
                |"bearer-token"
                |"callback"
                |"fields"
                |"key"
                |"oauth-token"
                |"pp"
                |"pretty-print"
                |"quota-user" => {
                    let map = [
                        ("$-xgafv", "$.xgafv"),
                        ("access-token", "access_token"),
                        ("bearer-token", "bearer_token"),
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
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

    fn _projects_logs_sinks_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.projects().logs_sinks_list(&self.opt.arg_projects_id, &self.opt.arg_logs_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "$-xgafv"
                |"access-token"
                |"alt"
                |"bearer-token"
                |"callback"
                |"fields"
                |"key"
                |"oauth-token"
                |"pp"
                |"pretty-print"
                |"quota-user" => {
                    let map = [
                        ("$-xgafv", "$.xgafv"),
                        ("access-token", "access_token"),
                        ("bearer-token", "bearer_token"),
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
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

    fn _projects_logs_sinks_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::LogSink = Default::default();
        let mut call = self.hub.projects().logs_sinks_update(&request, &self.opt.arg_projects_id, &self.opt.arg_logs_id, &self.opt.arg_sinks_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "$-xgafv"
                |"access-token"
                |"alt"
                |"bearer-token"
                |"callback"
                |"fields"
                |"key"
                |"oauth-token"
                |"pp"
                |"pretty-print"
                |"quota-user" => {
                    let map = [
                        ("$-xgafv", "$.xgafv"),
                        ("access-token", "access_token"),
                        ("bearer-token", "bearer_token"),
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
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
                "destination" => {
                        request.destination = Some(value.unwrap_or("").to_string());
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

    fn _doit(&self, dry_run: bool) -> (Option<api::Error>, Option<InvalidOptionsError>) {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Option<api::Error>;
        let mut err_opt: Option<InvalidOptionsError> = None;

        if self.opt.cmd_projects {
            if self.opt.cmd_log_services_indexes_list {
                call_result = self._projects_log_services_indexes_list(dry_run, &mut err);
            } else if self.opt.cmd_log_services_list {
                call_result = self._projects_log_services_list(dry_run, &mut err);
            } else if self.opt.cmd_log_services_sinks_create {
                call_result = self._projects_log_services_sinks_create(dry_run, &mut err);
            } else if self.opt.cmd_log_services_sinks_delete {
                call_result = self._projects_log_services_sinks_delete(dry_run, &mut err);
            } else if self.opt.cmd_log_services_sinks_get {
                call_result = self._projects_log_services_sinks_get(dry_run, &mut err);
            } else if self.opt.cmd_log_services_sinks_list {
                call_result = self._projects_log_services_sinks_list(dry_run, &mut err);
            } else if self.opt.cmd_log_services_sinks_update {
                call_result = self._projects_log_services_sinks_update(dry_run, &mut err);
            } else if self.opt.cmd_logs_delete {
                call_result = self._projects_logs_delete(dry_run, &mut err);
            } else if self.opt.cmd_logs_entries_write {
                call_result = self._projects_logs_entries_write(dry_run, &mut err);
            } else if self.opt.cmd_logs_list {
                call_result = self._projects_logs_list(dry_run, &mut err);
            } else if self.opt.cmd_logs_sinks_create {
                call_result = self._projects_logs_sinks_create(dry_run, &mut err);
            } else if self.opt.cmd_logs_sinks_delete {
                call_result = self._projects_logs_sinks_delete(dry_run, &mut err);
            } else if self.opt.cmd_logs_sinks_get {
                call_result = self._projects_logs_sinks_get(dry_run, &mut err);
            } else if self.opt.cmd_logs_sinks_list {
                call_result = self._projects_logs_sinks_list(dry_run, &mut err);
            } else if self.opt.cmd_logs_sinks_update {
                call_result = self._projects_logs_sinks_update(dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "logging1-beta3-secret.json") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                                      hyper::Client::new(),
                                      JsonTokenStorage {
                                        program_name: "logging1-beta3",
                                        db_dir: config_dir.clone(),
                                      }, None);
        let engine = Engine {
            opt: opt,
            hub: api::Logging::new(hyper::Client::new(), auth),
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