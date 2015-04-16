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
extern crate google_pubsub1_beta2 as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  pubsub1-beta2 [options] projects subscriptions-acknowledge <subscription> -r <kv>... [-p <v>]... [-o <out>]
  pubsub1-beta2 [options] projects subscriptions-create <name> -r <kv>... [-p <v>]... [-o <out>]
  pubsub1-beta2 [options] projects subscriptions-delete <subscription> [-p <v>]... [-o <out>]
  pubsub1-beta2 [options] projects subscriptions-get <subscription> [-p <v>]... [-o <out>]
  pubsub1-beta2 [options] projects subscriptions-list <project> [-p <v>]... [-o <out>]
  pubsub1-beta2 [options] projects subscriptions-modify-ack-deadline <subscription> -r <kv>... [-p <v>]... [-o <out>]
  pubsub1-beta2 [options] projects subscriptions-modify-push-config <subscription> -r <kv>... [-p <v>]... [-o <out>]
  pubsub1-beta2 [options] projects subscriptions-pull <subscription> -r <kv>... [-p <v>]... [-o <out>]
  pubsub1-beta2 [options] projects topics-create <name> -r <kv>... [-p <v>]... [-o <out>]
  pubsub1-beta2 [options] projects topics-delete <topic> [-p <v>]... [-o <out>]
  pubsub1-beta2 [options] projects topics-get <topic> [-p <v>]... [-o <out>]
  pubsub1-beta2 [options] projects topics-list <project> [-p <v>]... [-o <out>]
  pubsub1-beta2 [options] projects topics-publish <topic> -r <kv>... [-p <v>]... [-o <out>]
  pubsub1-beta2 [options] projects topics-subscriptions-list <topic> [-p <v>]... [-o <out>]
  pubsub1-beta2 --help

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
    hub: api::Pubsub<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _projects_subscriptions_acknowledge(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::AcknowledgeRequest = Default::default();
        let mut call = self.hub.projects().subscriptions_acknowledge(&request, &self.opt.arg_subscription);
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
                "ack-ids" => {
                        if request.ack_ids.is_none() {
                            request.ack_ids = Some(Default::default());
                        }
                        request.ack_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
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

    fn _projects_subscriptions_create(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Subscription = Default::default();
        let mut call = self.hub.projects().subscriptions_create(&request, &self.opt.arg_name);
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
            fn request_push_config_init(request: &mut api::Subscription) {
                if request.push_config.is_none() {
                    request.push_config = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "ack-deadline-seconds" => {
                        request.ack_deadline_seconds = Some(arg_from_str(value.unwrap_or("-0"), err, "ack-deadline-seconds", "integer"));
                    },
                "topic" => {
                        request.topic = Some(value.unwrap_or("").to_string());
                    },
                "push-config.attributes" => {
                        request_push_config_init(&mut request);
                        request.push_config.as_mut().unwrap().attributes.push(value.unwrap_or("").to_string());
                    },
                "push-config.push-endpoint" => {
                        request_push_config_init(&mut request);
                        request.push_config.as_mut().unwrap().push_endpoint = value.unwrap_or("").to_string();
                    },
                "name" => {
                        request_push_config_init(&mut request);
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

    fn _projects_subscriptions_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.projects().subscriptions_delete(&self.opt.arg_subscription);
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

    fn _projects_subscriptions_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.projects().subscriptions_get(&self.opt.arg_subscription);
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

    fn _projects_subscriptions_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.projects().subscriptions_list(&self.opt.arg_project);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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

    fn _projects_subscriptions_modify_ack_deadline(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::ModifyAckDeadlineRequest = Default::default();
        let mut call = self.hub.projects().subscriptions_modify_ack_deadline(&request, &self.opt.arg_subscription);
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
                "ack-deadline-seconds" => {
                        request.ack_deadline_seconds = Some(arg_from_str(value.unwrap_or("-0"), err, "ack-deadline-seconds", "integer"));
                    },
                "ack-id" => {
                        request.ack_id = Some(value.unwrap_or("").to_string());
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

    fn _projects_subscriptions_modify_push_config(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::ModifyPushConfigRequest = Default::default();
        let mut call = self.hub.projects().subscriptions_modify_push_config(&request, &self.opt.arg_subscription);
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
            fn request_push_config_init(request: &mut api::ModifyPushConfigRequest) {
                if request.push_config.is_none() {
                    request.push_config = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "push-config.attributes" => {
                        request_push_config_init(&mut request);
                        request.push_config.as_mut().unwrap().attributes.push(value.unwrap_or("").to_string());
                    },
                "push-config.push-endpoint" => {
                        request_push_config_init(&mut request);
                        request.push_config.as_mut().unwrap().push_endpoint = value.unwrap_or("").to_string();
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

    fn _projects_subscriptions_pull(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::PullRequest = Default::default();
        let mut call = self.hub.projects().subscriptions_pull(&request, &self.opt.arg_subscription);
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
                "return-immediately" => {
                        request.return_immediately = Some(arg_from_str(value.unwrap_or("false"), err, "return-immediately", "boolean"));
                    },
                "max-messages" => {
                        request.max_messages = Some(arg_from_str(value.unwrap_or("-0"), err, "max-messages", "integer"));
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

    fn _projects_topics_create(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Topic = Default::default();
        let mut call = self.hub.projects().topics_create(&request, &self.opt.arg_name);
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

    fn _projects_topics_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.projects().topics_delete(&self.opt.arg_topic);
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

    fn _projects_topics_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.projects().topics_get(&self.opt.arg_topic);
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

    fn _projects_topics_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.projects().topics_list(&self.opt.arg_project);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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

    fn _projects_topics_publish(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::PublishRequest = Default::default();
        let mut call = self.hub.projects().topics_publish(&request, &self.opt.arg_topic);
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

    fn _projects_topics_subscriptions_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.projects().topics_subscriptions_list(&self.opt.arg_topic);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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

            match cmn::application_secret_from_directory(&config_dir, "pubsub1-beta2-secret.json") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                                      hyper::Client::new(),
                                      JsonTokenStorage {
                                        program_name: "pubsub1-beta2",
                                        db_dir: config_dir.clone(),
                                      }, None);
        let engine = Engine {
            opt: opt,
            hub: api::Pubsub::new(hyper::Client::new(), auth),
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