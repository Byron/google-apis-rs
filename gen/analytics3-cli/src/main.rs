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
extern crate google_analytics3 as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  analytics3 [options] data ga-get <ids> <start-date> <end-date> <metrics> [-p <v>...] [-o <out>]
  analytics3 [options] data mcf-get <ids> <start-date> <end-date> <metrics> [-p <v>...] [-o <out>]
  analytics3 [options] data realtime-get <ids> <metrics> [-p <v>...] [-o <out>]
  analytics3 [options] management account-summaries-list [-p <v>...] [-o <out>]
  analytics3 [options] management account-user-links-delete <account-id> <link-id> [-p <v>...]
  analytics3 [options] management account-user-links-insert <account-id> -r <kv>... [-p <v>...] [-o <out>]
  analytics3 [options] management account-user-links-list <account-id> [-p <v>...] [-o <out>]
  analytics3 [options] management account-user-links-update <account-id> <link-id> -r <kv>... [-p <v>...] [-o <out>]
  analytics3 [options] management accounts-list [-p <v>...] [-o <out>]
  analytics3 [options] management custom-data-sources-list <account-id> <web-property-id> [-p <v>...] [-o <out>]
  analytics3 [options] management custom-dimensions-get <account-id> <web-property-id> <custom-dimension-id> [-p <v>...] [-o <out>]
  analytics3 [options] management custom-dimensions-insert <account-id> <web-property-id> -r <kv>... [-p <v>...] [-o <out>]
  analytics3 [options] management custom-dimensions-list <account-id> <web-property-id> [-p <v>...] [-o <out>]
  analytics3 [options] management custom-dimensions-patch <account-id> <web-property-id> <custom-dimension-id> -r <kv>... [-p <v>...] [-o <out>]
  analytics3 [options] management custom-dimensions-update <account-id> <web-property-id> <custom-dimension-id> -r <kv>... [-p <v>...] [-o <out>]
  analytics3 [options] management custom-metrics-get <account-id> <web-property-id> <custom-metric-id> [-p <v>...] [-o <out>]
  analytics3 [options] management custom-metrics-insert <account-id> <web-property-id> -r <kv>... [-p <v>...] [-o <out>]
  analytics3 [options] management custom-metrics-list <account-id> <web-property-id> [-p <v>...] [-o <out>]
  analytics3 [options] management custom-metrics-patch <account-id> <web-property-id> <custom-metric-id> -r <kv>... [-p <v>...] [-o <out>]
  analytics3 [options] management custom-metrics-update <account-id> <web-property-id> <custom-metric-id> -r <kv>... [-p <v>...] [-o <out>]
  analytics3 [options] management experiments-delete <account-id> <web-property-id> <profile-id> <experiment-id> [-p <v>...]
  analytics3 [options] management experiments-get <account-id> <web-property-id> <profile-id> <experiment-id> [-p <v>...] [-o <out>]
  analytics3 [options] management experiments-insert <account-id> <web-property-id> <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  analytics3 [options] management experiments-list <account-id> <web-property-id> <profile-id> [-p <v>...] [-o <out>]
  analytics3 [options] management experiments-patch <account-id> <web-property-id> <profile-id> <experiment-id> -r <kv>... [-p <v>...] [-o <out>]
  analytics3 [options] management experiments-update <account-id> <web-property-id> <profile-id> <experiment-id> -r <kv>... [-p <v>...] [-o <out>]
  analytics3 [options] management filters-delete <account-id> <filter-id> [-p <v>...] [-o <out>]
  analytics3 [options] management filters-get <account-id> <filter-id> [-p <v>...] [-o <out>]
  analytics3 [options] management filters-insert <account-id> -r <kv>... [-p <v>...] [-o <out>]
  analytics3 [options] management filters-list <account-id> [-p <v>...] [-o <out>]
  analytics3 [options] management filters-patch <account-id> <filter-id> -r <kv>... [-p <v>...] [-o <out>]
  analytics3 [options] management filters-update <account-id> <filter-id> -r <kv>... [-p <v>...] [-o <out>]
  analytics3 [options] management goals-get <account-id> <web-property-id> <profile-id> <goal-id> [-p <v>...] [-o <out>]
  analytics3 [options] management goals-insert <account-id> <web-property-id> <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  analytics3 [options] management goals-list <account-id> <web-property-id> <profile-id> [-p <v>...] [-o <out>]
  analytics3 [options] management goals-patch <account-id> <web-property-id> <profile-id> <goal-id> -r <kv>... [-p <v>...] [-o <out>]
  analytics3 [options] management goals-update <account-id> <web-property-id> <profile-id> <goal-id> -r <kv>... [-p <v>...] [-o <out>]
  analytics3 [options] management profile-filter-links-delete <account-id> <web-property-id> <profile-id> <link-id> [-p <v>...]
  analytics3 [options] management profile-filter-links-get <account-id> <web-property-id> <profile-id> <link-id> [-p <v>...] [-o <out>]
  analytics3 [options] management profile-filter-links-insert <account-id> <web-property-id> <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  analytics3 [options] management profile-filter-links-list <account-id> <web-property-id> <profile-id> [-p <v>...] [-o <out>]
  analytics3 [options] management profile-filter-links-patch <account-id> <web-property-id> <profile-id> <link-id> -r <kv>... [-p <v>...] [-o <out>]
  analytics3 [options] management profile-filter-links-update <account-id> <web-property-id> <profile-id> <link-id> -r <kv>... [-p <v>...] [-o <out>]
  analytics3 [options] management profile-user-links-delete <account-id> <web-property-id> <profile-id> <link-id> [-p <v>...]
  analytics3 [options] management profile-user-links-insert <account-id> <web-property-id> <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  analytics3 [options] management profile-user-links-list <account-id> <web-property-id> <profile-id> [-p <v>...] [-o <out>]
  analytics3 [options] management profile-user-links-update <account-id> <web-property-id> <profile-id> <link-id> -r <kv>... [-p <v>...] [-o <out>]
  analytics3 [options] management profiles-delete <account-id> <web-property-id> <profile-id> [-p <v>...]
  analytics3 [options] management profiles-get <account-id> <web-property-id> <profile-id> [-p <v>...] [-o <out>]
  analytics3 [options] management profiles-insert <account-id> <web-property-id> -r <kv>... [-p <v>...] [-o <out>]
  analytics3 [options] management profiles-list <account-id> <web-property-id> [-p <v>...] [-o <out>]
  analytics3 [options] management profiles-patch <account-id> <web-property-id> <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  analytics3 [options] management profiles-update <account-id> <web-property-id> <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  analytics3 [options] management segments-list [-p <v>...] [-o <out>]
  analytics3 [options] management unsampled-reports-get <account-id> <web-property-id> <profile-id> <unsampled-report-id> [-p <v>...] [-o <out>]
  analytics3 [options] management unsampled-reports-insert <account-id> <web-property-id> <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  analytics3 [options] management unsampled-reports-list <account-id> <web-property-id> <profile-id> [-p <v>...] [-o <out>]
  analytics3 [options] management uploads-delete-upload-data <account-id> <web-property-id> <custom-data-source-id> -r <kv>... [-p <v>...]
  analytics3 [options] management uploads-get <account-id> <web-property-id> <custom-data-source-id> <upload-id> [-p <v>...] [-o <out>]
  analytics3 [options] management uploads-list <account-id> <web-property-id> <custom-data-source-id> [-p <v>...] [-o <out>]
  analytics3 [options] management uploads-upload-data <account-id> <web-property-id> <custom-data-source-id> -u (simple|resumable) <file> <mime> [-p <v>...] [-o <out>]
  analytics3 [options] management web-property-ad-words-links-delete <account-id> <web-property-id> <web-property-ad-words-link-id> [-p <v>...]
  analytics3 [options] management web-property-ad-words-links-get <account-id> <web-property-id> <web-property-ad-words-link-id> [-p <v>...] [-o <out>]
  analytics3 [options] management web-property-ad-words-links-insert <account-id> <web-property-id> -r <kv>... [-p <v>...] [-o <out>]
  analytics3 [options] management web-property-ad-words-links-list <account-id> <web-property-id> [-p <v>...] [-o <out>]
  analytics3 [options] management web-property-ad-words-links-patch <account-id> <web-property-id> <web-property-ad-words-link-id> -r <kv>... [-p <v>...] [-o <out>]
  analytics3 [options] management web-property-ad-words-links-update <account-id> <web-property-id> <web-property-ad-words-link-id> -r <kv>... [-p <v>...] [-o <out>]
  analytics3 [options] management webproperties-get <account-id> <web-property-id> [-p <v>...] [-o <out>]
  analytics3 [options] management webproperties-insert <account-id> -r <kv>... [-p <v>...] [-o <out>]
  analytics3 [options] management webproperties-list <account-id> [-p <v>...] [-o <out>]
  analytics3 [options] management webproperties-patch <account-id> <web-property-id> -r <kv>... [-p <v>...] [-o <out>]
  analytics3 [options] management webproperties-update <account-id> <web-property-id> -r <kv>... [-p <v>...] [-o <out>]
  analytics3 [options] management webproperty-user-links-delete <account-id> <web-property-id> <link-id> [-p <v>...]
  analytics3 [options] management webproperty-user-links-insert <account-id> <web-property-id> -r <kv>... [-p <v>...] [-o <out>]
  analytics3 [options] management webproperty-user-links-list <account-id> <web-property-id> [-p <v>...] [-o <out>]
  analytics3 [options] management webproperty-user-links-update <account-id> <web-property-id> <link-id> -r <kv>... [-p <v>...] [-o <out>]
  analytics3 [options] metadata columns-list <report-type> [-p <v>...] [-o <out>]
  analytics3 [options] provisioning create-account-ticket -r <kv>... [-p <v>...] [-o <out>]
  analytics3 --help

All documentation details can be found at
http://byron.github.io/google-apis-rs/google_analytics3_cli/index.html

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
    hub: api::Analytics<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _data_ga_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.data().ga_get(&self.opt.arg_ids, &self.opt.arg_start_date, &self.opt.arg_end_date, &self.opt.arg_metrics);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
                },
                "sort" => {
                    call = call.sort(value.unwrap_or(""));
                },
                "segment" => {
                    call = call.segment(value.unwrap_or(""));
                },
                "sampling-level" => {
                    call = call.sampling_level(value.unwrap_or(""));
                },
                "output" => {
                    call = call.output(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "filters" => {
                    call = call.filters(value.unwrap_or(""));
                },
                "dimensions" => {
                    call = call.dimensions(value.unwrap_or(""));
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

    fn _data_mcf_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.data().mcf_get(&self.opt.arg_ids, &self.opt.arg_start_date, &self.opt.arg_end_date, &self.opt.arg_metrics);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
                },
                "sort" => {
                    call = call.sort(value.unwrap_or(""));
                },
                "sampling-level" => {
                    call = call.sampling_level(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "filters" => {
                    call = call.filters(value.unwrap_or(""));
                },
                "dimensions" => {
                    call = call.dimensions(value.unwrap_or(""));
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

    fn _data_realtime_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.data().realtime_get(&self.opt.arg_ids, &self.opt.arg_metrics);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "sort" => {
                    call = call.sort(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "filters" => {
                    call = call.filters(value.unwrap_or(""));
                },
                "dimensions" => {
                    call = call.dimensions(value.unwrap_or(""));
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

    fn _management_account_summaries_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.management().account_summaries_list();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
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

    fn _management_account_user_links_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.management().account_user_links_delete(&self.opt.arg_account_id, &self.opt.arg_link_id);
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

    fn _management_account_user_links_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::EntityUserLink::default();
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
            fn request_entity_account_ref_init(request: &mut api::EntityUserLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().account_ref.is_none() {
                    request.entity.as_mut().unwrap().account_ref = Some(Default::default());
                }
            }
            
            fn request_entity_init(request: &mut api::EntityUserLink) {
                if request.entity.is_none() {
                    request.entity = Some(Default::default());
                }
            }
            
            fn request_entity_profile_ref_init(request: &mut api::EntityUserLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().profile_ref.is_none() {
                    request.entity.as_mut().unwrap().profile_ref = Some(Default::default());
                }
            }
            
            fn request_entity_web_property_ref_init(request: &mut api::EntityUserLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().web_property_ref.is_none() {
                    request.entity.as_mut().unwrap().web_property_ref = Some(Default::default());
                }
            }
            
            fn request_permissions_init(request: &mut api::EntityUserLink) {
                if request.permissions.is_none() {
                    request.permissions = Some(Default::default());
                }
            }
            
            fn request_user_ref_init(request: &mut api::EntityUserLink) {
                if request.user_ref.is_none() {
                    request.user_ref = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.kind" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.href" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.id" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.name" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.kind" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.name" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.internal-web-property-id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.href" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.web-property-id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.account-id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.kind" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.name" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.internal-web-property-id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.href" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.account-id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "user-ref.kind" => {
                        request_user_ref_init(&mut request);
                        request.user_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "user-ref.email" => {
                        request_user_ref_init(&mut request);
                        request.user_ref.as_mut().unwrap().email = Some(value.unwrap_or("").to_string());
                    },
                "user-ref.id" => {
                        request_user_ref_init(&mut request);
                        request.user_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_user_ref_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_user_ref_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "permissions.local" => {
                        request_permissions_init(&mut request);
                        if request.permissions.as_mut().unwrap().local.is_none() {
                           request.permissions.as_mut().unwrap().local = Some(Default::default());
                        }
                                        request.permissions.as_mut().unwrap().local.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "permissions.effective" => {
                        request_permissions_init(&mut request);
                        if request.permissions.as_mut().unwrap().effective.is_none() {
                           request.permissions.as_mut().unwrap().effective = Some(Default::default());
                        }
                                        request.permissions.as_mut().unwrap().effective.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.management().account_user_links_insert(request, &self.opt.arg_account_id);
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

    fn _management_account_user_links_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.management().account_user_links_list(&self.opt.arg_account_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
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

    fn _management_account_user_links_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::EntityUserLink::default();
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
            fn request_entity_account_ref_init(request: &mut api::EntityUserLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().account_ref.is_none() {
                    request.entity.as_mut().unwrap().account_ref = Some(Default::default());
                }
            }
            
            fn request_entity_init(request: &mut api::EntityUserLink) {
                if request.entity.is_none() {
                    request.entity = Some(Default::default());
                }
            }
            
            fn request_entity_profile_ref_init(request: &mut api::EntityUserLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().profile_ref.is_none() {
                    request.entity.as_mut().unwrap().profile_ref = Some(Default::default());
                }
            }
            
            fn request_entity_web_property_ref_init(request: &mut api::EntityUserLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().web_property_ref.is_none() {
                    request.entity.as_mut().unwrap().web_property_ref = Some(Default::default());
                }
            }
            
            fn request_permissions_init(request: &mut api::EntityUserLink) {
                if request.permissions.is_none() {
                    request.permissions = Some(Default::default());
                }
            }
            
            fn request_user_ref_init(request: &mut api::EntityUserLink) {
                if request.user_ref.is_none() {
                    request.user_ref = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.kind" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.href" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.id" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.name" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.kind" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.name" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.internal-web-property-id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.href" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.web-property-id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.account-id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.kind" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.name" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.internal-web-property-id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.href" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.account-id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "user-ref.kind" => {
                        request_user_ref_init(&mut request);
                        request.user_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "user-ref.email" => {
                        request_user_ref_init(&mut request);
                        request.user_ref.as_mut().unwrap().email = Some(value.unwrap_or("").to_string());
                    },
                "user-ref.id" => {
                        request_user_ref_init(&mut request);
                        request.user_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_user_ref_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_user_ref_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "permissions.local" => {
                        request_permissions_init(&mut request);
                        if request.permissions.as_mut().unwrap().local.is_none() {
                           request.permissions.as_mut().unwrap().local = Some(Default::default());
                        }
                                        request.permissions.as_mut().unwrap().local.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "permissions.effective" => {
                        request_permissions_init(&mut request);
                        if request.permissions.as_mut().unwrap().effective.is_none() {
                           request.permissions.as_mut().unwrap().effective = Some(Default::default());
                        }
                                        request.permissions.as_mut().unwrap().effective.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.management().account_user_links_update(request, &self.opt.arg_account_id, &self.opt.arg_link_id);
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

    fn _management_accounts_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.management().accounts_list();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
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

    fn _management_custom_data_sources_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.management().custom_data_sources_list(&self.opt.arg_account_id, &self.opt.arg_web_property_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
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

    fn _management_custom_dimensions_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.management().custom_dimensions_get(&self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_custom_dimension_id);
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

    fn _management_custom_dimensions_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::CustomDimension::default();
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
            fn request_parent_link_init(request: &mut api::CustomDimension) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "index" => {
                        request.index = Some(arg_from_str(value.unwrap_or("-0"), err, "index", "integer"));
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "web-property-id" => {
                        request.web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "active" => {
                        request.active = Some(arg_from_str(value.unwrap_or("false"), err, "active", "boolean"));
                    },
                "scope" => {
                        request.scope = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_parent_link_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_parent_link_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_parent_link_init(&mut request);
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.management().custom_dimensions_insert(request, &self.opt.arg_account_id, &self.opt.arg_web_property_id);
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

    fn _management_custom_dimensions_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.management().custom_dimensions_list(&self.opt.arg_account_id, &self.opt.arg_web_property_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
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

    fn _management_custom_dimensions_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::CustomDimension::default();
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
            fn request_parent_link_init(request: &mut api::CustomDimension) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "index" => {
                        request.index = Some(arg_from_str(value.unwrap_or("-0"), err, "index", "integer"));
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "web-property-id" => {
                        request.web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "active" => {
                        request.active = Some(arg_from_str(value.unwrap_or("false"), err, "active", "boolean"));
                    },
                "scope" => {
                        request.scope = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_parent_link_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_parent_link_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_parent_link_init(&mut request);
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.management().custom_dimensions_patch(request, &self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_custom_dimension_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "ignore-custom-data-source-links" => {
                    call = call.ignore_custom_data_source_links(arg_from_str(value.unwrap_or("false"), err, "ignore-custom-data-source-links", "boolean"));
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

    fn _management_custom_dimensions_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::CustomDimension::default();
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
            fn request_parent_link_init(request: &mut api::CustomDimension) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "index" => {
                        request.index = Some(arg_from_str(value.unwrap_or("-0"), err, "index", "integer"));
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "web-property-id" => {
                        request.web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "active" => {
                        request.active = Some(arg_from_str(value.unwrap_or("false"), err, "active", "boolean"));
                    },
                "scope" => {
                        request.scope = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_parent_link_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_parent_link_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_parent_link_init(&mut request);
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.management().custom_dimensions_update(request, &self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_custom_dimension_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "ignore-custom-data-source-links" => {
                    call = call.ignore_custom_data_source_links(arg_from_str(value.unwrap_or("false"), err, "ignore-custom-data-source-links", "boolean"));
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

    fn _management_custom_metrics_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.management().custom_metrics_get(&self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_custom_metric_id);
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

    fn _management_custom_metrics_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::CustomMetric::default();
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
            fn request_parent_link_init(request: &mut api::CustomMetric) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "index" => {
                        request.index = Some(arg_from_str(value.unwrap_or("-0"), err, "index", "integer"));
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "max-value" => {
                        request.max_value = Some(value.unwrap_or("").to_string());
                    },
                "min-value" => {
                        request.min_value = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "web-property-id" => {
                        request.web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "active" => {
                        request.active = Some(arg_from_str(value.unwrap_or("false"), err, "active", "boolean"));
                    },
                "scope" => {
                        request.scope = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_parent_link_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_parent_link_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_parent_link_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_parent_link_init(&mut request);
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.management().custom_metrics_insert(request, &self.opt.arg_account_id, &self.opt.arg_web_property_id);
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

    fn _management_custom_metrics_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.management().custom_metrics_list(&self.opt.arg_account_id, &self.opt.arg_web_property_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
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

    fn _management_custom_metrics_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::CustomMetric::default();
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
            fn request_parent_link_init(request: &mut api::CustomMetric) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "index" => {
                        request.index = Some(arg_from_str(value.unwrap_or("-0"), err, "index", "integer"));
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "max-value" => {
                        request.max_value = Some(value.unwrap_or("").to_string());
                    },
                "min-value" => {
                        request.min_value = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "web-property-id" => {
                        request.web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "active" => {
                        request.active = Some(arg_from_str(value.unwrap_or("false"), err, "active", "boolean"));
                    },
                "scope" => {
                        request.scope = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_parent_link_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_parent_link_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_parent_link_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_parent_link_init(&mut request);
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.management().custom_metrics_patch(request, &self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_custom_metric_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "ignore-custom-data-source-links" => {
                    call = call.ignore_custom_data_source_links(arg_from_str(value.unwrap_or("false"), err, "ignore-custom-data-source-links", "boolean"));
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

    fn _management_custom_metrics_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::CustomMetric::default();
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
            fn request_parent_link_init(request: &mut api::CustomMetric) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "index" => {
                        request.index = Some(arg_from_str(value.unwrap_or("-0"), err, "index", "integer"));
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "max-value" => {
                        request.max_value = Some(value.unwrap_or("").to_string());
                    },
                "min-value" => {
                        request.min_value = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "web-property-id" => {
                        request.web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "active" => {
                        request.active = Some(arg_from_str(value.unwrap_or("false"), err, "active", "boolean"));
                    },
                "scope" => {
                        request.scope = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_parent_link_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_parent_link_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_parent_link_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_parent_link_init(&mut request);
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.management().custom_metrics_update(request, &self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_custom_metric_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "ignore-custom-data-source-links" => {
                    call = call.ignore_custom_data_source_links(arg_from_str(value.unwrap_or("false"), err, "ignore-custom-data-source-links", "boolean"));
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

    fn _management_experiments_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.management().experiments_delete(&self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_profile_id, &self.opt.arg_experiment_id);
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

    fn _management_experiments_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.management().experiments_get(&self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_profile_id, &self.opt.arg_experiment_id);
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

    fn _management_experiments_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Experiment::default();
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
            fn request_parent_link_init(request: &mut api::Experiment) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "traffic-coverage" => {
                        request.traffic_coverage = Some(arg_from_str(value.unwrap_or("0.0"), err, "traffic-coverage", "number"));
                    },
                "web-property-id" => {
                        request.web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "optimization-type" => {
                        request.optimization_type = Some(value.unwrap_or("").to_string());
                    },
                "objective-metric" => {
                        request.objective_metric = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "equal-weighting" => {
                        request.equal_weighting = Some(arg_from_str(value.unwrap_or("false"), err, "equal-weighting", "boolean"));
                    },
                "account-id" => {
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet" => {
                        request.snippet = Some(value.unwrap_or("").to_string());
                    },
                "editable-in-ga-ui" => {
                        request.editable_in_ga_ui = Some(arg_from_str(value.unwrap_or("false"), err, "editable-in-ga-ui", "boolean"));
                    },
                "rewrite-variation-urls-as-original" => {
                        request.rewrite_variation_urls_as_original = Some(arg_from_str(value.unwrap_or("false"), err, "rewrite-variation-urls-as-original", "boolean"));
                    },
                "serving-framework" => {
                        request.serving_framework = Some(value.unwrap_or("").to_string());
                    },
                "status" => {
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "internal-web-property-id" => {
                        request.internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "winner-confidence-level" => {
                        request.winner_confidence_level = Some(arg_from_str(value.unwrap_or("0.0"), err, "winner-confidence-level", "number"));
                    },
                "start-time" => {
                        request.start_time = Some(value.unwrap_or("").to_string());
                    },
                "winner-found" => {
                        request.winner_found = Some(arg_from_str(value.unwrap_or("false"), err, "winner-found", "boolean"));
                    },
                "reason-experiment-ended" => {
                        request.reason_experiment_ended = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "minimum-experiment-length-in-days" => {
                        request.minimum_experiment_length_in_days = Some(arg_from_str(value.unwrap_or("-0"), err, "minimum-experiment-length-in-days", "integer"));
                    },
                "profile-id" => {
                        request.profile_id = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "end-time" => {
                        request_parent_link_init(&mut request);
                        request.end_time = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_parent_link_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.management().experiments_insert(request, &self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_profile_id);
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

    fn _management_experiments_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.management().experiments_list(&self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_profile_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
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

    fn _management_experiments_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Experiment::default();
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
            fn request_parent_link_init(request: &mut api::Experiment) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "traffic-coverage" => {
                        request.traffic_coverage = Some(arg_from_str(value.unwrap_or("0.0"), err, "traffic-coverage", "number"));
                    },
                "web-property-id" => {
                        request.web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "optimization-type" => {
                        request.optimization_type = Some(value.unwrap_or("").to_string());
                    },
                "objective-metric" => {
                        request.objective_metric = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "equal-weighting" => {
                        request.equal_weighting = Some(arg_from_str(value.unwrap_or("false"), err, "equal-weighting", "boolean"));
                    },
                "account-id" => {
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet" => {
                        request.snippet = Some(value.unwrap_or("").to_string());
                    },
                "editable-in-ga-ui" => {
                        request.editable_in_ga_ui = Some(arg_from_str(value.unwrap_or("false"), err, "editable-in-ga-ui", "boolean"));
                    },
                "rewrite-variation-urls-as-original" => {
                        request.rewrite_variation_urls_as_original = Some(arg_from_str(value.unwrap_or("false"), err, "rewrite-variation-urls-as-original", "boolean"));
                    },
                "serving-framework" => {
                        request.serving_framework = Some(value.unwrap_or("").to_string());
                    },
                "status" => {
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "internal-web-property-id" => {
                        request.internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "winner-confidence-level" => {
                        request.winner_confidence_level = Some(arg_from_str(value.unwrap_or("0.0"), err, "winner-confidence-level", "number"));
                    },
                "start-time" => {
                        request.start_time = Some(value.unwrap_or("").to_string());
                    },
                "winner-found" => {
                        request.winner_found = Some(arg_from_str(value.unwrap_or("false"), err, "winner-found", "boolean"));
                    },
                "reason-experiment-ended" => {
                        request.reason_experiment_ended = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "minimum-experiment-length-in-days" => {
                        request.minimum_experiment_length_in_days = Some(arg_from_str(value.unwrap_or("-0"), err, "minimum-experiment-length-in-days", "integer"));
                    },
                "profile-id" => {
                        request.profile_id = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "end-time" => {
                        request_parent_link_init(&mut request);
                        request.end_time = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_parent_link_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.management().experiments_patch(request, &self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_profile_id, &self.opt.arg_experiment_id);
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

    fn _management_experiments_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Experiment::default();
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
            fn request_parent_link_init(request: &mut api::Experiment) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "traffic-coverage" => {
                        request.traffic_coverage = Some(arg_from_str(value.unwrap_or("0.0"), err, "traffic-coverage", "number"));
                    },
                "web-property-id" => {
                        request.web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "optimization-type" => {
                        request.optimization_type = Some(value.unwrap_or("").to_string());
                    },
                "objective-metric" => {
                        request.objective_metric = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "equal-weighting" => {
                        request.equal_weighting = Some(arg_from_str(value.unwrap_or("false"), err, "equal-weighting", "boolean"));
                    },
                "account-id" => {
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                "snippet" => {
                        request.snippet = Some(value.unwrap_or("").to_string());
                    },
                "editable-in-ga-ui" => {
                        request.editable_in_ga_ui = Some(arg_from_str(value.unwrap_or("false"), err, "editable-in-ga-ui", "boolean"));
                    },
                "rewrite-variation-urls-as-original" => {
                        request.rewrite_variation_urls_as_original = Some(arg_from_str(value.unwrap_or("false"), err, "rewrite-variation-urls-as-original", "boolean"));
                    },
                "serving-framework" => {
                        request.serving_framework = Some(value.unwrap_or("").to_string());
                    },
                "status" => {
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "internal-web-property-id" => {
                        request.internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "winner-confidence-level" => {
                        request.winner_confidence_level = Some(arg_from_str(value.unwrap_or("0.0"), err, "winner-confidence-level", "number"));
                    },
                "start-time" => {
                        request.start_time = Some(value.unwrap_or("").to_string());
                    },
                "winner-found" => {
                        request.winner_found = Some(arg_from_str(value.unwrap_or("false"), err, "winner-found", "boolean"));
                    },
                "reason-experiment-ended" => {
                        request.reason_experiment_ended = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "minimum-experiment-length-in-days" => {
                        request.minimum_experiment_length_in_days = Some(arg_from_str(value.unwrap_or("-0"), err, "minimum-experiment-length-in-days", "integer"));
                    },
                "profile-id" => {
                        request.profile_id = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "end-time" => {
                        request_parent_link_init(&mut request);
                        request.end_time = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_parent_link_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.management().experiments_update(request, &self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_profile_id, &self.opt.arg_experiment_id);
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

    fn _management_filters_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.management().filters_delete(&self.opt.arg_account_id, &self.opt.arg_filter_id);
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

    fn _management_filters_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.management().filters_get(&self.opt.arg_account_id, &self.opt.arg_filter_id);
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

    fn _management_filters_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Filter::default();
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
            fn request_advanced_details_init(request: &mut api::Filter) {
                if request.advanced_details.is_none() {
                    request.advanced_details = Some(Default::default());
                }
            }
            
            fn request_exclude_details_init(request: &mut api::Filter) {
                if request.exclude_details.is_none() {
                    request.exclude_details = Some(Default::default());
                }
            }
            
            fn request_include_details_init(request: &mut api::Filter) {
                if request.include_details.is_none() {
                    request.include_details = Some(Default::default());
                }
            }
            
            fn request_lowercase_details_init(request: &mut api::Filter) {
                if request.lowercase_details.is_none() {
                    request.lowercase_details = Some(Default::default());
                }
            }
            
            fn request_parent_link_init(request: &mut api::Filter) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            fn request_search_and_replace_details_init(request: &mut api::Filter) {
                if request.search_and_replace_details.is_none() {
                    request.search_and_replace_details = Some(Default::default());
                }
            }
            
            fn request_uppercase_details_init(request: &mut api::Filter) {
                if request.uppercase_details.is_none() {
                    request.uppercase_details = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "uppercase-details.field" => {
                        request_uppercase_details_init(&mut request);
                        request.uppercase_details.as_mut().unwrap().field = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request_uppercase_details_init(&mut request);
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "advanced-details.extract-b" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().extract_b = Some(value.unwrap_or("").to_string());
                    },
                "advanced-details.override-output-field" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().override_output_field = Some(arg_from_str(value.unwrap_or("false"), err, "advanced-details.override-output-field", "boolean"));
                    },
                "advanced-details.field-a-required" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().field_a_required = Some(arg_from_str(value.unwrap_or("false"), err, "advanced-details.field-a-required", "boolean"));
                    },
                "advanced-details.output-constructor" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().output_constructor = Some(value.unwrap_or("").to_string());
                    },
                "advanced-details.field-b-required" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().field_b_required = Some(arg_from_str(value.unwrap_or("false"), err, "advanced-details.field-b-required", "boolean"));
                    },
                "advanced-details.case-sensitive" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().case_sensitive = Some(arg_from_str(value.unwrap_or("false"), err, "advanced-details.case-sensitive", "boolean"));
                    },
                "advanced-details.field-b" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().field_b = Some(value.unwrap_or("").to_string());
                    },
                "advanced-details.field-a" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().field_a = Some(value.unwrap_or("").to_string());
                    },
                "advanced-details.extract-a" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().extract_a = Some(value.unwrap_or("").to_string());
                    },
                "advanced-details.output-to-field" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().output_to_field = Some(value.unwrap_or("").to_string());
                    },
                "lowercase-details.field" => {
                        request_lowercase_details_init(&mut request);
                        request.lowercase_details.as_mut().unwrap().field = Some(value.unwrap_or("").to_string());
                    },
                "search-and-replace-details.case-sensitive" => {
                        request_search_and_replace_details_init(&mut request);
                        request.search_and_replace_details.as_mut().unwrap().case_sensitive = Some(arg_from_str(value.unwrap_or("false"), err, "search-and-replace-details.case-sensitive", "boolean"));
                    },
                "search-and-replace-details.search-string" => {
                        request_search_and_replace_details_init(&mut request);
                        request.search_and_replace_details.as_mut().unwrap().search_string = Some(value.unwrap_or("").to_string());
                    },
                "search-and-replace-details.replace-string" => {
                        request_search_and_replace_details_init(&mut request);
                        request.search_and_replace_details.as_mut().unwrap().replace_string = Some(value.unwrap_or("").to_string());
                    },
                "search-and-replace-details.field" => {
                        request_search_and_replace_details_init(&mut request);
                        request.search_and_replace_details.as_mut().unwrap().field = Some(value.unwrap_or("").to_string());
                    },
                "exclude-details.case-sensitive" => {
                        request_exclude_details_init(&mut request);
                        request.exclude_details.as_mut().unwrap().case_sensitive = Some(arg_from_str(value.unwrap_or("false"), err, "exclude-details.case-sensitive", "boolean"));
                    },
                "exclude-details.kind" => {
                        request_exclude_details_init(&mut request);
                        request.exclude_details.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "exclude-details.match-type" => {
                        request_exclude_details_init(&mut request);
                        request.exclude_details.as_mut().unwrap().match_type = Some(value.unwrap_or("").to_string());
                    },
                "exclude-details.expression-value" => {
                        request_exclude_details_init(&mut request);
                        request.exclude_details.as_mut().unwrap().expression_value = Some(value.unwrap_or("").to_string());
                    },
                "exclude-details.field" => {
                        request_exclude_details_init(&mut request);
                        request.exclude_details.as_mut().unwrap().field = Some(value.unwrap_or("").to_string());
                    },
                "include-details.case-sensitive" => {
                        request_include_details_init(&mut request);
                        request.include_details.as_mut().unwrap().case_sensitive = Some(arg_from_str(value.unwrap_or("false"), err, "include-details.case-sensitive", "boolean"));
                    },
                "include-details.kind" => {
                        request_include_details_init(&mut request);
                        request.include_details.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "include-details.match-type" => {
                        request_include_details_init(&mut request);
                        request.include_details.as_mut().unwrap().match_type = Some(value.unwrap_or("").to_string());
                    },
                "include-details.expression-value" => {
                        request_include_details_init(&mut request);
                        request.include_details.as_mut().unwrap().expression_value = Some(value.unwrap_or("").to_string());
                    },
                "include-details.field" => {
                        request_include_details_init(&mut request);
                        request.include_details.as_mut().unwrap().field = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_parent_link_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_parent_link_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_parent_link_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_parent_link_init(&mut request);
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.management().filters_insert(request, &self.opt.arg_account_id);
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

    fn _management_filters_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.management().filters_list(&self.opt.arg_account_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
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

    fn _management_filters_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Filter::default();
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
            fn request_advanced_details_init(request: &mut api::Filter) {
                if request.advanced_details.is_none() {
                    request.advanced_details = Some(Default::default());
                }
            }
            
            fn request_exclude_details_init(request: &mut api::Filter) {
                if request.exclude_details.is_none() {
                    request.exclude_details = Some(Default::default());
                }
            }
            
            fn request_include_details_init(request: &mut api::Filter) {
                if request.include_details.is_none() {
                    request.include_details = Some(Default::default());
                }
            }
            
            fn request_lowercase_details_init(request: &mut api::Filter) {
                if request.lowercase_details.is_none() {
                    request.lowercase_details = Some(Default::default());
                }
            }
            
            fn request_parent_link_init(request: &mut api::Filter) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            fn request_search_and_replace_details_init(request: &mut api::Filter) {
                if request.search_and_replace_details.is_none() {
                    request.search_and_replace_details = Some(Default::default());
                }
            }
            
            fn request_uppercase_details_init(request: &mut api::Filter) {
                if request.uppercase_details.is_none() {
                    request.uppercase_details = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "uppercase-details.field" => {
                        request_uppercase_details_init(&mut request);
                        request.uppercase_details.as_mut().unwrap().field = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request_uppercase_details_init(&mut request);
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "advanced-details.extract-b" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().extract_b = Some(value.unwrap_or("").to_string());
                    },
                "advanced-details.override-output-field" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().override_output_field = Some(arg_from_str(value.unwrap_or("false"), err, "advanced-details.override-output-field", "boolean"));
                    },
                "advanced-details.field-a-required" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().field_a_required = Some(arg_from_str(value.unwrap_or("false"), err, "advanced-details.field-a-required", "boolean"));
                    },
                "advanced-details.output-constructor" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().output_constructor = Some(value.unwrap_or("").to_string());
                    },
                "advanced-details.field-b-required" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().field_b_required = Some(arg_from_str(value.unwrap_or("false"), err, "advanced-details.field-b-required", "boolean"));
                    },
                "advanced-details.case-sensitive" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().case_sensitive = Some(arg_from_str(value.unwrap_or("false"), err, "advanced-details.case-sensitive", "boolean"));
                    },
                "advanced-details.field-b" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().field_b = Some(value.unwrap_or("").to_string());
                    },
                "advanced-details.field-a" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().field_a = Some(value.unwrap_or("").to_string());
                    },
                "advanced-details.extract-a" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().extract_a = Some(value.unwrap_or("").to_string());
                    },
                "advanced-details.output-to-field" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().output_to_field = Some(value.unwrap_or("").to_string());
                    },
                "lowercase-details.field" => {
                        request_lowercase_details_init(&mut request);
                        request.lowercase_details.as_mut().unwrap().field = Some(value.unwrap_or("").to_string());
                    },
                "search-and-replace-details.case-sensitive" => {
                        request_search_and_replace_details_init(&mut request);
                        request.search_and_replace_details.as_mut().unwrap().case_sensitive = Some(arg_from_str(value.unwrap_or("false"), err, "search-and-replace-details.case-sensitive", "boolean"));
                    },
                "search-and-replace-details.search-string" => {
                        request_search_and_replace_details_init(&mut request);
                        request.search_and_replace_details.as_mut().unwrap().search_string = Some(value.unwrap_or("").to_string());
                    },
                "search-and-replace-details.replace-string" => {
                        request_search_and_replace_details_init(&mut request);
                        request.search_and_replace_details.as_mut().unwrap().replace_string = Some(value.unwrap_or("").to_string());
                    },
                "search-and-replace-details.field" => {
                        request_search_and_replace_details_init(&mut request);
                        request.search_and_replace_details.as_mut().unwrap().field = Some(value.unwrap_or("").to_string());
                    },
                "exclude-details.case-sensitive" => {
                        request_exclude_details_init(&mut request);
                        request.exclude_details.as_mut().unwrap().case_sensitive = Some(arg_from_str(value.unwrap_or("false"), err, "exclude-details.case-sensitive", "boolean"));
                    },
                "exclude-details.kind" => {
                        request_exclude_details_init(&mut request);
                        request.exclude_details.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "exclude-details.match-type" => {
                        request_exclude_details_init(&mut request);
                        request.exclude_details.as_mut().unwrap().match_type = Some(value.unwrap_or("").to_string());
                    },
                "exclude-details.expression-value" => {
                        request_exclude_details_init(&mut request);
                        request.exclude_details.as_mut().unwrap().expression_value = Some(value.unwrap_or("").to_string());
                    },
                "exclude-details.field" => {
                        request_exclude_details_init(&mut request);
                        request.exclude_details.as_mut().unwrap().field = Some(value.unwrap_or("").to_string());
                    },
                "include-details.case-sensitive" => {
                        request_include_details_init(&mut request);
                        request.include_details.as_mut().unwrap().case_sensitive = Some(arg_from_str(value.unwrap_or("false"), err, "include-details.case-sensitive", "boolean"));
                    },
                "include-details.kind" => {
                        request_include_details_init(&mut request);
                        request.include_details.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "include-details.match-type" => {
                        request_include_details_init(&mut request);
                        request.include_details.as_mut().unwrap().match_type = Some(value.unwrap_or("").to_string());
                    },
                "include-details.expression-value" => {
                        request_include_details_init(&mut request);
                        request.include_details.as_mut().unwrap().expression_value = Some(value.unwrap_or("").to_string());
                    },
                "include-details.field" => {
                        request_include_details_init(&mut request);
                        request.include_details.as_mut().unwrap().field = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_parent_link_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_parent_link_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_parent_link_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_parent_link_init(&mut request);
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.management().filters_patch(request, &self.opt.arg_account_id, &self.opt.arg_filter_id);
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

    fn _management_filters_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Filter::default();
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
            fn request_advanced_details_init(request: &mut api::Filter) {
                if request.advanced_details.is_none() {
                    request.advanced_details = Some(Default::default());
                }
            }
            
            fn request_exclude_details_init(request: &mut api::Filter) {
                if request.exclude_details.is_none() {
                    request.exclude_details = Some(Default::default());
                }
            }
            
            fn request_include_details_init(request: &mut api::Filter) {
                if request.include_details.is_none() {
                    request.include_details = Some(Default::default());
                }
            }
            
            fn request_lowercase_details_init(request: &mut api::Filter) {
                if request.lowercase_details.is_none() {
                    request.lowercase_details = Some(Default::default());
                }
            }
            
            fn request_parent_link_init(request: &mut api::Filter) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            fn request_search_and_replace_details_init(request: &mut api::Filter) {
                if request.search_and_replace_details.is_none() {
                    request.search_and_replace_details = Some(Default::default());
                }
            }
            
            fn request_uppercase_details_init(request: &mut api::Filter) {
                if request.uppercase_details.is_none() {
                    request.uppercase_details = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "uppercase-details.field" => {
                        request_uppercase_details_init(&mut request);
                        request.uppercase_details.as_mut().unwrap().field = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request_uppercase_details_init(&mut request);
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "advanced-details.extract-b" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().extract_b = Some(value.unwrap_or("").to_string());
                    },
                "advanced-details.override-output-field" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().override_output_field = Some(arg_from_str(value.unwrap_or("false"), err, "advanced-details.override-output-field", "boolean"));
                    },
                "advanced-details.field-a-required" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().field_a_required = Some(arg_from_str(value.unwrap_or("false"), err, "advanced-details.field-a-required", "boolean"));
                    },
                "advanced-details.output-constructor" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().output_constructor = Some(value.unwrap_or("").to_string());
                    },
                "advanced-details.field-b-required" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().field_b_required = Some(arg_from_str(value.unwrap_or("false"), err, "advanced-details.field-b-required", "boolean"));
                    },
                "advanced-details.case-sensitive" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().case_sensitive = Some(arg_from_str(value.unwrap_or("false"), err, "advanced-details.case-sensitive", "boolean"));
                    },
                "advanced-details.field-b" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().field_b = Some(value.unwrap_or("").to_string());
                    },
                "advanced-details.field-a" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().field_a = Some(value.unwrap_or("").to_string());
                    },
                "advanced-details.extract-a" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().extract_a = Some(value.unwrap_or("").to_string());
                    },
                "advanced-details.output-to-field" => {
                        request_advanced_details_init(&mut request);
                        request.advanced_details.as_mut().unwrap().output_to_field = Some(value.unwrap_or("").to_string());
                    },
                "lowercase-details.field" => {
                        request_lowercase_details_init(&mut request);
                        request.lowercase_details.as_mut().unwrap().field = Some(value.unwrap_or("").to_string());
                    },
                "search-and-replace-details.case-sensitive" => {
                        request_search_and_replace_details_init(&mut request);
                        request.search_and_replace_details.as_mut().unwrap().case_sensitive = Some(arg_from_str(value.unwrap_or("false"), err, "search-and-replace-details.case-sensitive", "boolean"));
                    },
                "search-and-replace-details.search-string" => {
                        request_search_and_replace_details_init(&mut request);
                        request.search_and_replace_details.as_mut().unwrap().search_string = Some(value.unwrap_or("").to_string());
                    },
                "search-and-replace-details.replace-string" => {
                        request_search_and_replace_details_init(&mut request);
                        request.search_and_replace_details.as_mut().unwrap().replace_string = Some(value.unwrap_or("").to_string());
                    },
                "search-and-replace-details.field" => {
                        request_search_and_replace_details_init(&mut request);
                        request.search_and_replace_details.as_mut().unwrap().field = Some(value.unwrap_or("").to_string());
                    },
                "exclude-details.case-sensitive" => {
                        request_exclude_details_init(&mut request);
                        request.exclude_details.as_mut().unwrap().case_sensitive = Some(arg_from_str(value.unwrap_or("false"), err, "exclude-details.case-sensitive", "boolean"));
                    },
                "exclude-details.kind" => {
                        request_exclude_details_init(&mut request);
                        request.exclude_details.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "exclude-details.match-type" => {
                        request_exclude_details_init(&mut request);
                        request.exclude_details.as_mut().unwrap().match_type = Some(value.unwrap_or("").to_string());
                    },
                "exclude-details.expression-value" => {
                        request_exclude_details_init(&mut request);
                        request.exclude_details.as_mut().unwrap().expression_value = Some(value.unwrap_or("").to_string());
                    },
                "exclude-details.field" => {
                        request_exclude_details_init(&mut request);
                        request.exclude_details.as_mut().unwrap().field = Some(value.unwrap_or("").to_string());
                    },
                "include-details.case-sensitive" => {
                        request_include_details_init(&mut request);
                        request.include_details.as_mut().unwrap().case_sensitive = Some(arg_from_str(value.unwrap_or("false"), err, "include-details.case-sensitive", "boolean"));
                    },
                "include-details.kind" => {
                        request_include_details_init(&mut request);
                        request.include_details.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "include-details.match-type" => {
                        request_include_details_init(&mut request);
                        request.include_details.as_mut().unwrap().match_type = Some(value.unwrap_or("").to_string());
                    },
                "include-details.expression-value" => {
                        request_include_details_init(&mut request);
                        request.include_details.as_mut().unwrap().expression_value = Some(value.unwrap_or("").to_string());
                    },
                "include-details.field" => {
                        request_include_details_init(&mut request);
                        request.include_details.as_mut().unwrap().field = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_parent_link_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_parent_link_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_parent_link_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_parent_link_init(&mut request);
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.management().filters_update(request, &self.opt.arg_account_id, &self.opt.arg_filter_id);
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

    fn _management_goals_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.management().goals_get(&self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_profile_id, &self.opt.arg_goal_id);
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

    fn _management_goals_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Goal::default();
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
            fn request_event_details_init(request: &mut api::Goal) {
                if request.event_details.is_none() {
                    request.event_details = Some(Default::default());
                }
            }
            
            fn request_parent_link_init(request: &mut api::Goal) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            fn request_url_destination_details_init(request: &mut api::Goal) {
                if request.url_destination_details.is_none() {
                    request.url_destination_details = Some(Default::default());
                }
            }
            
            fn request_visit_num_pages_details_init(request: &mut api::Goal) {
                if request.visit_num_pages_details.is_none() {
                    request.visit_num_pages_details = Some(Default::default());
                }
            }
            
            fn request_visit_time_on_site_details_init(request: &mut api::Goal) {
                if request.visit_time_on_site_details.is_none() {
                    request.visit_time_on_site_details = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "visit-time-on-site-details.comparison-type" => {
                        request_visit_time_on_site_details_init(&mut request);
                        request.visit_time_on_site_details.as_mut().unwrap().comparison_type = Some(value.unwrap_or("").to_string());
                    },
                "visit-time-on-site-details.comparison-value" => {
                        request_visit_time_on_site_details_init(&mut request);
                        request.visit_time_on_site_details.as_mut().unwrap().comparison_value = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_visit_time_on_site_details_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request_visit_time_on_site_details_init(&mut request);
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "url-destination-details.url" => {
                        request_url_destination_details_init(&mut request);
                        request.url_destination_details.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "url-destination-details.case-sensitive" => {
                        request_url_destination_details_init(&mut request);
                        request.url_destination_details.as_mut().unwrap().case_sensitive = Some(arg_from_str(value.unwrap_or("false"), err, "url-destination-details.case-sensitive", "boolean"));
                    },
                "url-destination-details.match-type" => {
                        request_url_destination_details_init(&mut request);
                        request.url_destination_details.as_mut().unwrap().match_type = Some(value.unwrap_or("").to_string());
                    },
                "url-destination-details.first-step-required" => {
                        request_url_destination_details_init(&mut request);
                        request.url_destination_details.as_mut().unwrap().first_step_required = Some(arg_from_str(value.unwrap_or("false"), err, "url-destination-details.first-step-required", "boolean"));
                    },
                "kind" => {
                        request_url_destination_details_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "value" => {
                        request_url_destination_details_init(&mut request);
                        request.value = Some(arg_from_str(value.unwrap_or("0.0"), err, "value", "number"));
                    },
                "visit-num-pages-details.comparison-type" => {
                        request_visit_num_pages_details_init(&mut request);
                        request.visit_num_pages_details.as_mut().unwrap().comparison_type = Some(value.unwrap_or("").to_string());
                    },
                "visit-num-pages-details.comparison-value" => {
                        request_visit_num_pages_details_init(&mut request);
                        request.visit_num_pages_details.as_mut().unwrap().comparison_value = Some(value.unwrap_or("").to_string());
                    },
                "internal-web-property-id" => {
                        request_visit_num_pages_details_init(&mut request);
                        request.internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "event-details.use-event-value" => {
                        request_event_details_init(&mut request);
                        request.event_details.as_mut().unwrap().use_event_value = Some(arg_from_str(value.unwrap_or("false"), err, "event-details.use-event-value", "boolean"));
                    },
                "web-property-id" => {
                        request_event_details_init(&mut request);
                        request.web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "active" => {
                        request_event_details_init(&mut request);
                        request.active = Some(arg_from_str(value.unwrap_or("false"), err, "active", "boolean"));
                    },
                "profile-id" => {
                        request_event_details_init(&mut request);
                        request.profile_id = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_parent_link_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_parent_link_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_parent_link_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_parent_link_init(&mut request);
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.management().goals_insert(request, &self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_profile_id);
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

    fn _management_goals_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.management().goals_list(&self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_profile_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
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

    fn _management_goals_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Goal::default();
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
            fn request_event_details_init(request: &mut api::Goal) {
                if request.event_details.is_none() {
                    request.event_details = Some(Default::default());
                }
            }
            
            fn request_parent_link_init(request: &mut api::Goal) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            fn request_url_destination_details_init(request: &mut api::Goal) {
                if request.url_destination_details.is_none() {
                    request.url_destination_details = Some(Default::default());
                }
            }
            
            fn request_visit_num_pages_details_init(request: &mut api::Goal) {
                if request.visit_num_pages_details.is_none() {
                    request.visit_num_pages_details = Some(Default::default());
                }
            }
            
            fn request_visit_time_on_site_details_init(request: &mut api::Goal) {
                if request.visit_time_on_site_details.is_none() {
                    request.visit_time_on_site_details = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "visit-time-on-site-details.comparison-type" => {
                        request_visit_time_on_site_details_init(&mut request);
                        request.visit_time_on_site_details.as_mut().unwrap().comparison_type = Some(value.unwrap_or("").to_string());
                    },
                "visit-time-on-site-details.comparison-value" => {
                        request_visit_time_on_site_details_init(&mut request);
                        request.visit_time_on_site_details.as_mut().unwrap().comparison_value = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_visit_time_on_site_details_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request_visit_time_on_site_details_init(&mut request);
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "url-destination-details.url" => {
                        request_url_destination_details_init(&mut request);
                        request.url_destination_details.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "url-destination-details.case-sensitive" => {
                        request_url_destination_details_init(&mut request);
                        request.url_destination_details.as_mut().unwrap().case_sensitive = Some(arg_from_str(value.unwrap_or("false"), err, "url-destination-details.case-sensitive", "boolean"));
                    },
                "url-destination-details.match-type" => {
                        request_url_destination_details_init(&mut request);
                        request.url_destination_details.as_mut().unwrap().match_type = Some(value.unwrap_or("").to_string());
                    },
                "url-destination-details.first-step-required" => {
                        request_url_destination_details_init(&mut request);
                        request.url_destination_details.as_mut().unwrap().first_step_required = Some(arg_from_str(value.unwrap_or("false"), err, "url-destination-details.first-step-required", "boolean"));
                    },
                "kind" => {
                        request_url_destination_details_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "value" => {
                        request_url_destination_details_init(&mut request);
                        request.value = Some(arg_from_str(value.unwrap_or("0.0"), err, "value", "number"));
                    },
                "visit-num-pages-details.comparison-type" => {
                        request_visit_num_pages_details_init(&mut request);
                        request.visit_num_pages_details.as_mut().unwrap().comparison_type = Some(value.unwrap_or("").to_string());
                    },
                "visit-num-pages-details.comparison-value" => {
                        request_visit_num_pages_details_init(&mut request);
                        request.visit_num_pages_details.as_mut().unwrap().comparison_value = Some(value.unwrap_or("").to_string());
                    },
                "internal-web-property-id" => {
                        request_visit_num_pages_details_init(&mut request);
                        request.internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "event-details.use-event-value" => {
                        request_event_details_init(&mut request);
                        request.event_details.as_mut().unwrap().use_event_value = Some(arg_from_str(value.unwrap_or("false"), err, "event-details.use-event-value", "boolean"));
                    },
                "web-property-id" => {
                        request_event_details_init(&mut request);
                        request.web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "active" => {
                        request_event_details_init(&mut request);
                        request.active = Some(arg_from_str(value.unwrap_or("false"), err, "active", "boolean"));
                    },
                "profile-id" => {
                        request_event_details_init(&mut request);
                        request.profile_id = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_parent_link_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_parent_link_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_parent_link_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_parent_link_init(&mut request);
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.management().goals_patch(request, &self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_profile_id, &self.opt.arg_goal_id);
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

    fn _management_goals_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Goal::default();
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
            fn request_event_details_init(request: &mut api::Goal) {
                if request.event_details.is_none() {
                    request.event_details = Some(Default::default());
                }
            }
            
            fn request_parent_link_init(request: &mut api::Goal) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            fn request_url_destination_details_init(request: &mut api::Goal) {
                if request.url_destination_details.is_none() {
                    request.url_destination_details = Some(Default::default());
                }
            }
            
            fn request_visit_num_pages_details_init(request: &mut api::Goal) {
                if request.visit_num_pages_details.is_none() {
                    request.visit_num_pages_details = Some(Default::default());
                }
            }
            
            fn request_visit_time_on_site_details_init(request: &mut api::Goal) {
                if request.visit_time_on_site_details.is_none() {
                    request.visit_time_on_site_details = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "visit-time-on-site-details.comparison-type" => {
                        request_visit_time_on_site_details_init(&mut request);
                        request.visit_time_on_site_details.as_mut().unwrap().comparison_type = Some(value.unwrap_or("").to_string());
                    },
                "visit-time-on-site-details.comparison-value" => {
                        request_visit_time_on_site_details_init(&mut request);
                        request.visit_time_on_site_details.as_mut().unwrap().comparison_value = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_visit_time_on_site_details_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request_visit_time_on_site_details_init(&mut request);
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "url-destination-details.url" => {
                        request_url_destination_details_init(&mut request);
                        request.url_destination_details.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "url-destination-details.case-sensitive" => {
                        request_url_destination_details_init(&mut request);
                        request.url_destination_details.as_mut().unwrap().case_sensitive = Some(arg_from_str(value.unwrap_or("false"), err, "url-destination-details.case-sensitive", "boolean"));
                    },
                "url-destination-details.match-type" => {
                        request_url_destination_details_init(&mut request);
                        request.url_destination_details.as_mut().unwrap().match_type = Some(value.unwrap_or("").to_string());
                    },
                "url-destination-details.first-step-required" => {
                        request_url_destination_details_init(&mut request);
                        request.url_destination_details.as_mut().unwrap().first_step_required = Some(arg_from_str(value.unwrap_or("false"), err, "url-destination-details.first-step-required", "boolean"));
                    },
                "kind" => {
                        request_url_destination_details_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "value" => {
                        request_url_destination_details_init(&mut request);
                        request.value = Some(arg_from_str(value.unwrap_or("0.0"), err, "value", "number"));
                    },
                "visit-num-pages-details.comparison-type" => {
                        request_visit_num_pages_details_init(&mut request);
                        request.visit_num_pages_details.as_mut().unwrap().comparison_type = Some(value.unwrap_or("").to_string());
                    },
                "visit-num-pages-details.comparison-value" => {
                        request_visit_num_pages_details_init(&mut request);
                        request.visit_num_pages_details.as_mut().unwrap().comparison_value = Some(value.unwrap_or("").to_string());
                    },
                "internal-web-property-id" => {
                        request_visit_num_pages_details_init(&mut request);
                        request.internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "event-details.use-event-value" => {
                        request_event_details_init(&mut request);
                        request.event_details.as_mut().unwrap().use_event_value = Some(arg_from_str(value.unwrap_or("false"), err, "event-details.use-event-value", "boolean"));
                    },
                "web-property-id" => {
                        request_event_details_init(&mut request);
                        request.web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "active" => {
                        request_event_details_init(&mut request);
                        request.active = Some(arg_from_str(value.unwrap_or("false"), err, "active", "boolean"));
                    },
                "profile-id" => {
                        request_event_details_init(&mut request);
                        request.profile_id = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_parent_link_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_parent_link_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_parent_link_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_parent_link_init(&mut request);
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.management().goals_update(request, &self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_profile_id, &self.opt.arg_goal_id);
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

    fn _management_profile_filter_links_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.management().profile_filter_links_delete(&self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_profile_id, &self.opt.arg_link_id);
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

    fn _management_profile_filter_links_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.management().profile_filter_links_get(&self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_profile_id, &self.opt.arg_link_id);
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

    fn _management_profile_filter_links_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::ProfileFilterLink::default();
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
            fn request_filter_ref_init(request: &mut api::ProfileFilterLink) {
                if request.filter_ref.is_none() {
                    request.filter_ref = Some(Default::default());
                }
            }
            
            fn request_profile_ref_init(request: &mut api::ProfileFilterLink) {
                if request.profile_ref.is_none() {
                    request.profile_ref = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "rank" => {
                        request.rank = Some(arg_from_str(value.unwrap_or("-0"), err, "rank", "integer"));
                    },
                "filter-ref.kind" => {
                        request_filter_ref_init(&mut request);
                        request.filter_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "filter-ref.href" => {
                        request_filter_ref_init(&mut request);
                        request.filter_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "filter-ref.id" => {
                        request_filter_ref_init(&mut request);
                        request.filter_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "filter-ref.name" => {
                        request_filter_ref_init(&mut request);
                        request.filter_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "filter-ref.account-id" => {
                        request_filter_ref_init(&mut request);
                        request.filter_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.kind" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.name" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.internal-web-property-id" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.href" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.web-property-id" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.id" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.account-id" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_profile_ref_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_profile_ref_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.management().profile_filter_links_insert(request, &self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_profile_id);
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

    fn _management_profile_filter_links_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.management().profile_filter_links_list(&self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_profile_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
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

    fn _management_profile_filter_links_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::ProfileFilterLink::default();
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
            fn request_filter_ref_init(request: &mut api::ProfileFilterLink) {
                if request.filter_ref.is_none() {
                    request.filter_ref = Some(Default::default());
                }
            }
            
            fn request_profile_ref_init(request: &mut api::ProfileFilterLink) {
                if request.profile_ref.is_none() {
                    request.profile_ref = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "rank" => {
                        request.rank = Some(arg_from_str(value.unwrap_or("-0"), err, "rank", "integer"));
                    },
                "filter-ref.kind" => {
                        request_filter_ref_init(&mut request);
                        request.filter_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "filter-ref.href" => {
                        request_filter_ref_init(&mut request);
                        request.filter_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "filter-ref.id" => {
                        request_filter_ref_init(&mut request);
                        request.filter_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "filter-ref.name" => {
                        request_filter_ref_init(&mut request);
                        request.filter_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "filter-ref.account-id" => {
                        request_filter_ref_init(&mut request);
                        request.filter_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.kind" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.name" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.internal-web-property-id" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.href" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.web-property-id" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.id" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.account-id" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_profile_ref_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_profile_ref_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.management().profile_filter_links_patch(request, &self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_profile_id, &self.opt.arg_link_id);
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

    fn _management_profile_filter_links_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::ProfileFilterLink::default();
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
            fn request_filter_ref_init(request: &mut api::ProfileFilterLink) {
                if request.filter_ref.is_none() {
                    request.filter_ref = Some(Default::default());
                }
            }
            
            fn request_profile_ref_init(request: &mut api::ProfileFilterLink) {
                if request.profile_ref.is_none() {
                    request.profile_ref = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "rank" => {
                        request.rank = Some(arg_from_str(value.unwrap_or("-0"), err, "rank", "integer"));
                    },
                "filter-ref.kind" => {
                        request_filter_ref_init(&mut request);
                        request.filter_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "filter-ref.href" => {
                        request_filter_ref_init(&mut request);
                        request.filter_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "filter-ref.id" => {
                        request_filter_ref_init(&mut request);
                        request.filter_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "filter-ref.name" => {
                        request_filter_ref_init(&mut request);
                        request.filter_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "filter-ref.account-id" => {
                        request_filter_ref_init(&mut request);
                        request.filter_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.kind" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.name" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.internal-web-property-id" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.href" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.web-property-id" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.id" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "profile-ref.account-id" => {
                        request_profile_ref_init(&mut request);
                        request.profile_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_profile_ref_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_profile_ref_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.management().profile_filter_links_update(request, &self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_profile_id, &self.opt.arg_link_id);
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

    fn _management_profile_user_links_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.management().profile_user_links_delete(&self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_profile_id, &self.opt.arg_link_id);
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

    fn _management_profile_user_links_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::EntityUserLink::default();
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
            fn request_entity_account_ref_init(request: &mut api::EntityUserLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().account_ref.is_none() {
                    request.entity.as_mut().unwrap().account_ref = Some(Default::default());
                }
            }
            
            fn request_entity_init(request: &mut api::EntityUserLink) {
                if request.entity.is_none() {
                    request.entity = Some(Default::default());
                }
            }
            
            fn request_entity_profile_ref_init(request: &mut api::EntityUserLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().profile_ref.is_none() {
                    request.entity.as_mut().unwrap().profile_ref = Some(Default::default());
                }
            }
            
            fn request_entity_web_property_ref_init(request: &mut api::EntityUserLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().web_property_ref.is_none() {
                    request.entity.as_mut().unwrap().web_property_ref = Some(Default::default());
                }
            }
            
            fn request_permissions_init(request: &mut api::EntityUserLink) {
                if request.permissions.is_none() {
                    request.permissions = Some(Default::default());
                }
            }
            
            fn request_user_ref_init(request: &mut api::EntityUserLink) {
                if request.user_ref.is_none() {
                    request.user_ref = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.kind" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.href" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.id" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.name" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.kind" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.name" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.internal-web-property-id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.href" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.web-property-id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.account-id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.kind" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.name" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.internal-web-property-id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.href" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.account-id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "user-ref.kind" => {
                        request_user_ref_init(&mut request);
                        request.user_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "user-ref.email" => {
                        request_user_ref_init(&mut request);
                        request.user_ref.as_mut().unwrap().email = Some(value.unwrap_or("").to_string());
                    },
                "user-ref.id" => {
                        request_user_ref_init(&mut request);
                        request.user_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_user_ref_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_user_ref_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "permissions.local" => {
                        request_permissions_init(&mut request);
                        if request.permissions.as_mut().unwrap().local.is_none() {
                           request.permissions.as_mut().unwrap().local = Some(Default::default());
                        }
                                        request.permissions.as_mut().unwrap().local.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "permissions.effective" => {
                        request_permissions_init(&mut request);
                        if request.permissions.as_mut().unwrap().effective.is_none() {
                           request.permissions.as_mut().unwrap().effective = Some(Default::default());
                        }
                                        request.permissions.as_mut().unwrap().effective.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.management().profile_user_links_insert(request, &self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_profile_id);
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

    fn _management_profile_user_links_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.management().profile_user_links_list(&self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_profile_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
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

    fn _management_profile_user_links_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::EntityUserLink::default();
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
            fn request_entity_account_ref_init(request: &mut api::EntityUserLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().account_ref.is_none() {
                    request.entity.as_mut().unwrap().account_ref = Some(Default::default());
                }
            }
            
            fn request_entity_init(request: &mut api::EntityUserLink) {
                if request.entity.is_none() {
                    request.entity = Some(Default::default());
                }
            }
            
            fn request_entity_profile_ref_init(request: &mut api::EntityUserLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().profile_ref.is_none() {
                    request.entity.as_mut().unwrap().profile_ref = Some(Default::default());
                }
            }
            
            fn request_entity_web_property_ref_init(request: &mut api::EntityUserLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().web_property_ref.is_none() {
                    request.entity.as_mut().unwrap().web_property_ref = Some(Default::default());
                }
            }
            
            fn request_permissions_init(request: &mut api::EntityUserLink) {
                if request.permissions.is_none() {
                    request.permissions = Some(Default::default());
                }
            }
            
            fn request_user_ref_init(request: &mut api::EntityUserLink) {
                if request.user_ref.is_none() {
                    request.user_ref = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.kind" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.href" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.id" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.name" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.kind" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.name" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.internal-web-property-id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.href" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.web-property-id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.account-id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.kind" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.name" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.internal-web-property-id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.href" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.account-id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "user-ref.kind" => {
                        request_user_ref_init(&mut request);
                        request.user_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "user-ref.email" => {
                        request_user_ref_init(&mut request);
                        request.user_ref.as_mut().unwrap().email = Some(value.unwrap_or("").to_string());
                    },
                "user-ref.id" => {
                        request_user_ref_init(&mut request);
                        request.user_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_user_ref_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_user_ref_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "permissions.local" => {
                        request_permissions_init(&mut request);
                        if request.permissions.as_mut().unwrap().local.is_none() {
                           request.permissions.as_mut().unwrap().local = Some(Default::default());
                        }
                                        request.permissions.as_mut().unwrap().local.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "permissions.effective" => {
                        request_permissions_init(&mut request);
                        if request.permissions.as_mut().unwrap().effective.is_none() {
                           request.permissions.as_mut().unwrap().effective = Some(Default::default());
                        }
                                        request.permissions.as_mut().unwrap().effective.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.management().profile_user_links_update(request, &self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_profile_id, &self.opt.arg_link_id);
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

    fn _management_profiles_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.management().profiles_delete(&self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_profile_id);
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

    fn _management_profiles_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.management().profiles_get(&self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_profile_id);
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

    fn _management_profiles_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Profile::default();
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
            fn request_child_link_init(request: &mut api::Profile) {
                if request.child_link.is_none() {
                    request.child_link = Some(Default::default());
                }
            }
            
            fn request_parent_link_init(request: &mut api::Profile) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            fn request_permissions_init(request: &mut api::Profile) {
                if request.permissions.is_none() {
                    request.permissions = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "currency" => {
                        request.currency = Some(value.unwrap_or("").to_string());
                    },
                "e-commerce-tracking" => {
                        request.e_commerce_tracking = Some(arg_from_str(value.unwrap_or("false"), err, "e-commerce-tracking", "boolean"));
                    },
                "web-property-id" => {
                        request.web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "timezone" => {
                        request.timezone = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                "strip-site-search-category-parameters" => {
                        request.strip_site_search_category_parameters = Some(arg_from_str(value.unwrap_or("false"), err, "strip-site-search-category-parameters", "boolean"));
                    },
                "site-search-category-parameters" => {
                        request.site_search_category_parameters = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "exclude-query-parameters" => {
                        request.exclude_query_parameters = Some(value.unwrap_or("").to_string());
                    },
                "internal-web-property-id" => {
                        request.internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "child-link.href" => {
                        request_child_link_init(&mut request);
                        request.child_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "child-link.type" => {
                        request_child_link_init(&mut request);
                        request.child_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "enhanced-e-commerce-tracking" => {
                        request_child_link_init(&mut request);
                        request.enhanced_e_commerce_tracking = Some(arg_from_str(value.unwrap_or("false"), err, "enhanced-e-commerce-tracking", "boolean"));
                    },
                "permissions.effective" => {
                        request_permissions_init(&mut request);
                        if request.permissions.as_mut().unwrap().effective.is_none() {
                           request.permissions.as_mut().unwrap().effective = Some(Default::default());
                        }
                                        request.permissions.as_mut().unwrap().effective.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "default-page" => {
                        request_permissions_init(&mut request);
                        request.default_page = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_permissions_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "strip-site-search-query-parameters" => {
                        request_permissions_init(&mut request);
                        request.strip_site_search_query_parameters = Some(arg_from_str(value.unwrap_or("false"), err, "strip-site-search-query-parameters", "boolean"));
                    },
                "name" => {
                        request_permissions_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request_permissions_init(&mut request);
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "site-search-query-parameters" => {
                        request_permissions_init(&mut request);
                        request.site_search_query_parameters = Some(value.unwrap_or("").to_string());
                    },
                "website-url" => {
                        request_permissions_init(&mut request);
                        request.website_url = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_parent_link_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.management().profiles_insert(request, &self.opt.arg_account_id, &self.opt.arg_web_property_id);
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

    fn _management_profiles_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.management().profiles_list(&self.opt.arg_account_id, &self.opt.arg_web_property_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
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

    fn _management_profiles_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Profile::default();
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
            fn request_child_link_init(request: &mut api::Profile) {
                if request.child_link.is_none() {
                    request.child_link = Some(Default::default());
                }
            }
            
            fn request_parent_link_init(request: &mut api::Profile) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            fn request_permissions_init(request: &mut api::Profile) {
                if request.permissions.is_none() {
                    request.permissions = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "currency" => {
                        request.currency = Some(value.unwrap_or("").to_string());
                    },
                "e-commerce-tracking" => {
                        request.e_commerce_tracking = Some(arg_from_str(value.unwrap_or("false"), err, "e-commerce-tracking", "boolean"));
                    },
                "web-property-id" => {
                        request.web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "timezone" => {
                        request.timezone = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                "strip-site-search-category-parameters" => {
                        request.strip_site_search_category_parameters = Some(arg_from_str(value.unwrap_or("false"), err, "strip-site-search-category-parameters", "boolean"));
                    },
                "site-search-category-parameters" => {
                        request.site_search_category_parameters = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "exclude-query-parameters" => {
                        request.exclude_query_parameters = Some(value.unwrap_or("").to_string());
                    },
                "internal-web-property-id" => {
                        request.internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "child-link.href" => {
                        request_child_link_init(&mut request);
                        request.child_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "child-link.type" => {
                        request_child_link_init(&mut request);
                        request.child_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "enhanced-e-commerce-tracking" => {
                        request_child_link_init(&mut request);
                        request.enhanced_e_commerce_tracking = Some(arg_from_str(value.unwrap_or("false"), err, "enhanced-e-commerce-tracking", "boolean"));
                    },
                "permissions.effective" => {
                        request_permissions_init(&mut request);
                        if request.permissions.as_mut().unwrap().effective.is_none() {
                           request.permissions.as_mut().unwrap().effective = Some(Default::default());
                        }
                                        request.permissions.as_mut().unwrap().effective.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "default-page" => {
                        request_permissions_init(&mut request);
                        request.default_page = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_permissions_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "strip-site-search-query-parameters" => {
                        request_permissions_init(&mut request);
                        request.strip_site_search_query_parameters = Some(arg_from_str(value.unwrap_or("false"), err, "strip-site-search-query-parameters", "boolean"));
                    },
                "name" => {
                        request_permissions_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request_permissions_init(&mut request);
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "site-search-query-parameters" => {
                        request_permissions_init(&mut request);
                        request.site_search_query_parameters = Some(value.unwrap_or("").to_string());
                    },
                "website-url" => {
                        request_permissions_init(&mut request);
                        request.website_url = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_parent_link_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.management().profiles_patch(request, &self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_profile_id);
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

    fn _management_profiles_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Profile::default();
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
            fn request_child_link_init(request: &mut api::Profile) {
                if request.child_link.is_none() {
                    request.child_link = Some(Default::default());
                }
            }
            
            fn request_parent_link_init(request: &mut api::Profile) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            fn request_permissions_init(request: &mut api::Profile) {
                if request.permissions.is_none() {
                    request.permissions = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "currency" => {
                        request.currency = Some(value.unwrap_or("").to_string());
                    },
                "e-commerce-tracking" => {
                        request.e_commerce_tracking = Some(arg_from_str(value.unwrap_or("false"), err, "e-commerce-tracking", "boolean"));
                    },
                "web-property-id" => {
                        request.web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "timezone" => {
                        request.timezone = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                "strip-site-search-category-parameters" => {
                        request.strip_site_search_category_parameters = Some(arg_from_str(value.unwrap_or("false"), err, "strip-site-search-category-parameters", "boolean"));
                    },
                "site-search-category-parameters" => {
                        request.site_search_category_parameters = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "exclude-query-parameters" => {
                        request.exclude_query_parameters = Some(value.unwrap_or("").to_string());
                    },
                "internal-web-property-id" => {
                        request.internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "child-link.href" => {
                        request_child_link_init(&mut request);
                        request.child_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "child-link.type" => {
                        request_child_link_init(&mut request);
                        request.child_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "enhanced-e-commerce-tracking" => {
                        request_child_link_init(&mut request);
                        request.enhanced_e_commerce_tracking = Some(arg_from_str(value.unwrap_or("false"), err, "enhanced-e-commerce-tracking", "boolean"));
                    },
                "permissions.effective" => {
                        request_permissions_init(&mut request);
                        if request.permissions.as_mut().unwrap().effective.is_none() {
                           request.permissions.as_mut().unwrap().effective = Some(Default::default());
                        }
                                        request.permissions.as_mut().unwrap().effective.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "default-page" => {
                        request_permissions_init(&mut request);
                        request.default_page = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_permissions_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "strip-site-search-query-parameters" => {
                        request_permissions_init(&mut request);
                        request.strip_site_search_query_parameters = Some(arg_from_str(value.unwrap_or("false"), err, "strip-site-search-query-parameters", "boolean"));
                    },
                "name" => {
                        request_permissions_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request_permissions_init(&mut request);
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "site-search-query-parameters" => {
                        request_permissions_init(&mut request);
                        request.site_search_query_parameters = Some(value.unwrap_or("").to_string());
                    },
                "website-url" => {
                        request_permissions_init(&mut request);
                        request.website_url = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_parent_link_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.management().profiles_update(request, &self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_profile_id);
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

    fn _management_segments_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.management().segments_list();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
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

    fn _management_unsampled_reports_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.management().unsampled_reports_get(&self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_profile_id, &self.opt.arg_unsampled_report_id);
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

    fn _management_unsampled_reports_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::UnsampledReport::default();
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
            fn request_cloud_storage_download_details_init(request: &mut api::UnsampledReport) {
                if request.cloud_storage_download_details.is_none() {
                    request.cloud_storage_download_details = Some(Default::default());
                }
            }
            
            fn request_drive_download_details_init(request: &mut api::UnsampledReport) {
                if request.drive_download_details.is_none() {
                    request.drive_download_details = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "status" => {
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "cloud-storage-download-details.bucket-id" => {
                        request_cloud_storage_download_details_init(&mut request);
                        request.cloud_storage_download_details.as_mut().unwrap().bucket_id = Some(value.unwrap_or("").to_string());
                    },
                "cloud-storage-download-details.object-id" => {
                        request_cloud_storage_download_details_init(&mut request);
                        request.cloud_storage_download_details.as_mut().unwrap().object_id = Some(value.unwrap_or("").to_string());
                    },
                "download-type" => {
                        request_cloud_storage_download_details_init(&mut request);
                        request.download_type = Some(value.unwrap_or("").to_string());
                    },
                "dimensions" => {
                        request_cloud_storage_download_details_init(&mut request);
                        request.dimensions = Some(value.unwrap_or("").to_string());
                    },
                "start-date" => {
                        request_cloud_storage_download_details_init(&mut request);
                        request.start_date = Some(value.unwrap_or("").to_string());
                    },
                "end-date" => {
                        request_cloud_storage_download_details_init(&mut request);
                        request.end_date = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request_cloud_storage_download_details_init(&mut request);
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_cloud_storage_download_details_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "drive-download-details.document-id" => {
                        request_drive_download_details_init(&mut request);
                        request.drive_download_details.as_mut().unwrap().document_id = Some(value.unwrap_or("").to_string());
                    },
                "profile-id" => {
                        request_drive_download_details_init(&mut request);
                        request.profile_id = Some(value.unwrap_or("").to_string());
                    },
                "metrics" => {
                        request_drive_download_details_init(&mut request);
                        request.metrics = Some(value.unwrap_or("").to_string());
                    },
                "filters" => {
                        request_drive_download_details_init(&mut request);
                        request.filters = Some(value.unwrap_or("").to_string());
                    },
                "web-property-id" => {
                        request_drive_download_details_init(&mut request);
                        request.web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "title" => {
                        request_drive_download_details_init(&mut request);
                        request.title = Some(value.unwrap_or("").to_string());
                    },
                "segment" => {
                        request_drive_download_details_init(&mut request);
                        request.segment = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_drive_download_details_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_drive_download_details_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_drive_download_details_init(&mut request);
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.management().unsampled_reports_insert(request, &self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_profile_id);
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

    fn _management_unsampled_reports_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.management().unsampled_reports_list(&self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_profile_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
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

    fn _management_uploads_delete_upload_data(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::AnalyticsDataimportDeleteUploadDataRequest::default();
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
                "custom-data-import-uids" => {
                        if request.custom_data_import_uids.is_none() {
                           request.custom_data_import_uids = Some(Default::default());
                        }
                                        request.custom_data_import_uids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.management().uploads_delete_upload_data(request, &self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_custom_data_source_id);
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

    fn _management_uploads_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.management().uploads_get(&self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_custom_data_source_id, &self.opt.arg_upload_id);
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

    fn _management_uploads_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.management().uploads_list(&self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_custom_data_source_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
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

    fn _management_uploads_upload_data(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.management().uploads_upload_data(&self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_custom_data_source_id);
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

    fn _management_web_property_ad_words_links_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.management().web_property_ad_words_links_delete(&self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_web_property_ad_words_link_id);
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

    fn _management_web_property_ad_words_links_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.management().web_property_ad_words_links_get(&self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_web_property_ad_words_link_id);
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

    fn _management_web_property_ad_words_links_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::EntityAdWordsLink::default();
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
            fn request_entity_init(request: &mut api::EntityAdWordsLink) {
                if request.entity.is_none() {
                    request.entity = Some(Default::default());
                }
            }
            
            fn request_entity_web_property_ref_init(request: &mut api::EntityAdWordsLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().web_property_ref.is_none() {
                    request.entity.as_mut().unwrap().web_property_ref = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "profile-ids" => {
                        if request.profile_ids.is_none() {
                           request.profile_ids = Some(Default::default());
                        }
                                        request.profile_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.kind" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.name" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.internal-web-property-id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.href" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.account-id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_entity_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_entity_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.management().web_property_ad_words_links_insert(request, &self.opt.arg_account_id, &self.opt.arg_web_property_id);
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

    fn _management_web_property_ad_words_links_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.management().web_property_ad_words_links_list(&self.opt.arg_account_id, &self.opt.arg_web_property_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
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

    fn _management_web_property_ad_words_links_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::EntityAdWordsLink::default();
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
            fn request_entity_init(request: &mut api::EntityAdWordsLink) {
                if request.entity.is_none() {
                    request.entity = Some(Default::default());
                }
            }
            
            fn request_entity_web_property_ref_init(request: &mut api::EntityAdWordsLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().web_property_ref.is_none() {
                    request.entity.as_mut().unwrap().web_property_ref = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "profile-ids" => {
                        if request.profile_ids.is_none() {
                           request.profile_ids = Some(Default::default());
                        }
                                        request.profile_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.kind" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.name" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.internal-web-property-id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.href" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.account-id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_entity_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_entity_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.management().web_property_ad_words_links_patch(request, &self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_web_property_ad_words_link_id);
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

    fn _management_web_property_ad_words_links_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::EntityAdWordsLink::default();
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
            fn request_entity_init(request: &mut api::EntityAdWordsLink) {
                if request.entity.is_none() {
                    request.entity = Some(Default::default());
                }
            }
            
            fn request_entity_web_property_ref_init(request: &mut api::EntityAdWordsLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().web_property_ref.is_none() {
                    request.entity.as_mut().unwrap().web_property_ref = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "profile-ids" => {
                        if request.profile_ids.is_none() {
                           request.profile_ids = Some(Default::default());
                        }
                                        request.profile_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.kind" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.name" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.internal-web-property-id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.href" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.account-id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_entity_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_entity_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.management().web_property_ad_words_links_update(request, &self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_web_property_ad_words_link_id);
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

    fn _management_webproperties_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.management().webproperties_get(&self.opt.arg_account_id, &self.opt.arg_web_property_id);
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

    fn _management_webproperties_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Webproperty::default();
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
            fn request_child_link_init(request: &mut api::Webproperty) {
                if request.child_link.is_none() {
                    request.child_link = Some(Default::default());
                }
            }
            
            fn request_parent_link_init(request: &mut api::Webproperty) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            fn request_permissions_init(request: &mut api::Webproperty) {
                if request.permissions.is_none() {
                    request.permissions = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "website-url" => {
                        request.website_url = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "default-profile-id" => {
                        request.default_profile_id = Some(value.unwrap_or("").to_string());
                    },
                "level" => {
                        request.level = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "profile-count" => {
                        request.profile_count = Some(arg_from_str(value.unwrap_or("-0"), err, "profile-count", "integer"));
                    },
                "internal-web-property-id" => {
                        request.internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "child-link.href" => {
                        request_child_link_init(&mut request);
                        request.child_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "child-link.type" => {
                        request_child_link_init(&mut request);
                        request.child_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "industry-vertical" => {
                        request_child_link_init(&mut request);
                        request.industry_vertical = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "permissions.effective" => {
                        request_permissions_init(&mut request);
                        if request.permissions.as_mut().unwrap().effective.is_none() {
                           request.permissions.as_mut().unwrap().effective = Some(Default::default());
                        }
                                        request.permissions.as_mut().unwrap().effective.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_permissions_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_permissions_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_permissions_init(&mut request);
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.management().webproperties_insert(request, &self.opt.arg_account_id);
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

    fn _management_webproperties_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.management().webproperties_list(&self.opt.arg_account_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
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

    fn _management_webproperties_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Webproperty::default();
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
            fn request_child_link_init(request: &mut api::Webproperty) {
                if request.child_link.is_none() {
                    request.child_link = Some(Default::default());
                }
            }
            
            fn request_parent_link_init(request: &mut api::Webproperty) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            fn request_permissions_init(request: &mut api::Webproperty) {
                if request.permissions.is_none() {
                    request.permissions = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "website-url" => {
                        request.website_url = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "default-profile-id" => {
                        request.default_profile_id = Some(value.unwrap_or("").to_string());
                    },
                "level" => {
                        request.level = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "profile-count" => {
                        request.profile_count = Some(arg_from_str(value.unwrap_or("-0"), err, "profile-count", "integer"));
                    },
                "internal-web-property-id" => {
                        request.internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "child-link.href" => {
                        request_child_link_init(&mut request);
                        request.child_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "child-link.type" => {
                        request_child_link_init(&mut request);
                        request.child_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "industry-vertical" => {
                        request_child_link_init(&mut request);
                        request.industry_vertical = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "permissions.effective" => {
                        request_permissions_init(&mut request);
                        if request.permissions.as_mut().unwrap().effective.is_none() {
                           request.permissions.as_mut().unwrap().effective = Some(Default::default());
                        }
                                        request.permissions.as_mut().unwrap().effective.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_permissions_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_permissions_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_permissions_init(&mut request);
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.management().webproperties_patch(request, &self.opt.arg_account_id, &self.opt.arg_web_property_id);
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

    fn _management_webproperties_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Webproperty::default();
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
            fn request_child_link_init(request: &mut api::Webproperty) {
                if request.child_link.is_none() {
                    request.child_link = Some(Default::default());
                }
            }
            
            fn request_parent_link_init(request: &mut api::Webproperty) {
                if request.parent_link.is_none() {
                    request.parent_link = Some(Default::default());
                }
            }
            
            fn request_permissions_init(request: &mut api::Webproperty) {
                if request.permissions.is_none() {
                    request.permissions = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "website-url" => {
                        request.website_url = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "default-profile-id" => {
                        request.default_profile_id = Some(value.unwrap_or("").to_string());
                    },
                "level" => {
                        request.level = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "profile-count" => {
                        request.profile_count = Some(arg_from_str(value.unwrap_or("-0"), err, "profile-count", "integer"));
                    },
                "internal-web-property-id" => {
                        request.internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "child-link.href" => {
                        request_child_link_init(&mut request);
                        request.child_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "child-link.type" => {
                        request_child_link_init(&mut request);
                        request.child_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "industry-vertical" => {
                        request_child_link_init(&mut request);
                        request.industry_vertical = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.href" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "parent-link.type" => {
                        request_parent_link_init(&mut request);
                        request.parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "permissions.effective" => {
                        request_permissions_init(&mut request);
                        if request.permissions.as_mut().unwrap().effective.is_none() {
                           request.permissions.as_mut().unwrap().effective = Some(Default::default());
                        }
                                        request.permissions.as_mut().unwrap().effective.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_permissions_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_permissions_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "account-id" => {
                        request_permissions_init(&mut request);
                        request.account_id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.management().webproperties_update(request, &self.opt.arg_account_id, &self.opt.arg_web_property_id);
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

    fn _management_webproperty_user_links_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.management().webproperty_user_links_delete(&self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_link_id);
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

    fn _management_webproperty_user_links_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::EntityUserLink::default();
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
            fn request_entity_account_ref_init(request: &mut api::EntityUserLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().account_ref.is_none() {
                    request.entity.as_mut().unwrap().account_ref = Some(Default::default());
                }
            }
            
            fn request_entity_init(request: &mut api::EntityUserLink) {
                if request.entity.is_none() {
                    request.entity = Some(Default::default());
                }
            }
            
            fn request_entity_profile_ref_init(request: &mut api::EntityUserLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().profile_ref.is_none() {
                    request.entity.as_mut().unwrap().profile_ref = Some(Default::default());
                }
            }
            
            fn request_entity_web_property_ref_init(request: &mut api::EntityUserLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().web_property_ref.is_none() {
                    request.entity.as_mut().unwrap().web_property_ref = Some(Default::default());
                }
            }
            
            fn request_permissions_init(request: &mut api::EntityUserLink) {
                if request.permissions.is_none() {
                    request.permissions = Some(Default::default());
                }
            }
            
            fn request_user_ref_init(request: &mut api::EntityUserLink) {
                if request.user_ref.is_none() {
                    request.user_ref = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.kind" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.href" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.id" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.name" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.kind" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.name" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.internal-web-property-id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.href" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.web-property-id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.account-id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.kind" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.name" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.internal-web-property-id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.href" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.account-id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "user-ref.kind" => {
                        request_user_ref_init(&mut request);
                        request.user_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "user-ref.email" => {
                        request_user_ref_init(&mut request);
                        request.user_ref.as_mut().unwrap().email = Some(value.unwrap_or("").to_string());
                    },
                "user-ref.id" => {
                        request_user_ref_init(&mut request);
                        request.user_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_user_ref_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_user_ref_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "permissions.local" => {
                        request_permissions_init(&mut request);
                        if request.permissions.as_mut().unwrap().local.is_none() {
                           request.permissions.as_mut().unwrap().local = Some(Default::default());
                        }
                                        request.permissions.as_mut().unwrap().local.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "permissions.effective" => {
                        request_permissions_init(&mut request);
                        if request.permissions.as_mut().unwrap().effective.is_none() {
                           request.permissions.as_mut().unwrap().effective = Some(Default::default());
                        }
                                        request.permissions.as_mut().unwrap().effective.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.management().webproperty_user_links_insert(request, &self.opt.arg_account_id, &self.opt.arg_web_property_id);
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

    fn _management_webproperty_user_links_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.management().webproperty_user_links_list(&self.opt.arg_account_id, &self.opt.arg_web_property_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
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

    fn _management_webproperty_user_links_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::EntityUserLink::default();
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
            fn request_entity_account_ref_init(request: &mut api::EntityUserLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().account_ref.is_none() {
                    request.entity.as_mut().unwrap().account_ref = Some(Default::default());
                }
            }
            
            fn request_entity_init(request: &mut api::EntityUserLink) {
                if request.entity.is_none() {
                    request.entity = Some(Default::default());
                }
            }
            
            fn request_entity_profile_ref_init(request: &mut api::EntityUserLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().profile_ref.is_none() {
                    request.entity.as_mut().unwrap().profile_ref = Some(Default::default());
                }
            }
            
            fn request_entity_web_property_ref_init(request: &mut api::EntityUserLink) {
                request_entity_init(request);
                if request.entity.as_mut().unwrap().web_property_ref.is_none() {
                    request.entity.as_mut().unwrap().web_property_ref = Some(Default::default());
                }
            }
            
            fn request_permissions_init(request: &mut api::EntityUserLink) {
                if request.permissions.is_none() {
                    request.permissions = Some(Default::default());
                }
            }
            
            fn request_user_ref_init(request: &mut api::EntityUserLink) {
                if request.user_ref.is_none() {
                    request.user_ref = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.kind" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.href" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.id" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.account-ref.name" => {
                        request_entity_account_ref_init(&mut request);
                        request.entity.as_mut().unwrap().account_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.kind" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.name" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.internal-web-property-id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.href" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.web-property-id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.profile-ref.account-id" => {
                        request_entity_profile_ref_init(&mut request);
                        request.entity.as_mut().unwrap().profile_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.kind" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.name" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.internal-web-property-id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.href" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "entity.web-property-ref.account-id" => {
                        request_entity_web_property_ref_init(&mut request);
                        request.entity.as_mut().unwrap().web_property_ref.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "user-ref.kind" => {
                        request_user_ref_init(&mut request);
                        request.user_ref.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "user-ref.email" => {
                        request_user_ref_init(&mut request);
                        request.user_ref.as_mut().unwrap().email = Some(value.unwrap_or("").to_string());
                    },
                "user-ref.id" => {
                        request_user_ref_init(&mut request);
                        request.user_ref.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_user_ref_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_user_ref_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "permissions.local" => {
                        request_permissions_init(&mut request);
                        if request.permissions.as_mut().unwrap().local.is_none() {
                           request.permissions.as_mut().unwrap().local = Some(Default::default());
                        }
                                        request.permissions.as_mut().unwrap().local.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "permissions.effective" => {
                        request_permissions_init(&mut request);
                        if request.permissions.as_mut().unwrap().effective.is_none() {
                           request.permissions.as_mut().unwrap().effective = Some(Default::default());
                        }
                                        request.permissions.as_mut().unwrap().effective.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.management().webproperty_user_links_update(request, &self.opt.arg_account_id, &self.opt.arg_web_property_id, &self.opt.arg_link_id);
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

    fn _metadata_columns_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.metadata().columns_list(&self.opt.arg_report_type);
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

    fn _provisioning_create_account_ticket(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::AccountTicket::default();
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
            fn request_account_child_link_init(request: &mut api::AccountTicket) {
                request_account_init(request);
                if request.account.as_mut().unwrap().child_link.is_none() {
                    request.account.as_mut().unwrap().child_link = Some(Default::default());
                }
            }
            
            fn request_account_init(request: &mut api::AccountTicket) {
                if request.account.is_none() {
                    request.account = Some(Default::default());
                }
            }
            
            fn request_account_permissions_init(request: &mut api::AccountTicket) {
                request_account_init(request);
                if request.account.as_mut().unwrap().permissions.is_none() {
                    request.account.as_mut().unwrap().permissions = Some(Default::default());
                }
            }
            
            fn request_profile_child_link_init(request: &mut api::AccountTicket) {
                request_profile_init(request);
                if request.profile.as_mut().unwrap().child_link.is_none() {
                    request.profile.as_mut().unwrap().child_link = Some(Default::default());
                }
            }
            
            fn request_profile_init(request: &mut api::AccountTicket) {
                if request.profile.is_none() {
                    request.profile = Some(Default::default());
                }
            }
            
            fn request_profile_parent_link_init(request: &mut api::AccountTicket) {
                request_profile_init(request);
                if request.profile.as_mut().unwrap().parent_link.is_none() {
                    request.profile.as_mut().unwrap().parent_link = Some(Default::default());
                }
            }
            
            fn request_profile_permissions_init(request: &mut api::AccountTicket) {
                request_profile_init(request);
                if request.profile.as_mut().unwrap().permissions.is_none() {
                    request.profile.as_mut().unwrap().permissions = Some(Default::default());
                }
            }
            
            fn request_webproperty_child_link_init(request: &mut api::AccountTicket) {
                request_webproperty_init(request);
                if request.webproperty.as_mut().unwrap().child_link.is_none() {
                    request.webproperty.as_mut().unwrap().child_link = Some(Default::default());
                }
            }
            
            fn request_webproperty_init(request: &mut api::AccountTicket) {
                if request.webproperty.is_none() {
                    request.webproperty = Some(Default::default());
                }
            }
            
            fn request_webproperty_parent_link_init(request: &mut api::AccountTicket) {
                request_webproperty_init(request);
                if request.webproperty.as_mut().unwrap().parent_link.is_none() {
                    request.webproperty.as_mut().unwrap().parent_link = Some(Default::default());
                }
            }
            
            fn request_webproperty_permissions_init(request: &mut api::AccountTicket) {
                request_webproperty_init(request);
                if request.webproperty.as_mut().unwrap().permissions.is_none() {
                    request.webproperty.as_mut().unwrap().permissions = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "profile.currency" => {
                        request_profile_init(&mut request);
                        request.profile.as_mut().unwrap().currency = Some(value.unwrap_or("").to_string());
                    },
                "profile.e-commerce-tracking" => {
                        request_profile_init(&mut request);
                        request.profile.as_mut().unwrap().e_commerce_tracking = Some(arg_from_str(value.unwrap_or("false"), err, "profile.e-commerce-tracking", "boolean"));
                    },
                "profile.web-property-id" => {
                        request_profile_init(&mut request);
                        request.profile.as_mut().unwrap().web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "profile.timezone" => {
                        request_profile_init(&mut request);
                        request.profile.as_mut().unwrap().timezone = Some(value.unwrap_or("").to_string());
                    },
                "profile.id" => {
                        request_profile_init(&mut request);
                        request.profile.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "profile.account-id" => {
                        request_profile_init(&mut request);
                        request.profile.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "profile.strip-site-search-category-parameters" => {
                        request_profile_init(&mut request);
                        request.profile.as_mut().unwrap().strip_site_search_category_parameters = Some(arg_from_str(value.unwrap_or("false"), err, "profile.strip-site-search-category-parameters", "boolean"));
                    },
                "profile.site-search-category-parameters" => {
                        request_profile_init(&mut request);
                        request.profile.as_mut().unwrap().site_search_category_parameters = Some(value.unwrap_or("").to_string());
                    },
                "profile.type" => {
                        request_profile_init(&mut request);
                        request.profile.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "profile.updated" => {
                        request_profile_init(&mut request);
                        request.profile.as_mut().unwrap().updated = Some(value.unwrap_or("").to_string());
                    },
                "profile.exclude-query-parameters" => {
                        request_profile_init(&mut request);
                        request.profile.as_mut().unwrap().exclude_query_parameters = Some(value.unwrap_or("").to_string());
                    },
                "profile.internal-web-property-id" => {
                        request_profile_init(&mut request);
                        request.profile.as_mut().unwrap().internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "profile.child-link.href" => {
                        request_profile_child_link_init(&mut request);
                        request.profile.as_mut().unwrap().child_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "profile.child-link.type" => {
                        request_profile_child_link_init(&mut request);
                        request.profile.as_mut().unwrap().child_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "profile.enhanced-e-commerce-tracking" => {
                        request_profile_child_link_init(&mut request);
                        request.profile.as_mut().unwrap().enhanced_e_commerce_tracking = Some(arg_from_str(value.unwrap_or("false"), err, "profile.enhanced-e-commerce-tracking", "boolean"));
                    },
                "profile.permissions.effective" => {
                        request_profile_permissions_init(&mut request);
                        if request.profile.as_mut().unwrap().permissions.as_mut().unwrap().effective.is_none() {
                           request.profile.as_mut().unwrap().permissions.as_mut().unwrap().effective = Some(Default::default());
                        }
                                        request.profile.as_mut().unwrap().permissions.as_mut().unwrap().effective.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "profile.default-page" => {
                        request_profile_permissions_init(&mut request);
                        request.profile.as_mut().unwrap().default_page = Some(value.unwrap_or("").to_string());
                    },
                "profile.kind" => {
                        request_profile_permissions_init(&mut request);
                        request.profile.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "profile.strip-site-search-query-parameters" => {
                        request_profile_permissions_init(&mut request);
                        request.profile.as_mut().unwrap().strip_site_search_query_parameters = Some(arg_from_str(value.unwrap_or("false"), err, "profile.strip-site-search-query-parameters", "boolean"));
                    },
                "profile.name" => {
                        request_profile_permissions_init(&mut request);
                        request.profile.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "profile.created" => {
                        request_profile_permissions_init(&mut request);
                        request.profile.as_mut().unwrap().created = Some(value.unwrap_or("").to_string());
                    },
                "profile.site-search-query-parameters" => {
                        request_profile_permissions_init(&mut request);
                        request.profile.as_mut().unwrap().site_search_query_parameters = Some(value.unwrap_or("").to_string());
                    },
                "profile.website-url" => {
                        request_profile_permissions_init(&mut request);
                        request.profile.as_mut().unwrap().website_url = Some(value.unwrap_or("").to_string());
                    },
                "profile.parent-link.href" => {
                        request_profile_parent_link_init(&mut request);
                        request.profile.as_mut().unwrap().parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "profile.parent-link.type" => {
                        request_profile_parent_link_init(&mut request);
                        request.profile.as_mut().unwrap().parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "profile.self-link" => {
                        request_profile_parent_link_init(&mut request);
                        request.profile.as_mut().unwrap().self_link = Some(value.unwrap_or("").to_string());
                    },
                "account.kind" => {
                        request_account_init(&mut request);
                        request.account.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "account.name" => {
                        request_account_init(&mut request);
                        request.account.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "account.created" => {
                        request_account_init(&mut request);
                        request.account.as_mut().unwrap().created = Some(value.unwrap_or("").to_string());
                    },
                "account.updated" => {
                        request_account_init(&mut request);
                        request.account.as_mut().unwrap().updated = Some(value.unwrap_or("").to_string());
                    },
                "account.child-link.href" => {
                        request_account_child_link_init(&mut request);
                        request.account.as_mut().unwrap().child_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "account.child-link.type" => {
                        request_account_child_link_init(&mut request);
                        request.account.as_mut().unwrap().child_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "account.id" => {
                        request_account_child_link_init(&mut request);
                        request.account.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "account.self-link" => {
                        request_account_child_link_init(&mut request);
                        request.account.as_mut().unwrap().self_link = Some(value.unwrap_or("").to_string());
                    },
                "account.permissions.effective" => {
                        request_account_permissions_init(&mut request);
                        if request.account.as_mut().unwrap().permissions.as_mut().unwrap().effective.is_none() {
                           request.account.as_mut().unwrap().permissions.as_mut().unwrap().effective = Some(Default::default());
                        }
                                        request.account.as_mut().unwrap().permissions.as_mut().unwrap().effective.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "webproperty.website-url" => {
                        request_webproperty_init(&mut request);
                        request.webproperty.as_mut().unwrap().website_url = Some(value.unwrap_or("").to_string());
                    },
                "webproperty.updated" => {
                        request_webproperty_init(&mut request);
                        request.webproperty.as_mut().unwrap().updated = Some(value.unwrap_or("").to_string());
                    },
                "webproperty.name" => {
                        request_webproperty_init(&mut request);
                        request.webproperty.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "webproperty.created" => {
                        request_webproperty_init(&mut request);
                        request.webproperty.as_mut().unwrap().created = Some(value.unwrap_or("").to_string());
                    },
                "webproperty.default-profile-id" => {
                        request_webproperty_init(&mut request);
                        request.webproperty.as_mut().unwrap().default_profile_id = Some(value.unwrap_or("").to_string());
                    },
                "webproperty.level" => {
                        request_webproperty_init(&mut request);
                        request.webproperty.as_mut().unwrap().level = Some(value.unwrap_or("").to_string());
                    },
                "webproperty.kind" => {
                        request_webproperty_init(&mut request);
                        request.webproperty.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "webproperty.profile-count" => {
                        request_webproperty_init(&mut request);
                        request.webproperty.as_mut().unwrap().profile_count = Some(arg_from_str(value.unwrap_or("-0"), err, "webproperty.profile-count", "integer"));
                    },
                "webproperty.internal-web-property-id" => {
                        request_webproperty_init(&mut request);
                        request.webproperty.as_mut().unwrap().internal_web_property_id = Some(value.unwrap_or("").to_string());
                    },
                "webproperty.child-link.href" => {
                        request_webproperty_child_link_init(&mut request);
                        request.webproperty.as_mut().unwrap().child_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "webproperty.child-link.type" => {
                        request_webproperty_child_link_init(&mut request);
                        request.webproperty.as_mut().unwrap().child_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "webproperty.industry-vertical" => {
                        request_webproperty_child_link_init(&mut request);
                        request.webproperty.as_mut().unwrap().industry_vertical = Some(value.unwrap_or("").to_string());
                    },
                "webproperty.parent-link.href" => {
                        request_webproperty_parent_link_init(&mut request);
                        request.webproperty.as_mut().unwrap().parent_link.as_mut().unwrap().href = Some(value.unwrap_or("").to_string());
                    },
                "webproperty.parent-link.type" => {
                        request_webproperty_parent_link_init(&mut request);
                        request.webproperty.as_mut().unwrap().parent_link.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "webproperty.permissions.effective" => {
                        request_webproperty_permissions_init(&mut request);
                        if request.webproperty.as_mut().unwrap().permissions.as_mut().unwrap().effective.is_none() {
                           request.webproperty.as_mut().unwrap().permissions.as_mut().unwrap().effective = Some(Default::default());
                        }
                                        request.webproperty.as_mut().unwrap().permissions.as_mut().unwrap().effective.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "webproperty.id" => {
                        request_webproperty_permissions_init(&mut request);
                        request.webproperty.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "webproperty.self-link" => {
                        request_webproperty_permissions_init(&mut request);
                        request.webproperty.as_mut().unwrap().self_link = Some(value.unwrap_or("").to_string());
                    },
                "webproperty.account-id" => {
                        request_webproperty_permissions_init(&mut request);
                        request.webproperty.as_mut().unwrap().account_id = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_webproperty_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "redirect-uri" => {
                        request_webproperty_init(&mut request);
                        request.redirect_uri = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_webproperty_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.provisioning().create_account_ticket(request);
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

        if self.opt.cmd_data {
            if self.opt.cmd_ga_get {
                call_result = self._data_ga_get(dry_run, &mut err);
            } else if self.opt.cmd_mcf_get {
                call_result = self._data_mcf_get(dry_run, &mut err);
            } else if self.opt.cmd_realtime_get {
                call_result = self._data_realtime_get(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_management {
            if self.opt.cmd_account_summaries_list {
                call_result = self._management_account_summaries_list(dry_run, &mut err);
            } else if self.opt.cmd_account_user_links_delete {
                call_result = self._management_account_user_links_delete(dry_run, &mut err);
            } else if self.opt.cmd_account_user_links_insert {
                call_result = self._management_account_user_links_insert(dry_run, &mut err);
            } else if self.opt.cmd_account_user_links_list {
                call_result = self._management_account_user_links_list(dry_run, &mut err);
            } else if self.opt.cmd_account_user_links_update {
                call_result = self._management_account_user_links_update(dry_run, &mut err);
            } else if self.opt.cmd_accounts_list {
                call_result = self._management_accounts_list(dry_run, &mut err);
            } else if self.opt.cmd_custom_data_sources_list {
                call_result = self._management_custom_data_sources_list(dry_run, &mut err);
            } else if self.opt.cmd_custom_dimensions_get {
                call_result = self._management_custom_dimensions_get(dry_run, &mut err);
            } else if self.opt.cmd_custom_dimensions_insert {
                call_result = self._management_custom_dimensions_insert(dry_run, &mut err);
            } else if self.opt.cmd_custom_dimensions_list {
                call_result = self._management_custom_dimensions_list(dry_run, &mut err);
            } else if self.opt.cmd_custom_dimensions_patch {
                call_result = self._management_custom_dimensions_patch(dry_run, &mut err);
            } else if self.opt.cmd_custom_dimensions_update {
                call_result = self._management_custom_dimensions_update(dry_run, &mut err);
            } else if self.opt.cmd_custom_metrics_get {
                call_result = self._management_custom_metrics_get(dry_run, &mut err);
            } else if self.opt.cmd_custom_metrics_insert {
                call_result = self._management_custom_metrics_insert(dry_run, &mut err);
            } else if self.opt.cmd_custom_metrics_list {
                call_result = self._management_custom_metrics_list(dry_run, &mut err);
            } else if self.opt.cmd_custom_metrics_patch {
                call_result = self._management_custom_metrics_patch(dry_run, &mut err);
            } else if self.opt.cmd_custom_metrics_update {
                call_result = self._management_custom_metrics_update(dry_run, &mut err);
            } else if self.opt.cmd_experiments_delete {
                call_result = self._management_experiments_delete(dry_run, &mut err);
            } else if self.opt.cmd_experiments_get {
                call_result = self._management_experiments_get(dry_run, &mut err);
            } else if self.opt.cmd_experiments_insert {
                call_result = self._management_experiments_insert(dry_run, &mut err);
            } else if self.opt.cmd_experiments_list {
                call_result = self._management_experiments_list(dry_run, &mut err);
            } else if self.opt.cmd_experiments_patch {
                call_result = self._management_experiments_patch(dry_run, &mut err);
            } else if self.opt.cmd_experiments_update {
                call_result = self._management_experiments_update(dry_run, &mut err);
            } else if self.opt.cmd_filters_delete {
                call_result = self._management_filters_delete(dry_run, &mut err);
            } else if self.opt.cmd_filters_get {
                call_result = self._management_filters_get(dry_run, &mut err);
            } else if self.opt.cmd_filters_insert {
                call_result = self._management_filters_insert(dry_run, &mut err);
            } else if self.opt.cmd_filters_list {
                call_result = self._management_filters_list(dry_run, &mut err);
            } else if self.opt.cmd_filters_patch {
                call_result = self._management_filters_patch(dry_run, &mut err);
            } else if self.opt.cmd_filters_update {
                call_result = self._management_filters_update(dry_run, &mut err);
            } else if self.opt.cmd_goals_get {
                call_result = self._management_goals_get(dry_run, &mut err);
            } else if self.opt.cmd_goals_insert {
                call_result = self._management_goals_insert(dry_run, &mut err);
            } else if self.opt.cmd_goals_list {
                call_result = self._management_goals_list(dry_run, &mut err);
            } else if self.opt.cmd_goals_patch {
                call_result = self._management_goals_patch(dry_run, &mut err);
            } else if self.opt.cmd_goals_update {
                call_result = self._management_goals_update(dry_run, &mut err);
            } else if self.opt.cmd_profile_filter_links_delete {
                call_result = self._management_profile_filter_links_delete(dry_run, &mut err);
            } else if self.opt.cmd_profile_filter_links_get {
                call_result = self._management_profile_filter_links_get(dry_run, &mut err);
            } else if self.opt.cmd_profile_filter_links_insert {
                call_result = self._management_profile_filter_links_insert(dry_run, &mut err);
            } else if self.opt.cmd_profile_filter_links_list {
                call_result = self._management_profile_filter_links_list(dry_run, &mut err);
            } else if self.opt.cmd_profile_filter_links_patch {
                call_result = self._management_profile_filter_links_patch(dry_run, &mut err);
            } else if self.opt.cmd_profile_filter_links_update {
                call_result = self._management_profile_filter_links_update(dry_run, &mut err);
            } else if self.opt.cmd_profile_user_links_delete {
                call_result = self._management_profile_user_links_delete(dry_run, &mut err);
            } else if self.opt.cmd_profile_user_links_insert {
                call_result = self._management_profile_user_links_insert(dry_run, &mut err);
            } else if self.opt.cmd_profile_user_links_list {
                call_result = self._management_profile_user_links_list(dry_run, &mut err);
            } else if self.opt.cmd_profile_user_links_update {
                call_result = self._management_profile_user_links_update(dry_run, &mut err);
            } else if self.opt.cmd_profiles_delete {
                call_result = self._management_profiles_delete(dry_run, &mut err);
            } else if self.opt.cmd_profiles_get {
                call_result = self._management_profiles_get(dry_run, &mut err);
            } else if self.opt.cmd_profiles_insert {
                call_result = self._management_profiles_insert(dry_run, &mut err);
            } else if self.opt.cmd_profiles_list {
                call_result = self._management_profiles_list(dry_run, &mut err);
            } else if self.opt.cmd_profiles_patch {
                call_result = self._management_profiles_patch(dry_run, &mut err);
            } else if self.opt.cmd_profiles_update {
                call_result = self._management_profiles_update(dry_run, &mut err);
            } else if self.opt.cmd_segments_list {
                call_result = self._management_segments_list(dry_run, &mut err);
            } else if self.opt.cmd_unsampled_reports_get {
                call_result = self._management_unsampled_reports_get(dry_run, &mut err);
            } else if self.opt.cmd_unsampled_reports_insert {
                call_result = self._management_unsampled_reports_insert(dry_run, &mut err);
            } else if self.opt.cmd_unsampled_reports_list {
                call_result = self._management_unsampled_reports_list(dry_run, &mut err);
            } else if self.opt.cmd_uploads_delete_upload_data {
                call_result = self._management_uploads_delete_upload_data(dry_run, &mut err);
            } else if self.opt.cmd_uploads_get {
                call_result = self._management_uploads_get(dry_run, &mut err);
            } else if self.opt.cmd_uploads_list {
                call_result = self._management_uploads_list(dry_run, &mut err);
            } else if self.opt.cmd_uploads_upload_data {
                call_result = self._management_uploads_upload_data(dry_run, &mut err);
            } else if self.opt.cmd_web_property_ad_words_links_delete {
                call_result = self._management_web_property_ad_words_links_delete(dry_run, &mut err);
            } else if self.opt.cmd_web_property_ad_words_links_get {
                call_result = self._management_web_property_ad_words_links_get(dry_run, &mut err);
            } else if self.opt.cmd_web_property_ad_words_links_insert {
                call_result = self._management_web_property_ad_words_links_insert(dry_run, &mut err);
            } else if self.opt.cmd_web_property_ad_words_links_list {
                call_result = self._management_web_property_ad_words_links_list(dry_run, &mut err);
            } else if self.opt.cmd_web_property_ad_words_links_patch {
                call_result = self._management_web_property_ad_words_links_patch(dry_run, &mut err);
            } else if self.opt.cmd_web_property_ad_words_links_update {
                call_result = self._management_web_property_ad_words_links_update(dry_run, &mut err);
            } else if self.opt.cmd_webproperties_get {
                call_result = self._management_webproperties_get(dry_run, &mut err);
            } else if self.opt.cmd_webproperties_insert {
                call_result = self._management_webproperties_insert(dry_run, &mut err);
            } else if self.opt.cmd_webproperties_list {
                call_result = self._management_webproperties_list(dry_run, &mut err);
            } else if self.opt.cmd_webproperties_patch {
                call_result = self._management_webproperties_patch(dry_run, &mut err);
            } else if self.opt.cmd_webproperties_update {
                call_result = self._management_webproperties_update(dry_run, &mut err);
            } else if self.opt.cmd_webproperty_user_links_delete {
                call_result = self._management_webproperty_user_links_delete(dry_run, &mut err);
            } else if self.opt.cmd_webproperty_user_links_insert {
                call_result = self._management_webproperty_user_links_insert(dry_run, &mut err);
            } else if self.opt.cmd_webproperty_user_links_list {
                call_result = self._management_webproperty_user_links_list(dry_run, &mut err);
            } else if self.opt.cmd_webproperty_user_links_update {
                call_result = self._management_webproperty_user_links_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_metadata {
            if self.opt.cmd_columns_list {
                call_result = self._metadata_columns_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_provisioning {
            if self.opt.cmd_create_account_ticket {
                call_result = self._provisioning_create_account_ticket(dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "analytics3-secret.json", 
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
                                          program_name: "analytics3",
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
            hub: api::Analytics::new(client, auth),
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