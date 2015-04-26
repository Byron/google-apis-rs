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
extern crate google_pubsub1_beta2 as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  pubsub1-beta2 [options] projects subscriptions-acknowledge <subscription> -r <kv>... [-p <v>...] [-o <out>]
  pubsub1-beta2 [options] projects subscriptions-create <name> -r <kv>... [-p <v>...] [-o <out>]
  pubsub1-beta2 [options] projects subscriptions-delete <subscription> [-p <v>...] [-o <out>]
  pubsub1-beta2 [options] projects subscriptions-get <subscription> [-p <v>...] [-o <out>]
  pubsub1-beta2 [options] projects subscriptions-list <project> [-p <v>...] [-o <out>]
  pubsub1-beta2 [options] projects subscriptions-modify-ack-deadline <subscription> -r <kv>... [-p <v>...] [-o <out>]
  pubsub1-beta2 [options] projects subscriptions-modify-push-config <subscription> -r <kv>... [-p <v>...] [-o <out>]
  pubsub1-beta2 [options] projects subscriptions-pull <subscription> -r <kv>... [-p <v>...] [-o <out>]
  pubsub1-beta2 [options] projects topics-create <name> -r <kv>... [-p <v>...] [-o <out>]
  pubsub1-beta2 [options] projects topics-delete <topic> [-p <v>...] [-o <out>]
  pubsub1-beta2 [options] projects topics-get <topic> [-p <v>...] [-o <out>]
  pubsub1-beta2 [options] projects topics-list <project> [-p <v>...] [-o <out>]
  pubsub1-beta2 [options] projects topics-publish <topic> -r <kv>... [-p <v>...] [-o <out>]
  pubsub1-beta2 [options] projects topics-subscriptions-list <topic> [-p <v>...] [-o <out>]
  pubsub1-beta2 --help

All documentation details can be found at
http://byron.github.io/google-apis-rs/google_pubsub1_beta2_cli/index.html

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
    hub: api::Pubsub<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _projects_subscriptions_acknowledge(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::AcknowledgeRequest::default();
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
                "ack-ids" => {
                        if request.ack_ids.is_none() {
                           request.ack_ids = Some(Default::default());
                        }
                                        request.ack_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.projects().subscriptions_acknowledge(request, &self.opt.arg_subscription);
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

    fn _projects_subscriptions_create(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Subscription::default();
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
            fn request_push_config_init(request: &mut api::Subscription) {
                if request.push_config.is_none() {
                    request.push_config = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "ack-deadline-seconds" => {
                        request.ack_deadline_seconds = Some(arg_from_str(value.unwrap_or("-0"), err, "ack-deadline-seconds", "integer"));
                    },
                "topic" => {
                        request.topic = Some(value.unwrap_or("").to_string());
                    },
                "push-config.attributes" => {
                        request_push_config_init(&mut request);
                        if request.push_config.as_mut().unwrap().attributes.is_none() {
                           request.push_config.as_mut().unwrap().attributes = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.push_config.as_mut().unwrap().attributes.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "push-config.push-endpoint" => {
                        request_push_config_init(&mut request);
                        request.push_config.as_mut().unwrap().push_endpoint = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_push_config_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.projects().subscriptions_create(request, &self.opt.arg_name);
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

    fn _projects_subscriptions_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.projects().subscriptions_delete(&self.opt.arg_subscription);
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

    fn _projects_subscriptions_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.projects().subscriptions_get(&self.opt.arg_subscription);
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

    fn _projects_subscriptions_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.projects().subscriptions_list(&self.opt.arg_project);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
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

    fn _projects_subscriptions_modify_ack_deadline(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::ModifyAckDeadlineRequest::default();
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
                "ack-deadline-seconds" => {
                        request.ack_deadline_seconds = Some(arg_from_str(value.unwrap_or("-0"), err, "ack-deadline-seconds", "integer"));
                    },
                "ack-id" => {
                        request.ack_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.projects().subscriptions_modify_ack_deadline(request, &self.opt.arg_subscription);
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

    fn _projects_subscriptions_modify_push_config(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::ModifyPushConfigRequest::default();
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
            fn request_push_config_init(request: &mut api::ModifyPushConfigRequest) {
                if request.push_config.is_none() {
                    request.push_config = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "push-config.attributes" => {
                        request_push_config_init(&mut request);
                        if request.push_config.as_mut().unwrap().attributes.is_none() {
                           request.push_config.as_mut().unwrap().attributes = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.push_config.as_mut().unwrap().attributes.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "push-config.push-endpoint" => {
                        request_push_config_init(&mut request);
                        request.push_config.as_mut().unwrap().push_endpoint = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.projects().subscriptions_modify_push_config(request, &self.opt.arg_subscription);
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

    fn _projects_subscriptions_pull(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::PullRequest::default();
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
                "return-immediately" => {
                        request.return_immediately = Some(arg_from_str(value.unwrap_or("false"), err, "return-immediately", "boolean"));
                    },
                "max-messages" => {
                        request.max_messages = Some(arg_from_str(value.unwrap_or("-0"), err, "max-messages", "integer"));
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.projects().subscriptions_pull(request, &self.opt.arg_subscription);
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

    fn _projects_topics_create(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Topic::default();
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
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.projects().topics_create(request, &self.opt.arg_name);
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

    fn _projects_topics_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.projects().topics_delete(&self.opt.arg_topic);
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

    fn _projects_topics_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.projects().topics_get(&self.opt.arg_topic);
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

    fn _projects_topics_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.projects().topics_list(&self.opt.arg_project);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
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

    fn _projects_topics_publish(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::PublishRequest::default();
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
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.projects().topics_publish(request, &self.opt.arg_topic);
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

    fn _projects_topics_subscriptions_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.projects().topics_subscriptions_list(&self.opt.arg_topic);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
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

        if self.opt.cmd_projects {
            if self.opt.cmd_subscriptions_acknowledge {
                call_result = self._projects_subscriptions_acknowledge(dry_run, &mut err);
            } else if self.opt.cmd_subscriptions_create {
                call_result = self._projects_subscriptions_create(dry_run, &mut err);
            } else if self.opt.cmd_subscriptions_delete {
                call_result = self._projects_subscriptions_delete(dry_run, &mut err);
            } else if self.opt.cmd_subscriptions_get {
                call_result = self._projects_subscriptions_get(dry_run, &mut err);
            } else if self.opt.cmd_subscriptions_list {
                call_result = self._projects_subscriptions_list(dry_run, &mut err);
            } else if self.opt.cmd_subscriptions_modify_ack_deadline {
                call_result = self._projects_subscriptions_modify_ack_deadline(dry_run, &mut err);
            } else if self.opt.cmd_subscriptions_modify_push_config {
                call_result = self._projects_subscriptions_modify_push_config(dry_run, &mut err);
            } else if self.opt.cmd_subscriptions_pull {
                call_result = self._projects_subscriptions_pull(dry_run, &mut err);
            } else if self.opt.cmd_topics_create {
                call_result = self._projects_topics_create(dry_run, &mut err);
            } else if self.opt.cmd_topics_delete {
                call_result = self._projects_topics_delete(dry_run, &mut err);
            } else if self.opt.cmd_topics_get {
                call_result = self._projects_topics_get(dry_run, &mut err);
            } else if self.opt.cmd_topics_list {
                call_result = self._projects_topics_list(dry_run, &mut err);
            } else if self.opt.cmd_topics_publish {
                call_result = self._projects_topics_publish(dry_run, &mut err);
            } else if self.opt.cmd_topics_subscriptions_list {
                call_result = self._projects_topics_subscriptions_list(dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "pubsub1-beta2-secret.json", 
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
                                          program_name: "pubsub1-beta2",
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
            hub: api::Pubsub::new(client, auth),
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