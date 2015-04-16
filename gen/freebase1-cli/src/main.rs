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
extern crate google_freebase1 as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  freebase1 [options] methods reconcile [-p <v>]... [-o <out>]
  freebase1 [options] methods search [-p <v>]... [-o <out>]
  freebase1 --help

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
    hub: api::Freebase<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _methods_reconcile(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.methods().reconcile();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "prop" => {
                    call = call.add_prop(value.unwrap_or(""));
                },
                "name" => {
                    call = call.name(value.unwrap_or(""));
                },
                "limit" => {
                    call = call.limit(arg_from_str(value.unwrap_or("-0"), err, "limit", "integer"));
                },
                "lang" => {
                    call = call.add_lang(value.unwrap_or(""));
                },
                "kind" => {
                    call = call.add_kind(value.unwrap_or(""));
                },
                "confidence" => {
                    call = call.confidence(arg_from_str(value.unwrap_or("0.0"), err, "confidence", "number"));
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

    fn _methods_search(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut download_mode = false;
        let mut call = self.hub.methods().search();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "without" => {
                    call = call.add_without(value.unwrap_or(""));
                },
                "with" => {
                    call = call.add_with(value.unwrap_or(""));
                },
                "type" => {
                    call = call.add_type(value.unwrap_or(""));
                },
                "stemmed" => {
                    call = call.stemmed(arg_from_str(value.unwrap_or("false"), err, "stemmed", "boolean"));
                },
                "spell" => {
                    call = call.spell(value.unwrap_or(""));
                },
                "scoring" => {
                    call = call.scoring(value.unwrap_or(""));
                },
                "query" => {
                    call = call.query(value.unwrap_or(""));
                },
                "prefixed" => {
                    call = call.prefixed(arg_from_str(value.unwrap_or("false"), err, "prefixed", "boolean"));
                },
                "output" => {
                    call = call.output(value.unwrap_or(""));
                },
                "mql-output" => {
                    call = call.mql_output(value.unwrap_or(""));
                },
                "mid" => {
                    call = call.add_mid(value.unwrap_or(""));
                },
                "limit" => {
                    call = call.limit(arg_from_str(value.unwrap_or("-0"), err, "limit", "integer"));
                },
                "lang" => {
                    call = call.add_lang(value.unwrap_or(""));
                },
                "indent" => {
                    call = call.indent(arg_from_str(value.unwrap_or("false"), err, "indent", "boolean"));
                },
                "help" => {
                    call = call.help(value.unwrap_or(""));
                },
                "format" => {
                    call = call.format(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.add_filter(value.unwrap_or(""));
                },
                "exact" => {
                    call = call.exact(arg_from_str(value.unwrap_or("false"), err, "exact", "boolean"));
                },
                "encode" => {
                    call = call.encode(value.unwrap_or(""));
                },
                "domain" => {
                    call = call.add_domain(value.unwrap_or(""));
                },
                "cursor" => {
                    call = call.cursor(arg_from_str(value.unwrap_or("-0"), err, "cursor", "integer"));
                },
                "callback" => {
                    call = call.callback(value.unwrap_or(""));
                },
                "as-of-time" => {
                    call = call.as_of_time(value.unwrap_or(""));
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
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok(mut response) => {
                    println!("DEBUG: REMOVE ME {:?}", response);
                    if !download_mode {
                    } else {
                    io::copy(&mut response, &mut ostream).unwrap();
                    }
                    None
                }
            }
        }
    }

    fn _doit(&self, dry_run: bool) -> (Option<api::Error>, Option<InvalidOptionsError>) {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Option<api::Error>;
        let mut err_opt: Option<InvalidOptionsError> = None;

        if self.opt.cmd_methods {
            if self.opt.cmd_reconcile {
                call_result = self._methods_reconcile(dry_run, &mut err);
            } else if self.opt.cmd_search {
                call_result = self._methods_search(dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "freebase1-secret.json") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                                      hyper::Client::new(),
                                      JsonTokenStorage {
                                        program_name: "freebase1",
                                        db_dir: config_dir.clone(),
                                      }, None);
        let engine = Engine {
            opt: opt,
            hub: api::Freebase::new(hyper::Client::new(), auth),
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