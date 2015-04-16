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
extern crate google_youtubeanalytics1 as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  youtubeanalytics1 [options] batch-report-definitions list <on-behalf-of-content-owner> [-p <v>]... [-o <out>]
  youtubeanalytics1 [options] batch-reports list <batch-report-definition-id> <on-behalf-of-content-owner> [-p <v>]... [-o <out>]
  youtubeanalytics1 [options] group-items delete <id> [-p <v>]...
  youtubeanalytics1 [options] group-items insert -r <kv>... [-p <v>]... [-o <out>]
  youtubeanalytics1 [options] group-items list <group-id> [-p <v>]... [-o <out>]
  youtubeanalytics1 [options] groups delete <id> [-p <v>]...
  youtubeanalytics1 [options] groups insert -r <kv>... [-p <v>]... [-o <out>]
  youtubeanalytics1 [options] groups list [-p <v>]... [-o <out>]
  youtubeanalytics1 [options] groups update -r <kv>... [-p <v>]... [-o <out>]
  youtubeanalytics1 [options] reports query <ids> <start-date> <end-date> <metrics> [-p <v>]... [-o <out>]
  youtubeanalytics1 --help

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
    hub: api::YouTubeAnalytics<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _batch_report_definitions_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.batch_report_definitions().list(&self.opt.arg_on_behalf_of_content_owner);
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

    fn _batch_reports_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.batch_reports().list(&self.opt.arg_batch_report_definition_id, &self.opt.arg_on_behalf_of_content_owner);
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

    fn _group_items_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.group_items().delete(&self.opt.arg_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
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
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok(mut response) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    None
                }
            }
        }
    }

    fn _group_items_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::GroupItem = Default::default();
        let mut call = self.hub.group_items().insert(&request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
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
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_resource_init(request: &mut api::GroupItem) {
                if request.resource.is_none() {
                    request.resource = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "resource.kind" => {
                        request_resource_init(&mut request);
                        request.resource.as_mut().unwrap().kind = value.unwrap_or("").to_string();
                    },
                "resource.id" => {
                        request_resource_init(&mut request);
                        request.resource.as_mut().unwrap().id = value.unwrap_or("").to_string();
                    },
                "group-id" => {
                        request_resource_init(&mut request);
                        request.group_id = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_resource_init(&mut request);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _group_items_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.group_items().list(&self.opt.arg_group_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
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

    fn _groups_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.groups().delete(&self.opt.arg_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
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
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok(mut response) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    None
                }
            }
        }
    }

    fn _groups_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Group = Default::default();
        let mut call = self.hub.groups().insert(&request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
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
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_content_details_init(request: &mut api::Group) {
                if request.content_details.is_none() {
                    request.content_details = Some(Default::default());
                }
            }
            
            fn request_snippet_init(request: &mut api::Group) {
                if request.snippet.is_none() {
                    request.snippet = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "snippet.published-at" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().published_at = value.unwrap_or("").to_string();
                    },
                "snippet.title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().title = value.unwrap_or("").to_string();
                    },
                "content-details.item-count" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().item_count = arg_from_str(value.unwrap_or("-0"), err, "content-details.item-count", "int64");
                    },
                "content-details.item-type" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().item_type = value.unwrap_or("").to_string();
                    },
                "kind" => {
                        request_content_details_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_content_details_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_content_details_init(&mut request);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _groups_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.groups().list();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
                },
                "mine" => {
                    call = call.mine(arg_from_str(value.unwrap_or("false"), err, "mine", "boolean"));
                },
                "id" => {
                    call = call.id(value.unwrap_or(""));
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

    fn _groups_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Group = Default::default();
        let mut call = self.hub.groups().update(&request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "on-behalf-of-content-owner" => {
                    call = call.on_behalf_of_content_owner(value.unwrap_or(""));
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
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_content_details_init(request: &mut api::Group) {
                if request.content_details.is_none() {
                    request.content_details = Some(Default::default());
                }
            }
            
            fn request_snippet_init(request: &mut api::Group) {
                if request.snippet.is_none() {
                    request.snippet = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "snippet.published-at" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().published_at = value.unwrap_or("").to_string();
                    },
                "snippet.title" => {
                        request_snippet_init(&mut request);
                        request.snippet.as_mut().unwrap().title = value.unwrap_or("").to_string();
                    },
                "content-details.item-count" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().item_count = arg_from_str(value.unwrap_or("-0"), err, "content-details.item-count", "int64");
                    },
                "content-details.item-type" => {
                        request_content_details_init(&mut request);
                        request.content_details.as_mut().unwrap().item_type = value.unwrap_or("").to_string();
                    },
                "kind" => {
                        request_content_details_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_content_details_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_content_details_init(&mut request);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _reports_query(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.reports().query(&self.opt.arg_ids, &self.opt.arg_start_date, &self.opt.arg_end_date, &self.opt.arg_metrics);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
                },
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
                "currency" => {
                    call = call.currency(value.unwrap_or(""));
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

        if self.opt.cmd_batch_report_definitions {
            if self.opt.cmd_list {
                call_result = self._batch_report_definitions_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_batch_reports {
            if self.opt.cmd_list {
                call_result = self._batch_reports_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_group_items {
            if self.opt.cmd_delete {
                call_result = self._group_items_delete(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._group_items_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._group_items_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_groups {
            if self.opt.cmd_delete {
                call_result = self._groups_delete(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._groups_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._groups_list(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._groups_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_reports {
            if self.opt.cmd_query {
                call_result = self._reports_query(dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "youtubeanalytics1-secret.json") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                                      hyper::Client::new(),
                                      JsonTokenStorage {
                                        program_name: "youtubeanalytics1",
                                        db_dir: config_dir.clone(),
                                      }, None);
        let engine = Engine {
            opt: opt,
            hub: api::YouTubeAnalytics::new(hyper::Client::new(), auth),
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