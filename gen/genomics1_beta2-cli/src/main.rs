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
extern crate google_genomics1_beta2 as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  genomics1-beta2 [options] annotation-sets create -r <kv>... [-p <v>]... [-o <out>]
  genomics1-beta2 [options] annotation-sets delete <annotation-set-id> [-p <v>]...
  genomics1-beta2 [options] annotation-sets get <annotation-set-id> [-p <v>]... [-o <out>]
  genomics1-beta2 [options] annotation-sets patch <annotation-set-id> -r <kv>... [-p <v>]... [-o <out>]
  genomics1-beta2 [options] annotation-sets search -r <kv>... [-p <v>]... [-o <out>]
  genomics1-beta2 [options] annotation-sets update <annotation-set-id> -r <kv>... [-p <v>]... [-o <out>]
  genomics1-beta2 [options] annotations batch-create -r <kv>... [-p <v>]... [-o <out>]
  genomics1-beta2 [options] annotations create -r <kv>... [-p <v>]... [-o <out>]
  genomics1-beta2 [options] annotations delete <annotation-id> [-p <v>]...
  genomics1-beta2 [options] annotations get <annotation-id> [-p <v>]... [-o <out>]
  genomics1-beta2 [options] annotations patch <annotation-id> -r <kv>... [-p <v>]... [-o <out>]
  genomics1-beta2 [options] annotations search -r <kv>... [-p <v>]... [-o <out>]
  genomics1-beta2 [options] annotations update <annotation-id> -r <kv>... [-p <v>]... [-o <out>]
  genomics1-beta2 [options] callsets create -r <kv>... [-p <v>]... [-o <out>]
  genomics1-beta2 [options] callsets delete <call-set-id> [-p <v>]...
  genomics1-beta2 [options] callsets get <call-set-id> [-p <v>]... [-o <out>]
  genomics1-beta2 [options] callsets patch <call-set-id> -r <kv>... [-p <v>]... [-o <out>]
  genomics1-beta2 [options] callsets search -r <kv>... [-p <v>]... [-o <out>]
  genomics1-beta2 [options] callsets update <call-set-id> -r <kv>... [-p <v>]... [-o <out>]
  genomics1-beta2 [options] datasets create -r <kv>... [-p <v>]... [-o <out>]
  genomics1-beta2 [options] datasets delete <dataset-id> [-p <v>]...
  genomics1-beta2 [options] datasets get <dataset-id> [-p <v>]... [-o <out>]
  genomics1-beta2 [options] datasets list [-p <v>]... [-o <out>]
  genomics1-beta2 [options] datasets patch <dataset-id> -r <kv>... [-p <v>]... [-o <out>]
  genomics1-beta2 [options] datasets undelete <dataset-id> [-p <v>]... [-o <out>]
  genomics1-beta2 [options] datasets update <dataset-id> -r <kv>... [-p <v>]... [-o <out>]
  genomics1-beta2 [options] experimental jobs-create -r <kv>... [-p <v>]... [-o <out>]
  genomics1-beta2 [options] jobs cancel <job-id> [-p <v>]...
  genomics1-beta2 [options] jobs get <job-id> [-p <v>]... [-o <out>]
  genomics1-beta2 [options] jobs search -r <kv>... [-p <v>]... [-o <out>]
  genomics1-beta2 [options] readgroupsets align -r <kv>... [-p <v>]... [-o <out>]
  genomics1-beta2 [options] readgroupsets call -r <kv>... [-p <v>]... [-o <out>]
  genomics1-beta2 [options] readgroupsets coveragebuckets-list <read-group-set-id> [-p <v>]... [-o <out>]
  genomics1-beta2 [options] readgroupsets delete <read-group-set-id> [-p <v>]...
  genomics1-beta2 [options] readgroupsets export -r <kv>... [-p <v>]... [-o <out>]
  genomics1-beta2 [options] readgroupsets get <read-group-set-id> [-p <v>]... [-o <out>]
  genomics1-beta2 [options] readgroupsets import -r <kv>... [-p <v>]... [-o <out>]
  genomics1-beta2 [options] readgroupsets patch <read-group-set-id> -r <kv>... [-p <v>]... [-o <out>]
  genomics1-beta2 [options] readgroupsets search -r <kv>... [-p <v>]... [-o <out>]
  genomics1-beta2 [options] readgroupsets update <read-group-set-id> -r <kv>... [-p <v>]... [-o <out>]
  genomics1-beta2 [options] reads search -r <kv>... [-p <v>]... [-o <out>]
  genomics1-beta2 [options] references bases-list <reference-id> [-p <v>]... [-o <out>]
  genomics1-beta2 [options] references get <reference-id> [-p <v>]... [-o <out>]
  genomics1-beta2 [options] references search -r <kv>... [-p <v>]... [-o <out>]
  genomics1-beta2 [options] referencesets get <reference-set-id> [-p <v>]... [-o <out>]
  genomics1-beta2 [options] referencesets search -r <kv>... [-p <v>]... [-o <out>]
  genomics1-beta2 [options] streaming-readstore streamreads -r <kv>... [-p <v>]... [-o <out>]
  genomics1-beta2 [options] variants create -r <kv>... [-p <v>]... [-o <out>]
  genomics1-beta2 [options] variants delete <variant-id> [-p <v>]...
  genomics1-beta2 [options] variants get <variant-id> [-p <v>]... [-o <out>]
  genomics1-beta2 [options] variants search -r <kv>... [-p <v>]... [-o <out>]
  genomics1-beta2 [options] variants update <variant-id> -r <kv>... [-p <v>]... [-o <out>]
  genomics1-beta2 [options] variantsets delete <variant-set-id> [-p <v>]...
  genomics1-beta2 [options] variantsets export <variant-set-id> -r <kv>... [-p <v>]... [-o <out>]
  genomics1-beta2 [options] variantsets get <variant-set-id> [-p <v>]... [-o <out>]
  genomics1-beta2 [options] variantsets import-variants <variant-set-id> -r <kv>... [-p <v>]... [-o <out>]
  genomics1-beta2 [options] variantsets merge-variants <variant-set-id> -r <kv>... [-p <v>]...
  genomics1-beta2 [options] variantsets patch <variant-set-id> -r <kv>... [-p <v>]... [-o <out>]
  genomics1-beta2 [options] variantsets search -r <kv>... [-p <v>]... [-o <out>]
  genomics1-beta2 [options] variantsets update <variant-set-id> -r <kv>... [-p <v>]... [-o <out>]
  genomics1-beta2 --help

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
    hub: api::Genomics<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _annotation_sets_create(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::AnnotationSet::default();
        let mut call = self.hub.annotation_sets().create(&request);
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
                "source-uri" => {
                        request.source_uri = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "reference-set-id" => {
                        request.reference_set_id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "dataset-id" => {
                        request.dataset_id = Some(value.unwrap_or("").to_string());
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

    fn _annotation_sets_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.annotation_sets().delete(&self.opt.arg_annotation_set_id);
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

    fn _annotation_sets_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.annotation_sets().get(&self.opt.arg_annotation_set_id);
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

    fn _annotation_sets_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::AnnotationSet::default();
        let mut call = self.hub.annotation_sets().patch(&request, &self.opt.arg_annotation_set_id);
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
                "source-uri" => {
                        request.source_uri = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "reference-set-id" => {
                        request.reference_set_id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "dataset-id" => {
                        request.dataset_id = Some(value.unwrap_or("").to_string());
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

    fn _annotation_sets_search(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::SearchAnnotationSetsRequest::default();
        let mut call = self.hub.annotation_sets().search(&request);
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
                "page-size" => {
                        request.page_size = Some(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
                    },
                "dataset-ids" => {
                        if request.dataset_ids.is_none() {
                           request.dataset_ids = Some(Default::default());
                        }
                                        request.dataset_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "page-token" => {
                        request.page_token = Some(value.unwrap_or("").to_string());
                    },
                "reference-set-id" => {
                        request.reference_set_id = Some(value.unwrap_or("").to_string());
                    },
                "types" => {
                        if request.types.is_none() {
                           request.types = Some(Default::default());
                        }
                                        request.types.as_mut().unwrap().push(value.unwrap_or("").to_string());
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

    fn _annotation_sets_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::AnnotationSet::default();
        let mut call = self.hub.annotation_sets().update(&request, &self.opt.arg_annotation_set_id);
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
                "source-uri" => {
                        request.source_uri = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "reference-set-id" => {
                        request.reference_set_id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "dataset-id" => {
                        request.dataset_id = Some(value.unwrap_or("").to_string());
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

    fn _annotations_batch_create(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::BatchCreateAnnotationsRequest::default();
        let mut call = self.hub.annotations().batch_create(&request);
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

    fn _annotations_create(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Annotation::default();
        let mut call = self.hub.annotations().create(&request);
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
            fn request_position_init(request: &mut api::Annotation) {
                if request.position.is_none() {
                    request.position = Some(Default::default());
                }
            }
            
            fn request_transcript_coding_sequence_init(request: &mut api::Annotation) {
                request_transcript_init(request);
                if request.transcript.as_mut().unwrap().coding_sequence.is_none() {
                    request.transcript.as_mut().unwrap().coding_sequence = Some(Default::default());
                }
            }
            
            fn request_transcript_init(request: &mut api::Annotation) {
                if request.transcript.is_none() {
                    request.transcript = Some(Default::default());
                }
            }
            
            fn request_variant_init(request: &mut api::Annotation) {
                if request.variant.is_none() {
                    request.variant = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "transcript.coding-sequence.start" => {
                        request_transcript_coding_sequence_init(&mut request);
                        request.transcript.as_mut().unwrap().coding_sequence.as_mut().unwrap().start = Some(value.unwrap_or("").to_string());
                    },
                "transcript.coding-sequence.end" => {
                        request_transcript_coding_sequence_init(&mut request);
                        request.transcript.as_mut().unwrap().coding_sequence.as_mut().unwrap().end = Some(value.unwrap_or("").to_string());
                    },
                "transcript.gene-id" => {
                        request_transcript_coding_sequence_init(&mut request);
                        request.transcript.as_mut().unwrap().gene_id = Some(value.unwrap_or("").to_string());
                    },
                "variant.effect" => {
                        request_variant_init(&mut request);
                        request.variant.as_mut().unwrap().effect = Some(value.unwrap_or("").to_string());
                    },
                "variant.transcript-ids" => {
                        request_variant_init(&mut request);
                        if request.variant.as_mut().unwrap().transcript_ids.is_none() {
                           request.variant.as_mut().unwrap().transcript_ids = Some(Default::default());
                        }
                                        request.variant.as_mut().unwrap().transcript_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "variant.alternate-bases" => {
                        request_variant_init(&mut request);
                        request.variant.as_mut().unwrap().alternate_bases = Some(value.unwrap_or("").to_string());
                    },
                "variant.clinical-significance" => {
                        request_variant_init(&mut request);
                        request.variant.as_mut().unwrap().clinical_significance = Some(value.unwrap_or("").to_string());
                    },
                "variant.type" => {
                        request_variant_init(&mut request);
                        request.variant.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "variant.gene-id" => {
                        request_variant_init(&mut request);
                        request.variant.as_mut().unwrap().gene_id = Some(value.unwrap_or("").to_string());
                    },
                "annotation-set-id" => {
                        request_variant_init(&mut request);
                        request.annotation_set_id = Some(value.unwrap_or("").to_string());
                    },
                "position.start" => {
                        request_position_init(&mut request);
                        request.position.as_mut().unwrap().start = Some(value.unwrap_or("").to_string());
                    },
                "position.reference-id" => {
                        request_position_init(&mut request);
                        request.position.as_mut().unwrap().reference_id = Some(value.unwrap_or("").to_string());
                    },
                "position.end" => {
                        request_position_init(&mut request);
                        request.position.as_mut().unwrap().end = Some(value.unwrap_or("").to_string());
                    },
                "position.reverse-strand" => {
                        request_position_init(&mut request);
                        request.position.as_mut().unwrap().reverse_strand = Some(arg_from_str(value.unwrap_or("false"), err, "position.reverse-strand", "boolean"));
                    },
                "position.reference-name" => {
                        request_position_init(&mut request);
                        request.position.as_mut().unwrap().reference_name = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_position_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_position_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
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

    fn _annotations_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.annotations().delete(&self.opt.arg_annotation_id);
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

    fn _annotations_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.annotations().get(&self.opt.arg_annotation_id);
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

    fn _annotations_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Annotation::default();
        let mut call = self.hub.annotations().patch(&request, &self.opt.arg_annotation_id);
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
            fn request_position_init(request: &mut api::Annotation) {
                if request.position.is_none() {
                    request.position = Some(Default::default());
                }
            }
            
            fn request_transcript_coding_sequence_init(request: &mut api::Annotation) {
                request_transcript_init(request);
                if request.transcript.as_mut().unwrap().coding_sequence.is_none() {
                    request.transcript.as_mut().unwrap().coding_sequence = Some(Default::default());
                }
            }
            
            fn request_transcript_init(request: &mut api::Annotation) {
                if request.transcript.is_none() {
                    request.transcript = Some(Default::default());
                }
            }
            
            fn request_variant_init(request: &mut api::Annotation) {
                if request.variant.is_none() {
                    request.variant = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "transcript.coding-sequence.start" => {
                        request_transcript_coding_sequence_init(&mut request);
                        request.transcript.as_mut().unwrap().coding_sequence.as_mut().unwrap().start = Some(value.unwrap_or("").to_string());
                    },
                "transcript.coding-sequence.end" => {
                        request_transcript_coding_sequence_init(&mut request);
                        request.transcript.as_mut().unwrap().coding_sequence.as_mut().unwrap().end = Some(value.unwrap_or("").to_string());
                    },
                "transcript.gene-id" => {
                        request_transcript_coding_sequence_init(&mut request);
                        request.transcript.as_mut().unwrap().gene_id = Some(value.unwrap_or("").to_string());
                    },
                "variant.effect" => {
                        request_variant_init(&mut request);
                        request.variant.as_mut().unwrap().effect = Some(value.unwrap_or("").to_string());
                    },
                "variant.transcript-ids" => {
                        request_variant_init(&mut request);
                        if request.variant.as_mut().unwrap().transcript_ids.is_none() {
                           request.variant.as_mut().unwrap().transcript_ids = Some(Default::default());
                        }
                                        request.variant.as_mut().unwrap().transcript_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "variant.alternate-bases" => {
                        request_variant_init(&mut request);
                        request.variant.as_mut().unwrap().alternate_bases = Some(value.unwrap_or("").to_string());
                    },
                "variant.clinical-significance" => {
                        request_variant_init(&mut request);
                        request.variant.as_mut().unwrap().clinical_significance = Some(value.unwrap_or("").to_string());
                    },
                "variant.type" => {
                        request_variant_init(&mut request);
                        request.variant.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "variant.gene-id" => {
                        request_variant_init(&mut request);
                        request.variant.as_mut().unwrap().gene_id = Some(value.unwrap_or("").to_string());
                    },
                "annotation-set-id" => {
                        request_variant_init(&mut request);
                        request.annotation_set_id = Some(value.unwrap_or("").to_string());
                    },
                "position.start" => {
                        request_position_init(&mut request);
                        request.position.as_mut().unwrap().start = Some(value.unwrap_or("").to_string());
                    },
                "position.reference-id" => {
                        request_position_init(&mut request);
                        request.position.as_mut().unwrap().reference_id = Some(value.unwrap_or("").to_string());
                    },
                "position.end" => {
                        request_position_init(&mut request);
                        request.position.as_mut().unwrap().end = Some(value.unwrap_or("").to_string());
                    },
                "position.reverse-strand" => {
                        request_position_init(&mut request);
                        request.position.as_mut().unwrap().reverse_strand = Some(arg_from_str(value.unwrap_or("false"), err, "position.reverse-strand", "boolean"));
                    },
                "position.reference-name" => {
                        request_position_init(&mut request);
                        request.position.as_mut().unwrap().reference_name = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_position_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_position_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
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

    fn _annotations_search(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::SearchAnnotationsRequest::default();
        let mut call = self.hub.annotations().search(&request);
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
            fn request_range_init(request: &mut api::SearchAnnotationsRequest) {
                if request.range.is_none() {
                    request.range = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "page-token" => {
                        request.page_token = Some(value.unwrap_or("").to_string());
                    },
                "range.start" => {
                        request_range_init(&mut request);
                        request.range.as_mut().unwrap().start = Some(value.unwrap_or("").to_string());
                    },
                "range.reference-id" => {
                        request_range_init(&mut request);
                        request.range.as_mut().unwrap().reference_id = Some(value.unwrap_or("").to_string());
                    },
                "range.end" => {
                        request_range_init(&mut request);
                        request.range.as_mut().unwrap().end = Some(value.unwrap_or("").to_string());
                    },
                "range.reference-name" => {
                        request_range_init(&mut request);
                        request.range.as_mut().unwrap().reference_name = Some(value.unwrap_or("").to_string());
                    },
                "annotation-set-ids" => {
                        request_range_init(&mut request);
                        if request.annotation_set_ids.is_none() {
                           request.annotation_set_ids = Some(Default::default());
                        }
                                        request.annotation_set_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "page-size" => {
                        request_range_init(&mut request);
                        request.page_size = Some(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
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

    fn _annotations_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Annotation::default();
        let mut call = self.hub.annotations().update(&request, &self.opt.arg_annotation_id);
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
            fn request_position_init(request: &mut api::Annotation) {
                if request.position.is_none() {
                    request.position = Some(Default::default());
                }
            }
            
            fn request_transcript_coding_sequence_init(request: &mut api::Annotation) {
                request_transcript_init(request);
                if request.transcript.as_mut().unwrap().coding_sequence.is_none() {
                    request.transcript.as_mut().unwrap().coding_sequence = Some(Default::default());
                }
            }
            
            fn request_transcript_init(request: &mut api::Annotation) {
                if request.transcript.is_none() {
                    request.transcript = Some(Default::default());
                }
            }
            
            fn request_variant_init(request: &mut api::Annotation) {
                if request.variant.is_none() {
                    request.variant = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "transcript.coding-sequence.start" => {
                        request_transcript_coding_sequence_init(&mut request);
                        request.transcript.as_mut().unwrap().coding_sequence.as_mut().unwrap().start = Some(value.unwrap_or("").to_string());
                    },
                "transcript.coding-sequence.end" => {
                        request_transcript_coding_sequence_init(&mut request);
                        request.transcript.as_mut().unwrap().coding_sequence.as_mut().unwrap().end = Some(value.unwrap_or("").to_string());
                    },
                "transcript.gene-id" => {
                        request_transcript_coding_sequence_init(&mut request);
                        request.transcript.as_mut().unwrap().gene_id = Some(value.unwrap_or("").to_string());
                    },
                "variant.effect" => {
                        request_variant_init(&mut request);
                        request.variant.as_mut().unwrap().effect = Some(value.unwrap_or("").to_string());
                    },
                "variant.transcript-ids" => {
                        request_variant_init(&mut request);
                        if request.variant.as_mut().unwrap().transcript_ids.is_none() {
                           request.variant.as_mut().unwrap().transcript_ids = Some(Default::default());
                        }
                                        request.variant.as_mut().unwrap().transcript_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "variant.alternate-bases" => {
                        request_variant_init(&mut request);
                        request.variant.as_mut().unwrap().alternate_bases = Some(value.unwrap_or("").to_string());
                    },
                "variant.clinical-significance" => {
                        request_variant_init(&mut request);
                        request.variant.as_mut().unwrap().clinical_significance = Some(value.unwrap_or("").to_string());
                    },
                "variant.type" => {
                        request_variant_init(&mut request);
                        request.variant.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "variant.gene-id" => {
                        request_variant_init(&mut request);
                        request.variant.as_mut().unwrap().gene_id = Some(value.unwrap_or("").to_string());
                    },
                "annotation-set-id" => {
                        request_variant_init(&mut request);
                        request.annotation_set_id = Some(value.unwrap_or("").to_string());
                    },
                "position.start" => {
                        request_position_init(&mut request);
                        request.position.as_mut().unwrap().start = Some(value.unwrap_or("").to_string());
                    },
                "position.reference-id" => {
                        request_position_init(&mut request);
                        request.position.as_mut().unwrap().reference_id = Some(value.unwrap_or("").to_string());
                    },
                "position.end" => {
                        request_position_init(&mut request);
                        request.position.as_mut().unwrap().end = Some(value.unwrap_or("").to_string());
                    },
                "position.reverse-strand" => {
                        request_position_init(&mut request);
                        request.position.as_mut().unwrap().reverse_strand = Some(arg_from_str(value.unwrap_or("false"), err, "position.reverse-strand", "boolean"));
                    },
                "position.reference-name" => {
                        request_position_init(&mut request);
                        request.position.as_mut().unwrap().reference_name = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_position_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_position_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
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

    fn _callsets_create(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::CallSet::default();
        let mut call = self.hub.callsets().create(&request);
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
                "sample-id" => {
                        request.sample_id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "variant-set-ids" => {
                        if request.variant_set_ids.is_none() {
                           request.variant_set_ids = Some(Default::default());
                        }
                                        request.variant_set_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request.created = Some(value.unwrap_or("").to_string());
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

    fn _callsets_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.callsets().delete(&self.opt.arg_call_set_id);
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

    fn _callsets_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.callsets().get(&self.opt.arg_call_set_id);
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

    fn _callsets_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::CallSet::default();
        let mut call = self.hub.callsets().patch(&request, &self.opt.arg_call_set_id);
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
                "sample-id" => {
                        request.sample_id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "variant-set-ids" => {
                        if request.variant_set_ids.is_none() {
                           request.variant_set_ids = Some(Default::default());
                        }
                                        request.variant_set_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request.created = Some(value.unwrap_or("").to_string());
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

    fn _callsets_search(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::SearchCallSetsRequest::default();
        let mut call = self.hub.callsets().search(&request);
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
                "page-token" => {
                        request.page_token = Some(value.unwrap_or("").to_string());
                    },
                "variant-set-ids" => {
                        if request.variant_set_ids.is_none() {
                           request.variant_set_ids = Some(Default::default());
                        }
                                        request.variant_set_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "page-size" => {
                        request.page_size = Some(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
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

    fn _callsets_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::CallSet::default();
        let mut call = self.hub.callsets().update(&request, &self.opt.arg_call_set_id);
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
                "sample-id" => {
                        request.sample_id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "variant-set-ids" => {
                        if request.variant_set_ids.is_none() {
                           request.variant_set_ids = Some(Default::default());
                        }
                                        request.variant_set_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request.created = Some(value.unwrap_or("").to_string());
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

    fn _datasets_create(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Dataset::default();
        let mut call = self.hub.datasets().create(&request);
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
                "is-public" => {
                        request.is_public = Some(arg_from_str(value.unwrap_or("false"), err, "is-public", "boolean"));
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "project-number" => {
                        request.project_number = Some(value.unwrap_or("").to_string());
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _datasets_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.datasets().delete(&self.opt.arg_dataset_id);
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

    fn _datasets_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.datasets().get(&self.opt.arg_dataset_id);
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

    fn _datasets_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.datasets().list();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "project-number" => {
                    call = call.project_number(value.unwrap_or(""));
                },
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _datasets_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Dataset::default();
        let mut call = self.hub.datasets().patch(&request, &self.opt.arg_dataset_id);
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
                "is-public" => {
                        request.is_public = Some(arg_from_str(value.unwrap_or("false"), err, "is-public", "boolean"));
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "project-number" => {
                        request.project_number = Some(value.unwrap_or("").to_string());
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _datasets_undelete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.datasets().undelete(&self.opt.arg_dataset_id);
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

    fn _datasets_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Dataset::default();
        let mut call = self.hub.datasets().update(&request, &self.opt.arg_dataset_id);
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
                "is-public" => {
                        request.is_public = Some(arg_from_str(value.unwrap_or("false"), err, "is-public", "boolean"));
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "project-number" => {
                        request.project_number = Some(value.unwrap_or("").to_string());
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _experimental_jobs_create(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::ExperimentalCreateJobRequest::default();
        let mut call = self.hub.experimental().jobs_create(&request);
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
                "paired-source-uris" => {
                        if request.paired_source_uris.is_none() {
                           request.paired_source_uris = Some(Default::default());
                        }
                                        request.paired_source_uris.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "align" => {
                        request.align = Some(arg_from_str(value.unwrap_or("false"), err, "align", "boolean"));
                    },
                "call-variants" => {
                        request.call_variants = Some(arg_from_str(value.unwrap_or("false"), err, "call-variants", "boolean"));
                    },
                "source-uris" => {
                        if request.source_uris.is_none() {
                           request.source_uris = Some(Default::default());
                        }
                                        request.source_uris.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "gcs-output-path" => {
                        request.gcs_output_path = Some(value.unwrap_or("").to_string());
                    },
                "project-number" => {
                        request.project_number = Some(value.unwrap_or("").to_string());
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

    fn _jobs_cancel(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.jobs().cancel(&self.opt.arg_job_id);
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

    fn _jobs_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.jobs().get(&self.opt.arg_job_id);
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

    fn _jobs_search(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::SearchJobsRequest::default();
        let mut call = self.hub.jobs().search(&request);
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
                "status" => {
                        if request.status.is_none() {
                           request.status = Some(Default::default());
                        }
                                        request.status.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "created-after" => {
                        request.created_after = Some(value.unwrap_or("").to_string());
                    },
                "page-size" => {
                        request.page_size = Some(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
                    },
                "page-token" => {
                        request.page_token = Some(value.unwrap_or("").to_string());
                    },
                "created-before" => {
                        request.created_before = Some(value.unwrap_or("").to_string());
                    },
                "project-number" => {
                        request.project_number = Some(value.unwrap_or("").to_string());
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

    fn _readgroupsets_align(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::AlignReadGroupSetsRequest::default();
        let mut call = self.hub.readgroupsets().align(&request);
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
            fn request_interleaved_fastq_source_init(request: &mut api::AlignReadGroupSetsRequest) {
                if request.interleaved_fastq_source.is_none() {
                    request.interleaved_fastq_source = Some(Default::default());
                }
            }
            
            fn request_interleaved_fastq_source_metadata_init(request: &mut api::AlignReadGroupSetsRequest) {
                request_interleaved_fastq_source_init(request);
                if request.interleaved_fastq_source.as_mut().unwrap().metadata.is_none() {
                    request.interleaved_fastq_source.as_mut().unwrap().metadata = Some(Default::default());
                }
            }
            
            fn request_paired_fastq_source_init(request: &mut api::AlignReadGroupSetsRequest) {
                if request.paired_fastq_source.is_none() {
                    request.paired_fastq_source = Some(Default::default());
                }
            }
            
            fn request_paired_fastq_source_metadata_init(request: &mut api::AlignReadGroupSetsRequest) {
                request_paired_fastq_source_init(request);
                if request.paired_fastq_source.as_mut().unwrap().metadata.is_none() {
                    request.paired_fastq_source.as_mut().unwrap().metadata = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "interleaved-fastq-source.source-uris" => {
                        request_interleaved_fastq_source_init(&mut request);
                        if request.interleaved_fastq_source.as_mut().unwrap().source_uris.is_none() {
                           request.interleaved_fastq_source.as_mut().unwrap().source_uris = Some(Default::default());
                        }
                                        request.interleaved_fastq_source.as_mut().unwrap().source_uris.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "interleaved-fastq-source.metadata.read-group-name" => {
                        request_interleaved_fastq_source_metadata_init(&mut request);
                        request.interleaved_fastq_source.as_mut().unwrap().metadata.as_mut().unwrap().read_group_name = Some(value.unwrap_or("").to_string());
                    },
                "interleaved-fastq-source.metadata.sample-name" => {
                        request_interleaved_fastq_source_metadata_init(&mut request);
                        request.interleaved_fastq_source.as_mut().unwrap().metadata.as_mut().unwrap().sample_name = Some(value.unwrap_or("").to_string());
                    },
                "interleaved-fastq-source.metadata.library-name" => {
                        request_interleaved_fastq_source_metadata_init(&mut request);
                        request.interleaved_fastq_source.as_mut().unwrap().metadata.as_mut().unwrap().library_name = Some(value.unwrap_or("").to_string());
                    },
                "interleaved-fastq-source.metadata.platform-name" => {
                        request_interleaved_fastq_source_metadata_init(&mut request);
                        request.interleaved_fastq_source.as_mut().unwrap().metadata.as_mut().unwrap().platform_name = Some(value.unwrap_or("").to_string());
                    },
                "interleaved-fastq-source.metadata.platform-unit" => {
                        request_interleaved_fastq_source_metadata_init(&mut request);
                        request.interleaved_fastq_source.as_mut().unwrap().metadata.as_mut().unwrap().platform_unit = Some(value.unwrap_or("").to_string());
                    },
                "bam-source-uris" => {
                        request_interleaved_fastq_source_init(&mut request);
                        if request.bam_source_uris.is_none() {
                           request.bam_source_uris = Some(Default::default());
                        }
                                        request.bam_source_uris.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "paired-fastq-source.second-source-uris" => {
                        request_paired_fastq_source_init(&mut request);
                        if request.paired_fastq_source.as_mut().unwrap().second_source_uris.is_none() {
                           request.paired_fastq_source.as_mut().unwrap().second_source_uris = Some(Default::default());
                        }
                                        request.paired_fastq_source.as_mut().unwrap().second_source_uris.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "paired-fastq-source.metadata.read-group-name" => {
                        request_paired_fastq_source_metadata_init(&mut request);
                        request.paired_fastq_source.as_mut().unwrap().metadata.as_mut().unwrap().read_group_name = Some(value.unwrap_or("").to_string());
                    },
                "paired-fastq-source.metadata.sample-name" => {
                        request_paired_fastq_source_metadata_init(&mut request);
                        request.paired_fastq_source.as_mut().unwrap().metadata.as_mut().unwrap().sample_name = Some(value.unwrap_or("").to_string());
                    },
                "paired-fastq-source.metadata.library-name" => {
                        request_paired_fastq_source_metadata_init(&mut request);
                        request.paired_fastq_source.as_mut().unwrap().metadata.as_mut().unwrap().library_name = Some(value.unwrap_or("").to_string());
                    },
                "paired-fastq-source.metadata.platform-name" => {
                        request_paired_fastq_source_metadata_init(&mut request);
                        request.paired_fastq_source.as_mut().unwrap().metadata.as_mut().unwrap().platform_name = Some(value.unwrap_or("").to_string());
                    },
                "paired-fastq-source.metadata.platform-unit" => {
                        request_paired_fastq_source_metadata_init(&mut request);
                        request.paired_fastq_source.as_mut().unwrap().metadata.as_mut().unwrap().platform_unit = Some(value.unwrap_or("").to_string());
                    },
                "paired-fastq-source.first-source-uris" => {
                        request_paired_fastq_source_metadata_init(&mut request);
                        if request.paired_fastq_source.as_mut().unwrap().first_source_uris.is_none() {
                           request.paired_fastq_source.as_mut().unwrap().first_source_uris = Some(Default::default());
                        }
                                        request.paired_fastq_source.as_mut().unwrap().first_source_uris.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "read-group-set-id" => {
                        request_paired_fastq_source_init(&mut request);
                        request.read_group_set_id = Some(value.unwrap_or("").to_string());
                    },
                "dataset-id" => {
                        request_paired_fastq_source_init(&mut request);
                        request.dataset_id = Some(value.unwrap_or("").to_string());
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

    fn _readgroupsets_call(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::CallReadGroupSetsRequest::default();
        let mut call = self.hub.readgroupsets().call(&request);
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
                "source-uris" => {
                        if request.source_uris.is_none() {
                           request.source_uris = Some(Default::default());
                        }
                                        request.source_uris.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "read-group-set-id" => {
                        request.read_group_set_id = Some(value.unwrap_or("").to_string());
                    },
                "dataset-id" => {
                        request.dataset_id = Some(value.unwrap_or("").to_string());
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

    fn _readgroupsets_coveragebuckets_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.readgroupsets().coveragebuckets_list(&self.opt.arg_read_group_set_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "target-bucket-width" => {
                    call = call.target_bucket_width(value.unwrap_or(""));
                },
                "range-start" => {
                    call = call.range_start(value.unwrap_or(""));
                },
                "range-reference-name" => {
                    call = call.range_reference_name(value.unwrap_or(""));
                },
                "range-end" => {
                    call = call.range_end(value.unwrap_or(""));
                },
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _readgroupsets_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.readgroupsets().delete(&self.opt.arg_read_group_set_id);
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

    fn _readgroupsets_export(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::ExportReadGroupSetsRequest::default();
        let mut call = self.hub.readgroupsets().export(&request);
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
                "export-uri" => {
                        request.export_uri = Some(value.unwrap_or("").to_string());
                    },
                "reference-names" => {
                        if request.reference_names.is_none() {
                           request.reference_names = Some(Default::default());
                        }
                                        request.reference_names.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "read-group-set-ids" => {
                        if request.read_group_set_ids.is_none() {
                           request.read_group_set_ids = Some(Default::default());
                        }
                                        request.read_group_set_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "project-number" => {
                        request.project_number = Some(value.unwrap_or("").to_string());
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

    fn _readgroupsets_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.readgroupsets().get(&self.opt.arg_read_group_set_id);
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

    fn _readgroupsets_import(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::ImportReadGroupSetsRequest::default();
        let mut call = self.hub.readgroupsets().import(&request);
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
                "reference-set-id" => {
                        request.reference_set_id = Some(value.unwrap_or("").to_string());
                    },
                "source-uris" => {
                        if request.source_uris.is_none() {
                           request.source_uris = Some(Default::default());
                        }
                                        request.source_uris.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "partition-strategy" => {
                        request.partition_strategy = Some(value.unwrap_or("").to_string());
                    },
                "dataset-id" => {
                        request.dataset_id = Some(value.unwrap_or("").to_string());
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

    fn _readgroupsets_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::ReadGroupSet::default();
        let mut call = self.hub.readgroupsets().patch(&request, &self.opt.arg_read_group_set_id);
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
                "filename" => {
                        request.filename = Some(value.unwrap_or("").to_string());
                    },
                "reference-set-id" => {
                        request.reference_set_id = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "dataset-id" => {
                        request.dataset_id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
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

    fn _readgroupsets_search(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::SearchReadGroupSetsRequest::default();
        let mut call = self.hub.readgroupsets().search(&request);
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
                "page-token" => {
                        request.page_token = Some(value.unwrap_or("").to_string());
                    },
                "dataset-ids" => {
                        if request.dataset_ids.is_none() {
                           request.dataset_ids = Some(Default::default());
                        }
                                        request.dataset_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "page-size" => {
                        request.page_size = Some(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
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

    fn _readgroupsets_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::ReadGroupSet::default();
        let mut call = self.hub.readgroupsets().update(&request, &self.opt.arg_read_group_set_id);
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
                "filename" => {
                        request.filename = Some(value.unwrap_or("").to_string());
                    },
                "reference-set-id" => {
                        request.reference_set_id = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "dataset-id" => {
                        request.dataset_id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
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

    fn _reads_search(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::SearchReadsRequest::default();
        let mut call = self.hub.reads().search(&request);
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
                "read-group-set-ids" => {
                        if request.read_group_set_ids.is_none() {
                           request.read_group_set_ids = Some(Default::default());
                        }
                                        request.read_group_set_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "read-group-ids" => {
                        if request.read_group_ids.is_none() {
                           request.read_group_ids = Some(Default::default());
                        }
                                        request.read_group_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "page-size" => {
                        request.page_size = Some(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
                    },
                "start" => {
                        request.start = Some(value.unwrap_or("").to_string());
                    },
                "page-token" => {
                        request.page_token = Some(value.unwrap_or("").to_string());
                    },
                "reference-name" => {
                        request.reference_name = Some(value.unwrap_or("").to_string());
                    },
                "end" => {
                        request.end = Some(value.unwrap_or("").to_string());
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

    fn _references_bases_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.references().bases_list(&self.opt.arg_reference_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start" => {
                    call = call.start(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
                },
                "end" => {
                    call = call.end(value.unwrap_or(""));
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

    fn _references_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.references().get(&self.opt.arg_reference_id);
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

    fn _references_search(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::SearchReferencesRequest::default();
        let mut call = self.hub.references().search(&request);
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
                "md5checksums" => {
                        if request.md5checksums.is_none() {
                           request.md5checksums = Some(Default::default());
                        }
                                        request.md5checksums.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "page-token" => {
                        request.page_token = Some(value.unwrap_or("").to_string());
                    },
                "reference-set-id" => {
                        request.reference_set_id = Some(value.unwrap_or("").to_string());
                    },
                "accessions" => {
                        if request.accessions.is_none() {
                           request.accessions = Some(Default::default());
                        }
                                        request.accessions.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "page-size" => {
                        request.page_size = Some(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
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

    fn _referencesets_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.referencesets().get(&self.opt.arg_reference_set_id);
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

    fn _referencesets_search(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::SearchReferenceSetsRequest::default();
        let mut call = self.hub.referencesets().search(&request);
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
                "md5checksums" => {
                        if request.md5checksums.is_none() {
                           request.md5checksums = Some(Default::default());
                        }
                                        request.md5checksums.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "assembly-id" => {
                        request.assembly_id = Some(value.unwrap_or("").to_string());
                    },
                "accessions" => {
                        if request.accessions.is_none() {
                           request.accessions = Some(Default::default());
                        }
                                        request.accessions.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "page-size" => {
                        request.page_size = Some(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
                    },
                "page-token" => {
                        request.page_token = Some(value.unwrap_or("").to_string());
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

    fn _streaming_readstore_streamreads(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::StreamReadsRequest::default();
        let mut call = self.hub.streaming_readstore().streamreads(&request);
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
                "read-group-set-ids" => {
                        if request.read_group_set_ids.is_none() {
                           request.read_group_set_ids = Some(Default::default());
                        }
                                        request.read_group_set_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "end" => {
                        request.end = Some(value.unwrap_or("").to_string());
                    },
                "start" => {
                        request.start = Some(value.unwrap_or("").to_string());
                    },
                "reference-name" => {
                        request.reference_name = Some(value.unwrap_or("").to_string());
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

    fn _variants_create(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Variant::default();
        let mut call = self.hub.variants().create(&request);
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
                "variant-set-id" => {
                        request.variant_set_id = Some(value.unwrap_or("").to_string());
                    },
                "end" => {
                        request.end = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "reference-bases" => {
                        request.reference_bases = Some(value.unwrap_or("").to_string());
                    },
                "filter" => {
                        if request.filter.is_none() {
                           request.filter = Some(Default::default());
                        }
                                        request.filter.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "start" => {
                        request.start = Some(value.unwrap_or("").to_string());
                    },
                "names" => {
                        if request.names.is_none() {
                           request.names = Some(Default::default());
                        }
                                        request.names.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "alternate-bases" => {
                        if request.alternate_bases.is_none() {
                           request.alternate_bases = Some(Default::default());
                        }
                                        request.alternate_bases.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "reference-name" => {
                        request.reference_name = Some(value.unwrap_or("").to_string());
                    },
                "quality" => {
                        request.quality = Some(arg_from_str(value.unwrap_or("0.0"), err, "quality", "number"));
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
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

    fn _variants_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.variants().delete(&self.opt.arg_variant_id);
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

    fn _variants_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.variants().get(&self.opt.arg_variant_id);
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

    fn _variants_search(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::SearchVariantsRequest::default();
        let mut call = self.hub.variants().search(&request);
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
                "end" => {
                        request.end = Some(value.unwrap_or("").to_string());
                    },
                "page-size" => {
                        request.page_size = Some(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
                    },
                "start" => {
                        request.start = Some(value.unwrap_or("").to_string());
                    },
                "max-calls" => {
                        request.max_calls = Some(arg_from_str(value.unwrap_or("-0"), err, "max-calls", "integer"));
                    },
                "page-token" => {
                        request.page_token = Some(value.unwrap_or("").to_string());
                    },
                "call-set-ids" => {
                        if request.call_set_ids.is_none() {
                           request.call_set_ids = Some(Default::default());
                        }
                                        request.call_set_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "variant-name" => {
                        request.variant_name = Some(value.unwrap_or("").to_string());
                    },
                "reference-name" => {
                        request.reference_name = Some(value.unwrap_or("").to_string());
                    },
                "variant-set-ids" => {
                        if request.variant_set_ids.is_none() {
                           request.variant_set_ids = Some(Default::default());
                        }
                                        request.variant_set_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
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

    fn _variants_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Variant::default();
        let mut call = self.hub.variants().update(&request, &self.opt.arg_variant_id);
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
                "variant-set-id" => {
                        request.variant_set_id = Some(value.unwrap_or("").to_string());
                    },
                "end" => {
                        request.end = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "reference-bases" => {
                        request.reference_bases = Some(value.unwrap_or("").to_string());
                    },
                "filter" => {
                        if request.filter.is_none() {
                           request.filter = Some(Default::default());
                        }
                                        request.filter.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "start" => {
                        request.start = Some(value.unwrap_or("").to_string());
                    },
                "names" => {
                        if request.names.is_none() {
                           request.names = Some(Default::default());
                        }
                                        request.names.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "alternate-bases" => {
                        if request.alternate_bases.is_none() {
                           request.alternate_bases = Some(Default::default());
                        }
                                        request.alternate_bases.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "reference-name" => {
                        request.reference_name = Some(value.unwrap_or("").to_string());
                    },
                "quality" => {
                        request.quality = Some(arg_from_str(value.unwrap_or("0.0"), err, "quality", "number"));
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
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

    fn _variantsets_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.variantsets().delete(&self.opt.arg_variant_set_id);
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

    fn _variantsets_export(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::ExportVariantSetRequest::default();
        let mut call = self.hub.variantsets().export(&request, &self.opt.arg_variant_set_id);
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
                "bigquery-dataset" => {
                        request.bigquery_dataset = Some(value.unwrap_or("").to_string());
                    },
                "format" => {
                        request.format = Some(value.unwrap_or("").to_string());
                    },
                "call-set-ids" => {
                        if request.call_set_ids.is_none() {
                           request.call_set_ids = Some(Default::default());
                        }
                                        request.call_set_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "project-number" => {
                        request.project_number = Some(value.unwrap_or("").to_string());
                    },
                "bigquery-table" => {
                        request.bigquery_table = Some(value.unwrap_or("").to_string());
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

    fn _variantsets_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.variantsets().get(&self.opt.arg_variant_set_id);
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

    fn _variantsets_import_variants(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::ImportVariantsRequest::default();
        let mut call = self.hub.variantsets().import_variants(&request, &self.opt.arg_variant_set_id);
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
                "source-uris" => {
                        if request.source_uris.is_none() {
                           request.source_uris = Some(Default::default());
                        }
                                        request.source_uris.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "format" => {
                        request.format = Some(value.unwrap_or("").to_string());
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

    fn _variantsets_merge_variants(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::MergeVariantsRequest::default();
        let mut call = self.hub.variantsets().merge_variants(&request, &self.opt.arg_variant_set_id);
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

    fn _variantsets_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::VariantSet::default();
        let mut call = self.hub.variantsets().patch(&request, &self.opt.arg_variant_set_id);
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
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "dataset-id" => {
                        request.dataset_id = Some(value.unwrap_or("").to_string());
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

    fn _variantsets_search(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::SearchVariantSetsRequest::default();
        let mut call = self.hub.variantsets().search(&request);
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
                "page-token" => {
                        request.page_token = Some(value.unwrap_or("").to_string());
                    },
                "dataset-ids" => {
                        if request.dataset_ids.is_none() {
                           request.dataset_ids = Some(Default::default());
                        }
                                        request.dataset_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "page-size" => {
                        request.page_size = Some(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
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

    fn _variantsets_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::VariantSet::default();
        let mut call = self.hub.variantsets().update(&request, &self.opt.arg_variant_set_id);
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
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "dataset-id" => {
                        request.dataset_id = Some(value.unwrap_or("").to_string());
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

        if self.opt.cmd_annotation_sets {
            if self.opt.cmd_create {
                call_result = self._annotation_sets_create(dry_run, &mut err);
            } else if self.opt.cmd_delete {
                call_result = self._annotation_sets_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._annotation_sets_get(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._annotation_sets_patch(dry_run, &mut err);
            } else if self.opt.cmd_search {
                call_result = self._annotation_sets_search(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._annotation_sets_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_annotations {
            if self.opt.cmd_batch_create {
                call_result = self._annotations_batch_create(dry_run, &mut err);
            } else if self.opt.cmd_create {
                call_result = self._annotations_create(dry_run, &mut err);
            } else if self.opt.cmd_delete {
                call_result = self._annotations_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._annotations_get(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._annotations_patch(dry_run, &mut err);
            } else if self.opt.cmd_search {
                call_result = self._annotations_search(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._annotations_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_callsets {
            if self.opt.cmd_create {
                call_result = self._callsets_create(dry_run, &mut err);
            } else if self.opt.cmd_delete {
                call_result = self._callsets_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._callsets_get(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._callsets_patch(dry_run, &mut err);
            } else if self.opt.cmd_search {
                call_result = self._callsets_search(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._callsets_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_datasets {
            if self.opt.cmd_create {
                call_result = self._datasets_create(dry_run, &mut err);
            } else if self.opt.cmd_delete {
                call_result = self._datasets_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._datasets_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._datasets_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._datasets_patch(dry_run, &mut err);
            } else if self.opt.cmd_undelete {
                call_result = self._datasets_undelete(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._datasets_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_experimental {
            if self.opt.cmd_jobs_create {
                call_result = self._experimental_jobs_create(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_jobs {
            if self.opt.cmd_cancel {
                call_result = self._jobs_cancel(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._jobs_get(dry_run, &mut err);
            } else if self.opt.cmd_search {
                call_result = self._jobs_search(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_readgroupsets {
            if self.opt.cmd_align {
                call_result = self._readgroupsets_align(dry_run, &mut err);
            } else if self.opt.cmd_call {
                call_result = self._readgroupsets_call(dry_run, &mut err);
            } else if self.opt.cmd_coveragebuckets_list {
                call_result = self._readgroupsets_coveragebuckets_list(dry_run, &mut err);
            } else if self.opt.cmd_delete {
                call_result = self._readgroupsets_delete(dry_run, &mut err);
            } else if self.opt.cmd_export {
                call_result = self._readgroupsets_export(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._readgroupsets_get(dry_run, &mut err);
            } else if self.opt.cmd_import {
                call_result = self._readgroupsets_import(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._readgroupsets_patch(dry_run, &mut err);
            } else if self.opt.cmd_search {
                call_result = self._readgroupsets_search(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._readgroupsets_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_reads {
            if self.opt.cmd_search {
                call_result = self._reads_search(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_references {
            if self.opt.cmd_bases_list {
                call_result = self._references_bases_list(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._references_get(dry_run, &mut err);
            } else if self.opt.cmd_search {
                call_result = self._references_search(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_referencesets {
            if self.opt.cmd_get {
                call_result = self._referencesets_get(dry_run, &mut err);
            } else if self.opt.cmd_search {
                call_result = self._referencesets_search(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_streaming_readstore {
            if self.opt.cmd_streamreads {
                call_result = self._streaming_readstore_streamreads(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_variants {
            if self.opt.cmd_create {
                call_result = self._variants_create(dry_run, &mut err);
            } else if self.opt.cmd_delete {
                call_result = self._variants_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._variants_get(dry_run, &mut err);
            } else if self.opt.cmd_search {
                call_result = self._variants_search(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._variants_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_variantsets {
            if self.opt.cmd_delete {
                call_result = self._variantsets_delete(dry_run, &mut err);
            } else if self.opt.cmd_export {
                call_result = self._variantsets_export(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._variantsets_get(dry_run, &mut err);
            } else if self.opt.cmd_import_variants {
                call_result = self._variantsets_import_variants(dry_run, &mut err);
            } else if self.opt.cmd_merge_variants {
                call_result = self._variantsets_merge_variants(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._variantsets_patch(dry_run, &mut err);
            } else if self.opt.cmd_search {
                call_result = self._variantsets_search(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._variantsets_update(dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "genomics1-beta2-secret.json", 
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
                                          program_name: "genomics1-beta2",
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
            hub: api::Genomics::new(client, auth),
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