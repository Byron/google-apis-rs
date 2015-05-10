// DO NOT EDIT !
// This file was generated automatically from 'src/mako/cli/main.rs.mako'
// DO NOT EDIT !
#![feature(plugin, exit_status)]
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;
extern crate yup_oauth2 as oauth2;
extern crate yup_hyper_mock as mock;
extern crate serde;
extern crate hyper;
extern crate mime;
extern crate strsim;
extern crate google_datastore1_beta2 as api;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

mod cmn;

use cmn::{InvalidOptionsError, CLIError, JsonTokenStorage, arg_from_str, writer_from_opts, parse_kv_arg, 
          input_file_from_opts, input_mime_from_opts, FieldCursor, FieldError, CallType, UploadProtocol,
          calltype_from_str, remove_json_null_values};

use std::default::Default;
use std::str::FromStr;

use oauth2::{Authenticator, DefaultAuthenticatorDelegate};
use serde::json;
use clap::ArgMatches;

enum DoitError {
    IoError(String, io::Error),
    ApiError(api::Error),
}

struct Engine<'n, 'a> {
    opt: ArgMatches<'n, 'a>,
    hub: api::Datastore<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n, 'a> Engine<'n, 'a> {
    fn _datasets_allocate_ids(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::AllocateIdsRequest::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            match &temp_cursor.to_string()[..] {
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec![]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.datasets().allocate_ids(request, opt.value_of("dataset-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _datasets_begin_transaction(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::BeginTransactionRequest::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            match &temp_cursor.to_string()[..] {
                "isolation-level" => {
                        request.isolation_level = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["isolation-level"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.datasets().begin_transaction(request, opt.value_of("dataset-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _datasets_commit(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::CommitRequest::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            fn request_mutation_init(request: &mut api::CommitRequest) {
                if request.mutation.is_none() {
                    request.mutation = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
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
                        request.mutation.as_mut().unwrap().force = Some(arg_from_str(value.unwrap_or("false"), err, "mutation.force", "boolean"));
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["force", "ignore-read-only", "mode", "mutation", "transaction"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.datasets().commit(request, opt.value_of("dataset-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _datasets_lookup(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::LookupRequest::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            fn request_read_options_init(request: &mut api::LookupRequest) {
                if request.read_options.is_none() {
                    request.read_options = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "read-options.transaction" => {
                        request_read_options_init(&mut request);
                        request.read_options.as_mut().unwrap().transaction = Some(value.unwrap_or("").to_string());
                    },
                "read-options.read-consistency" => {
                        request_read_options_init(&mut request);
                        request.read_options.as_mut().unwrap().read_consistency = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["read-consistency", "read-options", "transaction"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.datasets().lookup(request, opt.value_of("dataset-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _datasets_rollback(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::RollbackRequest::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            match &temp_cursor.to_string()[..] {
                "transaction" => {
                        request.transaction = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["transaction"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.datasets().rollback(request, opt.value_of("dataset-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _datasets_run_query(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::RunQueryRequest::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in opt.values_of("kv").unwrap_or(Vec::new()).iter() {
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
            
            fn request_query_filter_composite_filter_init(request: &mut api::RunQueryRequest) {
                request_query_filter_init(request);
                if request.query.as_mut().unwrap().filter.as_mut().unwrap().composite_filter.is_none() {
                    request.query.as_mut().unwrap().filter.as_mut().unwrap().composite_filter = Some(Default::default());
                }
            }
            
            fn request_query_filter_init(request: &mut api::RunQueryRequest) {
                request_query_init(request);
                if request.query.as_mut().unwrap().filter.is_none() {
                    request.query.as_mut().unwrap().filter = Some(Default::default());
                }
            }
            
            fn request_query_filter_property_filter_init(request: &mut api::RunQueryRequest) {
                request_query_filter_init(request);
                if request.query.as_mut().unwrap().filter.as_mut().unwrap().property_filter.is_none() {
                    request.query.as_mut().unwrap().filter.as_mut().unwrap().property_filter = Some(Default::default());
                }
            }
            
            fn request_query_filter_property_filter_property_init(request: &mut api::RunQueryRequest) {
                request_query_filter_property_filter_init(request);
                if request.query.as_mut().unwrap().filter.as_mut().unwrap().property_filter.as_mut().unwrap().property.is_none() {
                    request.query.as_mut().unwrap().filter.as_mut().unwrap().property_filter.as_mut().unwrap().property = Some(Default::default());
                }
            }
            
            fn request_query_filter_property_filter_value_entity_value_init(request: &mut api::RunQueryRequest) {
                request_query_filter_property_filter_value_init(request);
                if request.query.as_mut().unwrap().filter.as_mut().unwrap().property_filter.as_mut().unwrap().value.as_mut().unwrap().entity_value.is_none() {
                    request.query.as_mut().unwrap().filter.as_mut().unwrap().property_filter.as_mut().unwrap().value.as_mut().unwrap().entity_value = Some(Default::default());
                }
            }
            
            fn request_query_filter_property_filter_value_entity_value_key_init(request: &mut api::RunQueryRequest) {
                request_query_filter_property_filter_value_entity_value_init(request);
                if request.query.as_mut().unwrap().filter.as_mut().unwrap().property_filter.as_mut().unwrap().value.as_mut().unwrap().entity_value.as_mut().unwrap().key.is_none() {
                    request.query.as_mut().unwrap().filter.as_mut().unwrap().property_filter.as_mut().unwrap().value.as_mut().unwrap().entity_value.as_mut().unwrap().key = Some(Default::default());
                }
            }
            
            fn request_query_filter_property_filter_value_entity_value_key_partition_id_init(request: &mut api::RunQueryRequest) {
                request_query_filter_property_filter_value_entity_value_key_init(request);
                if request.query.as_mut().unwrap().filter.as_mut().unwrap().property_filter.as_mut().unwrap().value.as_mut().unwrap().entity_value.as_mut().unwrap().key.as_mut().unwrap().partition_id.is_none() {
                    request.query.as_mut().unwrap().filter.as_mut().unwrap().property_filter.as_mut().unwrap().value.as_mut().unwrap().entity_value.as_mut().unwrap().key.as_mut().unwrap().partition_id = Some(Default::default());
                }
            }
            
            fn request_query_filter_property_filter_value_init(request: &mut api::RunQueryRequest) {
                request_query_filter_property_filter_init(request);
                if request.query.as_mut().unwrap().filter.as_mut().unwrap().property_filter.as_mut().unwrap().value.is_none() {
                    request.query.as_mut().unwrap().filter.as_mut().unwrap().property_filter.as_mut().unwrap().value = Some(Default::default());
                }
            }
            
            fn request_query_filter_property_filter_value_key_value_init(request: &mut api::RunQueryRequest) {
                request_query_filter_property_filter_value_init(request);
                if request.query.as_mut().unwrap().filter.as_mut().unwrap().property_filter.as_mut().unwrap().value.as_mut().unwrap().key_value.is_none() {
                    request.query.as_mut().unwrap().filter.as_mut().unwrap().property_filter.as_mut().unwrap().value.as_mut().unwrap().key_value = Some(Default::default());
                }
            }
            
            fn request_query_filter_property_filter_value_key_value_partition_id_init(request: &mut api::RunQueryRequest) {
                request_query_filter_property_filter_value_key_value_init(request);
                if request.query.as_mut().unwrap().filter.as_mut().unwrap().property_filter.as_mut().unwrap().value.as_mut().unwrap().key_value.as_mut().unwrap().partition_id.is_none() {
                    request.query.as_mut().unwrap().filter.as_mut().unwrap().property_filter.as_mut().unwrap().value.as_mut().unwrap().key_value.as_mut().unwrap().partition_id = Some(Default::default());
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
            
            match &temp_cursor.to_string()[..] {
                "query.filter.composite-filter.operator" => {
                        request_query_filter_composite_filter_init(&mut request);
                        request.query.as_mut().unwrap().filter.as_mut().unwrap().composite_filter.as_mut().unwrap().operator = Some(value.unwrap_or("").to_string());
                    },
                "query.filter.property-filter.operator" => {
                        request_query_filter_property_filter_init(&mut request);
                        request.query.as_mut().unwrap().filter.as_mut().unwrap().property_filter.as_mut().unwrap().operator = Some(value.unwrap_or("").to_string());
                    },
                "query.filter.property-filter.property.name" => {
                        request_query_filter_property_filter_property_init(&mut request);
                        request.query.as_mut().unwrap().filter.as_mut().unwrap().property_filter.as_mut().unwrap().property.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "query.filter.property-filter.value.entity-value.key.partition-id.namespace" => {
                        request_query_filter_property_filter_value_entity_value_key_partition_id_init(&mut request);
                        request.query.as_mut().unwrap().filter.as_mut().unwrap().property_filter.as_mut().unwrap().value.as_mut().unwrap().entity_value.as_mut().unwrap().key.as_mut().unwrap().partition_id.as_mut().unwrap().namespace = Some(value.unwrap_or("").to_string());
                    },
                "query.filter.property-filter.value.entity-value.key.partition-id.dataset-id" => {
                        request_query_filter_property_filter_value_entity_value_key_partition_id_init(&mut request);
                        request.query.as_mut().unwrap().filter.as_mut().unwrap().property_filter.as_mut().unwrap().value.as_mut().unwrap().entity_value.as_mut().unwrap().key.as_mut().unwrap().partition_id.as_mut().unwrap().dataset_id = Some(value.unwrap_or("").to_string());
                    },
                "query.filter.property-filter.value.double-value" => {
                        request_query_filter_property_filter_value_entity_value_init(&mut request);
                        request.query.as_mut().unwrap().filter.as_mut().unwrap().property_filter.as_mut().unwrap().value.as_mut().unwrap().double_value = Some(arg_from_str(value.unwrap_or("0.0"), err, "query.filter.property-filter.value.double-value", "number"));
                    },
                "query.filter.property-filter.value.integer-value" => {
                        request_query_filter_property_filter_value_entity_value_init(&mut request);
                        request.query.as_mut().unwrap().filter.as_mut().unwrap().property_filter.as_mut().unwrap().value.as_mut().unwrap().integer_value = Some(value.unwrap_or("").to_string());
                    },
                "query.filter.property-filter.value.meaning" => {
                        request_query_filter_property_filter_value_entity_value_init(&mut request);
                        request.query.as_mut().unwrap().filter.as_mut().unwrap().property_filter.as_mut().unwrap().value.as_mut().unwrap().meaning = Some(arg_from_str(value.unwrap_or("-0"), err, "query.filter.property-filter.value.meaning", "integer"));
                    },
                "query.filter.property-filter.value.date-time-value" => {
                        request_query_filter_property_filter_value_entity_value_init(&mut request);
                        request.query.as_mut().unwrap().filter.as_mut().unwrap().property_filter.as_mut().unwrap().value.as_mut().unwrap().date_time_value = Some(value.unwrap_or("").to_string());
                    },
                "query.filter.property-filter.value.key-value.partition-id.namespace" => {
                        request_query_filter_property_filter_value_key_value_partition_id_init(&mut request);
                        request.query.as_mut().unwrap().filter.as_mut().unwrap().property_filter.as_mut().unwrap().value.as_mut().unwrap().key_value.as_mut().unwrap().partition_id.as_mut().unwrap().namespace = Some(value.unwrap_or("").to_string());
                    },
                "query.filter.property-filter.value.key-value.partition-id.dataset-id" => {
                        request_query_filter_property_filter_value_key_value_partition_id_init(&mut request);
                        request.query.as_mut().unwrap().filter.as_mut().unwrap().property_filter.as_mut().unwrap().value.as_mut().unwrap().key_value.as_mut().unwrap().partition_id.as_mut().unwrap().dataset_id = Some(value.unwrap_or("").to_string());
                    },
                "query.filter.property-filter.value.string-value" => {
                        request_query_filter_property_filter_value_key_value_init(&mut request);
                        request.query.as_mut().unwrap().filter.as_mut().unwrap().property_filter.as_mut().unwrap().value.as_mut().unwrap().string_value = Some(value.unwrap_or("").to_string());
                    },
                "query.filter.property-filter.value.indexed" => {
                        request_query_filter_property_filter_value_key_value_init(&mut request);
                        request.query.as_mut().unwrap().filter.as_mut().unwrap().property_filter.as_mut().unwrap().value.as_mut().unwrap().indexed = Some(arg_from_str(value.unwrap_or("false"), err, "query.filter.property-filter.value.indexed", "boolean"));
                    },
                "query.filter.property-filter.value.blob-value" => {
                        request_query_filter_property_filter_value_key_value_init(&mut request);
                        request.query.as_mut().unwrap().filter.as_mut().unwrap().property_filter.as_mut().unwrap().value.as_mut().unwrap().blob_value = Some(value.unwrap_or("").to_string());
                    },
                "query.filter.property-filter.value.boolean-value" => {
                        request_query_filter_property_filter_value_key_value_init(&mut request);
                        request.query.as_mut().unwrap().filter.as_mut().unwrap().property_filter.as_mut().unwrap().value.as_mut().unwrap().boolean_value = Some(arg_from_str(value.unwrap_or("false"), err, "query.filter.property-filter.value.boolean-value", "boolean"));
                    },
                "query.filter.property-filter.value.blob-key-value" => {
                        request_query_filter_property_filter_value_key_value_init(&mut request);
                        request.query.as_mut().unwrap().filter.as_mut().unwrap().property_filter.as_mut().unwrap().value.as_mut().unwrap().blob_key_value = Some(value.unwrap_or("").to_string());
                    },
                "query.start-cursor" => {
                        request_query_filter_init(&mut request);
                        request.query.as_mut().unwrap().start_cursor = Some(value.unwrap_or("").to_string());
                    },
                "query.end-cursor" => {
                        request_query_filter_init(&mut request);
                        request.query.as_mut().unwrap().end_cursor = Some(value.unwrap_or("").to_string());
                    },
                "query.limit" => {
                        request_query_filter_init(&mut request);
                        request.query.as_mut().unwrap().limit = Some(arg_from_str(value.unwrap_or("-0"), err, "query.limit", "integer"));
                    },
                "query.offset" => {
                        request_query_filter_init(&mut request);
                        request.query.as_mut().unwrap().offset = Some(arg_from_str(value.unwrap_or("-0"), err, "query.offset", "integer"));
                    },
                "partition-id.namespace" => {
                        request_partition_id_init(&mut request);
                        request.partition_id.as_mut().unwrap().namespace = Some(value.unwrap_or("").to_string());
                    },
                "partition-id.dataset-id" => {
                        request_partition_id_init(&mut request);
                        request.partition_id.as_mut().unwrap().dataset_id = Some(value.unwrap_or("").to_string());
                    },
                "gql-query.query-string" => {
                        request_gql_query_init(&mut request);
                        request.gql_query.as_mut().unwrap().query_string = Some(value.unwrap_or("").to_string());
                    },
                "gql-query.allow-literal" => {
                        request_gql_query_init(&mut request);
                        request.gql_query.as_mut().unwrap().allow_literal = Some(arg_from_str(value.unwrap_or("false"), err, "gql-query.allow-literal", "boolean"));
                    },
                "read-options.transaction" => {
                        request_read_options_init(&mut request);
                        request.read_options.as_mut().unwrap().transaction = Some(value.unwrap_or("").to_string());
                    },
                "read-options.read-consistency" => {
                        request_read_options_init(&mut request);
                        request.read_options.as_mut().unwrap().read_consistency = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["allow-literal", "blob-key-value", "blob-value", "boolean-value", "composite-filter", "dataset-id", "date-time-value", "double-value", "end-cursor", "entity-value", "filter", "gql-query", "indexed", "integer-value", "key", "key-value", "limit", "meaning", "name", "namespace", "offset", "operator", "partition-id", "property", "property-filter", "query", "query-string", "read-consistency", "read-options", "start-cursor", "string-value", "transaction", "value"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.datasets().run_query(request, opt.value_of("dataset-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
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
                                                Vec::new() + &self.gp + &[]
                                                            ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
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
            ("datasets", Some(opt)) => {
                match opt.subcommand() {
                    ("allocate-ids", Some(opt)) => {
                        call_result = self._datasets_allocate_ids(opt, dry_run, &mut err);
                    },
                    ("begin-transaction", Some(opt)) => {
                        call_result = self._datasets_begin_transaction(opt, dry_run, &mut err);
                    },
                    ("commit", Some(opt)) => {
                        call_result = self._datasets_commit(opt, dry_run, &mut err);
                    },
                    ("lookup", Some(opt)) => {
                        call_result = self._datasets_lookup(opt, dry_run, &mut err);
                    },
                    ("rollback", Some(opt)) => {
                        call_result = self._datasets_rollback(opt, dry_run, &mut err);
                    },
                    ("run-query", Some(opt)) => {
                        call_result = self._datasets_run_query(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("datasets".to_string()));
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
    fn new(opt: ArgMatches<'a, 'n>) -> Result<Engine<'a, 'n>, InvalidOptionsError> {
        let (config_dir, secret) = {
            let config_dir = match cmn::assure_config_dir_exists(opt.value_of("folder").unwrap_or("~/.google-service-cli")) {
                Err(e) => return Err(InvalidOptionsError::single(e, 3)),
                Ok(p) => p,
            };

            match cmn::application_secret_from_directory(&config_dir, "datastore1-beta2-secret.json", 
                                                         "{\"installed\":{\"auth_uri\":\"https://accounts.google.com/o/oauth2/auth\",\"client_secret\":\"hCsslbCUyfehWMmbkG8vTYxG\",\"token_uri\":\"https://accounts.google.com/o/oauth2/token\",\"client_email\":\"\",\"redirect_uris\":[\"urn:ietf:wg:oauth:2.0:oob\",\"oob\"],\"client_x509_cert_url\":\"\",\"client_id\":\"620010449518-9ngf7o4dhs0dka470npqvor6dc5lqb9b.apps.googleusercontent.com\",\"auth_provider_x509_cert_url\":\"https://www.googleapis.com/oauth2/v1/certs\"}}") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let auth = Authenticator::new(  &secret, DefaultAuthenticatorDelegate,
                                        if opt.is_present("debug-auth") {
                                            hyper::Client::with_connector(mock::TeeConnector {
                                                    connector: hyper::net::HttpConnector(None) 
                                                })
                                        } else {
                                            hyper::Client::new()
                                        },
                                        JsonTokenStorage {
                                          program_name: "datastore1-beta2",
                                          db_dir: config_dir.clone(),
                                        }, None);

        let client = 
            if opt.is_present("debug") {
                hyper::Client::with_connector(mock::TeeConnector {
                        connector: hyper::net::HttpConnector(None) 
                    })
            } else {
                hyper::Client::new()
            };
        let engine = Engine {
            opt: opt,
            hub: api::Datastore::new(client, auth),
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
    let arg_data = [
        ("datasets", "methods: 'allocate-ids', 'begin-transaction', 'commit', 'lookup', 'rollback' and 'run-query'", vec![
            ("allocate-ids",  
                    Some(r##"Allocate IDs for incomplete keys (useful for referencing an entity before it is inserted)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_datastore1_beta2_cli/datasets_allocate-ids",
                  vec![
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Identifies the dataset."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
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
            ("begin-transaction",  
                    Some(r##"Begin a new transaction."##),
                    "Details at http://byron.github.io/google-apis-rs/google_datastore1_beta2_cli/datasets_begin-transaction",
                  vec![
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Identifies the dataset."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
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
            ("commit",  
                    Some(r##"Commit a transaction, optionally creating, deleting or modifying some entities."##),
                    "Details at http://byron.github.io/google-apis-rs/google_datastore1_beta2_cli/datasets_commit",
                  vec![
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Identifies the dataset."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
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
            ("lookup",  
                    Some(r##"Look up some entities by key."##),
                    "Details at http://byron.github.io/google-apis-rs/google_datastore1_beta2_cli/datasets_lookup",
                  vec![
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Identifies the dataset."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
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
            ("rollback",  
                    Some(r##"Roll back a transaction."##),
                    "Details at http://byron.github.io/google-apis-rs/google_datastore1_beta2_cli/datasets_rollback",
                  vec![
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Identifies the dataset."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
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
            ("run-query",  
                    Some(r##"Query for entities."##),
                    "Details at http://byron.github.io/google-apis-rs/google_datastore1_beta2_cli/datasets_run-query",
                  vec![
                    (Some(r##"dataset-id"##),
                     None,
                     Some(r##"Identifies the dataset."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
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
    
    let mut app = App::new("datastore1-beta2")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("0.2.0+20150402")
           .about("API for accessing Google Cloud Datastore.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_datastore1_beta2_cli")
           .arg(Arg::with_name("url")
                   .long("scope")
                   .help("Specify the authentication a method should be executed in. Each scope requires the user to grant this application permission to use it.If unset, it defaults to the shortest scope url for a particular method.")
                   .multiple(true)
                   .takes_value(true))
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
           
           for &(main_command_name, ref about, ref subcommands) in arg_data.iter() {
               let mut mcmd = SubCommand::new(main_command_name).about(about);
           
               for &(sub_command_name, ref desc, url_info, ref args) in subcommands {
                   let mut scmd = SubCommand::new(sub_command_name);
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
                       let mut arg = Arg::with_name(arg_name_str);
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
            env::set_exit_status(err.exit_code);
            writeln!(io::stderr(), "{}", err).ok();
        },
        Ok(engine) => {
            if let Err(doit_err) = engine.doit() {
                env::set_exit_status(1);
                match doit_err {
                    DoitError::IoError(path, err) => {
                        writeln!(io::stderr(), "Failed to open output file '{}': {}", path, err).ok();
                    },
                    DoitError::ApiError(err) => {
                        if debug {
                            writeln!(io::stderr(), "{:?}", err).ok();
                        } else {
                            writeln!(io::stderr(), "{}", err).ok();
                        }
                    }
                }
            }
        }
    }
}