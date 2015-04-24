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
extern crate google_fusiontables2 as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  fusiontables2 [options] column delete <table-id> <column-id> [-p <v>]...
  fusiontables2 [options] column get <table-id> <column-id> [-p <v>]... [-o <out>]
  fusiontables2 [options] column insert <table-id> -r <kv>... [-p <v>]... [-o <out>]
  fusiontables2 [options] column list <table-id> [-p <v>]... [-o <out>]
  fusiontables2 [options] column patch <table-id> <column-id> -r <kv>... [-p <v>]... [-o <out>]
  fusiontables2 [options] column update <table-id> <column-id> -r <kv>... [-p <v>]... [-o <out>]
  fusiontables2 [options] query sql <sql> [-p <v>]... [-o <out>]
  fusiontables2 [options] query sql-get <sql> [-p <v>]... [-o <out>]
  fusiontables2 [options] style delete <table-id> <style-id> [-p <v>]...
  fusiontables2 [options] style get <table-id> <style-id> [-p <v>]... [-o <out>]
  fusiontables2 [options] style insert <table-id> -r <kv>... [-p <v>]... [-o <out>]
  fusiontables2 [options] style list <table-id> [-p <v>]... [-o <out>]
  fusiontables2 [options] style patch <table-id> <style-id> -r <kv>... [-p <v>]... [-o <out>]
  fusiontables2 [options] style update <table-id> <style-id> -r <kv>... [-p <v>]... [-o <out>]
  fusiontables2 [options] table copy <table-id> [-p <v>]... [-o <out>]
  fusiontables2 [options] table delete <table-id> [-p <v>]...
  fusiontables2 [options] table get <table-id> [-p <v>]... [-o <out>]
  fusiontables2 [options] table import-rows <table-id> -u (simple|resumable) <file> <mime> [-p <v>]... [-o <out>]
  fusiontables2 [options] table import-table <name> -u (simple|resumable) <file> <mime> [-p <v>]... [-o <out>]
  fusiontables2 [options] table insert -r <kv>... [-p <v>]... [-o <out>]
  fusiontables2 [options] table list [-p <v>]... [-o <out>]
  fusiontables2 [options] table patch <table-id> -r <kv>... [-p <v>]... [-o <out>]
  fusiontables2 [options] table replace-rows <table-id> -u (simple|resumable) <file> <mime> [-p <v>]... [-o <out>]
  fusiontables2 [options] table update <table-id> -r <kv>... [-p <v>]... [-o <out>]
  fusiontables2 [options] task delete <table-id> <task-id> [-p <v>]...
  fusiontables2 [options] task get <table-id> <task-id> [-p <v>]... [-o <out>]
  fusiontables2 [options] task list <table-id> [-p <v>]... [-o <out>]
  fusiontables2 [options] template delete <table-id> <template-id> [-p <v>]...
  fusiontables2 [options] template get <table-id> <template-id> [-p <v>]... [-o <out>]
  fusiontables2 [options] template insert <table-id> -r <kv>... [-p <v>]... [-o <out>]
  fusiontables2 [options] template list <table-id> [-p <v>]... [-o <out>]
  fusiontables2 [options] template patch <table-id> <template-id> -r <kv>... [-p <v>]... [-o <out>]
  fusiontables2 [options] template update <table-id> <template-id> -r <kv>... [-p <v>]... [-o <out>]
  fusiontables2 --help

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
  --debug
            Output all server communication to standard error. `tx` and `rx` are placed into 
            the same stream.
  --debug-auth
            Output all communication related to authentication to standard error. `tx` and `rx` are placed into 
            the same stream.
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
    hub: api::Fusiontables<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _column_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.column().delete(&self.opt.arg_table_id, &self.opt.arg_column_id);
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

    fn _column_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.column().get(&self.opt.arg_table_id, &self.opt.arg_column_id);
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

    fn _column_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Column::default();
        let mut call = self.hub.column().insert(&request, &self.opt.arg_table_id);
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
        
        let mut field_name = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_base_column_init(request: &mut api::Column) {
                if request.base_column.is_none() {
                    request.base_column = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "graph-predicate" => {
                        request.graph_predicate = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "valid-values" => {
                        if request.valid_values.is_none() {
                           request.valid_values = Some(Default::default());
                        }
                                        request.valid_values.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "base-column.table-index" => {
                        request_base_column_init(&mut request);
                        request.base_column.as_mut().unwrap().table_index = Some(arg_from_str(value.unwrap_or("-0"), err, "base-column.table-index", "integer"));
                    },
                "base-column.column-id" => {
                        request_base_column_init(&mut request);
                        request.base_column.as_mut().unwrap().column_id = Some(arg_from_str(value.unwrap_or("-0"), err, "base-column.column-id", "integer"));
                    },
                "name" => {
                        request_base_column_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "column-properties-json" => {
                        request_base_column_init(&mut request);
                        request.column_properties_json = Some(value.unwrap_or("").to_string());
                    },
                "format-pattern" => {
                        request_base_column_init(&mut request);
                        request.format_pattern = Some(value.unwrap_or("").to_string());
                    },
                "column-json-schema" => {
                        request_base_column_init(&mut request);
                        request.column_json_schema = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_base_column_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "validate-data" => {
                        request_base_column_init(&mut request);
                        request.validate_data = Some(arg_from_str(value.unwrap_or("false"), err, "validate-data", "boolean"));
                    },
                "column-id" => {
                        request_base_column_init(&mut request);
                        request.column_id = Some(arg_from_str(value.unwrap_or("-0"), err, "column-id", "integer"));
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _column_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.column().list(&self.opt.arg_table_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _column_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Column::default();
        let mut call = self.hub.column().patch(&request, &self.opt.arg_table_id, &self.opt.arg_column_id);
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
        
        let mut field_name = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_base_column_init(request: &mut api::Column) {
                if request.base_column.is_none() {
                    request.base_column = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "graph-predicate" => {
                        request.graph_predicate = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "valid-values" => {
                        if request.valid_values.is_none() {
                           request.valid_values = Some(Default::default());
                        }
                                        request.valid_values.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "base-column.table-index" => {
                        request_base_column_init(&mut request);
                        request.base_column.as_mut().unwrap().table_index = Some(arg_from_str(value.unwrap_or("-0"), err, "base-column.table-index", "integer"));
                    },
                "base-column.column-id" => {
                        request_base_column_init(&mut request);
                        request.base_column.as_mut().unwrap().column_id = Some(arg_from_str(value.unwrap_or("-0"), err, "base-column.column-id", "integer"));
                    },
                "name" => {
                        request_base_column_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "column-properties-json" => {
                        request_base_column_init(&mut request);
                        request.column_properties_json = Some(value.unwrap_or("").to_string());
                    },
                "format-pattern" => {
                        request_base_column_init(&mut request);
                        request.format_pattern = Some(value.unwrap_or("").to_string());
                    },
                "column-json-schema" => {
                        request_base_column_init(&mut request);
                        request.column_json_schema = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_base_column_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "validate-data" => {
                        request_base_column_init(&mut request);
                        request.validate_data = Some(arg_from_str(value.unwrap_or("false"), err, "validate-data", "boolean"));
                    },
                "column-id" => {
                        request_base_column_init(&mut request);
                        request.column_id = Some(arg_from_str(value.unwrap_or("-0"), err, "column-id", "integer"));
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _column_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Column::default();
        let mut call = self.hub.column().update(&request, &self.opt.arg_table_id, &self.opt.arg_column_id);
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
        
        let mut field_name = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_base_column_init(request: &mut api::Column) {
                if request.base_column.is_none() {
                    request.base_column = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "graph-predicate" => {
                        request.graph_predicate = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "valid-values" => {
                        if request.valid_values.is_none() {
                           request.valid_values = Some(Default::default());
                        }
                                        request.valid_values.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "base-column.table-index" => {
                        request_base_column_init(&mut request);
                        request.base_column.as_mut().unwrap().table_index = Some(arg_from_str(value.unwrap_or("-0"), err, "base-column.table-index", "integer"));
                    },
                "base-column.column-id" => {
                        request_base_column_init(&mut request);
                        request.base_column.as_mut().unwrap().column_id = Some(arg_from_str(value.unwrap_or("-0"), err, "base-column.column-id", "integer"));
                    },
                "name" => {
                        request_base_column_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "column-properties-json" => {
                        request_base_column_init(&mut request);
                        request.column_properties_json = Some(value.unwrap_or("").to_string());
                    },
                "format-pattern" => {
                        request_base_column_init(&mut request);
                        request.format_pattern = Some(value.unwrap_or("").to_string());
                    },
                "column-json-schema" => {
                        request_base_column_init(&mut request);
                        request.column_json_schema = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_base_column_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "validate-data" => {
                        request_base_column_init(&mut request);
                        request.validate_data = Some(arg_from_str(value.unwrap_or("false"), err, "validate-data", "boolean"));
                    },
                "column-id" => {
                        request_base_column_init(&mut request);
                        request.column_id = Some(arg_from_str(value.unwrap_or("-0"), err, "column-id", "integer"));
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _query_sql(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut download_mode = false;
        let mut call = self.hub.query().sql(&self.opt.arg_sql);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "typed" => {
                    call = call.typed(arg_from_str(value.unwrap_or("false"), err, "typed", "boolean"));
                },
                "hdrs" => {
                    call = call.hdrs(arg_from_str(value.unwrap_or("false"), err, "hdrs", "boolean"));
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
                Ok((mut response, output_schema)) => {
                    if !download_mode {
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    } else {
                    io::copy(&mut response, &mut ostream).unwrap();
                    }
                    None
                }
            }
        }
    }

    fn _query_sql_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut download_mode = false;
        let mut call = self.hub.query().sql_get(&self.opt.arg_sql);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "typed" => {
                    call = call.typed(arg_from_str(value.unwrap_or("false"), err, "typed", "boolean"));
                },
                "hdrs" => {
                    call = call.hdrs(arg_from_str(value.unwrap_or("false"), err, "hdrs", "boolean"));
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
                Ok((mut response, output_schema)) => {
                    if !download_mode {
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    } else {
                    io::copy(&mut response, &mut ostream).unwrap();
                    }
                    None
                }
            }
        }
    }

    fn _style_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let style_id: i32 = arg_from_str(&self.opt.arg_style_id, err, "<style-id>", "integer");
        let mut call = self.hub.style().delete(&self.opt.arg_table_id, style_id);
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

    fn _style_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let style_id: i32 = arg_from_str(&self.opt.arg_style_id, err, "<style-id>", "integer");
        let mut call = self.hub.style().get(&self.opt.arg_table_id, style_id);
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

    fn _style_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::StyleSetting::default();
        let mut call = self.hub.style().insert(&request, &self.opt.arg_table_id);
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
        
        let mut field_name = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_marker_options_icon_styler_gradient_init(request: &mut api::StyleSetting) {
                request_marker_options_icon_styler_init(request);
                if request.marker_options.as_mut().unwrap().icon_styler.as_mut().unwrap().gradient.is_none() {
                    request.marker_options.as_mut().unwrap().icon_styler.as_mut().unwrap().gradient = Some(Default::default());
                }
            }
            
            fn request_marker_options_icon_styler_init(request: &mut api::StyleSetting) {
                request_marker_options_init(request);
                if request.marker_options.as_mut().unwrap().icon_styler.is_none() {
                    request.marker_options.as_mut().unwrap().icon_styler = Some(Default::default());
                }
            }
            
            fn request_marker_options_init(request: &mut api::StyleSetting) {
                if request.marker_options.is_none() {
                    request.marker_options = Some(Default::default());
                }
            }
            
            fn request_polygon_options_fill_color_styler_gradient_init(request: &mut api::StyleSetting) {
                request_polygon_options_fill_color_styler_init(request);
                if request.polygon_options.as_mut().unwrap().fill_color_styler.as_mut().unwrap().gradient.is_none() {
                    request.polygon_options.as_mut().unwrap().fill_color_styler.as_mut().unwrap().gradient = Some(Default::default());
                }
            }
            
            fn request_polygon_options_fill_color_styler_init(request: &mut api::StyleSetting) {
                request_polygon_options_init(request);
                if request.polygon_options.as_mut().unwrap().fill_color_styler.is_none() {
                    request.polygon_options.as_mut().unwrap().fill_color_styler = Some(Default::default());
                }
            }
            
            fn request_polygon_options_init(request: &mut api::StyleSetting) {
                if request.polygon_options.is_none() {
                    request.polygon_options = Some(Default::default());
                }
            }
            
            fn request_polygon_options_stroke_color_styler_gradient_init(request: &mut api::StyleSetting) {
                request_polygon_options_stroke_color_styler_init(request);
                if request.polygon_options.as_mut().unwrap().stroke_color_styler.as_mut().unwrap().gradient.is_none() {
                    request.polygon_options.as_mut().unwrap().stroke_color_styler.as_mut().unwrap().gradient = Some(Default::default());
                }
            }
            
            fn request_polygon_options_stroke_color_styler_init(request: &mut api::StyleSetting) {
                request_polygon_options_init(request);
                if request.polygon_options.as_mut().unwrap().stroke_color_styler.is_none() {
                    request.polygon_options.as_mut().unwrap().stroke_color_styler = Some(Default::default());
                }
            }
            
            fn request_polygon_options_stroke_weight_styler_gradient_init(request: &mut api::StyleSetting) {
                request_polygon_options_stroke_weight_styler_init(request);
                if request.polygon_options.as_mut().unwrap().stroke_weight_styler.as_mut().unwrap().gradient.is_none() {
                    request.polygon_options.as_mut().unwrap().stroke_weight_styler.as_mut().unwrap().gradient = Some(Default::default());
                }
            }
            
            fn request_polygon_options_stroke_weight_styler_init(request: &mut api::StyleSetting) {
                request_polygon_options_init(request);
                if request.polygon_options.as_mut().unwrap().stroke_weight_styler.is_none() {
                    request.polygon_options.as_mut().unwrap().stroke_weight_styler = Some(Default::default());
                }
            }
            
            fn request_polyline_options_init(request: &mut api::StyleSetting) {
                if request.polyline_options.is_none() {
                    request.polyline_options = Some(Default::default());
                }
            }
            
            fn request_polyline_options_stroke_color_styler_gradient_init(request: &mut api::StyleSetting) {
                request_polyline_options_stroke_color_styler_init(request);
                if request.polyline_options.as_mut().unwrap().stroke_color_styler.as_mut().unwrap().gradient.is_none() {
                    request.polyline_options.as_mut().unwrap().stroke_color_styler.as_mut().unwrap().gradient = Some(Default::default());
                }
            }
            
            fn request_polyline_options_stroke_color_styler_init(request: &mut api::StyleSetting) {
                request_polyline_options_init(request);
                if request.polyline_options.as_mut().unwrap().stroke_color_styler.is_none() {
                    request.polyline_options.as_mut().unwrap().stroke_color_styler = Some(Default::default());
                }
            }
            
            fn request_polyline_options_stroke_weight_styler_gradient_init(request: &mut api::StyleSetting) {
                request_polyline_options_stroke_weight_styler_init(request);
                if request.polyline_options.as_mut().unwrap().stroke_weight_styler.as_mut().unwrap().gradient.is_none() {
                    request.polyline_options.as_mut().unwrap().stroke_weight_styler.as_mut().unwrap().gradient = Some(Default::default());
                }
            }
            
            fn request_polyline_options_stroke_weight_styler_init(request: &mut api::StyleSetting) {
                request_polyline_options_init(request);
                if request.polyline_options.as_mut().unwrap().stroke_weight_styler.is_none() {
                    request.polyline_options.as_mut().unwrap().stroke_weight_styler = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "marker-options.icon-styler.gradient.max" => {
                        request_marker_options_icon_styler_gradient_init(&mut request);
                        request.marker_options.as_mut().unwrap().icon_styler.as_mut().unwrap().gradient.as_mut().unwrap().max = Some(arg_from_str(value.unwrap_or("0.0"), err, "marker-options.icon-styler.gradient.max", "number"));
                    },
                "marker-options.icon-styler.gradient.min" => {
                        request_marker_options_icon_styler_gradient_init(&mut request);
                        request.marker_options.as_mut().unwrap().icon_styler.as_mut().unwrap().gradient.as_mut().unwrap().min = Some(arg_from_str(value.unwrap_or("0.0"), err, "marker-options.icon-styler.gradient.min", "number"));
                    },
                "marker-options.icon-styler.column-name" => {
                        request_marker_options_icon_styler_gradient_init(&mut request);
                        request.marker_options.as_mut().unwrap().icon_styler.as_mut().unwrap().column_name = Some(value.unwrap_or("").to_string());
                    },
                "marker-options.icon-styler.kind" => {
                        request_marker_options_icon_styler_gradient_init(&mut request);
                        request.marker_options.as_mut().unwrap().icon_styler.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "marker-options.icon-name" => {
                        request_marker_options_icon_styler_init(&mut request);
                        request.marker_options.as_mut().unwrap().icon_name = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_marker_options_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_marker_options_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "polygon-options.stroke-color-styler.gradient.max" => {
                        request_polygon_options_stroke_color_styler_gradient_init(&mut request);
                        request.polygon_options.as_mut().unwrap().stroke_color_styler.as_mut().unwrap().gradient.as_mut().unwrap().max = Some(arg_from_str(value.unwrap_or("0.0"), err, "polygon-options.stroke-color-styler.gradient.max", "number"));
                    },
                "polygon-options.stroke-color-styler.gradient.min" => {
                        request_polygon_options_stroke_color_styler_gradient_init(&mut request);
                        request.polygon_options.as_mut().unwrap().stroke_color_styler.as_mut().unwrap().gradient.as_mut().unwrap().min = Some(arg_from_str(value.unwrap_or("0.0"), err, "polygon-options.stroke-color-styler.gradient.min", "number"));
                    },
                "polygon-options.stroke-color-styler.column-name" => {
                        request_polygon_options_stroke_color_styler_gradient_init(&mut request);
                        request.polygon_options.as_mut().unwrap().stroke_color_styler.as_mut().unwrap().column_name = Some(value.unwrap_or("").to_string());
                    },
                "polygon-options.stroke-color-styler.kind" => {
                        request_polygon_options_stroke_color_styler_gradient_init(&mut request);
                        request.polygon_options.as_mut().unwrap().stroke_color_styler.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "polygon-options.stroke-weight" => {
                        request_polygon_options_stroke_color_styler_init(&mut request);
                        request.polygon_options.as_mut().unwrap().stroke_weight = Some(arg_from_str(value.unwrap_or("-0"), err, "polygon-options.stroke-weight", "integer"));
                    },
                "polygon-options.stroke-opacity" => {
                        request_polygon_options_stroke_color_styler_init(&mut request);
                        request.polygon_options.as_mut().unwrap().stroke_opacity = Some(arg_from_str(value.unwrap_or("0.0"), err, "polygon-options.stroke-opacity", "number"));
                    },
                "polygon-options.stroke-weight-styler.gradient.max" => {
                        request_polygon_options_stroke_weight_styler_gradient_init(&mut request);
                        request.polygon_options.as_mut().unwrap().stroke_weight_styler.as_mut().unwrap().gradient.as_mut().unwrap().max = Some(arg_from_str(value.unwrap_or("0.0"), err, "polygon-options.stroke-weight-styler.gradient.max", "number"));
                    },
                "polygon-options.stroke-weight-styler.gradient.min" => {
                        request_polygon_options_stroke_weight_styler_gradient_init(&mut request);
                        request.polygon_options.as_mut().unwrap().stroke_weight_styler.as_mut().unwrap().gradient.as_mut().unwrap().min = Some(arg_from_str(value.unwrap_or("0.0"), err, "polygon-options.stroke-weight-styler.gradient.min", "number"));
                    },
                "polygon-options.stroke-weight-styler.column-name" => {
                        request_polygon_options_stroke_weight_styler_gradient_init(&mut request);
                        request.polygon_options.as_mut().unwrap().stroke_weight_styler.as_mut().unwrap().column_name = Some(value.unwrap_or("").to_string());
                    },
                "polygon-options.stroke-weight-styler.kind" => {
                        request_polygon_options_stroke_weight_styler_gradient_init(&mut request);
                        request.polygon_options.as_mut().unwrap().stroke_weight_styler.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "polygon-options.fill-color-styler.gradient.max" => {
                        request_polygon_options_fill_color_styler_gradient_init(&mut request);
                        request.polygon_options.as_mut().unwrap().fill_color_styler.as_mut().unwrap().gradient.as_mut().unwrap().max = Some(arg_from_str(value.unwrap_or("0.0"), err, "polygon-options.fill-color-styler.gradient.max", "number"));
                    },
                "polygon-options.fill-color-styler.gradient.min" => {
                        request_polygon_options_fill_color_styler_gradient_init(&mut request);
                        request.polygon_options.as_mut().unwrap().fill_color_styler.as_mut().unwrap().gradient.as_mut().unwrap().min = Some(arg_from_str(value.unwrap_or("0.0"), err, "polygon-options.fill-color-styler.gradient.min", "number"));
                    },
                "polygon-options.fill-color-styler.column-name" => {
                        request_polygon_options_fill_color_styler_gradient_init(&mut request);
                        request.polygon_options.as_mut().unwrap().fill_color_styler.as_mut().unwrap().column_name = Some(value.unwrap_or("").to_string());
                    },
                "polygon-options.fill-color-styler.kind" => {
                        request_polygon_options_fill_color_styler_gradient_init(&mut request);
                        request.polygon_options.as_mut().unwrap().fill_color_styler.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "polygon-options.fill-color" => {
                        request_polygon_options_fill_color_styler_init(&mut request);
                        request.polygon_options.as_mut().unwrap().fill_color = Some(value.unwrap_or("").to_string());
                    },
                "polygon-options.stroke-color" => {
                        request_polygon_options_fill_color_styler_init(&mut request);
                        request.polygon_options.as_mut().unwrap().stroke_color = Some(value.unwrap_or("").to_string());
                    },
                "polygon-options.fill-opacity" => {
                        request_polygon_options_fill_color_styler_init(&mut request);
                        request.polygon_options.as_mut().unwrap().fill_opacity = Some(arg_from_str(value.unwrap_or("0.0"), err, "polygon-options.fill-opacity", "number"));
                    },
                "polyline-options.stroke-weight" => {
                        request_polyline_options_init(&mut request);
                        request.polyline_options.as_mut().unwrap().stroke_weight = Some(arg_from_str(value.unwrap_or("-0"), err, "polyline-options.stroke-weight", "integer"));
                    },
                "polyline-options.stroke-weight-styler.gradient.max" => {
                        request_polyline_options_stroke_weight_styler_gradient_init(&mut request);
                        request.polyline_options.as_mut().unwrap().stroke_weight_styler.as_mut().unwrap().gradient.as_mut().unwrap().max = Some(arg_from_str(value.unwrap_or("0.0"), err, "polyline-options.stroke-weight-styler.gradient.max", "number"));
                    },
                "polyline-options.stroke-weight-styler.gradient.min" => {
                        request_polyline_options_stroke_weight_styler_gradient_init(&mut request);
                        request.polyline_options.as_mut().unwrap().stroke_weight_styler.as_mut().unwrap().gradient.as_mut().unwrap().min = Some(arg_from_str(value.unwrap_or("0.0"), err, "polyline-options.stroke-weight-styler.gradient.min", "number"));
                    },
                "polyline-options.stroke-weight-styler.column-name" => {
                        request_polyline_options_stroke_weight_styler_gradient_init(&mut request);
                        request.polyline_options.as_mut().unwrap().stroke_weight_styler.as_mut().unwrap().column_name = Some(value.unwrap_or("").to_string());
                    },
                "polyline-options.stroke-weight-styler.kind" => {
                        request_polyline_options_stroke_weight_styler_gradient_init(&mut request);
                        request.polyline_options.as_mut().unwrap().stroke_weight_styler.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "polyline-options.stroke-color" => {
                        request_polyline_options_stroke_weight_styler_init(&mut request);
                        request.polyline_options.as_mut().unwrap().stroke_color = Some(value.unwrap_or("").to_string());
                    },
                "polyline-options.stroke-opacity" => {
                        request_polyline_options_stroke_weight_styler_init(&mut request);
                        request.polyline_options.as_mut().unwrap().stroke_opacity = Some(arg_from_str(value.unwrap_or("0.0"), err, "polyline-options.stroke-opacity", "number"));
                    },
                "polyline-options.stroke-color-styler.gradient.max" => {
                        request_polyline_options_stroke_color_styler_gradient_init(&mut request);
                        request.polyline_options.as_mut().unwrap().stroke_color_styler.as_mut().unwrap().gradient.as_mut().unwrap().max = Some(arg_from_str(value.unwrap_or("0.0"), err, "polyline-options.stroke-color-styler.gradient.max", "number"));
                    },
                "polyline-options.stroke-color-styler.gradient.min" => {
                        request_polyline_options_stroke_color_styler_gradient_init(&mut request);
                        request.polyline_options.as_mut().unwrap().stroke_color_styler.as_mut().unwrap().gradient.as_mut().unwrap().min = Some(arg_from_str(value.unwrap_or("0.0"), err, "polyline-options.stroke-color-styler.gradient.min", "number"));
                    },
                "polyline-options.stroke-color-styler.column-name" => {
                        request_polyline_options_stroke_color_styler_gradient_init(&mut request);
                        request.polyline_options.as_mut().unwrap().stroke_color_styler.as_mut().unwrap().column_name = Some(value.unwrap_or("").to_string());
                    },
                "polyline-options.stroke-color-styler.kind" => {
                        request_polyline_options_stroke_color_styler_gradient_init(&mut request);
                        request.polyline_options.as_mut().unwrap().stroke_color_styler.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "table-id" => {
                        request_polyline_options_init(&mut request);
                        request.table_id = Some(value.unwrap_or("").to_string());
                    },
                "style-id" => {
                        request_polyline_options_init(&mut request);
                        request.style_id = Some(arg_from_str(value.unwrap_or("-0"), err, "style-id", "integer"));
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _style_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.style().list(&self.opt.arg_table_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _style_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::StyleSetting::default();
        let style_id: i32 = arg_from_str(&self.opt.arg_style_id, err, "<style-id>", "integer");
        let mut call = self.hub.style().patch(&request, &self.opt.arg_table_id, style_id);
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
        
        let mut field_name = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_marker_options_icon_styler_gradient_init(request: &mut api::StyleSetting) {
                request_marker_options_icon_styler_init(request);
                if request.marker_options.as_mut().unwrap().icon_styler.as_mut().unwrap().gradient.is_none() {
                    request.marker_options.as_mut().unwrap().icon_styler.as_mut().unwrap().gradient = Some(Default::default());
                }
            }
            
            fn request_marker_options_icon_styler_init(request: &mut api::StyleSetting) {
                request_marker_options_init(request);
                if request.marker_options.as_mut().unwrap().icon_styler.is_none() {
                    request.marker_options.as_mut().unwrap().icon_styler = Some(Default::default());
                }
            }
            
            fn request_marker_options_init(request: &mut api::StyleSetting) {
                if request.marker_options.is_none() {
                    request.marker_options = Some(Default::default());
                }
            }
            
            fn request_polygon_options_fill_color_styler_gradient_init(request: &mut api::StyleSetting) {
                request_polygon_options_fill_color_styler_init(request);
                if request.polygon_options.as_mut().unwrap().fill_color_styler.as_mut().unwrap().gradient.is_none() {
                    request.polygon_options.as_mut().unwrap().fill_color_styler.as_mut().unwrap().gradient = Some(Default::default());
                }
            }
            
            fn request_polygon_options_fill_color_styler_init(request: &mut api::StyleSetting) {
                request_polygon_options_init(request);
                if request.polygon_options.as_mut().unwrap().fill_color_styler.is_none() {
                    request.polygon_options.as_mut().unwrap().fill_color_styler = Some(Default::default());
                }
            }
            
            fn request_polygon_options_init(request: &mut api::StyleSetting) {
                if request.polygon_options.is_none() {
                    request.polygon_options = Some(Default::default());
                }
            }
            
            fn request_polygon_options_stroke_color_styler_gradient_init(request: &mut api::StyleSetting) {
                request_polygon_options_stroke_color_styler_init(request);
                if request.polygon_options.as_mut().unwrap().stroke_color_styler.as_mut().unwrap().gradient.is_none() {
                    request.polygon_options.as_mut().unwrap().stroke_color_styler.as_mut().unwrap().gradient = Some(Default::default());
                }
            }
            
            fn request_polygon_options_stroke_color_styler_init(request: &mut api::StyleSetting) {
                request_polygon_options_init(request);
                if request.polygon_options.as_mut().unwrap().stroke_color_styler.is_none() {
                    request.polygon_options.as_mut().unwrap().stroke_color_styler = Some(Default::default());
                }
            }
            
            fn request_polygon_options_stroke_weight_styler_gradient_init(request: &mut api::StyleSetting) {
                request_polygon_options_stroke_weight_styler_init(request);
                if request.polygon_options.as_mut().unwrap().stroke_weight_styler.as_mut().unwrap().gradient.is_none() {
                    request.polygon_options.as_mut().unwrap().stroke_weight_styler.as_mut().unwrap().gradient = Some(Default::default());
                }
            }
            
            fn request_polygon_options_stroke_weight_styler_init(request: &mut api::StyleSetting) {
                request_polygon_options_init(request);
                if request.polygon_options.as_mut().unwrap().stroke_weight_styler.is_none() {
                    request.polygon_options.as_mut().unwrap().stroke_weight_styler = Some(Default::default());
                }
            }
            
            fn request_polyline_options_init(request: &mut api::StyleSetting) {
                if request.polyline_options.is_none() {
                    request.polyline_options = Some(Default::default());
                }
            }
            
            fn request_polyline_options_stroke_color_styler_gradient_init(request: &mut api::StyleSetting) {
                request_polyline_options_stroke_color_styler_init(request);
                if request.polyline_options.as_mut().unwrap().stroke_color_styler.as_mut().unwrap().gradient.is_none() {
                    request.polyline_options.as_mut().unwrap().stroke_color_styler.as_mut().unwrap().gradient = Some(Default::default());
                }
            }
            
            fn request_polyline_options_stroke_color_styler_init(request: &mut api::StyleSetting) {
                request_polyline_options_init(request);
                if request.polyline_options.as_mut().unwrap().stroke_color_styler.is_none() {
                    request.polyline_options.as_mut().unwrap().stroke_color_styler = Some(Default::default());
                }
            }
            
            fn request_polyline_options_stroke_weight_styler_gradient_init(request: &mut api::StyleSetting) {
                request_polyline_options_stroke_weight_styler_init(request);
                if request.polyline_options.as_mut().unwrap().stroke_weight_styler.as_mut().unwrap().gradient.is_none() {
                    request.polyline_options.as_mut().unwrap().stroke_weight_styler.as_mut().unwrap().gradient = Some(Default::default());
                }
            }
            
            fn request_polyline_options_stroke_weight_styler_init(request: &mut api::StyleSetting) {
                request_polyline_options_init(request);
                if request.polyline_options.as_mut().unwrap().stroke_weight_styler.is_none() {
                    request.polyline_options.as_mut().unwrap().stroke_weight_styler = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "marker-options.icon-styler.gradient.max" => {
                        request_marker_options_icon_styler_gradient_init(&mut request);
                        request.marker_options.as_mut().unwrap().icon_styler.as_mut().unwrap().gradient.as_mut().unwrap().max = Some(arg_from_str(value.unwrap_or("0.0"), err, "marker-options.icon-styler.gradient.max", "number"));
                    },
                "marker-options.icon-styler.gradient.min" => {
                        request_marker_options_icon_styler_gradient_init(&mut request);
                        request.marker_options.as_mut().unwrap().icon_styler.as_mut().unwrap().gradient.as_mut().unwrap().min = Some(arg_from_str(value.unwrap_or("0.0"), err, "marker-options.icon-styler.gradient.min", "number"));
                    },
                "marker-options.icon-styler.column-name" => {
                        request_marker_options_icon_styler_gradient_init(&mut request);
                        request.marker_options.as_mut().unwrap().icon_styler.as_mut().unwrap().column_name = Some(value.unwrap_or("").to_string());
                    },
                "marker-options.icon-styler.kind" => {
                        request_marker_options_icon_styler_gradient_init(&mut request);
                        request.marker_options.as_mut().unwrap().icon_styler.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "marker-options.icon-name" => {
                        request_marker_options_icon_styler_init(&mut request);
                        request.marker_options.as_mut().unwrap().icon_name = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_marker_options_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_marker_options_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "polygon-options.stroke-color-styler.gradient.max" => {
                        request_polygon_options_stroke_color_styler_gradient_init(&mut request);
                        request.polygon_options.as_mut().unwrap().stroke_color_styler.as_mut().unwrap().gradient.as_mut().unwrap().max = Some(arg_from_str(value.unwrap_or("0.0"), err, "polygon-options.stroke-color-styler.gradient.max", "number"));
                    },
                "polygon-options.stroke-color-styler.gradient.min" => {
                        request_polygon_options_stroke_color_styler_gradient_init(&mut request);
                        request.polygon_options.as_mut().unwrap().stroke_color_styler.as_mut().unwrap().gradient.as_mut().unwrap().min = Some(arg_from_str(value.unwrap_or("0.0"), err, "polygon-options.stroke-color-styler.gradient.min", "number"));
                    },
                "polygon-options.stroke-color-styler.column-name" => {
                        request_polygon_options_stroke_color_styler_gradient_init(&mut request);
                        request.polygon_options.as_mut().unwrap().stroke_color_styler.as_mut().unwrap().column_name = Some(value.unwrap_or("").to_string());
                    },
                "polygon-options.stroke-color-styler.kind" => {
                        request_polygon_options_stroke_color_styler_gradient_init(&mut request);
                        request.polygon_options.as_mut().unwrap().stroke_color_styler.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "polygon-options.stroke-weight" => {
                        request_polygon_options_stroke_color_styler_init(&mut request);
                        request.polygon_options.as_mut().unwrap().stroke_weight = Some(arg_from_str(value.unwrap_or("-0"), err, "polygon-options.stroke-weight", "integer"));
                    },
                "polygon-options.stroke-opacity" => {
                        request_polygon_options_stroke_color_styler_init(&mut request);
                        request.polygon_options.as_mut().unwrap().stroke_opacity = Some(arg_from_str(value.unwrap_or("0.0"), err, "polygon-options.stroke-opacity", "number"));
                    },
                "polygon-options.stroke-weight-styler.gradient.max" => {
                        request_polygon_options_stroke_weight_styler_gradient_init(&mut request);
                        request.polygon_options.as_mut().unwrap().stroke_weight_styler.as_mut().unwrap().gradient.as_mut().unwrap().max = Some(arg_from_str(value.unwrap_or("0.0"), err, "polygon-options.stroke-weight-styler.gradient.max", "number"));
                    },
                "polygon-options.stroke-weight-styler.gradient.min" => {
                        request_polygon_options_stroke_weight_styler_gradient_init(&mut request);
                        request.polygon_options.as_mut().unwrap().stroke_weight_styler.as_mut().unwrap().gradient.as_mut().unwrap().min = Some(arg_from_str(value.unwrap_or("0.0"), err, "polygon-options.stroke-weight-styler.gradient.min", "number"));
                    },
                "polygon-options.stroke-weight-styler.column-name" => {
                        request_polygon_options_stroke_weight_styler_gradient_init(&mut request);
                        request.polygon_options.as_mut().unwrap().stroke_weight_styler.as_mut().unwrap().column_name = Some(value.unwrap_or("").to_string());
                    },
                "polygon-options.stroke-weight-styler.kind" => {
                        request_polygon_options_stroke_weight_styler_gradient_init(&mut request);
                        request.polygon_options.as_mut().unwrap().stroke_weight_styler.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "polygon-options.fill-color-styler.gradient.max" => {
                        request_polygon_options_fill_color_styler_gradient_init(&mut request);
                        request.polygon_options.as_mut().unwrap().fill_color_styler.as_mut().unwrap().gradient.as_mut().unwrap().max = Some(arg_from_str(value.unwrap_or("0.0"), err, "polygon-options.fill-color-styler.gradient.max", "number"));
                    },
                "polygon-options.fill-color-styler.gradient.min" => {
                        request_polygon_options_fill_color_styler_gradient_init(&mut request);
                        request.polygon_options.as_mut().unwrap().fill_color_styler.as_mut().unwrap().gradient.as_mut().unwrap().min = Some(arg_from_str(value.unwrap_or("0.0"), err, "polygon-options.fill-color-styler.gradient.min", "number"));
                    },
                "polygon-options.fill-color-styler.column-name" => {
                        request_polygon_options_fill_color_styler_gradient_init(&mut request);
                        request.polygon_options.as_mut().unwrap().fill_color_styler.as_mut().unwrap().column_name = Some(value.unwrap_or("").to_string());
                    },
                "polygon-options.fill-color-styler.kind" => {
                        request_polygon_options_fill_color_styler_gradient_init(&mut request);
                        request.polygon_options.as_mut().unwrap().fill_color_styler.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "polygon-options.fill-color" => {
                        request_polygon_options_fill_color_styler_init(&mut request);
                        request.polygon_options.as_mut().unwrap().fill_color = Some(value.unwrap_or("").to_string());
                    },
                "polygon-options.stroke-color" => {
                        request_polygon_options_fill_color_styler_init(&mut request);
                        request.polygon_options.as_mut().unwrap().stroke_color = Some(value.unwrap_or("").to_string());
                    },
                "polygon-options.fill-opacity" => {
                        request_polygon_options_fill_color_styler_init(&mut request);
                        request.polygon_options.as_mut().unwrap().fill_opacity = Some(arg_from_str(value.unwrap_or("0.0"), err, "polygon-options.fill-opacity", "number"));
                    },
                "polyline-options.stroke-weight" => {
                        request_polyline_options_init(&mut request);
                        request.polyline_options.as_mut().unwrap().stroke_weight = Some(arg_from_str(value.unwrap_or("-0"), err, "polyline-options.stroke-weight", "integer"));
                    },
                "polyline-options.stroke-weight-styler.gradient.max" => {
                        request_polyline_options_stroke_weight_styler_gradient_init(&mut request);
                        request.polyline_options.as_mut().unwrap().stroke_weight_styler.as_mut().unwrap().gradient.as_mut().unwrap().max = Some(arg_from_str(value.unwrap_or("0.0"), err, "polyline-options.stroke-weight-styler.gradient.max", "number"));
                    },
                "polyline-options.stroke-weight-styler.gradient.min" => {
                        request_polyline_options_stroke_weight_styler_gradient_init(&mut request);
                        request.polyline_options.as_mut().unwrap().stroke_weight_styler.as_mut().unwrap().gradient.as_mut().unwrap().min = Some(arg_from_str(value.unwrap_or("0.0"), err, "polyline-options.stroke-weight-styler.gradient.min", "number"));
                    },
                "polyline-options.stroke-weight-styler.column-name" => {
                        request_polyline_options_stroke_weight_styler_gradient_init(&mut request);
                        request.polyline_options.as_mut().unwrap().stroke_weight_styler.as_mut().unwrap().column_name = Some(value.unwrap_or("").to_string());
                    },
                "polyline-options.stroke-weight-styler.kind" => {
                        request_polyline_options_stroke_weight_styler_gradient_init(&mut request);
                        request.polyline_options.as_mut().unwrap().stroke_weight_styler.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "polyline-options.stroke-color" => {
                        request_polyline_options_stroke_weight_styler_init(&mut request);
                        request.polyline_options.as_mut().unwrap().stroke_color = Some(value.unwrap_or("").to_string());
                    },
                "polyline-options.stroke-opacity" => {
                        request_polyline_options_stroke_weight_styler_init(&mut request);
                        request.polyline_options.as_mut().unwrap().stroke_opacity = Some(arg_from_str(value.unwrap_or("0.0"), err, "polyline-options.stroke-opacity", "number"));
                    },
                "polyline-options.stroke-color-styler.gradient.max" => {
                        request_polyline_options_stroke_color_styler_gradient_init(&mut request);
                        request.polyline_options.as_mut().unwrap().stroke_color_styler.as_mut().unwrap().gradient.as_mut().unwrap().max = Some(arg_from_str(value.unwrap_or("0.0"), err, "polyline-options.stroke-color-styler.gradient.max", "number"));
                    },
                "polyline-options.stroke-color-styler.gradient.min" => {
                        request_polyline_options_stroke_color_styler_gradient_init(&mut request);
                        request.polyline_options.as_mut().unwrap().stroke_color_styler.as_mut().unwrap().gradient.as_mut().unwrap().min = Some(arg_from_str(value.unwrap_or("0.0"), err, "polyline-options.stroke-color-styler.gradient.min", "number"));
                    },
                "polyline-options.stroke-color-styler.column-name" => {
                        request_polyline_options_stroke_color_styler_gradient_init(&mut request);
                        request.polyline_options.as_mut().unwrap().stroke_color_styler.as_mut().unwrap().column_name = Some(value.unwrap_or("").to_string());
                    },
                "polyline-options.stroke-color-styler.kind" => {
                        request_polyline_options_stroke_color_styler_gradient_init(&mut request);
                        request.polyline_options.as_mut().unwrap().stroke_color_styler.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "table-id" => {
                        request_polyline_options_init(&mut request);
                        request.table_id = Some(value.unwrap_or("").to_string());
                    },
                "style-id" => {
                        request_polyline_options_init(&mut request);
                        request.style_id = Some(arg_from_str(value.unwrap_or("-0"), err, "style-id", "integer"));
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _style_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::StyleSetting::default();
        let style_id: i32 = arg_from_str(&self.opt.arg_style_id, err, "<style-id>", "integer");
        let mut call = self.hub.style().update(&request, &self.opt.arg_table_id, style_id);
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
        
        let mut field_name = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_marker_options_icon_styler_gradient_init(request: &mut api::StyleSetting) {
                request_marker_options_icon_styler_init(request);
                if request.marker_options.as_mut().unwrap().icon_styler.as_mut().unwrap().gradient.is_none() {
                    request.marker_options.as_mut().unwrap().icon_styler.as_mut().unwrap().gradient = Some(Default::default());
                }
            }
            
            fn request_marker_options_icon_styler_init(request: &mut api::StyleSetting) {
                request_marker_options_init(request);
                if request.marker_options.as_mut().unwrap().icon_styler.is_none() {
                    request.marker_options.as_mut().unwrap().icon_styler = Some(Default::default());
                }
            }
            
            fn request_marker_options_init(request: &mut api::StyleSetting) {
                if request.marker_options.is_none() {
                    request.marker_options = Some(Default::default());
                }
            }
            
            fn request_polygon_options_fill_color_styler_gradient_init(request: &mut api::StyleSetting) {
                request_polygon_options_fill_color_styler_init(request);
                if request.polygon_options.as_mut().unwrap().fill_color_styler.as_mut().unwrap().gradient.is_none() {
                    request.polygon_options.as_mut().unwrap().fill_color_styler.as_mut().unwrap().gradient = Some(Default::default());
                }
            }
            
            fn request_polygon_options_fill_color_styler_init(request: &mut api::StyleSetting) {
                request_polygon_options_init(request);
                if request.polygon_options.as_mut().unwrap().fill_color_styler.is_none() {
                    request.polygon_options.as_mut().unwrap().fill_color_styler = Some(Default::default());
                }
            }
            
            fn request_polygon_options_init(request: &mut api::StyleSetting) {
                if request.polygon_options.is_none() {
                    request.polygon_options = Some(Default::default());
                }
            }
            
            fn request_polygon_options_stroke_color_styler_gradient_init(request: &mut api::StyleSetting) {
                request_polygon_options_stroke_color_styler_init(request);
                if request.polygon_options.as_mut().unwrap().stroke_color_styler.as_mut().unwrap().gradient.is_none() {
                    request.polygon_options.as_mut().unwrap().stroke_color_styler.as_mut().unwrap().gradient = Some(Default::default());
                }
            }
            
            fn request_polygon_options_stroke_color_styler_init(request: &mut api::StyleSetting) {
                request_polygon_options_init(request);
                if request.polygon_options.as_mut().unwrap().stroke_color_styler.is_none() {
                    request.polygon_options.as_mut().unwrap().stroke_color_styler = Some(Default::default());
                }
            }
            
            fn request_polygon_options_stroke_weight_styler_gradient_init(request: &mut api::StyleSetting) {
                request_polygon_options_stroke_weight_styler_init(request);
                if request.polygon_options.as_mut().unwrap().stroke_weight_styler.as_mut().unwrap().gradient.is_none() {
                    request.polygon_options.as_mut().unwrap().stroke_weight_styler.as_mut().unwrap().gradient = Some(Default::default());
                }
            }
            
            fn request_polygon_options_stroke_weight_styler_init(request: &mut api::StyleSetting) {
                request_polygon_options_init(request);
                if request.polygon_options.as_mut().unwrap().stroke_weight_styler.is_none() {
                    request.polygon_options.as_mut().unwrap().stroke_weight_styler = Some(Default::default());
                }
            }
            
            fn request_polyline_options_init(request: &mut api::StyleSetting) {
                if request.polyline_options.is_none() {
                    request.polyline_options = Some(Default::default());
                }
            }
            
            fn request_polyline_options_stroke_color_styler_gradient_init(request: &mut api::StyleSetting) {
                request_polyline_options_stroke_color_styler_init(request);
                if request.polyline_options.as_mut().unwrap().stroke_color_styler.as_mut().unwrap().gradient.is_none() {
                    request.polyline_options.as_mut().unwrap().stroke_color_styler.as_mut().unwrap().gradient = Some(Default::default());
                }
            }
            
            fn request_polyline_options_stroke_color_styler_init(request: &mut api::StyleSetting) {
                request_polyline_options_init(request);
                if request.polyline_options.as_mut().unwrap().stroke_color_styler.is_none() {
                    request.polyline_options.as_mut().unwrap().stroke_color_styler = Some(Default::default());
                }
            }
            
            fn request_polyline_options_stroke_weight_styler_gradient_init(request: &mut api::StyleSetting) {
                request_polyline_options_stroke_weight_styler_init(request);
                if request.polyline_options.as_mut().unwrap().stroke_weight_styler.as_mut().unwrap().gradient.is_none() {
                    request.polyline_options.as_mut().unwrap().stroke_weight_styler.as_mut().unwrap().gradient = Some(Default::default());
                }
            }
            
            fn request_polyline_options_stroke_weight_styler_init(request: &mut api::StyleSetting) {
                request_polyline_options_init(request);
                if request.polyline_options.as_mut().unwrap().stroke_weight_styler.is_none() {
                    request.polyline_options.as_mut().unwrap().stroke_weight_styler = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "marker-options.icon-styler.gradient.max" => {
                        request_marker_options_icon_styler_gradient_init(&mut request);
                        request.marker_options.as_mut().unwrap().icon_styler.as_mut().unwrap().gradient.as_mut().unwrap().max = Some(arg_from_str(value.unwrap_or("0.0"), err, "marker-options.icon-styler.gradient.max", "number"));
                    },
                "marker-options.icon-styler.gradient.min" => {
                        request_marker_options_icon_styler_gradient_init(&mut request);
                        request.marker_options.as_mut().unwrap().icon_styler.as_mut().unwrap().gradient.as_mut().unwrap().min = Some(arg_from_str(value.unwrap_or("0.0"), err, "marker-options.icon-styler.gradient.min", "number"));
                    },
                "marker-options.icon-styler.column-name" => {
                        request_marker_options_icon_styler_gradient_init(&mut request);
                        request.marker_options.as_mut().unwrap().icon_styler.as_mut().unwrap().column_name = Some(value.unwrap_or("").to_string());
                    },
                "marker-options.icon-styler.kind" => {
                        request_marker_options_icon_styler_gradient_init(&mut request);
                        request.marker_options.as_mut().unwrap().icon_styler.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "marker-options.icon-name" => {
                        request_marker_options_icon_styler_init(&mut request);
                        request.marker_options.as_mut().unwrap().icon_name = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_marker_options_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_marker_options_init(&mut request);
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "polygon-options.stroke-color-styler.gradient.max" => {
                        request_polygon_options_stroke_color_styler_gradient_init(&mut request);
                        request.polygon_options.as_mut().unwrap().stroke_color_styler.as_mut().unwrap().gradient.as_mut().unwrap().max = Some(arg_from_str(value.unwrap_or("0.0"), err, "polygon-options.stroke-color-styler.gradient.max", "number"));
                    },
                "polygon-options.stroke-color-styler.gradient.min" => {
                        request_polygon_options_stroke_color_styler_gradient_init(&mut request);
                        request.polygon_options.as_mut().unwrap().stroke_color_styler.as_mut().unwrap().gradient.as_mut().unwrap().min = Some(arg_from_str(value.unwrap_or("0.0"), err, "polygon-options.stroke-color-styler.gradient.min", "number"));
                    },
                "polygon-options.stroke-color-styler.column-name" => {
                        request_polygon_options_stroke_color_styler_gradient_init(&mut request);
                        request.polygon_options.as_mut().unwrap().stroke_color_styler.as_mut().unwrap().column_name = Some(value.unwrap_or("").to_string());
                    },
                "polygon-options.stroke-color-styler.kind" => {
                        request_polygon_options_stroke_color_styler_gradient_init(&mut request);
                        request.polygon_options.as_mut().unwrap().stroke_color_styler.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "polygon-options.stroke-weight" => {
                        request_polygon_options_stroke_color_styler_init(&mut request);
                        request.polygon_options.as_mut().unwrap().stroke_weight = Some(arg_from_str(value.unwrap_or("-0"), err, "polygon-options.stroke-weight", "integer"));
                    },
                "polygon-options.stroke-opacity" => {
                        request_polygon_options_stroke_color_styler_init(&mut request);
                        request.polygon_options.as_mut().unwrap().stroke_opacity = Some(arg_from_str(value.unwrap_or("0.0"), err, "polygon-options.stroke-opacity", "number"));
                    },
                "polygon-options.stroke-weight-styler.gradient.max" => {
                        request_polygon_options_stroke_weight_styler_gradient_init(&mut request);
                        request.polygon_options.as_mut().unwrap().stroke_weight_styler.as_mut().unwrap().gradient.as_mut().unwrap().max = Some(arg_from_str(value.unwrap_or("0.0"), err, "polygon-options.stroke-weight-styler.gradient.max", "number"));
                    },
                "polygon-options.stroke-weight-styler.gradient.min" => {
                        request_polygon_options_stroke_weight_styler_gradient_init(&mut request);
                        request.polygon_options.as_mut().unwrap().stroke_weight_styler.as_mut().unwrap().gradient.as_mut().unwrap().min = Some(arg_from_str(value.unwrap_or("0.0"), err, "polygon-options.stroke-weight-styler.gradient.min", "number"));
                    },
                "polygon-options.stroke-weight-styler.column-name" => {
                        request_polygon_options_stroke_weight_styler_gradient_init(&mut request);
                        request.polygon_options.as_mut().unwrap().stroke_weight_styler.as_mut().unwrap().column_name = Some(value.unwrap_or("").to_string());
                    },
                "polygon-options.stroke-weight-styler.kind" => {
                        request_polygon_options_stroke_weight_styler_gradient_init(&mut request);
                        request.polygon_options.as_mut().unwrap().stroke_weight_styler.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "polygon-options.fill-color-styler.gradient.max" => {
                        request_polygon_options_fill_color_styler_gradient_init(&mut request);
                        request.polygon_options.as_mut().unwrap().fill_color_styler.as_mut().unwrap().gradient.as_mut().unwrap().max = Some(arg_from_str(value.unwrap_or("0.0"), err, "polygon-options.fill-color-styler.gradient.max", "number"));
                    },
                "polygon-options.fill-color-styler.gradient.min" => {
                        request_polygon_options_fill_color_styler_gradient_init(&mut request);
                        request.polygon_options.as_mut().unwrap().fill_color_styler.as_mut().unwrap().gradient.as_mut().unwrap().min = Some(arg_from_str(value.unwrap_or("0.0"), err, "polygon-options.fill-color-styler.gradient.min", "number"));
                    },
                "polygon-options.fill-color-styler.column-name" => {
                        request_polygon_options_fill_color_styler_gradient_init(&mut request);
                        request.polygon_options.as_mut().unwrap().fill_color_styler.as_mut().unwrap().column_name = Some(value.unwrap_or("").to_string());
                    },
                "polygon-options.fill-color-styler.kind" => {
                        request_polygon_options_fill_color_styler_gradient_init(&mut request);
                        request.polygon_options.as_mut().unwrap().fill_color_styler.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "polygon-options.fill-color" => {
                        request_polygon_options_fill_color_styler_init(&mut request);
                        request.polygon_options.as_mut().unwrap().fill_color = Some(value.unwrap_or("").to_string());
                    },
                "polygon-options.stroke-color" => {
                        request_polygon_options_fill_color_styler_init(&mut request);
                        request.polygon_options.as_mut().unwrap().stroke_color = Some(value.unwrap_or("").to_string());
                    },
                "polygon-options.fill-opacity" => {
                        request_polygon_options_fill_color_styler_init(&mut request);
                        request.polygon_options.as_mut().unwrap().fill_opacity = Some(arg_from_str(value.unwrap_or("0.0"), err, "polygon-options.fill-opacity", "number"));
                    },
                "polyline-options.stroke-weight" => {
                        request_polyline_options_init(&mut request);
                        request.polyline_options.as_mut().unwrap().stroke_weight = Some(arg_from_str(value.unwrap_or("-0"), err, "polyline-options.stroke-weight", "integer"));
                    },
                "polyline-options.stroke-weight-styler.gradient.max" => {
                        request_polyline_options_stroke_weight_styler_gradient_init(&mut request);
                        request.polyline_options.as_mut().unwrap().stroke_weight_styler.as_mut().unwrap().gradient.as_mut().unwrap().max = Some(arg_from_str(value.unwrap_or("0.0"), err, "polyline-options.stroke-weight-styler.gradient.max", "number"));
                    },
                "polyline-options.stroke-weight-styler.gradient.min" => {
                        request_polyline_options_stroke_weight_styler_gradient_init(&mut request);
                        request.polyline_options.as_mut().unwrap().stroke_weight_styler.as_mut().unwrap().gradient.as_mut().unwrap().min = Some(arg_from_str(value.unwrap_or("0.0"), err, "polyline-options.stroke-weight-styler.gradient.min", "number"));
                    },
                "polyline-options.stroke-weight-styler.column-name" => {
                        request_polyline_options_stroke_weight_styler_gradient_init(&mut request);
                        request.polyline_options.as_mut().unwrap().stroke_weight_styler.as_mut().unwrap().column_name = Some(value.unwrap_or("").to_string());
                    },
                "polyline-options.stroke-weight-styler.kind" => {
                        request_polyline_options_stroke_weight_styler_gradient_init(&mut request);
                        request.polyline_options.as_mut().unwrap().stroke_weight_styler.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "polyline-options.stroke-color" => {
                        request_polyline_options_stroke_weight_styler_init(&mut request);
                        request.polyline_options.as_mut().unwrap().stroke_color = Some(value.unwrap_or("").to_string());
                    },
                "polyline-options.stroke-opacity" => {
                        request_polyline_options_stroke_weight_styler_init(&mut request);
                        request.polyline_options.as_mut().unwrap().stroke_opacity = Some(arg_from_str(value.unwrap_or("0.0"), err, "polyline-options.stroke-opacity", "number"));
                    },
                "polyline-options.stroke-color-styler.gradient.max" => {
                        request_polyline_options_stroke_color_styler_gradient_init(&mut request);
                        request.polyline_options.as_mut().unwrap().stroke_color_styler.as_mut().unwrap().gradient.as_mut().unwrap().max = Some(arg_from_str(value.unwrap_or("0.0"), err, "polyline-options.stroke-color-styler.gradient.max", "number"));
                    },
                "polyline-options.stroke-color-styler.gradient.min" => {
                        request_polyline_options_stroke_color_styler_gradient_init(&mut request);
                        request.polyline_options.as_mut().unwrap().stroke_color_styler.as_mut().unwrap().gradient.as_mut().unwrap().min = Some(arg_from_str(value.unwrap_or("0.0"), err, "polyline-options.stroke-color-styler.gradient.min", "number"));
                    },
                "polyline-options.stroke-color-styler.column-name" => {
                        request_polyline_options_stroke_color_styler_gradient_init(&mut request);
                        request.polyline_options.as_mut().unwrap().stroke_color_styler.as_mut().unwrap().column_name = Some(value.unwrap_or("").to_string());
                    },
                "polyline-options.stroke-color-styler.kind" => {
                        request_polyline_options_stroke_color_styler_gradient_init(&mut request);
                        request.polyline_options.as_mut().unwrap().stroke_color_styler.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "table-id" => {
                        request_polyline_options_init(&mut request);
                        request.table_id = Some(value.unwrap_or("").to_string());
                    },
                "style-id" => {
                        request_polyline_options_init(&mut request);
                        request.style_id = Some(arg_from_str(value.unwrap_or("-0"), err, "style-id", "integer"));
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _table_copy(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.table().copy(&self.opt.arg_table_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "copy-presentation" => {
                    call = call.copy_presentation(arg_from_str(value.unwrap_or("false"), err, "copy-presentation", "boolean"));
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

    fn _table_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.table().delete(&self.opt.arg_table_id);
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

    fn _table_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.table().get(&self.opt.arg_table_id);
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

    fn _table_import_rows(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.table().import_rows(&self.opt.arg_table_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-line" => {
                    call = call.start_line(arg_from_str(value.unwrap_or("-0"), err, "start-line", "integer"));
                },
                "is-strict" => {
                    call = call.is_strict(arg_from_str(value.unwrap_or("false"), err, "is-strict", "boolean"));
                },
                "end-line" => {
                    call = call.end_line(arg_from_str(value.unwrap_or("-0"), err, "end-line", "integer"));
                },
                "encoding" => {
                    call = call.encoding(value.unwrap_or(""));
                },
                "delimiter" => {
                    call = call.delimiter(value.unwrap_or(""));
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
        let protocol = 
            if self.opt.cmd_simple {
                "simple"
            } else if self.opt.cmd_resumable {
                "resumable"
            } else { 
                unreachable!() 
            };
        let mut input_file = input_file_from_opts(&self.opt.arg_file, err);
        let mime_type = input_mime_from_opts(&self.opt.arg_mime, err);
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "simple" => call.upload(input_file.unwrap(), mime_type.unwrap()),
                "resumable" => call.upload_resumable(input_file.unwrap(), mime_type.unwrap()),
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

    fn _table_import_table(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.table().import_table(&self.opt.arg_name);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "encoding" => {
                    call = call.encoding(value.unwrap_or(""));
                },
                "delimiter" => {
                    call = call.delimiter(value.unwrap_or(""));
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
        let protocol = 
            if self.opt.cmd_simple {
                "simple"
            } else if self.opt.cmd_resumable {
                "resumable"
            } else { 
                unreachable!() 
            };
        let mut input_file = input_file_from_opts(&self.opt.arg_file, err);
        let mime_type = input_mime_from_opts(&self.opt.arg_mime, err);
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "simple" => call.upload(input_file.unwrap(), mime_type.unwrap()),
                "resumable" => call.upload_resumable(input_file.unwrap(), mime_type.unwrap()),
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

    fn _table_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Table::default();
        let mut call = self.hub.table().insert(&request);
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
        
        let mut field_name = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            match &field_name.to_string()[..] {
                "table-properties-json-schema" => {
                        request.table_properties_json_schema = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "attribution" => {
                        request.attribution = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "column-properties-json-schema" => {
                        request.column_properties_json_schema = Some(value.unwrap_or("").to_string());
                    },
                "is-exportable" => {
                        request.is_exportable = Some(arg_from_str(value.unwrap_or("false"), err, "is-exportable", "boolean"));
                    },
                "base-table-ids" => {
                        if request.base_table_ids.is_none() {
                           request.base_table_ids = Some(Default::default());
                        }
                                        request.base_table_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "table-properties-json" => {
                        request.table_properties_json = Some(value.unwrap_or("").to_string());
                    },
                "attribution-link" => {
                        request.attribution_link = Some(value.unwrap_or("").to_string());
                    },
                "sql" => {
                        request.sql = Some(value.unwrap_or("").to_string());
                    },
                "table-id" => {
                        request.table_id = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _table_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.table().list();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _table_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Table::default();
        let mut call = self.hub.table().patch(&request, &self.opt.arg_table_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "replace-view-definition" => {
                    call = call.replace_view_definition(arg_from_str(value.unwrap_or("false"), err, "replace-view-definition", "boolean"));
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
        
        let mut field_name = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            match &field_name.to_string()[..] {
                "table-properties-json-schema" => {
                        request.table_properties_json_schema = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "attribution" => {
                        request.attribution = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "column-properties-json-schema" => {
                        request.column_properties_json_schema = Some(value.unwrap_or("").to_string());
                    },
                "is-exportable" => {
                        request.is_exportable = Some(arg_from_str(value.unwrap_or("false"), err, "is-exportable", "boolean"));
                    },
                "base-table-ids" => {
                        if request.base_table_ids.is_none() {
                           request.base_table_ids = Some(Default::default());
                        }
                                        request.base_table_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "table-properties-json" => {
                        request.table_properties_json = Some(value.unwrap_or("").to_string());
                    },
                "attribution-link" => {
                        request.attribution_link = Some(value.unwrap_or("").to_string());
                    },
                "sql" => {
                        request.sql = Some(value.unwrap_or("").to_string());
                    },
                "table-id" => {
                        request.table_id = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _table_replace_rows(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.table().replace_rows(&self.opt.arg_table_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-line" => {
                    call = call.start_line(arg_from_str(value.unwrap_or("-0"), err, "start-line", "integer"));
                },
                "is-strict" => {
                    call = call.is_strict(arg_from_str(value.unwrap_or("false"), err, "is-strict", "boolean"));
                },
                "end-line" => {
                    call = call.end_line(arg_from_str(value.unwrap_or("-0"), err, "end-line", "integer"));
                },
                "encoding" => {
                    call = call.encoding(value.unwrap_or(""));
                },
                "delimiter" => {
                    call = call.delimiter(value.unwrap_or(""));
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
        let protocol = 
            if self.opt.cmd_simple {
                "simple"
            } else if self.opt.cmd_resumable {
                "resumable"
            } else { 
                unreachable!() 
            };
        let mut input_file = input_file_from_opts(&self.opt.arg_file, err);
        let mime_type = input_mime_from_opts(&self.opt.arg_mime, err);
        if dry_run {
            None
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = writer_from_opts(self.opt.flag_o, &self.opt.arg_out);
            match match protocol {
                "simple" => call.upload(input_file.unwrap(), mime_type.unwrap()),
                "resumable" => call.upload_resumable(input_file.unwrap(), mime_type.unwrap()),
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

    fn _table_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Table::default();
        let mut call = self.hub.table().update(&request, &self.opt.arg_table_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "replace-view-definition" => {
                    call = call.replace_view_definition(arg_from_str(value.unwrap_or("false"), err, "replace-view-definition", "boolean"));
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
        
        let mut field_name = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            match &field_name.to_string()[..] {
                "table-properties-json-schema" => {
                        request.table_properties_json_schema = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "attribution" => {
                        request.attribution = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "column-properties-json-schema" => {
                        request.column_properties_json_schema = Some(value.unwrap_or("").to_string());
                    },
                "is-exportable" => {
                        request.is_exportable = Some(arg_from_str(value.unwrap_or("false"), err, "is-exportable", "boolean"));
                    },
                "base-table-ids" => {
                        if request.base_table_ids.is_none() {
                           request.base_table_ids = Some(Default::default());
                        }
                                        request.base_table_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "table-properties-json" => {
                        request.table_properties_json = Some(value.unwrap_or("").to_string());
                    },
                "attribution-link" => {
                        request.attribution_link = Some(value.unwrap_or("").to_string());
                    },
                "sql" => {
                        request.sql = Some(value.unwrap_or("").to_string());
                    },
                "table-id" => {
                        request.table_id = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _task_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.task().delete(&self.opt.arg_table_id, &self.opt.arg_task_id);
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

    fn _task_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.task().get(&self.opt.arg_table_id, &self.opt.arg_task_id);
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

    fn _task_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.task().list(&self.opt.arg_table_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
                },
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _template_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let template_id: i32 = arg_from_str(&self.opt.arg_template_id, err, "<template-id>", "integer");
        let mut call = self.hub.template().delete(&self.opt.arg_table_id, template_id);
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

    fn _template_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let template_id: i32 = arg_from_str(&self.opt.arg_template_id, err, "<template-id>", "integer");
        let mut call = self.hub.template().get(&self.opt.arg_table_id, template_id);
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

    fn _template_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Template::default();
        let mut call = self.hub.template().insert(&request, &self.opt.arg_table_id);
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
        
        let mut field_name = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            match &field_name.to_string()[..] {
                "body" => {
                        request.body = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "automatic-column-names" => {
                        if request.automatic_column_names.is_none() {
                           request.automatic_column_names = Some(Default::default());
                        }
                                        request.automatic_column_names.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "table-id" => {
                        request.table_id = Some(value.unwrap_or("").to_string());
                    },
                "template-id" => {
                        request.template_id = Some(arg_from_str(value.unwrap_or("-0"), err, "template-id", "integer"));
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _template_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.template().list(&self.opt.arg_table_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _template_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Template::default();
        let template_id: i32 = arg_from_str(&self.opt.arg_template_id, err, "<template-id>", "integer");
        let mut call = self.hub.template().patch(&request, &self.opt.arg_table_id, template_id);
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
        
        let mut field_name = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            match &field_name.to_string()[..] {
                "body" => {
                        request.body = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "automatic-column-names" => {
                        if request.automatic_column_names.is_none() {
                           request.automatic_column_names = Some(Default::default());
                        }
                                        request.automatic_column_names.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "table-id" => {
                        request.table_id = Some(value.unwrap_or("").to_string());
                    },
                "template-id" => {
                        request.template_id = Some(arg_from_str(value.unwrap_or("-0"), err, "template-id", "integer"));
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _template_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Template::default();
        let template_id: i32 = arg_from_str(&self.opt.arg_template_id, err, "<template-id>", "integer");
        let mut call = self.hub.template().update(&request, &self.opt.arg_table_id, template_id);
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
        
        let mut field_name = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            match &field_name.to_string()[..] {
                "body" => {
                        request.body = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "automatic-column-names" => {
                        if request.automatic_column_names.is_none() {
                           request.automatic_column_names = Some(Default::default());
                        }
                                        request.automatic_column_names.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "table-id" => {
                        request.table_id = Some(value.unwrap_or("").to_string());
                    },
                "template-id" => {
                        request.template_id = Some(arg_from_str(value.unwrap_or("-0"), err, "template-id", "integer"));
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

        if self.opt.cmd_column {
            if self.opt.cmd_delete {
                call_result = self._column_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._column_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._column_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._column_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._column_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._column_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_query {
            if self.opt.cmd_sql {
                call_result = self._query_sql(dry_run, &mut err);
            } else if self.opt.cmd_sql_get {
                call_result = self._query_sql_get(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_style {
            if self.opt.cmd_delete {
                call_result = self._style_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._style_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._style_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._style_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._style_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._style_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_table {
            if self.opt.cmd_copy {
                call_result = self._table_copy(dry_run, &mut err);
            } else if self.opt.cmd_delete {
                call_result = self._table_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._table_get(dry_run, &mut err);
            } else if self.opt.cmd_import_rows {
                call_result = self._table_import_rows(dry_run, &mut err);
            } else if self.opt.cmd_import_table {
                call_result = self._table_import_table(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._table_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._table_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._table_patch(dry_run, &mut err);
            } else if self.opt.cmd_replace_rows {
                call_result = self._table_replace_rows(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._table_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_task {
            if self.opt.cmd_delete {
                call_result = self._task_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._task_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._task_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_template {
            if self.opt.cmd_delete {
                call_result = self._template_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._template_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._template_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._template_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._template_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._template_update(dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "fusiontables2-secret.json", 
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
                                          program_name: "fusiontables2",
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
            hub: api::Fusiontables::new(client, auth),
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
            writeln!(io::stderr(), "{}", err).ok();
            env::set_exit_status(err.exit_code);
        },
        Ok(engine) => {
            if let Some(err) = engine.doit() {
                writeln!(io::stderr(), "{:?}", err).ok();
                writeln!(io::stderr(), "{}", err).ok();
                env::set_exit_status(1);
            }
        }
    }
}