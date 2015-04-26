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
extern crate google_urlshortener1 as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  urlshortener1 [options] url get <short-url> [-p <v>...] [-o <out>]
  urlshortener1 [options] url insert -r <kv>... [-p <v>...] [-o <out>]
  urlshortener1 [options] url list [-p <v>...] [-o <out>]
  urlshortener1 --help

All documentation details can be found at
http://byron.github.io/google-apis-rs/google_urlshortener1_cli/index.html

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
    hub: api::Urlshortener<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _url_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.url().get(&self.opt.arg_short_url);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
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

    fn _url_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Url::default();
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
            fn request_analytics_all_time_init(request: &mut api::Url) {
                request_analytics_init(request);
                if request.analytics.as_mut().unwrap().all_time.is_none() {
                    request.analytics.as_mut().unwrap().all_time = Some(Default::default());
                }
            }
            
            fn request_analytics_day_init(request: &mut api::Url) {
                request_analytics_init(request);
                if request.analytics.as_mut().unwrap().day.is_none() {
                    request.analytics.as_mut().unwrap().day = Some(Default::default());
                }
            }
            
            fn request_analytics_init(request: &mut api::Url) {
                if request.analytics.is_none() {
                    request.analytics = Some(Default::default());
                }
            }
            
            fn request_analytics_month_init(request: &mut api::Url) {
                request_analytics_init(request);
                if request.analytics.as_mut().unwrap().month.is_none() {
                    request.analytics.as_mut().unwrap().month = Some(Default::default());
                }
            }
            
            fn request_analytics_two_hours_init(request: &mut api::Url) {
                request_analytics_init(request);
                if request.analytics.as_mut().unwrap().two_hours.is_none() {
                    request.analytics.as_mut().unwrap().two_hours = Some(Default::default());
                }
            }
            
            fn request_analytics_week_init(request: &mut api::Url) {
                request_analytics_init(request);
                if request.analytics.as_mut().unwrap().week.is_none() {
                    request.analytics.as_mut().unwrap().week = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "status" => {
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "analytics.week.short-url-clicks" => {
                        request_analytics_week_init(&mut request);
                        request.analytics.as_mut().unwrap().week.as_mut().unwrap().short_url_clicks = Some(value.unwrap_or("").to_string());
                    },
                "analytics.week.long-url-clicks" => {
                        request_analytics_week_init(&mut request);
                        request.analytics.as_mut().unwrap().week.as_mut().unwrap().long_url_clicks = Some(value.unwrap_or("").to_string());
                    },
                "analytics.all-time.short-url-clicks" => {
                        request_analytics_all_time_init(&mut request);
                        request.analytics.as_mut().unwrap().all_time.as_mut().unwrap().short_url_clicks = Some(value.unwrap_or("").to_string());
                    },
                "analytics.all-time.long-url-clicks" => {
                        request_analytics_all_time_init(&mut request);
                        request.analytics.as_mut().unwrap().all_time.as_mut().unwrap().long_url_clicks = Some(value.unwrap_or("").to_string());
                    },
                "analytics.two-hours.short-url-clicks" => {
                        request_analytics_two_hours_init(&mut request);
                        request.analytics.as_mut().unwrap().two_hours.as_mut().unwrap().short_url_clicks = Some(value.unwrap_or("").to_string());
                    },
                "analytics.two-hours.long-url-clicks" => {
                        request_analytics_two_hours_init(&mut request);
                        request.analytics.as_mut().unwrap().two_hours.as_mut().unwrap().long_url_clicks = Some(value.unwrap_or("").to_string());
                    },
                "analytics.day.short-url-clicks" => {
                        request_analytics_day_init(&mut request);
                        request.analytics.as_mut().unwrap().day.as_mut().unwrap().short_url_clicks = Some(value.unwrap_or("").to_string());
                    },
                "analytics.day.long-url-clicks" => {
                        request_analytics_day_init(&mut request);
                        request.analytics.as_mut().unwrap().day.as_mut().unwrap().long_url_clicks = Some(value.unwrap_or("").to_string());
                    },
                "analytics.month.short-url-clicks" => {
                        request_analytics_month_init(&mut request);
                        request.analytics.as_mut().unwrap().month.as_mut().unwrap().short_url_clicks = Some(value.unwrap_or("").to_string());
                    },
                "analytics.month.long-url-clicks" => {
                        request_analytics_month_init(&mut request);
                        request.analytics.as_mut().unwrap().month.as_mut().unwrap().long_url_clicks = Some(value.unwrap_or("").to_string());
                    },
                "long-url" => {
                        request_analytics_init(&mut request);
                        request.long_url = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_analytics_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.url().insert(request);
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

    fn _url_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.url().list();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-token" => {
                    call = call.start_token(value.unwrap_or(""));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
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

        if self.opt.cmd_url {
            if self.opt.cmd_get {
                call_result = self._url_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._url_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._url_list(dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "urlshortener1-secret.json", 
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
                                          program_name: "urlshortener1",
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
            hub: api::Urlshortener::new(client, auth),
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