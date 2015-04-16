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
extern crate google_datastore1_beta2 as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  datastore1-beta2 [options] datasets allocate-ids <dataset-id> -r <kv>... [-p <v>]... [-o <out>]
  datastore1-beta2 [options] datasets begin-transaction <dataset-id> -r <kv>... [-p <v>]... [-o <out>]
  datastore1-beta2 [options] datasets commit <dataset-id> -r <kv>... [-p <v>]... [-o <out>]
  datastore1-beta2 [options] datasets lookup <dataset-id> -r <kv>... [-p <v>]... [-o <out>]
  datastore1-beta2 [options] datasets rollback <dataset-id> -r <kv>... [-p <v>]... [-o <out>]
  datastore1-beta2 [options] datasets run-query <dataset-id> -r <kv>... [-p <v>]... [-o <out>]
  datastore1-beta2 --help

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
    hub: api::Datastore<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _datasets_allocate_ids(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::AllocateIdsRequest = Default::default();
        let mut call = self.hub.datasets().allocate_ids(&request, &self.opt.arg_dataset_id);
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
            match &field_name.to_string()[..] {
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

    fn _datasets_begin_transaction(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::BeginTransactionRequest = Default::default();
        let mut call = self.hub.datasets().begin_transaction(&request, &self.opt.arg_dataset_id);
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
            match &field_name.to_string()[..] {
                "isolation-level" => {
                        request.isolation_level = Some(value.unwrap_or("").to_string());
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

    fn _datasets_commit(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::CommitRequest = Default::default();
        let mut call = self.hub.datasets().commit(&request, &self.opt.arg_dataset_id);
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
            fn request_mutation_init(request: &mut api::CommitRequest) {
                if request.mutation.is_none() {
                    request.mutation = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "ignore-read-only" => {
                        request.ignore_read_only = Some(arg_from_str(value.unwrap_or("false"), err, "ignore-read-only", "boolean"));
                    },
                "transaction" => {
                        request.transaction = Some(value.unwrap_or("").to_string());
                    },
                "mode" => {
                        request.mode = Some(value.unwrap_or("").to_string());
                    },
                "mutation.force" => {
                        request_mutation_init(&mut request);
                        request.mutation.as_mut().unwrap().force = arg_from_str(value.unwrap_or("false"), err, "mutation.force", "boolean");
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

    fn _datasets_lookup(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::LookupRequest = Default::default();
        let mut call = self.hub.datasets().lookup(&request, &self.opt.arg_dataset_id);
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
            fn request_read_options_init(request: &mut api::LookupRequest) {
                if request.read_options.is_none() {
                    request.read_options = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "read-options.transaction" => {
                        request_read_options_init(&mut request);
                        request.read_options.as_mut().unwrap().transaction = value.unwrap_or("").to_string();
                    },
                "read-options.read-consistency" => {
                        request_read_options_init(&mut request);
                        request.read_options.as_mut().unwrap().read_consistency = value.unwrap_or("").to_string();
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

    fn _datasets_rollback(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::RollbackRequest = Default::default();
        let mut call = self.hub.datasets().rollback(&request, &self.opt.arg_dataset_id);
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
            match &field_name.to_string()[..] {
                "transaction" => {
                        request.transaction = Some(value.unwrap_or("").to_string());
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

    fn _datasets_run_query(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::RunQueryRequest = Default::default();
        let mut call = self.hub.datasets().run_query(&request, &self.opt.arg_dataset_id);
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
            fn request_gql_query_init(request: &mut api::RunQueryRequest) {
                if request.gql_query.is_none() {
                    request.gql_query = Some(Default::default());
                }
            }
            
            fn request_partition_id_init(request: &mut api::RunQueryRequest) {
                if request.partition_id.is_none() {
                    request.partition_id = Some(Default::default());
                }
            }
            
            fn request_query_init(request: &mut api::RunQueryRequest) {
                if request.query.is_none() {
                    request.query = Some(Default::default());
                }
            }
            
            fn request_read_options_init(request: &mut api::RunQueryRequest) {
                if request.read_options.is_none() {
                    request.read_options = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "query.filter.composite-filter.operator" => {
                        request_query_init(&mut request);
                        request.query.as_mut().unwrap().filter.composite_filter.operator = value.unwrap_or("").to_string();
                    },
                "query.filter.property-filter.operator" => {
                        request_query_init(&mut request);
                        request.query.as_mut().unwrap().filter.property_filter.operator = value.unwrap_or("").to_string();
                    },
                "query.filter.property-filter.property.name" => {
                        request_query_init(&mut request);
                        request.query.as_mut().unwrap().filter.property_filter.property.name = value.unwrap_or("").to_string();
                    },
                "query.filter.property-filter.value.entity-value.key.partition-id.namespace" => {
                        request_query_init(&mut request);
                        request.query.as_mut().unwrap().filter.property_filter.value.entity_value.key.partition_id.namespace = value.unwrap_or("").to_string();
                    },
                "query.filter.property-filter.value.entity-value.key.partition-id.dataset-id" => {
                        request_query_init(&mut request);
                        request.query.as_mut().unwrap().filter.property_filter.value.entity_value.key.partition_id.dataset_id = value.unwrap_or("").to_string();
                    },
                "query.filter.property-filter.value.double-value" => {
                        request_query_init(&mut request);
                        request.query.as_mut().unwrap().filter.property_filter.value.double_value = arg_from_str(value.unwrap_or("0.0"), err, "query.filter.property-filter.value.double-value", "number");
                    },
                "query.filter.property-filter.value.integer-value" => {
                        request_query_init(&mut request);
                        request.query.as_mut().unwrap().filter.property_filter.value.integer_value = value.unwrap_or("").to_string();
                    },
                "query.filter.property-filter.value.meaning" => {
                        request_query_init(&mut request);
                        request.query.as_mut().unwrap().filter.property_filter.value.meaning = arg_from_str(value.unwrap_or("-0"), err, "query.filter.property-filter.value.meaning", "integer");
                    },
                "query.filter.property-filter.value.date-time-value" => {
                        request_query_init(&mut request);
                        request.query.as_mut().unwrap().filter.property_filter.value.date_time_value = value.unwrap_or("").to_string();
                    },
                "query.filter.property-filter.value.key-value.partition-id.namespace" => {
                        request_query_init(&mut request);
                        request.query.as_mut().unwrap().filter.property_filter.value.key_value.partition_id.namespace = value.unwrap_or("").to_string();
                    },
                "query.filter.property-filter.value.key-value.partition-id.dataset-id" => {
                        request_query_init(&mut request);
                        request.query.as_mut().unwrap().filter.property_filter.value.key_value.partition_id.dataset_id = value.unwrap_or("").to_string();
                    },
                "query.filter.property-filter.value.string-value" => {
                        request_query_init(&mut request);
                        request.query.as_mut().unwrap().filter.property_filter.value.string_value = value.unwrap_or("").to_string();
                    },
                "query.filter.property-filter.value.indexed" => {
                        request_query_init(&mut request);
                        request.query.as_mut().unwrap().filter.property_filter.value.indexed = arg_from_str(value.unwrap_or("false"), err, "query.filter.property-filter.value.indexed", "boolean");
                    },
                "query.filter.property-filter.value.blob-value" => {
                        request_query_init(&mut request);
                        request.query.as_mut().unwrap().filter.property_filter.value.blob_value = value.unwrap_or("").to_string();
                    },
                "query.filter.property-filter.value.boolean-value" => {
                        request_query_init(&mut request);
                        request.query.as_mut().unwrap().filter.property_filter.value.boolean_value = arg_from_str(value.unwrap_or("false"), err, "query.filter.property-filter.value.boolean-value", "boolean");
                    },
                "query.filter.property-filter.value.blob-key-value" => {
                        request_query_init(&mut request);
                        request.query.as_mut().unwrap().filter.property_filter.value.blob_key_value = value.unwrap_or("").to_string();
                    },
                "query.start-cursor" => {
                        request_query_init(&mut request);
                        request.query.as_mut().unwrap().start_cursor = value.unwrap_or("").to_string();
                    },
                "query.end-cursor" => {
                        request_query_init(&mut request);
                        request.query.as_mut().unwrap().end_cursor = value.unwrap_or("").to_string();
                    },
                "query.limit" => {
                        request_query_init(&mut request);
                        request.query.as_mut().unwrap().limit = arg_from_str(value.unwrap_or("-0"), err, "query.limit", "integer");
                    },
                "query.offset" => {
                        request_query_init(&mut request);
                        request.query.as_mut().unwrap().offset = arg_from_str(value.unwrap_or("-0"), err, "query.offset", "integer");
                    },
                "partition-id.namespace" => {
                        request_partition_id_init(&mut request);
                        request.partition_id.as_mut().unwrap().namespace = value.unwrap_or("").to_string();
                    },
                "partition-id.dataset-id" => {
                        request_partition_id_init(&mut request);
                        request.partition_id.as_mut().unwrap().dataset_id = value.unwrap_or("").to_string();
                    },
                "gql-query.query-string" => {
                        request_gql_query_init(&mut request);
                        request.gql_query.as_mut().unwrap().query_string = value.unwrap_or("").to_string();
                    },
                "gql-query.allow-literal" => {
                        request_gql_query_init(&mut request);
                        request.gql_query.as_mut().unwrap().allow_literal = arg_from_str(value.unwrap_or("false"), err, "gql-query.allow-literal", "boolean");
                    },
                "read-options.transaction" => {
                        request_read_options_init(&mut request);
                        request.read_options.as_mut().unwrap().transaction = value.unwrap_or("").to_string();
                    },
                "read-options.read-consistency" => {
                        request_read_options_init(&mut request);
                        request.read_options.as_mut().unwrap().read_consistency = value.unwrap_or("").to_string();
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

        if self.opt.cmd_datasets {
            if self.opt.cmd_allocate_ids {
                call_result = self._datasets_allocate_ids(dry_run, &mut err);
            } else if self.opt.cmd_begin_transaction {
                call_result = self._datasets_begin_transaction(dry_run, &mut err);
            } else if self.opt.cmd_commit {
                call_result = self._datasets_commit(dry_run, &mut err);
            } else if self.opt.cmd_lookup {
                call_result = self._datasets_lookup(dry_run, &mut err);
            } else if self.opt.cmd_rollback {
                call_result = self._datasets_rollback(dry_run, &mut err);
            } else if self.opt.cmd_run_query {
                call_result = self._datasets_run_query(dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "datastore1-beta2-secret.json") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                                      hyper::Client::new(),
                                      JsonTokenStorage {
                                        program_name: "datastore1-beta2",
                                        db_dir: config_dir.clone(),
                                      }, None);
        let engine = Engine {
            opt: opt,
            hub: api::Datastore::new(hyper::Client::new(), auth),
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