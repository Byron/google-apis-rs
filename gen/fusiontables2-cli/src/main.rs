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
extern crate google_fusiontables2 as api;

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
    hub: api::Fusiontables<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n, 'a> Engine<'n, 'a> {
    fn _column_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.column().delete(opt.value_of("table-id").unwrap_or(""), opt.value_of("column-id").unwrap_or(""));
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
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _column_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.column().get(opt.value_of("table-id").unwrap_or(""), opt.value_of("column-id").unwrap_or(""));
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

    fn _column_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Column::default();
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
            fn request_base_column_init(request: &mut api::Column) {
                if request.base_column.is_none() {
                    request.base_column = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
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
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["base-column", "column-id", "column-json-schema", "column-properties-json", "description", "format-pattern", "graph-predicate", "kind", "name", "table-index", "type", "valid-values", "validate-data"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.column().insert(request, opt.value_of("table-id").unwrap_or(""));
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

    fn _column_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.column().list(opt.value_of("table-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
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
                                                Vec::new() + &self.gp + &["page-token", "max-results"]
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

    fn _column_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Column::default();
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
            fn request_base_column_init(request: &mut api::Column) {
                if request.base_column.is_none() {
                    request.base_column = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
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
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["base-column", "column-id", "column-json-schema", "column-properties-json", "description", "format-pattern", "graph-predicate", "kind", "name", "table-index", "type", "valid-values", "validate-data"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.column().patch(request, opt.value_of("table-id").unwrap_or(""), opt.value_of("column-id").unwrap_or(""));
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

    fn _column_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Column::default();
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
            fn request_base_column_init(request: &mut api::Column) {
                if request.base_column.is_none() {
                    request.base_column = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
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
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["base-column", "column-id", "column-json-schema", "column-properties-json", "description", "format-pattern", "graph-predicate", "kind", "name", "table-index", "type", "valid-values", "validate-data"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.column().update(request, opt.value_of("table-id").unwrap_or(""), opt.value_of("column-id").unwrap_or(""));
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

    fn _query_sql(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut download_mode = false;
        let mut call = self.hub.query().sql(opt.value_of("sql").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "typed" => {
                    call = call.typed(arg_from_str(value.unwrap_or("false"), err, "typed", "boolean"));
                },
                "hdrs" => {
                    call = call.hdrs(arg_from_str(value.unwrap_or("false"), err, "hdrs", "boolean"));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            if key == "alt" && value.unwrap_or("unset") == "media" {
                                download_mode = true;
                            }
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                Vec::new() + &self.gp + &["typed", "hdrs"]
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
                    if !download_mode {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    } else {
                    io::copy(&mut response, &mut ostream).unwrap();
                    }
                    Ok(())
                }
            }
        }
    }

    fn _query_sql_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut download_mode = false;
        let mut call = self.hub.query().sql_get(opt.value_of("sql").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "typed" => {
                    call = call.typed(arg_from_str(value.unwrap_or("false"), err, "typed", "boolean"));
                },
                "hdrs" => {
                    call = call.hdrs(arg_from_str(value.unwrap_or("false"), err, "hdrs", "boolean"));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            if key == "alt" && value.unwrap_or("unset") == "media" {
                                download_mode = true;
                            }
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                Vec::new() + &self.gp + &["typed", "hdrs"]
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
                    if !download_mode {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    serde::json::to_writer_pretty(&mut ostream, &value).unwrap();
                    } else {
                    io::copy(&mut response, &mut ostream).unwrap();
                    }
                    Ok(())
                }
            }
        }
    }

    fn _style_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let style_id: i32 = arg_from_str(&opt.value_of("style-id").unwrap_or(""), err, "<style-id>", "integer");
        let mut call = self.hub.style().delete(opt.value_of("table-id").unwrap_or(""), style_id);
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
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _style_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let style_id: i32 = arg_from_str(&opt.value_of("style-id").unwrap_or(""), err, "<style-id>", "integer");
        let mut call = self.hub.style().get(opt.value_of("table-id").unwrap_or(""), style_id);
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

    fn _style_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::StyleSetting::default();
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
            
            match &temp_cursor.to_string()[..] {
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
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["column-name", "fill-color", "fill-color-styler", "fill-opacity", "gradient", "icon-name", "icon-styler", "kind", "marker-options", "max", "min", "name", "polygon-options", "polyline-options", "stroke-color", "stroke-color-styler", "stroke-opacity", "stroke-weight", "stroke-weight-styler", "style-id", "table-id"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.style().insert(request, opt.value_of("table-id").unwrap_or(""));
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

    fn _style_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.style().list(opt.value_of("table-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
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
                                                Vec::new() + &self.gp + &["page-token", "max-results"]
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

    fn _style_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::StyleSetting::default();
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
            
            match &temp_cursor.to_string()[..] {
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
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["column-name", "fill-color", "fill-color-styler", "fill-opacity", "gradient", "icon-name", "icon-styler", "kind", "marker-options", "max", "min", "name", "polygon-options", "polyline-options", "stroke-color", "stroke-color-styler", "stroke-opacity", "stroke-weight", "stroke-weight-styler", "style-id", "table-id"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let style_id: i32 = arg_from_str(&opt.value_of("style-id").unwrap_or(""), err, "<style-id>", "integer");
        let mut call = self.hub.style().patch(request, opt.value_of("table-id").unwrap_or(""), style_id);
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

    fn _style_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::StyleSetting::default();
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
            
            match &temp_cursor.to_string()[..] {
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
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["column-name", "fill-color", "fill-color-styler", "fill-opacity", "gradient", "icon-name", "icon-styler", "kind", "marker-options", "max", "min", "name", "polygon-options", "polyline-options", "stroke-color", "stroke-color-styler", "stroke-opacity", "stroke-weight", "stroke-weight-styler", "style-id", "table-id"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let style_id: i32 = arg_from_str(&opt.value_of("style-id").unwrap_or(""), err, "<style-id>", "integer");
        let mut call = self.hub.style().update(request, opt.value_of("table-id").unwrap_or(""), style_id);
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

    fn _table_copy(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.table().copy(opt.value_of("table-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "copy-presentation" => {
                    call = call.copy_presentation(arg_from_str(value.unwrap_or("false"), err, "copy-presentation", "boolean"));
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
                                                Vec::new() + &self.gp + &["copy-presentation"]
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

    fn _table_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.table().delete(opt.value_of("table-id").unwrap_or(""));
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
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _table_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.table().get(opt.value_of("table-id").unwrap_or(""));
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

    fn _table_import_rows(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.table().import_rows(opt.value_of("table-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["encoding", "end-line", "start-line", "delimiter", "is-strict"]
                                                            ));
                    }
                }
            }
        }
        let vals = opt.values_of("mode").unwrap();
        let protocol = calltype_from_str(vals[0], ["simple", "resumable"].iter().map(|&v| v.to_string()).collect(), err);
        let mut input_file = input_file_from_opts(vals[1], err);
        let mime_type = input_mime_from_opts(opt.value_of("mime").unwrap_or("application/octet-stream"), err);
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
                CallType::Upload(UploadProtocol::Simple) => call.upload(input_file.unwrap(), mime_type.unwrap()),
                CallType::Upload(UploadProtocol::Resumable) => call.upload_resumable(input_file.unwrap(), mime_type.unwrap()),
                CallType::Standard => unreachable!()
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

    fn _table_import_table(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.table().import_table(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "encoding" => {
                    call = call.encoding(value.unwrap_or(""));
                },
                "delimiter" => {
                    call = call.delimiter(value.unwrap_or(""));
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
                                                Vec::new() + &self.gp + &["delimiter", "encoding"]
                                                            ));
                    }
                }
            }
        }
        let vals = opt.values_of("mode").unwrap();
        let protocol = calltype_from_str(vals[0], ["simple", "resumable"].iter().map(|&v| v.to_string()).collect(), err);
        let mut input_file = input_file_from_opts(vals[1], err);
        let mime_type = input_mime_from_opts(opt.value_of("mime").unwrap_or("application/octet-stream"), err);
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
                CallType::Upload(UploadProtocol::Simple) => call.upload(input_file.unwrap(), mime_type.unwrap()),
                CallType::Upload(UploadProtocol::Resumable) => call.upload_resumable(input_file.unwrap(), mime_type.unwrap()),
                CallType::Standard => unreachable!()
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

    fn _table_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Table::default();
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
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["attribution", "attribution-link", "base-table-ids", "column-properties-json-schema", "description", "is-exportable", "kind", "name", "sql", "table-id", "table-properties-json", "table-properties-json-schema"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.table().insert(request);
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

    fn _table_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.table().list();
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
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
                                                Vec::new() + &self.gp + &["page-token", "max-results"]
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

    fn _table_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Table::default();
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
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["attribution", "attribution-link", "base-table-ids", "column-properties-json-schema", "description", "is-exportable", "kind", "name", "sql", "table-id", "table-properties-json", "table-properties-json-schema"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.table().patch(request, opt.value_of("table-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "replace-view-definition" => {
                    call = call.replace_view_definition(arg_from_str(value.unwrap_or("false"), err, "replace-view-definition", "boolean"));
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
                                                Vec::new() + &self.gp + &["replace-view-definition"]
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

    fn _table_replace_rows(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.table().replace_rows(opt.value_of("table-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["encoding", "end-line", "start-line", "delimiter", "is-strict"]
                                                            ));
                    }
                }
            }
        }
        let vals = opt.values_of("mode").unwrap();
        let protocol = calltype_from_str(vals[0], ["simple", "resumable"].iter().map(|&v| v.to_string()).collect(), err);
        let mut input_file = input_file_from_opts(vals[1], err);
        let mime_type = input_mime_from_opts(opt.value_of("mime").unwrap_or("application/octet-stream"), err);
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
                CallType::Upload(UploadProtocol::Simple) => call.upload(input_file.unwrap(), mime_type.unwrap()),
                CallType::Upload(UploadProtocol::Resumable) => call.upload_resumable(input_file.unwrap(), mime_type.unwrap()),
                CallType::Standard => unreachable!()
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

    fn _table_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Table::default();
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
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["attribution", "attribution-link", "base-table-ids", "column-properties-json-schema", "description", "is-exportable", "kind", "name", "sql", "table-id", "table-properties-json", "table-properties-json-schema"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.table().update(request, opt.value_of("table-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "replace-view-definition" => {
                    call = call.replace_view_definition(arg_from_str(value.unwrap_or("false"), err, "replace-view-definition", "boolean"));
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
                                                Vec::new() + &self.gp + &["replace-view-definition"]
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

    fn _task_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.task().delete(opt.value_of("table-id").unwrap_or(""), opt.value_of("task-id").unwrap_or(""));
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
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _task_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.task().get(opt.value_of("table-id").unwrap_or(""), opt.value_of("task-id").unwrap_or(""));
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

    fn _task_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.task().list(opt.value_of("table-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
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
                                                Vec::new() + &self.gp + &["page-token", "start-index", "max-results"]
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

    fn _template_delete(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let template_id: i32 = arg_from_str(&opt.value_of("template-id").unwrap_or(""), err, "<template-id>", "integer");
        let mut call = self.hub.template().delete(opt.value_of("table-id").unwrap_or(""), template_id);
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
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok(mut response) => {
                    Ok(())
                }
            }
        }
    }

    fn _template_get(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let template_id: i32 = arg_from_str(&opt.value_of("template-id").unwrap_or(""), err, "<template-id>", "integer");
        let mut call = self.hub.template().get(opt.value_of("table-id").unwrap_or(""), template_id);
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

    fn _template_insert(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Template::default();
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
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["automatic-column-names", "body", "kind", "name", "table-id", "template-id"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let mut call = self.hub.template().insert(request, opt.value_of("table-id").unwrap_or(""));
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

    fn _template_list(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.template().list(opt.value_of("table-id").unwrap_or(""));
        for parg in opt.values_of("v").unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
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
                                                Vec::new() + &self.gp + &["page-token", "max-results"]
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

    fn _template_patch(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Template::default();
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
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["automatic-column-names", "body", "kind", "name", "table-id", "template-id"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let template_id: i32 = arg_from_str(&opt.value_of("template-id").unwrap_or(""), err, "<template-id>", "integer");
        let mut call = self.hub.template().patch(request, opt.value_of("table-id").unwrap_or(""), template_id);
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

    fn _template_update(&self, opt: &ArgMatches<'n, 'a>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut request = api::Template::default();
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
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["automatic-column-names", "body", "kind", "name", "table-id", "template-id"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                }
            }
        }
        let template_id: i32 = arg_from_str(&opt.value_of("template-id").unwrap_or(""), err, "<template-id>", "integer");
        let mut call = self.hub.template().update(request, opt.value_of("table-id").unwrap_or(""), template_id);
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
            ("column", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._column_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._column_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._column_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._column_list(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._column_patch(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._column_update(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("column".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("query", Some(opt)) => {
                match opt.subcommand() {
                    ("sql", Some(opt)) => {
                        call_result = self._query_sql(opt, dry_run, &mut err);
                    },
                    ("sql-get", Some(opt)) => {
                        call_result = self._query_sql_get(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("query".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("style", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._style_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._style_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._style_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._style_list(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._style_patch(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._style_update(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("style".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("table", Some(opt)) => {
                match opt.subcommand() {
                    ("copy", Some(opt)) => {
                        call_result = self._table_copy(opt, dry_run, &mut err);
                    },
                    ("delete", Some(opt)) => {
                        call_result = self._table_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._table_get(opt, dry_run, &mut err);
                    },
                    ("import-rows", Some(opt)) => {
                        call_result = self._table_import_rows(opt, dry_run, &mut err);
                    },
                    ("import-table", Some(opt)) => {
                        call_result = self._table_import_table(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._table_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._table_list(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._table_patch(opt, dry_run, &mut err);
                    },
                    ("replace-rows", Some(opt)) => {
                        call_result = self._table_replace_rows(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._table_update(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("table".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("task", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._task_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._task_get(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._task_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("task".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("template", Some(opt)) => {
                match opt.subcommand() {
                    ("delete", Some(opt)) => {
                        call_result = self._template_delete(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._template_get(opt, dry_run, &mut err);
                    },
                    ("insert", Some(opt)) => {
                        call_result = self._template_insert(opt, dry_run, &mut err);
                    },
                    ("list", Some(opt)) => {
                        call_result = self._template_list(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._template_patch(opt, dry_run, &mut err);
                    },
                    ("update", Some(opt)) => {
                        call_result = self._template_update(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("template".to_string()));
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

            match cmn::application_secret_from_directory(&config_dir, "fusiontables2-secret.json", 
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
                                          program_name: "fusiontables2",
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
            hub: api::Fusiontables::new(client, auth),
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
    let upload_value_names = ["mode", "file"];
    let arg_data = [
        ("column", "methods: 'delete', 'get', 'insert', 'list', 'patch' and 'update'", vec![
            ("delete",  
                    Some(r##"Deletes the specified column."##),
                    "Details at http://byron.github.io/google-apis-rs/google_fusiontables2_cli/column_delete",
                  vec![
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Table from which the column is being deleted."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"column-id"##),
                     None,
                     Some(r##"Name or identifier for the column being deleted."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",  
                    Some(r##"Retrieves a specific column by its ID."##),
                    "Details at http://byron.github.io/google-apis-rs/google_fusiontables2_cli/column_get",
                  vec![
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Table to which the column belongs."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"column-id"##),
                     None,
                     Some(r##"Name or identifier for the column that is being requested."##),
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
            ("insert",  
                    Some(r##"Adds a new column to the table."##),
                    "Details at http://byron.github.io/google-apis-rs/google_fusiontables2_cli/column_insert",
                  vec![
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Table for which a new column is being added."##),
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
            ("list",  
                    Some(r##"Retrieves a list of columns."##),
                    "Details at http://byron.github.io/google-apis-rs/google_fusiontables2_cli/column_list",
                  vec![
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Table whose columns are being listed."##),
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
            ("patch",  
                    Some(r##"Updates the name or type of an existing column. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_fusiontables2_cli/column_patch",
                  vec![
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Table for which the column is being updated."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"column-id"##),
                     None,
                     Some(r##"Name or identifier for the column that is being updated."##),
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
            ("update",  
                    Some(r##"Updates the name or type of an existing column."##),
                    "Details at http://byron.github.io/google-apis-rs/google_fusiontables2_cli/column_update",
                  vec![
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Table for which the column is being updated."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"column-id"##),
                     None,
                     Some(r##"Name or identifier for the column that is being updated."##),
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
        
        ("query", "methods: 'sql' and 'sql-get'", vec![
            ("sql",  
                    Some(r##"Executes a Fusion Tables SQL statement, which can be any of 
        - SELECT
        - INSERT
        - UPDATE
        - DELETE
        - SHOW
        - DESCRIBE
        - CREATE statement."##),
                    "Details at http://byron.github.io/google-apis-rs/google_fusiontables2_cli/query_sql",
                  vec![
                    (Some(r##"sql"##),
                     None,
                     Some(r##"A Fusion Tables SQL statement, which can be any of 
        - SELECT
        - INSERT
        - UPDATE
        - DELETE
        - SHOW
        - DESCRIBE
        - CREATE"##),
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
            ("sql-get",  
                    Some(r##"Executes a SQL statement which can be any of 
        - SELECT
        - SHOW
        - DESCRIBE"##),
                    "Details at http://byron.github.io/google-apis-rs/google_fusiontables2_cli/query_sql-get",
                  vec![
                    (Some(r##"sql"##),
                     None,
                     Some(r##"A SQL statement which can be any of 
        - SELECT
        - SHOW
        - DESCRIBE"##),
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
        
        ("style", "methods: 'delete', 'get', 'insert', 'list', 'patch' and 'update'", vec![
            ("delete",  
                    Some(r##"Deletes a style."##),
                    "Details at http://byron.github.io/google-apis-rs/google_fusiontables2_cli/style_delete",
                  vec![
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Table from which the style is being deleted"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"style-id"##),
                     None,
                     Some(r##"Identifier (within a table) for the style being deleted"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",  
                    Some(r##"Gets a specific style."##),
                    "Details at http://byron.github.io/google-apis-rs/google_fusiontables2_cli/style_get",
                  vec![
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Table to which the requested style belongs"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"style-id"##),
                     None,
                     Some(r##"Identifier (integer) for a specific style in a table"##),
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
            ("insert",  
                    Some(r##"Adds a new style for the table."##),
                    "Details at http://byron.github.io/google-apis-rs/google_fusiontables2_cli/style_insert",
                  vec![
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Table for which a new style is being added"##),
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
            ("list",  
                    Some(r##"Retrieves a list of styles."##),
                    "Details at http://byron.github.io/google-apis-rs/google_fusiontables2_cli/style_list",
                  vec![
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Table whose styles are being listed"##),
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
            ("patch",  
                    Some(r##"Updates an existing style. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_fusiontables2_cli/style_patch",
                  vec![
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Table whose style is being updated."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"style-id"##),
                     None,
                     Some(r##"Identifier (within a table) for the style being updated."##),
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
            ("update",  
                    Some(r##"Updates an existing style."##),
                    "Details at http://byron.github.io/google-apis-rs/google_fusiontables2_cli/style_update",
                  vec![
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Table whose style is being updated."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"style-id"##),
                     None,
                     Some(r##"Identifier (within a table) for the style being updated."##),
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
        
        ("table", "methods: 'copy', 'delete', 'get', 'import-rows', 'import-table', 'insert', 'list', 'patch', 'replace-rows' and 'update'", vec![
            ("copy",  
                    Some(r##"Copies a table."##),
                    "Details at http://byron.github.io/google-apis-rs/google_fusiontables2_cli/table_copy",
                  vec![
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"ID of the table that is being copied."##),
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
            ("delete",  
                    Some(r##"Deletes a table."##),
                    "Details at http://byron.github.io/google-apis-rs/google_fusiontables2_cli/table_delete",
                  vec![
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"ID of the table to be deleted."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",  
                    Some(r##"Retrieves a specific table by its ID."##),
                    "Details at http://byron.github.io/google-apis-rs/google_fusiontables2_cli/table_get",
                  vec![
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Identifier for the table being requested."##),
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
            ("import-rows",  
                    Some(r##"Imports more rows into a table."##),
                    "Details at http://byron.github.io/google-apis-rs/google_fusiontables2_cli/table_import-rows",
                  vec![
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"The table into which new rows are being imported."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"mode"##),
                     Some(r##"u"##),
                     Some(r##"Specify the upload protocol (simple|resumable) and the file to upload"##),
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
            ("import-table",  
                    Some(r##"Imports a new table."##),
                    "Details at http://byron.github.io/google-apis-rs/google_fusiontables2_cli/table_import-table",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name to be assigned to the new table."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"mode"##),
                     Some(r##"u"##),
                     Some(r##"Specify the upload protocol (simple|resumable) and the file to upload"##),
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
            ("insert",  
                    Some(r##"Creates a new table."##),
                    "Details at http://byron.github.io/google-apis-rs/google_fusiontables2_cli/table_insert",
                  vec![
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
            ("list",  
                    Some(r##"Retrieves a list of tables a user owns."##),
                    "Details at http://byron.github.io/google-apis-rs/google_fusiontables2_cli/table_list",
                  vec![
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
            ("patch",  
                    Some(r##"Updates an existing table. Unless explicitly requested, only the name, description, and attribution will be updated. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_fusiontables2_cli/table_patch",
                  vec![
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"ID of the table that is being updated."##),
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
            ("replace-rows",  
                    Some(r##"Replaces rows of an existing table. Current rows remain visible until all replacement rows are ready."##),
                    "Details at http://byron.github.io/google-apis-rs/google_fusiontables2_cli/table_replace-rows",
                  vec![
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Table whose rows will be replaced."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"mode"##),
                     Some(r##"u"##),
                     Some(r##"Specify the upload protocol (simple|resumable) and the file to upload"##),
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
            ("update",  
                    Some(r##"Updates an existing table. Unless explicitly requested, only the name, description, and attribution will be updated."##),
                    "Details at http://byron.github.io/google-apis-rs/google_fusiontables2_cli/table_update",
                  vec![
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"ID of the table that is being updated."##),
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
        
        ("task", "methods: 'delete', 'get' and 'list'", vec![
            ("delete",  
                    Some(r##"Deletes a specific task by its ID, unless that task has already started running."##),
                    "Details at http://byron.github.io/google-apis-rs/google_fusiontables2_cli/task_delete",
                  vec![
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Table from which the task is being deleted."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"task-id"##),
                     None,
                     Some(r##"The identifier of the task to delete."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",  
                    Some(r##"Retrieves a specific task by its ID."##),
                    "Details at http://byron.github.io/google-apis-rs/google_fusiontables2_cli/task_get",
                  vec![
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Table to which the task belongs."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"task-id"##),
                     None,
                     Some(r##"The identifier of the task to get."##),
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
            ("list",  
                    Some(r##"Retrieves a list of tasks."##),
                    "Details at http://byron.github.io/google-apis-rs/google_fusiontables2_cli/task_list",
                  vec![
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Table whose tasks are being listed."##),
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
        
        ("template", "methods: 'delete', 'get', 'insert', 'list', 'patch' and 'update'", vec![
            ("delete",  
                    Some(r##"Deletes a template"##),
                    "Details at http://byron.github.io/google-apis-rs/google_fusiontables2_cli/template_delete",
                  vec![
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Table from which the template is being deleted"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"template-id"##),
                     None,
                     Some(r##"Identifier for the template which is being deleted"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
                  ]),
            ("get",  
                    Some(r##"Retrieves a specific template by its id"##),
                    "Details at http://byron.github.io/google-apis-rs/google_fusiontables2_cli/template_get",
                  vec![
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Table to which the template belongs"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"template-id"##),
                     None,
                     Some(r##"Identifier for the template that is being requested"##),
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
            ("insert",  
                    Some(r##"Creates a new template for the table."##),
                    "Details at http://byron.github.io/google-apis-rs/google_fusiontables2_cli/template_insert",
                  vec![
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Table for which a new template is being created"##),
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
            ("list",  
                    Some(r##"Retrieves a list of templates."##),
                    "Details at http://byron.github.io/google-apis-rs/google_fusiontables2_cli/template_list",
                  vec![
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Identifier for the table whose templates are being requested"##),
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
            ("patch",  
                    Some(r##"Updates an existing template. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_fusiontables2_cli/template_patch",
                  vec![
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Table to which the updated template belongs"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"template-id"##),
                     None,
                     Some(r##"Identifier for the template that is being updated"##),
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
            ("update",  
                    Some(r##"Updates an existing template"##),
                    "Details at http://byron.github.io/google-apis-rs/google_fusiontables2_cli/template_update",
                  vec![
                    (Some(r##"table-id"##),
                     None,
                     Some(r##"Table to which the updated template belongs"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"template-id"##),
                     None,
                     Some(r##"Identifier for the template that is being updated"##),
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
    
    let mut app = App::new("fusiontables2")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("0.2.0+20150326")
           .about("API for working with Fusion Tables data.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_fusiontables2_cli")
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
                       if arg_name_str == "mode" {
                           arg = arg.number_of_values(2);
                           arg = arg.value_names(&upload_value_names);
           
                           scmd = scmd.arg(Arg::with_name("mime")
                                               .short("m")
                                               .requires("mode")
                                               .required(false)
                                               .help("The file's mime time, like 'application/octet-stream'")
                                               .takes_value(true));
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