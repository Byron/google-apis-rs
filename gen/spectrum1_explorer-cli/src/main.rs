// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_spectrum1_explorer::{api, Error, oauth2, client::chrono, FieldMask};


use google_clis_common as client;

use client::{InvalidOptionsError, CLIError, arg_from_str, writer_from_opts, parse_kv_arg,
          input_file_from_opts, input_mime_from_opts, FieldCursor, FieldError, CallType, UploadProtocol,
          calltype_from_str, remove_json_null_values, ComplexType, JsonType, JsonTypeInfo};

use std::default::Default;
use std::error::Error as StdError;
use std::str::FromStr;

use serde_json as json;
use clap::ArgMatches;
use http::Uri;
use hyper::client::connect;
use tokio::io::{AsyncRead, AsyncWrite};
use tower_service;

enum DoitError {
    IoError(String, io::Error),
    ApiError(Error),
}

struct Engine<'n, S> {
    opt: ArgMatches<'n>,
    hub: api::Spectrum<S>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n, S> Engine<'n, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    async fn _paws_get_spectrum(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "antenna.height" => Some(("antenna.height", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "antenna.height-type" => Some(("antenna.heightType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "antenna.height-uncertainty" => Some(("antenna.heightUncertainty", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device-desc.etsi-en-device-category" => Some(("deviceDesc.etsiEnDeviceCategory", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-desc.etsi-en-device-emissions-class" => Some(("deviceDesc.etsiEnDeviceEmissionsClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-desc.etsi-en-device-type" => Some(("deviceDesc.etsiEnDeviceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-desc.etsi-en-technology-id" => Some(("deviceDesc.etsiEnTechnologyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-desc.fcc-id" => Some(("deviceDesc.fccId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-desc.fcc-tvbd-device-type" => Some(("deviceDesc.fccTvbdDeviceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-desc.manufacturer-id" => Some(("deviceDesc.manufacturerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-desc.model-id" => Some(("deviceDesc.modelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-desc.ruleset-ids" => Some(("deviceDesc.rulesetIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "device-desc.serial-number" => Some(("deviceDesc.serialNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.confidence" => Some(("location.confidence", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "location.point.center.latitude" => Some(("location.point.center.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "location.point.center.longitude" => Some(("location.point.center.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "location.point.orientation" => Some(("location.point.orientation", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "location.point.semi-major-axis" => Some(("location.point.semiMajorAxis", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "location.point.semi-minor-axis" => Some(("location.point.semiMinorAxis", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "master-device-desc.etsi-en-device-category" => Some(("masterDeviceDesc.etsiEnDeviceCategory", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "master-device-desc.etsi-en-device-emissions-class" => Some(("masterDeviceDesc.etsiEnDeviceEmissionsClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "master-device-desc.etsi-en-device-type" => Some(("masterDeviceDesc.etsiEnDeviceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "master-device-desc.etsi-en-technology-id" => Some(("masterDeviceDesc.etsiEnTechnologyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "master-device-desc.fcc-id" => Some(("masterDeviceDesc.fccId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "master-device-desc.fcc-tvbd-device-type" => Some(("masterDeviceDesc.fccTvbdDeviceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "master-device-desc.manufacturer-id" => Some(("masterDeviceDesc.manufacturerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "master-device-desc.model-id" => Some(("masterDeviceDesc.modelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "master-device-desc.ruleset-ids" => Some(("masterDeviceDesc.rulesetIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "master-device-desc.serial-number" => Some(("masterDeviceDesc.serialNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.operator.adr.code" => Some(("owner.operator.adr.code", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.operator.adr.country" => Some(("owner.operator.adr.country", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.operator.adr.locality" => Some(("owner.operator.adr.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.operator.adr.pobox" => Some(("owner.operator.adr.pobox", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.operator.adr.region" => Some(("owner.operator.adr.region", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.operator.adr.street" => Some(("owner.operator.adr.street", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.operator.email.text" => Some(("owner.operator.email.text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.operator.fn" => Some(("owner.operator.fn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.operator.org.text" => Some(("owner.operator.org.text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.operator.tel.uri" => Some(("owner.operator.tel.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.owner.adr.code" => Some(("owner.owner.adr.code", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.owner.adr.country" => Some(("owner.owner.adr.country", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.owner.adr.locality" => Some(("owner.owner.adr.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.owner.adr.pobox" => Some(("owner.owner.adr.pobox", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.owner.adr.region" => Some(("owner.owner.adr.region", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.owner.adr.street" => Some(("owner.owner.adr.street", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.owner.email.text" => Some(("owner.owner.email.text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.owner.fn" => Some(("owner.owner.fn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.owner.org.text" => Some(("owner.owner.org.text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.owner.tel.uri" => Some(("owner.owner.tel.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-type" => Some(("requestType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "version" => Some(("version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["adr", "antenna", "center", "code", "confidence", "country", "device-desc", "email", "etsi-en-device-category", "etsi-en-device-emissions-class", "etsi-en-device-type", "etsi-en-technology-id", "fcc-id", "fcc-tvbd-device-type", "fn", "height", "height-type", "height-uncertainty", "latitude", "locality", "location", "longitude", "manufacturer-id", "master-device-desc", "model-id", "operator", "org", "orientation", "owner", "pobox", "point", "region", "request-type", "ruleset-ids", "semi-major-axis", "semi-minor-axis", "serial-number", "street", "tel", "text", "type", "uri", "version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::PawsGetSpectrumRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.paws().get_spectrum(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _paws_get_spectrum_batch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "antenna.height" => Some(("antenna.height", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "antenna.height-type" => Some(("antenna.heightType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "antenna.height-uncertainty" => Some(("antenna.heightUncertainty", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device-desc.etsi-en-device-category" => Some(("deviceDesc.etsiEnDeviceCategory", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-desc.etsi-en-device-emissions-class" => Some(("deviceDesc.etsiEnDeviceEmissionsClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-desc.etsi-en-device-type" => Some(("deviceDesc.etsiEnDeviceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-desc.etsi-en-technology-id" => Some(("deviceDesc.etsiEnTechnologyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-desc.fcc-id" => Some(("deviceDesc.fccId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-desc.fcc-tvbd-device-type" => Some(("deviceDesc.fccTvbdDeviceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-desc.manufacturer-id" => Some(("deviceDesc.manufacturerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-desc.model-id" => Some(("deviceDesc.modelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-desc.ruleset-ids" => Some(("deviceDesc.rulesetIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "device-desc.serial-number" => Some(("deviceDesc.serialNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "master-device-desc.etsi-en-device-category" => Some(("masterDeviceDesc.etsiEnDeviceCategory", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "master-device-desc.etsi-en-device-emissions-class" => Some(("masterDeviceDesc.etsiEnDeviceEmissionsClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "master-device-desc.etsi-en-device-type" => Some(("masterDeviceDesc.etsiEnDeviceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "master-device-desc.etsi-en-technology-id" => Some(("masterDeviceDesc.etsiEnTechnologyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "master-device-desc.fcc-id" => Some(("masterDeviceDesc.fccId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "master-device-desc.fcc-tvbd-device-type" => Some(("masterDeviceDesc.fccTvbdDeviceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "master-device-desc.manufacturer-id" => Some(("masterDeviceDesc.manufacturerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "master-device-desc.model-id" => Some(("masterDeviceDesc.modelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "master-device-desc.ruleset-ids" => Some(("masterDeviceDesc.rulesetIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "master-device-desc.serial-number" => Some(("masterDeviceDesc.serialNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.operator.adr.code" => Some(("owner.operator.adr.code", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.operator.adr.country" => Some(("owner.operator.adr.country", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.operator.adr.locality" => Some(("owner.operator.adr.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.operator.adr.pobox" => Some(("owner.operator.adr.pobox", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.operator.adr.region" => Some(("owner.operator.adr.region", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.operator.adr.street" => Some(("owner.operator.adr.street", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.operator.email.text" => Some(("owner.operator.email.text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.operator.fn" => Some(("owner.operator.fn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.operator.org.text" => Some(("owner.operator.org.text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.operator.tel.uri" => Some(("owner.operator.tel.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.owner.adr.code" => Some(("owner.owner.adr.code", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.owner.adr.country" => Some(("owner.owner.adr.country", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.owner.adr.locality" => Some(("owner.owner.adr.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.owner.adr.pobox" => Some(("owner.owner.adr.pobox", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.owner.adr.region" => Some(("owner.owner.adr.region", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.owner.adr.street" => Some(("owner.owner.adr.street", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.owner.email.text" => Some(("owner.owner.email.text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.owner.fn" => Some(("owner.owner.fn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.owner.org.text" => Some(("owner.owner.org.text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "owner.owner.tel.uri" => Some(("owner.owner.tel.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "request-type" => Some(("requestType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "version" => Some(("version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["adr", "antenna", "code", "country", "device-desc", "email", "etsi-en-device-category", "etsi-en-device-emissions-class", "etsi-en-device-type", "etsi-en-technology-id", "fcc-id", "fcc-tvbd-device-type", "fn", "height", "height-type", "height-uncertainty", "locality", "manufacturer-id", "master-device-desc", "model-id", "operator", "org", "owner", "pobox", "region", "request-type", "ruleset-ids", "serial-number", "street", "tel", "text", "type", "uri", "version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::PawsGetSpectrumBatchRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.paws().get_spectrum_batch(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _paws_init(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "device-desc.etsi-en-device-category" => Some(("deviceDesc.etsiEnDeviceCategory", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-desc.etsi-en-device-emissions-class" => Some(("deviceDesc.etsiEnDeviceEmissionsClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-desc.etsi-en-device-type" => Some(("deviceDesc.etsiEnDeviceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-desc.etsi-en-technology-id" => Some(("deviceDesc.etsiEnTechnologyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-desc.fcc-id" => Some(("deviceDesc.fccId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-desc.fcc-tvbd-device-type" => Some(("deviceDesc.fccTvbdDeviceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-desc.manufacturer-id" => Some(("deviceDesc.manufacturerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-desc.model-id" => Some(("deviceDesc.modelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-desc.ruleset-ids" => Some(("deviceDesc.rulesetIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "device-desc.serial-number" => Some(("deviceDesc.serialNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.confidence" => Some(("location.confidence", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "location.point.center.latitude" => Some(("location.point.center.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "location.point.center.longitude" => Some(("location.point.center.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "location.point.orientation" => Some(("location.point.orientation", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "location.point.semi-major-axis" => Some(("location.point.semiMajorAxis", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "location.point.semi-minor-axis" => Some(("location.point.semiMinorAxis", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "version" => Some(("version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["center", "confidence", "device-desc", "etsi-en-device-category", "etsi-en-device-emissions-class", "etsi-en-device-type", "etsi-en-technology-id", "fcc-id", "fcc-tvbd-device-type", "latitude", "location", "longitude", "manufacturer-id", "model-id", "orientation", "point", "ruleset-ids", "semi-major-axis", "semi-minor-axis", "serial-number", "type", "version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::PawsInitRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.paws().init(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _paws_notify_spectrum_use(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "device-desc.etsi-en-device-category" => Some(("deviceDesc.etsiEnDeviceCategory", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-desc.etsi-en-device-emissions-class" => Some(("deviceDesc.etsiEnDeviceEmissionsClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-desc.etsi-en-device-type" => Some(("deviceDesc.etsiEnDeviceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-desc.etsi-en-technology-id" => Some(("deviceDesc.etsiEnTechnologyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-desc.fcc-id" => Some(("deviceDesc.fccId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-desc.fcc-tvbd-device-type" => Some(("deviceDesc.fccTvbdDeviceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-desc.manufacturer-id" => Some(("deviceDesc.manufacturerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-desc.model-id" => Some(("deviceDesc.modelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-desc.ruleset-ids" => Some(("deviceDesc.rulesetIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "device-desc.serial-number" => Some(("deviceDesc.serialNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.confidence" => Some(("location.confidence", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "location.point.center.latitude" => Some(("location.point.center.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "location.point.center.longitude" => Some(("location.point.center.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "location.point.orientation" => Some(("location.point.orientation", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "location.point.semi-major-axis" => Some(("location.point.semiMajorAxis", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "location.point.semi-minor-axis" => Some(("location.point.semiMinorAxis", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "version" => Some(("version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["center", "confidence", "device-desc", "etsi-en-device-category", "etsi-en-device-emissions-class", "etsi-en-device-type", "etsi-en-technology-id", "fcc-id", "fcc-tvbd-device-type", "latitude", "location", "longitude", "manufacturer-id", "model-id", "orientation", "point", "ruleset-ids", "semi-major-axis", "semi-minor-axis", "serial-number", "type", "version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::PawsNotifySpectrumUseRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.paws().notify_spectrum_use(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _paws_register(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "antenna.height" => Some(("antenna.height", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "antenna.height-type" => Some(("antenna.heightType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "antenna.height-uncertainty" => Some(("antenna.heightUncertainty", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device-desc.etsi-en-device-category" => Some(("deviceDesc.etsiEnDeviceCategory", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-desc.etsi-en-device-emissions-class" => Some(("deviceDesc.etsiEnDeviceEmissionsClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-desc.etsi-en-device-type" => Some(("deviceDesc.etsiEnDeviceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-desc.etsi-en-technology-id" => Some(("deviceDesc.etsiEnTechnologyId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-desc.fcc-id" => Some(("deviceDesc.fccId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-desc.fcc-tvbd-device-type" => Some(("deviceDesc.fccTvbdDeviceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-desc.manufacturer-id" => Some(("deviceDesc.manufacturerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-desc.model-id" => Some(("deviceDesc.modelId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-desc.ruleset-ids" => Some(("deviceDesc.rulesetIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "device-desc.serial-number" => Some(("deviceDesc.serialNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-owner.operator.adr.code" => Some(("deviceOwner.operator.adr.code", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-owner.operator.adr.country" => Some(("deviceOwner.operator.adr.country", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-owner.operator.adr.locality" => Some(("deviceOwner.operator.adr.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-owner.operator.adr.pobox" => Some(("deviceOwner.operator.adr.pobox", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-owner.operator.adr.region" => Some(("deviceOwner.operator.adr.region", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-owner.operator.adr.street" => Some(("deviceOwner.operator.adr.street", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-owner.operator.email.text" => Some(("deviceOwner.operator.email.text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-owner.operator.fn" => Some(("deviceOwner.operator.fn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-owner.operator.org.text" => Some(("deviceOwner.operator.org.text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-owner.operator.tel.uri" => Some(("deviceOwner.operator.tel.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-owner.owner.adr.code" => Some(("deviceOwner.owner.adr.code", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-owner.owner.adr.country" => Some(("deviceOwner.owner.adr.country", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-owner.owner.adr.locality" => Some(("deviceOwner.owner.adr.locality", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-owner.owner.adr.pobox" => Some(("deviceOwner.owner.adr.pobox", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-owner.owner.adr.region" => Some(("deviceOwner.owner.adr.region", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-owner.owner.adr.street" => Some(("deviceOwner.owner.adr.street", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-owner.owner.email.text" => Some(("deviceOwner.owner.email.text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-owner.owner.fn" => Some(("deviceOwner.owner.fn", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-owner.owner.org.text" => Some(("deviceOwner.owner.org.text", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-owner.owner.tel.uri" => Some(("deviceOwner.owner.tel.uri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location.confidence" => Some(("location.confidence", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "location.point.center.latitude" => Some(("location.point.center.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "location.point.center.longitude" => Some(("location.point.center.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "location.point.orientation" => Some(("location.point.orientation", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "location.point.semi-major-axis" => Some(("location.point.semiMajorAxis", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "location.point.semi-minor-axis" => Some(("location.point.semiMinorAxis", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "version" => Some(("version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["adr", "antenna", "center", "code", "confidence", "country", "device-desc", "device-owner", "email", "etsi-en-device-category", "etsi-en-device-emissions-class", "etsi-en-device-type", "etsi-en-technology-id", "fcc-id", "fcc-tvbd-device-type", "fn", "height", "height-type", "height-uncertainty", "latitude", "locality", "location", "longitude", "manufacturer-id", "model-id", "operator", "org", "orientation", "owner", "pobox", "point", "region", "ruleset-ids", "semi-major-axis", "semi-minor-axis", "serial-number", "street", "tel", "text", "type", "uri", "version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::PawsRegisterRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.paws().register(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _paws_verify_device(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "version" => Some(("version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["type", "version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::PawsVerifyDeviceRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.paws().verify_device(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _doit(&self, dry_run: bool) -> Result<Result<(), DoitError>, Option<InvalidOptionsError>> {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Result<(), DoitError> = Ok(());
        let mut err_opt: Option<InvalidOptionsError> = None;
        match self.opt.subcommand() {
            ("paws", Some(opt)) => {
                match opt.subcommand() {
                    ("get-spectrum", Some(opt)) => {
                        call_result = self._paws_get_spectrum(opt, dry_run, &mut err).await;
                    },
                    ("get-spectrum-batch", Some(opt)) => {
                        call_result = self._paws_get_spectrum_batch(opt, dry_run, &mut err).await;
                    },
                    ("init", Some(opt)) => {
                        call_result = self._paws_init(opt, dry_run, &mut err).await;
                    },
                    ("notify-spectrum-use", Some(opt)) => {
                        call_result = self._paws_notify_spectrum_use(opt, dry_run, &mut err).await;
                    },
                    ("register", Some(opt)) => {
                        call_result = self._paws_register(opt, dry_run, &mut err).await;
                    },
                    ("verify-device", Some(opt)) => {
                        call_result = self._paws_verify_device(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("paws".to_string()));
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
    async fn new(opt: ArgMatches<'n>, connector: S) -> Result<Engine<'n, S>, InvalidOptionsError> {
        let (config_dir, secret) = {
            let config_dir = match client::assure_config_dir_exists(opt.value_of("folder").unwrap_or("~/.google-service-cli")) {
                Err(e) => return Err(InvalidOptionsError::single(e, 3)),
                Ok(p) => p,
            };

            match client::application_secret_from_directory(&config_dir, "spectrum1-explorer-secret.json",
                                                         "{\"installed\":{\"auth_uri\":\"https://accounts.google.com/o/oauth2/auth\",\"client_secret\":\"hCsslbCUyfehWMmbkG8vTYxG\",\"token_uri\":\"https://accounts.google.com/o/oauth2/token\",\"client_email\":\"\",\"redirect_uris\":[\"urn:ietf:wg:oauth:2.0:oob\",\"oob\"],\"client_x509_cert_url\":\"\",\"client_id\":\"620010449518-9ngf7o4dhs0dka470npqvor6dc5lqb9b.apps.googleusercontent.com\",\"auth_provider_x509_cert_url\":\"https://www.googleapis.com/oauth2/v1/certs\"}}") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let client = hyper::Client::builder().build(connector);

        let auth = oauth2::InstalledFlowAuthenticator::with_client(
            secret,
            oauth2::InstalledFlowReturnMethod::HTTPRedirect,
            client.clone(),
        ).persist_tokens_to_disk(format!("{}/spectrum1-explorer", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::Spectrum::new(client, auth),
            gp: vec!["alt", "fields", "key", "oauth-token", "pretty-print", "quota-user", "user-ip"],
            gpm: vec![
                    ("oauth-token", "oauth_token"),
                    ("pretty-print", "prettyPrint"),
                    ("quota-user", "quotaUser"),
                    ("user-ip", "userIp"),
                ]
        };

        match engine._doit(true).await {
            Err(Some(err)) => Err(err),
            Err(None)      => Ok(engine),
            Ok(_)          => unreachable!(),
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
        ("paws", "methods: 'get-spectrum', 'get-spectrum-batch', 'init', 'notify-spectrum-use', 'register' and 'verify-device'", vec![
            ("get-spectrum",
                    Some(r##"Requests information about the available spectrum for a device at a location. Requests from a fixed-mode device must include owner information so the device can be registered with the database."##),
                    "Details at http://byron.github.io/google-apis-rs/google_spectrum1_explorer_cli/paws_get-spectrum",
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
            ("get-spectrum-batch",
                    Some(r##"The Google Spectrum Database does not support batch requests, so this method always yields an UNIMPLEMENTED error."##),
                    "Details at http://byron.github.io/google-apis-rs/google_spectrum1_explorer_cli/paws_get-spectrum-batch",
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
            ("init",
                    Some(r##"Initializes the connection between a white space device and the database."##),
                    "Details at http://byron.github.io/google-apis-rs/google_spectrum1_explorer_cli/paws_init",
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
            ("notify-spectrum-use",
                    Some(r##"Notifies the database that the device has selected certain frequency ranges for transmission. Only to be invoked when required by the regulator. The Google Spectrum Database does not operate in domains that require notification, so this always yields an UNIMPLEMENTED error."##),
                    "Details at http://byron.github.io/google-apis-rs/google_spectrum1_explorer_cli/paws_notify-spectrum-use",
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
            ("register",
                    Some(r##"The Google Spectrum Database implements registration in the getSpectrum method. As such this always returns an UNIMPLEMENTED error."##),
                    "Details at http://byron.github.io/google-apis-rs/google_spectrum1_explorer_cli/paws_register",
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
            ("verify-device",
                    Some(r##"Validates a device for white space use in accordance with regulatory rules. The Google Spectrum Database does not support master/slave configurations, so this always yields an UNIMPLEMENTED error."##),
                    "Details at http://byron.github.io/google-apis-rs/google_spectrum1_explorer_cli/paws_verify-device",
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
            ]),
        
    ];
    
    let mut app = App::new("spectrum1-explorer")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+20170306")
           .about("API for spectrum-management functions.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_spectrum1_explorer_cli")
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

    let debug = matches.is_present("adebug");
    let connector = hyper_rustls::HttpsConnectorBuilder::new().with_native_roots()
        .unwrap()
        .https_or_http()
        .enable_http1()
        .build();

    match Engine::new(matches, connector).await {
        Err(err) => {
            exit_status = err.exit_code;
            writeln!(io::stderr(), "{}", err).ok();
        },
        Ok(engine) => {
            if let Err(doit_err) = engine.doit().await {
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
