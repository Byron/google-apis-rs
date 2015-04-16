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
extern crate google_cloudsearch1 as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  cloudsearch1 [options] projects indexes-documents-create <project-id> <index-id> -r <kv>... [-p <v>]... [-o <out>]
  cloudsearch1 [options] projects indexes-documents-delete <project-id> <index-id> <doc-id> [-p <v>]... [-o <out>]
  cloudsearch1 [options] projects indexes-documents-get <project-id> <index-id> <doc-id> [-p <v>]... [-o <out>]
  cloudsearch1 [options] projects indexes-documents-list <project-id> <index-id> <page-size> <page-token> <view> [-p <v>]... [-o <out>]
  cloudsearch1 [options] projects indexes-list <project-id> <index-name-prefix> <page-size> <page-token> <view> [-p <v>]... [-o <out>]
  cloudsearch1 [options] projects indexes-search <project-id> <index-id> <query> <field-expressions> <page-size> <page-token> <offset> <matched-count-accuracy> <order-by> <scorer> <scorer-size> <return-fields> [-p <v>]... [-o <out>]
  cloudsearch1 --help

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
    hub: api::Cloudsearch<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _projects_indexes_documents_create(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Document = Default::default();
        let mut call = self.hub.projects().indexes_documents_create(&request, &self.opt.arg_project_id, &self.opt.arg_index_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "$-xgafv"
                |"access-token"
                |"alt"
                |"bearer-token"
                |"callback"
                |"fields"
                |"key"
                |"oauth-token"
                |"pp"
                |"pretty-print"
                |"quota-user" => {
                    let map = [
                        ("$-xgafv", "$.xgafv"),
                        ("access-token", "access_token"),
                        ("bearer-token", "bearer_token"),
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
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
                "doc-id" => {
                        request.doc_id = Some(value.unwrap_or("").to_string());
                    },
                "rank" => {
                        request.rank = Some(arg_from_str(value.unwrap_or("-0"), err, "rank", "integer"));
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

    fn _projects_indexes_documents_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.projects().indexes_documents_delete(&self.opt.arg_project_id, &self.opt.arg_index_id, &self.opt.arg_doc_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "$-xgafv"
                |"access-token"
                |"alt"
                |"bearer-token"
                |"callback"
                |"fields"
                |"key"
                |"oauth-token"
                |"pp"
                |"pretty-print"
                |"quota-user" => {
                    let map = [
                        ("$-xgafv", "$.xgafv"),
                        ("access-token", "access_token"),
                        ("bearer-token", "bearer_token"),
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
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

    fn _projects_indexes_documents_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.projects().indexes_documents_get(&self.opt.arg_project_id, &self.opt.arg_index_id, &self.opt.arg_doc_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "$-xgafv"
                |"access-token"
                |"alt"
                |"bearer-token"
                |"callback"
                |"fields"
                |"key"
                |"oauth-token"
                |"pp"
                |"pretty-print"
                |"quota-user" => {
                    let map = [
                        ("$-xgafv", "$.xgafv"),
                        ("access-token", "access_token"),
                        ("bearer-token", "bearer_token"),
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
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

    fn _projects_indexes_documents_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let page_size: i32 = arg_from_str(&self.opt.arg_page_size, err, "<page-size>", "integer");
        let mut call = self.hub.projects().indexes_documents_list(&self.opt.arg_project_id, &self.opt.arg_index_id, page_size, &self.opt.arg_page_token, &self.opt.arg_view);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "$-xgafv"
                |"access-token"
                |"alt"
                |"bearer-token"
                |"callback"
                |"fields"
                |"key"
                |"oauth-token"
                |"pp"
                |"pretty-print"
                |"quota-user" => {
                    let map = [
                        ("$-xgafv", "$.xgafv"),
                        ("access-token", "access_token"),
                        ("bearer-token", "bearer_token"),
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
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

    fn _projects_indexes_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let page_size: i32 = arg_from_str(&self.opt.arg_page_size, err, "<page-size>", "integer");
        let mut call = self.hub.projects().indexes_list(&self.opt.arg_project_id, &self.opt.arg_index_name_prefix, page_size, &self.opt.arg_page_token, &self.opt.arg_view);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "$-xgafv"
                |"access-token"
                |"alt"
                |"bearer-token"
                |"callback"
                |"fields"
                |"key"
                |"oauth-token"
                |"pp"
                |"pretty-print"
                |"quota-user" => {
                    let map = [
                        ("$-xgafv", "$.xgafv"),
                        ("access-token", "access_token"),
                        ("bearer-token", "bearer_token"),
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
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

    fn _projects_indexes_search(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let page_size: i32 = arg_from_str(&self.opt.arg_page_size, err, "<page-size>", "integer");
        let offset: i32 = arg_from_str(&self.opt.arg_offset, err, "<offset>", "integer");
        let matched_count_accuracy: i32 = arg_from_str(&self.opt.arg_matched_count_accuracy, err, "<matched-count-accuracy>", "integer");
        let scorer_size: i32 = arg_from_str(&self.opt.arg_scorer_size, err, "<scorer-size>", "integer");
        let mut call = self.hub.projects().indexes_search(&self.opt.arg_project_id, &self.opt.arg_index_id, &self.opt.arg_query, &self.opt.arg_field_expressions, page_size, &self.opt.arg_page_token, offset, matched_count_accuracy, &self.opt.arg_order_by, &self.opt.arg_scorer, scorer_size, &self.opt.arg_return_fields);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "$-xgafv"
                |"access-token"
                |"alt"
                |"bearer-token"
                |"callback"
                |"fields"
                |"key"
                |"oauth-token"
                |"pp"
                |"pretty-print"
                |"quota-user" => {
                    let map = [
                        ("$-xgafv", "$.xgafv"),
                        ("access-token", "access_token"),
                        ("bearer-token", "bearer_token"),
                        ("oauth-token", "oauth_token"),
                        ("pretty-print", "prettyPrint"),
                        ("quota-user", "quotaUser"),
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

        if self.opt.cmd_projects {
            if self.opt.cmd_indexes_documents_create {
                call_result = self._projects_indexes_documents_create(dry_run, &mut err);
            } else if self.opt.cmd_indexes_documents_delete {
                call_result = self._projects_indexes_documents_delete(dry_run, &mut err);
            } else if self.opt.cmd_indexes_documents_get {
                call_result = self._projects_indexes_documents_get(dry_run, &mut err);
            } else if self.opt.cmd_indexes_documents_list {
                call_result = self._projects_indexes_documents_list(dry_run, &mut err);
            } else if self.opt.cmd_indexes_list {
                call_result = self._projects_indexes_list(dry_run, &mut err);
            } else if self.opt.cmd_indexes_search {
                call_result = self._projects_indexes_search(dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "cloudsearch1-secret.json") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                                      hyper::Client::new(),
                                      JsonTokenStorage {
                                        program_name: "cloudsearch1",
                                        db_dir: config_dir.clone(),
                                      }, None);
        let engine = Engine {
            opt: opt,
            hub: api::Cloudsearch::new(hyper::Client::new(), auth),
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