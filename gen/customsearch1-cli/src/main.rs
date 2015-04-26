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
extern crate google_customsearch1 as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  customsearch1 [options] cse list <q> [-p <v>...] [-o <out>]
  customsearch1 --help

All documentation details can be found at
http://byron.github.io/google-apis-rs/google_customsearch1_cli/index.html

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
    hub: api::Customsearch<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _cse_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.cse().list(&self.opt.arg_q);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start" => {
                    call = call.start(arg_from_str(value.unwrap_or("-0"), err, "start", "integer"));
                },
                "sort" => {
                    call = call.sort(value.unwrap_or(""));
                },
                "site-search-filter" => {
                    call = call.site_search_filter(value.unwrap_or(""));
                },
                "site-search" => {
                    call = call.site_search(value.unwrap_or(""));
                },
                "search-type" => {
                    call = call.search_type(value.unwrap_or(""));
                },
                "safe" => {
                    call = call.safe(value.unwrap_or(""));
                },
                "rights" => {
                    call = call.rights(value.unwrap_or(""));
                },
                "related-site" => {
                    call = call.related_site(value.unwrap_or(""));
                },
                "or-terms" => {
                    call = call.or_terms(value.unwrap_or(""));
                },
                "num" => {
                    call = call.num(arg_from_str(value.unwrap_or("-0"), err, "num", "integer"));
                },
                "lr" => {
                    call = call.lr(value.unwrap_or(""));
                },
                "low-range" => {
                    call = call.low_range(value.unwrap_or(""));
                },
                "link-site" => {
                    call = call.link_site(value.unwrap_or(""));
                },
                "img-type" => {
                    call = call.img_type(value.unwrap_or(""));
                },
                "img-size" => {
                    call = call.img_size(value.unwrap_or(""));
                },
                "img-dominant-color" => {
                    call = call.img_dominant_color(value.unwrap_or(""));
                },
                "img-color-type" => {
                    call = call.img_color_type(value.unwrap_or(""));
                },
                "hq" => {
                    call = call.hq(value.unwrap_or(""));
                },
                "hl" => {
                    call = call.hl(value.unwrap_or(""));
                },
                "high-range" => {
                    call = call.high_range(value.unwrap_or(""));
                },
                "googlehost" => {
                    call = call.googlehost(value.unwrap_or(""));
                },
                "gl" => {
                    call = call.gl(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                "file-type" => {
                    call = call.file_type(value.unwrap_or(""));
                },
                "exclude-terms" => {
                    call = call.exclude_terms(value.unwrap_or(""));
                },
                "exact-terms" => {
                    call = call.exact_terms(value.unwrap_or(""));
                },
                "date-restrict" => {
                    call = call.date_restrict(value.unwrap_or(""));
                },
                "cx" => {
                    call = call.cx(value.unwrap_or(""));
                },
                "cref" => {
                    call = call.cref(value.unwrap_or(""));
                },
                "cr" => {
                    call = call.cr(value.unwrap_or(""));
                },
                "c2coff" => {
                    call = call.c2coff(value.unwrap_or(""));
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

    fn _doit(&self, dry_run: bool) -> (Option<api::Error>, Option<InvalidOptionsError>) {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Option<api::Error>;
        let mut err_opt: Option<InvalidOptionsError> = None;

        if self.opt.cmd_cse {
            if self.opt.cmd_list {
                call_result = self._cse_list(dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "customsearch1-secret.json", 
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
                                          program_name: "customsearch1",
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
            hub: api::Customsearch::new(client, auth),
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