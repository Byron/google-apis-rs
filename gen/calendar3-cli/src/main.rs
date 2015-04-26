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
extern crate google_calendar3 as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  calendar3 [options] acl delete <calendar-id> <rule-id> [-p <v>...]
  calendar3 [options] acl get <calendar-id> <rule-id> [-p <v>...] [-o <out>]
  calendar3 [options] acl insert <calendar-id> -r <kv>... [-p <v>...] [-o <out>]
  calendar3 [options] acl list <calendar-id> [-p <v>...] [-o <out>]
  calendar3 [options] acl patch <calendar-id> <rule-id> -r <kv>... [-p <v>...] [-o <out>]
  calendar3 [options] acl update <calendar-id> <rule-id> -r <kv>... [-p <v>...] [-o <out>]
  calendar3 [options] acl watch <calendar-id> -r <kv>... [-p <v>...] [-o <out>]
  calendar3 [options] calendar-list delete <calendar-id> [-p <v>...]
  calendar3 [options] calendar-list get <calendar-id> [-p <v>...] [-o <out>]
  calendar3 [options] calendar-list insert -r <kv>... [-p <v>...] [-o <out>]
  calendar3 [options] calendar-list list [-p <v>...] [-o <out>]
  calendar3 [options] calendar-list patch <calendar-id> -r <kv>... [-p <v>...] [-o <out>]
  calendar3 [options] calendar-list update <calendar-id> -r <kv>... [-p <v>...] [-o <out>]
  calendar3 [options] calendar-list watch -r <kv>... [-p <v>...] [-o <out>]
  calendar3 [options] calendars clear <calendar-id> [-p <v>...]
  calendar3 [options] calendars delete <calendar-id> [-p <v>...]
  calendar3 [options] calendars get <calendar-id> [-p <v>...] [-o <out>]
  calendar3 [options] calendars insert -r <kv>... [-p <v>...] [-o <out>]
  calendar3 [options] calendars patch <calendar-id> -r <kv>... [-p <v>...] [-o <out>]
  calendar3 [options] calendars update <calendar-id> -r <kv>... [-p <v>...] [-o <out>]
  calendar3 [options] channels stop -r <kv>... [-p <v>...]
  calendar3 [options] colors get [-p <v>...] [-o <out>]
  calendar3 [options] events delete <calendar-id> <event-id> [-p <v>...]
  calendar3 [options] events get <calendar-id> <event-id> [-p <v>...] [-o <out>]
  calendar3 [options] events import <calendar-id> -r <kv>... [-p <v>...] [-o <out>]
  calendar3 [options] events insert <calendar-id> -r <kv>... [-p <v>...] [-o <out>]
  calendar3 [options] events instances <calendar-id> <event-id> [-p <v>...] [-o <out>]
  calendar3 [options] events list <calendar-id> [-p <v>...] [-o <out>]
  calendar3 [options] events move <calendar-id> <event-id> <destination> [-p <v>...] [-o <out>]
  calendar3 [options] events patch <calendar-id> <event-id> -r <kv>... [-p <v>...] [-o <out>]
  calendar3 [options] events quick-add <calendar-id> <text> [-p <v>...] [-o <out>]
  calendar3 [options] events update <calendar-id> <event-id> -r <kv>... [-p <v>...] [-o <out>]
  calendar3 [options] events watch <calendar-id> -r <kv>... [-p <v>...] [-o <out>]
  calendar3 [options] freebusy query -r <kv>... [-p <v>...] [-o <out>]
  calendar3 [options] settings get <setting> [-p <v>...] [-o <out>]
  calendar3 [options] settings list [-p <v>...] [-o <out>]
  calendar3 [options] settings watch -r <kv>... [-p <v>...] [-o <out>]
  calendar3 --help

All documentation details can be found at
http://byron.github.io/google-apis-rs/google_calendar3_cli/index.html

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
    hub: api::CalendarHub<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _acl_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.acl().delete(&self.opt.arg_calendar_id, &self.opt.arg_rule_id);
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

    fn _acl_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.acl().get(&self.opt.arg_calendar_id, &self.opt.arg_rule_id);
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

    fn _acl_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::AclRule::default();
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
            fn request_scope_init(request: &mut api::AclRule) {
                if request.scope.is_none() {
                    request.scope = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "scope.type" => {
                        request_scope_init(&mut request);
                        request.scope.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "scope.value" => {
                        request_scope_init(&mut request);
                        request.scope.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_scope_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_scope_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "role" => {
                        request_scope_init(&mut request);
                        request.role = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_scope_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.acl().insert(request, &self.opt.arg_calendar_id);
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

    fn _acl_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.acl().list(&self.opt.arg_calendar_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "sync-token" => {
                    call = call.sync_token(value.unwrap_or(""));
                },
                "show-deleted" => {
                    call = call.show_deleted(arg_from_str(value.unwrap_or("false"), err, "show-deleted", "boolean"));
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

    fn _acl_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::AclRule::default();
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
            fn request_scope_init(request: &mut api::AclRule) {
                if request.scope.is_none() {
                    request.scope = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "scope.type" => {
                        request_scope_init(&mut request);
                        request.scope.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "scope.value" => {
                        request_scope_init(&mut request);
                        request.scope.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_scope_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_scope_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "role" => {
                        request_scope_init(&mut request);
                        request.role = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_scope_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.acl().patch(request, &self.opt.arg_calendar_id, &self.opt.arg_rule_id);
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

    fn _acl_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::AclRule::default();
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
            fn request_scope_init(request: &mut api::AclRule) {
                if request.scope.is_none() {
                    request.scope = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "scope.type" => {
                        request_scope_init(&mut request);
                        request.scope.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "scope.value" => {
                        request_scope_init(&mut request);
                        request.scope.as_mut().unwrap().value = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_scope_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_scope_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "role" => {
                        request_scope_init(&mut request);
                        request.role = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_scope_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.acl().update(request, &self.opt.arg_calendar_id, &self.opt.arg_rule_id);
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

    fn _acl_watch(&self, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.acl().watch(request, &self.opt.arg_calendar_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "sync-token" => {
                    call = call.sync_token(value.unwrap_or(""));
                },
                "show-deleted" => {
                    call = call.show_deleted(arg_from_str(value.unwrap_or("false"), err, "show-deleted", "boolean"));
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

    fn _calendar_list_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.calendar_list().delete(&self.opt.arg_calendar_id);
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

    fn _calendar_list_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.calendar_list().get(&self.opt.arg_calendar_id);
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

    fn _calendar_list_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::CalendarListEntry::default();
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
                "foreground-color" => {
                        request.foreground_color = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "deleted" => {
                        request.deleted = Some(arg_from_str(value.unwrap_or("false"), err, "deleted", "boolean"));
                    },
                "color-id" => {
                        request.color_id = Some(value.unwrap_or("").to_string());
                    },
                "selected" => {
                        request.selected = Some(arg_from_str(value.unwrap_or("false"), err, "selected", "boolean"));
                    },
                "primary" => {
                        request.primary = Some(arg_from_str(value.unwrap_or("false"), err, "primary", "boolean"));
                    },
                "summary" => {
                        request.summary = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "location" => {
                        request.location = Some(value.unwrap_or("").to_string());
                    },
                "background-color" => {
                        request.background_color = Some(value.unwrap_or("").to_string());
                    },
                "summary-override" => {
                        request.summary_override = Some(value.unwrap_or("").to_string());
                    },
                "time-zone" => {
                        request.time_zone = Some(value.unwrap_or("").to_string());
                    },
                "hidden" => {
                        request.hidden = Some(arg_from_str(value.unwrap_or("false"), err, "hidden", "boolean"));
                    },
                "access-role" => {
                        request.access_role = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.calendar_list().insert(request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "color-rgb-format" => {
                    call = call.color_rgb_format(arg_from_str(value.unwrap_or("false"), err, "color-rgb-format", "boolean"));
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

    fn _calendar_list_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.calendar_list().list();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "sync-token" => {
                    call = call.sync_token(value.unwrap_or(""));
                },
                "show-hidden" => {
                    call = call.show_hidden(arg_from_str(value.unwrap_or("false"), err, "show-hidden", "boolean"));
                },
                "show-deleted" => {
                    call = call.show_deleted(arg_from_str(value.unwrap_or("false"), err, "show-deleted", "boolean"));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "min-access-role" => {
                    call = call.min_access_role(value.unwrap_or(""));
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

    fn _calendar_list_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::CalendarListEntry::default();
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
                "foreground-color" => {
                        request.foreground_color = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "deleted" => {
                        request.deleted = Some(arg_from_str(value.unwrap_or("false"), err, "deleted", "boolean"));
                    },
                "color-id" => {
                        request.color_id = Some(value.unwrap_or("").to_string());
                    },
                "selected" => {
                        request.selected = Some(arg_from_str(value.unwrap_or("false"), err, "selected", "boolean"));
                    },
                "primary" => {
                        request.primary = Some(arg_from_str(value.unwrap_or("false"), err, "primary", "boolean"));
                    },
                "summary" => {
                        request.summary = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "location" => {
                        request.location = Some(value.unwrap_or("").to_string());
                    },
                "background-color" => {
                        request.background_color = Some(value.unwrap_or("").to_string());
                    },
                "summary-override" => {
                        request.summary_override = Some(value.unwrap_or("").to_string());
                    },
                "time-zone" => {
                        request.time_zone = Some(value.unwrap_or("").to_string());
                    },
                "hidden" => {
                        request.hidden = Some(arg_from_str(value.unwrap_or("false"), err, "hidden", "boolean"));
                    },
                "access-role" => {
                        request.access_role = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.calendar_list().patch(request, &self.opt.arg_calendar_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "color-rgb-format" => {
                    call = call.color_rgb_format(arg_from_str(value.unwrap_or("false"), err, "color-rgb-format", "boolean"));
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

    fn _calendar_list_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::CalendarListEntry::default();
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
                "foreground-color" => {
                        request.foreground_color = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "deleted" => {
                        request.deleted = Some(arg_from_str(value.unwrap_or("false"), err, "deleted", "boolean"));
                    },
                "color-id" => {
                        request.color_id = Some(value.unwrap_or("").to_string());
                    },
                "selected" => {
                        request.selected = Some(arg_from_str(value.unwrap_or("false"), err, "selected", "boolean"));
                    },
                "primary" => {
                        request.primary = Some(arg_from_str(value.unwrap_or("false"), err, "primary", "boolean"));
                    },
                "summary" => {
                        request.summary = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "location" => {
                        request.location = Some(value.unwrap_or("").to_string());
                    },
                "background-color" => {
                        request.background_color = Some(value.unwrap_or("").to_string());
                    },
                "summary-override" => {
                        request.summary_override = Some(value.unwrap_or("").to_string());
                    },
                "time-zone" => {
                        request.time_zone = Some(value.unwrap_or("").to_string());
                    },
                "hidden" => {
                        request.hidden = Some(arg_from_str(value.unwrap_or("false"), err, "hidden", "boolean"));
                    },
                "access-role" => {
                        request.access_role = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.calendar_list().update(request, &self.opt.arg_calendar_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "color-rgb-format" => {
                    call = call.color_rgb_format(arg_from_str(value.unwrap_or("false"), err, "color-rgb-format", "boolean"));
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

    fn _calendar_list_watch(&self, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.calendar_list().watch(request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "sync-token" => {
                    call = call.sync_token(value.unwrap_or(""));
                },
                "show-hidden" => {
                    call = call.show_hidden(arg_from_str(value.unwrap_or("false"), err, "show-hidden", "boolean"));
                },
                "show-deleted" => {
                    call = call.show_deleted(arg_from_str(value.unwrap_or("false"), err, "show-deleted", "boolean"));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "min-access-role" => {
                    call = call.min_access_role(value.unwrap_or(""));
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

    fn _calendars_clear(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.calendars().clear(&self.opt.arg_calendar_id);
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

    fn _calendars_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.calendars().delete(&self.opt.arg_calendar_id);
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

    fn _calendars_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.calendars().get(&self.opt.arg_calendar_id);
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

    fn _calendars_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Calendar::default();
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
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "summary" => {
                        request.summary = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "location" => {
                        request.location = Some(value.unwrap_or("").to_string());
                    },
                "time-zone" => {
                        request.time_zone = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.calendars().insert(request);
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

    fn _calendars_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Calendar::default();
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
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "summary" => {
                        request.summary = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "location" => {
                        request.location = Some(value.unwrap_or("").to_string());
                    },
                "time-zone" => {
                        request.time_zone = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.calendars().patch(request, &self.opt.arg_calendar_id);
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

    fn _calendars_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Calendar::default();
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
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "summary" => {
                        request.summary = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "location" => {
                        request.location = Some(value.unwrap_or("").to_string());
                    },
                "time-zone" => {
                        request.time_zone = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.calendars().update(request, &self.opt.arg_calendar_id);
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

    fn _colors_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.colors().get();
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

    fn _events_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.events().delete(&self.opt.arg_calendar_id, &self.opt.arg_event_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "send-notifications" => {
                    call = call.send_notifications(arg_from_str(value.unwrap_or("false"), err, "send-notifications", "boolean"));
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

    fn _events_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.events().get(&self.opt.arg_calendar_id, &self.opt.arg_event_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "time-zone" => {
                    call = call.time_zone(value.unwrap_or(""));
                },
                "max-attendees" => {
                    call = call.max_attendees(arg_from_str(value.unwrap_or("-0"), err, "max-attendees", "integer"));
                },
                "always-include-email" => {
                    call = call.always_include_email(arg_from_str(value.unwrap_or("false"), err, "always-include-email", "boolean"));
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

    fn _events_import(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Event::default();
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
            fn request_creator_init(request: &mut api::Event) {
                if request.creator.is_none() {
                    request.creator = Some(Default::default());
                }
            }
            
            fn request_end_init(request: &mut api::Event) {
                if request.end.is_none() {
                    request.end = Some(Default::default());
                }
            }
            
            fn request_extended_properties_init(request: &mut api::Event) {
                if request.extended_properties.is_none() {
                    request.extended_properties = Some(Default::default());
                }
            }
            
            fn request_gadget_init(request: &mut api::Event) {
                if request.gadget.is_none() {
                    request.gadget = Some(Default::default());
                }
            }
            
            fn request_organizer_init(request: &mut api::Event) {
                if request.organizer.is_none() {
                    request.organizer = Some(Default::default());
                }
            }
            
            fn request_original_start_time_init(request: &mut api::Event) {
                if request.original_start_time.is_none() {
                    request.original_start_time = Some(Default::default());
                }
            }
            
            fn request_reminders_init(request: &mut api::Event) {
                if request.reminders.is_none() {
                    request.reminders = Some(Default::default());
                }
            }
            
            fn request_source_init(request: &mut api::Event) {
                if request.source.is_none() {
                    request.source = Some(Default::default());
                }
            }
            
            fn request_start_init(request: &mut api::Event) {
                if request.start.is_none() {
                    request.start = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "extended-properties.shared" => {
                        request_extended_properties_init(&mut request);
                        if request.extended_properties.as_mut().unwrap().shared.is_none() {
                           request.extended_properties.as_mut().unwrap().shared = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.extended_properties.as_mut().unwrap().shared.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "extended-properties.private" => {
                        request_extended_properties_init(&mut request);
                        if request.extended_properties.as_mut().unwrap().private.is_none() {
                           request.extended_properties.as_mut().unwrap().private = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.extended_properties.as_mut().unwrap().private.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "creator.self" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().self_ = Some(arg_from_str(value.unwrap_or("false"), err, "creator.self", "boolean"));
                    },
                "creator.display-name" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "creator.email" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().email = Some(value.unwrap_or("").to_string());
                    },
                "creator.id" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "organizer.self" => {
                        request_organizer_init(&mut request);
                        request.organizer.as_mut().unwrap().self_ = Some(arg_from_str(value.unwrap_or("false"), err, "organizer.self", "boolean"));
                    },
                "organizer.display-name" => {
                        request_organizer_init(&mut request);
                        request.organizer.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "organizer.email" => {
                        request_organizer_init(&mut request);
                        request.organizer.as_mut().unwrap().email = Some(value.unwrap_or("").to_string());
                    },
                "organizer.id" => {
                        request_organizer_init(&mut request);
                        request.organizer.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_organizer_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "hangout-link" => {
                        request_organizer_init(&mut request);
                        request.hangout_link = Some(value.unwrap_or("").to_string());
                    },
                "end.date" => {
                        request_end_init(&mut request);
                        request.end.as_mut().unwrap().date = Some(value.unwrap_or("").to_string());
                    },
                "end.time-zone" => {
                        request_end_init(&mut request);
                        request.end.as_mut().unwrap().time_zone = Some(value.unwrap_or("").to_string());
                    },
                "end.date-time" => {
                        request_end_init(&mut request);
                        request.end.as_mut().unwrap().date_time = Some(value.unwrap_or("").to_string());
                    },
                "source.url" => {
                        request_source_init(&mut request);
                        request.source.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "source.title" => {
                        request_source_init(&mut request);
                        request.source.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "html-link" => {
                        request_source_init(&mut request);
                        request.html_link = Some(value.unwrap_or("").to_string());
                    },
                "recurrence" => {
                        request_source_init(&mut request);
                        if request.recurrence.is_none() {
                           request.recurrence = Some(Default::default());
                        }
                                        request.recurrence.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "start.date" => {
                        request_start_init(&mut request);
                        request.start.as_mut().unwrap().date = Some(value.unwrap_or("").to_string());
                    },
                "start.time-zone" => {
                        request_start_init(&mut request);
                        request.start.as_mut().unwrap().time_zone = Some(value.unwrap_or("").to_string());
                    },
                "start.date-time" => {
                        request_start_init(&mut request);
                        request.start.as_mut().unwrap().date_time = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_start_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "location" => {
                        request_start_init(&mut request);
                        request.location = Some(value.unwrap_or("").to_string());
                    },
                "recurring-event-id" => {
                        request_start_init(&mut request);
                        request.recurring_event_id = Some(value.unwrap_or("").to_string());
                    },
                "original-start-time.date" => {
                        request_original_start_time_init(&mut request);
                        request.original_start_time.as_mut().unwrap().date = Some(value.unwrap_or("").to_string());
                    },
                "original-start-time.time-zone" => {
                        request_original_start_time_init(&mut request);
                        request.original_start_time.as_mut().unwrap().time_zone = Some(value.unwrap_or("").to_string());
                    },
                "original-start-time.date-time" => {
                        request_original_start_time_init(&mut request);
                        request.original_start_time.as_mut().unwrap().date_time = Some(value.unwrap_or("").to_string());
                    },
                "status" => {
                        request_original_start_time_init(&mut request);
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request_original_start_time_init(&mut request);
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request_original_start_time_init(&mut request);
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "i-cal-uid" => {
                        request_original_start_time_init(&mut request);
                        request.i_cal_uid = Some(value.unwrap_or("").to_string());
                    },
                "gadget.preferences" => {
                        request_gadget_init(&mut request);
                        if request.gadget.as_mut().unwrap().preferences.is_none() {
                           request.gadget.as_mut().unwrap().preferences = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.gadget.as_mut().unwrap().preferences.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "gadget.title" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "gadget.height" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "gadget.height", "integer"));
                    },
                "gadget.width" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "gadget.width", "integer"));
                    },
                "gadget.link" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().link = Some(value.unwrap_or("").to_string());
                    },
                "gadget.type" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "gadget.display" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().display = Some(value.unwrap_or("").to_string());
                    },
                "gadget.icon-link" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().icon_link = Some(value.unwrap_or("").to_string());
                    },
                "end-time-unspecified" => {
                        request_gadget_init(&mut request);
                        request.end_time_unspecified = Some(arg_from_str(value.unwrap_or("false"), err, "end-time-unspecified", "boolean"));
                    },
                "sequence" => {
                        request_gadget_init(&mut request);
                        request.sequence = Some(arg_from_str(value.unwrap_or("-0"), err, "sequence", "integer"));
                    },
                "visibility" => {
                        request_gadget_init(&mut request);
                        request.visibility = Some(value.unwrap_or("").to_string());
                    },
                "guests-can-modify" => {
                        request_gadget_init(&mut request);
                        request.guests_can_modify = Some(arg_from_str(value.unwrap_or("false"), err, "guests-can-modify", "boolean"));
                    },
                "attendees-omitted" => {
                        request_gadget_init(&mut request);
                        request.attendees_omitted = Some(arg_from_str(value.unwrap_or("false"), err, "attendees-omitted", "boolean"));
                    },
                "kind" => {
                        request_gadget_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "locked" => {
                        request_gadget_init(&mut request);
                        request.locked = Some(arg_from_str(value.unwrap_or("false"), err, "locked", "boolean"));
                    },
                "created" => {
                        request_gadget_init(&mut request);
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "color-id" => {
                        request_gadget_init(&mut request);
                        request.color_id = Some(value.unwrap_or("").to_string());
                    },
                "anyone-can-add-self" => {
                        request_gadget_init(&mut request);
                        request.anyone_can_add_self = Some(arg_from_str(value.unwrap_or("false"), err, "anyone-can-add-self", "boolean"));
                    },
                "reminders.use-default" => {
                        request_reminders_init(&mut request);
                        request.reminders.as_mut().unwrap().use_default = Some(arg_from_str(value.unwrap_or("false"), err, "reminders.use-default", "boolean"));
                    },
                "guests-can-see-other-guests" => {
                        request_reminders_init(&mut request);
                        request.guests_can_see_other_guests = Some(arg_from_str(value.unwrap_or("false"), err, "guests-can-see-other-guests", "boolean"));
                    },
                "summary" => {
                        request_reminders_init(&mut request);
                        request.summary = Some(value.unwrap_or("").to_string());
                    },
                "guests-can-invite-others" => {
                        request_reminders_init(&mut request);
                        request.guests_can_invite_others = Some(arg_from_str(value.unwrap_or("false"), err, "guests-can-invite-others", "boolean"));
                    },
                "transparency" => {
                        request_reminders_init(&mut request);
                        request.transparency = Some(value.unwrap_or("").to_string());
                    },
                "private-copy" => {
                        request_reminders_init(&mut request);
                        request.private_copy = Some(arg_from_str(value.unwrap_or("false"), err, "private-copy", "boolean"));
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.events().import(request, &self.opt.arg_calendar_id);
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

    fn _events_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Event::default();
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
            fn request_creator_init(request: &mut api::Event) {
                if request.creator.is_none() {
                    request.creator = Some(Default::default());
                }
            }
            
            fn request_end_init(request: &mut api::Event) {
                if request.end.is_none() {
                    request.end = Some(Default::default());
                }
            }
            
            fn request_extended_properties_init(request: &mut api::Event) {
                if request.extended_properties.is_none() {
                    request.extended_properties = Some(Default::default());
                }
            }
            
            fn request_gadget_init(request: &mut api::Event) {
                if request.gadget.is_none() {
                    request.gadget = Some(Default::default());
                }
            }
            
            fn request_organizer_init(request: &mut api::Event) {
                if request.organizer.is_none() {
                    request.organizer = Some(Default::default());
                }
            }
            
            fn request_original_start_time_init(request: &mut api::Event) {
                if request.original_start_time.is_none() {
                    request.original_start_time = Some(Default::default());
                }
            }
            
            fn request_reminders_init(request: &mut api::Event) {
                if request.reminders.is_none() {
                    request.reminders = Some(Default::default());
                }
            }
            
            fn request_source_init(request: &mut api::Event) {
                if request.source.is_none() {
                    request.source = Some(Default::default());
                }
            }
            
            fn request_start_init(request: &mut api::Event) {
                if request.start.is_none() {
                    request.start = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "extended-properties.shared" => {
                        request_extended_properties_init(&mut request);
                        if request.extended_properties.as_mut().unwrap().shared.is_none() {
                           request.extended_properties.as_mut().unwrap().shared = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.extended_properties.as_mut().unwrap().shared.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "extended-properties.private" => {
                        request_extended_properties_init(&mut request);
                        if request.extended_properties.as_mut().unwrap().private.is_none() {
                           request.extended_properties.as_mut().unwrap().private = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.extended_properties.as_mut().unwrap().private.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "creator.self" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().self_ = Some(arg_from_str(value.unwrap_or("false"), err, "creator.self", "boolean"));
                    },
                "creator.display-name" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "creator.email" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().email = Some(value.unwrap_or("").to_string());
                    },
                "creator.id" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "organizer.self" => {
                        request_organizer_init(&mut request);
                        request.organizer.as_mut().unwrap().self_ = Some(arg_from_str(value.unwrap_or("false"), err, "organizer.self", "boolean"));
                    },
                "organizer.display-name" => {
                        request_organizer_init(&mut request);
                        request.organizer.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "organizer.email" => {
                        request_organizer_init(&mut request);
                        request.organizer.as_mut().unwrap().email = Some(value.unwrap_or("").to_string());
                    },
                "organizer.id" => {
                        request_organizer_init(&mut request);
                        request.organizer.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_organizer_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "hangout-link" => {
                        request_organizer_init(&mut request);
                        request.hangout_link = Some(value.unwrap_or("").to_string());
                    },
                "end.date" => {
                        request_end_init(&mut request);
                        request.end.as_mut().unwrap().date = Some(value.unwrap_or("").to_string());
                    },
                "end.time-zone" => {
                        request_end_init(&mut request);
                        request.end.as_mut().unwrap().time_zone = Some(value.unwrap_or("").to_string());
                    },
                "end.date-time" => {
                        request_end_init(&mut request);
                        request.end.as_mut().unwrap().date_time = Some(value.unwrap_or("").to_string());
                    },
                "source.url" => {
                        request_source_init(&mut request);
                        request.source.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "source.title" => {
                        request_source_init(&mut request);
                        request.source.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "html-link" => {
                        request_source_init(&mut request);
                        request.html_link = Some(value.unwrap_or("").to_string());
                    },
                "recurrence" => {
                        request_source_init(&mut request);
                        if request.recurrence.is_none() {
                           request.recurrence = Some(Default::default());
                        }
                                        request.recurrence.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "start.date" => {
                        request_start_init(&mut request);
                        request.start.as_mut().unwrap().date = Some(value.unwrap_or("").to_string());
                    },
                "start.time-zone" => {
                        request_start_init(&mut request);
                        request.start.as_mut().unwrap().time_zone = Some(value.unwrap_or("").to_string());
                    },
                "start.date-time" => {
                        request_start_init(&mut request);
                        request.start.as_mut().unwrap().date_time = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_start_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "location" => {
                        request_start_init(&mut request);
                        request.location = Some(value.unwrap_or("").to_string());
                    },
                "recurring-event-id" => {
                        request_start_init(&mut request);
                        request.recurring_event_id = Some(value.unwrap_or("").to_string());
                    },
                "original-start-time.date" => {
                        request_original_start_time_init(&mut request);
                        request.original_start_time.as_mut().unwrap().date = Some(value.unwrap_or("").to_string());
                    },
                "original-start-time.time-zone" => {
                        request_original_start_time_init(&mut request);
                        request.original_start_time.as_mut().unwrap().time_zone = Some(value.unwrap_or("").to_string());
                    },
                "original-start-time.date-time" => {
                        request_original_start_time_init(&mut request);
                        request.original_start_time.as_mut().unwrap().date_time = Some(value.unwrap_or("").to_string());
                    },
                "status" => {
                        request_original_start_time_init(&mut request);
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request_original_start_time_init(&mut request);
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request_original_start_time_init(&mut request);
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "i-cal-uid" => {
                        request_original_start_time_init(&mut request);
                        request.i_cal_uid = Some(value.unwrap_or("").to_string());
                    },
                "gadget.preferences" => {
                        request_gadget_init(&mut request);
                        if request.gadget.as_mut().unwrap().preferences.is_none() {
                           request.gadget.as_mut().unwrap().preferences = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.gadget.as_mut().unwrap().preferences.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "gadget.title" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "gadget.height" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "gadget.height", "integer"));
                    },
                "gadget.width" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "gadget.width", "integer"));
                    },
                "gadget.link" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().link = Some(value.unwrap_or("").to_string());
                    },
                "gadget.type" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "gadget.display" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().display = Some(value.unwrap_or("").to_string());
                    },
                "gadget.icon-link" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().icon_link = Some(value.unwrap_or("").to_string());
                    },
                "end-time-unspecified" => {
                        request_gadget_init(&mut request);
                        request.end_time_unspecified = Some(arg_from_str(value.unwrap_or("false"), err, "end-time-unspecified", "boolean"));
                    },
                "sequence" => {
                        request_gadget_init(&mut request);
                        request.sequence = Some(arg_from_str(value.unwrap_or("-0"), err, "sequence", "integer"));
                    },
                "visibility" => {
                        request_gadget_init(&mut request);
                        request.visibility = Some(value.unwrap_or("").to_string());
                    },
                "guests-can-modify" => {
                        request_gadget_init(&mut request);
                        request.guests_can_modify = Some(arg_from_str(value.unwrap_or("false"), err, "guests-can-modify", "boolean"));
                    },
                "attendees-omitted" => {
                        request_gadget_init(&mut request);
                        request.attendees_omitted = Some(arg_from_str(value.unwrap_or("false"), err, "attendees-omitted", "boolean"));
                    },
                "kind" => {
                        request_gadget_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "locked" => {
                        request_gadget_init(&mut request);
                        request.locked = Some(arg_from_str(value.unwrap_or("false"), err, "locked", "boolean"));
                    },
                "created" => {
                        request_gadget_init(&mut request);
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "color-id" => {
                        request_gadget_init(&mut request);
                        request.color_id = Some(value.unwrap_or("").to_string());
                    },
                "anyone-can-add-self" => {
                        request_gadget_init(&mut request);
                        request.anyone_can_add_self = Some(arg_from_str(value.unwrap_or("false"), err, "anyone-can-add-self", "boolean"));
                    },
                "reminders.use-default" => {
                        request_reminders_init(&mut request);
                        request.reminders.as_mut().unwrap().use_default = Some(arg_from_str(value.unwrap_or("false"), err, "reminders.use-default", "boolean"));
                    },
                "guests-can-see-other-guests" => {
                        request_reminders_init(&mut request);
                        request.guests_can_see_other_guests = Some(arg_from_str(value.unwrap_or("false"), err, "guests-can-see-other-guests", "boolean"));
                    },
                "summary" => {
                        request_reminders_init(&mut request);
                        request.summary = Some(value.unwrap_or("").to_string());
                    },
                "guests-can-invite-others" => {
                        request_reminders_init(&mut request);
                        request.guests_can_invite_others = Some(arg_from_str(value.unwrap_or("false"), err, "guests-can-invite-others", "boolean"));
                    },
                "transparency" => {
                        request_reminders_init(&mut request);
                        request.transparency = Some(value.unwrap_or("").to_string());
                    },
                "private-copy" => {
                        request_reminders_init(&mut request);
                        request.private_copy = Some(arg_from_str(value.unwrap_or("false"), err, "private-copy", "boolean"));
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.events().insert(request, &self.opt.arg_calendar_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "send-notifications" => {
                    call = call.send_notifications(arg_from_str(value.unwrap_or("false"), err, "send-notifications", "boolean"));
                },
                "max-attendees" => {
                    call = call.max_attendees(arg_from_str(value.unwrap_or("-0"), err, "max-attendees", "integer"));
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

    fn _events_instances(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.events().instances(&self.opt.arg_calendar_id, &self.opt.arg_event_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "time-zone" => {
                    call = call.time_zone(value.unwrap_or(""));
                },
                "time-min" => {
                    call = call.time_min(value.unwrap_or(""));
                },
                "time-max" => {
                    call = call.time_max(value.unwrap_or(""));
                },
                "show-deleted" => {
                    call = call.show_deleted(arg_from_str(value.unwrap_or("false"), err, "show-deleted", "boolean"));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "original-start" => {
                    call = call.original_start(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "max-attendees" => {
                    call = call.max_attendees(arg_from_str(value.unwrap_or("-0"), err, "max-attendees", "integer"));
                },
                "always-include-email" => {
                    call = call.always_include_email(arg_from_str(value.unwrap_or("false"), err, "always-include-email", "boolean"));
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

    fn _events_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.events().list(&self.opt.arg_calendar_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "updated-min" => {
                    call = call.updated_min(value.unwrap_or(""));
                },
                "time-zone" => {
                    call = call.time_zone(value.unwrap_or(""));
                },
                "time-min" => {
                    call = call.time_min(value.unwrap_or(""));
                },
                "time-max" => {
                    call = call.time_max(value.unwrap_or(""));
                },
                "sync-token" => {
                    call = call.sync_token(value.unwrap_or(""));
                },
                "single-events" => {
                    call = call.single_events(arg_from_str(value.unwrap_or("false"), err, "single-events", "boolean"));
                },
                "show-hidden-invitations" => {
                    call = call.show_hidden_invitations(arg_from_str(value.unwrap_or("false"), err, "show-hidden-invitations", "boolean"));
                },
                "show-deleted" => {
                    call = call.show_deleted(arg_from_str(value.unwrap_or("false"), err, "show-deleted", "boolean"));
                },
                "shared-extended-property" => {
                    call = call.add_shared_extended_property(value.unwrap_or(""));
                },
                "q" => {
                    call = call.q(value.unwrap_or(""));
                },
                "private-extended-property" => {
                    call = call.add_private_extended_property(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "max-attendees" => {
                    call = call.max_attendees(arg_from_str(value.unwrap_or("-0"), err, "max-attendees", "integer"));
                },
                "i-cal-uid" => {
                    call = call.i_cal_uid(value.unwrap_or(""));
                },
                "always-include-email" => {
                    call = call.always_include_email(arg_from_str(value.unwrap_or("false"), err, "always-include-email", "boolean"));
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

    fn _events_move(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.events().move_(&self.opt.arg_calendar_id, &self.opt.arg_event_id, &self.opt.arg_destination);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "send-notifications" => {
                    call = call.send_notifications(arg_from_str(value.unwrap_or("false"), err, "send-notifications", "boolean"));
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

    fn _events_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Event::default();
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
            fn request_creator_init(request: &mut api::Event) {
                if request.creator.is_none() {
                    request.creator = Some(Default::default());
                }
            }
            
            fn request_end_init(request: &mut api::Event) {
                if request.end.is_none() {
                    request.end = Some(Default::default());
                }
            }
            
            fn request_extended_properties_init(request: &mut api::Event) {
                if request.extended_properties.is_none() {
                    request.extended_properties = Some(Default::default());
                }
            }
            
            fn request_gadget_init(request: &mut api::Event) {
                if request.gadget.is_none() {
                    request.gadget = Some(Default::default());
                }
            }
            
            fn request_organizer_init(request: &mut api::Event) {
                if request.organizer.is_none() {
                    request.organizer = Some(Default::default());
                }
            }
            
            fn request_original_start_time_init(request: &mut api::Event) {
                if request.original_start_time.is_none() {
                    request.original_start_time = Some(Default::default());
                }
            }
            
            fn request_reminders_init(request: &mut api::Event) {
                if request.reminders.is_none() {
                    request.reminders = Some(Default::default());
                }
            }
            
            fn request_source_init(request: &mut api::Event) {
                if request.source.is_none() {
                    request.source = Some(Default::default());
                }
            }
            
            fn request_start_init(request: &mut api::Event) {
                if request.start.is_none() {
                    request.start = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "extended-properties.shared" => {
                        request_extended_properties_init(&mut request);
                        if request.extended_properties.as_mut().unwrap().shared.is_none() {
                           request.extended_properties.as_mut().unwrap().shared = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.extended_properties.as_mut().unwrap().shared.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "extended-properties.private" => {
                        request_extended_properties_init(&mut request);
                        if request.extended_properties.as_mut().unwrap().private.is_none() {
                           request.extended_properties.as_mut().unwrap().private = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.extended_properties.as_mut().unwrap().private.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "creator.self" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().self_ = Some(arg_from_str(value.unwrap_or("false"), err, "creator.self", "boolean"));
                    },
                "creator.display-name" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "creator.email" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().email = Some(value.unwrap_or("").to_string());
                    },
                "creator.id" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "organizer.self" => {
                        request_organizer_init(&mut request);
                        request.organizer.as_mut().unwrap().self_ = Some(arg_from_str(value.unwrap_or("false"), err, "organizer.self", "boolean"));
                    },
                "organizer.display-name" => {
                        request_organizer_init(&mut request);
                        request.organizer.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "organizer.email" => {
                        request_organizer_init(&mut request);
                        request.organizer.as_mut().unwrap().email = Some(value.unwrap_or("").to_string());
                    },
                "organizer.id" => {
                        request_organizer_init(&mut request);
                        request.organizer.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_organizer_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "hangout-link" => {
                        request_organizer_init(&mut request);
                        request.hangout_link = Some(value.unwrap_or("").to_string());
                    },
                "end.date" => {
                        request_end_init(&mut request);
                        request.end.as_mut().unwrap().date = Some(value.unwrap_or("").to_string());
                    },
                "end.time-zone" => {
                        request_end_init(&mut request);
                        request.end.as_mut().unwrap().time_zone = Some(value.unwrap_or("").to_string());
                    },
                "end.date-time" => {
                        request_end_init(&mut request);
                        request.end.as_mut().unwrap().date_time = Some(value.unwrap_or("").to_string());
                    },
                "source.url" => {
                        request_source_init(&mut request);
                        request.source.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "source.title" => {
                        request_source_init(&mut request);
                        request.source.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "html-link" => {
                        request_source_init(&mut request);
                        request.html_link = Some(value.unwrap_or("").to_string());
                    },
                "recurrence" => {
                        request_source_init(&mut request);
                        if request.recurrence.is_none() {
                           request.recurrence = Some(Default::default());
                        }
                                        request.recurrence.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "start.date" => {
                        request_start_init(&mut request);
                        request.start.as_mut().unwrap().date = Some(value.unwrap_or("").to_string());
                    },
                "start.time-zone" => {
                        request_start_init(&mut request);
                        request.start.as_mut().unwrap().time_zone = Some(value.unwrap_or("").to_string());
                    },
                "start.date-time" => {
                        request_start_init(&mut request);
                        request.start.as_mut().unwrap().date_time = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_start_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "location" => {
                        request_start_init(&mut request);
                        request.location = Some(value.unwrap_or("").to_string());
                    },
                "recurring-event-id" => {
                        request_start_init(&mut request);
                        request.recurring_event_id = Some(value.unwrap_or("").to_string());
                    },
                "original-start-time.date" => {
                        request_original_start_time_init(&mut request);
                        request.original_start_time.as_mut().unwrap().date = Some(value.unwrap_or("").to_string());
                    },
                "original-start-time.time-zone" => {
                        request_original_start_time_init(&mut request);
                        request.original_start_time.as_mut().unwrap().time_zone = Some(value.unwrap_or("").to_string());
                    },
                "original-start-time.date-time" => {
                        request_original_start_time_init(&mut request);
                        request.original_start_time.as_mut().unwrap().date_time = Some(value.unwrap_or("").to_string());
                    },
                "status" => {
                        request_original_start_time_init(&mut request);
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request_original_start_time_init(&mut request);
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request_original_start_time_init(&mut request);
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "i-cal-uid" => {
                        request_original_start_time_init(&mut request);
                        request.i_cal_uid = Some(value.unwrap_or("").to_string());
                    },
                "gadget.preferences" => {
                        request_gadget_init(&mut request);
                        if request.gadget.as_mut().unwrap().preferences.is_none() {
                           request.gadget.as_mut().unwrap().preferences = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.gadget.as_mut().unwrap().preferences.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "gadget.title" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "gadget.height" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "gadget.height", "integer"));
                    },
                "gadget.width" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "gadget.width", "integer"));
                    },
                "gadget.link" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().link = Some(value.unwrap_or("").to_string());
                    },
                "gadget.type" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "gadget.display" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().display = Some(value.unwrap_or("").to_string());
                    },
                "gadget.icon-link" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().icon_link = Some(value.unwrap_or("").to_string());
                    },
                "end-time-unspecified" => {
                        request_gadget_init(&mut request);
                        request.end_time_unspecified = Some(arg_from_str(value.unwrap_or("false"), err, "end-time-unspecified", "boolean"));
                    },
                "sequence" => {
                        request_gadget_init(&mut request);
                        request.sequence = Some(arg_from_str(value.unwrap_or("-0"), err, "sequence", "integer"));
                    },
                "visibility" => {
                        request_gadget_init(&mut request);
                        request.visibility = Some(value.unwrap_or("").to_string());
                    },
                "guests-can-modify" => {
                        request_gadget_init(&mut request);
                        request.guests_can_modify = Some(arg_from_str(value.unwrap_or("false"), err, "guests-can-modify", "boolean"));
                    },
                "attendees-omitted" => {
                        request_gadget_init(&mut request);
                        request.attendees_omitted = Some(arg_from_str(value.unwrap_or("false"), err, "attendees-omitted", "boolean"));
                    },
                "kind" => {
                        request_gadget_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "locked" => {
                        request_gadget_init(&mut request);
                        request.locked = Some(arg_from_str(value.unwrap_or("false"), err, "locked", "boolean"));
                    },
                "created" => {
                        request_gadget_init(&mut request);
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "color-id" => {
                        request_gadget_init(&mut request);
                        request.color_id = Some(value.unwrap_or("").to_string());
                    },
                "anyone-can-add-self" => {
                        request_gadget_init(&mut request);
                        request.anyone_can_add_self = Some(arg_from_str(value.unwrap_or("false"), err, "anyone-can-add-self", "boolean"));
                    },
                "reminders.use-default" => {
                        request_reminders_init(&mut request);
                        request.reminders.as_mut().unwrap().use_default = Some(arg_from_str(value.unwrap_or("false"), err, "reminders.use-default", "boolean"));
                    },
                "guests-can-see-other-guests" => {
                        request_reminders_init(&mut request);
                        request.guests_can_see_other_guests = Some(arg_from_str(value.unwrap_or("false"), err, "guests-can-see-other-guests", "boolean"));
                    },
                "summary" => {
                        request_reminders_init(&mut request);
                        request.summary = Some(value.unwrap_or("").to_string());
                    },
                "guests-can-invite-others" => {
                        request_reminders_init(&mut request);
                        request.guests_can_invite_others = Some(arg_from_str(value.unwrap_or("false"), err, "guests-can-invite-others", "boolean"));
                    },
                "transparency" => {
                        request_reminders_init(&mut request);
                        request.transparency = Some(value.unwrap_or("").to_string());
                    },
                "private-copy" => {
                        request_reminders_init(&mut request);
                        request.private_copy = Some(arg_from_str(value.unwrap_or("false"), err, "private-copy", "boolean"));
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.events().patch(request, &self.opt.arg_calendar_id, &self.opt.arg_event_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "send-notifications" => {
                    call = call.send_notifications(arg_from_str(value.unwrap_or("false"), err, "send-notifications", "boolean"));
                },
                "max-attendees" => {
                    call = call.max_attendees(arg_from_str(value.unwrap_or("-0"), err, "max-attendees", "integer"));
                },
                "always-include-email" => {
                    call = call.always_include_email(arg_from_str(value.unwrap_or("false"), err, "always-include-email", "boolean"));
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

    fn _events_quick_add(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.events().quick_add(&self.opt.arg_calendar_id, &self.opt.arg_text);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "send-notifications" => {
                    call = call.send_notifications(arg_from_str(value.unwrap_or("false"), err, "send-notifications", "boolean"));
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

    fn _events_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Event::default();
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
            fn request_creator_init(request: &mut api::Event) {
                if request.creator.is_none() {
                    request.creator = Some(Default::default());
                }
            }
            
            fn request_end_init(request: &mut api::Event) {
                if request.end.is_none() {
                    request.end = Some(Default::default());
                }
            }
            
            fn request_extended_properties_init(request: &mut api::Event) {
                if request.extended_properties.is_none() {
                    request.extended_properties = Some(Default::default());
                }
            }
            
            fn request_gadget_init(request: &mut api::Event) {
                if request.gadget.is_none() {
                    request.gadget = Some(Default::default());
                }
            }
            
            fn request_organizer_init(request: &mut api::Event) {
                if request.organizer.is_none() {
                    request.organizer = Some(Default::default());
                }
            }
            
            fn request_original_start_time_init(request: &mut api::Event) {
                if request.original_start_time.is_none() {
                    request.original_start_time = Some(Default::default());
                }
            }
            
            fn request_reminders_init(request: &mut api::Event) {
                if request.reminders.is_none() {
                    request.reminders = Some(Default::default());
                }
            }
            
            fn request_source_init(request: &mut api::Event) {
                if request.source.is_none() {
                    request.source = Some(Default::default());
                }
            }
            
            fn request_start_init(request: &mut api::Event) {
                if request.start.is_none() {
                    request.start = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "extended-properties.shared" => {
                        request_extended_properties_init(&mut request);
                        if request.extended_properties.as_mut().unwrap().shared.is_none() {
                           request.extended_properties.as_mut().unwrap().shared = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.extended_properties.as_mut().unwrap().shared.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "extended-properties.private" => {
                        request_extended_properties_init(&mut request);
                        if request.extended_properties.as_mut().unwrap().private.is_none() {
                           request.extended_properties.as_mut().unwrap().private = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.extended_properties.as_mut().unwrap().private.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "creator.self" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().self_ = Some(arg_from_str(value.unwrap_or("false"), err, "creator.self", "boolean"));
                    },
                "creator.display-name" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "creator.email" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().email = Some(value.unwrap_or("").to_string());
                    },
                "creator.id" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "organizer.self" => {
                        request_organizer_init(&mut request);
                        request.organizer.as_mut().unwrap().self_ = Some(arg_from_str(value.unwrap_or("false"), err, "organizer.self", "boolean"));
                    },
                "organizer.display-name" => {
                        request_organizer_init(&mut request);
                        request.organizer.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "organizer.email" => {
                        request_organizer_init(&mut request);
                        request.organizer.as_mut().unwrap().email = Some(value.unwrap_or("").to_string());
                    },
                "organizer.id" => {
                        request_organizer_init(&mut request);
                        request.organizer.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_organizer_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "hangout-link" => {
                        request_organizer_init(&mut request);
                        request.hangout_link = Some(value.unwrap_or("").to_string());
                    },
                "end.date" => {
                        request_end_init(&mut request);
                        request.end.as_mut().unwrap().date = Some(value.unwrap_or("").to_string());
                    },
                "end.time-zone" => {
                        request_end_init(&mut request);
                        request.end.as_mut().unwrap().time_zone = Some(value.unwrap_or("").to_string());
                    },
                "end.date-time" => {
                        request_end_init(&mut request);
                        request.end.as_mut().unwrap().date_time = Some(value.unwrap_or("").to_string());
                    },
                "source.url" => {
                        request_source_init(&mut request);
                        request.source.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "source.title" => {
                        request_source_init(&mut request);
                        request.source.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "html-link" => {
                        request_source_init(&mut request);
                        request.html_link = Some(value.unwrap_or("").to_string());
                    },
                "recurrence" => {
                        request_source_init(&mut request);
                        if request.recurrence.is_none() {
                           request.recurrence = Some(Default::default());
                        }
                                        request.recurrence.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "start.date" => {
                        request_start_init(&mut request);
                        request.start.as_mut().unwrap().date = Some(value.unwrap_or("").to_string());
                    },
                "start.time-zone" => {
                        request_start_init(&mut request);
                        request.start.as_mut().unwrap().time_zone = Some(value.unwrap_or("").to_string());
                    },
                "start.date-time" => {
                        request_start_init(&mut request);
                        request.start.as_mut().unwrap().date_time = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_start_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "location" => {
                        request_start_init(&mut request);
                        request.location = Some(value.unwrap_or("").to_string());
                    },
                "recurring-event-id" => {
                        request_start_init(&mut request);
                        request.recurring_event_id = Some(value.unwrap_or("").to_string());
                    },
                "original-start-time.date" => {
                        request_original_start_time_init(&mut request);
                        request.original_start_time.as_mut().unwrap().date = Some(value.unwrap_or("").to_string());
                    },
                "original-start-time.time-zone" => {
                        request_original_start_time_init(&mut request);
                        request.original_start_time.as_mut().unwrap().time_zone = Some(value.unwrap_or("").to_string());
                    },
                "original-start-time.date-time" => {
                        request_original_start_time_init(&mut request);
                        request.original_start_time.as_mut().unwrap().date_time = Some(value.unwrap_or("").to_string());
                    },
                "status" => {
                        request_original_start_time_init(&mut request);
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request_original_start_time_init(&mut request);
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request_original_start_time_init(&mut request);
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "i-cal-uid" => {
                        request_original_start_time_init(&mut request);
                        request.i_cal_uid = Some(value.unwrap_or("").to_string());
                    },
                "gadget.preferences" => {
                        request_gadget_init(&mut request);
                        if request.gadget.as_mut().unwrap().preferences.is_none() {
                           request.gadget.as_mut().unwrap().preferences = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.gadget.as_mut().unwrap().preferences.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "gadget.title" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "gadget.height" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("-0"), err, "gadget.height", "integer"));
                    },
                "gadget.width" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().width = Some(arg_from_str(value.unwrap_or("-0"), err, "gadget.width", "integer"));
                    },
                "gadget.link" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().link = Some(value.unwrap_or("").to_string());
                    },
                "gadget.type" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "gadget.display" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().display = Some(value.unwrap_or("").to_string());
                    },
                "gadget.icon-link" => {
                        request_gadget_init(&mut request);
                        request.gadget.as_mut().unwrap().icon_link = Some(value.unwrap_or("").to_string());
                    },
                "end-time-unspecified" => {
                        request_gadget_init(&mut request);
                        request.end_time_unspecified = Some(arg_from_str(value.unwrap_or("false"), err, "end-time-unspecified", "boolean"));
                    },
                "sequence" => {
                        request_gadget_init(&mut request);
                        request.sequence = Some(arg_from_str(value.unwrap_or("-0"), err, "sequence", "integer"));
                    },
                "visibility" => {
                        request_gadget_init(&mut request);
                        request.visibility = Some(value.unwrap_or("").to_string());
                    },
                "guests-can-modify" => {
                        request_gadget_init(&mut request);
                        request.guests_can_modify = Some(arg_from_str(value.unwrap_or("false"), err, "guests-can-modify", "boolean"));
                    },
                "attendees-omitted" => {
                        request_gadget_init(&mut request);
                        request.attendees_omitted = Some(arg_from_str(value.unwrap_or("false"), err, "attendees-omitted", "boolean"));
                    },
                "kind" => {
                        request_gadget_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "locked" => {
                        request_gadget_init(&mut request);
                        request.locked = Some(arg_from_str(value.unwrap_or("false"), err, "locked", "boolean"));
                    },
                "created" => {
                        request_gadget_init(&mut request);
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "color-id" => {
                        request_gadget_init(&mut request);
                        request.color_id = Some(value.unwrap_or("").to_string());
                    },
                "anyone-can-add-self" => {
                        request_gadget_init(&mut request);
                        request.anyone_can_add_self = Some(arg_from_str(value.unwrap_or("false"), err, "anyone-can-add-self", "boolean"));
                    },
                "reminders.use-default" => {
                        request_reminders_init(&mut request);
                        request.reminders.as_mut().unwrap().use_default = Some(arg_from_str(value.unwrap_or("false"), err, "reminders.use-default", "boolean"));
                    },
                "guests-can-see-other-guests" => {
                        request_reminders_init(&mut request);
                        request.guests_can_see_other_guests = Some(arg_from_str(value.unwrap_or("false"), err, "guests-can-see-other-guests", "boolean"));
                    },
                "summary" => {
                        request_reminders_init(&mut request);
                        request.summary = Some(value.unwrap_or("").to_string());
                    },
                "guests-can-invite-others" => {
                        request_reminders_init(&mut request);
                        request.guests_can_invite_others = Some(arg_from_str(value.unwrap_or("false"), err, "guests-can-invite-others", "boolean"));
                    },
                "transparency" => {
                        request_reminders_init(&mut request);
                        request.transparency = Some(value.unwrap_or("").to_string());
                    },
                "private-copy" => {
                        request_reminders_init(&mut request);
                        request.private_copy = Some(arg_from_str(value.unwrap_or("false"), err, "private-copy", "boolean"));
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.events().update(request, &self.opt.arg_calendar_id, &self.opt.arg_event_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "send-notifications" => {
                    call = call.send_notifications(arg_from_str(value.unwrap_or("false"), err, "send-notifications", "boolean"));
                },
                "max-attendees" => {
                    call = call.max_attendees(arg_from_str(value.unwrap_or("-0"), err, "max-attendees", "integer"));
                },
                "always-include-email" => {
                    call = call.always_include_email(arg_from_str(value.unwrap_or("false"), err, "always-include-email", "boolean"));
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

    fn _events_watch(&self, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.events().watch(request, &self.opt.arg_calendar_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "updated-min" => {
                    call = call.updated_min(value.unwrap_or(""));
                },
                "time-zone" => {
                    call = call.time_zone(value.unwrap_or(""));
                },
                "time-min" => {
                    call = call.time_min(value.unwrap_or(""));
                },
                "time-max" => {
                    call = call.time_max(value.unwrap_or(""));
                },
                "sync-token" => {
                    call = call.sync_token(value.unwrap_or(""));
                },
                "single-events" => {
                    call = call.single_events(arg_from_str(value.unwrap_or("false"), err, "single-events", "boolean"));
                },
                "show-hidden-invitations" => {
                    call = call.show_hidden_invitations(arg_from_str(value.unwrap_or("false"), err, "show-hidden-invitations", "boolean"));
                },
                "show-deleted" => {
                    call = call.show_deleted(arg_from_str(value.unwrap_or("false"), err, "show-deleted", "boolean"));
                },
                "shared-extended-property" => {
                    call = call.add_shared_extended_property(value.unwrap_or(""));
                },
                "q" => {
                    call = call.q(value.unwrap_or(""));
                },
                "private-extended-property" => {
                    call = call.add_private_extended_property(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "max-attendees" => {
                    call = call.max_attendees(arg_from_str(value.unwrap_or("-0"), err, "max-attendees", "integer"));
                },
                "i-cal-uid" => {
                    call = call.i_cal_uid(value.unwrap_or(""));
                },
                "always-include-email" => {
                    call = call.always_include_email(arg_from_str(value.unwrap_or("false"), err, "always-include-email", "boolean"));
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

    fn _freebusy_query(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::FreeBusyRequest::default();
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
                "time-max" => {
                        request.time_max = Some(value.unwrap_or("").to_string());
                    },
                "calendar-expansion-max" => {
                        request.calendar_expansion_max = Some(arg_from_str(value.unwrap_or("-0"), err, "calendar-expansion-max", "integer"));
                    },
                "time-zone" => {
                        request.time_zone = Some(value.unwrap_or("").to_string());
                    },
                "time-min" => {
                        request.time_min = Some(value.unwrap_or("").to_string());
                    },
                "group-expansion-max" => {
                        request.group_expansion_max = Some(arg_from_str(value.unwrap_or("-0"), err, "group-expansion-max", "integer"));
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.freebusy().query(request);
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

    fn _settings_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.settings().get(&self.opt.arg_setting);
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

    fn _settings_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.settings().list();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "sync-token" => {
                    call = call.sync_token(value.unwrap_or(""));
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

    fn _settings_watch(&self, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.settings().watch(request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "sync-token" => {
                    call = call.sync_token(value.unwrap_or(""));
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

    fn _doit(&self, dry_run: bool) -> (Option<api::Error>, Option<InvalidOptionsError>) {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Option<api::Error>;
        let mut err_opt: Option<InvalidOptionsError> = None;

        if self.opt.cmd_acl {
            if self.opt.cmd_delete {
                call_result = self._acl_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._acl_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._acl_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._acl_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._acl_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._acl_update(dry_run, &mut err);
            } else if self.opt.cmd_watch {
                call_result = self._acl_watch(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_calendar_list {
            if self.opt.cmd_delete {
                call_result = self._calendar_list_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._calendar_list_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._calendar_list_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._calendar_list_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._calendar_list_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._calendar_list_update(dry_run, &mut err);
            } else if self.opt.cmd_watch {
                call_result = self._calendar_list_watch(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_calendars {
            if self.opt.cmd_clear {
                call_result = self._calendars_clear(dry_run, &mut err);
            } else if self.opt.cmd_delete {
                call_result = self._calendars_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._calendars_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._calendars_insert(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._calendars_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._calendars_update(dry_run, &mut err);
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
 else if self.opt.cmd_colors {
            if self.opt.cmd_get {
                call_result = self._colors_get(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_events {
            if self.opt.cmd_delete {
                call_result = self._events_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._events_get(dry_run, &mut err);
            } else if self.opt.cmd_import {
                call_result = self._events_import(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._events_insert(dry_run, &mut err);
            } else if self.opt.cmd_instances {
                call_result = self._events_instances(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._events_list(dry_run, &mut err);
            } else if self.opt.cmd_move {
                call_result = self._events_move(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._events_patch(dry_run, &mut err);
            } else if self.opt.cmd_quick_add {
                call_result = self._events_quick_add(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._events_update(dry_run, &mut err);
            } else if self.opt.cmd_watch {
                call_result = self._events_watch(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_freebusy {
            if self.opt.cmd_query {
                call_result = self._freebusy_query(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_settings {
            if self.opt.cmd_get {
                call_result = self._settings_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._settings_list(dry_run, &mut err);
            } else if self.opt.cmd_watch {
                call_result = self._settings_watch(dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "calendar3-secret.json", 
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
                                          program_name: "calendar3",
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
            hub: api::CalendarHub::new(client, auth),
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