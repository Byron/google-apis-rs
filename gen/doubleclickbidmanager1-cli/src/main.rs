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
extern crate google_doubleclickbidmanager1 as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  doubleclickbidmanager1 [options] lineitems downloadlineitems -r <kv>... [-p <v>...] [-o <out>]
  doubleclickbidmanager1 [options] lineitems uploadlineitems -r <kv>... [-p <v>...] [-o <out>]
  doubleclickbidmanager1 [options] queries createquery -r <kv>... [-p <v>...] [-o <out>]
  doubleclickbidmanager1 [options] queries deletequery <query-id> [-p <v>...]
  doubleclickbidmanager1 [options] queries getquery <query-id> [-p <v>...] [-o <out>]
  doubleclickbidmanager1 [options] queries listqueries [-p <v>...] [-o <out>]
  doubleclickbidmanager1 [options] queries runquery <query-id> -r <kv>... [-p <v>...]
  doubleclickbidmanager1 [options] reports listreports <query-id> [-p <v>...] [-o <out>]
  doubleclickbidmanager1 --help

All documentation details can be found at
http://byron.github.io/google-apis-rs/google_doubleclickbidmanager1_cli/index.html

Configuration:
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
    hub: api::DoubleClickBidManager<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _lineitems_downloadlineitems(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::DownloadLineItemsRequest::default();
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
                "filter-type" => {
                        request.filter_type = Some(value.unwrap_or("").to_string());
                    },
                "filter-ids" => {
                        if request.filter_ids.is_none() {
                           request.filter_ids = Some(Default::default());
                        }
                                        request.filter_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "format" => {
                        request.format = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.lineitems().downloadlineitems(request);
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

    fn _lineitems_uploadlineitems(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::UploadLineItemsRequest::default();
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
                "line-items" => {
                        request.line_items = Some(value.unwrap_or("").to_string());
                    },
                "dry-run" => {
                        request.dry_run = Some(arg_from_str(value.unwrap_or("false"), err, "dry-run", "boolean"));
                    },
                "format" => {
                        request.format = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.lineitems().uploadlineitems(request);
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

    fn _queries_createquery(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Query::default();
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
            fn request_metadata_init(request: &mut api::Query) {
                if request.metadata.is_none() {
                    request.metadata = Some(Default::default());
                }
            }
            
            fn request_params_init(request: &mut api::Query) {
                if request.params.is_none() {
                    request.params = Some(Default::default());
                }
            }
            
            fn request_schedule_init(request: &mut api::Query) {
                if request.schedule.is_none() {
                    request.schedule = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "schedule.end-time-ms" => {
                        request_schedule_init(&mut request);
                        request.schedule.as_mut().unwrap().end_time_ms = Some(value.unwrap_or("").to_string());
                    },
                "schedule.next-run-timezone-code" => {
                        request_schedule_init(&mut request);
                        request.schedule.as_mut().unwrap().next_run_timezone_code = Some(value.unwrap_or("").to_string());
                    },
                "schedule.frequency" => {
                        request_schedule_init(&mut request);
                        request.schedule.as_mut().unwrap().frequency = Some(value.unwrap_or("").to_string());
                    },
                "schedule.next-run-minute-of-day" => {
                        request_schedule_init(&mut request);
                        request.schedule.as_mut().unwrap().next_run_minute_of_day = Some(arg_from_str(value.unwrap_or("-0"), err, "schedule.next-run-minute-of-day", "integer"));
                    },
                "timezone-code" => {
                        request_schedule_init(&mut request);
                        request.timezone_code = Some(value.unwrap_or("").to_string());
                    },
                "report-data-end-time-ms" => {
                        request_schedule_init(&mut request);
                        request.report_data_end_time_ms = Some(value.unwrap_or("").to_string());
                    },
                "query-id" => {
                        request_schedule_init(&mut request);
                        request.query_id = Some(value.unwrap_or("").to_string());
                    },
                "params.metrics" => {
                        request_params_init(&mut request);
                        if request.params.as_mut().unwrap().metrics.is_none() {
                           request.params.as_mut().unwrap().metrics = Some(Default::default());
                        }
                                        request.params.as_mut().unwrap().metrics.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "params.type" => {
                        request_params_init(&mut request);
                        request.params.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "params.group-bys" => {
                        request_params_init(&mut request);
                        if request.params.as_mut().unwrap().group_bys.is_none() {
                           request.params.as_mut().unwrap().group_bys = Some(Default::default());
                        }
                                        request.params.as_mut().unwrap().group_bys.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "params.include-invite-data" => {
                        request_params_init(&mut request);
                        request.params.as_mut().unwrap().include_invite_data = Some(arg_from_str(value.unwrap_or("false"), err, "params.include-invite-data", "boolean"));
                    },
                "report-data-start-time-ms" => {
                        request_params_init(&mut request);
                        request.report_data_start_time_ms = Some(value.unwrap_or("").to_string());
                    },
                "metadata.google-cloud-storage-path-for-latest-report" => {
                        request_metadata_init(&mut request);
                        request.metadata.as_mut().unwrap().google_cloud_storage_path_for_latest_report = Some(value.unwrap_or("").to_string());
                    },
                "metadata.data-range" => {
                        request_metadata_init(&mut request);
                        request.metadata.as_mut().unwrap().data_range = Some(value.unwrap_or("").to_string());
                    },
                "metadata.format" => {
                        request_metadata_init(&mut request);
                        request.metadata.as_mut().unwrap().format = Some(value.unwrap_or("").to_string());
                    },
                "metadata.locale" => {
                        request_metadata_init(&mut request);
                        request.metadata.as_mut().unwrap().locale = Some(value.unwrap_or("").to_string());
                    },
                "metadata.google-drive-path-for-latest-report" => {
                        request_metadata_init(&mut request);
                        request.metadata.as_mut().unwrap().google_drive_path_for_latest_report = Some(value.unwrap_or("").to_string());
                    },
                "metadata.send-notification" => {
                        request_metadata_init(&mut request);
                        request.metadata.as_mut().unwrap().send_notification = Some(arg_from_str(value.unwrap_or("false"), err, "metadata.send-notification", "boolean"));
                    },
                "metadata.share-email-address" => {
                        request_metadata_init(&mut request);
                        if request.metadata.as_mut().unwrap().share_email_address.is_none() {
                           request.metadata.as_mut().unwrap().share_email_address = Some(Default::default());
                        }
                                        request.metadata.as_mut().unwrap().share_email_address.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "metadata.report-count" => {
                        request_metadata_init(&mut request);
                        request.metadata.as_mut().unwrap().report_count = Some(arg_from_str(value.unwrap_or("-0"), err, "metadata.report-count", "integer"));
                    },
                "metadata.running" => {
                        request_metadata_init(&mut request);
                        request.metadata.as_mut().unwrap().running = Some(arg_from_str(value.unwrap_or("false"), err, "metadata.running", "boolean"));
                    },
                "metadata.latest-report-run-time-ms" => {
                        request_metadata_init(&mut request);
                        request.metadata.as_mut().unwrap().latest_report_run_time_ms = Some(value.unwrap_or("").to_string());
                    },
                "metadata.title" => {
                        request_metadata_init(&mut request);
                        request.metadata.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.queries().createquery(request);
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

    fn _queries_deletequery(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.queries().deletequery(&self.opt.arg_query_id);
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

    fn _queries_getquery(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.queries().getquery(&self.opt.arg_query_id);
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

    fn _queries_listqueries(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.queries().listqueries();
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

    fn _queries_runquery(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::RunQueryRequest::default();
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
                "report-data-end-time-ms" => {
                        request.report_data_end_time_ms = Some(value.unwrap_or("").to_string());
                    },
                "timezone-code" => {
                        request.timezone_code = Some(value.unwrap_or("").to_string());
                    },
                "report-data-start-time-ms" => {
                        request.report_data_start_time_ms = Some(value.unwrap_or("").to_string());
                    },
                "data-range" => {
                        request.data_range = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.queries().runquery(request, &self.opt.arg_query_id);
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

    fn _reports_listreports(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.reports().listreports(&self.opt.arg_query_id);
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

        if self.opt.cmd_lineitems {
            if self.opt.cmd_downloadlineitems {
                call_result = self._lineitems_downloadlineitems(dry_run, &mut err);
            } else if self.opt.cmd_uploadlineitems {
                call_result = self._lineitems_uploadlineitems(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_queries {
            if self.opt.cmd_createquery {
                call_result = self._queries_createquery(dry_run, &mut err);
            } else if self.opt.cmd_deletequery {
                call_result = self._queries_deletequery(dry_run, &mut err);
            } else if self.opt.cmd_getquery {
                call_result = self._queries_getquery(dry_run, &mut err);
            } else if self.opt.cmd_listqueries {
                call_result = self._queries_listqueries(dry_run, &mut err);
            } else if self.opt.cmd_runquery {
                call_result = self._queries_runquery(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_reports {
            if self.opt.cmd_listreports {
                call_result = self._reports_listreports(dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "doubleclickbidmanager1-secret.json", 
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
                                          program_name: "doubleclickbidmanager1",
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
            hub: api::DoubleClickBidManager::new(client, auth),
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