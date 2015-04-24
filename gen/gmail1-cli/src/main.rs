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
extern crate google_gmail1 as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  gmail1 [options] users drafts-create <user-id> -r <kv>... -u (simple|resumable) <file> <mime> [-p <v>]... [-o <out>]
  gmail1 [options] users drafts-delete <user-id> <id> [-p <v>]...
  gmail1 [options] users drafts-get <user-id> <id> [-p <v>]... [-o <out>]
  gmail1 [options] users drafts-list <user-id> [-p <v>]... [-o <out>]
  gmail1 [options] users drafts-send <user-id> -r <kv>... -u (simple|resumable) <file> <mime> [-p <v>]... [-o <out>]
  gmail1 [options] users drafts-update <user-id> <id> -r <kv>... -u (simple|resumable) <file> <mime> [-p <v>]... [-o <out>]
  gmail1 [options] users get-profile <user-id> [-p <v>]... [-o <out>]
  gmail1 [options] users history-list <user-id> [-p <v>]... [-o <out>]
  gmail1 [options] users labels-create <user-id> -r <kv>... [-p <v>]... [-o <out>]
  gmail1 [options] users labels-delete <user-id> <id> [-p <v>]...
  gmail1 [options] users labels-get <user-id> <id> [-p <v>]... [-o <out>]
  gmail1 [options] users labels-list <user-id> [-p <v>]... [-o <out>]
  gmail1 [options] users labels-patch <user-id> <id> -r <kv>... [-p <v>]... [-o <out>]
  gmail1 [options] users labels-update <user-id> <id> -r <kv>... [-p <v>]... [-o <out>]
  gmail1 [options] users messages-attachments-get <user-id> <message-id> <id> [-p <v>]... [-o <out>]
  gmail1 [options] users messages-delete <user-id> <id> [-p <v>]...
  gmail1 [options] users messages-get <user-id> <id> [-p <v>]... [-o <out>]
  gmail1 [options] users messages-import <user-id> -r <kv>... -u (simple|resumable) <file> <mime> [-p <v>]... [-o <out>]
  gmail1 [options] users messages-insert <user-id> -r <kv>... -u (simple|resumable) <file> <mime> [-p <v>]... [-o <out>]
  gmail1 [options] users messages-list <user-id> [-p <v>]... [-o <out>]
  gmail1 [options] users messages-modify <user-id> <id> -r <kv>... [-p <v>]... [-o <out>]
  gmail1 [options] users messages-send <user-id> -r <kv>... -u (simple|resumable) <file> <mime> [-p <v>]... [-o <out>]
  gmail1 [options] users messages-trash <user-id> <id> [-p <v>]... [-o <out>]
  gmail1 [options] users messages-untrash <user-id> <id> [-p <v>]... [-o <out>]
  gmail1 [options] users threads-delete <user-id> <id> [-p <v>]...
  gmail1 [options] users threads-get <user-id> <id> [-p <v>]... [-o <out>]
  gmail1 [options] users threads-list <user-id> [-p <v>]... [-o <out>]
  gmail1 [options] users threads-modify <user-id> <id> -r <kv>... [-p <v>]... [-o <out>]
  gmail1 [options] users threads-trash <user-id> <id> [-p <v>]... [-o <out>]
  gmail1 [options] users threads-untrash <user-id> <id> [-p <v>]... [-o <out>]
  gmail1 --help

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
    hub: api::Gmail<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _users_drafts_create(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Draft::default();
        let mut call = self.hub.users().drafts_create(&request, &self.opt.arg_user_id);
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
            fn request_message_init(request: &mut api::Draft) {
                if request.message.is_none() {
                    request.message = Some(Default::default());
                }
            }
            
            fn request_message_payload_body_init(request: &mut api::Draft) {
                request_message_payload_init(request);
                if request.message.as_mut().unwrap().payload.as_mut().unwrap().body.is_none() {
                    request.message.as_mut().unwrap().payload.as_mut().unwrap().body = Some(Default::default());
                }
            }
            
            fn request_message_payload_init(request: &mut api::Draft) {
                request_message_init(request);
                if request.message.as_mut().unwrap().payload.is_none() {
                    request.message.as_mut().unwrap().payload = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "message.history-id" => {
                        request_message_init(&mut request);
                        request.message.as_mut().unwrap().history_id = Some(value.unwrap_or("").to_string());
                    },
                "message.payload.body.data" => {
                        request_message_payload_body_init(&mut request);
                        request.message.as_mut().unwrap().payload.as_mut().unwrap().body.as_mut().unwrap().data = Some(value.unwrap_or("").to_string());
                    },
                "message.payload.body.attachment-id" => {
                        request_message_payload_body_init(&mut request);
                        request.message.as_mut().unwrap().payload.as_mut().unwrap().body.as_mut().unwrap().attachment_id = Some(value.unwrap_or("").to_string());
                    },
                "message.payload.body.size" => {
                        request_message_payload_body_init(&mut request);
                        request.message.as_mut().unwrap().payload.as_mut().unwrap().body.as_mut().unwrap().size = Some(arg_from_str(value.unwrap_or("-0"), err, "message.payload.body.size", "integer"));
                    },
                "message.payload.mime-type" => {
                        request_message_payload_body_init(&mut request);
                        request.message.as_mut().unwrap().payload.as_mut().unwrap().mime_type = Some(value.unwrap_or("").to_string());
                    },
                "message.payload.part-id" => {
                        request_message_payload_body_init(&mut request);
                        request.message.as_mut().unwrap().payload.as_mut().unwrap().part_id = Some(value.unwrap_or("").to_string());
                    },
                "message.payload.filename" => {
                        request_message_payload_body_init(&mut request);
                        request.message.as_mut().unwrap().payload.as_mut().unwrap().filename = Some(value.unwrap_or("").to_string());
                    },
                "message.snippet" => {
                        request_message_payload_init(&mut request);
                        request.message.as_mut().unwrap().snippet = Some(value.unwrap_or("").to_string());
                    },
                "message.raw" => {
                        request_message_payload_init(&mut request);
                        request.message.as_mut().unwrap().raw = Some(value.unwrap_or("").to_string());
                    },
                "message.size-estimate" => {
                        request_message_payload_init(&mut request);
                        request.message.as_mut().unwrap().size_estimate = Some(arg_from_str(value.unwrap_or("-0"), err, "message.size-estimate", "integer"));
                    },
                "message.thread-id" => {
                        request_message_payload_init(&mut request);
                        request.message.as_mut().unwrap().thread_id = Some(value.unwrap_or("").to_string());
                    },
                "message.label-ids" => {
                        request_message_payload_init(&mut request);
                        if request.message.as_mut().unwrap().label_ids.is_none() {
                           request.message.as_mut().unwrap().label_ids = Some(Default::default());
                        }
                                        request.message.as_mut().unwrap().label_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "message.id" => {
                        request_message_payload_init(&mut request);
                        request.message.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_message_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
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

    fn _users_drafts_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.users().drafts_delete(&self.opt.arg_user_id, &self.opt.arg_id);
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

    fn _users_drafts_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.users().drafts_get(&self.opt.arg_user_id, &self.opt.arg_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "format" => {
                    call = call.format(value.unwrap_or(""));
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

    fn _users_drafts_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.users().drafts_list(&self.opt.arg_user_id);
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

    fn _users_drafts_send(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Draft::default();
        let mut call = self.hub.users().drafts_send(&request, &self.opt.arg_user_id);
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
            fn request_message_init(request: &mut api::Draft) {
                if request.message.is_none() {
                    request.message = Some(Default::default());
                }
            }
            
            fn request_message_payload_body_init(request: &mut api::Draft) {
                request_message_payload_init(request);
                if request.message.as_mut().unwrap().payload.as_mut().unwrap().body.is_none() {
                    request.message.as_mut().unwrap().payload.as_mut().unwrap().body = Some(Default::default());
                }
            }
            
            fn request_message_payload_init(request: &mut api::Draft) {
                request_message_init(request);
                if request.message.as_mut().unwrap().payload.is_none() {
                    request.message.as_mut().unwrap().payload = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "message.history-id" => {
                        request_message_init(&mut request);
                        request.message.as_mut().unwrap().history_id = Some(value.unwrap_or("").to_string());
                    },
                "message.payload.body.data" => {
                        request_message_payload_body_init(&mut request);
                        request.message.as_mut().unwrap().payload.as_mut().unwrap().body.as_mut().unwrap().data = Some(value.unwrap_or("").to_string());
                    },
                "message.payload.body.attachment-id" => {
                        request_message_payload_body_init(&mut request);
                        request.message.as_mut().unwrap().payload.as_mut().unwrap().body.as_mut().unwrap().attachment_id = Some(value.unwrap_or("").to_string());
                    },
                "message.payload.body.size" => {
                        request_message_payload_body_init(&mut request);
                        request.message.as_mut().unwrap().payload.as_mut().unwrap().body.as_mut().unwrap().size = Some(arg_from_str(value.unwrap_or("-0"), err, "message.payload.body.size", "integer"));
                    },
                "message.payload.mime-type" => {
                        request_message_payload_body_init(&mut request);
                        request.message.as_mut().unwrap().payload.as_mut().unwrap().mime_type = Some(value.unwrap_or("").to_string());
                    },
                "message.payload.part-id" => {
                        request_message_payload_body_init(&mut request);
                        request.message.as_mut().unwrap().payload.as_mut().unwrap().part_id = Some(value.unwrap_or("").to_string());
                    },
                "message.payload.filename" => {
                        request_message_payload_body_init(&mut request);
                        request.message.as_mut().unwrap().payload.as_mut().unwrap().filename = Some(value.unwrap_or("").to_string());
                    },
                "message.snippet" => {
                        request_message_payload_init(&mut request);
                        request.message.as_mut().unwrap().snippet = Some(value.unwrap_or("").to_string());
                    },
                "message.raw" => {
                        request_message_payload_init(&mut request);
                        request.message.as_mut().unwrap().raw = Some(value.unwrap_or("").to_string());
                    },
                "message.size-estimate" => {
                        request_message_payload_init(&mut request);
                        request.message.as_mut().unwrap().size_estimate = Some(arg_from_str(value.unwrap_or("-0"), err, "message.size-estimate", "integer"));
                    },
                "message.thread-id" => {
                        request_message_payload_init(&mut request);
                        request.message.as_mut().unwrap().thread_id = Some(value.unwrap_or("").to_string());
                    },
                "message.label-ids" => {
                        request_message_payload_init(&mut request);
                        if request.message.as_mut().unwrap().label_ids.is_none() {
                           request.message.as_mut().unwrap().label_ids = Some(Default::default());
                        }
                                        request.message.as_mut().unwrap().label_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "message.id" => {
                        request_message_payload_init(&mut request);
                        request.message.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_message_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
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

    fn _users_drafts_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Draft::default();
        let mut call = self.hub.users().drafts_update(&request, &self.opt.arg_user_id, &self.opt.arg_id);
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
            fn request_message_init(request: &mut api::Draft) {
                if request.message.is_none() {
                    request.message = Some(Default::default());
                }
            }
            
            fn request_message_payload_body_init(request: &mut api::Draft) {
                request_message_payload_init(request);
                if request.message.as_mut().unwrap().payload.as_mut().unwrap().body.is_none() {
                    request.message.as_mut().unwrap().payload.as_mut().unwrap().body = Some(Default::default());
                }
            }
            
            fn request_message_payload_init(request: &mut api::Draft) {
                request_message_init(request);
                if request.message.as_mut().unwrap().payload.is_none() {
                    request.message.as_mut().unwrap().payload = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "message.history-id" => {
                        request_message_init(&mut request);
                        request.message.as_mut().unwrap().history_id = Some(value.unwrap_or("").to_string());
                    },
                "message.payload.body.data" => {
                        request_message_payload_body_init(&mut request);
                        request.message.as_mut().unwrap().payload.as_mut().unwrap().body.as_mut().unwrap().data = Some(value.unwrap_or("").to_string());
                    },
                "message.payload.body.attachment-id" => {
                        request_message_payload_body_init(&mut request);
                        request.message.as_mut().unwrap().payload.as_mut().unwrap().body.as_mut().unwrap().attachment_id = Some(value.unwrap_or("").to_string());
                    },
                "message.payload.body.size" => {
                        request_message_payload_body_init(&mut request);
                        request.message.as_mut().unwrap().payload.as_mut().unwrap().body.as_mut().unwrap().size = Some(arg_from_str(value.unwrap_or("-0"), err, "message.payload.body.size", "integer"));
                    },
                "message.payload.mime-type" => {
                        request_message_payload_body_init(&mut request);
                        request.message.as_mut().unwrap().payload.as_mut().unwrap().mime_type = Some(value.unwrap_or("").to_string());
                    },
                "message.payload.part-id" => {
                        request_message_payload_body_init(&mut request);
                        request.message.as_mut().unwrap().payload.as_mut().unwrap().part_id = Some(value.unwrap_or("").to_string());
                    },
                "message.payload.filename" => {
                        request_message_payload_body_init(&mut request);
                        request.message.as_mut().unwrap().payload.as_mut().unwrap().filename = Some(value.unwrap_or("").to_string());
                    },
                "message.snippet" => {
                        request_message_payload_init(&mut request);
                        request.message.as_mut().unwrap().snippet = Some(value.unwrap_or("").to_string());
                    },
                "message.raw" => {
                        request_message_payload_init(&mut request);
                        request.message.as_mut().unwrap().raw = Some(value.unwrap_or("").to_string());
                    },
                "message.size-estimate" => {
                        request_message_payload_init(&mut request);
                        request.message.as_mut().unwrap().size_estimate = Some(arg_from_str(value.unwrap_or("-0"), err, "message.size-estimate", "integer"));
                    },
                "message.thread-id" => {
                        request_message_payload_init(&mut request);
                        request.message.as_mut().unwrap().thread_id = Some(value.unwrap_or("").to_string());
                    },
                "message.label-ids" => {
                        request_message_payload_init(&mut request);
                        if request.message.as_mut().unwrap().label_ids.is_none() {
                           request.message.as_mut().unwrap().label_ids = Some(Default::default());
                        }
                                        request.message.as_mut().unwrap().label_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "message.id" => {
                        request_message_payload_init(&mut request);
                        request.message.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_message_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
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

    fn _users_get_profile(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.users().get_profile(&self.opt.arg_user_id);
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

    fn _users_history_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.users().history_list(&self.opt.arg_user_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-history-id" => {
                    call = call.start_history_id(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "label-id" => {
                    call = call.label_id(value.unwrap_or(""));
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

    fn _users_labels_create(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Label::default();
        let mut call = self.hub.users().labels_create(&request, &self.opt.arg_user_id);
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
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "messages-total" => {
                        request.messages_total = Some(arg_from_str(value.unwrap_or("-0"), err, "messages-total", "integer"));
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "threads-total" => {
                        request.threads_total = Some(arg_from_str(value.unwrap_or("-0"), err, "threads-total", "integer"));
                    },
                "label-list-visibility" => {
                        request.label_list_visibility = Some(value.unwrap_or("").to_string());
                    },
                "threads-unread" => {
                        request.threads_unread = Some(arg_from_str(value.unwrap_or("-0"), err, "threads-unread", "integer"));
                    },
                "message-list-visibility" => {
                        request.message_list_visibility = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "messages-unread" => {
                        request.messages_unread = Some(arg_from_str(value.unwrap_or("-0"), err, "messages-unread", "integer"));
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

    fn _users_labels_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.users().labels_delete(&self.opt.arg_user_id, &self.opt.arg_id);
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

    fn _users_labels_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.users().labels_get(&self.opt.arg_user_id, &self.opt.arg_id);
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

    fn _users_labels_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.users().labels_list(&self.opt.arg_user_id);
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

    fn _users_labels_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Label::default();
        let mut call = self.hub.users().labels_patch(&request, &self.opt.arg_user_id, &self.opt.arg_id);
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
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "messages-total" => {
                        request.messages_total = Some(arg_from_str(value.unwrap_or("-0"), err, "messages-total", "integer"));
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "threads-total" => {
                        request.threads_total = Some(arg_from_str(value.unwrap_or("-0"), err, "threads-total", "integer"));
                    },
                "label-list-visibility" => {
                        request.label_list_visibility = Some(value.unwrap_or("").to_string());
                    },
                "threads-unread" => {
                        request.threads_unread = Some(arg_from_str(value.unwrap_or("-0"), err, "threads-unread", "integer"));
                    },
                "message-list-visibility" => {
                        request.message_list_visibility = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "messages-unread" => {
                        request.messages_unread = Some(arg_from_str(value.unwrap_or("-0"), err, "messages-unread", "integer"));
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

    fn _users_labels_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Label::default();
        let mut call = self.hub.users().labels_update(&request, &self.opt.arg_user_id, &self.opt.arg_id);
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
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "messages-total" => {
                        request.messages_total = Some(arg_from_str(value.unwrap_or("-0"), err, "messages-total", "integer"));
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "threads-total" => {
                        request.threads_total = Some(arg_from_str(value.unwrap_or("-0"), err, "threads-total", "integer"));
                    },
                "label-list-visibility" => {
                        request.label_list_visibility = Some(value.unwrap_or("").to_string());
                    },
                "threads-unread" => {
                        request.threads_unread = Some(arg_from_str(value.unwrap_or("-0"), err, "threads-unread", "integer"));
                    },
                "message-list-visibility" => {
                        request.message_list_visibility = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "messages-unread" => {
                        request.messages_unread = Some(arg_from_str(value.unwrap_or("-0"), err, "messages-unread", "integer"));
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

    fn _users_messages_attachments_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.users().messages_attachments_get(&self.opt.arg_user_id, &self.opt.arg_message_id, &self.opt.arg_id);
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

    fn _users_messages_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.users().messages_delete(&self.opt.arg_user_id, &self.opt.arg_id);
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

    fn _users_messages_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.users().messages_get(&self.opt.arg_user_id, &self.opt.arg_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "metadata-headers" => {
                    call = call.add_metadata_headers(value.unwrap_or(""));
                },
                "format" => {
                    call = call.format(value.unwrap_or(""));
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

    fn _users_messages_import(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Message::default();
        let mut call = self.hub.users().messages_import(&request, &self.opt.arg_user_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "process-for-calendar" => {
                    call = call.process_for_calendar(arg_from_str(value.unwrap_or("false"), err, "process-for-calendar", "boolean"));
                },
                "never-mark-spam" => {
                    call = call.never_mark_spam(arg_from_str(value.unwrap_or("false"), err, "never-mark-spam", "boolean"));
                },
                "internal-date-source" => {
                    call = call.internal_date_source(value.unwrap_or(""));
                },
                "deleted" => {
                    call = call.deleted(arg_from_str(value.unwrap_or("false"), err, "deleted", "boolean"));
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
            fn request_payload_body_init(request: &mut api::Message) {
                request_payload_init(request);
                if request.payload.as_mut().unwrap().body.is_none() {
                    request.payload.as_mut().unwrap().body = Some(Default::default());
                }
            }
            
            fn request_payload_init(request: &mut api::Message) {
                if request.payload.is_none() {
                    request.payload = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "history-id" => {
                        request.history_id = Some(value.unwrap_or("").to_string());
                    },
                "payload.body.data" => {
                        request_payload_body_init(&mut request);
                        request.payload.as_mut().unwrap().body.as_mut().unwrap().data = Some(value.unwrap_or("").to_string());
                    },
                "payload.body.attachment-id" => {
                        request_payload_body_init(&mut request);
                        request.payload.as_mut().unwrap().body.as_mut().unwrap().attachment_id = Some(value.unwrap_or("").to_string());
                    },
                "payload.body.size" => {
                        request_payload_body_init(&mut request);
                        request.payload.as_mut().unwrap().body.as_mut().unwrap().size = Some(arg_from_str(value.unwrap_or("-0"), err, "payload.body.size", "integer"));
                    },
                "payload.mime-type" => {
                        request_payload_body_init(&mut request);
                        request.payload.as_mut().unwrap().mime_type = Some(value.unwrap_or("").to_string());
                    },
                "payload.part-id" => {
                        request_payload_body_init(&mut request);
                        request.payload.as_mut().unwrap().part_id = Some(value.unwrap_or("").to_string());
                    },
                "payload.filename" => {
                        request_payload_body_init(&mut request);
                        request.payload.as_mut().unwrap().filename = Some(value.unwrap_or("").to_string());
                    },
                "snippet" => {
                        request_payload_init(&mut request);
                        request.snippet = Some(value.unwrap_or("").to_string());
                    },
                "raw" => {
                        request_payload_init(&mut request);
                        request.raw = Some(value.unwrap_or("").to_string());
                    },
                "size-estimate" => {
                        request_payload_init(&mut request);
                        request.size_estimate = Some(arg_from_str(value.unwrap_or("-0"), err, "size-estimate", "integer"));
                    },
                "thread-id" => {
                        request_payload_init(&mut request);
                        request.thread_id = Some(value.unwrap_or("").to_string());
                    },
                "label-ids" => {
                        request_payload_init(&mut request);
                        if request.label_ids.is_none() {
                           request.label_ids = Some(Default::default());
                        }
                                        request.label_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_payload_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
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

    fn _users_messages_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Message::default();
        let mut call = self.hub.users().messages_insert(&request, &self.opt.arg_user_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "internal-date-source" => {
                    call = call.internal_date_source(value.unwrap_or(""));
                },
                "deleted" => {
                    call = call.deleted(arg_from_str(value.unwrap_or("false"), err, "deleted", "boolean"));
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
            fn request_payload_body_init(request: &mut api::Message) {
                request_payload_init(request);
                if request.payload.as_mut().unwrap().body.is_none() {
                    request.payload.as_mut().unwrap().body = Some(Default::default());
                }
            }
            
            fn request_payload_init(request: &mut api::Message) {
                if request.payload.is_none() {
                    request.payload = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "history-id" => {
                        request.history_id = Some(value.unwrap_or("").to_string());
                    },
                "payload.body.data" => {
                        request_payload_body_init(&mut request);
                        request.payload.as_mut().unwrap().body.as_mut().unwrap().data = Some(value.unwrap_or("").to_string());
                    },
                "payload.body.attachment-id" => {
                        request_payload_body_init(&mut request);
                        request.payload.as_mut().unwrap().body.as_mut().unwrap().attachment_id = Some(value.unwrap_or("").to_string());
                    },
                "payload.body.size" => {
                        request_payload_body_init(&mut request);
                        request.payload.as_mut().unwrap().body.as_mut().unwrap().size = Some(arg_from_str(value.unwrap_or("-0"), err, "payload.body.size", "integer"));
                    },
                "payload.mime-type" => {
                        request_payload_body_init(&mut request);
                        request.payload.as_mut().unwrap().mime_type = Some(value.unwrap_or("").to_string());
                    },
                "payload.part-id" => {
                        request_payload_body_init(&mut request);
                        request.payload.as_mut().unwrap().part_id = Some(value.unwrap_or("").to_string());
                    },
                "payload.filename" => {
                        request_payload_body_init(&mut request);
                        request.payload.as_mut().unwrap().filename = Some(value.unwrap_or("").to_string());
                    },
                "snippet" => {
                        request_payload_init(&mut request);
                        request.snippet = Some(value.unwrap_or("").to_string());
                    },
                "raw" => {
                        request_payload_init(&mut request);
                        request.raw = Some(value.unwrap_or("").to_string());
                    },
                "size-estimate" => {
                        request_payload_init(&mut request);
                        request.size_estimate = Some(arg_from_str(value.unwrap_or("-0"), err, "size-estimate", "integer"));
                    },
                "thread-id" => {
                        request_payload_init(&mut request);
                        request.thread_id = Some(value.unwrap_or("").to_string());
                    },
                "label-ids" => {
                        request_payload_init(&mut request);
                        if request.label_ids.is_none() {
                           request.label_ids = Some(Default::default());
                        }
                                        request.label_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_payload_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
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

    fn _users_messages_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.users().messages_list(&self.opt.arg_user_id);
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
                "label-ids" => {
                    call = call.add_label_ids(value.unwrap_or(""));
                },
                "include-spam-trash" => {
                    call = call.include_spam_trash(arg_from_str(value.unwrap_or("false"), err, "include-spam-trash", "boolean"));
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

    fn _users_messages_modify(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::ModifyMessageRequest::default();
        let mut call = self.hub.users().messages_modify(&request, &self.opt.arg_user_id, &self.opt.arg_id);
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
                "remove-label-ids" => {
                        if request.remove_label_ids.is_none() {
                           request.remove_label_ids = Some(Default::default());
                        }
                                        request.remove_label_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "add-label-ids" => {
                        if request.add_label_ids.is_none() {
                           request.add_label_ids = Some(Default::default());
                        }
                                        request.add_label_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
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

    fn _users_messages_send(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Message::default();
        let mut call = self.hub.users().messages_send(&request, &self.opt.arg_user_id);
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
            fn request_payload_body_init(request: &mut api::Message) {
                request_payload_init(request);
                if request.payload.as_mut().unwrap().body.is_none() {
                    request.payload.as_mut().unwrap().body = Some(Default::default());
                }
            }
            
            fn request_payload_init(request: &mut api::Message) {
                if request.payload.is_none() {
                    request.payload = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "history-id" => {
                        request.history_id = Some(value.unwrap_or("").to_string());
                    },
                "payload.body.data" => {
                        request_payload_body_init(&mut request);
                        request.payload.as_mut().unwrap().body.as_mut().unwrap().data = Some(value.unwrap_or("").to_string());
                    },
                "payload.body.attachment-id" => {
                        request_payload_body_init(&mut request);
                        request.payload.as_mut().unwrap().body.as_mut().unwrap().attachment_id = Some(value.unwrap_or("").to_string());
                    },
                "payload.body.size" => {
                        request_payload_body_init(&mut request);
                        request.payload.as_mut().unwrap().body.as_mut().unwrap().size = Some(arg_from_str(value.unwrap_or("-0"), err, "payload.body.size", "integer"));
                    },
                "payload.mime-type" => {
                        request_payload_body_init(&mut request);
                        request.payload.as_mut().unwrap().mime_type = Some(value.unwrap_or("").to_string());
                    },
                "payload.part-id" => {
                        request_payload_body_init(&mut request);
                        request.payload.as_mut().unwrap().part_id = Some(value.unwrap_or("").to_string());
                    },
                "payload.filename" => {
                        request_payload_body_init(&mut request);
                        request.payload.as_mut().unwrap().filename = Some(value.unwrap_or("").to_string());
                    },
                "snippet" => {
                        request_payload_init(&mut request);
                        request.snippet = Some(value.unwrap_or("").to_string());
                    },
                "raw" => {
                        request_payload_init(&mut request);
                        request.raw = Some(value.unwrap_or("").to_string());
                    },
                "size-estimate" => {
                        request_payload_init(&mut request);
                        request.size_estimate = Some(arg_from_str(value.unwrap_or("-0"), err, "size-estimate", "integer"));
                    },
                "thread-id" => {
                        request_payload_init(&mut request);
                        request.thread_id = Some(value.unwrap_or("").to_string());
                    },
                "label-ids" => {
                        request_payload_init(&mut request);
                        if request.label_ids.is_none() {
                           request.label_ids = Some(Default::default());
                        }
                                        request.label_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_payload_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
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

    fn _users_messages_trash(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.users().messages_trash(&self.opt.arg_user_id, &self.opt.arg_id);
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

    fn _users_messages_untrash(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.users().messages_untrash(&self.opt.arg_user_id, &self.opt.arg_id);
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

    fn _users_threads_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.users().threads_delete(&self.opt.arg_user_id, &self.opt.arg_id);
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

    fn _users_threads_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.users().threads_get(&self.opt.arg_user_id, &self.opt.arg_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "metadata-headers" => {
                    call = call.add_metadata_headers(value.unwrap_or(""));
                },
                "format" => {
                    call = call.format(value.unwrap_or(""));
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

    fn _users_threads_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.users().threads_list(&self.opt.arg_user_id);
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
                "label-ids" => {
                    call = call.add_label_ids(value.unwrap_or(""));
                },
                "include-spam-trash" => {
                    call = call.include_spam_trash(arg_from_str(value.unwrap_or("false"), err, "include-spam-trash", "boolean"));
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

    fn _users_threads_modify(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::ModifyThreadRequest::default();
        let mut call = self.hub.users().threads_modify(&request, &self.opt.arg_user_id, &self.opt.arg_id);
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
                "remove-label-ids" => {
                        if request.remove_label_ids.is_none() {
                           request.remove_label_ids = Some(Default::default());
                        }
                                        request.remove_label_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "add-label-ids" => {
                        if request.add_label_ids.is_none() {
                           request.add_label_ids = Some(Default::default());
                        }
                                        request.add_label_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
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

    fn _users_threads_trash(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.users().threads_trash(&self.opt.arg_user_id, &self.opt.arg_id);
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

    fn _users_threads_untrash(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.users().threads_untrash(&self.opt.arg_user_id, &self.opt.arg_id);
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

        if self.opt.cmd_users {
            if self.opt.cmd_drafts_create {
                call_result = self._users_drafts_create(dry_run, &mut err);
            } else if self.opt.cmd_drafts_delete {
                call_result = self._users_drafts_delete(dry_run, &mut err);
            } else if self.opt.cmd_drafts_get {
                call_result = self._users_drafts_get(dry_run, &mut err);
            } else if self.opt.cmd_drafts_list {
                call_result = self._users_drafts_list(dry_run, &mut err);
            } else if self.opt.cmd_drafts_send {
                call_result = self._users_drafts_send(dry_run, &mut err);
            } else if self.opt.cmd_drafts_update {
                call_result = self._users_drafts_update(dry_run, &mut err);
            } else if self.opt.cmd_get_profile {
                call_result = self._users_get_profile(dry_run, &mut err);
            } else if self.opt.cmd_history_list {
                call_result = self._users_history_list(dry_run, &mut err);
            } else if self.opt.cmd_labels_create {
                call_result = self._users_labels_create(dry_run, &mut err);
            } else if self.opt.cmd_labels_delete {
                call_result = self._users_labels_delete(dry_run, &mut err);
            } else if self.opt.cmd_labels_get {
                call_result = self._users_labels_get(dry_run, &mut err);
            } else if self.opt.cmd_labels_list {
                call_result = self._users_labels_list(dry_run, &mut err);
            } else if self.opt.cmd_labels_patch {
                call_result = self._users_labels_patch(dry_run, &mut err);
            } else if self.opt.cmd_labels_update {
                call_result = self._users_labels_update(dry_run, &mut err);
            } else if self.opt.cmd_messages_attachments_get {
                call_result = self._users_messages_attachments_get(dry_run, &mut err);
            } else if self.opt.cmd_messages_delete {
                call_result = self._users_messages_delete(dry_run, &mut err);
            } else if self.opt.cmd_messages_get {
                call_result = self._users_messages_get(dry_run, &mut err);
            } else if self.opt.cmd_messages_import {
                call_result = self._users_messages_import(dry_run, &mut err);
            } else if self.opt.cmd_messages_insert {
                call_result = self._users_messages_insert(dry_run, &mut err);
            } else if self.opt.cmd_messages_list {
                call_result = self._users_messages_list(dry_run, &mut err);
            } else if self.opt.cmd_messages_modify {
                call_result = self._users_messages_modify(dry_run, &mut err);
            } else if self.opt.cmd_messages_send {
                call_result = self._users_messages_send(dry_run, &mut err);
            } else if self.opt.cmd_messages_trash {
                call_result = self._users_messages_trash(dry_run, &mut err);
            } else if self.opt.cmd_messages_untrash {
                call_result = self._users_messages_untrash(dry_run, &mut err);
            } else if self.opt.cmd_threads_delete {
                call_result = self._users_threads_delete(dry_run, &mut err);
            } else if self.opt.cmd_threads_get {
                call_result = self._users_threads_get(dry_run, &mut err);
            } else if self.opt.cmd_threads_list {
                call_result = self._users_threads_list(dry_run, &mut err);
            } else if self.opt.cmd_threads_modify {
                call_result = self._users_threads_modify(dry_run, &mut err);
            } else if self.opt.cmd_threads_trash {
                call_result = self._users_threads_trash(dry_run, &mut err);
            } else if self.opt.cmd_threads_untrash {
                call_result = self._users_threads_untrash(dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "gmail1-secret.json", 
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
                                          program_name: "gmail1",
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
            hub: api::Gmail::new(client, auth),
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