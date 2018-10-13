// DO NOT EDIT !
// This file was generated automatically from 'src/mako/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;
extern crate yup_oauth2 as oauth2;
extern crate yup_hyper_mock as mock;
extern crate hyper_rustls;
extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate mime;
extern crate strsim;
extern crate google_customsearch1 as api;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

mod cmn;

use cmn::{InvalidOptionsError, CLIError, JsonTokenStorage, arg_from_str, writer_from_opts, parse_kv_arg,
          input_file_from_opts, input_mime_from_opts, FieldCursor, FieldError, CallType, UploadProtocol,
          calltype_from_str, remove_json_null_values, ComplexType, JsonType, JsonTypeInfo};

use std::default::Default;
use std::str::FromStr;

use oauth2::{Authenticator, DefaultAuthenticatorDelegate, FlowType};
use serde_json as json;
use clap::ArgMatches;

enum DoitError {
    IoError(String, io::Error),
    ApiError(api::Error),
}

struct Engine<'n> {
    opt: ArgMatches<'n>,
    hub: api::Customsearch<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n> Engine<'n> {
    fn _cse_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.cse().list(opt.value_of("q").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
                "cr" => {
                    call = call.cr(value.unwrap_or(""));
                },
                "c2coff" => {
                    call = call.c2coff(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["date-restrict", "or-terms", "high-range", "num", "cr", "img-type", "related-site", "gl", "search-type", "file-type", "start", "img-dominant-color", "lr", "site-search", "sort", "safe", "c2coff", "googlehost", "hq", "exact-terms", "hl", "low-range", "img-size", "img-color-type", "rights", "exclude-terms", "filter", "link-site", "cx", "site-search-filter"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _cse_siterestrict_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.cse().siterestrict_list(opt.value_of("q").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
                "cr" => {
                    call = call.cr(value.unwrap_or(""));
                },
                "c2coff" => {
                    call = call.c2coff(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["date-restrict", "or-terms", "high-range", "num", "cr", "img-type", "related-site", "gl", "search-type", "file-type", "start", "img-dominant-color", "lr", "site-search", "sort", "safe", "c2coff", "googlehost", "hq", "exact-terms", "hl", "low-range", "img-size", "img-color-type", "rights", "exclude-terms", "filter", "link-site", "cx", "site-search-filter"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _doit(&self, dry_run: bool) -> Result<Result<(), DoitError>, Option<InvalidOptionsError>> {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Result<(), DoitError> = Ok(());
        let mut err_opt: Option<InvalidOptionsError> = None;
        match self.opt.subcommand() {
            ("cse", Some(opt)) => {
                match opt.subcommand() {
                    ("list", Some(opt)) => {
                        call_result = self._cse_list(opt, dry_run, &mut err);
                    },
                    ("siterestrict-list", Some(opt)) => {
                        call_result = self._cse_siterestrict_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("cse".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            _ => {
                err.issues.push(CLIError::MissingCommandError);
                writeln!(io::stderr(), "{}\n", self.opt.usage()).ok();
            }
        }

        if dry_run {
            if err.issues.len() > 0 {
                err_opt = Some(err);
            }
            Err(err_opt)
        } else {
            Ok(call_result)
        }
    }

    // Please note that this call will fail if any part of the opt can't be handled
    fn new(opt: ArgMatches<'n>) -> Result<Engine<'n>, InvalidOptionsError> {
        let (config_dir, secret) = {
            let config_dir = match cmn::assure_config_dir_exists(opt.value_of("folder").unwrap_or("~/.google-service-cli")) {
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
                                        if opt.is_present("debug-auth") {
                                            hyper::Client::with_connector(mock::TeeConnector {
                                                    connector: hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())
                                                })
                                        } else {
                                            hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new()))
                                        },
                                        JsonTokenStorage {
                                          program_name: "customsearch1",
                                          db_dir: config_dir.clone(),
                                        }, Some(FlowType::InstalledRedirect(54324)));

        let client =
            if opt.is_present("debug") {
                hyper::Client::with_connector(mock::TeeConnector {
                        connector: hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())
                    })
            } else {
                hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new()))
            };
        let engine = Engine {
            opt: opt,
            hub: api::Customsearch::new(client, auth),
            gp: vec!["alt", "fields", "key", "oauth-token", "pretty-print", "quota-user", "user-ip"],
            gpm: vec![
                    ("oauth-token", "oauth_token"),
                    ("pretty-print", "prettyPrint"),
                    ("quota-user", "quotaUser"),
                    ("user-ip", "userIp"),
                ]
        };

        match engine._doit(true) {
            Err(Some(err)) => Err(err),
            Err(None)      => Ok(engine),
            Ok(_)          => unreachable!(),
        }
    }

    fn doit(&self) -> Result<(), DoitError> {
        match self._doit(false) {
            Ok(res) => res,
            Err(_) => unreachable!(),
        }
    }
}

fn main() {
    let mut exit_status = 0i32;
    let arg_data = [
        ("cse", "methods: 'list' and 'siterestrict-list'", vec![
            ("list",
                    Some(r##"Returns metadata about the search performed, metadata about the custom search engine used for the search, and the search results."##),
                    "Details at http://byron.github.io/google-apis-rs/google_customsearch1_cli/cse_list",
                  vec![
                    (Some(r##"q"##),
                     None,
                     Some(r##"Query"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("siterestrict-list",
                    Some(r##"Returns metadata about the search performed, metadata about the custom search engine used for the search, and the search results. Uses a small set of url patterns."##),
                    "Details at http://byron.github.io/google-apis-rs/google_customsearch1_cli/cse_siterestrict-list",
                  vec![
                    (Some(r##"q"##),
                     None,
                     Some(r##"Query"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
    ];
    
    let mut app = App::new("customsearch1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("1.0.7+20181001")
           .about("Searches over a website or collection of websites")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_customsearch1_cli")
           .arg(Arg::with_name("folder")
                   .long("config-dir")
                   .help("A directory into which we will store our persistent data. Defaults to a user-writable directory that we will create during the first invocation.[default: ~/.google-service-cli")
                   .multiple(false)
                   .takes_value(true))
           .arg(Arg::with_name("debug")
                   .long("debug")
                   .help("Output all server communication to standard error. `tx` and `rx` are placed into the same stream.")
                   .multiple(false)
                   .takes_value(false))
           .arg(Arg::with_name("debug-auth")
                   .long("debug-auth")
                   .help("Output all communication related to authentication to standard error. `tx` and `rx` are placed into the same stream.")
                   .multiple(false)
                   .takes_value(false));
           
           for &(main_command_name, about, ref subcommands) in arg_data.iter() {
               let mut mcmd = SubCommand::with_name(main_command_name).about(about);
           
               for &(sub_command_name, ref desc, url_info, ref args) in subcommands {
                   let mut scmd = SubCommand::with_name(sub_command_name);
                   if let &Some(desc) = desc {
                       scmd = scmd.about(desc);
                   }
                   scmd = scmd.after_help(url_info);
           
                   for &(ref arg_name, ref flag, ref desc, ref required, ref multi) in args {
                       let arg_name_str =
                           match (arg_name, flag) {
                                   (&Some(an), _       ) => an,
                                   (_        , &Some(f)) => f,
                                    _                    => unreachable!(),
                            };
                       let mut arg = Arg::with_name(arg_name_str)
                                         .empty_values(false);
                       if let &Some(short_flag) = flag {
                           arg = arg.short(short_flag);
                       }
                       if let &Some(desc) = desc {
                           arg = arg.help(desc);
                       }
                       if arg_name.is_some() && flag.is_some() {
                           arg = arg.takes_value(true);
                       }
                       if let &Some(required) = required {
                           arg = arg.required(required);
                       }
                       if let &Some(multi) = multi {
                           arg = arg.multiple(multi);
                       }
                       scmd = scmd.arg(arg);
                   }
                   mcmd = mcmd.subcommand(scmd);
               }
               app = app.subcommand(mcmd);
           }
           
        let matches = app.get_matches();

    let debug = matches.is_present("debug");
    match Engine::new(matches) {
        Err(err) => {
            exit_status = err.exit_code;
            writeln!(io::stderr(), "{}", err).ok();
        },
        Ok(engine) => {
            if let Err(doit_err) = engine.doit() {
                exit_status = 1;
                match doit_err {
                    DoitError::IoError(path, err) => {
                        writeln!(io::stderr(), "Failed to open output file '{}': {}", path, err).ok();
                    },
                    DoitError::ApiError(err) => {
                        if debug {
                            writeln!(io::stderr(), "{:#?}", err).ok();
                        } else {
                            writeln!(io::stderr(), "{}", err).ok();
                        }
                    }
                }
            }
        }
    }

    std::process::exit(exit_status);
}