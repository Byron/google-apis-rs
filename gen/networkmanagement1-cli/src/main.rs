// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::io::Write;

use clap::{App, Arg, SubCommand};

use google_networkmanagement1::{api, yup_oauth2, Error};

use google_apis_common as apis_common;
use google_clis_common as common;

use std::str::FromStr;

use clap::ArgMatches;
use http_body_util::BodyExt;

use common::{
    arg_from_str, calltype_from_str, input_file_from_opts, input_mime_from_opts, parse_kv_arg,
    remove_json_null_values, writer_from_opts, CLIError, CallType, ComplexType, FieldCursor,
    FieldError, InvalidOptionsError, JsonType, JsonTypeInfo, UploadProtocol,
};

enum DoitError {
    IoError(String, std::io::Error),
    ApiError(Error),
}

struct Engine<'n, C> {
    opt: ArgMatches<'n>,
    hub: api::NetworkManagement<C>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}

impl<'n, C> Engine<'n, C>
where
    C: apis_common::Connector,
{
    async fn _organizations_locations_get(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .organizations()
            .locations_get(opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _organizations_locations_list(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .organizations()
            .locations_list(opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                }
                "page-size" => {
                    call = call.page_size(
                        value
                            .map(|v| arg_from_str(v, err, "page-size", "int32"))
                            .unwrap_or(-0),
                    );
                }
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                }
                "extra-location-types" => {
                    call = call.add_extra_location_types(value.unwrap_or(""));
                }
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v.extend(
                                    ["extra-location-types", "filter", "page-size", "page-token"]
                                        .iter()
                                        .map(|v| *v),
                                );
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _organizations_locations_operations_cancel(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec![]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::CancelOperationRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self
            .hub
            .organizations()
            .locations_operations_cancel(request, opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _organizations_locations_operations_delete(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .organizations()
            .locations_operations_delete(opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _organizations_locations_operations_get(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .organizations()
            .locations_operations_get(opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _organizations_locations_operations_list(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .organizations()
            .locations_operations_list(opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "return-partial-success" => {
                    call = call.return_partial_success(
                        value
                            .map(|v| arg_from_str(v, err, "return-partial-success", "boolean"))
                            .unwrap_or(false),
                    );
                }
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                }
                "page-size" => {
                    call = call.page_size(
                        value
                            .map(|v| arg_from_str(v, err, "page-size", "int32"))
                            .unwrap_or(-0),
                    );
                }
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                }
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v.extend(
                                    [
                                        "filter",
                                        "page-size",
                                        "page-token",
                                        "return-partial-success",
                                    ]
                                    .iter()
                                    .map(|v| *v),
                                );
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _organizations_locations_vpc_flow_logs_configs_create(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "aggregation-interval" => Some((
                    "aggregationInterval",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "create-time" => Some((
                    "createTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "cross-project-metadata" => Some((
                    "crossProjectMetadata",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "description" => Some((
                    "description",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "filter-expr" => Some((
                    "filterExpr",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "flow-sampling" => Some((
                    "flowSampling",
                    JsonTypeInfo {
                        jtype: JsonType::Float,
                        ctype: ComplexType::Pod,
                    },
                )),
                "interconnect-attachment" => Some((
                    "interconnectAttachment",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "labels" => Some((
                    "labels",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Map,
                    },
                )),
                "metadata" => Some((
                    "metadata",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "metadata-fields" => Some((
                    "metadataFields",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Vec,
                    },
                )),
                "name" => Some((
                    "name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "network" => Some((
                    "network",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "state" => Some((
                    "state",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "subnet" => Some((
                    "subnet",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "target-resource-state" => Some((
                    "targetResourceState",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "update-time" => Some((
                    "updateTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "vpn-tunnel" => Some((
                    "vpnTunnel",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "aggregation-interval",
                            "create-time",
                            "cross-project-metadata",
                            "description",
                            "filter-expr",
                            "flow-sampling",
                            "interconnect-attachment",
                            "labels",
                            "metadata",
                            "metadata-fields",
                            "name",
                            "network",
                            "state",
                            "subnet",
                            "target-resource-state",
                            "update-time",
                            "vpn-tunnel",
                        ],
                    );
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::VpcFlowLogsConfig = serde_json::value::from_value(object).unwrap();
        let mut call = self
            .hub
            .organizations()
            .locations_vpc_flow_logs_configs_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "vpc-flow-logs-config-id" => {
                    call = call.vpc_flow_logs_config_id(value.unwrap_or(""));
                }
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v.extend(["vpc-flow-logs-config-id"].iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _organizations_locations_vpc_flow_logs_configs_delete(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .organizations()
            .locations_vpc_flow_logs_configs_delete(opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _organizations_locations_vpc_flow_logs_configs_get(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .organizations()
            .locations_vpc_flow_logs_configs_get(opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _organizations_locations_vpc_flow_logs_configs_list(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .organizations()
            .locations_vpc_flow_logs_configs_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                }
                "page-size" => {
                    call = call.page_size(
                        value
                            .map(|v| arg_from_str(v, err, "page-size", "int32"))
                            .unwrap_or(-0),
                    );
                }
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                }
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                }
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v.extend(
                                    ["filter", "order-by", "page-size", "page-token"]
                                        .iter()
                                        .map(|v| *v),
                                );
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _organizations_locations_vpc_flow_logs_configs_patch(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "aggregation-interval" => Some((
                    "aggregationInterval",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "create-time" => Some((
                    "createTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "cross-project-metadata" => Some((
                    "crossProjectMetadata",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "description" => Some((
                    "description",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "filter-expr" => Some((
                    "filterExpr",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "flow-sampling" => Some((
                    "flowSampling",
                    JsonTypeInfo {
                        jtype: JsonType::Float,
                        ctype: ComplexType::Pod,
                    },
                )),
                "interconnect-attachment" => Some((
                    "interconnectAttachment",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "labels" => Some((
                    "labels",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Map,
                    },
                )),
                "metadata" => Some((
                    "metadata",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "metadata-fields" => Some((
                    "metadataFields",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Vec,
                    },
                )),
                "name" => Some((
                    "name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "network" => Some((
                    "network",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "state" => Some((
                    "state",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "subnet" => Some((
                    "subnet",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "target-resource-state" => Some((
                    "targetResourceState",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "update-time" => Some((
                    "updateTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "vpn-tunnel" => Some((
                    "vpnTunnel",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "aggregation-interval",
                            "create-time",
                            "cross-project-metadata",
                            "description",
                            "filter-expr",
                            "flow-sampling",
                            "interconnect-attachment",
                            "labels",
                            "metadata",
                            "metadata-fields",
                            "name",
                            "network",
                            "state",
                            "subnet",
                            "target-resource-state",
                            "update-time",
                            "vpn-tunnel",
                        ],
                    );
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::VpcFlowLogsConfig = serde_json::value::from_value(object).unwrap();
        let mut call = self
            .hub
            .organizations()
            .locations_vpc_flow_logs_configs_patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(
                        value
                            .map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask"))
                            .unwrap_or(apis_common::FieldMask::default()),
                    );
                }
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v.extend(["update-mask"].iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_get(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .projects()
            .locations_get(opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_global_connectivity_tests_create(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "bypass-firewall-checks" => Some((
                    "bypassFirewallChecks",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "create-time" => Some((
                    "createTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "description" => Some((
                    "description",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "destination.app-engine-version.uri" => Some((
                    "destination.appEngineVersion.uri",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "destination.cloud-function.uri" => Some((
                    "destination.cloudFunction.uri",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "destination.cloud-run-revision.service-uri" => Some((
                    "destination.cloudRunRevision.serviceUri",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "destination.cloud-run-revision.uri" => Some((
                    "destination.cloudRunRevision.uri",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "destination.cloud-sql-instance" => Some((
                    "destination.cloudSqlInstance",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "destination.forwarding-rule" => Some((
                    "destination.forwardingRule",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "destination.forwarding-rule-target" => Some((
                    "destination.forwardingRuleTarget",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "destination.fqdn" => Some((
                    "destination.fqdn",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "destination.gke-master-cluster" => Some((
                    "destination.gkeMasterCluster",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "destination.instance" => Some((
                    "destination.instance",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "destination.ip-address" => Some((
                    "destination.ipAddress",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "destination.load-balancer-id" => Some((
                    "destination.loadBalancerId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "destination.load-balancer-type" => Some((
                    "destination.loadBalancerType",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "destination.network" => Some((
                    "destination.network",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "destination.network-type" => Some((
                    "destination.networkType",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "destination.port" => Some((
                    "destination.port",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "destination.project-id" => Some((
                    "destination.projectId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "destination.redis-cluster" => Some((
                    "destination.redisCluster",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "destination.redis-instance" => Some((
                    "destination.redisInstance",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "display-name" => Some((
                    "displayName",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "labels" => Some((
                    "labels",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Map,
                    },
                )),
                "name" => Some((
                    "name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "probing-details.abort-cause" => Some((
                    "probingDetails.abortCause",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "probing-details.destination-egress-location.metropolitan-area" => Some((
                    "probingDetails.destinationEgressLocation.metropolitanArea",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "probing-details.endpoint-info.destination-ip" => Some((
                    "probingDetails.endpointInfo.destinationIp",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "probing-details.endpoint-info.destination-network-uri" => Some((
                    "probingDetails.endpointInfo.destinationNetworkUri",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "probing-details.endpoint-info.destination-port" => Some((
                    "probingDetails.endpointInfo.destinationPort",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "probing-details.endpoint-info.protocol" => Some((
                    "probingDetails.endpointInfo.protocol",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "probing-details.endpoint-info.source-agent-uri" => Some((
                    "probingDetails.endpointInfo.sourceAgentUri",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "probing-details.endpoint-info.source-ip" => Some((
                    "probingDetails.endpointInfo.sourceIp",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "probing-details.endpoint-info.source-network-uri" => Some((
                    "probingDetails.endpointInfo.sourceNetworkUri",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "probing-details.endpoint-info.source-port" => Some((
                    "probingDetails.endpointInfo.sourcePort",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "probing-details.error.code" => Some((
                    "probingDetails.error.code",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "probing-details.error.message" => Some((
                    "probingDetails.error.message",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "probing-details.probed-all-devices" => Some((
                    "probingDetails.probedAllDevices",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "probing-details.result" => Some((
                    "probingDetails.result",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "probing-details.sent-probe-count" => Some((
                    "probingDetails.sentProbeCount",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "probing-details.successful-probe-count" => Some((
                    "probingDetails.successfulProbeCount",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "probing-details.verify-time" => Some((
                    "probingDetails.verifyTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "protocol" => Some((
                    "protocol",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "reachability-details.error.code" => Some((
                    "reachabilityDetails.error.code",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "reachability-details.error.message" => Some((
                    "reachabilityDetails.error.message",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "reachability-details.result" => Some((
                    "reachabilityDetails.result",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "reachability-details.verify-time" => Some((
                    "reachabilityDetails.verifyTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "related-projects" => Some((
                    "relatedProjects",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Vec,
                    },
                )),
                "return-reachability-details.error.code" => Some((
                    "returnReachabilityDetails.error.code",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "return-reachability-details.error.message" => Some((
                    "returnReachabilityDetails.error.message",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "return-reachability-details.result" => Some((
                    "returnReachabilityDetails.result",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "return-reachability-details.verify-time" => Some((
                    "returnReachabilityDetails.verifyTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "round-trip" => Some((
                    "roundTrip",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "source.app-engine-version.uri" => Some((
                    "source.appEngineVersion.uri",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "source.cloud-function.uri" => Some((
                    "source.cloudFunction.uri",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "source.cloud-run-revision.service-uri" => Some((
                    "source.cloudRunRevision.serviceUri",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "source.cloud-run-revision.uri" => Some((
                    "source.cloudRunRevision.uri",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "source.cloud-sql-instance" => Some((
                    "source.cloudSqlInstance",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "source.forwarding-rule" => Some((
                    "source.forwardingRule",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "source.forwarding-rule-target" => Some((
                    "source.forwardingRuleTarget",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "source.fqdn" => Some((
                    "source.fqdn",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "source.gke-master-cluster" => Some((
                    "source.gkeMasterCluster",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "source.instance" => Some((
                    "source.instance",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "source.ip-address" => Some((
                    "source.ipAddress",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "source.load-balancer-id" => Some((
                    "source.loadBalancerId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "source.load-balancer-type" => Some((
                    "source.loadBalancerType",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "source.network" => Some((
                    "source.network",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "source.network-type" => Some((
                    "source.networkType",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "source.port" => Some((
                    "source.port",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "source.project-id" => Some((
                    "source.projectId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "source.redis-cluster" => Some((
                    "source.redisCluster",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "source.redis-instance" => Some((
                    "source.redisInstance",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "update-time" => Some((
                    "updateTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "abort-cause",
                            "app-engine-version",
                            "bypass-firewall-checks",
                            "cloud-function",
                            "cloud-run-revision",
                            "cloud-sql-instance",
                            "code",
                            "create-time",
                            "description",
                            "destination",
                            "destination-egress-location",
                            "destination-ip",
                            "destination-network-uri",
                            "destination-port",
                            "display-name",
                            "endpoint-info",
                            "error",
                            "forwarding-rule",
                            "forwarding-rule-target",
                            "fqdn",
                            "gke-master-cluster",
                            "instance",
                            "ip-address",
                            "labels",
                            "load-balancer-id",
                            "load-balancer-type",
                            "message",
                            "metropolitan-area",
                            "name",
                            "network",
                            "network-type",
                            "port",
                            "probed-all-devices",
                            "probing-details",
                            "project-id",
                            "protocol",
                            "reachability-details",
                            "redis-cluster",
                            "redis-instance",
                            "related-projects",
                            "result",
                            "return-reachability-details",
                            "round-trip",
                            "sent-probe-count",
                            "service-uri",
                            "source",
                            "source-agent-uri",
                            "source-ip",
                            "source-network-uri",
                            "source-port",
                            "successful-probe-count",
                            "update-time",
                            "uri",
                            "verify-time",
                        ],
                    );
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::ConnectivityTest = serde_json::value::from_value(object).unwrap();
        let mut call = self
            .hub
            .projects()
            .locations_global_connectivity_tests_create(
                request,
                opt.value_of("parent").unwrap_or(""),
            );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "test-id" => {
                    call = call.test_id(value.unwrap_or(""));
                }
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v.extend(["test-id"].iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_global_connectivity_tests_delete(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .projects()
            .locations_global_connectivity_tests_delete(opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_global_connectivity_tests_get(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .projects()
            .locations_global_connectivity_tests_get(opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_global_connectivity_tests_get_iam_policy(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .projects()
            .locations_global_connectivity_tests_get_iam_policy(
                opt.value_of("resource").unwrap_or(""),
            );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "options-requested-policy-version" => {
                    call = call.options_requested_policy_version(
                        value
                            .map(|v| {
                                arg_from_str(v, err, "options-requested-policy-version", "int32")
                            })
                            .unwrap_or(-0),
                    );
                }
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v.extend(["options-requested-policy-version"].iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_global_connectivity_tests_list(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .projects()
            .locations_global_connectivity_tests_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                }
                "page-size" => {
                    call = call.page_size(
                        value
                            .map(|v| arg_from_str(v, err, "page-size", "int32"))
                            .unwrap_or(-0),
                    );
                }
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                }
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                }
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v.extend(
                                    ["filter", "order-by", "page-size", "page-token"]
                                        .iter()
                                        .map(|v| *v),
                                );
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_global_connectivity_tests_patch(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "bypass-firewall-checks" => Some((
                    "bypassFirewallChecks",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "create-time" => Some((
                    "createTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "description" => Some((
                    "description",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "destination.app-engine-version.uri" => Some((
                    "destination.appEngineVersion.uri",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "destination.cloud-function.uri" => Some((
                    "destination.cloudFunction.uri",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "destination.cloud-run-revision.service-uri" => Some((
                    "destination.cloudRunRevision.serviceUri",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "destination.cloud-run-revision.uri" => Some((
                    "destination.cloudRunRevision.uri",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "destination.cloud-sql-instance" => Some((
                    "destination.cloudSqlInstance",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "destination.forwarding-rule" => Some((
                    "destination.forwardingRule",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "destination.forwarding-rule-target" => Some((
                    "destination.forwardingRuleTarget",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "destination.fqdn" => Some((
                    "destination.fqdn",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "destination.gke-master-cluster" => Some((
                    "destination.gkeMasterCluster",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "destination.instance" => Some((
                    "destination.instance",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "destination.ip-address" => Some((
                    "destination.ipAddress",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "destination.load-balancer-id" => Some((
                    "destination.loadBalancerId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "destination.load-balancer-type" => Some((
                    "destination.loadBalancerType",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "destination.network" => Some((
                    "destination.network",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "destination.network-type" => Some((
                    "destination.networkType",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "destination.port" => Some((
                    "destination.port",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "destination.project-id" => Some((
                    "destination.projectId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "destination.redis-cluster" => Some((
                    "destination.redisCluster",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "destination.redis-instance" => Some((
                    "destination.redisInstance",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "display-name" => Some((
                    "displayName",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "labels" => Some((
                    "labels",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Map,
                    },
                )),
                "name" => Some((
                    "name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "probing-details.abort-cause" => Some((
                    "probingDetails.abortCause",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "probing-details.destination-egress-location.metropolitan-area" => Some((
                    "probingDetails.destinationEgressLocation.metropolitanArea",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "probing-details.endpoint-info.destination-ip" => Some((
                    "probingDetails.endpointInfo.destinationIp",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "probing-details.endpoint-info.destination-network-uri" => Some((
                    "probingDetails.endpointInfo.destinationNetworkUri",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "probing-details.endpoint-info.destination-port" => Some((
                    "probingDetails.endpointInfo.destinationPort",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "probing-details.endpoint-info.protocol" => Some((
                    "probingDetails.endpointInfo.protocol",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "probing-details.endpoint-info.source-agent-uri" => Some((
                    "probingDetails.endpointInfo.sourceAgentUri",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "probing-details.endpoint-info.source-ip" => Some((
                    "probingDetails.endpointInfo.sourceIp",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "probing-details.endpoint-info.source-network-uri" => Some((
                    "probingDetails.endpointInfo.sourceNetworkUri",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "probing-details.endpoint-info.source-port" => Some((
                    "probingDetails.endpointInfo.sourcePort",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "probing-details.error.code" => Some((
                    "probingDetails.error.code",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "probing-details.error.message" => Some((
                    "probingDetails.error.message",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "probing-details.probed-all-devices" => Some((
                    "probingDetails.probedAllDevices",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "probing-details.result" => Some((
                    "probingDetails.result",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "probing-details.sent-probe-count" => Some((
                    "probingDetails.sentProbeCount",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "probing-details.successful-probe-count" => Some((
                    "probingDetails.successfulProbeCount",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "probing-details.verify-time" => Some((
                    "probingDetails.verifyTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "protocol" => Some((
                    "protocol",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "reachability-details.error.code" => Some((
                    "reachabilityDetails.error.code",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "reachability-details.error.message" => Some((
                    "reachabilityDetails.error.message",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "reachability-details.result" => Some((
                    "reachabilityDetails.result",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "reachability-details.verify-time" => Some((
                    "reachabilityDetails.verifyTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "related-projects" => Some((
                    "relatedProjects",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Vec,
                    },
                )),
                "return-reachability-details.error.code" => Some((
                    "returnReachabilityDetails.error.code",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "return-reachability-details.error.message" => Some((
                    "returnReachabilityDetails.error.message",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "return-reachability-details.result" => Some((
                    "returnReachabilityDetails.result",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "return-reachability-details.verify-time" => Some((
                    "returnReachabilityDetails.verifyTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "round-trip" => Some((
                    "roundTrip",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "source.app-engine-version.uri" => Some((
                    "source.appEngineVersion.uri",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "source.cloud-function.uri" => Some((
                    "source.cloudFunction.uri",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "source.cloud-run-revision.service-uri" => Some((
                    "source.cloudRunRevision.serviceUri",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "source.cloud-run-revision.uri" => Some((
                    "source.cloudRunRevision.uri",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "source.cloud-sql-instance" => Some((
                    "source.cloudSqlInstance",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "source.forwarding-rule" => Some((
                    "source.forwardingRule",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "source.forwarding-rule-target" => Some((
                    "source.forwardingRuleTarget",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "source.fqdn" => Some((
                    "source.fqdn",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "source.gke-master-cluster" => Some((
                    "source.gkeMasterCluster",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "source.instance" => Some((
                    "source.instance",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "source.ip-address" => Some((
                    "source.ipAddress",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "source.load-balancer-id" => Some((
                    "source.loadBalancerId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "source.load-balancer-type" => Some((
                    "source.loadBalancerType",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "source.network" => Some((
                    "source.network",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "source.network-type" => Some((
                    "source.networkType",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "source.port" => Some((
                    "source.port",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "source.project-id" => Some((
                    "source.projectId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "source.redis-cluster" => Some((
                    "source.redisCluster",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "source.redis-instance" => Some((
                    "source.redisInstance",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "update-time" => Some((
                    "updateTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "abort-cause",
                            "app-engine-version",
                            "bypass-firewall-checks",
                            "cloud-function",
                            "cloud-run-revision",
                            "cloud-sql-instance",
                            "code",
                            "create-time",
                            "description",
                            "destination",
                            "destination-egress-location",
                            "destination-ip",
                            "destination-network-uri",
                            "destination-port",
                            "display-name",
                            "endpoint-info",
                            "error",
                            "forwarding-rule",
                            "forwarding-rule-target",
                            "fqdn",
                            "gke-master-cluster",
                            "instance",
                            "ip-address",
                            "labels",
                            "load-balancer-id",
                            "load-balancer-type",
                            "message",
                            "metropolitan-area",
                            "name",
                            "network",
                            "network-type",
                            "port",
                            "probed-all-devices",
                            "probing-details",
                            "project-id",
                            "protocol",
                            "reachability-details",
                            "redis-cluster",
                            "redis-instance",
                            "related-projects",
                            "result",
                            "return-reachability-details",
                            "round-trip",
                            "sent-probe-count",
                            "service-uri",
                            "source",
                            "source-agent-uri",
                            "source-ip",
                            "source-network-uri",
                            "source-port",
                            "successful-probe-count",
                            "update-time",
                            "uri",
                            "verify-time",
                        ],
                    );
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::ConnectivityTest = serde_json::value::from_value(object).unwrap();
        let mut call = self
            .hub
            .projects()
            .locations_global_connectivity_tests_patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(
                        value
                            .map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask"))
                            .unwrap_or(apis_common::FieldMask::default()),
                    );
                }
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v.extend(["update-mask"].iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_global_connectivity_tests_rerun(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec![]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::RerunConnectivityTestRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self
            .hub
            .projects()
            .locations_global_connectivity_tests_rerun(request, opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_global_connectivity_tests_set_iam_policy(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "policy.etag" => Some((
                    "policy.etag",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "policy.version" => Some((
                    "policy.version",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "update-mask" => Some((
                    "updateMask",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec!["etag", "policy", "update-mask", "version"],
                    );
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::SetIamPolicyRequest = serde_json::value::from_value(object).unwrap();
        let mut call = self
            .hub
            .projects()
            .locations_global_connectivity_tests_set_iam_policy(
                request,
                opt.value_of("resource").unwrap_or(""),
            );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_global_connectivity_tests_test_iam_permissions(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "permissions" => Some((
                    "permissions",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Vec,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["permissions"]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::TestIamPermissionsRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self
            .hub
            .projects()
            .locations_global_connectivity_tests_test_iam_permissions(
                request,
                opt.value_of("resource").unwrap_or(""),
            );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_global_operations_cancel(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec![]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::CancelOperationRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self
            .hub
            .projects()
            .locations_global_operations_cancel(request, opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_global_operations_delete(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .projects()
            .locations_global_operations_delete(opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_global_operations_get(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .projects()
            .locations_global_operations_get(opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_global_operations_list(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .projects()
            .locations_global_operations_list(opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "return-partial-success" => {
                    call = call.return_partial_success(
                        value
                            .map(|v| arg_from_str(v, err, "return-partial-success", "boolean"))
                            .unwrap_or(false),
                    );
                }
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                }
                "page-size" => {
                    call = call.page_size(
                        value
                            .map(|v| arg_from_str(v, err, "page-size", "int32"))
                            .unwrap_or(-0),
                    );
                }
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                }
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v.extend(
                                    [
                                        "filter",
                                        "page-size",
                                        "page-token",
                                        "return-partial-success",
                                    ]
                                    .iter()
                                    .map(|v| *v),
                                );
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_list(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .projects()
            .locations_list(opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                }
                "page-size" => {
                    call = call.page_size(
                        value
                            .map(|v| arg_from_str(v, err, "page-size", "int32"))
                            .unwrap_or(-0),
                    );
                }
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                }
                "extra-location-types" => {
                    call = call.add_extra_location_types(value.unwrap_or(""));
                }
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v.extend(
                                    ["extra-location-types", "filter", "page-size", "page-token"]
                                        .iter()
                                        .map(|v| *v),
                                );
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_vpc_flow_logs_configs_create(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "aggregation-interval" => Some((
                    "aggregationInterval",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "create-time" => Some((
                    "createTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "cross-project-metadata" => Some((
                    "crossProjectMetadata",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "description" => Some((
                    "description",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "filter-expr" => Some((
                    "filterExpr",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "flow-sampling" => Some((
                    "flowSampling",
                    JsonTypeInfo {
                        jtype: JsonType::Float,
                        ctype: ComplexType::Pod,
                    },
                )),
                "interconnect-attachment" => Some((
                    "interconnectAttachment",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "labels" => Some((
                    "labels",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Map,
                    },
                )),
                "metadata" => Some((
                    "metadata",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "metadata-fields" => Some((
                    "metadataFields",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Vec,
                    },
                )),
                "name" => Some((
                    "name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "network" => Some((
                    "network",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "state" => Some((
                    "state",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "subnet" => Some((
                    "subnet",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "target-resource-state" => Some((
                    "targetResourceState",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "update-time" => Some((
                    "updateTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "vpn-tunnel" => Some((
                    "vpnTunnel",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "aggregation-interval",
                            "create-time",
                            "cross-project-metadata",
                            "description",
                            "filter-expr",
                            "flow-sampling",
                            "interconnect-attachment",
                            "labels",
                            "metadata",
                            "metadata-fields",
                            "name",
                            "network",
                            "state",
                            "subnet",
                            "target-resource-state",
                            "update-time",
                            "vpn-tunnel",
                        ],
                    );
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::VpcFlowLogsConfig = serde_json::value::from_value(object).unwrap();
        let mut call = self
            .hub
            .projects()
            .locations_vpc_flow_logs_configs_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "vpc-flow-logs-config-id" => {
                    call = call.vpc_flow_logs_config_id(value.unwrap_or(""));
                }
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v.extend(["vpc-flow-logs-config-id"].iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_vpc_flow_logs_configs_delete(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .projects()
            .locations_vpc_flow_logs_configs_delete(opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_vpc_flow_logs_configs_get(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .projects()
            .locations_vpc_flow_logs_configs_get(opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_vpc_flow_logs_configs_list(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .projects()
            .locations_vpc_flow_logs_configs_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                }
                "page-size" => {
                    call = call.page_size(
                        value
                            .map(|v| arg_from_str(v, err, "page-size", "int32"))
                            .unwrap_or(-0),
                    );
                }
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                }
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                }
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v.extend(
                                    ["filter", "order-by", "page-size", "page-token"]
                                        .iter()
                                        .map(|v| *v),
                                );
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_vpc_flow_logs_configs_patch(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "aggregation-interval" => Some((
                    "aggregationInterval",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "create-time" => Some((
                    "createTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "cross-project-metadata" => Some((
                    "crossProjectMetadata",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "description" => Some((
                    "description",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "filter-expr" => Some((
                    "filterExpr",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "flow-sampling" => Some((
                    "flowSampling",
                    JsonTypeInfo {
                        jtype: JsonType::Float,
                        ctype: ComplexType::Pod,
                    },
                )),
                "interconnect-attachment" => Some((
                    "interconnectAttachment",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "labels" => Some((
                    "labels",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Map,
                    },
                )),
                "metadata" => Some((
                    "metadata",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "metadata-fields" => Some((
                    "metadataFields",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Vec,
                    },
                )),
                "name" => Some((
                    "name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "network" => Some((
                    "network",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "state" => Some((
                    "state",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "subnet" => Some((
                    "subnet",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "target-resource-state" => Some((
                    "targetResourceState",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "update-time" => Some((
                    "updateTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "vpn-tunnel" => Some((
                    "vpnTunnel",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "aggregation-interval",
                            "create-time",
                            "cross-project-metadata",
                            "description",
                            "filter-expr",
                            "flow-sampling",
                            "interconnect-attachment",
                            "labels",
                            "metadata",
                            "metadata-fields",
                            "name",
                            "network",
                            "state",
                            "subnet",
                            "target-resource-state",
                            "update-time",
                            "vpn-tunnel",
                        ],
                    );
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::VpcFlowLogsConfig = serde_json::value::from_value(object).unwrap();
        let mut call = self
            .hub
            .projects()
            .locations_vpc_flow_logs_configs_patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(
                        value
                            .map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask"))
                            .unwrap_or(apis_common::FieldMask::default()),
                    );
                }
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v.extend(["update-mask"].iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_vpc_flow_logs_configs_query_org_vpc_flow_logs_configs(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .projects()
            .locations_vpc_flow_logs_configs_query_org_vpc_flow_logs_configs(
                opt.value_of("parent").unwrap_or(""),
            );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                }
                "page-size" => {
                    call = call.page_size(
                        value
                            .map(|v| arg_from_str(v, err, "page-size", "int32"))
                            .unwrap_or(-0),
                    );
                }
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                }
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v.extend(["filter", "page-size", "page-token"].iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_vpc_flow_logs_configs_show_effective_flow_logs_configs(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .projects()
            .locations_vpc_flow_logs_configs_show_effective_flow_logs_configs(
                opt.value_of("parent").unwrap_or(""),
            );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "resource" => {
                    call = call.resource(value.unwrap_or(""));
                }
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                }
                "page-size" => {
                    call = call.page_size(
                        value
                            .map(|v| arg_from_str(v, err, "page-size", "int32"))
                            .unwrap_or(-0),
                    );
                }
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                }
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v.extend(
                                    ["filter", "page-size", "page-token", "resource"]
                                        .iter()
                                        .map(|v| *v),
                                );
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _doit(
        &self,
        dry_run: bool,
    ) -> Result<Result<(), DoitError>, Option<InvalidOptionsError>> {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Result<(), DoitError> = Ok(());
        let mut err_opt: Option<InvalidOptionsError> = None;
        match self.opt.subcommand() {
            ("organizations", Some(opt)) => match opt.subcommand() {
                ("locations-get", Some(opt)) => {
                    call_result = self
                        ._organizations_locations_get(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-list", Some(opt)) => {
                    call_result = self
                        ._organizations_locations_list(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-operations-cancel", Some(opt)) => {
                    call_result = self
                        ._organizations_locations_operations_cancel(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-operations-delete", Some(opt)) => {
                    call_result = self
                        ._organizations_locations_operations_delete(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-operations-get", Some(opt)) => {
                    call_result = self
                        ._organizations_locations_operations_get(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-operations-list", Some(opt)) => {
                    call_result = self
                        ._organizations_locations_operations_list(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-vpc-flow-logs-configs-create", Some(opt)) => {
                    call_result = self
                        ._organizations_locations_vpc_flow_logs_configs_create(
                            opt, dry_run, &mut err,
                        )
                        .await;
                }
                ("locations-vpc-flow-logs-configs-delete", Some(opt)) => {
                    call_result = self
                        ._organizations_locations_vpc_flow_logs_configs_delete(
                            opt, dry_run, &mut err,
                        )
                        .await;
                }
                ("locations-vpc-flow-logs-configs-get", Some(opt)) => {
                    call_result = self
                        ._organizations_locations_vpc_flow_logs_configs_get(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-vpc-flow-logs-configs-list", Some(opt)) => {
                    call_result = self
                        ._organizations_locations_vpc_flow_logs_configs_list(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-vpc-flow-logs-configs-patch", Some(opt)) => {
                    call_result = self
                        ._organizations_locations_vpc_flow_logs_configs_patch(
                            opt, dry_run, &mut err,
                        )
                        .await;
                }
                _ => {
                    err.issues
                        .push(CLIError::MissingMethodError("organizations".to_string()));
                    writeln!(std::io::stderr(), "{}\n", opt.usage()).ok();
                }
            },
            ("projects", Some(opt)) => match opt.subcommand() {
                ("locations-get", Some(opt)) => {
                    call_result = self._projects_locations_get(opt, dry_run, &mut err).await;
                }
                ("locations-global-connectivity-tests-create", Some(opt)) => {
                    call_result = self
                        ._projects_locations_global_connectivity_tests_create(
                            opt, dry_run, &mut err,
                        )
                        .await;
                }
                ("locations-global-connectivity-tests-delete", Some(opt)) => {
                    call_result = self
                        ._projects_locations_global_connectivity_tests_delete(
                            opt, dry_run, &mut err,
                        )
                        .await;
                }
                ("locations-global-connectivity-tests-get", Some(opt)) => {
                    call_result = self
                        ._projects_locations_global_connectivity_tests_get(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-global-connectivity-tests-get-iam-policy", Some(opt)) => {
                    call_result = self
                        ._projects_locations_global_connectivity_tests_get_iam_policy(
                            opt, dry_run, &mut err,
                        )
                        .await;
                }
                ("locations-global-connectivity-tests-list", Some(opt)) => {
                    call_result = self
                        ._projects_locations_global_connectivity_tests_list(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-global-connectivity-tests-patch", Some(opt)) => {
                    call_result = self
                        ._projects_locations_global_connectivity_tests_patch(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-global-connectivity-tests-rerun", Some(opt)) => {
                    call_result = self
                        ._projects_locations_global_connectivity_tests_rerun(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-global-connectivity-tests-set-iam-policy", Some(opt)) => {
                    call_result = self
                        ._projects_locations_global_connectivity_tests_set_iam_policy(
                            opt, dry_run, &mut err,
                        )
                        .await;
                }
                ("locations-global-connectivity-tests-test-iam-permissions", Some(opt)) => {
                    call_result = self
                        ._projects_locations_global_connectivity_tests_test_iam_permissions(
                            opt, dry_run, &mut err,
                        )
                        .await;
                }
                ("locations-global-operations-cancel", Some(opt)) => {
                    call_result = self
                        ._projects_locations_global_operations_cancel(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-global-operations-delete", Some(opt)) => {
                    call_result = self
                        ._projects_locations_global_operations_delete(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-global-operations-get", Some(opt)) => {
                    call_result = self
                        ._projects_locations_global_operations_get(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-global-operations-list", Some(opt)) => {
                    call_result = self
                        ._projects_locations_global_operations_list(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-list", Some(opt)) => {
                    call_result = self._projects_locations_list(opt, dry_run, &mut err).await;
                }
                ("locations-vpc-flow-logs-configs-create", Some(opt)) => {
                    call_result = self
                        ._projects_locations_vpc_flow_logs_configs_create(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-vpc-flow-logs-configs-delete", Some(opt)) => {
                    call_result = self
                        ._projects_locations_vpc_flow_logs_configs_delete(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-vpc-flow-logs-configs-get", Some(opt)) => {
                    call_result = self
                        ._projects_locations_vpc_flow_logs_configs_get(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-vpc-flow-logs-configs-list", Some(opt)) => {
                    call_result = self
                        ._projects_locations_vpc_flow_logs_configs_list(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-vpc-flow-logs-configs-patch", Some(opt)) => {
                    call_result = self
                        ._projects_locations_vpc_flow_logs_configs_patch(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-vpc-flow-logs-configs-query-org-vpc-flow-logs-configs", Some(opt)) => {
                    call_result = self
                        ._projects_locations_vpc_flow_logs_configs_query_org_vpc_flow_logs_configs(
                            opt, dry_run, &mut err,
                        )
                        .await;
                }
                ("locations-vpc-flow-logs-configs-show-effective-flow-logs-configs", Some(opt)) => {
                    call_result = self
                        ._projects_locations_vpc_flow_logs_configs_show_effective_flow_logs_configs(
                            opt, dry_run, &mut err,
                        )
                        .await;
                }
                _ => {
                    err.issues
                        .push(CLIError::MissingMethodError("projects".to_string()));
                    writeln!(std::io::stderr(), "{}\n", opt.usage()).ok();
                }
            },
            _ => {
                err.issues.push(CLIError::MissingCommandError);
                writeln!(std::io::stderr(), "{}\n", self.opt.usage()).ok();
            }
        }

        if dry_run {
            if !err.issues.is_empty() {
                err_opt = Some(err);
            }
            Err(err_opt)
        } else {
            Ok(call_result)
        }
    }

    // Please note that this call will fail if any part of the opt can't be handled
    async fn new(opt: ArgMatches<'n>, connector: C) -> Result<Engine<'n, C>, InvalidOptionsError> {
        let (config_dir, secret) = {
            let config_dir = match common::assure_config_dir_exists(
                opt.value_of("folder").unwrap_or("~/.google-service-cli"),
            ) {
                Err(e) => return Err(InvalidOptionsError::single(e, 3)),
                Ok(p) => p,
            };

            match common::application_secret_from_directory(&config_dir, "networkmanagement1-secret.json",
                                                         "{\"installed\":{\"auth_uri\":\"https://accounts.google.com/o/oauth2/auth\",\"client_secret\":\"hCsslbCUyfehWMmbkG8vTYxG\",\"token_uri\":\"https://accounts.google.com/o/oauth2/token\",\"client_email\":\"\",\"redirect_uris\":[\"urn:ietf:wg:oauth:2.0:oob\",\"oob\"],\"client_x509_cert_url\":\"\",\"client_id\":\"620010449518-9ngf7o4dhs0dka470npqvor6dc5lqb9b.apps.googleusercontent.com\",\"auth_provider_x509_cert_url\":\"https://www.googleapis.com/oauth2/v1/certs\"}}") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let executor = hyper_util::rt::TokioExecutor::new();
        let client =
            hyper_util::client::legacy::Client::builder(executor.clone()).build(connector.clone());

        let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
            secret,
            yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
            yup_oauth2::client::CustomHyperClientBuilder::from(
                hyper_util::client::legacy::Client::builder(executor).build(connector),
            ),
        )
        .persist_tokens_to_disk(format!("{}/networkmanagement1", config_dir))
        .build()
        .await
        .unwrap();

        let engine = Engine {
            opt,
            hub: api::NetworkManagement::new(client, auth),
            gp: vec![
                "$-xgafv",
                "access-token",
                "alt",
                "callback",
                "fields",
                "key",
                "oauth-token",
                "pretty-print",
                "quota-user",
                "upload-type",
                "upload-protocol",
            ],
            gpm: vec![
                ("$-xgafv", "$.xgafv"),
                ("access-token", "access_token"),
                ("oauth-token", "oauth_token"),
                ("pretty-print", "prettyPrint"),
                ("quota-user", "quotaUser"),
                ("upload-type", "uploadType"),
                ("upload-protocol", "upload_protocol"),
            ],
        };

        match engine._doit(true).await {
            Err(Some(err)) => Err(err),
            Err(None) => Ok(engine),
            Ok(_) => unreachable!(),
        }
    }

    async fn doit(&self) -> Result<(), DoitError> {
        match self._doit(false).await {
            Ok(res) => res,
            Err(_) => unreachable!(),
        }
    }
}

#[tokio::main]
async fn main() {
    let mut exit_status = 0i32;
    let arg_data = [
        ("organizations", "methods: 'locations-get', 'locations-list', 'locations-operations-cancel', 'locations-operations-delete', 'locations-operations-get', 'locations-operations-list', 'locations-vpc-flow-logs-configs-create', 'locations-vpc-flow-logs-configs-delete', 'locations-vpc-flow-logs-configs-get', 'locations-vpc-flow-logs-configs-list' and 'locations-vpc-flow-logs-configs-patch'", vec![
            ("locations-get",
                    Some(r##"Gets information about a location."##),
                    "Details at http://byron.github.io/google-apis-rs/google_networkmanagement1_cli/organizations_locations-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Resource name for the location."##),
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
            ("locations-list",
                    Some(r##"Lists information about the supported locations for this service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_networkmanagement1_cli/organizations_locations-list",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The resource that owns the locations collection, if applicable."##),
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
            ("locations-operations-cancel",
                    Some(r##"Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of `1`, corresponding to `Code.CANCELLED`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_networkmanagement1_cli/organizations_locations-operations-cancel",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the operation resource to be cancelled."##),
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
            ("locations-operations-delete",
                    Some(r##"Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_networkmanagement1_cli/organizations_locations-operations-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the operation resource to be deleted."##),
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
            ("locations-operations-get",
                    Some(r##"Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_networkmanagement1_cli/organizations_locations-operations-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the operation resource."##),
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
            ("locations-operations-list",
                    Some(r##"Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_networkmanagement1_cli/organizations_locations-operations-list",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the operation's parent resource."##),
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
            ("locations-vpc-flow-logs-configs-create",
                    Some(r##"Creates a new `VpcFlowLogsConfig`. If a configuration with the exact same settings already exists (even if the ID is different), the creation fails. Notes: 1. Creating a configuration with `state=DISABLED` will fail 2. The following fields are not considered as settings for the purpose of the check mentioned above, therefore - creating another configuration with the same fields but different values for the following fields will fail as well: * name * create_time * update_time * labels * description"##),
                    "Details at http://byron.github.io/google-apis-rs/google_networkmanagement1_cli/organizations_locations-vpc-flow-logs-configs-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource of the VpcFlowLogsConfig to create, in one of the following formats: - For project-level resources: `projects/{project_id}/locations/global` - For organization-level resources: `organizations/{organization_id}/locations/global`"##),
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
            ("locations-vpc-flow-logs-configs-delete",
                    Some(r##"Deletes a specific `VpcFlowLogsConfig`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_networkmanagement1_cli/organizations_locations-vpc-flow-logs-configs-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name of the VpcFlowLogsConfig, in one of the following formats: - For a project-level resource: `projects/{project_id}/locations/global/vpcFlowLogsConfigs/{vpc_flow_logs_config_id}` - For an organization-level resource: `organizations/{organization_id}/locations/global/vpcFlowLogsConfigs/{vpc_flow_logs_config_id}`"##),
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
            ("locations-vpc-flow-logs-configs-get",
                    Some(r##"Gets the details of a specific `VpcFlowLogsConfig`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_networkmanagement1_cli/organizations_locations-vpc-flow-logs-configs-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name of the VpcFlowLogsConfig, in one of the following formats: - For project-level resources: `projects/{project_id}/locations/global/vpcFlowLogsConfigs/{vpc_flow_logs_config_id}` - For organization-level resources: `organizations/{organization_id}/locations/global/vpcFlowLogsConfigs/{vpc_flow_logs_config_id}`"##),
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
            ("locations-vpc-flow-logs-configs-list",
                    Some(r##"Lists all `VpcFlowLogsConfigs` in a given organization."##),
                    "Details at http://byron.github.io/google-apis-rs/google_networkmanagement1_cli/organizations_locations-vpc-flow-logs-configs-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource of the VpcFlowLogsConfig, in one of the following formats: - For project-level resources: `projects/{project_id}/locations/global` - For organization-level resources: `organizations/{organization_id}/locations/global`"##),
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
            ("locations-vpc-flow-logs-configs-patch",
                    Some(r##"Updates an existing `VpcFlowLogsConfig`. If a configuration with the exact same settings already exists (even if the ID is different), the creation fails. Notes: 1. Updating a configuration with `state=DISABLED` will fail 2. The following fields are not considered as settings for the purpose of the check mentioned above, therefore - updating another configuration with the same fields but different values for the following fields will fail as well: * name * create_time * update_time * labels * description"##),
                    "Details at http://byron.github.io/google-apis-rs/google_networkmanagement1_cli/organizations_locations-vpc-flow-logs-configs-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Identifier. Unique name of the configuration. The name can have one of the following forms: - For project-level configurations: `projects/{project_id}/locations/global/vpcFlowLogsConfigs/{vpc_flow_logs_config_id}` - For organization-level configurations: `organizations/{organization_id}/locations/global/vpcFlowLogsConfigs/{vpc_flow_logs_config_id}`"##),
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
            ("projects", "methods: 'locations-get', 'locations-global-connectivity-tests-create', 'locations-global-connectivity-tests-delete', 'locations-global-connectivity-tests-get', 'locations-global-connectivity-tests-get-iam-policy', 'locations-global-connectivity-tests-list', 'locations-global-connectivity-tests-patch', 'locations-global-connectivity-tests-rerun', 'locations-global-connectivity-tests-set-iam-policy', 'locations-global-connectivity-tests-test-iam-permissions', 'locations-global-operations-cancel', 'locations-global-operations-delete', 'locations-global-operations-get', 'locations-global-operations-list', 'locations-list', 'locations-vpc-flow-logs-configs-create', 'locations-vpc-flow-logs-configs-delete', 'locations-vpc-flow-logs-configs-get', 'locations-vpc-flow-logs-configs-list', 'locations-vpc-flow-logs-configs-patch', 'locations-vpc-flow-logs-configs-query-org-vpc-flow-logs-configs' and 'locations-vpc-flow-logs-configs-show-effective-flow-logs-configs'", vec![
            ("locations-get",
                    Some(r##"Gets information about a location."##),
                    "Details at http://byron.github.io/google-apis-rs/google_networkmanagement1_cli/projects_locations-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Resource name for the location."##),
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
            ("locations-global-connectivity-tests-create",
                    Some(r##"Creates a new Connectivity Test. After you create a test, the reachability analysis is performed as part of the long running operation, which completes when the analysis completes. If the endpoint specifications in `ConnectivityTest` are invalid (for example, containing non-existent resources in the network, or you don't have read permissions to the network configurations of listed projects), then the reachability result returns a value of `UNKNOWN`. If the endpoint specifications in `ConnectivityTest` are incomplete, the reachability result returns a value of AMBIGUOUS. For more information, see the Connectivity Test documentation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_networkmanagement1_cli/projects_locations-global-connectivity-tests-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource of the Connectivity Test to create: `projects/{project_id}/locations/global`"##),
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
            ("locations-global-connectivity-tests-delete",
                    Some(r##"Deletes a specific `ConnectivityTest`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_networkmanagement1_cli/projects_locations-global-connectivity-tests-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Connectivity Test resource name using the form: `projects/{project_id}/locations/global/connectivityTests/{test_id}`"##),
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
            ("locations-global-connectivity-tests-get",
                    Some(r##"Gets the details of a specific Connectivity Test."##),
                    "Details at http://byron.github.io/google-apis-rs/google_networkmanagement1_cli/projects_locations-global-connectivity-tests-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. `ConnectivityTest` resource name using the form: `projects/{project_id}/locations/global/connectivityTests/{test_id}`"##),
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
            ("locations-global-connectivity-tests-get-iam-policy",
                    Some(r##"Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set."##),
                    "Details at http://byron.github.io/google-apis-rs/google_networkmanagement1_cli/projects_locations-global-connectivity-tests-get-iam-policy",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
            ("locations-global-connectivity-tests-list",
                    Some(r##"Lists all Connectivity Tests owned by a project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_networkmanagement1_cli/projects_locations-global-connectivity-tests-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource of the Connectivity Tests: `projects/{project_id}/locations/global`"##),
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
            ("locations-global-connectivity-tests-patch",
                    Some(r##"Updates the configuration of an existing `ConnectivityTest`. After you update a test, the reachability analysis is performed as part of the long running operation, which completes when the analysis completes. The Reachability state in the test resource is updated with the new result. If the endpoint specifications in `ConnectivityTest` are invalid (for example, they contain non-existent resources in the network, or the user does not have read permissions to the network configurations of listed projects), then the reachability result returns a value of UNKNOWN. If the endpoint specifications in `ConnectivityTest` are incomplete, the reachability result returns a value of `AMBIGUOUS`. See the documentation in `ConnectivityTest` for more details."##),
                    "Details at http://byron.github.io/google-apis-rs/google_networkmanagement1_cli/projects_locations-global-connectivity-tests-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Identifier. Unique name of the resource using the form: `projects/{project_id}/locations/global/connectivityTests/{test_id}`"##),
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
            ("locations-global-connectivity-tests-rerun",
                    Some(r##"Rerun an existing `ConnectivityTest`. After the user triggers the rerun, the reachability analysis is performed as part of the long running operation, which completes when the analysis completes. Even though the test configuration remains the same, the reachability result may change due to underlying network configuration changes. If the endpoint specifications in `ConnectivityTest` become invalid (for example, specified resources are deleted in the network, or you lost read permissions to the network configurations of listed projects), then the reachability result returns a value of `UNKNOWN`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_networkmanagement1_cli/projects_locations-global-connectivity-tests-rerun",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. Connectivity Test resource name using the form: `projects/{project_id}/locations/global/connectivityTests/{test_id}`"##),
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
            ("locations-global-connectivity-tests-set-iam-policy",
                    Some(r##"Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors."##),
                    "Details at http://byron.github.io/google-apis-rs/google_networkmanagement1_cli/projects_locations-global-connectivity-tests-set-iam-policy",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
            ("locations-global-connectivity-tests-test-iam-permissions",
                    Some(r##"Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning."##),
                    "Details at http://byron.github.io/google-apis-rs/google_networkmanagement1_cli/projects_locations-global-connectivity-tests-test-iam-permissions",
                  vec![
                    (Some(r##"resource"##),
                     None,
                     Some(r##"REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field."##),
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
            ("locations-global-operations-cancel",
                    Some(r##"Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of `1`, corresponding to `Code.CANCELLED`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_networkmanagement1_cli/projects_locations-global-operations-cancel",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the operation resource to be cancelled."##),
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
            ("locations-global-operations-delete",
                    Some(r##"Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_networkmanagement1_cli/projects_locations-global-operations-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the operation resource to be deleted."##),
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
            ("locations-global-operations-get",
                    Some(r##"Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_networkmanagement1_cli/projects_locations-global-operations-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the operation resource."##),
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
            ("locations-global-operations-list",
                    Some(r##"Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_networkmanagement1_cli/projects_locations-global-operations-list",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the operation's parent resource."##),
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
            ("locations-list",
                    Some(r##"Lists information about the supported locations for this service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_networkmanagement1_cli/projects_locations-list",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The resource that owns the locations collection, if applicable."##),
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
            ("locations-vpc-flow-logs-configs-create",
                    Some(r##"Creates a new `VpcFlowLogsConfig`. If a configuration with the exact same settings already exists (even if the ID is different), the creation fails. Notes: 1. Creating a configuration with `state=DISABLED` will fail 2. The following fields are not considered as settings for the purpose of the check mentioned above, therefore - creating another configuration with the same fields but different values for the following fields will fail as well: * name * create_time * update_time * labels * description"##),
                    "Details at http://byron.github.io/google-apis-rs/google_networkmanagement1_cli/projects_locations-vpc-flow-logs-configs-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource of the VpcFlowLogsConfig to create, in one of the following formats: - For project-level resources: `projects/{project_id}/locations/global` - For organization-level resources: `organizations/{organization_id}/locations/global`"##),
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
            ("locations-vpc-flow-logs-configs-delete",
                    Some(r##"Deletes a specific `VpcFlowLogsConfig`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_networkmanagement1_cli/projects_locations-vpc-flow-logs-configs-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name of the VpcFlowLogsConfig, in one of the following formats: - For a project-level resource: `projects/{project_id}/locations/global/vpcFlowLogsConfigs/{vpc_flow_logs_config_id}` - For an organization-level resource: `organizations/{organization_id}/locations/global/vpcFlowLogsConfigs/{vpc_flow_logs_config_id}`"##),
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
            ("locations-vpc-flow-logs-configs-get",
                    Some(r##"Gets the details of a specific `VpcFlowLogsConfig`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_networkmanagement1_cli/projects_locations-vpc-flow-logs-configs-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name of the VpcFlowLogsConfig, in one of the following formats: - For project-level resources: `projects/{project_id}/locations/global/vpcFlowLogsConfigs/{vpc_flow_logs_config_id}` - For organization-level resources: `organizations/{organization_id}/locations/global/vpcFlowLogsConfigs/{vpc_flow_logs_config_id}`"##),
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
            ("locations-vpc-flow-logs-configs-list",
                    Some(r##"Lists all `VpcFlowLogsConfigs` in a given project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_networkmanagement1_cli/projects_locations-vpc-flow-logs-configs-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource of the VpcFlowLogsConfig, in one of the following formats: - For project-level resources: `projects/{project_id}/locations/global` - For organization-level resources: `organizations/{organization_id}/locations/global`"##),
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
            ("locations-vpc-flow-logs-configs-patch",
                    Some(r##"Updates an existing `VpcFlowLogsConfig`. If a configuration with the exact same settings already exists (even if the ID is different), the creation fails. Notes: 1. Updating a configuration with `state=DISABLED` will fail. 2. The following fields are not considered as settings for the purpose of the check mentioned above, therefore - updating another configuration with the same fields but different values for the following fields will fail as well: * name * create_time * update_time * labels * description"##),
                    "Details at http://byron.github.io/google-apis-rs/google_networkmanagement1_cli/projects_locations-vpc-flow-logs-configs-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Identifier. Unique name of the configuration. The name can have one of the following forms: - For project-level configurations: `projects/{project_id}/locations/global/vpcFlowLogsConfigs/{vpc_flow_logs_config_id}` - For organization-level configurations: `organizations/{organization_id}/locations/global/vpcFlowLogsConfigs/{vpc_flow_logs_config_id}`"##),
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
            ("locations-vpc-flow-logs-configs-query-org-vpc-flow-logs-configs",
                    Some(r##"QueryOrgVpcFlowLogsConfigs returns a list of all organization-level VPC Flow Logs configurations applicable to the specified project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_networkmanagement1_cli/projects_locations-vpc-flow-logs-configs-query-org-vpc-flow-logs-configs",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource of the VpcFlowLogsConfig, specified in the following format: `projects/{project_id}/locations/global`"##),
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
            ("locations-vpc-flow-logs-configs-show-effective-flow-logs-configs",
                    Some(r##"ShowEffectiveFlowLogsConfigs returns a list of all VPC Flow Logs configurations applicable to a specified resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_networkmanagement1_cli/projects_locations-vpc-flow-logs-configs-show-effective-flow-logs-configs",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource of the VpcFlowLogsConfig, specified in the following format: `projects/{project_id}/locations/global`"##),
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

    let mut app = App::new("networkmanagement1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("7.0.0+20251203")
           .about("The Network Management API provides a collection of network performance monitoring and diagnostic capabilities.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_networkmanagement1_cli")
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
                   .help("Debug print all errors")
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
                let arg_name_str = match (arg_name, flag) {
                    (&Some(an), _) => an,
                    (_, &Some(f)) => f,
                    _ => unreachable!(),
                };
                let mut arg = Arg::with_name(arg_name_str).empty_values(false);
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

    let debug = matches.is_present("adebug");
    let connector = hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .unwrap()
        .https_or_http()
        .enable_http2()
        .build();

    match Engine::new(matches, connector).await {
        Err(err) => {
            exit_status = err.exit_code;
            writeln!(std::io::stderr(), "{}", err).ok();
        }
        Ok(engine) => {
            if let Err(doit_err) = engine.doit().await {
                exit_status = 1;
                match doit_err {
                    DoitError::IoError(path, err) => {
                        writeln!(
                            std::io::stderr(),
                            "Failed to open output file '{}': {}",
                            path,
                            err
                        )
                        .ok();
                    }
                    DoitError::ApiError(err) => {
                        if debug {
                            writeln!(std::io::stderr(), "{:#?}", err).ok();
                        } else {
                            writeln!(std::io::stderr(), "{}", err).ok();
                        }
                    }
                }
            }
        }
    }

    std::process::exit(exit_status);
}
