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
extern crate google_qpxexpress1 as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  qpxexpress1 [options] trips search -r <kv>... [-p <v>]... [-o <out>]
  qpxexpress1 --help

All documentation details can be found TODO: <URL to github.io docs here, see #51>

Configuration:
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
    hub: api::QPXExpress<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _trips_search(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::TripsSearchRequest = Default::default();
        let mut call = self.hub.trips().search(&request);
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
            fn request_request_init(request: &mut api::TripsSearchRequest) {
                if request.request.is_none() {
                    request.request = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "request.refundable" => {
                        request_request_init(&mut request);
                        request.request.as_mut().unwrap().refundable = arg_from_str(value.unwrap_or("false"), err, "request.refundable", "boolean");
                    },
                "request.passengers.kind" => {
                        request_request_init(&mut request);
                        request.request.as_mut().unwrap().passengers.kind = value.unwrap_or("").to_string();
                    },
                "request.passengers.infant-in-lap-count" => {
                        request_request_init(&mut request);
                        request.request.as_mut().unwrap().passengers.infant_in_lap_count = arg_from_str(value.unwrap_or("-0"), err, "request.passengers.infant-in-lap-count", "integer");
                    },
                "request.passengers.senior-count" => {
                        request_request_init(&mut request);
                        request.request.as_mut().unwrap().passengers.senior_count = arg_from_str(value.unwrap_or("-0"), err, "request.passengers.senior-count", "integer");
                    },
                "request.passengers.infant-in-seat-count" => {
                        request_request_init(&mut request);
                        request.request.as_mut().unwrap().passengers.infant_in_seat_count = arg_from_str(value.unwrap_or("-0"), err, "request.passengers.infant-in-seat-count", "integer");
                    },
                "request.passengers.child-count" => {
                        request_request_init(&mut request);
                        request.request.as_mut().unwrap().passengers.child_count = arg_from_str(value.unwrap_or("-0"), err, "request.passengers.child-count", "integer");
                    },
                "request.passengers.adult-count" => {
                        request_request_init(&mut request);
                        request.request.as_mut().unwrap().passengers.adult_count = arg_from_str(value.unwrap_or("-0"), err, "request.passengers.adult-count", "integer");
                    },
                "request.sale-country" => {
                        request_request_init(&mut request);
                        request.request.as_mut().unwrap().sale_country = arg_from_str(value.unwrap_or("-0"), err, "request.sale-country", "int64");
                    },
                "request.solutions" => {
                        request_request_init(&mut request);
                        request.request.as_mut().unwrap().solutions = arg_from_str(value.unwrap_or("-0"), err, "request.solutions", "integer");
                    },
                "request.max-price" => {
                        request_request_init(&mut request);
                        request.request.as_mut().unwrap().max_price = value.unwrap_or("").to_string();
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

    fn _doit(&self, dry_run: bool) -> (Option<api::Error>, Option<InvalidOptionsError>) {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Option<api::Error>;
        let mut err_opt: Option<InvalidOptionsError> = None;

        if self.opt.cmd_trips {
            if self.opt.cmd_search {
                call_result = self._trips_search(dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "qpxexpress1-secret.json") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                                      hyper::Client::new(),
                                      JsonTokenStorage {
                                        program_name: "qpxexpress1",
                                        db_dir: config_dir.clone(),
                                      }, None);
        let engine = Engine {
            opt: opt,
            hub: api::QPXExpress::new(hyper::Client::new(), auth),
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