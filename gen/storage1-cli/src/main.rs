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
extern crate google_storage1 as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  storage1 [options] bucket-access-controls delete <bucket> <entity> [-p <v>...]
  storage1 [options] bucket-access-controls get <bucket> <entity> [-p <v>...] [-o <out>]
  storage1 [options] bucket-access-controls insert <bucket> -r <kv>... [-p <v>...] [-o <out>]
  storage1 [options] bucket-access-controls list <bucket> [-p <v>...] [-o <out>]
  storage1 [options] bucket-access-controls patch <bucket> <entity> -r <kv>... [-p <v>...] [-o <out>]
  storage1 [options] bucket-access-controls update <bucket> <entity> -r <kv>... [-p <v>...] [-o <out>]
  storage1 [options] buckets delete <bucket> [-p <v>...]
  storage1 [options] buckets get <bucket> [-p <v>...] [-o <out>]
  storage1 [options] buckets insert <project> -r <kv>... [-p <v>...] [-o <out>]
  storage1 [options] buckets list <project> [-p <v>...] [-o <out>]
  storage1 [options] buckets patch <bucket> -r <kv>... [-p <v>...] [-o <out>]
  storage1 [options] buckets update <bucket> -r <kv>... [-p <v>...] [-o <out>]
  storage1 [options] channels stop -r <kv>... [-p <v>...]
  storage1 [options] default-object-access-controls delete <bucket> <entity> [-p <v>...]
  storage1 [options] default-object-access-controls get <bucket> <entity> [-p <v>...] [-o <out>]
  storage1 [options] default-object-access-controls insert <bucket> -r <kv>... [-p <v>...] [-o <out>]
  storage1 [options] default-object-access-controls list <bucket> [-p <v>...] [-o <out>]
  storage1 [options] default-object-access-controls patch <bucket> <entity> -r <kv>... [-p <v>...] [-o <out>]
  storage1 [options] default-object-access-controls update <bucket> <entity> -r <kv>... [-p <v>...] [-o <out>]
  storage1 [options] object-access-controls delete <bucket> <object> <entity> [-p <v>...]
  storage1 [options] object-access-controls get <bucket> <object> <entity> [-p <v>...] [-o <out>]
  storage1 [options] object-access-controls insert <bucket> <object> -r <kv>... [-p <v>...] [-o <out>]
  storage1 [options] object-access-controls list <bucket> <object> [-p <v>...] [-o <out>]
  storage1 [options] object-access-controls patch <bucket> <object> <entity> -r <kv>... [-p <v>...] [-o <out>]
  storage1 [options] object-access-controls update <bucket> <object> <entity> -r <kv>... [-p <v>...] [-o <out>]
  storage1 [options] objects compose <destination-bucket> <destination-object> -r <kv>... [-p <v>...] [-o <out>]
  storage1 [options] objects copy <source-bucket> <source-object> <destination-bucket> <destination-object> -r <kv>... [-p <v>...] [-o <out>]
  storage1 [options] objects delete <bucket> <object> [-p <v>...]
  storage1 [options] objects get <bucket> <object> [-p <v>...] [-o <out>]
  storage1 [options] objects insert <bucket> -r <kv>... -u (simple|resumable) <file> <mime> [-p <v>...] [-o <out>]
  storage1 [options] objects list <bucket> [-p <v>...] [-o <out>]
  storage1 [options] objects patch <bucket> <object> -r <kv>... [-p <v>...] [-o <out>]
  storage1 [options] objects rewrite <source-bucket> <source-object> <destination-bucket> <destination-object> -r <kv>... [-p <v>...] [-o <out>]
  storage1 [options] objects update <bucket> <object> -r <kv>... [-p <v>...] [-o <out>]
  storage1 [options] objects watch-all <bucket> -r <kv>... [-p <v>...] [-o <out>]
  storage1 --help

All documentation details can be found at
http://byron.github.io/google-apis-rs/google_storage1_cli/index.html

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
    hub: api::Storage<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _bucket_access_controls_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.bucket_access_controls().delete(&self.opt.arg_bucket, &self.opt.arg_entity);
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

    fn _bucket_access_controls_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.bucket_access_controls().get(&self.opt.arg_bucket, &self.opt.arg_entity);
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

    fn _bucket_access_controls_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::BucketAccessControl::default();
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
            fn request_project_team_init(request: &mut api::BucketAccessControl) {
                if request.project_team.is_none() {
                    request.project_team = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "domain" => {
                        request.domain = Some(value.unwrap_or("").to_string());
                    },
                "bucket" => {
                        request.bucket = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "entity" => {
                        request.entity = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "role" => {
                        request.role = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "entity-id" => {
                        request.entity_id = Some(value.unwrap_or("").to_string());
                    },
                "project-team.project-number" => {
                        request_project_team_init(&mut request);
                        request.project_team.as_mut().unwrap().project_number = Some(value.unwrap_or("").to_string());
                    },
                "project-team.team" => {
                        request_project_team_init(&mut request);
                        request.project_team.as_mut().unwrap().team = Some(value.unwrap_or("").to_string());
                    },
                "email" => {
                        request_project_team_init(&mut request);
                        request.email = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_project_team_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.bucket_access_controls().insert(request, &self.opt.arg_bucket);
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

    fn _bucket_access_controls_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.bucket_access_controls().list(&self.opt.arg_bucket);
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

    fn _bucket_access_controls_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::BucketAccessControl::default();
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
            fn request_project_team_init(request: &mut api::BucketAccessControl) {
                if request.project_team.is_none() {
                    request.project_team = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "domain" => {
                        request.domain = Some(value.unwrap_or("").to_string());
                    },
                "bucket" => {
                        request.bucket = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "entity" => {
                        request.entity = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "role" => {
                        request.role = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "entity-id" => {
                        request.entity_id = Some(value.unwrap_or("").to_string());
                    },
                "project-team.project-number" => {
                        request_project_team_init(&mut request);
                        request.project_team.as_mut().unwrap().project_number = Some(value.unwrap_or("").to_string());
                    },
                "project-team.team" => {
                        request_project_team_init(&mut request);
                        request.project_team.as_mut().unwrap().team = Some(value.unwrap_or("").to_string());
                    },
                "email" => {
                        request_project_team_init(&mut request);
                        request.email = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_project_team_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.bucket_access_controls().patch(request, &self.opt.arg_bucket, &self.opt.arg_entity);
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

    fn _bucket_access_controls_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::BucketAccessControl::default();
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
            fn request_project_team_init(request: &mut api::BucketAccessControl) {
                if request.project_team.is_none() {
                    request.project_team = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "domain" => {
                        request.domain = Some(value.unwrap_or("").to_string());
                    },
                "bucket" => {
                        request.bucket = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "entity" => {
                        request.entity = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "role" => {
                        request.role = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "entity-id" => {
                        request.entity_id = Some(value.unwrap_or("").to_string());
                    },
                "project-team.project-number" => {
                        request_project_team_init(&mut request);
                        request.project_team.as_mut().unwrap().project_number = Some(value.unwrap_or("").to_string());
                    },
                "project-team.team" => {
                        request_project_team_init(&mut request);
                        request.project_team.as_mut().unwrap().team = Some(value.unwrap_or("").to_string());
                    },
                "email" => {
                        request_project_team_init(&mut request);
                        request.email = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_project_team_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.bucket_access_controls().update(request, &self.opt.arg_bucket, &self.opt.arg_entity);
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

    fn _buckets_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.buckets().delete(&self.opt.arg_bucket);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "if-metageneration-not-match" => {
                    call = call.if_metageneration_not_match(value.unwrap_or(""));
                },
                "if-metageneration-match" => {
                    call = call.if_metageneration_match(value.unwrap_or(""));
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

    fn _buckets_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.buckets().get(&self.opt.arg_bucket);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "if-metageneration-not-match" => {
                    call = call.if_metageneration_not_match(value.unwrap_or(""));
                },
                "if-metageneration-match" => {
                    call = call.if_metageneration_match(value.unwrap_or(""));
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

    fn _buckets_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Bucket::default();
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
            fn request_logging_init(request: &mut api::Bucket) {
                if request.logging.is_none() {
                    request.logging = Some(Default::default());
                }
            }
            
            fn request_owner_init(request: &mut api::Bucket) {
                if request.owner.is_none() {
                    request.owner = Some(Default::default());
                }
            }
            
            fn request_versioning_init(request: &mut api::Bucket) {
                if request.versioning.is_none() {
                    request.versioning = Some(Default::default());
                }
            }
            
            fn request_website_init(request: &mut api::Bucket) {
                if request.website.is_none() {
                    request.website = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "website.not-found-page" => {
                        request_website_init(&mut request);
                        request.website.as_mut().unwrap().not_found_page = Some(value.unwrap_or("").to_string());
                    },
                "website.main-page-suffix" => {
                        request_website_init(&mut request);
                        request.website.as_mut().unwrap().main_page_suffix = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_website_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_website_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "logging.log-object-prefix" => {
                        request_logging_init(&mut request);
                        request.logging.as_mut().unwrap().log_object_prefix = Some(value.unwrap_or("").to_string());
                    },
                "logging.log-bucket" => {
                        request_logging_init(&mut request);
                        request.logging.as_mut().unwrap().log_bucket = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_logging_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "time-created" => {
                        request_logging_init(&mut request);
                        request.time_created = Some(value.unwrap_or("").to_string());
                    },
                "location" => {
                        request_logging_init(&mut request);
                        request.location = Some(value.unwrap_or("").to_string());
                    },
                "project-number" => {
                        request_logging_init(&mut request);
                        request.project_number = Some(value.unwrap_or("").to_string());
                    },
                "metageneration" => {
                        request_logging_init(&mut request);
                        request.metageneration = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_logging_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "owner.entity-id" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().entity_id = Some(value.unwrap_or("").to_string());
                    },
                "owner.entity" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().entity = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_owner_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "versioning.enabled" => {
                        request_versioning_init(&mut request);
                        request.versioning.as_mut().unwrap().enabled = Some(arg_from_str(value.unwrap_or("false"), err, "versioning.enabled", "boolean"));
                    },
                "storage-class" => {
                        request_versioning_init(&mut request);
                        request.storage_class = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.buckets().insert(request, &self.opt.arg_project);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "predefined-default-object-acl" => {
                    call = call.predefined_default_object_acl(value.unwrap_or(""));
                },
                "predefined-acl" => {
                    call = call.predefined_acl(value.unwrap_or(""));
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

    fn _buckets_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.buckets().list(&self.opt.arg_project);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "prefix" => {
                    call = call.prefix(value.unwrap_or(""));
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

    fn _buckets_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Bucket::default();
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
            fn request_logging_init(request: &mut api::Bucket) {
                if request.logging.is_none() {
                    request.logging = Some(Default::default());
                }
            }
            
            fn request_owner_init(request: &mut api::Bucket) {
                if request.owner.is_none() {
                    request.owner = Some(Default::default());
                }
            }
            
            fn request_versioning_init(request: &mut api::Bucket) {
                if request.versioning.is_none() {
                    request.versioning = Some(Default::default());
                }
            }
            
            fn request_website_init(request: &mut api::Bucket) {
                if request.website.is_none() {
                    request.website = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "website.not-found-page" => {
                        request_website_init(&mut request);
                        request.website.as_mut().unwrap().not_found_page = Some(value.unwrap_or("").to_string());
                    },
                "website.main-page-suffix" => {
                        request_website_init(&mut request);
                        request.website.as_mut().unwrap().main_page_suffix = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_website_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_website_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "logging.log-object-prefix" => {
                        request_logging_init(&mut request);
                        request.logging.as_mut().unwrap().log_object_prefix = Some(value.unwrap_or("").to_string());
                    },
                "logging.log-bucket" => {
                        request_logging_init(&mut request);
                        request.logging.as_mut().unwrap().log_bucket = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_logging_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "time-created" => {
                        request_logging_init(&mut request);
                        request.time_created = Some(value.unwrap_or("").to_string());
                    },
                "location" => {
                        request_logging_init(&mut request);
                        request.location = Some(value.unwrap_or("").to_string());
                    },
                "project-number" => {
                        request_logging_init(&mut request);
                        request.project_number = Some(value.unwrap_or("").to_string());
                    },
                "metageneration" => {
                        request_logging_init(&mut request);
                        request.metageneration = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_logging_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "owner.entity-id" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().entity_id = Some(value.unwrap_or("").to_string());
                    },
                "owner.entity" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().entity = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_owner_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "versioning.enabled" => {
                        request_versioning_init(&mut request);
                        request.versioning.as_mut().unwrap().enabled = Some(arg_from_str(value.unwrap_or("false"), err, "versioning.enabled", "boolean"));
                    },
                "storage-class" => {
                        request_versioning_init(&mut request);
                        request.storage_class = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.buckets().patch(request, &self.opt.arg_bucket);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "predefined-default-object-acl" => {
                    call = call.predefined_default_object_acl(value.unwrap_or(""));
                },
                "predefined-acl" => {
                    call = call.predefined_acl(value.unwrap_or(""));
                },
                "if-metageneration-not-match" => {
                    call = call.if_metageneration_not_match(value.unwrap_or(""));
                },
                "if-metageneration-match" => {
                    call = call.if_metageneration_match(value.unwrap_or(""));
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

    fn _buckets_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Bucket::default();
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
            fn request_logging_init(request: &mut api::Bucket) {
                if request.logging.is_none() {
                    request.logging = Some(Default::default());
                }
            }
            
            fn request_owner_init(request: &mut api::Bucket) {
                if request.owner.is_none() {
                    request.owner = Some(Default::default());
                }
            }
            
            fn request_versioning_init(request: &mut api::Bucket) {
                if request.versioning.is_none() {
                    request.versioning = Some(Default::default());
                }
            }
            
            fn request_website_init(request: &mut api::Bucket) {
                if request.website.is_none() {
                    request.website = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "website.not-found-page" => {
                        request_website_init(&mut request);
                        request.website.as_mut().unwrap().not_found_page = Some(value.unwrap_or("").to_string());
                    },
                "website.main-page-suffix" => {
                        request_website_init(&mut request);
                        request.website.as_mut().unwrap().main_page_suffix = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_website_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_website_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "logging.log-object-prefix" => {
                        request_logging_init(&mut request);
                        request.logging.as_mut().unwrap().log_object_prefix = Some(value.unwrap_or("").to_string());
                    },
                "logging.log-bucket" => {
                        request_logging_init(&mut request);
                        request.logging.as_mut().unwrap().log_bucket = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_logging_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "time-created" => {
                        request_logging_init(&mut request);
                        request.time_created = Some(value.unwrap_or("").to_string());
                    },
                "location" => {
                        request_logging_init(&mut request);
                        request.location = Some(value.unwrap_or("").to_string());
                    },
                "project-number" => {
                        request_logging_init(&mut request);
                        request.project_number = Some(value.unwrap_or("").to_string());
                    },
                "metageneration" => {
                        request_logging_init(&mut request);
                        request.metageneration = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_logging_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "owner.entity-id" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().entity_id = Some(value.unwrap_or("").to_string());
                    },
                "owner.entity" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().entity = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_owner_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "versioning.enabled" => {
                        request_versioning_init(&mut request);
                        request.versioning.as_mut().unwrap().enabled = Some(arg_from_str(value.unwrap_or("false"), err, "versioning.enabled", "boolean"));
                    },
                "storage-class" => {
                        request_versioning_init(&mut request);
                        request.storage_class = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.buckets().update(request, &self.opt.arg_bucket);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "predefined-default-object-acl" => {
                    call = call.predefined_default_object_acl(value.unwrap_or(""));
                },
                "predefined-acl" => {
                    call = call.predefined_acl(value.unwrap_or(""));
                },
                "if-metageneration-not-match" => {
                    call = call.if_metageneration_not_match(value.unwrap_or(""));
                },
                "if-metageneration-match" => {
                    call = call.if_metageneration_match(value.unwrap_or(""));
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

    fn _default_object_access_controls_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.default_object_access_controls().delete(&self.opt.arg_bucket, &self.opt.arg_entity);
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

    fn _default_object_access_controls_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.default_object_access_controls().get(&self.opt.arg_bucket, &self.opt.arg_entity);
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

    fn _default_object_access_controls_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::ObjectAccessControl::default();
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
            fn request_project_team_init(request: &mut api::ObjectAccessControl) {
                if request.project_team.is_none() {
                    request.project_team = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "domain" => {
                        request.domain = Some(value.unwrap_or("").to_string());
                    },
                "generation" => {
                        request.generation = Some(value.unwrap_or("").to_string());
                    },
                "object" => {
                        request.object = Some(value.unwrap_or("").to_string());
                    },
                "bucket" => {
                        request.bucket = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "entity" => {
                        request.entity = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "role" => {
                        request.role = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "entity-id" => {
                        request.entity_id = Some(value.unwrap_or("").to_string());
                    },
                "project-team.project-number" => {
                        request_project_team_init(&mut request);
                        request.project_team.as_mut().unwrap().project_number = Some(value.unwrap_or("").to_string());
                    },
                "project-team.team" => {
                        request_project_team_init(&mut request);
                        request.project_team.as_mut().unwrap().team = Some(value.unwrap_or("").to_string());
                    },
                "email" => {
                        request_project_team_init(&mut request);
                        request.email = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_project_team_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.default_object_access_controls().insert(request, &self.opt.arg_bucket);
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

    fn _default_object_access_controls_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.default_object_access_controls().list(&self.opt.arg_bucket);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "if-metageneration-not-match" => {
                    call = call.if_metageneration_not_match(value.unwrap_or(""));
                },
                "if-metageneration-match" => {
                    call = call.if_metageneration_match(value.unwrap_or(""));
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

    fn _default_object_access_controls_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::ObjectAccessControl::default();
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
            fn request_project_team_init(request: &mut api::ObjectAccessControl) {
                if request.project_team.is_none() {
                    request.project_team = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "domain" => {
                        request.domain = Some(value.unwrap_or("").to_string());
                    },
                "generation" => {
                        request.generation = Some(value.unwrap_or("").to_string());
                    },
                "object" => {
                        request.object = Some(value.unwrap_or("").to_string());
                    },
                "bucket" => {
                        request.bucket = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "entity" => {
                        request.entity = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "role" => {
                        request.role = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "entity-id" => {
                        request.entity_id = Some(value.unwrap_or("").to_string());
                    },
                "project-team.project-number" => {
                        request_project_team_init(&mut request);
                        request.project_team.as_mut().unwrap().project_number = Some(value.unwrap_or("").to_string());
                    },
                "project-team.team" => {
                        request_project_team_init(&mut request);
                        request.project_team.as_mut().unwrap().team = Some(value.unwrap_or("").to_string());
                    },
                "email" => {
                        request_project_team_init(&mut request);
                        request.email = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_project_team_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.default_object_access_controls().patch(request, &self.opt.arg_bucket, &self.opt.arg_entity);
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

    fn _default_object_access_controls_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::ObjectAccessControl::default();
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
            fn request_project_team_init(request: &mut api::ObjectAccessControl) {
                if request.project_team.is_none() {
                    request.project_team = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "domain" => {
                        request.domain = Some(value.unwrap_or("").to_string());
                    },
                "generation" => {
                        request.generation = Some(value.unwrap_or("").to_string());
                    },
                "object" => {
                        request.object = Some(value.unwrap_or("").to_string());
                    },
                "bucket" => {
                        request.bucket = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "entity" => {
                        request.entity = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "role" => {
                        request.role = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "entity-id" => {
                        request.entity_id = Some(value.unwrap_or("").to_string());
                    },
                "project-team.project-number" => {
                        request_project_team_init(&mut request);
                        request.project_team.as_mut().unwrap().project_number = Some(value.unwrap_or("").to_string());
                    },
                "project-team.team" => {
                        request_project_team_init(&mut request);
                        request.project_team.as_mut().unwrap().team = Some(value.unwrap_or("").to_string());
                    },
                "email" => {
                        request_project_team_init(&mut request);
                        request.email = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_project_team_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.default_object_access_controls().update(request, &self.opt.arg_bucket, &self.opt.arg_entity);
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

    fn _object_access_controls_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.object_access_controls().delete(&self.opt.arg_bucket, &self.opt.arg_object, &self.opt.arg_entity);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "generation" => {
                    call = call.generation(value.unwrap_or(""));
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

    fn _object_access_controls_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.object_access_controls().get(&self.opt.arg_bucket, &self.opt.arg_object, &self.opt.arg_entity);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "generation" => {
                    call = call.generation(value.unwrap_or(""));
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

    fn _object_access_controls_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::ObjectAccessControl::default();
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
            fn request_project_team_init(request: &mut api::ObjectAccessControl) {
                if request.project_team.is_none() {
                    request.project_team = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "domain" => {
                        request.domain = Some(value.unwrap_or("").to_string());
                    },
                "generation" => {
                        request.generation = Some(value.unwrap_or("").to_string());
                    },
                "object" => {
                        request.object = Some(value.unwrap_or("").to_string());
                    },
                "bucket" => {
                        request.bucket = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "entity" => {
                        request.entity = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "role" => {
                        request.role = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "entity-id" => {
                        request.entity_id = Some(value.unwrap_or("").to_string());
                    },
                "project-team.project-number" => {
                        request_project_team_init(&mut request);
                        request.project_team.as_mut().unwrap().project_number = Some(value.unwrap_or("").to_string());
                    },
                "project-team.team" => {
                        request_project_team_init(&mut request);
                        request.project_team.as_mut().unwrap().team = Some(value.unwrap_or("").to_string());
                    },
                "email" => {
                        request_project_team_init(&mut request);
                        request.email = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_project_team_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.object_access_controls().insert(request, &self.opt.arg_bucket, &self.opt.arg_object);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "generation" => {
                    call = call.generation(value.unwrap_or(""));
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

    fn _object_access_controls_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.object_access_controls().list(&self.opt.arg_bucket, &self.opt.arg_object);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "generation" => {
                    call = call.generation(value.unwrap_or(""));
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

    fn _object_access_controls_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::ObjectAccessControl::default();
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
            fn request_project_team_init(request: &mut api::ObjectAccessControl) {
                if request.project_team.is_none() {
                    request.project_team = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "domain" => {
                        request.domain = Some(value.unwrap_or("").to_string());
                    },
                "generation" => {
                        request.generation = Some(value.unwrap_or("").to_string());
                    },
                "object" => {
                        request.object = Some(value.unwrap_or("").to_string());
                    },
                "bucket" => {
                        request.bucket = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "entity" => {
                        request.entity = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "role" => {
                        request.role = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "entity-id" => {
                        request.entity_id = Some(value.unwrap_or("").to_string());
                    },
                "project-team.project-number" => {
                        request_project_team_init(&mut request);
                        request.project_team.as_mut().unwrap().project_number = Some(value.unwrap_or("").to_string());
                    },
                "project-team.team" => {
                        request_project_team_init(&mut request);
                        request.project_team.as_mut().unwrap().team = Some(value.unwrap_or("").to_string());
                    },
                "email" => {
                        request_project_team_init(&mut request);
                        request.email = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_project_team_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.object_access_controls().patch(request, &self.opt.arg_bucket, &self.opt.arg_object, &self.opt.arg_entity);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "generation" => {
                    call = call.generation(value.unwrap_or(""));
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

    fn _object_access_controls_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::ObjectAccessControl::default();
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
            fn request_project_team_init(request: &mut api::ObjectAccessControl) {
                if request.project_team.is_none() {
                    request.project_team = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "domain" => {
                        request.domain = Some(value.unwrap_or("").to_string());
                    },
                "generation" => {
                        request.generation = Some(value.unwrap_or("").to_string());
                    },
                "object" => {
                        request.object = Some(value.unwrap_or("").to_string());
                    },
                "bucket" => {
                        request.bucket = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "entity" => {
                        request.entity = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "role" => {
                        request.role = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "entity-id" => {
                        request.entity_id = Some(value.unwrap_or("").to_string());
                    },
                "project-team.project-number" => {
                        request_project_team_init(&mut request);
                        request.project_team.as_mut().unwrap().project_number = Some(value.unwrap_or("").to_string());
                    },
                "project-team.team" => {
                        request_project_team_init(&mut request);
                        request.project_team.as_mut().unwrap().team = Some(value.unwrap_or("").to_string());
                    },
                "email" => {
                        request_project_team_init(&mut request);
                        request.email = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_project_team_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.object_access_controls().update(request, &self.opt.arg_bucket, &self.opt.arg_object, &self.opt.arg_entity);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "generation" => {
                    call = call.generation(value.unwrap_or(""));
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

    fn _objects_compose(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::ComposeRequest::default();
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
            fn request_destination_init(request: &mut api::ComposeRequest) {
                if request.destination.is_none() {
                    request.destination = Some(Default::default());
                }
            }
            
            fn request_destination_owner_init(request: &mut api::ComposeRequest) {
                request_destination_init(request);
                if request.destination.as_mut().unwrap().owner.is_none() {
                    request.destination.as_mut().unwrap().owner = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "destination.self-link" => {
                        request_destination_init(&mut request);
                        request.destination.as_mut().unwrap().self_link = Some(value.unwrap_or("").to_string());
                    },
                "destination.generation" => {
                        request_destination_init(&mut request);
                        request.destination.as_mut().unwrap().generation = Some(value.unwrap_or("").to_string());
                    },
                "destination.component-count" => {
                        request_destination_init(&mut request);
                        request.destination.as_mut().unwrap().component_count = Some(arg_from_str(value.unwrap_or("-0"), err, "destination.component-count", "integer"));
                    },
                "destination.media-link" => {
                        request_destination_init(&mut request);
                        request.destination.as_mut().unwrap().media_link = Some(value.unwrap_or("").to_string());
                    },
                "destination.owner.entity-id" => {
                        request_destination_owner_init(&mut request);
                        request.destination.as_mut().unwrap().owner.as_mut().unwrap().entity_id = Some(value.unwrap_or("").to_string());
                    },
                "destination.owner.entity" => {
                        request_destination_owner_init(&mut request);
                        request.destination.as_mut().unwrap().owner.as_mut().unwrap().entity = Some(value.unwrap_or("").to_string());
                    },
                "destination.cache-control" => {
                        request_destination_owner_init(&mut request);
                        request.destination.as_mut().unwrap().cache_control = Some(value.unwrap_or("").to_string());
                    },
                "destination.id" => {
                        request_destination_owner_init(&mut request);
                        request.destination.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "destination.size" => {
                        request_destination_owner_init(&mut request);
                        request.destination.as_mut().unwrap().size = Some(value.unwrap_or("").to_string());
                    },
                "destination.time-deleted" => {
                        request_destination_owner_init(&mut request);
                        request.destination.as_mut().unwrap().time_deleted = Some(value.unwrap_or("").to_string());
                    },
                "destination.md5-hash" => {
                        request_destination_owner_init(&mut request);
                        request.destination.as_mut().unwrap().md5_hash = Some(value.unwrap_or("").to_string());
                    },
                "destination.crc32c" => {
                        request_destination_owner_init(&mut request);
                        request.destination.as_mut().unwrap().crc32c = Some(value.unwrap_or("").to_string());
                    },
                "destination.etag" => {
                        request_destination_owner_init(&mut request);
                        request.destination.as_mut().unwrap().etag = Some(value.unwrap_or("").to_string());
                    },
                "destination.metadata" => {
                        request_destination_owner_init(&mut request);
                        if request.destination.as_mut().unwrap().metadata.is_none() {
                           request.destination.as_mut().unwrap().metadata = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.destination.as_mut().unwrap().metadata.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "destination.updated" => {
                        request_destination_owner_init(&mut request);
                        request.destination.as_mut().unwrap().updated = Some(value.unwrap_or("").to_string());
                    },
                "destination.content-type" => {
                        request_destination_owner_init(&mut request);
                        request.destination.as_mut().unwrap().content_type = Some(value.unwrap_or("").to_string());
                    },
                "destination.content-language" => {
                        request_destination_owner_init(&mut request);
                        request.destination.as_mut().unwrap().content_language = Some(value.unwrap_or("").to_string());
                    },
                "destination.metageneration" => {
                        request_destination_owner_init(&mut request);
                        request.destination.as_mut().unwrap().metageneration = Some(value.unwrap_or("").to_string());
                    },
                "destination.kind" => {
                        request_destination_owner_init(&mut request);
                        request.destination.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "destination.name" => {
                        request_destination_owner_init(&mut request);
                        request.destination.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "destination.bucket" => {
                        request_destination_owner_init(&mut request);
                        request.destination.as_mut().unwrap().bucket = Some(value.unwrap_or("").to_string());
                    },
                "destination.content-encoding" => {
                        request_destination_owner_init(&mut request);
                        request.destination.as_mut().unwrap().content_encoding = Some(value.unwrap_or("").to_string());
                    },
                "destination.storage-class" => {
                        request_destination_owner_init(&mut request);
                        request.destination.as_mut().unwrap().storage_class = Some(value.unwrap_or("").to_string());
                    },
                "destination.content-disposition" => {
                        request_destination_owner_init(&mut request);
                        request.destination.as_mut().unwrap().content_disposition = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut download_mode = false;
        let mut call = self.hub.objects().compose(request, &self.opt.arg_destination_bucket, &self.opt.arg_destination_object);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "if-metageneration-match" => {
                    call = call.if_metageneration_match(value.unwrap_or(""));
                },
                "if-generation-match" => {
                    call = call.if_generation_match(value.unwrap_or(""));
                },
                "destination-predefined-acl" => {
                    call = call.destination_predefined_acl(value.unwrap_or(""));
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

    fn _objects_copy(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Object::default();
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
            fn request_owner_init(request: &mut api::Object) {
                if request.owner.is_none() {
                    request.owner = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "self-link" => {
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "generation" => {
                        request.generation = Some(value.unwrap_or("").to_string());
                    },
                "component-count" => {
                        request.component_count = Some(arg_from_str(value.unwrap_or("-0"), err, "component-count", "integer"));
                    },
                "media-link" => {
                        request.media_link = Some(value.unwrap_or("").to_string());
                    },
                "owner.entity-id" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().entity_id = Some(value.unwrap_or("").to_string());
                    },
                "owner.entity" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().entity = Some(value.unwrap_or("").to_string());
                    },
                "cache-control" => {
                        request_owner_init(&mut request);
                        request.cache_control = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_owner_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "size" => {
                        request_owner_init(&mut request);
                        request.size = Some(value.unwrap_or("").to_string());
                    },
                "time-deleted" => {
                        request_owner_init(&mut request);
                        request.time_deleted = Some(value.unwrap_or("").to_string());
                    },
                "md5-hash" => {
                        request_owner_init(&mut request);
                        request.md5_hash = Some(value.unwrap_or("").to_string());
                    },
                "crc32c" => {
                        request_owner_init(&mut request);
                        request.crc32c = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_owner_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "metadata" => {
                        request_owner_init(&mut request);
                        if request.metadata.is_none() {
                           request.metadata = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.metadata.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request_owner_init(&mut request);
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "content-type" => {
                        request_owner_init(&mut request);
                        request.content_type = Some(value.unwrap_or("").to_string());
                    },
                "content-language" => {
                        request_owner_init(&mut request);
                        request.content_language = Some(value.unwrap_or("").to_string());
                    },
                "metageneration" => {
                        request_owner_init(&mut request);
                        request.metageneration = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_owner_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_owner_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "bucket" => {
                        request_owner_init(&mut request);
                        request.bucket = Some(value.unwrap_or("").to_string());
                    },
                "content-encoding" => {
                        request_owner_init(&mut request);
                        request.content_encoding = Some(value.unwrap_or("").to_string());
                    },
                "storage-class" => {
                        request_owner_init(&mut request);
                        request.storage_class = Some(value.unwrap_or("").to_string());
                    },
                "content-disposition" => {
                        request_owner_init(&mut request);
                        request.content_disposition = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut download_mode = false;
        let mut call = self.hub.objects().copy(request, &self.opt.arg_source_bucket, &self.opt.arg_source_object, &self.opt.arg_destination_bucket, &self.opt.arg_destination_object);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "source-generation" => {
                    call = call.source_generation(value.unwrap_or(""));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "if-source-metageneration-not-match" => {
                    call = call.if_source_metageneration_not_match(value.unwrap_or(""));
                },
                "if-source-metageneration-match" => {
                    call = call.if_source_metageneration_match(value.unwrap_or(""));
                },
                "if-source-generation-not-match" => {
                    call = call.if_source_generation_not_match(value.unwrap_or(""));
                },
                "if-source-generation-match" => {
                    call = call.if_source_generation_match(value.unwrap_or(""));
                },
                "if-metageneration-not-match" => {
                    call = call.if_metageneration_not_match(value.unwrap_or(""));
                },
                "if-metageneration-match" => {
                    call = call.if_metageneration_match(value.unwrap_or(""));
                },
                "if-generation-not-match" => {
                    call = call.if_generation_not_match(value.unwrap_or(""));
                },
                "if-generation-match" => {
                    call = call.if_generation_match(value.unwrap_or(""));
                },
                "destination-predefined-acl" => {
                    call = call.destination_predefined_acl(value.unwrap_or(""));
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

    fn _objects_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.objects().delete(&self.opt.arg_bucket, &self.opt.arg_object);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "if-metageneration-not-match" => {
                    call = call.if_metageneration_not_match(value.unwrap_or(""));
                },
                "if-metageneration-match" => {
                    call = call.if_metageneration_match(value.unwrap_or(""));
                },
                "if-generation-not-match" => {
                    call = call.if_generation_not_match(value.unwrap_or(""));
                },
                "if-generation-match" => {
                    call = call.if_generation_match(value.unwrap_or(""));
                },
                "generation" => {
                    call = call.generation(value.unwrap_or(""));
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

    fn _objects_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut download_mode = false;
        let mut call = self.hub.objects().get(&self.opt.arg_bucket, &self.opt.arg_object);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "if-metageneration-not-match" => {
                    call = call.if_metageneration_not_match(value.unwrap_or(""));
                },
                "if-metageneration-match" => {
                    call = call.if_metageneration_match(value.unwrap_or(""));
                },
                "if-generation-not-match" => {
                    call = call.if_generation_not_match(value.unwrap_or(""));
                },
                "if-generation-match" => {
                    call = call.if_generation_match(value.unwrap_or(""));
                },
                "generation" => {
                    call = call.generation(value.unwrap_or(""));
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

    fn _objects_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Object::default();
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
            fn request_owner_init(request: &mut api::Object) {
                if request.owner.is_none() {
                    request.owner = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "self-link" => {
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "generation" => {
                        request.generation = Some(value.unwrap_or("").to_string());
                    },
                "component-count" => {
                        request.component_count = Some(arg_from_str(value.unwrap_or("-0"), err, "component-count", "integer"));
                    },
                "media-link" => {
                        request.media_link = Some(value.unwrap_or("").to_string());
                    },
                "owner.entity-id" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().entity_id = Some(value.unwrap_or("").to_string());
                    },
                "owner.entity" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().entity = Some(value.unwrap_or("").to_string());
                    },
                "cache-control" => {
                        request_owner_init(&mut request);
                        request.cache_control = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_owner_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "size" => {
                        request_owner_init(&mut request);
                        request.size = Some(value.unwrap_or("").to_string());
                    },
                "time-deleted" => {
                        request_owner_init(&mut request);
                        request.time_deleted = Some(value.unwrap_or("").to_string());
                    },
                "md5-hash" => {
                        request_owner_init(&mut request);
                        request.md5_hash = Some(value.unwrap_or("").to_string());
                    },
                "crc32c" => {
                        request_owner_init(&mut request);
                        request.crc32c = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_owner_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "metadata" => {
                        request_owner_init(&mut request);
                        if request.metadata.is_none() {
                           request.metadata = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.metadata.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request_owner_init(&mut request);
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "content-type" => {
                        request_owner_init(&mut request);
                        request.content_type = Some(value.unwrap_or("").to_string());
                    },
                "content-language" => {
                        request_owner_init(&mut request);
                        request.content_language = Some(value.unwrap_or("").to_string());
                    },
                "metageneration" => {
                        request_owner_init(&mut request);
                        request.metageneration = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_owner_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_owner_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "bucket" => {
                        request_owner_init(&mut request);
                        request.bucket = Some(value.unwrap_or("").to_string());
                    },
                "content-encoding" => {
                        request_owner_init(&mut request);
                        request.content_encoding = Some(value.unwrap_or("").to_string());
                    },
                "storage-class" => {
                        request_owner_init(&mut request);
                        request.storage_class = Some(value.unwrap_or("").to_string());
                    },
                "content-disposition" => {
                        request_owner_init(&mut request);
                        request.content_disposition = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.objects().insert(request, &self.opt.arg_bucket);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "predefined-acl" => {
                    call = call.predefined_acl(value.unwrap_or(""));
                },
                "name" => {
                    call = call.name(value.unwrap_or(""));
                },
                "if-metageneration-not-match" => {
                    call = call.if_metageneration_not_match(value.unwrap_or(""));
                },
                "if-metageneration-match" => {
                    call = call.if_metageneration_match(value.unwrap_or(""));
                },
                "if-generation-not-match" => {
                    call = call.if_generation_not_match(value.unwrap_or(""));
                },
                "if-generation-match" => {
                    call = call.if_generation_match(value.unwrap_or(""));
                },
                "content-encoding" => {
                    call = call.content_encoding(value.unwrap_or(""));
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
                    io::copy(&mut response, &mut ostream).unwrap();
                    None
                }
            }
        }
    }

    fn _objects_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.objects().list(&self.opt.arg_bucket);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "versions" => {
                    call = call.versions(arg_from_str(value.unwrap_or("false"), err, "versions", "boolean"));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "prefix" => {
                    call = call.prefix(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "delimiter" => {
                    call = call.delimiter(value.unwrap_or(""));
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

    fn _objects_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Object::default();
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
            fn request_owner_init(request: &mut api::Object) {
                if request.owner.is_none() {
                    request.owner = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "self-link" => {
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "generation" => {
                        request.generation = Some(value.unwrap_or("").to_string());
                    },
                "component-count" => {
                        request.component_count = Some(arg_from_str(value.unwrap_or("-0"), err, "component-count", "integer"));
                    },
                "media-link" => {
                        request.media_link = Some(value.unwrap_or("").to_string());
                    },
                "owner.entity-id" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().entity_id = Some(value.unwrap_or("").to_string());
                    },
                "owner.entity" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().entity = Some(value.unwrap_or("").to_string());
                    },
                "cache-control" => {
                        request_owner_init(&mut request);
                        request.cache_control = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_owner_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "size" => {
                        request_owner_init(&mut request);
                        request.size = Some(value.unwrap_or("").to_string());
                    },
                "time-deleted" => {
                        request_owner_init(&mut request);
                        request.time_deleted = Some(value.unwrap_or("").to_string());
                    },
                "md5-hash" => {
                        request_owner_init(&mut request);
                        request.md5_hash = Some(value.unwrap_or("").to_string());
                    },
                "crc32c" => {
                        request_owner_init(&mut request);
                        request.crc32c = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_owner_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "metadata" => {
                        request_owner_init(&mut request);
                        if request.metadata.is_none() {
                           request.metadata = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.metadata.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request_owner_init(&mut request);
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "content-type" => {
                        request_owner_init(&mut request);
                        request.content_type = Some(value.unwrap_or("").to_string());
                    },
                "content-language" => {
                        request_owner_init(&mut request);
                        request.content_language = Some(value.unwrap_or("").to_string());
                    },
                "metageneration" => {
                        request_owner_init(&mut request);
                        request.metageneration = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_owner_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_owner_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "bucket" => {
                        request_owner_init(&mut request);
                        request.bucket = Some(value.unwrap_or("").to_string());
                    },
                "content-encoding" => {
                        request_owner_init(&mut request);
                        request.content_encoding = Some(value.unwrap_or("").to_string());
                    },
                "storage-class" => {
                        request_owner_init(&mut request);
                        request.storage_class = Some(value.unwrap_or("").to_string());
                    },
                "content-disposition" => {
                        request_owner_init(&mut request);
                        request.content_disposition = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.objects().patch(request, &self.opt.arg_bucket, &self.opt.arg_object);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "predefined-acl" => {
                    call = call.predefined_acl(value.unwrap_or(""));
                },
                "if-metageneration-not-match" => {
                    call = call.if_metageneration_not_match(value.unwrap_or(""));
                },
                "if-metageneration-match" => {
                    call = call.if_metageneration_match(value.unwrap_or(""));
                },
                "if-generation-not-match" => {
                    call = call.if_generation_not_match(value.unwrap_or(""));
                },
                "if-generation-match" => {
                    call = call.if_generation_match(value.unwrap_or(""));
                },
                "generation" => {
                    call = call.generation(value.unwrap_or(""));
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

    fn _objects_rewrite(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Object::default();
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
            fn request_owner_init(request: &mut api::Object) {
                if request.owner.is_none() {
                    request.owner = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "self-link" => {
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "generation" => {
                        request.generation = Some(value.unwrap_or("").to_string());
                    },
                "component-count" => {
                        request.component_count = Some(arg_from_str(value.unwrap_or("-0"), err, "component-count", "integer"));
                    },
                "media-link" => {
                        request.media_link = Some(value.unwrap_or("").to_string());
                    },
                "owner.entity-id" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().entity_id = Some(value.unwrap_or("").to_string());
                    },
                "owner.entity" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().entity = Some(value.unwrap_or("").to_string());
                    },
                "cache-control" => {
                        request_owner_init(&mut request);
                        request.cache_control = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_owner_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "size" => {
                        request_owner_init(&mut request);
                        request.size = Some(value.unwrap_or("").to_string());
                    },
                "time-deleted" => {
                        request_owner_init(&mut request);
                        request.time_deleted = Some(value.unwrap_or("").to_string());
                    },
                "md5-hash" => {
                        request_owner_init(&mut request);
                        request.md5_hash = Some(value.unwrap_or("").to_string());
                    },
                "crc32c" => {
                        request_owner_init(&mut request);
                        request.crc32c = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_owner_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "metadata" => {
                        request_owner_init(&mut request);
                        if request.metadata.is_none() {
                           request.metadata = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.metadata.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request_owner_init(&mut request);
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "content-type" => {
                        request_owner_init(&mut request);
                        request.content_type = Some(value.unwrap_or("").to_string());
                    },
                "content-language" => {
                        request_owner_init(&mut request);
                        request.content_language = Some(value.unwrap_or("").to_string());
                    },
                "metageneration" => {
                        request_owner_init(&mut request);
                        request.metageneration = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_owner_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_owner_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "bucket" => {
                        request_owner_init(&mut request);
                        request.bucket = Some(value.unwrap_or("").to_string());
                    },
                "content-encoding" => {
                        request_owner_init(&mut request);
                        request.content_encoding = Some(value.unwrap_or("").to_string());
                    },
                "storage-class" => {
                        request_owner_init(&mut request);
                        request.storage_class = Some(value.unwrap_or("").to_string());
                    },
                "content-disposition" => {
                        request_owner_init(&mut request);
                        request.content_disposition = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.objects().rewrite(request, &self.opt.arg_source_bucket, &self.opt.arg_source_object, &self.opt.arg_destination_bucket, &self.opt.arg_destination_object);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "source-generation" => {
                    call = call.source_generation(value.unwrap_or(""));
                },
                "rewrite-token" => {
                    call = call.rewrite_token(value.unwrap_or(""));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "max-bytes-rewritten-per-call" => {
                    call = call.max_bytes_rewritten_per_call(value.unwrap_or(""));
                },
                "if-source-metageneration-not-match" => {
                    call = call.if_source_metageneration_not_match(value.unwrap_or(""));
                },
                "if-source-metageneration-match" => {
                    call = call.if_source_metageneration_match(value.unwrap_or(""));
                },
                "if-source-generation-not-match" => {
                    call = call.if_source_generation_not_match(value.unwrap_or(""));
                },
                "if-source-generation-match" => {
                    call = call.if_source_generation_match(value.unwrap_or(""));
                },
                "if-metageneration-not-match" => {
                    call = call.if_metageneration_not_match(value.unwrap_or(""));
                },
                "if-metageneration-match" => {
                    call = call.if_metageneration_match(value.unwrap_or(""));
                },
                "if-generation-not-match" => {
                    call = call.if_generation_not_match(value.unwrap_or(""));
                },
                "if-generation-match" => {
                    call = call.if_generation_match(value.unwrap_or(""));
                },
                "destination-predefined-acl" => {
                    call = call.destination_predefined_acl(value.unwrap_or(""));
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

    fn _objects_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Object::default();
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
            fn request_owner_init(request: &mut api::Object) {
                if request.owner.is_none() {
                    request.owner = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "self-link" => {
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "generation" => {
                        request.generation = Some(value.unwrap_or("").to_string());
                    },
                "component-count" => {
                        request.component_count = Some(arg_from_str(value.unwrap_or("-0"), err, "component-count", "integer"));
                    },
                "media-link" => {
                        request.media_link = Some(value.unwrap_or("").to_string());
                    },
                "owner.entity-id" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().entity_id = Some(value.unwrap_or("").to_string());
                    },
                "owner.entity" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().entity = Some(value.unwrap_or("").to_string());
                    },
                "cache-control" => {
                        request_owner_init(&mut request);
                        request.cache_control = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_owner_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "size" => {
                        request_owner_init(&mut request);
                        request.size = Some(value.unwrap_or("").to_string());
                    },
                "time-deleted" => {
                        request_owner_init(&mut request);
                        request.time_deleted = Some(value.unwrap_or("").to_string());
                    },
                "md5-hash" => {
                        request_owner_init(&mut request);
                        request.md5_hash = Some(value.unwrap_or("").to_string());
                    },
                "crc32c" => {
                        request_owner_init(&mut request);
                        request.crc32c = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_owner_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "metadata" => {
                        request_owner_init(&mut request);
                        if request.metadata.is_none() {
                           request.metadata = Some(Default::default());
                        }
                        let (key, value) = parse_kv_arg(value.unwrap_or(""), err, true);
                        request.metadata.as_mut().unwrap().insert(key.to_string(), value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request_owner_init(&mut request);
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "content-type" => {
                        request_owner_init(&mut request);
                        request.content_type = Some(value.unwrap_or("").to_string());
                    },
                "content-language" => {
                        request_owner_init(&mut request);
                        request.content_language = Some(value.unwrap_or("").to_string());
                    },
                "metageneration" => {
                        request_owner_init(&mut request);
                        request.metageneration = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_owner_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_owner_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "bucket" => {
                        request_owner_init(&mut request);
                        request.bucket = Some(value.unwrap_or("").to_string());
                    },
                "content-encoding" => {
                        request_owner_init(&mut request);
                        request.content_encoding = Some(value.unwrap_or("").to_string());
                    },
                "storage-class" => {
                        request_owner_init(&mut request);
                        request.storage_class = Some(value.unwrap_or("").to_string());
                    },
                "content-disposition" => {
                        request_owner_init(&mut request);
                        request.content_disposition = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut download_mode = false;
        let mut call = self.hub.objects().update(request, &self.opt.arg_bucket, &self.opt.arg_object);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "predefined-acl" => {
                    call = call.predefined_acl(value.unwrap_or(""));
                },
                "if-metageneration-not-match" => {
                    call = call.if_metageneration_not_match(value.unwrap_or(""));
                },
                "if-metageneration-match" => {
                    call = call.if_metageneration_match(value.unwrap_or(""));
                },
                "if-generation-not-match" => {
                    call = call.if_generation_not_match(value.unwrap_or(""));
                },
                "if-generation-match" => {
                    call = call.if_generation_match(value.unwrap_or(""));
                },
                "generation" => {
                    call = call.generation(value.unwrap_or(""));
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

    fn _objects_watch_all(&self, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut call = self.hub.objects().watch_all(request, &self.opt.arg_bucket);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "versions" => {
                    call = call.versions(arg_from_str(value.unwrap_or("false"), err, "versions", "boolean"));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "prefix" => {
                    call = call.prefix(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "delimiter" => {
                    call = call.delimiter(value.unwrap_or(""));
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

        if self.opt.cmd_bucket_access_controls {
            if self.opt.cmd_delete {
                call_result = self._bucket_access_controls_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._bucket_access_controls_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._bucket_access_controls_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._bucket_access_controls_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._bucket_access_controls_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._bucket_access_controls_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_buckets {
            if self.opt.cmd_delete {
                call_result = self._buckets_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._buckets_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._buckets_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._buckets_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._buckets_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._buckets_update(dry_run, &mut err);
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
 else if self.opt.cmd_default_object_access_controls {
            if self.opt.cmd_delete {
                call_result = self._default_object_access_controls_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._default_object_access_controls_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._default_object_access_controls_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._default_object_access_controls_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._default_object_access_controls_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._default_object_access_controls_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_object_access_controls {
            if self.opt.cmd_delete {
                call_result = self._object_access_controls_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._object_access_controls_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._object_access_controls_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._object_access_controls_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._object_access_controls_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._object_access_controls_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_objects {
            if self.opt.cmd_compose {
                call_result = self._objects_compose(dry_run, &mut err);
            } else if self.opt.cmd_copy {
                call_result = self._objects_copy(dry_run, &mut err);
            } else if self.opt.cmd_delete {
                call_result = self._objects_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._objects_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._objects_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._objects_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._objects_patch(dry_run, &mut err);
            } else if self.opt.cmd_rewrite {
                call_result = self._objects_rewrite(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._objects_update(dry_run, &mut err);
            } else if self.opt.cmd_watch_all {
                call_result = self._objects_watch_all(dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "storage1-secret.json", 
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
                                          program_name: "storage1",
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
            hub: api::Storage::new(client, auth),
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