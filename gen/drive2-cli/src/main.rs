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
extern crate google_drive2 as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  drive2 [options] about get [-p <v>...] [-o <out>]
  drive2 [options] apps get <app-id> [-p <v>...] [-o <out>]
  drive2 [options] apps list [-p <v>...] [-o <out>]
  drive2 [options] changes get <change-id> [-p <v>...] [-o <out>]
  drive2 [options] changes list [-p <v>...] [-o <out>]
  drive2 [options] changes watch -r <kv>... [-p <v>...] [-o <out>]
  drive2 [options] channels stop -r <kv>... [-p <v>...]
  drive2 [options] children delete <folder-id> <child-id> [-p <v>...]
  drive2 [options] children get <folder-id> <child-id> [-p <v>...] [-o <out>]
  drive2 [options] children insert <folder-id> -r <kv>... [-p <v>...] [-o <out>]
  drive2 [options] children list <folder-id> [-p <v>...] [-o <out>]
  drive2 [options] comments delete <file-id> <comment-id> [-p <v>...]
  drive2 [options] comments get <file-id> <comment-id> [-p <v>...] [-o <out>]
  drive2 [options] comments insert <file-id> -r <kv>... [-p <v>...] [-o <out>]
  drive2 [options] comments list <file-id> [-p <v>...] [-o <out>]
  drive2 [options] comments patch <file-id> <comment-id> -r <kv>... [-p <v>...] [-o <out>]
  drive2 [options] comments update <file-id> <comment-id> -r <kv>... [-p <v>...] [-o <out>]
  drive2 [options] files copy <file-id> -r <kv>... [-p <v>...] [-o <out>]
  drive2 [options] files delete <file-id> [-p <v>...]
  drive2 [options] files empty-trash [-p <v>...]
  drive2 [options] files get <file-id> [-p <v>...] [-o <out>]
  drive2 [options] files insert -r <kv>... -u (simple|resumable) <file> <mime> [-p <v>...] [-o <out>]
  drive2 [options] files list [-p <v>...] [-o <out>]
  drive2 [options] files patch <file-id> -r <kv>... [-p <v>...] [-o <out>]
  drive2 [options] files touch <file-id> [-p <v>...] [-o <out>]
  drive2 [options] files trash <file-id> [-p <v>...] [-o <out>]
  drive2 [options] files untrash <file-id> [-p <v>...] [-o <out>]
  drive2 [options] files update <file-id> -r <kv>... -u (simple|resumable) <file> <mime> [-p <v>...] [-o <out>]
  drive2 [options] files watch <file-id> -r <kv>... [-p <v>...] [-o <out>]
  drive2 [options] parents delete <file-id> <parent-id> [-p <v>...]
  drive2 [options] parents get <file-id> <parent-id> [-p <v>...] [-o <out>]
  drive2 [options] parents insert <file-id> -r <kv>... [-p <v>...] [-o <out>]
  drive2 [options] parents list <file-id> [-p <v>...] [-o <out>]
  drive2 [options] permissions delete <file-id> <permission-id> [-p <v>...]
  drive2 [options] permissions get <file-id> <permission-id> [-p <v>...] [-o <out>]
  drive2 [options] permissions get-id-for-email <email> [-p <v>...] [-o <out>]
  drive2 [options] permissions insert <file-id> -r <kv>... [-p <v>...] [-o <out>]
  drive2 [options] permissions list <file-id> [-p <v>...] [-o <out>]
  drive2 [options] permissions patch <file-id> <permission-id> -r <kv>... [-p <v>...] [-o <out>]
  drive2 [options] permissions update <file-id> <permission-id> -r <kv>... [-p <v>...] [-o <out>]
  drive2 [options] properties delete <file-id> <property-key> [-p <v>...]
  drive2 [options] properties get <file-id> <property-key> [-p <v>...] [-o <out>]
  drive2 [options] properties insert <file-id> -r <kv>... [-p <v>...] [-o <out>]
  drive2 [options] properties list <file-id> [-p <v>...] [-o <out>]
  drive2 [options] properties patch <file-id> <property-key> -r <kv>... [-p <v>...] [-o <out>]
  drive2 [options] properties update <file-id> <property-key> -r <kv>... [-p <v>...] [-o <out>]
  drive2 [options] realtime get <file-id> [-p <v>...] [-o <out>]
  drive2 [options] realtime update <file-id> -u (simple|resumable) <file> <mime> [-p <v>...]
  drive2 [options] replies delete <file-id> <comment-id> <reply-id> [-p <v>...]
  drive2 [options] replies get <file-id> <comment-id> <reply-id> [-p <v>...] [-o <out>]
  drive2 [options] replies insert <file-id> <comment-id> -r <kv>... [-p <v>...] [-o <out>]
  drive2 [options] replies list <file-id> <comment-id> [-p <v>...] [-o <out>]
  drive2 [options] replies patch <file-id> <comment-id> <reply-id> -r <kv>... [-p <v>...] [-o <out>]
  drive2 [options] replies update <file-id> <comment-id> <reply-id> -r <kv>... [-p <v>...] [-o <out>]
  drive2 [options] revisions delete <file-id> <revision-id> [-p <v>...]
  drive2 [options] revisions get <file-id> <revision-id> [-p <v>...] [-o <out>]
  drive2 [options] revisions list <file-id> [-p <v>...] [-o <out>]
  drive2 [options] revisions patch <file-id> <revision-id> -r <kv>... [-p <v>...] [-o <out>]
  drive2 [options] revisions update <file-id> <revision-id> -r <kv>... [-p <v>...] [-o <out>]
  drive2 --help

All documentation details can be found at
http://byron.github.io/google-apis-rs/google_drive2_cli/index.html

Configuration:
  --scope <url>  
            Specify the authentication a method should be executed in. Each scope 
            requires the user to grant this application permission to use it.
            If unset, it defaults to the shortest scope url for a particular method.
  --config-dir <folder>
            A directory into which we will store our persistent data. Defaults to 
            a user-writable directory that we will create during the first invocation.
            [default: ~/.google-service-cli]
  --debug
            Output all server communication to standard error. `tx` and `rx` are placed 
            into the same stream.
  --debug-auth
            Output all communication related to authentication to standard error. `tx` 
            and `rx` are placed into the same stream.
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
    hub: api::Drive<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _about_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.about().get();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-change-id" => {
                    call = call.start_change_id(value.unwrap_or(""));
                },
                "max-change-id-count" => {
                    call = call.max_change_id_count(arg_from_str(value.unwrap_or("-0"), err, "max-change-id-count", "int64"));
                },
                "include-subscribed" => {
                    call = call.include_subscribed(arg_from_str(value.unwrap_or("false"), err, "include-subscribed", "boolean"));
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _apps_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.apps().get(&self.opt.arg_app_id);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _apps_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.apps().list();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "language-code" => {
                    call = call.language_code(value.unwrap_or(""));
                },
                "app-filter-mime-types" => {
                    call = call.app_filter_mime_types(value.unwrap_or(""));
                },
                "app-filter-extensions" => {
                    call = call.app_filter_extensions(value.unwrap_or(""));
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _changes_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.changes().get(&self.opt.arg_change_id);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _changes_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.changes().list();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-change-id" => {
                    call = call.start_change_id(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "include-subscribed" => {
                    call = call.include_subscribed(arg_from_str(value.unwrap_or("false"), err, "include-subscribed", "boolean"));
                },
                "include-deleted" => {
                    call = call.include_deleted(arg_from_str(value.unwrap_or("false"), err, "include-deleted", "boolean"));
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _changes_watch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Channel::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
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
                "resource-uri" => {
                        request.resource_uri = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "resource-id" => {
                        request.resource_id = Some(value.unwrap_or("").to_string());
                    },
                "payload" => {
                        request.payload = Some(arg_from_str(value.unwrap_or("false"), err, "payload", "boolean"));
                    },
                "token" => {
                        request.token = Some(value.unwrap_or("").to_string());
                    },
                "params" => {
                        if request.params.is_none() {
                           request.params = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.params.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "expiration" => {
                        request.expiration = Some(value.unwrap_or("").to_string());
                    },
                "address" => {
                        request.address = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.changes().watch(request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-change-id" => {
                    call = call.start_change_id(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "include-subscribed" => {
                    call = call.include_subscribed(arg_from_str(value.unwrap_or("false"), err, "include-subscribed", "boolean"));
                },
                "include-deleted" => {
                    call = call.include_deleted(arg_from_str(value.unwrap_or("false"), err, "include-deleted", "boolean"));
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _channels_stop(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Channel::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
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
                "resource-uri" => {
                        request.resource_uri = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "resource-id" => {
                        request.resource_id = Some(value.unwrap_or("").to_string());
                    },
                "payload" => {
                        request.payload = Some(arg_from_str(value.unwrap_or("false"), err, "payload", "boolean"));
                    },
                "token" => {
                        request.token = Some(value.unwrap_or("").to_string());
                    },
                "params" => {
                        if request.params.is_none() {
                           request.params = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.params.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "expiration" => {
                        request.expiration = Some(value.unwrap_or("").to_string());
                    },
                "address" => {
                        request.address = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.channels().stop(request);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _children_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.children().delete(&self.opt.arg_folder_id, &self.opt.arg_child_id);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _children_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.children().get(&self.opt.arg_folder_id, &self.opt.arg_child_id);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _children_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::ChildReference::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
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
                "child-link" => {
                        request.child_link = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.children().insert(request, &self.opt.arg_folder_id);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _children_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.children().list(&self.opt.arg_folder_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "q" => {
                    call = call.q(value.unwrap_or(""));
                },
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _comments_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.comments().delete(&self.opt.arg_file_id, &self.opt.arg_comment_id);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _comments_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.comments().get(&self.opt.arg_file_id, &self.opt.arg_comment_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "include-deleted" => {
                    call = call.include_deleted(arg_from_str(value.unwrap_or("false"), err, "include-deleted", "boolean"));
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _comments_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Comment::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
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
            fn request_author_init(request: &mut api::Comment) {
                if request.author.is_none() {
                    request.author = Some(Default::default());
                }
            }
            
            fn request_author_picture_init(request: &mut api::Comment) {
                request_author_init(request);
                if request.author.as_mut().unwrap().picture.is_none() {
                    request.author.as_mut().unwrap().picture = Some(Default::default());
                }
            }
            
            fn request_context_init(request: &mut api::Comment) {
                if request.context.is_none() {
                    request.context = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "status" => {
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "author.picture.url" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().picture.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "author.kind" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "author.display-name" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "author.permission-id" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().permission_id = Some(value.unwrap_or("").to_string());
                    },
                "author.is-authenticated-user" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().is_authenticated_user = Some(arg_from_str(value.unwrap_or("false"), err, "author.is-authenticated-user", "boolean"));
                    },
                "author.email-address" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().email_address = Some(value.unwrap_or("").to_string());
                    },
                "deleted" => {
                        request_author_init(&mut request);
                        request.deleted = Some(arg_from_str(value.unwrap_or("false"), err, "deleted", "boolean"));
                    },
                "html-content" => {
                        request_author_init(&mut request);
                        request.html_content = Some(value.unwrap_or("").to_string());
                    },
                "file-title" => {
                        request_author_init(&mut request);
                        request.file_title = Some(value.unwrap_or("").to_string());
                    },
                "content" => {
                        request_author_init(&mut request);
                        request.content = Some(value.unwrap_or("").to_string());
                    },
                "modified-date" => {
                        request_author_init(&mut request);
                        request.modified_date = Some(value.unwrap_or("").to_string());
                    },
                "context.type" => {
                        request_context_init(&mut request);
                        request.context.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "context.value" => {
                        request_context_init(&mut request);
                        request.context.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "created-date" => {
                        request_context_init(&mut request);
                        request.created_date = Some(value.unwrap_or("").to_string());
                    },
                "comment-id" => {
                        request_context_init(&mut request);
                        request.comment_id = Some(value.unwrap_or("").to_string());
                    },
                "anchor" => {
                        request_context_init(&mut request);
                        request.anchor = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_context_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "file-id" => {
                        request_context_init(&mut request);
                        request.file_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.comments().insert(request, &self.opt.arg_file_id);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _comments_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.comments().list(&self.opt.arg_file_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "updated-min" => {
                    call = call.updated_min(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "include-deleted" => {
                    call = call.include_deleted(arg_from_str(value.unwrap_or("false"), err, "include-deleted", "boolean"));
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _comments_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Comment::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
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
            fn request_author_init(request: &mut api::Comment) {
                if request.author.is_none() {
                    request.author = Some(Default::default());
                }
            }
            
            fn request_author_picture_init(request: &mut api::Comment) {
                request_author_init(request);
                if request.author.as_mut().unwrap().picture.is_none() {
                    request.author.as_mut().unwrap().picture = Some(Default::default());
                }
            }
            
            fn request_context_init(request: &mut api::Comment) {
                if request.context.is_none() {
                    request.context = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "status" => {
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "author.picture.url" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().picture.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "author.kind" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "author.display-name" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "author.permission-id" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().permission_id = Some(value.unwrap_or("").to_string());
                    },
                "author.is-authenticated-user" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().is_authenticated_user = Some(arg_from_str(value.unwrap_or("false"), err, "author.is-authenticated-user", "boolean"));
                    },
                "author.email-address" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().email_address = Some(value.unwrap_or("").to_string());
                    },
                "deleted" => {
                        request_author_init(&mut request);
                        request.deleted = Some(arg_from_str(value.unwrap_or("false"), err, "deleted", "boolean"));
                    },
                "html-content" => {
                        request_author_init(&mut request);
                        request.html_content = Some(value.unwrap_or("").to_string());
                    },
                "file-title" => {
                        request_author_init(&mut request);
                        request.file_title = Some(value.unwrap_or("").to_string());
                    },
                "content" => {
                        request_author_init(&mut request);
                        request.content = Some(value.unwrap_or("").to_string());
                    },
                "modified-date" => {
                        request_author_init(&mut request);
                        request.modified_date = Some(value.unwrap_or("").to_string());
                    },
                "context.type" => {
                        request_context_init(&mut request);
                        request.context.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "context.value" => {
                        request_context_init(&mut request);
                        request.context.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "created-date" => {
                        request_context_init(&mut request);
                        request.created_date = Some(value.unwrap_or("").to_string());
                    },
                "comment-id" => {
                        request_context_init(&mut request);
                        request.comment_id = Some(value.unwrap_or("").to_string());
                    },
                "anchor" => {
                        request_context_init(&mut request);
                        request.anchor = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_context_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "file-id" => {
                        request_context_init(&mut request);
                        request.file_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.comments().patch(request, &self.opt.arg_file_id, &self.opt.arg_comment_id);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _comments_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Comment::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
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
            fn request_author_init(request: &mut api::Comment) {
                if request.author.is_none() {
                    request.author = Some(Default::default());
                }
            }
            
            fn request_author_picture_init(request: &mut api::Comment) {
                request_author_init(request);
                if request.author.as_mut().unwrap().picture.is_none() {
                    request.author.as_mut().unwrap().picture = Some(Default::default());
                }
            }
            
            fn request_context_init(request: &mut api::Comment) {
                if request.context.is_none() {
                    request.context = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "status" => {
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "author.picture.url" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().picture.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "author.kind" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "author.display-name" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "author.permission-id" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().permission_id = Some(value.unwrap_or("").to_string());
                    },
                "author.is-authenticated-user" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().is_authenticated_user = Some(arg_from_str(value.unwrap_or("false"), err, "author.is-authenticated-user", "boolean"));
                    },
                "author.email-address" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().email_address = Some(value.unwrap_or("").to_string());
                    },
                "deleted" => {
                        request_author_init(&mut request);
                        request.deleted = Some(arg_from_str(value.unwrap_or("false"), err, "deleted", "boolean"));
                    },
                "html-content" => {
                        request_author_init(&mut request);
                        request.html_content = Some(value.unwrap_or("").to_string());
                    },
                "file-title" => {
                        request_author_init(&mut request);
                        request.file_title = Some(value.unwrap_or("").to_string());
                    },
                "content" => {
                        request_author_init(&mut request);
                        request.content = Some(value.unwrap_or("").to_string());
                    },
                "modified-date" => {
                        request_author_init(&mut request);
                        request.modified_date = Some(value.unwrap_or("").to_string());
                    },
                "context.type" => {
                        request_context_init(&mut request);
                        request.context.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "context.value" => {
                        request_context_init(&mut request);
                        request.context.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "created-date" => {
                        request_context_init(&mut request);
                        request.created_date = Some(value.unwrap_or("").to_string());
                    },
                "comment-id" => {
                        request_context_init(&mut request);
                        request.comment_id = Some(value.unwrap_or("").to_string());
                    },
                "anchor" => {
                        request_context_init(&mut request);
                        request.anchor = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_context_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "file-id" => {
                        request_context_init(&mut request);
                        request.file_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.comments().update(request, &self.opt.arg_file_id, &self.opt.arg_comment_id);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _files_copy(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::File::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
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
            fn request_image_media_metadata_init(request: &mut api::File) {
                if request.image_media_metadata.is_none() {
                    request.image_media_metadata = Some(Default::default());
                }
            }
            
            fn request_image_media_metadata_location_init(request: &mut api::File) {
                request_image_media_metadata_init(request);
                if request.image_media_metadata.as_mut().unwrap().location.is_none() {
                    request.image_media_metadata.as_mut().unwrap().location = Some(Default::default());
                }
            }
            
            fn request_indexable_text_init(request: &mut api::File) {
                if request.indexable_text.is_none() {
                    request.indexable_text = Some(Default::default());
                }
            }
            
            fn request_labels_init(request: &mut api::File) {
                if request.labels.is_none() {
                    request.labels = Some(Default::default());
                }
            }
            
            fn request_last_modifying_user_init(request: &mut api::File) {
                if request.last_modifying_user.is_none() {
                    request.last_modifying_user = Some(Default::default());
                }
            }
            
            fn request_last_modifying_user_picture_init(request: &mut api::File) {
                request_last_modifying_user_init(request);
                if request.last_modifying_user.as_mut().unwrap().picture.is_none() {
                    request.last_modifying_user.as_mut().unwrap().picture = Some(Default::default());
                }
            }
            
            fn request_sharing_user_init(request: &mut api::File) {
                if request.sharing_user.is_none() {
                    request.sharing_user = Some(Default::default());
                }
            }
            
            fn request_sharing_user_picture_init(request: &mut api::File) {
                request_sharing_user_init(request);
                if request.sharing_user.as_mut().unwrap().picture.is_none() {
                    request.sharing_user.as_mut().unwrap().picture = Some(Default::default());
                }
            }
            
            fn request_thumbnail_init(request: &mut api::File) {
                if request.thumbnail.is_none() {
                    request.thumbnail = Some(Default::default());
                }
            }
            
            fn request_user_permission_init(request: &mut api::File) {
                if request.user_permission.is_none() {
                    request.user_permission = Some(Default::default());
                }
            }
            
            fn request_video_media_metadata_init(request: &mut api::File) {
                if request.video_media_metadata.is_none() {
                    request.video_media_metadata = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "mime-type" => {
                        request.mime_type = Some(value.unwrap_or("").to_string());
                    },
                "last-viewed-by-me-date" => {
                        request.last_viewed_by_me_date = Some(value.unwrap_or("").to_string());
                    },
                "app-data-contents" => {
                        request.app_data_contents = Some(arg_from_str(value.unwrap_or("false"), err, "app-data-contents", "boolean"));
                    },
                "thumbnail-link" => {
                        request.thumbnail_link = Some(value.unwrap_or("").to_string());
                    },
                "labels.restricted" => {
                        request_labels_init(&mut request);
                        request.labels.as_mut().unwrap().restricted = Some(arg_from_str(value.unwrap_or("false"), err, "labels.restricted", "boolean"));
                    },
                "labels.hidden" => {
                        request_labels_init(&mut request);
                        request.labels.as_mut().unwrap().hidden = Some(arg_from_str(value.unwrap_or("false"), err, "labels.hidden", "boolean"));
                    },
                "labels.viewed" => {
                        request_labels_init(&mut request);
                        request.labels.as_mut().unwrap().viewed = Some(arg_from_str(value.unwrap_or("false"), err, "labels.viewed", "boolean"));
                    },
                "labels.starred" => {
                        request_labels_init(&mut request);
                        request.labels.as_mut().unwrap().starred = Some(arg_from_str(value.unwrap_or("false"), err, "labels.starred", "boolean"));
                    },
                "labels.trashed" => {
                        request_labels_init(&mut request);
                        request.labels.as_mut().unwrap().trashed = Some(arg_from_str(value.unwrap_or("false"), err, "labels.trashed", "boolean"));
                    },
                "indexable-text.text" => {
                        request_indexable_text_init(&mut request);
                        request.indexable_text.as_mut().unwrap().text = Some(value.unwrap_or("").to_string());
                    },
                "explicitly-trashed" => {
                        request_indexable_text_init(&mut request);
                        request.explicitly_trashed = Some(arg_from_str(value.unwrap_or("false"), err, "explicitly-trashed", "boolean"));
                    },
                "etag" => {
                        request_indexable_text_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user-name" => {
                        request_indexable_text_init(&mut request);
                        request.last_modifying_user_name = Some(value.unwrap_or("").to_string());
                    },
                "writers-can-share" => {
                        request_indexable_text_init(&mut request);
                        request.writers_can_share = Some(arg_from_str(value.unwrap_or("false"), err, "writers-can-share", "boolean"));
                    },
                "id" => {
                        request_indexable_text_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "sharing-user.picture.url" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().picture.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "sharing-user.kind" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "sharing-user.display-name" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "sharing-user.permission-id" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().permission_id = Some(value.unwrap_or("").to_string());
                    },
                "sharing-user.is-authenticated-user" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().is_authenticated_user = Some(arg_from_str(value.unwrap_or("false"), err, "sharing-user.is-authenticated-user", "boolean"));
                    },
                "sharing-user.email-address" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().email_address = Some(value.unwrap_or("").to_string());
                    },
                "video-media-metadata.width" => {
                        request_video_media_metadata_init(&mut request);
                        request.video_media_metadata.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "video-media-metadata.width", "integer"));
                    },
                "video-media-metadata.duration-millis" => {
                        request_video_media_metadata_init(&mut request);
                        request.video_media_metadata.as_mut().unwrap().duration_millis = Some(value.unwrap_or("").to_string());
                    },
                "video-media-metadata.height" => {
                        request_video_media_metadata_init(&mut request);
                        request.video_media_metadata.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "video-media-metadata.height", "integer"));
                    },
                "last-modifying-user.picture.url" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().picture.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.kind" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.display-name" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.permission-id" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().permission_id = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.is-authenticated-user" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().is_authenticated_user = Some(arg_from_str(value.unwrap_or("false"), err, "last-modifying-user.is-authenticated-user", "boolean"));
                    },
                "last-modifying-user.email-address" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().email_address = Some(value.unwrap_or("").to_string());
                    },
                "copyable" => {
                        request_last_modifying_user_init(&mut request);
                        request.copyable = Some(arg_from_str(value.unwrap_or("false"), err, "copyable", "boolean"));
                    },
                "folder-color-rgb" => {
                        request_last_modifying_user_init(&mut request);
                        request.folder_color_rgb = Some(value.unwrap_or("").to_string());
                    },
                "owner-names" => {
                        request_last_modifying_user_init(&mut request);
                        if request.owner_names.is_none() {
                           request.owner_names = Some(Default::default());
                        }
                                        request.owner_names.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "shared-with-me-date" => {
                        request_last_modifying_user_init(&mut request);
                        request.shared_with_me_date = Some(value.unwrap_or("").to_string());
                    },
                "web-view-link" => {
                        request_last_modifying_user_init(&mut request);
                        request.web_view_link = Some(value.unwrap_or("").to_string());
                    },
                "version" => {
                        request_last_modifying_user_init(&mut request);
                        request.version = Some(value.unwrap_or("").to_string());
                    },
                "export-links" => {
                        request_last_modifying_user_init(&mut request);
                        if request.export_links.is_none() {
                           request.export_links = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.export_links.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "shared" => {
                        request_last_modifying_user_init(&mut request);
                        request.shared = Some(arg_from_str(value.unwrap_or("false"), err, "shared", "boolean"));
                    },
                "thumbnail.mime-type" => {
                        request_thumbnail_init(&mut request);
                        request.thumbnail.as_mut().unwrap().mime_type = Some(value.unwrap_or("").to_string());
                    },
                "thumbnail.image" => {
                        request_thumbnail_init(&mut request);
                        request.thumbnail.as_mut().unwrap().image = Some(value.unwrap_or("").to_string());
                    },
                "open-with-links" => {
                        request_thumbnail_init(&mut request);
                        if request.open_with_links.is_none() {
                           request.open_with_links = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.open_with_links.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "image-media-metadata.exposure-bias" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().exposure_bias = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.exposure-bias", "number"));
                    },
                "image-media-metadata.exposure-time" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().exposure_time = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.exposure-time", "number"));
                    },
                "image-media-metadata.max-aperture-value" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().max_aperture_value = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.max-aperture-value", "number"));
                    },
                "image-media-metadata.width" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "image-media-metadata.width", "integer"));
                    },
                "image-media-metadata.focal-length" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().focal_length = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.focal-length", "number"));
                    },
                "image-media-metadata.camera-make" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().camera_make = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.exposure-mode" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().exposure_mode = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.color-space" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().color_space = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.location.latitude" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().location.as_mut().unwrap().latitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.location.latitude", "number"));
                    },
                "image-media-metadata.location.altitude" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().location.as_mut().unwrap().altitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.location.altitude", "number"));
                    },
                "image-media-metadata.location.longitude" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().location.as_mut().unwrap().longitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.location.longitude", "number"));
                    },
                "image-media-metadata.subject-distance" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().subject_distance = Some(arg_from_str(value.unwrap_or("-0"), err, "image-media-metadata.subject-distance", "integer"));
                    },
                "image-media-metadata.height" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "image-media-metadata.height", "integer"));
                    },
                "image-media-metadata.lens" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().lens = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.date" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().date = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.iso-speed" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().iso_speed = Some(arg_from_str(value.unwrap_or("-0"), err, "image-media-metadata.iso-speed", "integer"));
                    },
                "image-media-metadata.metering-mode" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().metering_mode = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.flash-used" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().flash_used = Some(arg_from_str(value.unwrap_or("false"), err, "image-media-metadata.flash-used", "boolean"));
                    },
                "image-media-metadata.aperture" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().aperture = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.aperture", "number"));
                    },
                "image-media-metadata.rotation" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().rotation = Some(arg_from_str(value.unwrap_or("-0"), err, "image-media-metadata.rotation", "integer"));
                    },
                "image-media-metadata.sensor" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().sensor = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.white-balance" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().white_balance = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.camera-model" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().camera_model = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request_image_media_metadata_init(&mut request);
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "web-content-link" => {
                        request_image_media_metadata_init(&mut request);
                        request.web_content_link = Some(value.unwrap_or("").to_string());
                    },
                "editable" => {
                        request_image_media_metadata_init(&mut request);
                        request.editable = Some(arg_from_str(value.unwrap_or("false"), err, "editable", "boolean"));
                    },
                "embed-link" => {
                        request_image_media_metadata_init(&mut request);
                        request.embed_link = Some(value.unwrap_or("").to_string());
                    },
                "marked-viewed-by-me-date" => {
                        request_image_media_metadata_init(&mut request);
                        request.marked_viewed_by_me_date = Some(value.unwrap_or("").to_string());
                    },
                "quota-bytes-used" => {
                        request_image_media_metadata_init(&mut request);
                        request.quota_bytes_used = Some(value.unwrap_or("").to_string());
                    },
                "file-size" => {
                        request_image_media_metadata_init(&mut request);
                        request.file_size = Some(value.unwrap_or("").to_string());
                    },
                "created-date" => {
                        request_image_media_metadata_init(&mut request);
                        request.created_date = Some(value.unwrap_or("").to_string());
                    },
                "md5-checksum" => {
                        request_image_media_metadata_init(&mut request);
                        request.md5_checksum = Some(value.unwrap_or("").to_string());
                    },
                "icon-link" => {
                        request_image_media_metadata_init(&mut request);
                        request.icon_link = Some(value.unwrap_or("").to_string());
                    },
                "default-open-with-link" => {
                        request_image_media_metadata_init(&mut request);
                        request.default_open_with_link = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_image_media_metadata_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "alternate-link" => {
                        request_image_media_metadata_init(&mut request);
                        request.alternate_link = Some(value.unwrap_or("").to_string());
                    },
                "title" => {
                        request_image_media_metadata_init(&mut request);
                        request.title = Some(value.unwrap_or("").to_string());
                    },
                "modified-by-me-date" => {
                        request_image_media_metadata_init(&mut request);
                        request.modified_by_me_date = Some(value.unwrap_or("").to_string());
                    },
                "download-url" => {
                        request_image_media_metadata_init(&mut request);
                        request.download_url = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.with-link" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().with_link = Some(arg_from_str(value.unwrap_or("false"), err, "user-permission.with-link", "boolean"));
                    },
                "user-permission.domain" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().domain = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.name" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.kind" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.value" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.id" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.auth-key" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().auth_key = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.etag" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().etag = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.email-address" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().email_address = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.photo-link" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().photo_link = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.role" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().role = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.type" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.additional-roles" => {
                        request_user_permission_init(&mut request);
                        if request.user_permission.as_mut().unwrap().additional_roles.is_none() {
                           request.user_permission.as_mut().unwrap().additional_roles = Some(Default::default());
                        }
                                        request.user_permission.as_mut().unwrap().additional_roles.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "user-permission.self-link" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().self_link = Some(value.unwrap_or("").to_string());
                    },
                "original-filename" => {
                        request_user_permission_init(&mut request);
                        request.original_filename = Some(value.unwrap_or("").to_string());
                    },
                "file-extension" => {
                        request_user_permission_init(&mut request);
                        request.file_extension = Some(value.unwrap_or("").to_string());
                    },
                "head-revision-id" => {
                        request_user_permission_init(&mut request);
                        request.head_revision_id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_user_permission_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "modified-date" => {
                        request_user_permission_init(&mut request);
                        request.modified_date = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.files().copy(request, &self.opt.arg_file_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "visibility" => {
                    call = call.visibility(value.unwrap_or(""));
                },
                "timed-text-track-name" => {
                    call = call.timed_text_track_name(value.unwrap_or(""));
                },
                "timed-text-language" => {
                    call = call.timed_text_language(value.unwrap_or(""));
                },
                "pinned" => {
                    call = call.pinned(arg_from_str(value.unwrap_or("false"), err, "pinned", "boolean"));
                },
                "ocr-language" => {
                    call = call.ocr_language(value.unwrap_or(""));
                },
                "ocr" => {
                    call = call.ocr(arg_from_str(value.unwrap_or("false"), err, "ocr", "boolean"));
                },
                "convert" => {
                    call = call.convert(arg_from_str(value.unwrap_or("false"), err, "convert", "boolean"));
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _files_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.files().delete(&self.opt.arg_file_id);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _files_empty_trash(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.files().empty_trash();
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _files_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut download_mode = false;
        let mut call = self.hub.files().get(&self.opt.arg_file_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-viewed-date" => {
                    call = call.update_viewed_date(arg_from_str(value.unwrap_or("false"), err, "update-viewed-date", "boolean"));
                },
                "revision-id" => {
                    call = call.revision_id(value.unwrap_or(""));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "acknowledge-abuse" => {
                    call = call.acknowledge_abuse(arg_from_str(value.unwrap_or("false"), err, "acknowledge-abuse", "boolean"));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    if key == "alt" && value.unwrap_or("unset") == "media" {
                        download_mode = true;
                    }
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    if !download_mode {
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    } else {
                    io::copy(&mut response, &mut ostream).unwrap();
                    }
                    None
                }
            }
        }
    }

    fn _files_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::File::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
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
            fn request_image_media_metadata_init(request: &mut api::File) {
                if request.image_media_metadata.is_none() {
                    request.image_media_metadata = Some(Default::default());
                }
            }
            
            fn request_image_media_metadata_location_init(request: &mut api::File) {
                request_image_media_metadata_init(request);
                if request.image_media_metadata.as_mut().unwrap().location.is_none() {
                    request.image_media_metadata.as_mut().unwrap().location = Some(Default::default());
                }
            }
            
            fn request_indexable_text_init(request: &mut api::File) {
                if request.indexable_text.is_none() {
                    request.indexable_text = Some(Default::default());
                }
            }
            
            fn request_labels_init(request: &mut api::File) {
                if request.labels.is_none() {
                    request.labels = Some(Default::default());
                }
            }
            
            fn request_last_modifying_user_init(request: &mut api::File) {
                if request.last_modifying_user.is_none() {
                    request.last_modifying_user = Some(Default::default());
                }
            }
            
            fn request_last_modifying_user_picture_init(request: &mut api::File) {
                request_last_modifying_user_init(request);
                if request.last_modifying_user.as_mut().unwrap().picture.is_none() {
                    request.last_modifying_user.as_mut().unwrap().picture = Some(Default::default());
                }
            }
            
            fn request_sharing_user_init(request: &mut api::File) {
                if request.sharing_user.is_none() {
                    request.sharing_user = Some(Default::default());
                }
            }
            
            fn request_sharing_user_picture_init(request: &mut api::File) {
                request_sharing_user_init(request);
                if request.sharing_user.as_mut().unwrap().picture.is_none() {
                    request.sharing_user.as_mut().unwrap().picture = Some(Default::default());
                }
            }
            
            fn request_thumbnail_init(request: &mut api::File) {
                if request.thumbnail.is_none() {
                    request.thumbnail = Some(Default::default());
                }
            }
            
            fn request_user_permission_init(request: &mut api::File) {
                if request.user_permission.is_none() {
                    request.user_permission = Some(Default::default());
                }
            }
            
            fn request_video_media_metadata_init(request: &mut api::File) {
                if request.video_media_metadata.is_none() {
                    request.video_media_metadata = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "mime-type" => {
                        request.mime_type = Some(value.unwrap_or("").to_string());
                    },
                "last-viewed-by-me-date" => {
                        request.last_viewed_by_me_date = Some(value.unwrap_or("").to_string());
                    },
                "app-data-contents" => {
                        request.app_data_contents = Some(arg_from_str(value.unwrap_or("false"), err, "app-data-contents", "boolean"));
                    },
                "thumbnail-link" => {
                        request.thumbnail_link = Some(value.unwrap_or("").to_string());
                    },
                "labels.restricted" => {
                        request_labels_init(&mut request);
                        request.labels.as_mut().unwrap().restricted = Some(arg_from_str(value.unwrap_or("false"), err, "labels.restricted", "boolean"));
                    },
                "labels.hidden" => {
                        request_labels_init(&mut request);
                        request.labels.as_mut().unwrap().hidden = Some(arg_from_str(value.unwrap_or("false"), err, "labels.hidden", "boolean"));
                    },
                "labels.viewed" => {
                        request_labels_init(&mut request);
                        request.labels.as_mut().unwrap().viewed = Some(arg_from_str(value.unwrap_or("false"), err, "labels.viewed", "boolean"));
                    },
                "labels.starred" => {
                        request_labels_init(&mut request);
                        request.labels.as_mut().unwrap().starred = Some(arg_from_str(value.unwrap_or("false"), err, "labels.starred", "boolean"));
                    },
                "labels.trashed" => {
                        request_labels_init(&mut request);
                        request.labels.as_mut().unwrap().trashed = Some(arg_from_str(value.unwrap_or("false"), err, "labels.trashed", "boolean"));
                    },
                "indexable-text.text" => {
                        request_indexable_text_init(&mut request);
                        request.indexable_text.as_mut().unwrap().text = Some(value.unwrap_or("").to_string());
                    },
                "explicitly-trashed" => {
                        request_indexable_text_init(&mut request);
                        request.explicitly_trashed = Some(arg_from_str(value.unwrap_or("false"), err, "explicitly-trashed", "boolean"));
                    },
                "etag" => {
                        request_indexable_text_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user-name" => {
                        request_indexable_text_init(&mut request);
                        request.last_modifying_user_name = Some(value.unwrap_or("").to_string());
                    },
                "writers-can-share" => {
                        request_indexable_text_init(&mut request);
                        request.writers_can_share = Some(arg_from_str(value.unwrap_or("false"), err, "writers-can-share", "boolean"));
                    },
                "id" => {
                        request_indexable_text_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "sharing-user.picture.url" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().picture.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "sharing-user.kind" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "sharing-user.display-name" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "sharing-user.permission-id" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().permission_id = Some(value.unwrap_or("").to_string());
                    },
                "sharing-user.is-authenticated-user" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().is_authenticated_user = Some(arg_from_str(value.unwrap_or("false"), err, "sharing-user.is-authenticated-user", "boolean"));
                    },
                "sharing-user.email-address" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().email_address = Some(value.unwrap_or("").to_string());
                    },
                "video-media-metadata.width" => {
                        request_video_media_metadata_init(&mut request);
                        request.video_media_metadata.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "video-media-metadata.width", "integer"));
                    },
                "video-media-metadata.duration-millis" => {
                        request_video_media_metadata_init(&mut request);
                        request.video_media_metadata.as_mut().unwrap().duration_millis = Some(value.unwrap_or("").to_string());
                    },
                "video-media-metadata.height" => {
                        request_video_media_metadata_init(&mut request);
                        request.video_media_metadata.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "video-media-metadata.height", "integer"));
                    },
                "last-modifying-user.picture.url" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().picture.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.kind" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.display-name" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.permission-id" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().permission_id = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.is-authenticated-user" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().is_authenticated_user = Some(arg_from_str(value.unwrap_or("false"), err, "last-modifying-user.is-authenticated-user", "boolean"));
                    },
                "last-modifying-user.email-address" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().email_address = Some(value.unwrap_or("").to_string());
                    },
                "copyable" => {
                        request_last_modifying_user_init(&mut request);
                        request.copyable = Some(arg_from_str(value.unwrap_or("false"), err, "copyable", "boolean"));
                    },
                "folder-color-rgb" => {
                        request_last_modifying_user_init(&mut request);
                        request.folder_color_rgb = Some(value.unwrap_or("").to_string());
                    },
                "owner-names" => {
                        request_last_modifying_user_init(&mut request);
                        if request.owner_names.is_none() {
                           request.owner_names = Some(Default::default());
                        }
                                        request.owner_names.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "shared-with-me-date" => {
                        request_last_modifying_user_init(&mut request);
                        request.shared_with_me_date = Some(value.unwrap_or("").to_string());
                    },
                "web-view-link" => {
                        request_last_modifying_user_init(&mut request);
                        request.web_view_link = Some(value.unwrap_or("").to_string());
                    },
                "version" => {
                        request_last_modifying_user_init(&mut request);
                        request.version = Some(value.unwrap_or("").to_string());
                    },
                "export-links" => {
                        request_last_modifying_user_init(&mut request);
                        if request.export_links.is_none() {
                           request.export_links = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.export_links.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "shared" => {
                        request_last_modifying_user_init(&mut request);
                        request.shared = Some(arg_from_str(value.unwrap_or("false"), err, "shared", "boolean"));
                    },
                "thumbnail.mime-type" => {
                        request_thumbnail_init(&mut request);
                        request.thumbnail.as_mut().unwrap().mime_type = Some(value.unwrap_or("").to_string());
                    },
                "thumbnail.image" => {
                        request_thumbnail_init(&mut request);
                        request.thumbnail.as_mut().unwrap().image = Some(value.unwrap_or("").to_string());
                    },
                "open-with-links" => {
                        request_thumbnail_init(&mut request);
                        if request.open_with_links.is_none() {
                           request.open_with_links = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.open_with_links.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "image-media-metadata.exposure-bias" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().exposure_bias = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.exposure-bias", "number"));
                    },
                "image-media-metadata.exposure-time" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().exposure_time = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.exposure-time", "number"));
                    },
                "image-media-metadata.max-aperture-value" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().max_aperture_value = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.max-aperture-value", "number"));
                    },
                "image-media-metadata.width" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "image-media-metadata.width", "integer"));
                    },
                "image-media-metadata.focal-length" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().focal_length = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.focal-length", "number"));
                    },
                "image-media-metadata.camera-make" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().camera_make = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.exposure-mode" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().exposure_mode = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.color-space" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().color_space = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.location.latitude" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().location.as_mut().unwrap().latitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.location.latitude", "number"));
                    },
                "image-media-metadata.location.altitude" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().location.as_mut().unwrap().altitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.location.altitude", "number"));
                    },
                "image-media-metadata.location.longitude" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().location.as_mut().unwrap().longitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.location.longitude", "number"));
                    },
                "image-media-metadata.subject-distance" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().subject_distance = Some(arg_from_str(value.unwrap_or("-0"), err, "image-media-metadata.subject-distance", "integer"));
                    },
                "image-media-metadata.height" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "image-media-metadata.height", "integer"));
                    },
                "image-media-metadata.lens" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().lens = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.date" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().date = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.iso-speed" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().iso_speed = Some(arg_from_str(value.unwrap_or("-0"), err, "image-media-metadata.iso-speed", "integer"));
                    },
                "image-media-metadata.metering-mode" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().metering_mode = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.flash-used" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().flash_used = Some(arg_from_str(value.unwrap_or("false"), err, "image-media-metadata.flash-used", "boolean"));
                    },
                "image-media-metadata.aperture" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().aperture = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.aperture", "number"));
                    },
                "image-media-metadata.rotation" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().rotation = Some(arg_from_str(value.unwrap_or("-0"), err, "image-media-metadata.rotation", "integer"));
                    },
                "image-media-metadata.sensor" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().sensor = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.white-balance" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().white_balance = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.camera-model" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().camera_model = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request_image_media_metadata_init(&mut request);
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "web-content-link" => {
                        request_image_media_metadata_init(&mut request);
                        request.web_content_link = Some(value.unwrap_or("").to_string());
                    },
                "editable" => {
                        request_image_media_metadata_init(&mut request);
                        request.editable = Some(arg_from_str(value.unwrap_or("false"), err, "editable", "boolean"));
                    },
                "embed-link" => {
                        request_image_media_metadata_init(&mut request);
                        request.embed_link = Some(value.unwrap_or("").to_string());
                    },
                "marked-viewed-by-me-date" => {
                        request_image_media_metadata_init(&mut request);
                        request.marked_viewed_by_me_date = Some(value.unwrap_or("").to_string());
                    },
                "quota-bytes-used" => {
                        request_image_media_metadata_init(&mut request);
                        request.quota_bytes_used = Some(value.unwrap_or("").to_string());
                    },
                "file-size" => {
                        request_image_media_metadata_init(&mut request);
                        request.file_size = Some(value.unwrap_or("").to_string());
                    },
                "created-date" => {
                        request_image_media_metadata_init(&mut request);
                        request.created_date = Some(value.unwrap_or("").to_string());
                    },
                "md5-checksum" => {
                        request_image_media_metadata_init(&mut request);
                        request.md5_checksum = Some(value.unwrap_or("").to_string());
                    },
                "icon-link" => {
                        request_image_media_metadata_init(&mut request);
                        request.icon_link = Some(value.unwrap_or("").to_string());
                    },
                "default-open-with-link" => {
                        request_image_media_metadata_init(&mut request);
                        request.default_open_with_link = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_image_media_metadata_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "alternate-link" => {
                        request_image_media_metadata_init(&mut request);
                        request.alternate_link = Some(value.unwrap_or("").to_string());
                    },
                "title" => {
                        request_image_media_metadata_init(&mut request);
                        request.title = Some(value.unwrap_or("").to_string());
                    },
                "modified-by-me-date" => {
                        request_image_media_metadata_init(&mut request);
                        request.modified_by_me_date = Some(value.unwrap_or("").to_string());
                    },
                "download-url" => {
                        request_image_media_metadata_init(&mut request);
                        request.download_url = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.with-link" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().with_link = Some(arg_from_str(value.unwrap_or("false"), err, "user-permission.with-link", "boolean"));
                    },
                "user-permission.domain" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().domain = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.name" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.kind" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.value" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.id" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.auth-key" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().auth_key = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.etag" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().etag = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.email-address" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().email_address = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.photo-link" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().photo_link = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.role" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().role = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.type" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.additional-roles" => {
                        request_user_permission_init(&mut request);
                        if request.user_permission.as_mut().unwrap().additional_roles.is_none() {
                           request.user_permission.as_mut().unwrap().additional_roles = Some(Default::default());
                        }
                                        request.user_permission.as_mut().unwrap().additional_roles.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "user-permission.self-link" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().self_link = Some(value.unwrap_or("").to_string());
                    },
                "original-filename" => {
                        request_user_permission_init(&mut request);
                        request.original_filename = Some(value.unwrap_or("").to_string());
                    },
                "file-extension" => {
                        request_user_permission_init(&mut request);
                        request.file_extension = Some(value.unwrap_or("").to_string());
                    },
                "head-revision-id" => {
                        request_user_permission_init(&mut request);
                        request.head_revision_id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_user_permission_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "modified-date" => {
                        request_user_permission_init(&mut request);
                        request.modified_date = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.files().insert(request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "visibility" => {
                    call = call.visibility(value.unwrap_or(""));
                },
                "use-content-as-indexable-text" => {
                    call = call.use_content_as_indexable_text(arg_from_str(value.unwrap_or("false"), err, "use-content-as-indexable-text", "boolean"));
                },
                "timed-text-track-name" => {
                    call = call.timed_text_track_name(value.unwrap_or(""));
                },
                "timed-text-language" => {
                    call = call.timed_text_language(value.unwrap_or(""));
                },
                "pinned" => {
                    call = call.pinned(arg_from_str(value.unwrap_or("false"), err, "pinned", "boolean"));
                },
                "ocr-language" => {
                    call = call.ocr_language(value.unwrap_or(""));
                },
                "ocr" => {
                    call = call.ocr(arg_from_str(value.unwrap_or("false"), err, "ocr", "boolean"));
                },
                "convert" => {
                    call = call.convert(arg_from_str(value.unwrap_or("false"), err, "convert", "boolean"));
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
        let protocol = 
            if self.opt.cmd_simple {
                "simple"
            } else if self.opt.cmd_resumable {
                "resumable"
            } else { 
                unreachable!() 
            };
        let mut input_file = input_file_from_opts(&self.opt.arg_file, err);
        let mime_type = input_mime_from_opts(&self.opt.arg_mime, err);
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "simple" => call.upload(input_file.unwrap(), mime_type.unwrap()),
                "resumable" => call.upload_resumable(input_file.unwrap(), mime_type.unwrap()),
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

    fn _files_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.files().list();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "q" => {
                    call = call.q(value.unwrap_or(""));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "corpus" => {
                    call = call.corpus(value.unwrap_or(""));
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _files_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::File::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
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
            fn request_image_media_metadata_init(request: &mut api::File) {
                if request.image_media_metadata.is_none() {
                    request.image_media_metadata = Some(Default::default());
                }
            }
            
            fn request_image_media_metadata_location_init(request: &mut api::File) {
                request_image_media_metadata_init(request);
                if request.image_media_metadata.as_mut().unwrap().location.is_none() {
                    request.image_media_metadata.as_mut().unwrap().location = Some(Default::default());
                }
            }
            
            fn request_indexable_text_init(request: &mut api::File) {
                if request.indexable_text.is_none() {
                    request.indexable_text = Some(Default::default());
                }
            }
            
            fn request_labels_init(request: &mut api::File) {
                if request.labels.is_none() {
                    request.labels = Some(Default::default());
                }
            }
            
            fn request_last_modifying_user_init(request: &mut api::File) {
                if request.last_modifying_user.is_none() {
                    request.last_modifying_user = Some(Default::default());
                }
            }
            
            fn request_last_modifying_user_picture_init(request: &mut api::File) {
                request_last_modifying_user_init(request);
                if request.last_modifying_user.as_mut().unwrap().picture.is_none() {
                    request.last_modifying_user.as_mut().unwrap().picture = Some(Default::default());
                }
            }
            
            fn request_sharing_user_init(request: &mut api::File) {
                if request.sharing_user.is_none() {
                    request.sharing_user = Some(Default::default());
                }
            }
            
            fn request_sharing_user_picture_init(request: &mut api::File) {
                request_sharing_user_init(request);
                if request.sharing_user.as_mut().unwrap().picture.is_none() {
                    request.sharing_user.as_mut().unwrap().picture = Some(Default::default());
                }
            }
            
            fn request_thumbnail_init(request: &mut api::File) {
                if request.thumbnail.is_none() {
                    request.thumbnail = Some(Default::default());
                }
            }
            
            fn request_user_permission_init(request: &mut api::File) {
                if request.user_permission.is_none() {
                    request.user_permission = Some(Default::default());
                }
            }
            
            fn request_video_media_metadata_init(request: &mut api::File) {
                if request.video_media_metadata.is_none() {
                    request.video_media_metadata = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "mime-type" => {
                        request.mime_type = Some(value.unwrap_or("").to_string());
                    },
                "last-viewed-by-me-date" => {
                        request.last_viewed_by_me_date = Some(value.unwrap_or("").to_string());
                    },
                "app-data-contents" => {
                        request.app_data_contents = Some(arg_from_str(value.unwrap_or("false"), err, "app-data-contents", "boolean"));
                    },
                "thumbnail-link" => {
                        request.thumbnail_link = Some(value.unwrap_or("").to_string());
                    },
                "labels.restricted" => {
                        request_labels_init(&mut request);
                        request.labels.as_mut().unwrap().restricted = Some(arg_from_str(value.unwrap_or("false"), err, "labels.restricted", "boolean"));
                    },
                "labels.hidden" => {
                        request_labels_init(&mut request);
                        request.labels.as_mut().unwrap().hidden = Some(arg_from_str(value.unwrap_or("false"), err, "labels.hidden", "boolean"));
                    },
                "labels.viewed" => {
                        request_labels_init(&mut request);
                        request.labels.as_mut().unwrap().viewed = Some(arg_from_str(value.unwrap_or("false"), err, "labels.viewed", "boolean"));
                    },
                "labels.starred" => {
                        request_labels_init(&mut request);
                        request.labels.as_mut().unwrap().starred = Some(arg_from_str(value.unwrap_or("false"), err, "labels.starred", "boolean"));
                    },
                "labels.trashed" => {
                        request_labels_init(&mut request);
                        request.labels.as_mut().unwrap().trashed = Some(arg_from_str(value.unwrap_or("false"), err, "labels.trashed", "boolean"));
                    },
                "indexable-text.text" => {
                        request_indexable_text_init(&mut request);
                        request.indexable_text.as_mut().unwrap().text = Some(value.unwrap_or("").to_string());
                    },
                "explicitly-trashed" => {
                        request_indexable_text_init(&mut request);
                        request.explicitly_trashed = Some(arg_from_str(value.unwrap_or("false"), err, "explicitly-trashed", "boolean"));
                    },
                "etag" => {
                        request_indexable_text_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user-name" => {
                        request_indexable_text_init(&mut request);
                        request.last_modifying_user_name = Some(value.unwrap_or("").to_string());
                    },
                "writers-can-share" => {
                        request_indexable_text_init(&mut request);
                        request.writers_can_share = Some(arg_from_str(value.unwrap_or("false"), err, "writers-can-share", "boolean"));
                    },
                "id" => {
                        request_indexable_text_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "sharing-user.picture.url" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().picture.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "sharing-user.kind" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "sharing-user.display-name" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "sharing-user.permission-id" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().permission_id = Some(value.unwrap_or("").to_string());
                    },
                "sharing-user.is-authenticated-user" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().is_authenticated_user = Some(arg_from_str(value.unwrap_or("false"), err, "sharing-user.is-authenticated-user", "boolean"));
                    },
                "sharing-user.email-address" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().email_address = Some(value.unwrap_or("").to_string());
                    },
                "video-media-metadata.width" => {
                        request_video_media_metadata_init(&mut request);
                        request.video_media_metadata.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "video-media-metadata.width", "integer"));
                    },
                "video-media-metadata.duration-millis" => {
                        request_video_media_metadata_init(&mut request);
                        request.video_media_metadata.as_mut().unwrap().duration_millis = Some(value.unwrap_or("").to_string());
                    },
                "video-media-metadata.height" => {
                        request_video_media_metadata_init(&mut request);
                        request.video_media_metadata.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "video-media-metadata.height", "integer"));
                    },
                "last-modifying-user.picture.url" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().picture.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.kind" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.display-name" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.permission-id" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().permission_id = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.is-authenticated-user" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().is_authenticated_user = Some(arg_from_str(value.unwrap_or("false"), err, "last-modifying-user.is-authenticated-user", "boolean"));
                    },
                "last-modifying-user.email-address" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().email_address = Some(value.unwrap_or("").to_string());
                    },
                "copyable" => {
                        request_last_modifying_user_init(&mut request);
                        request.copyable = Some(arg_from_str(value.unwrap_or("false"), err, "copyable", "boolean"));
                    },
                "folder-color-rgb" => {
                        request_last_modifying_user_init(&mut request);
                        request.folder_color_rgb = Some(value.unwrap_or("").to_string());
                    },
                "owner-names" => {
                        request_last_modifying_user_init(&mut request);
                        if request.owner_names.is_none() {
                           request.owner_names = Some(Default::default());
                        }
                                        request.owner_names.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "shared-with-me-date" => {
                        request_last_modifying_user_init(&mut request);
                        request.shared_with_me_date = Some(value.unwrap_or("").to_string());
                    },
                "web-view-link" => {
                        request_last_modifying_user_init(&mut request);
                        request.web_view_link = Some(value.unwrap_or("").to_string());
                    },
                "version" => {
                        request_last_modifying_user_init(&mut request);
                        request.version = Some(value.unwrap_or("").to_string());
                    },
                "export-links" => {
                        request_last_modifying_user_init(&mut request);
                        if request.export_links.is_none() {
                           request.export_links = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.export_links.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "shared" => {
                        request_last_modifying_user_init(&mut request);
                        request.shared = Some(arg_from_str(value.unwrap_or("false"), err, "shared", "boolean"));
                    },
                "thumbnail.mime-type" => {
                        request_thumbnail_init(&mut request);
                        request.thumbnail.as_mut().unwrap().mime_type = Some(value.unwrap_or("").to_string());
                    },
                "thumbnail.image" => {
                        request_thumbnail_init(&mut request);
                        request.thumbnail.as_mut().unwrap().image = Some(value.unwrap_or("").to_string());
                    },
                "open-with-links" => {
                        request_thumbnail_init(&mut request);
                        if request.open_with_links.is_none() {
                           request.open_with_links = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.open_with_links.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "image-media-metadata.exposure-bias" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().exposure_bias = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.exposure-bias", "number"));
                    },
                "image-media-metadata.exposure-time" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().exposure_time = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.exposure-time", "number"));
                    },
                "image-media-metadata.max-aperture-value" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().max_aperture_value = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.max-aperture-value", "number"));
                    },
                "image-media-metadata.width" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "image-media-metadata.width", "integer"));
                    },
                "image-media-metadata.focal-length" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().focal_length = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.focal-length", "number"));
                    },
                "image-media-metadata.camera-make" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().camera_make = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.exposure-mode" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().exposure_mode = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.color-space" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().color_space = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.location.latitude" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().location.as_mut().unwrap().latitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.location.latitude", "number"));
                    },
                "image-media-metadata.location.altitude" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().location.as_mut().unwrap().altitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.location.altitude", "number"));
                    },
                "image-media-metadata.location.longitude" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().location.as_mut().unwrap().longitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.location.longitude", "number"));
                    },
                "image-media-metadata.subject-distance" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().subject_distance = Some(arg_from_str(value.unwrap_or("-0"), err, "image-media-metadata.subject-distance", "integer"));
                    },
                "image-media-metadata.height" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "image-media-metadata.height", "integer"));
                    },
                "image-media-metadata.lens" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().lens = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.date" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().date = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.iso-speed" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().iso_speed = Some(arg_from_str(value.unwrap_or("-0"), err, "image-media-metadata.iso-speed", "integer"));
                    },
                "image-media-metadata.metering-mode" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().metering_mode = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.flash-used" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().flash_used = Some(arg_from_str(value.unwrap_or("false"), err, "image-media-metadata.flash-used", "boolean"));
                    },
                "image-media-metadata.aperture" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().aperture = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.aperture", "number"));
                    },
                "image-media-metadata.rotation" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().rotation = Some(arg_from_str(value.unwrap_or("-0"), err, "image-media-metadata.rotation", "integer"));
                    },
                "image-media-metadata.sensor" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().sensor = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.white-balance" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().white_balance = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.camera-model" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().camera_model = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request_image_media_metadata_init(&mut request);
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "web-content-link" => {
                        request_image_media_metadata_init(&mut request);
                        request.web_content_link = Some(value.unwrap_or("").to_string());
                    },
                "editable" => {
                        request_image_media_metadata_init(&mut request);
                        request.editable = Some(arg_from_str(value.unwrap_or("false"), err, "editable", "boolean"));
                    },
                "embed-link" => {
                        request_image_media_metadata_init(&mut request);
                        request.embed_link = Some(value.unwrap_or("").to_string());
                    },
                "marked-viewed-by-me-date" => {
                        request_image_media_metadata_init(&mut request);
                        request.marked_viewed_by_me_date = Some(value.unwrap_or("").to_string());
                    },
                "quota-bytes-used" => {
                        request_image_media_metadata_init(&mut request);
                        request.quota_bytes_used = Some(value.unwrap_or("").to_string());
                    },
                "file-size" => {
                        request_image_media_metadata_init(&mut request);
                        request.file_size = Some(value.unwrap_or("").to_string());
                    },
                "created-date" => {
                        request_image_media_metadata_init(&mut request);
                        request.created_date = Some(value.unwrap_or("").to_string());
                    },
                "md5-checksum" => {
                        request_image_media_metadata_init(&mut request);
                        request.md5_checksum = Some(value.unwrap_or("").to_string());
                    },
                "icon-link" => {
                        request_image_media_metadata_init(&mut request);
                        request.icon_link = Some(value.unwrap_or("").to_string());
                    },
                "default-open-with-link" => {
                        request_image_media_metadata_init(&mut request);
                        request.default_open_with_link = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_image_media_metadata_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "alternate-link" => {
                        request_image_media_metadata_init(&mut request);
                        request.alternate_link = Some(value.unwrap_or("").to_string());
                    },
                "title" => {
                        request_image_media_metadata_init(&mut request);
                        request.title = Some(value.unwrap_or("").to_string());
                    },
                "modified-by-me-date" => {
                        request_image_media_metadata_init(&mut request);
                        request.modified_by_me_date = Some(value.unwrap_or("").to_string());
                    },
                "download-url" => {
                        request_image_media_metadata_init(&mut request);
                        request.download_url = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.with-link" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().with_link = Some(arg_from_str(value.unwrap_or("false"), err, "user-permission.with-link", "boolean"));
                    },
                "user-permission.domain" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().domain = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.name" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.kind" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.value" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.id" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.auth-key" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().auth_key = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.etag" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().etag = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.email-address" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().email_address = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.photo-link" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().photo_link = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.role" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().role = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.type" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.additional-roles" => {
                        request_user_permission_init(&mut request);
                        if request.user_permission.as_mut().unwrap().additional_roles.is_none() {
                           request.user_permission.as_mut().unwrap().additional_roles = Some(Default::default());
                        }
                                        request.user_permission.as_mut().unwrap().additional_roles.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "user-permission.self-link" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().self_link = Some(value.unwrap_or("").to_string());
                    },
                "original-filename" => {
                        request_user_permission_init(&mut request);
                        request.original_filename = Some(value.unwrap_or("").to_string());
                    },
                "file-extension" => {
                        request_user_permission_init(&mut request);
                        request.file_extension = Some(value.unwrap_or("").to_string());
                    },
                "head-revision-id" => {
                        request_user_permission_init(&mut request);
                        request.head_revision_id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_user_permission_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "modified-date" => {
                        request_user_permission_init(&mut request);
                        request.modified_date = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.files().patch(request, &self.opt.arg_file_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "use-content-as-indexable-text" => {
                    call = call.use_content_as_indexable_text(arg_from_str(value.unwrap_or("false"), err, "use-content-as-indexable-text", "boolean"));
                },
                "update-viewed-date" => {
                    call = call.update_viewed_date(arg_from_str(value.unwrap_or("false"), err, "update-viewed-date", "boolean"));
                },
                "timed-text-track-name" => {
                    call = call.timed_text_track_name(value.unwrap_or(""));
                },
                "timed-text-language" => {
                    call = call.timed_text_language(value.unwrap_or(""));
                },
                "set-modified-date" => {
                    call = call.set_modified_date(arg_from_str(value.unwrap_or("false"), err, "set-modified-date", "boolean"));
                },
                "remove-parents" => {
                    call = call.remove_parents(value.unwrap_or(""));
                },
                "pinned" => {
                    call = call.pinned(arg_from_str(value.unwrap_or("false"), err, "pinned", "boolean"));
                },
                "ocr-language" => {
                    call = call.ocr_language(value.unwrap_or(""));
                },
                "ocr" => {
                    call = call.ocr(arg_from_str(value.unwrap_or("false"), err, "ocr", "boolean"));
                },
                "new-revision" => {
                    call = call.new_revision(arg_from_str(value.unwrap_or("false"), err, "new-revision", "boolean"));
                },
                "convert" => {
                    call = call.convert(arg_from_str(value.unwrap_or("false"), err, "convert", "boolean"));
                },
                "add-parents" => {
                    call = call.add_parents(value.unwrap_or(""));
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _files_touch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.files().touch(&self.opt.arg_file_id);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _files_trash(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.files().trash(&self.opt.arg_file_id);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _files_untrash(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.files().untrash(&self.opt.arg_file_id);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _files_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::File::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
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
            fn request_image_media_metadata_init(request: &mut api::File) {
                if request.image_media_metadata.is_none() {
                    request.image_media_metadata = Some(Default::default());
                }
            }
            
            fn request_image_media_metadata_location_init(request: &mut api::File) {
                request_image_media_metadata_init(request);
                if request.image_media_metadata.as_mut().unwrap().location.is_none() {
                    request.image_media_metadata.as_mut().unwrap().location = Some(Default::default());
                }
            }
            
            fn request_indexable_text_init(request: &mut api::File) {
                if request.indexable_text.is_none() {
                    request.indexable_text = Some(Default::default());
                }
            }
            
            fn request_labels_init(request: &mut api::File) {
                if request.labels.is_none() {
                    request.labels = Some(Default::default());
                }
            }
            
            fn request_last_modifying_user_init(request: &mut api::File) {
                if request.last_modifying_user.is_none() {
                    request.last_modifying_user = Some(Default::default());
                }
            }
            
            fn request_last_modifying_user_picture_init(request: &mut api::File) {
                request_last_modifying_user_init(request);
                if request.last_modifying_user.as_mut().unwrap().picture.is_none() {
                    request.last_modifying_user.as_mut().unwrap().picture = Some(Default::default());
                }
            }
            
            fn request_sharing_user_init(request: &mut api::File) {
                if request.sharing_user.is_none() {
                    request.sharing_user = Some(Default::default());
                }
            }
            
            fn request_sharing_user_picture_init(request: &mut api::File) {
                request_sharing_user_init(request);
                if request.sharing_user.as_mut().unwrap().picture.is_none() {
                    request.sharing_user.as_mut().unwrap().picture = Some(Default::default());
                }
            }
            
            fn request_thumbnail_init(request: &mut api::File) {
                if request.thumbnail.is_none() {
                    request.thumbnail = Some(Default::default());
                }
            }
            
            fn request_user_permission_init(request: &mut api::File) {
                if request.user_permission.is_none() {
                    request.user_permission = Some(Default::default());
                }
            }
            
            fn request_video_media_metadata_init(request: &mut api::File) {
                if request.video_media_metadata.is_none() {
                    request.video_media_metadata = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "mime-type" => {
                        request.mime_type = Some(value.unwrap_or("").to_string());
                    },
                "last-viewed-by-me-date" => {
                        request.last_viewed_by_me_date = Some(value.unwrap_or("").to_string());
                    },
                "app-data-contents" => {
                        request.app_data_contents = Some(arg_from_str(value.unwrap_or("false"), err, "app-data-contents", "boolean"));
                    },
                "thumbnail-link" => {
                        request.thumbnail_link = Some(value.unwrap_or("").to_string());
                    },
                "labels.restricted" => {
                        request_labels_init(&mut request);
                        request.labels.as_mut().unwrap().restricted = Some(arg_from_str(value.unwrap_or("false"), err, "labels.restricted", "boolean"));
                    },
                "labels.hidden" => {
                        request_labels_init(&mut request);
                        request.labels.as_mut().unwrap().hidden = Some(arg_from_str(value.unwrap_or("false"), err, "labels.hidden", "boolean"));
                    },
                "labels.viewed" => {
                        request_labels_init(&mut request);
                        request.labels.as_mut().unwrap().viewed = Some(arg_from_str(value.unwrap_or("false"), err, "labels.viewed", "boolean"));
                    },
                "labels.starred" => {
                        request_labels_init(&mut request);
                        request.labels.as_mut().unwrap().starred = Some(arg_from_str(value.unwrap_or("false"), err, "labels.starred", "boolean"));
                    },
                "labels.trashed" => {
                        request_labels_init(&mut request);
                        request.labels.as_mut().unwrap().trashed = Some(arg_from_str(value.unwrap_or("false"), err, "labels.trashed", "boolean"));
                    },
                "indexable-text.text" => {
                        request_indexable_text_init(&mut request);
                        request.indexable_text.as_mut().unwrap().text = Some(value.unwrap_or("").to_string());
                    },
                "explicitly-trashed" => {
                        request_indexable_text_init(&mut request);
                        request.explicitly_trashed = Some(arg_from_str(value.unwrap_or("false"), err, "explicitly-trashed", "boolean"));
                    },
                "etag" => {
                        request_indexable_text_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user-name" => {
                        request_indexable_text_init(&mut request);
                        request.last_modifying_user_name = Some(value.unwrap_or("").to_string());
                    },
                "writers-can-share" => {
                        request_indexable_text_init(&mut request);
                        request.writers_can_share = Some(arg_from_str(value.unwrap_or("false"), err, "writers-can-share", "boolean"));
                    },
                "id" => {
                        request_indexable_text_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "sharing-user.picture.url" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().picture.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "sharing-user.kind" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "sharing-user.display-name" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "sharing-user.permission-id" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().permission_id = Some(value.unwrap_or("").to_string());
                    },
                "sharing-user.is-authenticated-user" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().is_authenticated_user = Some(arg_from_str(value.unwrap_or("false"), err, "sharing-user.is-authenticated-user", "boolean"));
                    },
                "sharing-user.email-address" => {
                        request_sharing_user_picture_init(&mut request);
                        request.sharing_user.as_mut().unwrap().email_address = Some(value.unwrap_or("").to_string());
                    },
                "video-media-metadata.width" => {
                        request_video_media_metadata_init(&mut request);
                        request.video_media_metadata.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "video-media-metadata.width", "integer"));
                    },
                "video-media-metadata.duration-millis" => {
                        request_video_media_metadata_init(&mut request);
                        request.video_media_metadata.as_mut().unwrap().duration_millis = Some(value.unwrap_or("").to_string());
                    },
                "video-media-metadata.height" => {
                        request_video_media_metadata_init(&mut request);
                        request.video_media_metadata.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "video-media-metadata.height", "integer"));
                    },
                "last-modifying-user.picture.url" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().picture.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.kind" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.display-name" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.permission-id" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().permission_id = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.is-authenticated-user" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().is_authenticated_user = Some(arg_from_str(value.unwrap_or("false"), err, "last-modifying-user.is-authenticated-user", "boolean"));
                    },
                "last-modifying-user.email-address" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().email_address = Some(value.unwrap_or("").to_string());
                    },
                "copyable" => {
                        request_last_modifying_user_init(&mut request);
                        request.copyable = Some(arg_from_str(value.unwrap_or("false"), err, "copyable", "boolean"));
                    },
                "folder-color-rgb" => {
                        request_last_modifying_user_init(&mut request);
                        request.folder_color_rgb = Some(value.unwrap_or("").to_string());
                    },
                "owner-names" => {
                        request_last_modifying_user_init(&mut request);
                        if request.owner_names.is_none() {
                           request.owner_names = Some(Default::default());
                        }
                                        request.owner_names.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "shared-with-me-date" => {
                        request_last_modifying_user_init(&mut request);
                        request.shared_with_me_date = Some(value.unwrap_or("").to_string());
                    },
                "web-view-link" => {
                        request_last_modifying_user_init(&mut request);
                        request.web_view_link = Some(value.unwrap_or("").to_string());
                    },
                "version" => {
                        request_last_modifying_user_init(&mut request);
                        request.version = Some(value.unwrap_or("").to_string());
                    },
                "export-links" => {
                        request_last_modifying_user_init(&mut request);
                        if request.export_links.is_none() {
                           request.export_links = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.export_links.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "shared" => {
                        request_last_modifying_user_init(&mut request);
                        request.shared = Some(arg_from_str(value.unwrap_or("false"), err, "shared", "boolean"));
                    },
                "thumbnail.mime-type" => {
                        request_thumbnail_init(&mut request);
                        request.thumbnail.as_mut().unwrap().mime_type = Some(value.unwrap_or("").to_string());
                    },
                "thumbnail.image" => {
                        request_thumbnail_init(&mut request);
                        request.thumbnail.as_mut().unwrap().image = Some(value.unwrap_or("").to_string());
                    },
                "open-with-links" => {
                        request_thumbnail_init(&mut request);
                        if request.open_with_links.is_none() {
                           request.open_with_links = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.open_with_links.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "image-media-metadata.exposure-bias" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().exposure_bias = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.exposure-bias", "number"));
                    },
                "image-media-metadata.exposure-time" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().exposure_time = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.exposure-time", "number"));
                    },
                "image-media-metadata.max-aperture-value" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().max_aperture_value = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.max-aperture-value", "number"));
                    },
                "image-media-metadata.width" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "image-media-metadata.width", "integer"));
                    },
                "image-media-metadata.focal-length" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().focal_length = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.focal-length", "number"));
                    },
                "image-media-metadata.camera-make" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().camera_make = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.exposure-mode" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().exposure_mode = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.color-space" => {
                        request_image_media_metadata_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().color_space = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.location.latitude" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().location.as_mut().unwrap().latitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.location.latitude", "number"));
                    },
                "image-media-metadata.location.altitude" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().location.as_mut().unwrap().altitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.location.altitude", "number"));
                    },
                "image-media-metadata.location.longitude" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().location.as_mut().unwrap().longitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.location.longitude", "number"));
                    },
                "image-media-metadata.subject-distance" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().subject_distance = Some(arg_from_str(value.unwrap_or("-0"), err, "image-media-metadata.subject-distance", "integer"));
                    },
                "image-media-metadata.height" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "image-media-metadata.height", "integer"));
                    },
                "image-media-metadata.lens" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().lens = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.date" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().date = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.iso-speed" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().iso_speed = Some(arg_from_str(value.unwrap_or("-0"), err, "image-media-metadata.iso-speed", "integer"));
                    },
                "image-media-metadata.metering-mode" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().metering_mode = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.flash-used" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().flash_used = Some(arg_from_str(value.unwrap_or("false"), err, "image-media-metadata.flash-used", "boolean"));
                    },
                "image-media-metadata.aperture" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().aperture = Some(arg_from_str(value.unwrap_or("0.0"), err, "image-media-metadata.aperture", "number"));
                    },
                "image-media-metadata.rotation" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().rotation = Some(arg_from_str(value.unwrap_or("-0"), err, "image-media-metadata.rotation", "integer"));
                    },
                "image-media-metadata.sensor" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().sensor = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.white-balance" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().white_balance = Some(value.unwrap_or("").to_string());
                    },
                "image-media-metadata.camera-model" => {
                        request_image_media_metadata_location_init(&mut request);
                        request.image_media_metadata.as_mut().unwrap().camera_model = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request_image_media_metadata_init(&mut request);
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "web-content-link" => {
                        request_image_media_metadata_init(&mut request);
                        request.web_content_link = Some(value.unwrap_or("").to_string());
                    },
                "editable" => {
                        request_image_media_metadata_init(&mut request);
                        request.editable = Some(arg_from_str(value.unwrap_or("false"), err, "editable", "boolean"));
                    },
                "embed-link" => {
                        request_image_media_metadata_init(&mut request);
                        request.embed_link = Some(value.unwrap_or("").to_string());
                    },
                "marked-viewed-by-me-date" => {
                        request_image_media_metadata_init(&mut request);
                        request.marked_viewed_by_me_date = Some(value.unwrap_or("").to_string());
                    },
                "quota-bytes-used" => {
                        request_image_media_metadata_init(&mut request);
                        request.quota_bytes_used = Some(value.unwrap_or("").to_string());
                    },
                "file-size" => {
                        request_image_media_metadata_init(&mut request);
                        request.file_size = Some(value.unwrap_or("").to_string());
                    },
                "created-date" => {
                        request_image_media_metadata_init(&mut request);
                        request.created_date = Some(value.unwrap_or("").to_string());
                    },
                "md5-checksum" => {
                        request_image_media_metadata_init(&mut request);
                        request.md5_checksum = Some(value.unwrap_or("").to_string());
                    },
                "icon-link" => {
                        request_image_media_metadata_init(&mut request);
                        request.icon_link = Some(value.unwrap_or("").to_string());
                    },
                "default-open-with-link" => {
                        request_image_media_metadata_init(&mut request);
                        request.default_open_with_link = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_image_media_metadata_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "alternate-link" => {
                        request_image_media_metadata_init(&mut request);
                        request.alternate_link = Some(value.unwrap_or("").to_string());
                    },
                "title" => {
                        request_image_media_metadata_init(&mut request);
                        request.title = Some(value.unwrap_or("").to_string());
                    },
                "modified-by-me-date" => {
                        request_image_media_metadata_init(&mut request);
                        request.modified_by_me_date = Some(value.unwrap_or("").to_string());
                    },
                "download-url" => {
                        request_image_media_metadata_init(&mut request);
                        request.download_url = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.with-link" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().with_link = Some(arg_from_str(value.unwrap_or("false"), err, "user-permission.with-link", "boolean"));
                    },
                "user-permission.domain" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().domain = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.name" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.kind" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.value" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.id" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.auth-key" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().auth_key = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.etag" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().etag = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.email-address" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().email_address = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.photo-link" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().photo_link = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.role" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().role = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.type" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "user-permission.additional-roles" => {
                        request_user_permission_init(&mut request);
                        if request.user_permission.as_mut().unwrap().additional_roles.is_none() {
                           request.user_permission.as_mut().unwrap().additional_roles = Some(Default::default());
                        }
                                        request.user_permission.as_mut().unwrap().additional_roles.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "user-permission.self-link" => {
                        request_user_permission_init(&mut request);
                        request.user_permission.as_mut().unwrap().self_link = Some(value.unwrap_or("").to_string());
                    },
                "original-filename" => {
                        request_user_permission_init(&mut request);
                        request.original_filename = Some(value.unwrap_or("").to_string());
                    },
                "file-extension" => {
                        request_user_permission_init(&mut request);
                        request.file_extension = Some(value.unwrap_or("").to_string());
                    },
                "head-revision-id" => {
                        request_user_permission_init(&mut request);
                        request.head_revision_id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_user_permission_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "modified-date" => {
                        request_user_permission_init(&mut request);
                        request.modified_date = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.files().update(request, &self.opt.arg_file_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "use-content-as-indexable-text" => {
                    call = call.use_content_as_indexable_text(arg_from_str(value.unwrap_or("false"), err, "use-content-as-indexable-text", "boolean"));
                },
                "update-viewed-date" => {
                    call = call.update_viewed_date(arg_from_str(value.unwrap_or("false"), err, "update-viewed-date", "boolean"));
                },
                "timed-text-track-name" => {
                    call = call.timed_text_track_name(value.unwrap_or(""));
                },
                "timed-text-language" => {
                    call = call.timed_text_language(value.unwrap_or(""));
                },
                "set-modified-date" => {
                    call = call.set_modified_date(arg_from_str(value.unwrap_or("false"), err, "set-modified-date", "boolean"));
                },
                "remove-parents" => {
                    call = call.remove_parents(value.unwrap_or(""));
                },
                "pinned" => {
                    call = call.pinned(arg_from_str(value.unwrap_or("false"), err, "pinned", "boolean"));
                },
                "ocr-language" => {
                    call = call.ocr_language(value.unwrap_or(""));
                },
                "ocr" => {
                    call = call.ocr(arg_from_str(value.unwrap_or("false"), err, "ocr", "boolean"));
                },
                "new-revision" => {
                    call = call.new_revision(arg_from_str(value.unwrap_or("false"), err, "new-revision", "boolean"));
                },
                "convert" => {
                    call = call.convert(arg_from_str(value.unwrap_or("false"), err, "convert", "boolean"));
                },
                "add-parents" => {
                    call = call.add_parents(value.unwrap_or(""));
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
        let protocol = 
            if self.opt.cmd_simple {
                "simple"
            } else if self.opt.cmd_resumable {
                "resumable"
            } else { 
                unreachable!() 
            };
        let mut input_file = input_file_from_opts(&self.opt.arg_file, err);
        let mime_type = input_mime_from_opts(&self.opt.arg_mime, err);
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "simple" => call.upload(input_file.unwrap(), mime_type.unwrap()),
                "resumable" => call.upload_resumable(input_file.unwrap(), mime_type.unwrap()),
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

    fn _files_watch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Channel::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
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
                "resource-uri" => {
                        request.resource_uri = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "resource-id" => {
                        request.resource_id = Some(value.unwrap_or("").to_string());
                    },
                "payload" => {
                        request.payload = Some(arg_from_str(value.unwrap_or("false"), err, "payload", "boolean"));
                    },
                "token" => {
                        request.token = Some(value.unwrap_or("").to_string());
                    },
                "params" => {
                        if request.params.is_none() {
                           request.params = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.params.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "expiration" => {
                        request.expiration = Some(value.unwrap_or("").to_string());
                    },
                "address" => {
                        request.address = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut download_mode = false;
        let mut call = self.hub.files().watch(request, &self.opt.arg_file_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-viewed-date" => {
                    call = call.update_viewed_date(arg_from_str(value.unwrap_or("false"), err, "update-viewed-date", "boolean"));
                },
                "revision-id" => {
                    call = call.revision_id(value.unwrap_or(""));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "acknowledge-abuse" => {
                    call = call.acknowledge_abuse(arg_from_str(value.unwrap_or("false"), err, "acknowledge-abuse", "boolean"));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    if key == "alt" && value.unwrap_or("unset") == "media" {
                        download_mode = true;
                    }
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok((mut response, output_schema)) => {
                    if !download_mode {
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    } else {
                    io::copy(&mut response, &mut ostream).unwrap();
                    }
                    None
                }
            }
        }
    }

    fn _parents_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.parents().delete(&self.opt.arg_file_id, &self.opt.arg_parent_id);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _parents_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.parents().get(&self.opt.arg_file_id, &self.opt.arg_parent_id);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _parents_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::ParentReference::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
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
                "self-link" => {
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "is-root" => {
                        request.is_root = Some(arg_from_str(value.unwrap_or("false"), err, "is-root", "boolean"));
                    },
                "parent-link" => {
                        request.parent_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.parents().insert(request, &self.opt.arg_file_id);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _parents_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.parents().list(&self.opt.arg_file_id);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _permissions_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.permissions().delete(&self.opt.arg_file_id, &self.opt.arg_permission_id);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _permissions_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.permissions().get(&self.opt.arg_file_id, &self.opt.arg_permission_id);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _permissions_get_id_for_email(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.permissions().get_id_for_email(&self.opt.arg_email);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _permissions_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Permission::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
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
                "with-link" => {
                        request.with_link = Some(arg_from_str(value.unwrap_or("false"), err, "with-link", "boolean"));
                    },
                "domain" => {
                        request.domain = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "value" => {
                        request.value = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "auth-key" => {
                        request.auth_key = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "email-address" => {
                        request.email_address = Some(value.unwrap_or("").to_string());
                    },
                "photo-link" => {
                        request.photo_link = Some(value.unwrap_or("").to_string());
                    },
                "role" => {
                        request.role = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "additional-roles" => {
                        if request.additional_roles.is_none() {
                           request.additional_roles = Some(Default::default());
                        }
                                        request.additional_roles.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.permissions().insert(request, &self.opt.arg_file_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "send-notification-emails" => {
                    call = call.send_notification_emails(arg_from_str(value.unwrap_or("false"), err, "send-notification-emails", "boolean"));
                },
                "email-message" => {
                    call = call.email_message(value.unwrap_or(""));
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _permissions_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.permissions().list(&self.opt.arg_file_id);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _permissions_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Permission::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
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
                "with-link" => {
                        request.with_link = Some(arg_from_str(value.unwrap_or("false"), err, "with-link", "boolean"));
                    },
                "domain" => {
                        request.domain = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "value" => {
                        request.value = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "auth-key" => {
                        request.auth_key = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "email-address" => {
                        request.email_address = Some(value.unwrap_or("").to_string());
                    },
                "photo-link" => {
                        request.photo_link = Some(value.unwrap_or("").to_string());
                    },
                "role" => {
                        request.role = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "additional-roles" => {
                        if request.additional_roles.is_none() {
                           request.additional_roles = Some(Default::default());
                        }
                                        request.additional_roles.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.permissions().patch(request, &self.opt.arg_file_id, &self.opt.arg_permission_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "transfer-ownership" => {
                    call = call.transfer_ownership(arg_from_str(value.unwrap_or("false"), err, "transfer-ownership", "boolean"));
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _permissions_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Permission::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
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
                "with-link" => {
                        request.with_link = Some(arg_from_str(value.unwrap_or("false"), err, "with-link", "boolean"));
                    },
                "domain" => {
                        request.domain = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "value" => {
                        request.value = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "auth-key" => {
                        request.auth_key = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "email-address" => {
                        request.email_address = Some(value.unwrap_or("").to_string());
                    },
                "photo-link" => {
                        request.photo_link = Some(value.unwrap_or("").to_string());
                    },
                "role" => {
                        request.role = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "additional-roles" => {
                        if request.additional_roles.is_none() {
                           request.additional_roles = Some(Default::default());
                        }
                                        request.additional_roles.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.permissions().update(request, &self.opt.arg_file_id, &self.opt.arg_permission_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "transfer-ownership" => {
                    call = call.transfer_ownership(arg_from_str(value.unwrap_or("false"), err, "transfer-ownership", "boolean"));
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _properties_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.properties().delete(&self.opt.arg_file_id, &self.opt.arg_property_key);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "visibility" => {
                    call = call.visibility(value.unwrap_or(""));
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _properties_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.properties().get(&self.opt.arg_file_id, &self.opt.arg_property_key);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "visibility" => {
                    call = call.visibility(value.unwrap_or(""));
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _properties_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Property::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
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
                "value" => {
                        request.value = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "visibility" => {
                        request.visibility = Some(value.unwrap_or("").to_string());
                    },
                "key" => {
                        request.key = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.properties().insert(request, &self.opt.arg_file_id);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _properties_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.properties().list(&self.opt.arg_file_id);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _properties_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Property::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
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
                "value" => {
                        request.value = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "visibility" => {
                        request.visibility = Some(value.unwrap_or("").to_string());
                    },
                "key" => {
                        request.key = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.properties().patch(request, &self.opt.arg_file_id, &self.opt.arg_property_key);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "visibility" => {
                    call = call.visibility(value.unwrap_or(""));
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _properties_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Property::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
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
                "value" => {
                        request.value = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "visibility" => {
                        request.visibility = Some(value.unwrap_or("").to_string());
                    },
                "key" => {
                        request.key = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.properties().update(request, &self.opt.arg_file_id, &self.opt.arg_property_key);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "visibility" => {
                    call = call.visibility(value.unwrap_or(""));
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _realtime_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut download_mode = false;
        let mut call = self.hub.realtime().get(&self.opt.arg_file_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "revision" => {
                    call = call.revision(arg_from_str(value.unwrap_or("-0"), err, "revision", "integer"));
                },
                "alt"
                |"fields"
                |"key"
                |"oauth-token"
                |"pretty-print"
                |"quota-user"
                |"user-ip" => {
                    if key == "alt" && value.unwrap_or("unset") == "media" {
                        download_mode = true;
                    }
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok(mut response) => {
                    if !download_mode {
                    } else {
                    io::copy(&mut response, &mut ostream).unwrap();
                    }
                    None
                }
            }
        }
    }

    fn _realtime_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.realtime().update(&self.opt.arg_file_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "base-revision" => {
                    call = call.base_revision(value.unwrap_or(""));
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
        let protocol = 
            if self.opt.cmd_simple {
                "simple"
            } else if self.opt.cmd_resumable {
                "resumable"
            } else { 
                unreachable!() 
            };
        let mut input_file = input_file_from_opts(&self.opt.arg_file, err);
        let mime_type = input_mime_from_opts(&self.opt.arg_mime, err);
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
            match match protocol {
                "simple" => call.upload(input_file.unwrap(), mime_type.unwrap()),
                "resumable" => call.upload_resumable(input_file.unwrap(), mime_type.unwrap()),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok(mut response) => {
                    None
                }
            }
        }
    }

    fn _replies_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.replies().delete(&self.opt.arg_file_id, &self.opt.arg_comment_id, &self.opt.arg_reply_id);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _replies_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.replies().get(&self.opt.arg_file_id, &self.opt.arg_comment_id, &self.opt.arg_reply_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "include-deleted" => {
                    call = call.include_deleted(arg_from_str(value.unwrap_or("false"), err, "include-deleted", "boolean"));
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _replies_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::CommentReply::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
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
            fn request_author_init(request: &mut api::CommentReply) {
                if request.author.is_none() {
                    request.author = Some(Default::default());
                }
            }
            
            fn request_author_picture_init(request: &mut api::CommentReply) {
                request_author_init(request);
                if request.author.as_mut().unwrap().picture.is_none() {
                    request.author.as_mut().unwrap().picture = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "author.picture.url" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().picture.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "author.kind" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "author.display-name" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "author.permission-id" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().permission_id = Some(value.unwrap_or("").to_string());
                    },
                "author.is-authenticated-user" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().is_authenticated_user = Some(arg_from_str(value.unwrap_or("false"), err, "author.is-authenticated-user", "boolean"));
                    },
                "author.email-address" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().email_address = Some(value.unwrap_or("").to_string());
                    },
                "deleted" => {
                        request_author_init(&mut request);
                        request.deleted = Some(arg_from_str(value.unwrap_or("false"), err, "deleted", "boolean"));
                    },
                "html-content" => {
                        request_author_init(&mut request);
                        request.html_content = Some(value.unwrap_or("").to_string());
                    },
                "content" => {
                        request_author_init(&mut request);
                        request.content = Some(value.unwrap_or("").to_string());
                    },
                "verb" => {
                        request_author_init(&mut request);
                        request.verb = Some(value.unwrap_or("").to_string());
                    },
                "reply-id" => {
                        request_author_init(&mut request);
                        request.reply_id = Some(value.unwrap_or("").to_string());
                    },
                "modified-date" => {
                        request_author_init(&mut request);
                        request.modified_date = Some(value.unwrap_or("").to_string());
                    },
                "created-date" => {
                        request_author_init(&mut request);
                        request.created_date = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.replies().insert(request, &self.opt.arg_file_id, &self.opt.arg_comment_id);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _replies_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.replies().list(&self.opt.arg_file_id, &self.opt.arg_comment_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "include-deleted" => {
                    call = call.include_deleted(arg_from_str(value.unwrap_or("false"), err, "include-deleted", "boolean"));
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _replies_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::CommentReply::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
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
            fn request_author_init(request: &mut api::CommentReply) {
                if request.author.is_none() {
                    request.author = Some(Default::default());
                }
            }
            
            fn request_author_picture_init(request: &mut api::CommentReply) {
                request_author_init(request);
                if request.author.as_mut().unwrap().picture.is_none() {
                    request.author.as_mut().unwrap().picture = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "author.picture.url" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().picture.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "author.kind" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "author.display-name" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "author.permission-id" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().permission_id = Some(value.unwrap_or("").to_string());
                    },
                "author.is-authenticated-user" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().is_authenticated_user = Some(arg_from_str(value.unwrap_or("false"), err, "author.is-authenticated-user", "boolean"));
                    },
                "author.email-address" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().email_address = Some(value.unwrap_or("").to_string());
                    },
                "deleted" => {
                        request_author_init(&mut request);
                        request.deleted = Some(arg_from_str(value.unwrap_or("false"), err, "deleted", "boolean"));
                    },
                "html-content" => {
                        request_author_init(&mut request);
                        request.html_content = Some(value.unwrap_or("").to_string());
                    },
                "content" => {
                        request_author_init(&mut request);
                        request.content = Some(value.unwrap_or("").to_string());
                    },
                "verb" => {
                        request_author_init(&mut request);
                        request.verb = Some(value.unwrap_or("").to_string());
                    },
                "reply-id" => {
                        request_author_init(&mut request);
                        request.reply_id = Some(value.unwrap_or("").to_string());
                    },
                "modified-date" => {
                        request_author_init(&mut request);
                        request.modified_date = Some(value.unwrap_or("").to_string());
                    },
                "created-date" => {
                        request_author_init(&mut request);
                        request.created_date = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.replies().patch(request, &self.opt.arg_file_id, &self.opt.arg_comment_id, &self.opt.arg_reply_id);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _replies_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::CommentReply::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
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
            fn request_author_init(request: &mut api::CommentReply) {
                if request.author.is_none() {
                    request.author = Some(Default::default());
                }
            }
            
            fn request_author_picture_init(request: &mut api::CommentReply) {
                request_author_init(request);
                if request.author.as_mut().unwrap().picture.is_none() {
                    request.author.as_mut().unwrap().picture = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "author.picture.url" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().picture.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "author.kind" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "author.display-name" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "author.permission-id" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().permission_id = Some(value.unwrap_or("").to_string());
                    },
                "author.is-authenticated-user" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().is_authenticated_user = Some(arg_from_str(value.unwrap_or("false"), err, "author.is-authenticated-user", "boolean"));
                    },
                "author.email-address" => {
                        request_author_picture_init(&mut request);
                        request.author.as_mut().unwrap().email_address = Some(value.unwrap_or("").to_string());
                    },
                "deleted" => {
                        request_author_init(&mut request);
                        request.deleted = Some(arg_from_str(value.unwrap_or("false"), err, "deleted", "boolean"));
                    },
                "html-content" => {
                        request_author_init(&mut request);
                        request.html_content = Some(value.unwrap_or("").to_string());
                    },
                "content" => {
                        request_author_init(&mut request);
                        request.content = Some(value.unwrap_or("").to_string());
                    },
                "verb" => {
                        request_author_init(&mut request);
                        request.verb = Some(value.unwrap_or("").to_string());
                    },
                "reply-id" => {
                        request_author_init(&mut request);
                        request.reply_id = Some(value.unwrap_or("").to_string());
                    },
                "modified-date" => {
                        request_author_init(&mut request);
                        request.modified_date = Some(value.unwrap_or("").to_string());
                    },
                "created-date" => {
                        request_author_init(&mut request);
                        request.created_date = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.replies().update(request, &self.opt.arg_file_id, &self.opt.arg_comment_id, &self.opt.arg_reply_id);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _revisions_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.revisions().delete(&self.opt.arg_file_id, &self.opt.arg_revision_id);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _revisions_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.revisions().get(&self.opt.arg_file_id, &self.opt.arg_revision_id);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _revisions_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.revisions().list(&self.opt.arg_file_id);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _revisions_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Revision::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
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
            fn request_last_modifying_user_init(request: &mut api::Revision) {
                if request.last_modifying_user.is_none() {
                    request.last_modifying_user = Some(Default::default());
                }
            }
            
            fn request_last_modifying_user_picture_init(request: &mut api::Revision) {
                request_last_modifying_user_init(request);
                if request.last_modifying_user.as_mut().unwrap().picture.is_none() {
                    request.last_modifying_user.as_mut().unwrap().picture = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "mime-type" => {
                        request.mime_type = Some(value.unwrap_or("").to_string());
                    },
                "pinned" => {
                        request.pinned = Some(arg_from_str(value.unwrap_or("false"), err, "pinned", "boolean"));
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "publish-auto" => {
                        request.publish_auto = Some(arg_from_str(value.unwrap_or("false"), err, "publish-auto", "boolean"));
                    },
                "published-outside-domain" => {
                        request.published_outside_domain = Some(arg_from_str(value.unwrap_or("false"), err, "published-outside-domain", "boolean"));
                    },
                "last-modifying-user.picture.url" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().picture.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.kind" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.display-name" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.permission-id" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().permission_id = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.is-authenticated-user" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().is_authenticated_user = Some(arg_from_str(value.unwrap_or("false"), err, "last-modifying-user.is-authenticated-user", "boolean"));
                    },
                "last-modifying-user.email-address" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().email_address = Some(value.unwrap_or("").to_string());
                    },
                "published-link" => {
                        request_last_modifying_user_init(&mut request);
                        request.published_link = Some(value.unwrap_or("").to_string());
                    },
                "download-url" => {
                        request_last_modifying_user_init(&mut request);
                        request.download_url = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_last_modifying_user_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_last_modifying_user_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "file-size" => {
                        request_last_modifying_user_init(&mut request);
                        request.file_size = Some(value.unwrap_or("").to_string());
                    },
                "export-links" => {
                        request_last_modifying_user_init(&mut request);
                        if request.export_links.is_none() {
                           request.export_links = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.export_links.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "last-modifying-user-name" => {
                        request_last_modifying_user_init(&mut request);
                        request.last_modifying_user_name = Some(value.unwrap_or("").to_string());
                    },
                "modified-date" => {
                        request_last_modifying_user_init(&mut request);
                        request.modified_date = Some(value.unwrap_or("").to_string());
                    },
                "original-filename" => {
                        request_last_modifying_user_init(&mut request);
                        request.original_filename = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_last_modifying_user_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "md5-checksum" => {
                        request_last_modifying_user_init(&mut request);
                        request.md5_checksum = Some(value.unwrap_or("").to_string());
                    },
                "published" => {
                        request_last_modifying_user_init(&mut request);
                        request.published = Some(arg_from_str(value.unwrap_or("false"), err, "published", "boolean"));
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.revisions().patch(request, &self.opt.arg_file_id, &self.opt.arg_revision_id);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _revisions_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Revision::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
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
            fn request_last_modifying_user_init(request: &mut api::Revision) {
                if request.last_modifying_user.is_none() {
                    request.last_modifying_user = Some(Default::default());
                }
            }
            
            fn request_last_modifying_user_picture_init(request: &mut api::Revision) {
                request_last_modifying_user_init(request);
                if request.last_modifying_user.as_mut().unwrap().picture.is_none() {
                    request.last_modifying_user.as_mut().unwrap().picture = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "mime-type" => {
                        request.mime_type = Some(value.unwrap_or("").to_string());
                    },
                "pinned" => {
                        request.pinned = Some(arg_from_str(value.unwrap_or("false"), err, "pinned", "boolean"));
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "publish-auto" => {
                        request.publish_auto = Some(arg_from_str(value.unwrap_or("false"), err, "publish-auto", "boolean"));
                    },
                "published-outside-domain" => {
                        request.published_outside_domain = Some(arg_from_str(value.unwrap_or("false"), err, "published-outside-domain", "boolean"));
                    },
                "last-modifying-user.picture.url" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().picture.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.kind" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.display-name" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.permission-id" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().permission_id = Some(value.unwrap_or("").to_string());
                    },
                "last-modifying-user.is-authenticated-user" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().is_authenticated_user = Some(arg_from_str(value.unwrap_or("false"), err, "last-modifying-user.is-authenticated-user", "boolean"));
                    },
                "last-modifying-user.email-address" => {
                        request_last_modifying_user_picture_init(&mut request);
                        request.last_modifying_user.as_mut().unwrap().email_address = Some(value.unwrap_or("").to_string());
                    },
                "published-link" => {
                        request_last_modifying_user_init(&mut request);
                        request.published_link = Some(value.unwrap_or("").to_string());
                    },
                "download-url" => {
                        request_last_modifying_user_init(&mut request);
                        request.download_url = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_last_modifying_user_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_last_modifying_user_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "file-size" => {
                        request_last_modifying_user_init(&mut request);
                        request.file_size = Some(value.unwrap_or("").to_string());
                    },
                "export-links" => {
                        request_last_modifying_user_init(&mut request);
                        if request.export_links.is_none() {
                           request.export_links = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.export_links.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "last-modifying-user-name" => {
                        request_last_modifying_user_init(&mut request);
                        request.last_modifying_user_name = Some(value.unwrap_or("").to_string());
                    },
                "modified-date" => {
                        request_last_modifying_user_init(&mut request);
                        request.modified_date = Some(value.unwrap_or("").to_string());
                    },
                "original-filename" => {
                        request_last_modifying_user_init(&mut request);
                        request.original_filename = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_last_modifying_user_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "md5-checksum" => {
                        request_last_modifying_user_init(&mut request);
                        request.md5_checksum = Some(value.unwrap_or("").to_string());
                    },
                "published" => {
                        request_last_modifying_user_init(&mut request);
                        request.published = Some(arg_from_str(value.unwrap_or("false"), err, "published", "boolean"));
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.revisions().update(request, &self.opt.arg_file_id, &self.opt.arg_revision_id);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

        if self.opt.cmd_about {
            if self.opt.cmd_get {
                call_result = self._about_get(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_apps {
            if self.opt.cmd_get {
                call_result = self._apps_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._apps_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_changes {
            if self.opt.cmd_get {
                call_result = self._changes_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._changes_list(dry_run, &mut err);
            } else if self.opt.cmd_watch {
                call_result = self._changes_watch(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_channels {
            if self.opt.cmd_stop {
                call_result = self._channels_stop(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_children {
            if self.opt.cmd_delete {
                call_result = self._children_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._children_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._children_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._children_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_comments {
            if self.opt.cmd_delete {
                call_result = self._comments_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._comments_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._comments_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._comments_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._comments_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._comments_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_files {
            if self.opt.cmd_copy {
                call_result = self._files_copy(dry_run, &mut err);
            } else if self.opt.cmd_delete {
                call_result = self._files_delete(dry_run, &mut err);
            } else if self.opt.cmd_empty_trash {
                call_result = self._files_empty_trash(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._files_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._files_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._files_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._files_patch(dry_run, &mut err);
            } else if self.opt.cmd_touch {
                call_result = self._files_touch(dry_run, &mut err);
            } else if self.opt.cmd_trash {
                call_result = self._files_trash(dry_run, &mut err);
            } else if self.opt.cmd_untrash {
                call_result = self._files_untrash(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._files_update(dry_run, &mut err);
            } else if self.opt.cmd_watch {
                call_result = self._files_watch(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_parents {
            if self.opt.cmd_delete {
                call_result = self._parents_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._parents_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._parents_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._parents_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_permissions {
            if self.opt.cmd_delete {
                call_result = self._permissions_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._permissions_get(dry_run, &mut err);
            } else if self.opt.cmd_get_id_for_email {
                call_result = self._permissions_get_id_for_email(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._permissions_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._permissions_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._permissions_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._permissions_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_properties {
            if self.opt.cmd_delete {
                call_result = self._properties_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._properties_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._properties_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._properties_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._properties_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._properties_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_realtime {
            if self.opt.cmd_get {
                call_result = self._realtime_get(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._realtime_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_replies {
            if self.opt.cmd_delete {
                call_result = self._replies_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._replies_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._replies_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._replies_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._replies_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._replies_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_revisions {
            if self.opt.cmd_delete {
                call_result = self._revisions_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._revisions_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._revisions_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._revisions_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._revisions_update(dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "drive2-secret.json", 
                                                         "{\"installed\":{\"auth_uri\":\"https://accounts.google.com/o/oauth2/auth\",\"client_secret\":\"De0ub0IbWruJbBXUyseFYvZ-\",\"token_uri\":\"https://accounts.google.com/o/oauth2/token\",\"client_email\":\"\",\"redirect_uris\":[\"urn:ietf:wg:oauth:2.0:oob\",\"oob\"],\"client_x509_cert_url\":\"\",\"client_id\":\"276875258587-5gbp23a7aqnrl6p06c0jt5fskuktactq.apps.googleusercontent.com\",\"auth_provider_x509_cert_url\":\"https://www.googleapis.com/oauth2/v1/certs\"}}") {
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
                                          program_name: "drive2",
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
            hub: api::Drive::new(client, auth),
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
    let debug = opts.flag_debug;
    match Engine::new(opts) {
        Err(err) => {
            writeln!(io::stderr(), "{}", err).ok();
            env::set_exit_status(err.exit_code);
        },
        Ok(engine) => {
            if let Some(err) = engine.doit() {
                if debug {
                    writeln!(io::stderr(), "{:?}", err).ok();
                } else {
                    writeln!(io::stderr(), "{}", err).ok();
                }
                env::set_exit_status(1);
            }
        }
    }
}