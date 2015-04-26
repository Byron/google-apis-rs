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
extern crate google_mirror1 as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  mirror1 [options] accounts insert <user-token> <account-type> <account-name> -r <kv>... [-p <v>...] [-o <out>]
  mirror1 [options] contacts delete <id> [-p <v>...]
  mirror1 [options] contacts get <id> [-p <v>...] [-o <out>]
  mirror1 [options] contacts insert -r <kv>... [-p <v>...] [-o <out>]
  mirror1 [options] contacts list [-p <v>...] [-o <out>]
  mirror1 [options] contacts patch <id> -r <kv>... [-p <v>...] [-o <out>]
  mirror1 [options] contacts update <id> -r <kv>... [-p <v>...] [-o <out>]
  mirror1 [options] locations get <id> [-p <v>...] [-o <out>]
  mirror1 [options] locations list [-p <v>...] [-o <out>]
  mirror1 [options] settings get <id> [-p <v>...] [-o <out>]
  mirror1 [options] subscriptions delete <id> [-p <v>...]
  mirror1 [options] subscriptions insert -r <kv>... [-p <v>...] [-o <out>]
  mirror1 [options] subscriptions list [-p <v>...] [-o <out>]
  mirror1 [options] subscriptions update <id> -r <kv>... [-p <v>...] [-o <out>]
  mirror1 [options] timeline attachments-delete <item-id> <attachment-id> [-p <v>...]
  mirror1 [options] timeline attachments-get <item-id> <attachment-id> [-p <v>...] [-o <out>]
  mirror1 [options] timeline attachments-insert <item-id> -u (simple|resumable) <file> <mime> [-p <v>...] [-o <out>]
  mirror1 [options] timeline attachments-list <item-id> [-p <v>...] [-o <out>]
  mirror1 [options] timeline delete <id> [-p <v>...]
  mirror1 [options] timeline get <id> [-p <v>...] [-o <out>]
  mirror1 [options] timeline insert -r <kv>... -u (simple|resumable) <file> <mime> [-p <v>...] [-o <out>]
  mirror1 [options] timeline list [-p <v>...] [-o <out>]
  mirror1 [options] timeline patch <id> -r <kv>... [-p <v>...] [-o <out>]
  mirror1 [options] timeline update <id> -r <kv>... -u (simple|resumable) <file> <mime> [-p <v>...] [-o <out>]
  mirror1 --help

All documentation details can be found at
http://byron.github.io/google-apis-rs/google_mirror1_cli/index.html

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
    hub: api::Mirror<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _accounts_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Account::default();
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
                "password" => {
                        request.password = Some(value.unwrap_or("").to_string());
                    },
                "features" => {
                        if request.features.is_none() {
                           request.features = Some(Default::default());
                        }
                                        request.features.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.accounts().insert(request, &self.opt.arg_user_token, &self.opt.arg_account_type, &self.opt.arg_account_name);
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

    fn _contacts_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.contacts().delete(&self.opt.arg_id);
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

    fn _contacts_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.contacts().get(&self.opt.arg_id);
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

    fn _contacts_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Contact::default();
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
                "display-name" => {
                        request.display_name = Some(value.unwrap_or("").to_string());
                    },
                "accept-types" => {
                        if request.accept_types.is_none() {
                           request.accept_types = Some(Default::default());
                        }
                                        request.accept_types.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "image-urls" => {
                        if request.image_urls.is_none() {
                           request.image_urls = Some(Default::default());
                        }
                                        request.image_urls.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "priority" => {
                        request.priority = Some(arg_from_str(value.unwrap_or("-0"), err, "priority", "integer"));
                    },
                "source" => {
                        request.source = Some(value.unwrap_or("").to_string());
                    },
                "phone-number" => {
                        request.phone_number = Some(value.unwrap_or("").to_string());
                    },
                "sharing-features" => {
                        if request.sharing_features.is_none() {
                           request.sharing_features = Some(Default::default());
                        }
                                        request.sharing_features.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "speakable-name" => {
                        request.speakable_name = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.contacts().insert(request);
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

    fn _contacts_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.contacts().list();
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

    fn _contacts_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Contact::default();
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
                "display-name" => {
                        request.display_name = Some(value.unwrap_or("").to_string());
                    },
                "accept-types" => {
                        if request.accept_types.is_none() {
                           request.accept_types = Some(Default::default());
                        }
                                        request.accept_types.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "image-urls" => {
                        if request.image_urls.is_none() {
                           request.image_urls = Some(Default::default());
                        }
                                        request.image_urls.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "priority" => {
                        request.priority = Some(arg_from_str(value.unwrap_or("-0"), err, "priority", "integer"));
                    },
                "source" => {
                        request.source = Some(value.unwrap_or("").to_string());
                    },
                "phone-number" => {
                        request.phone_number = Some(value.unwrap_or("").to_string());
                    },
                "sharing-features" => {
                        if request.sharing_features.is_none() {
                           request.sharing_features = Some(Default::default());
                        }
                                        request.sharing_features.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "speakable-name" => {
                        request.speakable_name = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.contacts().patch(request, &self.opt.arg_id);
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

    fn _contacts_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Contact::default();
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
                "display-name" => {
                        request.display_name = Some(value.unwrap_or("").to_string());
                    },
                "accept-types" => {
                        if request.accept_types.is_none() {
                           request.accept_types = Some(Default::default());
                        }
                                        request.accept_types.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "image-urls" => {
                        if request.image_urls.is_none() {
                           request.image_urls = Some(Default::default());
                        }
                                        request.image_urls.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "priority" => {
                        request.priority = Some(arg_from_str(value.unwrap_or("-0"), err, "priority", "integer"));
                    },
                "source" => {
                        request.source = Some(value.unwrap_or("").to_string());
                    },
                "phone-number" => {
                        request.phone_number = Some(value.unwrap_or("").to_string());
                    },
                "sharing-features" => {
                        if request.sharing_features.is_none() {
                           request.sharing_features = Some(Default::default());
                        }
                                        request.sharing_features.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "speakable-name" => {
                        request.speakable_name = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.contacts().update(request, &self.opt.arg_id);
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

    fn _locations_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.locations().get(&self.opt.arg_id);
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

    fn _locations_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.locations().list();
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
        let mut call = self.hub.settings().get(&self.opt.arg_id);
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

    fn _subscriptions_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.subscriptions().delete(&self.opt.arg_id);
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

    fn _subscriptions_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
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
            fn request_notification_init(request: &mut api::Subscription) {
                if request.notification.is_none() {
                    request.notification = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "notification.item-id" => {
                        request_notification_init(&mut request);
                        request.notification.as_mut().unwrap().item_id = Some(value.unwrap_or("").to_string());
                    },
                "notification.user-token" => {
                        request_notification_init(&mut request);
                        request.notification.as_mut().unwrap().user_token = Some(value.unwrap_or("").to_string());
                    },
                "notification.operation" => {
                        request_notification_init(&mut request);
                        request.notification.as_mut().unwrap().operation = Some(value.unwrap_or("").to_string());
                    },
                "notification.verify-token" => {
                        request_notification_init(&mut request);
                        request.notification.as_mut().unwrap().verify_token = Some(value.unwrap_or("").to_string());
                    },
                "notification.collection" => {
                        request_notification_init(&mut request);
                        request.notification.as_mut().unwrap().collection = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request_notification_init(&mut request);
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "collection" => {
                        request_notification_init(&mut request);
                        request.collection = Some(value.unwrap_or("").to_string());
                    },
                "verify-token" => {
                        request_notification_init(&mut request);
                        request.verify_token = Some(value.unwrap_or("").to_string());
                    },
                "user-token" => {
                        request_notification_init(&mut request);
                        request.user_token = Some(value.unwrap_or("").to_string());
                    },
                "operation" => {
                        request_notification_init(&mut request);
                        if request.operation.is_none() {
                           request.operation = Some(Default::default());
                        }
                                        request.operation.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_notification_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "callback-url" => {
                        request_notification_init(&mut request);
                        request.callback_url = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.subscriptions().insert(request);
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

    fn _subscriptions_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.subscriptions().list();
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

    fn _subscriptions_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
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
            fn request_notification_init(request: &mut api::Subscription) {
                if request.notification.is_none() {
                    request.notification = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "notification.item-id" => {
                        request_notification_init(&mut request);
                        request.notification.as_mut().unwrap().item_id = Some(value.unwrap_or("").to_string());
                    },
                "notification.user-token" => {
                        request_notification_init(&mut request);
                        request.notification.as_mut().unwrap().user_token = Some(value.unwrap_or("").to_string());
                    },
                "notification.operation" => {
                        request_notification_init(&mut request);
                        request.notification.as_mut().unwrap().operation = Some(value.unwrap_or("").to_string());
                    },
                "notification.verify-token" => {
                        request_notification_init(&mut request);
                        request.notification.as_mut().unwrap().verify_token = Some(value.unwrap_or("").to_string());
                    },
                "notification.collection" => {
                        request_notification_init(&mut request);
                        request.notification.as_mut().unwrap().collection = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request_notification_init(&mut request);
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "collection" => {
                        request_notification_init(&mut request);
                        request.collection = Some(value.unwrap_or("").to_string());
                    },
                "verify-token" => {
                        request_notification_init(&mut request);
                        request.verify_token = Some(value.unwrap_or("").to_string());
                    },
                "user-token" => {
                        request_notification_init(&mut request);
                        request.user_token = Some(value.unwrap_or("").to_string());
                    },
                "operation" => {
                        request_notification_init(&mut request);
                        if request.operation.is_none() {
                           request.operation = Some(Default::default());
                        }
                                        request.operation.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_notification_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "callback-url" => {
                        request_notification_init(&mut request);
                        request.callback_url = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.subscriptions().update(request, &self.opt.arg_id);
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

    fn _timeline_attachments_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.timeline().attachments_delete(&self.opt.arg_item_id, &self.opt.arg_attachment_id);
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

    fn _timeline_attachments_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut download_mode = false;
        let mut call = self.hub.timeline().attachments_get(&self.opt.arg_item_id, &self.opt.arg_attachment_id);
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

    fn _timeline_attachments_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.timeline().attachments_insert(&self.opt.arg_item_id);
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

    fn _timeline_attachments_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.timeline().attachments_list(&self.opt.arg_item_id);
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

    fn _timeline_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.timeline().delete(&self.opt.arg_id);
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

    fn _timeline_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.timeline().get(&self.opt.arg_id);
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

    fn _timeline_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::TimelineItem::default();
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
            fn request_creator_init(request: &mut api::TimelineItem) {
                if request.creator.is_none() {
                    request.creator = Some(Default::default());
                }
            }
            
            fn request_location_init(request: &mut api::TimelineItem) {
                if request.location.is_none() {
                    request.location = Some(Default::default());
                }
            }
            
            fn request_notification_init(request: &mut api::TimelineItem) {
                if request.notification.is_none() {
                    request.notification = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "display-time" => {
                        request.display_time = Some(value.unwrap_or("").to_string());
                    },
                "creator.kind" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "creator.display-name" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "creator.accept-types" => {
                        request_creator_init(&mut request);
                        if request.creator.as_mut().unwrap().accept_types.is_none() {
                           request.creator.as_mut().unwrap().accept_types = Some(Default::default());
                        }
                                        request.creator.as_mut().unwrap().accept_types.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "creator.image-urls" => {
                        request_creator_init(&mut request);
                        if request.creator.as_mut().unwrap().image_urls.is_none() {
                           request.creator.as_mut().unwrap().image_urls = Some(Default::default());
                        }
                                        request.creator.as_mut().unwrap().image_urls.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "creator.priority" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().priority = Some(arg_from_str(value.unwrap_or("-0"), err, "creator.priority", "integer"));
                    },
                "creator.source" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().source = Some(value.unwrap_or("").to_string());
                    },
                "creator.phone-number" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().phone_number = Some(value.unwrap_or("").to_string());
                    },
                "creator.sharing-features" => {
                        request_creator_init(&mut request);
                        if request.creator.as_mut().unwrap().sharing_features.is_none() {
                           request.creator.as_mut().unwrap().sharing_features = Some(Default::default());
                        }
                                        request.creator.as_mut().unwrap().sharing_features.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "creator.type" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "creator.id" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "creator.speakable-name" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().speakable_name = Some(value.unwrap_or("").to_string());
                    },
                "text" => {
                        request_creator_init(&mut request);
                        request.text = Some(value.unwrap_or("").to_string());
                    },
                "is-bundle-cover" => {
                        request_creator_init(&mut request);
                        request.is_bundle_cover = Some(arg_from_str(value.unwrap_or("false"), err, "is-bundle-cover", "boolean"));
                    },
                "etag" => {
                        request_creator_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_creator_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "is-deleted" => {
                        request_creator_init(&mut request);
                        request.is_deleted = Some(arg_from_str(value.unwrap_or("false"), err, "is-deleted", "boolean"));
                    },
                "bundle-id" => {
                        request_creator_init(&mut request);
                        request.bundle_id = Some(value.unwrap_or("").to_string());
                    },
                "is-pinned" => {
                        request_creator_init(&mut request);
                        request.is_pinned = Some(arg_from_str(value.unwrap_or("false"), err, "is-pinned", "boolean"));
                    },
                "title" => {
                        request_creator_init(&mut request);
                        request.title = Some(value.unwrap_or("").to_string());
                    },
                "pin-score" => {
                        request_creator_init(&mut request);
                        request.pin_score = Some(arg_from_str(value.unwrap_or("-0"), err, "pin-score", "integer"));
                    },
                "speakable-text" => {
                        request_creator_init(&mut request);
                        request.speakable_text = Some(value.unwrap_or("").to_string());
                    },
                "html" => {
                        request_creator_init(&mut request);
                        request.html = Some(value.unwrap_or("").to_string());
                    },
                "location.kind" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "location.display-name" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "location.timestamp" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().timestamp = Some(value.unwrap_or("").to_string());
                    },
                "location.longitude" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().longitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "location.longitude", "number"));
                    },
                "location.address" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().address = Some(value.unwrap_or("").to_string());
                    },
                "location.latitude" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().latitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "location.latitude", "number"));
                    },
                "location.id" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "location.accuracy" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().accuracy = Some(arg_from_str(value.unwrap_or("0.0"), err, "location.accuracy", "number"));
                    },
                "source-item-id" => {
                        request_location_init(&mut request);
                        request.source_item_id = Some(value.unwrap_or("").to_string());
                    },
                "in-reply-to" => {
                        request_location_init(&mut request);
                        request.in_reply_to = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request_location_init(&mut request);
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "canonical-url" => {
                        request_location_init(&mut request);
                        request.canonical_url = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_location_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request_location_init(&mut request);
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "notification.level" => {
                        request_notification_init(&mut request);
                        request.notification.as_mut().unwrap().level = Some(value.unwrap_or("").to_string());
                    },
                "notification.delivery-time" => {
                        request_notification_init(&mut request);
                        request.notification.as_mut().unwrap().delivery_time = Some(value.unwrap_or("").to_string());
                    },
                "speakable-type" => {
                        request_notification_init(&mut request);
                        request.speakable_type = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_notification_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.timeline().insert(request);
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

    fn _timeline_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.timeline().list();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "source-item-id" => {
                    call = call.source_item_id(value.unwrap_or(""));
                },
                "pinned-only" => {
                    call = call.pinned_only(arg_from_str(value.unwrap_or("false"), err, "pinned-only", "boolean"));
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
                "include-deleted" => {
                    call = call.include_deleted(arg_from_str(value.unwrap_or("false"), err, "include-deleted", "boolean"));
                },
                "bundle-id" => {
                    call = call.bundle_id(value.unwrap_or(""));
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

    fn _timeline_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::TimelineItem::default();
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
            fn request_creator_init(request: &mut api::TimelineItem) {
                if request.creator.is_none() {
                    request.creator = Some(Default::default());
                }
            }
            
            fn request_location_init(request: &mut api::TimelineItem) {
                if request.location.is_none() {
                    request.location = Some(Default::default());
                }
            }
            
            fn request_notification_init(request: &mut api::TimelineItem) {
                if request.notification.is_none() {
                    request.notification = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "display-time" => {
                        request.display_time = Some(value.unwrap_or("").to_string());
                    },
                "creator.kind" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "creator.display-name" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "creator.accept-types" => {
                        request_creator_init(&mut request);
                        if request.creator.as_mut().unwrap().accept_types.is_none() {
                           request.creator.as_mut().unwrap().accept_types = Some(Default::default());
                        }
                                        request.creator.as_mut().unwrap().accept_types.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "creator.image-urls" => {
                        request_creator_init(&mut request);
                        if request.creator.as_mut().unwrap().image_urls.is_none() {
                           request.creator.as_mut().unwrap().image_urls = Some(Default::default());
                        }
                                        request.creator.as_mut().unwrap().image_urls.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "creator.priority" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().priority = Some(arg_from_str(value.unwrap_or("-0"), err, "creator.priority", "integer"));
                    },
                "creator.source" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().source = Some(value.unwrap_or("").to_string());
                    },
                "creator.phone-number" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().phone_number = Some(value.unwrap_or("").to_string());
                    },
                "creator.sharing-features" => {
                        request_creator_init(&mut request);
                        if request.creator.as_mut().unwrap().sharing_features.is_none() {
                           request.creator.as_mut().unwrap().sharing_features = Some(Default::default());
                        }
                                        request.creator.as_mut().unwrap().sharing_features.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "creator.type" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "creator.id" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "creator.speakable-name" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().speakable_name = Some(value.unwrap_or("").to_string());
                    },
                "text" => {
                        request_creator_init(&mut request);
                        request.text = Some(value.unwrap_or("").to_string());
                    },
                "is-bundle-cover" => {
                        request_creator_init(&mut request);
                        request.is_bundle_cover = Some(arg_from_str(value.unwrap_or("false"), err, "is-bundle-cover", "boolean"));
                    },
                "etag" => {
                        request_creator_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_creator_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "is-deleted" => {
                        request_creator_init(&mut request);
                        request.is_deleted = Some(arg_from_str(value.unwrap_or("false"), err, "is-deleted", "boolean"));
                    },
                "bundle-id" => {
                        request_creator_init(&mut request);
                        request.bundle_id = Some(value.unwrap_or("").to_string());
                    },
                "is-pinned" => {
                        request_creator_init(&mut request);
                        request.is_pinned = Some(arg_from_str(value.unwrap_or("false"), err, "is-pinned", "boolean"));
                    },
                "title" => {
                        request_creator_init(&mut request);
                        request.title = Some(value.unwrap_or("").to_string());
                    },
                "pin-score" => {
                        request_creator_init(&mut request);
                        request.pin_score = Some(arg_from_str(value.unwrap_or("-0"), err, "pin-score", "integer"));
                    },
                "speakable-text" => {
                        request_creator_init(&mut request);
                        request.speakable_text = Some(value.unwrap_or("").to_string());
                    },
                "html" => {
                        request_creator_init(&mut request);
                        request.html = Some(value.unwrap_or("").to_string());
                    },
                "location.kind" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "location.display-name" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "location.timestamp" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().timestamp = Some(value.unwrap_or("").to_string());
                    },
                "location.longitude" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().longitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "location.longitude", "number"));
                    },
                "location.address" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().address = Some(value.unwrap_or("").to_string());
                    },
                "location.latitude" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().latitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "location.latitude", "number"));
                    },
                "location.id" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "location.accuracy" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().accuracy = Some(arg_from_str(value.unwrap_or("0.0"), err, "location.accuracy", "number"));
                    },
                "source-item-id" => {
                        request_location_init(&mut request);
                        request.source_item_id = Some(value.unwrap_or("").to_string());
                    },
                "in-reply-to" => {
                        request_location_init(&mut request);
                        request.in_reply_to = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request_location_init(&mut request);
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "canonical-url" => {
                        request_location_init(&mut request);
                        request.canonical_url = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_location_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request_location_init(&mut request);
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "notification.level" => {
                        request_notification_init(&mut request);
                        request.notification.as_mut().unwrap().level = Some(value.unwrap_or("").to_string());
                    },
                "notification.delivery-time" => {
                        request_notification_init(&mut request);
                        request.notification.as_mut().unwrap().delivery_time = Some(value.unwrap_or("").to_string());
                    },
                "speakable-type" => {
                        request_notification_init(&mut request);
                        request.speakable_type = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_notification_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.timeline().patch(request, &self.opt.arg_id);
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

    fn _timeline_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::TimelineItem::default();
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
            fn request_creator_init(request: &mut api::TimelineItem) {
                if request.creator.is_none() {
                    request.creator = Some(Default::default());
                }
            }
            
            fn request_location_init(request: &mut api::TimelineItem) {
                if request.location.is_none() {
                    request.location = Some(Default::default());
                }
            }
            
            fn request_notification_init(request: &mut api::TimelineItem) {
                if request.notification.is_none() {
                    request.notification = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "display-time" => {
                        request.display_time = Some(value.unwrap_or("").to_string());
                    },
                "creator.kind" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "creator.display-name" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "creator.accept-types" => {
                        request_creator_init(&mut request);
                        if request.creator.as_mut().unwrap().accept_types.is_none() {
                           request.creator.as_mut().unwrap().accept_types = Some(Default::default());
                        }
                                        request.creator.as_mut().unwrap().accept_types.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "creator.image-urls" => {
                        request_creator_init(&mut request);
                        if request.creator.as_mut().unwrap().image_urls.is_none() {
                           request.creator.as_mut().unwrap().image_urls = Some(Default::default());
                        }
                                        request.creator.as_mut().unwrap().image_urls.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "creator.priority" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().priority = Some(arg_from_str(value.unwrap_or("-0"), err, "creator.priority", "integer"));
                    },
                "creator.source" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().source = Some(value.unwrap_or("").to_string());
                    },
                "creator.phone-number" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().phone_number = Some(value.unwrap_or("").to_string());
                    },
                "creator.sharing-features" => {
                        request_creator_init(&mut request);
                        if request.creator.as_mut().unwrap().sharing_features.is_none() {
                           request.creator.as_mut().unwrap().sharing_features = Some(Default::default());
                        }
                                        request.creator.as_mut().unwrap().sharing_features.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "creator.type" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "creator.id" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "creator.speakable-name" => {
                        request_creator_init(&mut request);
                        request.creator.as_mut().unwrap().speakable_name = Some(value.unwrap_or("").to_string());
                    },
                "text" => {
                        request_creator_init(&mut request);
                        request.text = Some(value.unwrap_or("").to_string());
                    },
                "is-bundle-cover" => {
                        request_creator_init(&mut request);
                        request.is_bundle_cover = Some(arg_from_str(value.unwrap_or("false"), err, "is-bundle-cover", "boolean"));
                    },
                "etag" => {
                        request_creator_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_creator_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "is-deleted" => {
                        request_creator_init(&mut request);
                        request.is_deleted = Some(arg_from_str(value.unwrap_or("false"), err, "is-deleted", "boolean"));
                    },
                "bundle-id" => {
                        request_creator_init(&mut request);
                        request.bundle_id = Some(value.unwrap_or("").to_string());
                    },
                "is-pinned" => {
                        request_creator_init(&mut request);
                        request.is_pinned = Some(arg_from_str(value.unwrap_or("false"), err, "is-pinned", "boolean"));
                    },
                "title" => {
                        request_creator_init(&mut request);
                        request.title = Some(value.unwrap_or("").to_string());
                    },
                "pin-score" => {
                        request_creator_init(&mut request);
                        request.pin_score = Some(arg_from_str(value.unwrap_or("-0"), err, "pin-score", "integer"));
                    },
                "speakable-text" => {
                        request_creator_init(&mut request);
                        request.speakable_text = Some(value.unwrap_or("").to_string());
                    },
                "html" => {
                        request_creator_init(&mut request);
                        request.html = Some(value.unwrap_or("").to_string());
                    },
                "location.kind" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "location.display-name" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "location.timestamp" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().timestamp = Some(value.unwrap_or("").to_string());
                    },
                "location.longitude" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().longitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "location.longitude", "number"));
                    },
                "location.address" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().address = Some(value.unwrap_or("").to_string());
                    },
                "location.latitude" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().latitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "location.latitude", "number"));
                    },
                "location.id" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "location.accuracy" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().accuracy = Some(arg_from_str(value.unwrap_or("0.0"), err, "location.accuracy", "number"));
                    },
                "source-item-id" => {
                        request_location_init(&mut request);
                        request.source_item_id = Some(value.unwrap_or("").to_string());
                    },
                "in-reply-to" => {
                        request_location_init(&mut request);
                        request.in_reply_to = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request_location_init(&mut request);
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "canonical-url" => {
                        request_location_init(&mut request);
                        request.canonical_url = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_location_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request_location_init(&mut request);
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "notification.level" => {
                        request_notification_init(&mut request);
                        request.notification.as_mut().unwrap().level = Some(value.unwrap_or("").to_string());
                    },
                "notification.delivery-time" => {
                        request_notification_init(&mut request);
                        request.notification.as_mut().unwrap().delivery_time = Some(value.unwrap_or("").to_string());
                    },
                "speakable-type" => {
                        request_notification_init(&mut request);
                        request.speakable_type = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_notification_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.timeline().update(request, &self.opt.arg_id);
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

    fn _doit(&self, dry_run: bool) -> (Option<api::Error>, Option<InvalidOptionsError>) {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Option<api::Error>;
        let mut err_opt: Option<InvalidOptionsError> = None;

        if self.opt.cmd_accounts {
            if self.opt.cmd_insert {
                call_result = self._accounts_insert(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_contacts {
            if self.opt.cmd_delete {
                call_result = self._contacts_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._contacts_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._contacts_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._contacts_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._contacts_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._contacts_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_locations {
            if self.opt.cmd_get {
                call_result = self._locations_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._locations_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_settings {
            if self.opt.cmd_get {
                call_result = self._settings_get(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_subscriptions {
            if self.opt.cmd_delete {
                call_result = self._subscriptions_delete(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._subscriptions_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._subscriptions_list(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._subscriptions_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_timeline {
            if self.opt.cmd_attachments_delete {
                call_result = self._timeline_attachments_delete(dry_run, &mut err);
            } else if self.opt.cmd_attachments_get {
                call_result = self._timeline_attachments_get(dry_run, &mut err);
            } else if self.opt.cmd_attachments_insert {
                call_result = self._timeline_attachments_insert(dry_run, &mut err);
            } else if self.opt.cmd_attachments_list {
                call_result = self._timeline_attachments_list(dry_run, &mut err);
            } else if self.opt.cmd_delete {
                call_result = self._timeline_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._timeline_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._timeline_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._timeline_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._timeline_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._timeline_update(dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "mirror1-secret.json", 
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
                                          program_name: "mirror1",
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
            hub: api::Mirror::new(client, auth),
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