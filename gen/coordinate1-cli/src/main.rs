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
extern crate google_coordinate1 as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  coordinate1 [options] custom-field-def list <team-id> [-p <v>]... [-o <out>]
  coordinate1 [options] jobs get <team-id> <job-id> [-p <v>]... [-o <out>]
  coordinate1 [options] jobs insert <team-id> <address> <lat> <lng> <title> -r <kv>... [-p <v>]... [-o <out>]
  coordinate1 [options] jobs list <team-id> [-p <v>]... [-o <out>]
  coordinate1 [options] jobs patch <team-id> <job-id> -r <kv>... [-p <v>]... [-o <out>]
  coordinate1 [options] jobs update <team-id> <job-id> -r <kv>... [-p <v>]... [-o <out>]
  coordinate1 [options] location list <team-id> <worker-email> <start-timestamp-ms> [-p <v>]... [-o <out>]
  coordinate1 [options] schedule get <team-id> <job-id> [-p <v>]... [-o <out>]
  coordinate1 [options] schedule patch <team-id> <job-id> -r <kv>... [-p <v>]... [-o <out>]
  coordinate1 [options] schedule update <team-id> <job-id> -r <kv>... [-p <v>]... [-o <out>]
  coordinate1 [options] team list [-p <v>]... [-o <out>]
  coordinate1 [options] worker list <team-id> [-p <v>]... [-o <out>]
  coordinate1 --help

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
    hub: api::Coordinate<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _custom_field_def_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.custom_field_def().list(&self.opt.arg_team_id);
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

    fn _jobs_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.jobs().get(&self.opt.arg_team_id, &self.opt.arg_job_id);
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

    fn _jobs_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Job = Default::default();
        let lat: f64 = arg_from_str(&self.opt.arg_lat, err, "<lat>", "number");
        let lng: f64 = arg_from_str(&self.opt.arg_lng, err, "<lng>", "number");
        let mut call = self.hub.jobs().insert(&request, &self.opt.arg_team_id, &self.opt.arg_address, lat, lng, &self.opt.arg_title);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "note" => {
                    call = call.note(value.unwrap_or(""));
                },
                "customer-phone-number" => {
                    call = call.customer_phone_number(value.unwrap_or(""));
                },
                "customer-name" => {
                    call = call.customer_name(value.unwrap_or(""));
                },
                "custom-field" => {
                    call = call.add_custom_field(value.unwrap_or(""));
                },
                "assignee" => {
                    call = call.assignee(value.unwrap_or(""));
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
            fn request_state_init(request: &mut api::Job) {
                if request.state.is_none() {
                    request.state = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "state.kind" => {
                        request_state_init(&mut request);
                        request.state.as_mut().unwrap().kind = value.unwrap_or("").to_string();
                    },
                "state.customer-name" => {
                        request_state_init(&mut request);
                        request.state.as_mut().unwrap().customer_name = value.unwrap_or("").to_string();
                    },
                "state.title" => {
                        request_state_init(&mut request);
                        request.state.as_mut().unwrap().title = value.unwrap_or("").to_string();
                    },
                "state.note" => {
                        request_state_init(&mut request);
                        request.state.as_mut().unwrap().note.push(value.unwrap_or("").to_string());
                    },
                "state.assignee" => {
                        request_state_init(&mut request);
                        request.state.as_mut().unwrap().assignee = value.unwrap_or("").to_string();
                    },
                "state.customer-phone-number" => {
                        request_state_init(&mut request);
                        request.state.as_mut().unwrap().customer_phone_number = value.unwrap_or("").to_string();
                    },
                "state.location.lat" => {
                        request_state_init(&mut request);
                        request.state.as_mut().unwrap().location.lat = arg_from_str(value.unwrap_or("0.0"), err, "state.location.lat", "number");
                    },
                "state.location.kind" => {
                        request_state_init(&mut request);
                        request.state.as_mut().unwrap().location.kind = value.unwrap_or("").to_string();
                    },
                "state.location.address-line" => {
                        request_state_init(&mut request);
                        request.state.as_mut().unwrap().location.address_line.push(value.unwrap_or("").to_string());
                    },
                "state.location.lng" => {
                        request_state_init(&mut request);
                        request.state.as_mut().unwrap().location.lng = arg_from_str(value.unwrap_or("0.0"), err, "state.location.lng", "number");
                    },
                "state.progress" => {
                        request_state_init(&mut request);
                        request.state.as_mut().unwrap().progress = value.unwrap_or("").to_string();
                    },
                "state.custom-fields.kind" => {
                        request_state_init(&mut request);
                        request.state.as_mut().unwrap().custom_fields.kind = value.unwrap_or("").to_string();
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

    fn _jobs_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.jobs().list(&self.opt.arg_team_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "min-modified-timestamp-ms" => {
                    call = call.min_modified_timestamp_ms(value.unwrap_or(""));
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _jobs_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Job = Default::default();
        let mut call = self.hub.jobs().patch(&request, &self.opt.arg_team_id, &self.opt.arg_job_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "title" => {
                    call = call.title(value.unwrap_or(""));
                },
                "progress" => {
                    call = call.progress(value.unwrap_or(""));
                },
                "note" => {
                    call = call.note(value.unwrap_or(""));
                },
                "lng" => {
                    call = call.lng(arg_from_str(value.unwrap_or("0.0"), err, "lng", "number"));
                },
                "lat" => {
                    call = call.lat(arg_from_str(value.unwrap_or("0.0"), err, "lat", "number"));
                },
                "customer-phone-number" => {
                    call = call.customer_phone_number(value.unwrap_or(""));
                },
                "customer-name" => {
                    call = call.customer_name(value.unwrap_or(""));
                },
                "custom-field" => {
                    call = call.add_custom_field(value.unwrap_or(""));
                },
                "assignee" => {
                    call = call.assignee(value.unwrap_or(""));
                },
                "address" => {
                    call = call.address(value.unwrap_or(""));
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
            fn request_state_init(request: &mut api::Job) {
                if request.state.is_none() {
                    request.state = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "state.kind" => {
                        request_state_init(&mut request);
                        request.state.as_mut().unwrap().kind = value.unwrap_or("").to_string();
                    },
                "state.customer-name" => {
                        request_state_init(&mut request);
                        request.state.as_mut().unwrap().customer_name = value.unwrap_or("").to_string();
                    },
                "state.title" => {
                        request_state_init(&mut request);
                        request.state.as_mut().unwrap().title = value.unwrap_or("").to_string();
                    },
                "state.note" => {
                        request_state_init(&mut request);
                        request.state.as_mut().unwrap().note.push(value.unwrap_or("").to_string());
                    },
                "state.assignee" => {
                        request_state_init(&mut request);
                        request.state.as_mut().unwrap().assignee = value.unwrap_or("").to_string();
                    },
                "state.customer-phone-number" => {
                        request_state_init(&mut request);
                        request.state.as_mut().unwrap().customer_phone_number = value.unwrap_or("").to_string();
                    },
                "state.location.lat" => {
                        request_state_init(&mut request);
                        request.state.as_mut().unwrap().location.lat = arg_from_str(value.unwrap_or("0.0"), err, "state.location.lat", "number");
                    },
                "state.location.kind" => {
                        request_state_init(&mut request);
                        request.state.as_mut().unwrap().location.kind = value.unwrap_or("").to_string();
                    },
                "state.location.address-line" => {
                        request_state_init(&mut request);
                        request.state.as_mut().unwrap().location.address_line.push(value.unwrap_or("").to_string());
                    },
                "state.location.lng" => {
                        request_state_init(&mut request);
                        request.state.as_mut().unwrap().location.lng = arg_from_str(value.unwrap_or("0.0"), err, "state.location.lng", "number");
                    },
                "state.progress" => {
                        request_state_init(&mut request);
                        request.state.as_mut().unwrap().progress = value.unwrap_or("").to_string();
                    },
                "state.custom-fields.kind" => {
                        request_state_init(&mut request);
                        request.state.as_mut().unwrap().custom_fields.kind = value.unwrap_or("").to_string();
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

    fn _jobs_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Job = Default::default();
        let mut call = self.hub.jobs().update(&request, &self.opt.arg_team_id, &self.opt.arg_job_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "title" => {
                    call = call.title(value.unwrap_or(""));
                },
                "progress" => {
                    call = call.progress(value.unwrap_or(""));
                },
                "note" => {
                    call = call.note(value.unwrap_or(""));
                },
                "lng" => {
                    call = call.lng(arg_from_str(value.unwrap_or("0.0"), err, "lng", "number"));
                },
                "lat" => {
                    call = call.lat(arg_from_str(value.unwrap_or("0.0"), err, "lat", "number"));
                },
                "customer-phone-number" => {
                    call = call.customer_phone_number(value.unwrap_or(""));
                },
                "customer-name" => {
                    call = call.customer_name(value.unwrap_or(""));
                },
                "custom-field" => {
                    call = call.add_custom_field(value.unwrap_or(""));
                },
                "assignee" => {
                    call = call.assignee(value.unwrap_or(""));
                },
                "address" => {
                    call = call.address(value.unwrap_or(""));
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
            fn request_state_init(request: &mut api::Job) {
                if request.state.is_none() {
                    request.state = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "state.kind" => {
                        request_state_init(&mut request);
                        request.state.as_mut().unwrap().kind = value.unwrap_or("").to_string();
                    },
                "state.customer-name" => {
                        request_state_init(&mut request);
                        request.state.as_mut().unwrap().customer_name = value.unwrap_or("").to_string();
                    },
                "state.title" => {
                        request_state_init(&mut request);
                        request.state.as_mut().unwrap().title = value.unwrap_or("").to_string();
                    },
                "state.note" => {
                        request_state_init(&mut request);
                        request.state.as_mut().unwrap().note.push(value.unwrap_or("").to_string());
                    },
                "state.assignee" => {
                        request_state_init(&mut request);
                        request.state.as_mut().unwrap().assignee = value.unwrap_or("").to_string();
                    },
                "state.customer-phone-number" => {
                        request_state_init(&mut request);
                        request.state.as_mut().unwrap().customer_phone_number = value.unwrap_or("").to_string();
                    },
                "state.location.lat" => {
                        request_state_init(&mut request);
                        request.state.as_mut().unwrap().location.lat = arg_from_str(value.unwrap_or("0.0"), err, "state.location.lat", "number");
                    },
                "state.location.kind" => {
                        request_state_init(&mut request);
                        request.state.as_mut().unwrap().location.kind = value.unwrap_or("").to_string();
                    },
                "state.location.address-line" => {
                        request_state_init(&mut request);
                        request.state.as_mut().unwrap().location.address_line.push(value.unwrap_or("").to_string());
                    },
                "state.location.lng" => {
                        request_state_init(&mut request);
                        request.state.as_mut().unwrap().location.lng = arg_from_str(value.unwrap_or("0.0"), err, "state.location.lng", "number");
                    },
                "state.progress" => {
                        request_state_init(&mut request);
                        request.state.as_mut().unwrap().progress = value.unwrap_or("").to_string();
                    },
                "state.custom-fields.kind" => {
                        request_state_init(&mut request);
                        request.state.as_mut().unwrap().custom_fields.kind = value.unwrap_or("").to_string();
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

    fn _location_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.location().list(&self.opt.arg_team_id, &self.opt.arg_worker_email, &self.opt.arg_start_timestamp_ms);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _schedule_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.schedule().get(&self.opt.arg_team_id, &self.opt.arg_job_id);
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

    fn _schedule_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Schedule = Default::default();
        let mut call = self.hub.schedule().patch(&request, &self.opt.arg_team_id, &self.opt.arg_job_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "start-time" => {
                    call = call.start_time(value.unwrap_or(""));
                },
                "end-time" => {
                    call = call.end_time(value.unwrap_or(""));
                },
                "duration" => {
                    call = call.duration(value.unwrap_or(""));
                },
                "all-day" => {
                    call = call.all_day(arg_from_str(value.unwrap_or("false"), err, "all-day", "boolean"));
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
            match &field_name.to_string()[..] {
                "duration" => {
                        request.duration = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "all-day" => {
                        request.all_day = Some(arg_from_str(value.unwrap_or("false"), err, "all-day", "boolean"));
                    },
                "start-time" => {
                        request.start_time = Some(value.unwrap_or("").to_string());
                    },
                "end-time" => {
                        request.end_time = Some(value.unwrap_or("").to_string());
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

    fn _schedule_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Schedule = Default::default();
        let mut call = self.hub.schedule().update(&request, &self.opt.arg_team_id, &self.opt.arg_job_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "start-time" => {
                    call = call.start_time(value.unwrap_or(""));
                },
                "end-time" => {
                    call = call.end_time(value.unwrap_or(""));
                },
                "duration" => {
                    call = call.duration(value.unwrap_or(""));
                },
                "all-day" => {
                    call = call.all_day(arg_from_str(value.unwrap_or("false"), err, "all-day", "boolean"));
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
            match &field_name.to_string()[..] {
                "duration" => {
                        request.duration = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "all-day" => {
                        request.all_day = Some(arg_from_str(value.unwrap_or("false"), err, "all-day", "boolean"));
                    },
                "start-time" => {
                        request.start_time = Some(value.unwrap_or("").to_string());
                    },
                "end-time" => {
                        request.end_time = Some(value.unwrap_or("").to_string());
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

    fn _team_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.team().list();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "worker" => {
                    call = call.worker(arg_from_str(value.unwrap_or("false"), err, "worker", "boolean"));
                },
                "dispatcher" => {
                    call = call.dispatcher(arg_from_str(value.unwrap_or("false"), err, "dispatcher", "boolean"));
                },
                "admin" => {
                    call = call.admin(arg_from_str(value.unwrap_or("false"), err, "admin", "boolean"));
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

    fn _worker_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.worker().list(&self.opt.arg_team_id);
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

    fn _doit(&self, dry_run: bool) -> (Option<api::Error>, Option<InvalidOptionsError>) {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Option<api::Error>;
        let mut err_opt: Option<InvalidOptionsError> = None;

        if self.opt.cmd_custom_field_def {
            if self.opt.cmd_list {
                call_result = self._custom_field_def_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_jobs {
            if self.opt.cmd_get {
                call_result = self._jobs_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._jobs_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._jobs_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._jobs_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._jobs_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_location {
            if self.opt.cmd_list {
                call_result = self._location_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_schedule {
            if self.opt.cmd_get {
                call_result = self._schedule_get(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._schedule_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._schedule_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_team {
            if self.opt.cmd_list {
                call_result = self._team_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_worker {
            if self.opt.cmd_list {
                call_result = self._worker_list(dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "coordinate1-secret.json") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                                      hyper::Client::new(),
                                      JsonTokenStorage {
                                        program_name: "coordinate1",
                                        db_dir: config_dir.clone(),
                                      }, None);
        let engine = Engine {
            opt: opt,
            hub: api::Coordinate::new(hyper::Client::new(), auth),
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