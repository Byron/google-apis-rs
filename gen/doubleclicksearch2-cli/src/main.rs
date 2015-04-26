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
extern crate google_doubleclicksearch2 as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  doubleclicksearch2 [options] conversion get <agency-id> <advertiser-id> <engine-account-id> <end-date> <row-count> <start-date> <start-row> [-p <v>...] [-o <out>]
  doubleclicksearch2 [options] conversion insert -r <kv>... [-p <v>...] [-o <out>]
  doubleclicksearch2 [options] conversion patch <advertiser-id> <agency-id> <end-date> <engine-account-id> <row-count> <start-date> <start-row> -r <kv>... [-p <v>...] [-o <out>]
  doubleclicksearch2 [options] conversion update -r <kv>... [-p <v>...] [-o <out>]
  doubleclicksearch2 [options] conversion update-availability -r <kv>... [-p <v>...] [-o <out>]
  doubleclicksearch2 [options] reports generate -r <kv>... [-p <v>...] [-o <out>]
  doubleclicksearch2 [options] reports get <report-id> [-p <v>...] [-o <out>]
  doubleclicksearch2 [options] reports get-file <report-id> <report-fragment> [-p <v>...] [-o <out>]
  doubleclicksearch2 [options] reports request -r <kv>... [-p <v>...] [-o <out>]
  doubleclicksearch2 [options] saved-columns list <agency-id> <advertiser-id> [-p <v>...] [-o <out>]
  doubleclicksearch2 --help

All documentation details can be found at
http://byron.github.io/google-apis-rs/google_doubleclicksearch2_cli/index.html

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
    hub: api::Doubleclicksearch<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _conversion_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let end_date: i32 = arg_from_str(&self.opt.arg_end_date, err, "<end-date>", "integer");
        let row_count: i32 = arg_from_str(&self.opt.arg_row_count, err, "<row-count>", "integer");
        let start_date: i32 = arg_from_str(&self.opt.arg_start_date, err, "<start-date>", "integer");
        let start_row: u32 = arg_from_str(&self.opt.arg_start_row, err, "<start-row>", "integer");
        let mut call = self.hub.conversion().get(&self.opt.arg_agency_id, &self.opt.arg_advertiser_id, &self.opt.arg_engine_account_id, end_date, row_count, start_date, start_row);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "criterion-id" => {
                    call = call.criterion_id(value.unwrap_or(""));
                },
                "campaign-id" => {
                    call = call.campaign_id(value.unwrap_or(""));
                },
                "ad-id" => {
                    call = call.ad_id(value.unwrap_or(""));
                },
                "ad-group-id" => {
                    call = call.ad_group_id(value.unwrap_or(""));
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

    fn _conversion_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::ConversionList::default();
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
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.conversion().insert(request);
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

    fn _conversion_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::ConversionList::default();
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
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let end_date: i32 = arg_from_str(&self.opt.arg_end_date, err, "<end-date>", "integer");
        let row_count: i32 = arg_from_str(&self.opt.arg_row_count, err, "<row-count>", "integer");
        let start_date: i32 = arg_from_str(&self.opt.arg_start_date, err, "<start-date>", "integer");
        let start_row: u32 = arg_from_str(&self.opt.arg_start_row, err, "<start-row>", "integer");
        let mut call = self.hub.conversion().patch(request, &self.opt.arg_advertiser_id, &self.opt.arg_agency_id, end_date, &self.opt.arg_engine_account_id, row_count, start_date, start_row);
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

    fn _conversion_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::ConversionList::default();
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
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.conversion().update(request);
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

    fn _conversion_update_availability(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::UpdateAvailabilityRequest::default();
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
        let mut call = self.hub.conversion().update_availability(request);
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

    fn _reports_generate(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::ReportRequest::default();
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
            fn request_report_scope_init(request: &mut api::ReportRequest) {
                if request.report_scope.is_none() {
                    request.report_scope = Some(Default::default());
                }
            }
            
            fn request_time_range_init(request: &mut api::ReportRequest) {
                if request.time_range.is_none() {
                    request.time_range = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "report-scope.ad-group-id" => {
                        request_report_scope_init(&mut request);
                        request.report_scope.as_mut().unwrap().ad_group_id = Some(value.unwrap_or("").to_string());
                    },
                "report-scope.agency-id" => {
                        request_report_scope_init(&mut request);
                        request.report_scope.as_mut().unwrap().agency_id = Some(value.unwrap_or("").to_string());
                    },
                "report-scope.engine-account-id" => {
                        request_report_scope_init(&mut request);
                        request.report_scope.as_mut().unwrap().engine_account_id = Some(value.unwrap_or("").to_string());
                    },
                "report-scope.campaign-id" => {
                        request_report_scope_init(&mut request);
                        request.report_scope.as_mut().unwrap().campaign_id = Some(value.unwrap_or("").to_string());
                    },
                "report-scope.advertiser-id" => {
                        request_report_scope_init(&mut request);
                        request.report_scope.as_mut().unwrap().advertiser_id = Some(value.unwrap_or("").to_string());
                    },
                "report-scope.keyword-id" => {
                        request_report_scope_init(&mut request);
                        request.report_scope.as_mut().unwrap().keyword_id = Some(value.unwrap_or("").to_string());
                    },
                "report-scope.ad-id" => {
                        request_report_scope_init(&mut request);
                        request.report_scope.as_mut().unwrap().ad_id = Some(value.unwrap_or("").to_string());
                    },
                "max-rows-per-file" => {
                        request_report_scope_init(&mut request);
                        request.max_rows_per_file = Some(arg_from_str(value.unwrap_or("-0"), err, "max-rows-per-file", "integer"));
                    },
                "statistics-currency" => {
                        request_report_scope_init(&mut request);
                        request.statistics_currency = Some(value.unwrap_or("").to_string());
                    },
                "time-range.changed-metrics-since-timestamp" => {
                        request_time_range_init(&mut request);
                        request.time_range.as_mut().unwrap().changed_metrics_since_timestamp = Some(value.unwrap_or("").to_string());
                    },
                "time-range.end-date" => {
                        request_time_range_init(&mut request);
                        request.time_range.as_mut().unwrap().end_date = Some(value.unwrap_or("").to_string());
                    },
                "time-range.changed-attributes-since-timestamp" => {
                        request_time_range_init(&mut request);
                        request.time_range.as_mut().unwrap().changed_attributes_since_timestamp = Some(value.unwrap_or("").to_string());
                    },
                "time-range.start-date" => {
                        request_time_range_init(&mut request);
                        request.time_range.as_mut().unwrap().start_date = Some(value.unwrap_or("").to_string());
                    },
                "start-row" => {
                        request_time_range_init(&mut request);
                        request.start_row = Some(arg_from_str(value.unwrap_or("-0"), err, "start-row", "integer"));
                    },
                "row-count" => {
                        request_time_range_init(&mut request);
                        request.row_count = Some(arg_from_str(value.unwrap_or("-0"), err, "row-count", "integer"));
                    },
                "report-type" => {
                        request_time_range_init(&mut request);
                        request.report_type = Some(value.unwrap_or("").to_string());
                    },
                "download-format" => {
                        request_time_range_init(&mut request);
                        request.download_format = Some(value.unwrap_or("").to_string());
                    },
                "include-deleted-entities" => {
                        request_time_range_init(&mut request);
                        request.include_deleted_entities = Some(arg_from_str(value.unwrap_or("false"), err, "include-deleted-entities", "boolean"));
                    },
                "verify-single-time-zone" => {
                        request_time_range_init(&mut request);
                        request.verify_single_time_zone = Some(arg_from_str(value.unwrap_or("false"), err, "verify-single-time-zone", "boolean"));
                    },
                "include-removed-entities" => {
                        request_time_range_init(&mut request);
                        request.include_removed_entities = Some(arg_from_str(value.unwrap_or("false"), err, "include-removed-entities", "boolean"));
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.reports().generate(request);
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

    fn _reports_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.reports().get(&self.opt.arg_report_id);
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

    fn _reports_get_file(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let report_fragment: i32 = arg_from_str(&self.opt.arg_report_fragment, err, "<report-fragment>", "integer");
        let mut download_mode = false;
        let mut call = self.hub.reports().get_file(&self.opt.arg_report_id, report_fragment);
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

    fn _reports_request(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::ReportRequest::default();
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
            fn request_report_scope_init(request: &mut api::ReportRequest) {
                if request.report_scope.is_none() {
                    request.report_scope = Some(Default::default());
                }
            }
            
            fn request_time_range_init(request: &mut api::ReportRequest) {
                if request.time_range.is_none() {
                    request.time_range = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "report-scope.ad-group-id" => {
                        request_report_scope_init(&mut request);
                        request.report_scope.as_mut().unwrap().ad_group_id = Some(value.unwrap_or("").to_string());
                    },
                "report-scope.agency-id" => {
                        request_report_scope_init(&mut request);
                        request.report_scope.as_mut().unwrap().agency_id = Some(value.unwrap_or("").to_string());
                    },
                "report-scope.engine-account-id" => {
                        request_report_scope_init(&mut request);
                        request.report_scope.as_mut().unwrap().engine_account_id = Some(value.unwrap_or("").to_string());
                    },
                "report-scope.campaign-id" => {
                        request_report_scope_init(&mut request);
                        request.report_scope.as_mut().unwrap().campaign_id = Some(value.unwrap_or("").to_string());
                    },
                "report-scope.advertiser-id" => {
                        request_report_scope_init(&mut request);
                        request.report_scope.as_mut().unwrap().advertiser_id = Some(value.unwrap_or("").to_string());
                    },
                "report-scope.keyword-id" => {
                        request_report_scope_init(&mut request);
                        request.report_scope.as_mut().unwrap().keyword_id = Some(value.unwrap_or("").to_string());
                    },
                "report-scope.ad-id" => {
                        request_report_scope_init(&mut request);
                        request.report_scope.as_mut().unwrap().ad_id = Some(value.unwrap_or("").to_string());
                    },
                "max-rows-per-file" => {
                        request_report_scope_init(&mut request);
                        request.max_rows_per_file = Some(arg_from_str(value.unwrap_or("-0"), err, "max-rows-per-file", "integer"));
                    },
                "statistics-currency" => {
                        request_report_scope_init(&mut request);
                        request.statistics_currency = Some(value.unwrap_or("").to_string());
                    },
                "time-range.changed-metrics-since-timestamp" => {
                        request_time_range_init(&mut request);
                        request.time_range.as_mut().unwrap().changed_metrics_since_timestamp = Some(value.unwrap_or("").to_string());
                    },
                "time-range.end-date" => {
                        request_time_range_init(&mut request);
                        request.time_range.as_mut().unwrap().end_date = Some(value.unwrap_or("").to_string());
                    },
                "time-range.changed-attributes-since-timestamp" => {
                        request_time_range_init(&mut request);
                        request.time_range.as_mut().unwrap().changed_attributes_since_timestamp = Some(value.unwrap_or("").to_string());
                    },
                "time-range.start-date" => {
                        request_time_range_init(&mut request);
                        request.time_range.as_mut().unwrap().start_date = Some(value.unwrap_or("").to_string());
                    },
                "start-row" => {
                        request_time_range_init(&mut request);
                        request.start_row = Some(arg_from_str(value.unwrap_or("-0"), err, "start-row", "integer"));
                    },
                "row-count" => {
                        request_time_range_init(&mut request);
                        request.row_count = Some(arg_from_str(value.unwrap_or("-0"), err, "row-count", "integer"));
                    },
                "report-type" => {
                        request_time_range_init(&mut request);
                        request.report_type = Some(value.unwrap_or("").to_string());
                    },
                "download-format" => {
                        request_time_range_init(&mut request);
                        request.download_format = Some(value.unwrap_or("").to_string());
                    },
                "include-deleted-entities" => {
                        request_time_range_init(&mut request);
                        request.include_deleted_entities = Some(arg_from_str(value.unwrap_or("false"), err, "include-deleted-entities", "boolean"));
                    },
                "verify-single-time-zone" => {
                        request_time_range_init(&mut request);
                        request.verify_single_time_zone = Some(arg_from_str(value.unwrap_or("false"), err, "verify-single-time-zone", "boolean"));
                    },
                "include-removed-entities" => {
                        request_time_range_init(&mut request);
                        request.include_removed_entities = Some(arg_from_str(value.unwrap_or("false"), err, "include-removed-entities", "boolean"));
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.reports().request(request);
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

    fn _saved_columns_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.saved_columns().list(&self.opt.arg_agency_id, &self.opt.arg_advertiser_id);
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

        if self.opt.cmd_conversion {
            if self.opt.cmd_get {
                call_result = self._conversion_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._conversion_insert(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._conversion_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._conversion_update(dry_run, &mut err);
            } else if self.opt.cmd_update_availability {
                call_result = self._conversion_update_availability(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_reports {
            if self.opt.cmd_generate {
                call_result = self._reports_generate(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._reports_get(dry_run, &mut err);
            } else if self.opt.cmd_get_file {
                call_result = self._reports_get_file(dry_run, &mut err);
            } else if self.opt.cmd_request {
                call_result = self._reports_request(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_saved_columns {
            if self.opt.cmd_list {
                call_result = self._saved_columns_list(dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "doubleclicksearch2-secret.json", 
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
                                          program_name: "doubleclicksearch2",
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
            hub: api::Doubleclicksearch::new(client, auth),
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