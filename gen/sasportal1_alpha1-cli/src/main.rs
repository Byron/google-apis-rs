// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_sasportal1_alpha1::{api, Error, oauth2, client::chrono, FieldMask};


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
    hub: api::Sasportal<S>,
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
    async fn _customers_deployments_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "frns" => Some(("frns", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sas-user-ids" => Some(("sasUserIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["display-name", "frns", "name", "sas-user-ids"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalDeployment = json::value::from_value(object).unwrap();
        let mut call = self.hub.customers().deployments_create(request, opt.value_of("parent").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _customers_deployments_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.customers().deployments_delete(opt.value_of("name").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _customers_deployments_devices_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "active-config.air-interface.radio-technology" => Some(("activeConfig.airInterface.radioTechnology", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.air-interface.supported-spec" => Some(("activeConfig.airInterface.supportedSpec", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.call-sign" => Some(("activeConfig.callSign", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.category" => Some(("activeConfig.category", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-azimuth" => Some(("activeConfig.installationParams.antennaAzimuth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-beamwidth" => Some(("activeConfig.installationParams.antennaBeamwidth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-downtilt" => Some(("activeConfig.installationParams.antennaDowntilt", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-gain" => Some(("activeConfig.installationParams.antennaGain", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-model" => Some(("activeConfig.installationParams.antennaModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.installation-params.cpe-cbsd-indication" => Some(("activeConfig.installationParams.cpeCbsdIndication", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "active-config.installation-params.eirp-capability" => Some(("activeConfig.installationParams.eirpCapability", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "active-config.installation-params.height" => Some(("activeConfig.installationParams.height", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.height-type" => Some(("activeConfig.installationParams.heightType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.installation-params.horizontal-accuracy" => Some(("activeConfig.installationParams.horizontalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.indoor-deployment" => Some(("activeConfig.installationParams.indoorDeployment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "active-config.installation-params.latitude" => Some(("activeConfig.installationParams.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.longitude" => Some(("activeConfig.installationParams.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.vertical-accuracy" => Some(("activeConfig.installationParams.verticalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.is-signed" => Some(("activeConfig.isSigned", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "active-config.measurement-capabilities" => Some(("activeConfig.measurementCapabilities", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "active-config.model.firmware-version" => Some(("activeConfig.model.firmwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.model.hardware-version" => Some(("activeConfig.model.hardwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.model.name" => Some(("activeConfig.model.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.model.software-version" => Some(("activeConfig.model.softwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.model.vendor" => Some(("activeConfig.model.vendor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.state" => Some(("activeConfig.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.update-time" => Some(("activeConfig.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.user-id" => Some(("activeConfig.userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.antenna-model" => Some(("deviceMetadata.antennaModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.common-channel-group" => Some(("deviceMetadata.commonChannelGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.interference-coordination-group" => Some(("deviceMetadata.interferenceCoordinationGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validated" => Some(("deviceMetadata.nrqzValidated", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.case-id" => Some(("deviceMetadata.nrqzValidation.caseId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.cpi-id" => Some(("deviceMetadata.nrqzValidation.cpiId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.latitude" => Some(("deviceMetadata.nrqzValidation.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.longitude" => Some(("deviceMetadata.nrqzValidation.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.state" => Some(("deviceMetadata.nrqzValidation.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fcc-id" => Some(("fccId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.air-interface.radio-technology" => Some(("preloadedConfig.airInterface.radioTechnology", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.air-interface.supported-spec" => Some(("preloadedConfig.airInterface.supportedSpec", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.call-sign" => Some(("preloadedConfig.callSign", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.category" => Some(("preloadedConfig.category", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-azimuth" => Some(("preloadedConfig.installationParams.antennaAzimuth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-beamwidth" => Some(("preloadedConfig.installationParams.antennaBeamwidth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-downtilt" => Some(("preloadedConfig.installationParams.antennaDowntilt", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-gain" => Some(("preloadedConfig.installationParams.antennaGain", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-model" => Some(("preloadedConfig.installationParams.antennaModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.cpe-cbsd-indication" => Some(("preloadedConfig.installationParams.cpeCbsdIndication", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.eirp-capability" => Some(("preloadedConfig.installationParams.eirpCapability", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.height" => Some(("preloadedConfig.installationParams.height", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.height-type" => Some(("preloadedConfig.installationParams.heightType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.horizontal-accuracy" => Some(("preloadedConfig.installationParams.horizontalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.indoor-deployment" => Some(("preloadedConfig.installationParams.indoorDeployment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.latitude" => Some(("preloadedConfig.installationParams.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.longitude" => Some(("preloadedConfig.installationParams.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.vertical-accuracy" => Some(("preloadedConfig.installationParams.verticalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.is-signed" => Some(("preloadedConfig.isSigned", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "preloaded-config.measurement-capabilities" => Some(("preloadedConfig.measurementCapabilities", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "preloaded-config.model.firmware-version" => Some(("preloadedConfig.model.firmwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.model.hardware-version" => Some(("preloadedConfig.model.hardwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.model.name" => Some(("preloadedConfig.model.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.model.software-version" => Some(("preloadedConfig.model.softwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.model.vendor" => Some(("preloadedConfig.model.vendor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.state" => Some(("preloadedConfig.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.update-time" => Some(("preloadedConfig.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.user-id" => Some(("preloadedConfig.userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "serial-number" => Some(("serialNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["active-config", "air-interface", "antenna-azimuth", "antenna-beamwidth", "antenna-downtilt", "antenna-gain", "antenna-model", "call-sign", "case-id", "category", "common-channel-group", "cpe-cbsd-indication", "cpi-id", "device-metadata", "display-name", "eirp-capability", "fcc-id", "firmware-version", "hardware-version", "height", "height-type", "horizontal-accuracy", "indoor-deployment", "installation-params", "interference-coordination-group", "is-signed", "latitude", "longitude", "measurement-capabilities", "model", "name", "nrqz-validated", "nrqz-validation", "preloaded-config", "radio-technology", "serial-number", "software-version", "state", "supported-spec", "update-time", "user-id", "vendor", "vertical-accuracy"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalDevice = json::value::from_value(object).unwrap();
        let mut call = self.hub.customers().deployments_devices_create(request, opt.value_of("parent").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _customers_deployments_devices_create_signed(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "encoded-device" => Some(("encodedDevice", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "installer-id" => Some(("installerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["encoded-device", "installer-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalCreateSignedDeviceRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.customers().deployments_devices_create_signed(request, opt.value_of("parent").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _customers_deployments_devices_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.customers().deployments_devices_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
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
                                                                           v.extend(["filter", "page-size", "page-token"].iter().map(|v|*v));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _customers_deployments_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.customers().deployments_get(opt.value_of("name").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _customers_deployments_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.customers().deployments_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
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
                                                                           v.extend(["filter", "page-size", "page-token"].iter().map(|v|*v));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _customers_deployments_move(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "destination" => Some(("destination", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["destination"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalMoveDeploymentRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.customers().deployments_move(request, opt.value_of("name").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _customers_deployments_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "frns" => Some(("frns", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sas-user-ids" => Some(("sasUserIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["display-name", "frns", "name", "sas-user-ids"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalDeployment = json::value::from_value(object).unwrap();
        let mut call = self.hub.customers().deployments_patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
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
                                                                           v.extend(["update-mask"].iter().map(|v|*v));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _customers_devices_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "active-config.air-interface.radio-technology" => Some(("activeConfig.airInterface.radioTechnology", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.air-interface.supported-spec" => Some(("activeConfig.airInterface.supportedSpec", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.call-sign" => Some(("activeConfig.callSign", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.category" => Some(("activeConfig.category", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-azimuth" => Some(("activeConfig.installationParams.antennaAzimuth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-beamwidth" => Some(("activeConfig.installationParams.antennaBeamwidth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-downtilt" => Some(("activeConfig.installationParams.antennaDowntilt", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-gain" => Some(("activeConfig.installationParams.antennaGain", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-model" => Some(("activeConfig.installationParams.antennaModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.installation-params.cpe-cbsd-indication" => Some(("activeConfig.installationParams.cpeCbsdIndication", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "active-config.installation-params.eirp-capability" => Some(("activeConfig.installationParams.eirpCapability", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "active-config.installation-params.height" => Some(("activeConfig.installationParams.height", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.height-type" => Some(("activeConfig.installationParams.heightType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.installation-params.horizontal-accuracy" => Some(("activeConfig.installationParams.horizontalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.indoor-deployment" => Some(("activeConfig.installationParams.indoorDeployment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "active-config.installation-params.latitude" => Some(("activeConfig.installationParams.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.longitude" => Some(("activeConfig.installationParams.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.vertical-accuracy" => Some(("activeConfig.installationParams.verticalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.is-signed" => Some(("activeConfig.isSigned", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "active-config.measurement-capabilities" => Some(("activeConfig.measurementCapabilities", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "active-config.model.firmware-version" => Some(("activeConfig.model.firmwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.model.hardware-version" => Some(("activeConfig.model.hardwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.model.name" => Some(("activeConfig.model.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.model.software-version" => Some(("activeConfig.model.softwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.model.vendor" => Some(("activeConfig.model.vendor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.state" => Some(("activeConfig.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.update-time" => Some(("activeConfig.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.user-id" => Some(("activeConfig.userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.antenna-model" => Some(("deviceMetadata.antennaModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.common-channel-group" => Some(("deviceMetadata.commonChannelGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.interference-coordination-group" => Some(("deviceMetadata.interferenceCoordinationGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validated" => Some(("deviceMetadata.nrqzValidated", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.case-id" => Some(("deviceMetadata.nrqzValidation.caseId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.cpi-id" => Some(("deviceMetadata.nrqzValidation.cpiId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.latitude" => Some(("deviceMetadata.nrqzValidation.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.longitude" => Some(("deviceMetadata.nrqzValidation.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.state" => Some(("deviceMetadata.nrqzValidation.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fcc-id" => Some(("fccId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.air-interface.radio-technology" => Some(("preloadedConfig.airInterface.radioTechnology", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.air-interface.supported-spec" => Some(("preloadedConfig.airInterface.supportedSpec", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.call-sign" => Some(("preloadedConfig.callSign", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.category" => Some(("preloadedConfig.category", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-azimuth" => Some(("preloadedConfig.installationParams.antennaAzimuth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-beamwidth" => Some(("preloadedConfig.installationParams.antennaBeamwidth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-downtilt" => Some(("preloadedConfig.installationParams.antennaDowntilt", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-gain" => Some(("preloadedConfig.installationParams.antennaGain", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-model" => Some(("preloadedConfig.installationParams.antennaModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.cpe-cbsd-indication" => Some(("preloadedConfig.installationParams.cpeCbsdIndication", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.eirp-capability" => Some(("preloadedConfig.installationParams.eirpCapability", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.height" => Some(("preloadedConfig.installationParams.height", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.height-type" => Some(("preloadedConfig.installationParams.heightType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.horizontal-accuracy" => Some(("preloadedConfig.installationParams.horizontalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.indoor-deployment" => Some(("preloadedConfig.installationParams.indoorDeployment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.latitude" => Some(("preloadedConfig.installationParams.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.longitude" => Some(("preloadedConfig.installationParams.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.vertical-accuracy" => Some(("preloadedConfig.installationParams.verticalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.is-signed" => Some(("preloadedConfig.isSigned", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "preloaded-config.measurement-capabilities" => Some(("preloadedConfig.measurementCapabilities", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "preloaded-config.model.firmware-version" => Some(("preloadedConfig.model.firmwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.model.hardware-version" => Some(("preloadedConfig.model.hardwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.model.name" => Some(("preloadedConfig.model.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.model.software-version" => Some(("preloadedConfig.model.softwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.model.vendor" => Some(("preloadedConfig.model.vendor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.state" => Some(("preloadedConfig.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.update-time" => Some(("preloadedConfig.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.user-id" => Some(("preloadedConfig.userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "serial-number" => Some(("serialNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["active-config", "air-interface", "antenna-azimuth", "antenna-beamwidth", "antenna-downtilt", "antenna-gain", "antenna-model", "call-sign", "case-id", "category", "common-channel-group", "cpe-cbsd-indication", "cpi-id", "device-metadata", "display-name", "eirp-capability", "fcc-id", "firmware-version", "hardware-version", "height", "height-type", "horizontal-accuracy", "indoor-deployment", "installation-params", "interference-coordination-group", "is-signed", "latitude", "longitude", "measurement-capabilities", "model", "name", "nrqz-validated", "nrqz-validation", "preloaded-config", "radio-technology", "serial-number", "software-version", "state", "supported-spec", "update-time", "user-id", "vendor", "vertical-accuracy"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalDevice = json::value::from_value(object).unwrap();
        let mut call = self.hub.customers().devices_create(request, opt.value_of("parent").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _customers_devices_create_signed(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "encoded-device" => Some(("encodedDevice", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "installer-id" => Some(("installerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["encoded-device", "installer-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalCreateSignedDeviceRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.customers().devices_create_signed(request, opt.value_of("parent").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _customers_devices_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.customers().devices_delete(opt.value_of("name").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _customers_devices_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.customers().devices_get(opt.value_of("name").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _customers_devices_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.customers().devices_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
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
                                                                           v.extend(["filter", "page-size", "page-token"].iter().map(|v|*v));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _customers_devices_move(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "destination" => Some(("destination", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["destination"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalMoveDeviceRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.customers().devices_move(request, opt.value_of("name").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _customers_devices_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "active-config.air-interface.radio-technology" => Some(("activeConfig.airInterface.radioTechnology", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.air-interface.supported-spec" => Some(("activeConfig.airInterface.supportedSpec", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.call-sign" => Some(("activeConfig.callSign", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.category" => Some(("activeConfig.category", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-azimuth" => Some(("activeConfig.installationParams.antennaAzimuth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-beamwidth" => Some(("activeConfig.installationParams.antennaBeamwidth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-downtilt" => Some(("activeConfig.installationParams.antennaDowntilt", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-gain" => Some(("activeConfig.installationParams.antennaGain", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-model" => Some(("activeConfig.installationParams.antennaModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.installation-params.cpe-cbsd-indication" => Some(("activeConfig.installationParams.cpeCbsdIndication", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "active-config.installation-params.eirp-capability" => Some(("activeConfig.installationParams.eirpCapability", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "active-config.installation-params.height" => Some(("activeConfig.installationParams.height", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.height-type" => Some(("activeConfig.installationParams.heightType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.installation-params.horizontal-accuracy" => Some(("activeConfig.installationParams.horizontalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.indoor-deployment" => Some(("activeConfig.installationParams.indoorDeployment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "active-config.installation-params.latitude" => Some(("activeConfig.installationParams.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.longitude" => Some(("activeConfig.installationParams.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.vertical-accuracy" => Some(("activeConfig.installationParams.verticalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.is-signed" => Some(("activeConfig.isSigned", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "active-config.measurement-capabilities" => Some(("activeConfig.measurementCapabilities", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "active-config.model.firmware-version" => Some(("activeConfig.model.firmwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.model.hardware-version" => Some(("activeConfig.model.hardwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.model.name" => Some(("activeConfig.model.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.model.software-version" => Some(("activeConfig.model.softwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.model.vendor" => Some(("activeConfig.model.vendor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.state" => Some(("activeConfig.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.update-time" => Some(("activeConfig.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.user-id" => Some(("activeConfig.userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.antenna-model" => Some(("deviceMetadata.antennaModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.common-channel-group" => Some(("deviceMetadata.commonChannelGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.interference-coordination-group" => Some(("deviceMetadata.interferenceCoordinationGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validated" => Some(("deviceMetadata.nrqzValidated", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.case-id" => Some(("deviceMetadata.nrqzValidation.caseId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.cpi-id" => Some(("deviceMetadata.nrqzValidation.cpiId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.latitude" => Some(("deviceMetadata.nrqzValidation.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.longitude" => Some(("deviceMetadata.nrqzValidation.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.state" => Some(("deviceMetadata.nrqzValidation.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fcc-id" => Some(("fccId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.air-interface.radio-technology" => Some(("preloadedConfig.airInterface.radioTechnology", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.air-interface.supported-spec" => Some(("preloadedConfig.airInterface.supportedSpec", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.call-sign" => Some(("preloadedConfig.callSign", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.category" => Some(("preloadedConfig.category", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-azimuth" => Some(("preloadedConfig.installationParams.antennaAzimuth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-beamwidth" => Some(("preloadedConfig.installationParams.antennaBeamwidth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-downtilt" => Some(("preloadedConfig.installationParams.antennaDowntilt", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-gain" => Some(("preloadedConfig.installationParams.antennaGain", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-model" => Some(("preloadedConfig.installationParams.antennaModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.cpe-cbsd-indication" => Some(("preloadedConfig.installationParams.cpeCbsdIndication", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.eirp-capability" => Some(("preloadedConfig.installationParams.eirpCapability", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.height" => Some(("preloadedConfig.installationParams.height", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.height-type" => Some(("preloadedConfig.installationParams.heightType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.horizontal-accuracy" => Some(("preloadedConfig.installationParams.horizontalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.indoor-deployment" => Some(("preloadedConfig.installationParams.indoorDeployment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.latitude" => Some(("preloadedConfig.installationParams.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.longitude" => Some(("preloadedConfig.installationParams.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.vertical-accuracy" => Some(("preloadedConfig.installationParams.verticalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.is-signed" => Some(("preloadedConfig.isSigned", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "preloaded-config.measurement-capabilities" => Some(("preloadedConfig.measurementCapabilities", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "preloaded-config.model.firmware-version" => Some(("preloadedConfig.model.firmwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.model.hardware-version" => Some(("preloadedConfig.model.hardwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.model.name" => Some(("preloadedConfig.model.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.model.software-version" => Some(("preloadedConfig.model.softwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.model.vendor" => Some(("preloadedConfig.model.vendor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.state" => Some(("preloadedConfig.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.update-time" => Some(("preloadedConfig.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.user-id" => Some(("preloadedConfig.userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "serial-number" => Some(("serialNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["active-config", "air-interface", "antenna-azimuth", "antenna-beamwidth", "antenna-downtilt", "antenna-gain", "antenna-model", "call-sign", "case-id", "category", "common-channel-group", "cpe-cbsd-indication", "cpi-id", "device-metadata", "display-name", "eirp-capability", "fcc-id", "firmware-version", "hardware-version", "height", "height-type", "horizontal-accuracy", "indoor-deployment", "installation-params", "interference-coordination-group", "is-signed", "latitude", "longitude", "measurement-capabilities", "model", "name", "nrqz-validated", "nrqz-validation", "preloaded-config", "radio-technology", "serial-number", "software-version", "state", "supported-spec", "update-time", "user-id", "vendor", "vertical-accuracy"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalDevice = json::value::from_value(object).unwrap();
        let mut call = self.hub.customers().devices_patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
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
                                                                           v.extend(["update-mask"].iter().map(|v|*v));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _customers_devices_sign_device(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "device.active-config.air-interface.radio-technology" => Some(("device.activeConfig.airInterface.radioTechnology", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.active-config.air-interface.supported-spec" => Some(("device.activeConfig.airInterface.supportedSpec", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.active-config.call-sign" => Some(("device.activeConfig.callSign", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.active-config.category" => Some(("device.activeConfig.category", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.antenna-azimuth" => Some(("device.activeConfig.installationParams.antennaAzimuth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.antenna-beamwidth" => Some(("device.activeConfig.installationParams.antennaBeamwidth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.antenna-downtilt" => Some(("device.activeConfig.installationParams.antennaDowntilt", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.antenna-gain" => Some(("device.activeConfig.installationParams.antennaGain", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.antenna-model" => Some(("device.activeConfig.installationParams.antennaModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.cpe-cbsd-indication" => Some(("device.activeConfig.installationParams.cpeCbsdIndication", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.eirp-capability" => Some(("device.activeConfig.installationParams.eirpCapability", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.height" => Some(("device.activeConfig.installationParams.height", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.height-type" => Some(("device.activeConfig.installationParams.heightType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.horizontal-accuracy" => Some(("device.activeConfig.installationParams.horizontalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.indoor-deployment" => Some(("device.activeConfig.installationParams.indoorDeployment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.latitude" => Some(("device.activeConfig.installationParams.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.longitude" => Some(("device.activeConfig.installationParams.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.vertical-accuracy" => Some(("device.activeConfig.installationParams.verticalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.active-config.is-signed" => Some(("device.activeConfig.isSigned", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "device.active-config.measurement-capabilities" => Some(("device.activeConfig.measurementCapabilities", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "device.active-config.model.firmware-version" => Some(("device.activeConfig.model.firmwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.active-config.model.hardware-version" => Some(("device.activeConfig.model.hardwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.active-config.model.name" => Some(("device.activeConfig.model.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.active-config.model.software-version" => Some(("device.activeConfig.model.softwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.active-config.model.vendor" => Some(("device.activeConfig.model.vendor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.active-config.state" => Some(("device.activeConfig.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.active-config.update-time" => Some(("device.activeConfig.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.active-config.user-id" => Some(("device.activeConfig.userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.device-metadata.antenna-model" => Some(("device.deviceMetadata.antennaModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.device-metadata.common-channel-group" => Some(("device.deviceMetadata.commonChannelGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.device-metadata.interference-coordination-group" => Some(("device.deviceMetadata.interferenceCoordinationGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.device-metadata.nrqz-validated" => Some(("device.deviceMetadata.nrqzValidated", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "device.device-metadata.nrqz-validation.case-id" => Some(("device.deviceMetadata.nrqzValidation.caseId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.device-metadata.nrqz-validation.cpi-id" => Some(("device.deviceMetadata.nrqzValidation.cpiId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.device-metadata.nrqz-validation.latitude" => Some(("device.deviceMetadata.nrqzValidation.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.device-metadata.nrqz-validation.longitude" => Some(("device.deviceMetadata.nrqzValidation.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.device-metadata.nrqz-validation.state" => Some(("device.deviceMetadata.nrqzValidation.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.display-name" => Some(("device.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.fcc-id" => Some(("device.fccId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.name" => Some(("device.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.air-interface.radio-technology" => Some(("device.preloadedConfig.airInterface.radioTechnology", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.air-interface.supported-spec" => Some(("device.preloadedConfig.airInterface.supportedSpec", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.call-sign" => Some(("device.preloadedConfig.callSign", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.category" => Some(("device.preloadedConfig.category", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.antenna-azimuth" => Some(("device.preloadedConfig.installationParams.antennaAzimuth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.antenna-beamwidth" => Some(("device.preloadedConfig.installationParams.antennaBeamwidth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.antenna-downtilt" => Some(("device.preloadedConfig.installationParams.antennaDowntilt", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.antenna-gain" => Some(("device.preloadedConfig.installationParams.antennaGain", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.antenna-model" => Some(("device.preloadedConfig.installationParams.antennaModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.cpe-cbsd-indication" => Some(("device.preloadedConfig.installationParams.cpeCbsdIndication", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.eirp-capability" => Some(("device.preloadedConfig.installationParams.eirpCapability", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.height" => Some(("device.preloadedConfig.installationParams.height", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.height-type" => Some(("device.preloadedConfig.installationParams.heightType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.horizontal-accuracy" => Some(("device.preloadedConfig.installationParams.horizontalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.indoor-deployment" => Some(("device.preloadedConfig.installationParams.indoorDeployment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.latitude" => Some(("device.preloadedConfig.installationParams.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.longitude" => Some(("device.preloadedConfig.installationParams.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.vertical-accuracy" => Some(("device.preloadedConfig.installationParams.verticalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.preloaded-config.is-signed" => Some(("device.preloadedConfig.isSigned", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "device.preloaded-config.measurement-capabilities" => Some(("device.preloadedConfig.measurementCapabilities", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "device.preloaded-config.model.firmware-version" => Some(("device.preloadedConfig.model.firmwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.model.hardware-version" => Some(("device.preloadedConfig.model.hardwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.model.name" => Some(("device.preloadedConfig.model.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.model.software-version" => Some(("device.preloadedConfig.model.softwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.model.vendor" => Some(("device.preloadedConfig.model.vendor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.state" => Some(("device.preloadedConfig.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.update-time" => Some(("device.preloadedConfig.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.user-id" => Some(("device.preloadedConfig.userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.serial-number" => Some(("device.serialNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.state" => Some(("device.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["active-config", "air-interface", "antenna-azimuth", "antenna-beamwidth", "antenna-downtilt", "antenna-gain", "antenna-model", "call-sign", "case-id", "category", "common-channel-group", "cpe-cbsd-indication", "cpi-id", "device", "device-metadata", "display-name", "eirp-capability", "fcc-id", "firmware-version", "hardware-version", "height", "height-type", "horizontal-accuracy", "indoor-deployment", "installation-params", "interference-coordination-group", "is-signed", "latitude", "longitude", "measurement-capabilities", "model", "name", "nrqz-validated", "nrqz-validation", "preloaded-config", "radio-technology", "serial-number", "software-version", "state", "supported-spec", "update-time", "user-id", "vendor", "vertical-accuracy"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalSignDeviceRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.customers().devices_sign_device(request, opt.value_of("name").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _customers_devices_update_signed(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "encoded-device" => Some(("encodedDevice", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "installer-id" => Some(("installerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["encoded-device", "installer-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalUpdateSignedDeviceRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.customers().devices_update_signed(request, opt.value_of("name").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _customers_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.customers().get(opt.value_of("name").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _customers_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.customers().list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["page-size", "page-token"].iter().map(|v|*v));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _customers_list_gcp_project_deployments(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.customers().list_gcp_project_deployments();
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _customers_list_legacy_organizations(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.customers().list_legacy_organizations();
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _customers_migrate_organization(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "organization-id" => Some(("organizationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["organization-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalMigrateOrganizationRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.customers().migrate_organization(request);
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _customers_nodes_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sas-user-ids" => Some(("sasUserIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["display-name", "name", "sas-user-ids"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalNode = json::value::from_value(object).unwrap();
        let mut call = self.hub.customers().nodes_create(request, opt.value_of("parent").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _customers_nodes_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.customers().nodes_delete(opt.value_of("name").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _customers_nodes_deployments_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "frns" => Some(("frns", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sas-user-ids" => Some(("sasUserIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["display-name", "frns", "name", "sas-user-ids"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalDeployment = json::value::from_value(object).unwrap();
        let mut call = self.hub.customers().nodes_deployments_create(request, opt.value_of("parent").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _customers_nodes_deployments_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.customers().nodes_deployments_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
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
                                                                           v.extend(["filter", "page-size", "page-token"].iter().map(|v|*v));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _customers_nodes_devices_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "active-config.air-interface.radio-technology" => Some(("activeConfig.airInterface.radioTechnology", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.air-interface.supported-spec" => Some(("activeConfig.airInterface.supportedSpec", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.call-sign" => Some(("activeConfig.callSign", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.category" => Some(("activeConfig.category", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-azimuth" => Some(("activeConfig.installationParams.antennaAzimuth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-beamwidth" => Some(("activeConfig.installationParams.antennaBeamwidth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-downtilt" => Some(("activeConfig.installationParams.antennaDowntilt", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-gain" => Some(("activeConfig.installationParams.antennaGain", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-model" => Some(("activeConfig.installationParams.antennaModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.installation-params.cpe-cbsd-indication" => Some(("activeConfig.installationParams.cpeCbsdIndication", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "active-config.installation-params.eirp-capability" => Some(("activeConfig.installationParams.eirpCapability", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "active-config.installation-params.height" => Some(("activeConfig.installationParams.height", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.height-type" => Some(("activeConfig.installationParams.heightType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.installation-params.horizontal-accuracy" => Some(("activeConfig.installationParams.horizontalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.indoor-deployment" => Some(("activeConfig.installationParams.indoorDeployment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "active-config.installation-params.latitude" => Some(("activeConfig.installationParams.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.longitude" => Some(("activeConfig.installationParams.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.vertical-accuracy" => Some(("activeConfig.installationParams.verticalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.is-signed" => Some(("activeConfig.isSigned", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "active-config.measurement-capabilities" => Some(("activeConfig.measurementCapabilities", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "active-config.model.firmware-version" => Some(("activeConfig.model.firmwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.model.hardware-version" => Some(("activeConfig.model.hardwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.model.name" => Some(("activeConfig.model.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.model.software-version" => Some(("activeConfig.model.softwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.model.vendor" => Some(("activeConfig.model.vendor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.state" => Some(("activeConfig.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.update-time" => Some(("activeConfig.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.user-id" => Some(("activeConfig.userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.antenna-model" => Some(("deviceMetadata.antennaModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.common-channel-group" => Some(("deviceMetadata.commonChannelGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.interference-coordination-group" => Some(("deviceMetadata.interferenceCoordinationGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validated" => Some(("deviceMetadata.nrqzValidated", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.case-id" => Some(("deviceMetadata.nrqzValidation.caseId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.cpi-id" => Some(("deviceMetadata.nrqzValidation.cpiId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.latitude" => Some(("deviceMetadata.nrqzValidation.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.longitude" => Some(("deviceMetadata.nrqzValidation.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.state" => Some(("deviceMetadata.nrqzValidation.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fcc-id" => Some(("fccId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.air-interface.radio-technology" => Some(("preloadedConfig.airInterface.radioTechnology", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.air-interface.supported-spec" => Some(("preloadedConfig.airInterface.supportedSpec", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.call-sign" => Some(("preloadedConfig.callSign", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.category" => Some(("preloadedConfig.category", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-azimuth" => Some(("preloadedConfig.installationParams.antennaAzimuth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-beamwidth" => Some(("preloadedConfig.installationParams.antennaBeamwidth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-downtilt" => Some(("preloadedConfig.installationParams.antennaDowntilt", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-gain" => Some(("preloadedConfig.installationParams.antennaGain", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-model" => Some(("preloadedConfig.installationParams.antennaModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.cpe-cbsd-indication" => Some(("preloadedConfig.installationParams.cpeCbsdIndication", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.eirp-capability" => Some(("preloadedConfig.installationParams.eirpCapability", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.height" => Some(("preloadedConfig.installationParams.height", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.height-type" => Some(("preloadedConfig.installationParams.heightType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.horizontal-accuracy" => Some(("preloadedConfig.installationParams.horizontalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.indoor-deployment" => Some(("preloadedConfig.installationParams.indoorDeployment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.latitude" => Some(("preloadedConfig.installationParams.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.longitude" => Some(("preloadedConfig.installationParams.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.vertical-accuracy" => Some(("preloadedConfig.installationParams.verticalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.is-signed" => Some(("preloadedConfig.isSigned", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "preloaded-config.measurement-capabilities" => Some(("preloadedConfig.measurementCapabilities", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "preloaded-config.model.firmware-version" => Some(("preloadedConfig.model.firmwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.model.hardware-version" => Some(("preloadedConfig.model.hardwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.model.name" => Some(("preloadedConfig.model.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.model.software-version" => Some(("preloadedConfig.model.softwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.model.vendor" => Some(("preloadedConfig.model.vendor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.state" => Some(("preloadedConfig.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.update-time" => Some(("preloadedConfig.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.user-id" => Some(("preloadedConfig.userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "serial-number" => Some(("serialNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["active-config", "air-interface", "antenna-azimuth", "antenna-beamwidth", "antenna-downtilt", "antenna-gain", "antenna-model", "call-sign", "case-id", "category", "common-channel-group", "cpe-cbsd-indication", "cpi-id", "device-metadata", "display-name", "eirp-capability", "fcc-id", "firmware-version", "hardware-version", "height", "height-type", "horizontal-accuracy", "indoor-deployment", "installation-params", "interference-coordination-group", "is-signed", "latitude", "longitude", "measurement-capabilities", "model", "name", "nrqz-validated", "nrqz-validation", "preloaded-config", "radio-technology", "serial-number", "software-version", "state", "supported-spec", "update-time", "user-id", "vendor", "vertical-accuracy"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalDevice = json::value::from_value(object).unwrap();
        let mut call = self.hub.customers().nodes_devices_create(request, opt.value_of("parent").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _customers_nodes_devices_create_signed(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "encoded-device" => Some(("encodedDevice", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "installer-id" => Some(("installerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["encoded-device", "installer-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalCreateSignedDeviceRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.customers().nodes_devices_create_signed(request, opt.value_of("parent").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _customers_nodes_devices_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.customers().nodes_devices_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
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
                                                                           v.extend(["filter", "page-size", "page-token"].iter().map(|v|*v));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _customers_nodes_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.customers().nodes_get(opt.value_of("name").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _customers_nodes_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.customers().nodes_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
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
                                                                           v.extend(["filter", "page-size", "page-token"].iter().map(|v|*v));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _customers_nodes_move(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "destination" => Some(("destination", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["destination"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalMoveNodeRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.customers().nodes_move(request, opt.value_of("name").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _customers_nodes_nodes_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sas-user-ids" => Some(("sasUserIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["display-name", "name", "sas-user-ids"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalNode = json::value::from_value(object).unwrap();
        let mut call = self.hub.customers().nodes_nodes_create(request, opt.value_of("parent").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _customers_nodes_nodes_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.customers().nodes_nodes_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
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
                                                                           v.extend(["filter", "page-size", "page-token"].iter().map(|v|*v));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _customers_nodes_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sas-user-ids" => Some(("sasUserIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["display-name", "name", "sas-user-ids"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalNode = json::value::from_value(object).unwrap();
        let mut call = self.hub.customers().nodes_patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
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
                                                                           v.extend(["update-mask"].iter().map(|v|*v));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _customers_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sas-user-ids" => Some(("sasUserIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["display-name", "name", "sas-user-ids"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalCustomer = json::value::from_value(object).unwrap();
        let mut call = self.hub.customers().patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
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
                                                                           v.extend(["update-mask"].iter().map(|v|*v));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _customers_provision_deployment(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "new-deployment-display-name" => Some(("newDeploymentDisplayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "new-organization-display-name" => Some(("newOrganizationDisplayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "organization-id" => Some(("organizationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["new-deployment-display-name", "new-organization-display-name", "organization-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalProvisionDeploymentRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.customers().provision_deployment(request);
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _customers_setup_sas_analytics(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "user-id" => Some(("userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["user-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalSetupSasAnalyticsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.customers().setup_sas_analytics(request);
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _deployments_devices_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.deployments().devices_delete(opt.value_of("name").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _deployments_devices_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.deployments().devices_get(opt.value_of("name").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _deployments_devices_move(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "destination" => Some(("destination", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["destination"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalMoveDeviceRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.deployments().devices_move(request, opt.value_of("name").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _deployments_devices_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "active-config.air-interface.radio-technology" => Some(("activeConfig.airInterface.radioTechnology", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.air-interface.supported-spec" => Some(("activeConfig.airInterface.supportedSpec", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.call-sign" => Some(("activeConfig.callSign", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.category" => Some(("activeConfig.category", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-azimuth" => Some(("activeConfig.installationParams.antennaAzimuth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-beamwidth" => Some(("activeConfig.installationParams.antennaBeamwidth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-downtilt" => Some(("activeConfig.installationParams.antennaDowntilt", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-gain" => Some(("activeConfig.installationParams.antennaGain", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-model" => Some(("activeConfig.installationParams.antennaModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.installation-params.cpe-cbsd-indication" => Some(("activeConfig.installationParams.cpeCbsdIndication", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "active-config.installation-params.eirp-capability" => Some(("activeConfig.installationParams.eirpCapability", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "active-config.installation-params.height" => Some(("activeConfig.installationParams.height", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.height-type" => Some(("activeConfig.installationParams.heightType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.installation-params.horizontal-accuracy" => Some(("activeConfig.installationParams.horizontalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.indoor-deployment" => Some(("activeConfig.installationParams.indoorDeployment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "active-config.installation-params.latitude" => Some(("activeConfig.installationParams.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.longitude" => Some(("activeConfig.installationParams.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.vertical-accuracy" => Some(("activeConfig.installationParams.verticalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.is-signed" => Some(("activeConfig.isSigned", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "active-config.measurement-capabilities" => Some(("activeConfig.measurementCapabilities", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "active-config.model.firmware-version" => Some(("activeConfig.model.firmwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.model.hardware-version" => Some(("activeConfig.model.hardwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.model.name" => Some(("activeConfig.model.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.model.software-version" => Some(("activeConfig.model.softwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.model.vendor" => Some(("activeConfig.model.vendor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.state" => Some(("activeConfig.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.update-time" => Some(("activeConfig.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.user-id" => Some(("activeConfig.userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.antenna-model" => Some(("deviceMetadata.antennaModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.common-channel-group" => Some(("deviceMetadata.commonChannelGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.interference-coordination-group" => Some(("deviceMetadata.interferenceCoordinationGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validated" => Some(("deviceMetadata.nrqzValidated", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.case-id" => Some(("deviceMetadata.nrqzValidation.caseId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.cpi-id" => Some(("deviceMetadata.nrqzValidation.cpiId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.latitude" => Some(("deviceMetadata.nrqzValidation.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.longitude" => Some(("deviceMetadata.nrqzValidation.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.state" => Some(("deviceMetadata.nrqzValidation.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fcc-id" => Some(("fccId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.air-interface.radio-technology" => Some(("preloadedConfig.airInterface.radioTechnology", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.air-interface.supported-spec" => Some(("preloadedConfig.airInterface.supportedSpec", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.call-sign" => Some(("preloadedConfig.callSign", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.category" => Some(("preloadedConfig.category", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-azimuth" => Some(("preloadedConfig.installationParams.antennaAzimuth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-beamwidth" => Some(("preloadedConfig.installationParams.antennaBeamwidth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-downtilt" => Some(("preloadedConfig.installationParams.antennaDowntilt", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-gain" => Some(("preloadedConfig.installationParams.antennaGain", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-model" => Some(("preloadedConfig.installationParams.antennaModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.cpe-cbsd-indication" => Some(("preloadedConfig.installationParams.cpeCbsdIndication", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.eirp-capability" => Some(("preloadedConfig.installationParams.eirpCapability", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.height" => Some(("preloadedConfig.installationParams.height", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.height-type" => Some(("preloadedConfig.installationParams.heightType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.horizontal-accuracy" => Some(("preloadedConfig.installationParams.horizontalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.indoor-deployment" => Some(("preloadedConfig.installationParams.indoorDeployment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.latitude" => Some(("preloadedConfig.installationParams.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.longitude" => Some(("preloadedConfig.installationParams.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.vertical-accuracy" => Some(("preloadedConfig.installationParams.verticalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.is-signed" => Some(("preloadedConfig.isSigned", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "preloaded-config.measurement-capabilities" => Some(("preloadedConfig.measurementCapabilities", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "preloaded-config.model.firmware-version" => Some(("preloadedConfig.model.firmwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.model.hardware-version" => Some(("preloadedConfig.model.hardwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.model.name" => Some(("preloadedConfig.model.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.model.software-version" => Some(("preloadedConfig.model.softwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.model.vendor" => Some(("preloadedConfig.model.vendor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.state" => Some(("preloadedConfig.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.update-time" => Some(("preloadedConfig.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.user-id" => Some(("preloadedConfig.userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "serial-number" => Some(("serialNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["active-config", "air-interface", "antenna-azimuth", "antenna-beamwidth", "antenna-downtilt", "antenna-gain", "antenna-model", "call-sign", "case-id", "category", "common-channel-group", "cpe-cbsd-indication", "cpi-id", "device-metadata", "display-name", "eirp-capability", "fcc-id", "firmware-version", "hardware-version", "height", "height-type", "horizontal-accuracy", "indoor-deployment", "installation-params", "interference-coordination-group", "is-signed", "latitude", "longitude", "measurement-capabilities", "model", "name", "nrqz-validated", "nrqz-validation", "preloaded-config", "radio-technology", "serial-number", "software-version", "state", "supported-spec", "update-time", "user-id", "vendor", "vertical-accuracy"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalDevice = json::value::from_value(object).unwrap();
        let mut call = self.hub.deployments().devices_patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
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
                                                                           v.extend(["update-mask"].iter().map(|v|*v));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _deployments_devices_sign_device(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "device.active-config.air-interface.radio-technology" => Some(("device.activeConfig.airInterface.radioTechnology", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.active-config.air-interface.supported-spec" => Some(("device.activeConfig.airInterface.supportedSpec", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.active-config.call-sign" => Some(("device.activeConfig.callSign", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.active-config.category" => Some(("device.activeConfig.category", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.antenna-azimuth" => Some(("device.activeConfig.installationParams.antennaAzimuth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.antenna-beamwidth" => Some(("device.activeConfig.installationParams.antennaBeamwidth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.antenna-downtilt" => Some(("device.activeConfig.installationParams.antennaDowntilt", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.antenna-gain" => Some(("device.activeConfig.installationParams.antennaGain", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.antenna-model" => Some(("device.activeConfig.installationParams.antennaModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.cpe-cbsd-indication" => Some(("device.activeConfig.installationParams.cpeCbsdIndication", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.eirp-capability" => Some(("device.activeConfig.installationParams.eirpCapability", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.height" => Some(("device.activeConfig.installationParams.height", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.height-type" => Some(("device.activeConfig.installationParams.heightType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.horizontal-accuracy" => Some(("device.activeConfig.installationParams.horizontalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.indoor-deployment" => Some(("device.activeConfig.installationParams.indoorDeployment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.latitude" => Some(("device.activeConfig.installationParams.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.longitude" => Some(("device.activeConfig.installationParams.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.vertical-accuracy" => Some(("device.activeConfig.installationParams.verticalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.active-config.is-signed" => Some(("device.activeConfig.isSigned", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "device.active-config.measurement-capabilities" => Some(("device.activeConfig.measurementCapabilities", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "device.active-config.model.firmware-version" => Some(("device.activeConfig.model.firmwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.active-config.model.hardware-version" => Some(("device.activeConfig.model.hardwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.active-config.model.name" => Some(("device.activeConfig.model.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.active-config.model.software-version" => Some(("device.activeConfig.model.softwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.active-config.model.vendor" => Some(("device.activeConfig.model.vendor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.active-config.state" => Some(("device.activeConfig.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.active-config.update-time" => Some(("device.activeConfig.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.active-config.user-id" => Some(("device.activeConfig.userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.device-metadata.antenna-model" => Some(("device.deviceMetadata.antennaModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.device-metadata.common-channel-group" => Some(("device.deviceMetadata.commonChannelGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.device-metadata.interference-coordination-group" => Some(("device.deviceMetadata.interferenceCoordinationGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.device-metadata.nrqz-validated" => Some(("device.deviceMetadata.nrqzValidated", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "device.device-metadata.nrqz-validation.case-id" => Some(("device.deviceMetadata.nrqzValidation.caseId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.device-metadata.nrqz-validation.cpi-id" => Some(("device.deviceMetadata.nrqzValidation.cpiId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.device-metadata.nrqz-validation.latitude" => Some(("device.deviceMetadata.nrqzValidation.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.device-metadata.nrqz-validation.longitude" => Some(("device.deviceMetadata.nrqzValidation.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.device-metadata.nrqz-validation.state" => Some(("device.deviceMetadata.nrqzValidation.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.display-name" => Some(("device.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.fcc-id" => Some(("device.fccId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.name" => Some(("device.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.air-interface.radio-technology" => Some(("device.preloadedConfig.airInterface.radioTechnology", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.air-interface.supported-spec" => Some(("device.preloadedConfig.airInterface.supportedSpec", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.call-sign" => Some(("device.preloadedConfig.callSign", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.category" => Some(("device.preloadedConfig.category", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.antenna-azimuth" => Some(("device.preloadedConfig.installationParams.antennaAzimuth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.antenna-beamwidth" => Some(("device.preloadedConfig.installationParams.antennaBeamwidth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.antenna-downtilt" => Some(("device.preloadedConfig.installationParams.antennaDowntilt", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.antenna-gain" => Some(("device.preloadedConfig.installationParams.antennaGain", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.antenna-model" => Some(("device.preloadedConfig.installationParams.antennaModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.cpe-cbsd-indication" => Some(("device.preloadedConfig.installationParams.cpeCbsdIndication", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.eirp-capability" => Some(("device.preloadedConfig.installationParams.eirpCapability", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.height" => Some(("device.preloadedConfig.installationParams.height", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.height-type" => Some(("device.preloadedConfig.installationParams.heightType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.horizontal-accuracy" => Some(("device.preloadedConfig.installationParams.horizontalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.indoor-deployment" => Some(("device.preloadedConfig.installationParams.indoorDeployment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.latitude" => Some(("device.preloadedConfig.installationParams.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.longitude" => Some(("device.preloadedConfig.installationParams.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.vertical-accuracy" => Some(("device.preloadedConfig.installationParams.verticalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.preloaded-config.is-signed" => Some(("device.preloadedConfig.isSigned", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "device.preloaded-config.measurement-capabilities" => Some(("device.preloadedConfig.measurementCapabilities", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "device.preloaded-config.model.firmware-version" => Some(("device.preloadedConfig.model.firmwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.model.hardware-version" => Some(("device.preloadedConfig.model.hardwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.model.name" => Some(("device.preloadedConfig.model.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.model.software-version" => Some(("device.preloadedConfig.model.softwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.model.vendor" => Some(("device.preloadedConfig.model.vendor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.state" => Some(("device.preloadedConfig.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.update-time" => Some(("device.preloadedConfig.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.user-id" => Some(("device.preloadedConfig.userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.serial-number" => Some(("device.serialNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.state" => Some(("device.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["active-config", "air-interface", "antenna-azimuth", "antenna-beamwidth", "antenna-downtilt", "antenna-gain", "antenna-model", "call-sign", "case-id", "category", "common-channel-group", "cpe-cbsd-indication", "cpi-id", "device", "device-metadata", "display-name", "eirp-capability", "fcc-id", "firmware-version", "hardware-version", "height", "height-type", "horizontal-accuracy", "indoor-deployment", "installation-params", "interference-coordination-group", "is-signed", "latitude", "longitude", "measurement-capabilities", "model", "name", "nrqz-validated", "nrqz-validation", "preloaded-config", "radio-technology", "serial-number", "software-version", "state", "supported-spec", "update-time", "user-id", "vendor", "vertical-accuracy"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalSignDeviceRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.deployments().devices_sign_device(request, opt.value_of("name").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _deployments_devices_update_signed(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "encoded-device" => Some(("encodedDevice", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "installer-id" => Some(("installerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["encoded-device", "installer-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalUpdateSignedDeviceRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.deployments().devices_update_signed(request, opt.value_of("name").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _deployments_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.deployments().get(opt.value_of("name").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _installer_generate_secret(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec![]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalGenerateSecretRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.installer().generate_secret(request);
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _installer_validate(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "encoded-secret" => Some(("encodedSecret", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "installer-id" => Some(("installerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "secret" => Some(("secret", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["encoded-secret", "installer-id", "secret"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalValidateInstallerRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.installer().validate(request);
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _nodes_deployments_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.nodes().deployments_delete(opt.value_of("name").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _nodes_deployments_devices_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "active-config.air-interface.radio-technology" => Some(("activeConfig.airInterface.radioTechnology", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.air-interface.supported-spec" => Some(("activeConfig.airInterface.supportedSpec", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.call-sign" => Some(("activeConfig.callSign", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.category" => Some(("activeConfig.category", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-azimuth" => Some(("activeConfig.installationParams.antennaAzimuth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-beamwidth" => Some(("activeConfig.installationParams.antennaBeamwidth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-downtilt" => Some(("activeConfig.installationParams.antennaDowntilt", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-gain" => Some(("activeConfig.installationParams.antennaGain", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-model" => Some(("activeConfig.installationParams.antennaModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.installation-params.cpe-cbsd-indication" => Some(("activeConfig.installationParams.cpeCbsdIndication", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "active-config.installation-params.eirp-capability" => Some(("activeConfig.installationParams.eirpCapability", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "active-config.installation-params.height" => Some(("activeConfig.installationParams.height", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.height-type" => Some(("activeConfig.installationParams.heightType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.installation-params.horizontal-accuracy" => Some(("activeConfig.installationParams.horizontalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.indoor-deployment" => Some(("activeConfig.installationParams.indoorDeployment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "active-config.installation-params.latitude" => Some(("activeConfig.installationParams.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.longitude" => Some(("activeConfig.installationParams.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.vertical-accuracy" => Some(("activeConfig.installationParams.verticalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.is-signed" => Some(("activeConfig.isSigned", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "active-config.measurement-capabilities" => Some(("activeConfig.measurementCapabilities", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "active-config.model.firmware-version" => Some(("activeConfig.model.firmwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.model.hardware-version" => Some(("activeConfig.model.hardwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.model.name" => Some(("activeConfig.model.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.model.software-version" => Some(("activeConfig.model.softwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.model.vendor" => Some(("activeConfig.model.vendor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.state" => Some(("activeConfig.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.update-time" => Some(("activeConfig.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.user-id" => Some(("activeConfig.userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.antenna-model" => Some(("deviceMetadata.antennaModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.common-channel-group" => Some(("deviceMetadata.commonChannelGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.interference-coordination-group" => Some(("deviceMetadata.interferenceCoordinationGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validated" => Some(("deviceMetadata.nrqzValidated", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.case-id" => Some(("deviceMetadata.nrqzValidation.caseId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.cpi-id" => Some(("deviceMetadata.nrqzValidation.cpiId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.latitude" => Some(("deviceMetadata.nrqzValidation.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.longitude" => Some(("deviceMetadata.nrqzValidation.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.state" => Some(("deviceMetadata.nrqzValidation.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fcc-id" => Some(("fccId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.air-interface.radio-technology" => Some(("preloadedConfig.airInterface.radioTechnology", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.air-interface.supported-spec" => Some(("preloadedConfig.airInterface.supportedSpec", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.call-sign" => Some(("preloadedConfig.callSign", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.category" => Some(("preloadedConfig.category", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-azimuth" => Some(("preloadedConfig.installationParams.antennaAzimuth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-beamwidth" => Some(("preloadedConfig.installationParams.antennaBeamwidth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-downtilt" => Some(("preloadedConfig.installationParams.antennaDowntilt", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-gain" => Some(("preloadedConfig.installationParams.antennaGain", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-model" => Some(("preloadedConfig.installationParams.antennaModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.cpe-cbsd-indication" => Some(("preloadedConfig.installationParams.cpeCbsdIndication", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.eirp-capability" => Some(("preloadedConfig.installationParams.eirpCapability", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.height" => Some(("preloadedConfig.installationParams.height", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.height-type" => Some(("preloadedConfig.installationParams.heightType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.horizontal-accuracy" => Some(("preloadedConfig.installationParams.horizontalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.indoor-deployment" => Some(("preloadedConfig.installationParams.indoorDeployment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.latitude" => Some(("preloadedConfig.installationParams.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.longitude" => Some(("preloadedConfig.installationParams.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.vertical-accuracy" => Some(("preloadedConfig.installationParams.verticalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.is-signed" => Some(("preloadedConfig.isSigned", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "preloaded-config.measurement-capabilities" => Some(("preloadedConfig.measurementCapabilities", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "preloaded-config.model.firmware-version" => Some(("preloadedConfig.model.firmwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.model.hardware-version" => Some(("preloadedConfig.model.hardwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.model.name" => Some(("preloadedConfig.model.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.model.software-version" => Some(("preloadedConfig.model.softwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.model.vendor" => Some(("preloadedConfig.model.vendor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.state" => Some(("preloadedConfig.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.update-time" => Some(("preloadedConfig.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.user-id" => Some(("preloadedConfig.userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "serial-number" => Some(("serialNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["active-config", "air-interface", "antenna-azimuth", "antenna-beamwidth", "antenna-downtilt", "antenna-gain", "antenna-model", "call-sign", "case-id", "category", "common-channel-group", "cpe-cbsd-indication", "cpi-id", "device-metadata", "display-name", "eirp-capability", "fcc-id", "firmware-version", "hardware-version", "height", "height-type", "horizontal-accuracy", "indoor-deployment", "installation-params", "interference-coordination-group", "is-signed", "latitude", "longitude", "measurement-capabilities", "model", "name", "nrqz-validated", "nrqz-validation", "preloaded-config", "radio-technology", "serial-number", "software-version", "state", "supported-spec", "update-time", "user-id", "vendor", "vertical-accuracy"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalDevice = json::value::from_value(object).unwrap();
        let mut call = self.hub.nodes().deployments_devices_create(request, opt.value_of("parent").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _nodes_deployments_devices_create_signed(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "encoded-device" => Some(("encodedDevice", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "installer-id" => Some(("installerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["encoded-device", "installer-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalCreateSignedDeviceRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.nodes().deployments_devices_create_signed(request, opt.value_of("parent").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _nodes_deployments_devices_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.nodes().deployments_devices_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
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
                                                                           v.extend(["filter", "page-size", "page-token"].iter().map(|v|*v));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _nodes_deployments_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.nodes().deployments_get(opt.value_of("name").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _nodes_deployments_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.nodes().deployments_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
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
                                                                           v.extend(["filter", "page-size", "page-token"].iter().map(|v|*v));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _nodes_deployments_move(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "destination" => Some(("destination", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["destination"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalMoveDeploymentRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.nodes().deployments_move(request, opt.value_of("name").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _nodes_deployments_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "frns" => Some(("frns", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sas-user-ids" => Some(("sasUserIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["display-name", "frns", "name", "sas-user-ids"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalDeployment = json::value::from_value(object).unwrap();
        let mut call = self.hub.nodes().deployments_patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
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
                                                                           v.extend(["update-mask"].iter().map(|v|*v));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _nodes_devices_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "active-config.air-interface.radio-technology" => Some(("activeConfig.airInterface.radioTechnology", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.air-interface.supported-spec" => Some(("activeConfig.airInterface.supportedSpec", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.call-sign" => Some(("activeConfig.callSign", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.category" => Some(("activeConfig.category", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-azimuth" => Some(("activeConfig.installationParams.antennaAzimuth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-beamwidth" => Some(("activeConfig.installationParams.antennaBeamwidth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-downtilt" => Some(("activeConfig.installationParams.antennaDowntilt", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-gain" => Some(("activeConfig.installationParams.antennaGain", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-model" => Some(("activeConfig.installationParams.antennaModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.installation-params.cpe-cbsd-indication" => Some(("activeConfig.installationParams.cpeCbsdIndication", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "active-config.installation-params.eirp-capability" => Some(("activeConfig.installationParams.eirpCapability", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "active-config.installation-params.height" => Some(("activeConfig.installationParams.height", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.height-type" => Some(("activeConfig.installationParams.heightType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.installation-params.horizontal-accuracy" => Some(("activeConfig.installationParams.horizontalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.indoor-deployment" => Some(("activeConfig.installationParams.indoorDeployment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "active-config.installation-params.latitude" => Some(("activeConfig.installationParams.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.longitude" => Some(("activeConfig.installationParams.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.vertical-accuracy" => Some(("activeConfig.installationParams.verticalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.is-signed" => Some(("activeConfig.isSigned", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "active-config.measurement-capabilities" => Some(("activeConfig.measurementCapabilities", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "active-config.model.firmware-version" => Some(("activeConfig.model.firmwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.model.hardware-version" => Some(("activeConfig.model.hardwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.model.name" => Some(("activeConfig.model.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.model.software-version" => Some(("activeConfig.model.softwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.model.vendor" => Some(("activeConfig.model.vendor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.state" => Some(("activeConfig.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.update-time" => Some(("activeConfig.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.user-id" => Some(("activeConfig.userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.antenna-model" => Some(("deviceMetadata.antennaModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.common-channel-group" => Some(("deviceMetadata.commonChannelGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.interference-coordination-group" => Some(("deviceMetadata.interferenceCoordinationGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validated" => Some(("deviceMetadata.nrqzValidated", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.case-id" => Some(("deviceMetadata.nrqzValidation.caseId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.cpi-id" => Some(("deviceMetadata.nrqzValidation.cpiId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.latitude" => Some(("deviceMetadata.nrqzValidation.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.longitude" => Some(("deviceMetadata.nrqzValidation.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.state" => Some(("deviceMetadata.nrqzValidation.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fcc-id" => Some(("fccId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.air-interface.radio-technology" => Some(("preloadedConfig.airInterface.radioTechnology", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.air-interface.supported-spec" => Some(("preloadedConfig.airInterface.supportedSpec", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.call-sign" => Some(("preloadedConfig.callSign", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.category" => Some(("preloadedConfig.category", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-azimuth" => Some(("preloadedConfig.installationParams.antennaAzimuth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-beamwidth" => Some(("preloadedConfig.installationParams.antennaBeamwidth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-downtilt" => Some(("preloadedConfig.installationParams.antennaDowntilt", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-gain" => Some(("preloadedConfig.installationParams.antennaGain", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-model" => Some(("preloadedConfig.installationParams.antennaModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.cpe-cbsd-indication" => Some(("preloadedConfig.installationParams.cpeCbsdIndication", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.eirp-capability" => Some(("preloadedConfig.installationParams.eirpCapability", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.height" => Some(("preloadedConfig.installationParams.height", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.height-type" => Some(("preloadedConfig.installationParams.heightType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.horizontal-accuracy" => Some(("preloadedConfig.installationParams.horizontalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.indoor-deployment" => Some(("preloadedConfig.installationParams.indoorDeployment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.latitude" => Some(("preloadedConfig.installationParams.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.longitude" => Some(("preloadedConfig.installationParams.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.vertical-accuracy" => Some(("preloadedConfig.installationParams.verticalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.is-signed" => Some(("preloadedConfig.isSigned", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "preloaded-config.measurement-capabilities" => Some(("preloadedConfig.measurementCapabilities", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "preloaded-config.model.firmware-version" => Some(("preloadedConfig.model.firmwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.model.hardware-version" => Some(("preloadedConfig.model.hardwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.model.name" => Some(("preloadedConfig.model.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.model.software-version" => Some(("preloadedConfig.model.softwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.model.vendor" => Some(("preloadedConfig.model.vendor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.state" => Some(("preloadedConfig.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.update-time" => Some(("preloadedConfig.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.user-id" => Some(("preloadedConfig.userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "serial-number" => Some(("serialNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["active-config", "air-interface", "antenna-azimuth", "antenna-beamwidth", "antenna-downtilt", "antenna-gain", "antenna-model", "call-sign", "case-id", "category", "common-channel-group", "cpe-cbsd-indication", "cpi-id", "device-metadata", "display-name", "eirp-capability", "fcc-id", "firmware-version", "hardware-version", "height", "height-type", "horizontal-accuracy", "indoor-deployment", "installation-params", "interference-coordination-group", "is-signed", "latitude", "longitude", "measurement-capabilities", "model", "name", "nrqz-validated", "nrqz-validation", "preloaded-config", "radio-technology", "serial-number", "software-version", "state", "supported-spec", "update-time", "user-id", "vendor", "vertical-accuracy"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalDevice = json::value::from_value(object).unwrap();
        let mut call = self.hub.nodes().devices_create(request, opt.value_of("parent").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _nodes_devices_create_signed(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "encoded-device" => Some(("encodedDevice", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "installer-id" => Some(("installerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["encoded-device", "installer-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalCreateSignedDeviceRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.nodes().devices_create_signed(request, opt.value_of("parent").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _nodes_devices_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.nodes().devices_delete(opt.value_of("name").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _nodes_devices_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.nodes().devices_get(opt.value_of("name").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _nodes_devices_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.nodes().devices_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
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
                                                                           v.extend(["filter", "page-size", "page-token"].iter().map(|v|*v));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _nodes_devices_move(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "destination" => Some(("destination", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["destination"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalMoveDeviceRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.nodes().devices_move(request, opt.value_of("name").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _nodes_devices_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "active-config.air-interface.radio-technology" => Some(("activeConfig.airInterface.radioTechnology", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.air-interface.supported-spec" => Some(("activeConfig.airInterface.supportedSpec", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.call-sign" => Some(("activeConfig.callSign", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.category" => Some(("activeConfig.category", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-azimuth" => Some(("activeConfig.installationParams.antennaAzimuth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-beamwidth" => Some(("activeConfig.installationParams.antennaBeamwidth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-downtilt" => Some(("activeConfig.installationParams.antennaDowntilt", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-gain" => Some(("activeConfig.installationParams.antennaGain", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-model" => Some(("activeConfig.installationParams.antennaModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.installation-params.cpe-cbsd-indication" => Some(("activeConfig.installationParams.cpeCbsdIndication", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "active-config.installation-params.eirp-capability" => Some(("activeConfig.installationParams.eirpCapability", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "active-config.installation-params.height" => Some(("activeConfig.installationParams.height", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.height-type" => Some(("activeConfig.installationParams.heightType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.installation-params.horizontal-accuracy" => Some(("activeConfig.installationParams.horizontalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.indoor-deployment" => Some(("activeConfig.installationParams.indoorDeployment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "active-config.installation-params.latitude" => Some(("activeConfig.installationParams.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.longitude" => Some(("activeConfig.installationParams.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.vertical-accuracy" => Some(("activeConfig.installationParams.verticalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.is-signed" => Some(("activeConfig.isSigned", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "active-config.measurement-capabilities" => Some(("activeConfig.measurementCapabilities", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "active-config.model.firmware-version" => Some(("activeConfig.model.firmwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.model.hardware-version" => Some(("activeConfig.model.hardwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.model.name" => Some(("activeConfig.model.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.model.software-version" => Some(("activeConfig.model.softwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.model.vendor" => Some(("activeConfig.model.vendor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.state" => Some(("activeConfig.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.update-time" => Some(("activeConfig.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.user-id" => Some(("activeConfig.userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.antenna-model" => Some(("deviceMetadata.antennaModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.common-channel-group" => Some(("deviceMetadata.commonChannelGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.interference-coordination-group" => Some(("deviceMetadata.interferenceCoordinationGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validated" => Some(("deviceMetadata.nrqzValidated", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.case-id" => Some(("deviceMetadata.nrqzValidation.caseId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.cpi-id" => Some(("deviceMetadata.nrqzValidation.cpiId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.latitude" => Some(("deviceMetadata.nrqzValidation.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.longitude" => Some(("deviceMetadata.nrqzValidation.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.state" => Some(("deviceMetadata.nrqzValidation.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fcc-id" => Some(("fccId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.air-interface.radio-technology" => Some(("preloadedConfig.airInterface.radioTechnology", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.air-interface.supported-spec" => Some(("preloadedConfig.airInterface.supportedSpec", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.call-sign" => Some(("preloadedConfig.callSign", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.category" => Some(("preloadedConfig.category", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-azimuth" => Some(("preloadedConfig.installationParams.antennaAzimuth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-beamwidth" => Some(("preloadedConfig.installationParams.antennaBeamwidth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-downtilt" => Some(("preloadedConfig.installationParams.antennaDowntilt", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-gain" => Some(("preloadedConfig.installationParams.antennaGain", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-model" => Some(("preloadedConfig.installationParams.antennaModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.cpe-cbsd-indication" => Some(("preloadedConfig.installationParams.cpeCbsdIndication", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.eirp-capability" => Some(("preloadedConfig.installationParams.eirpCapability", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.height" => Some(("preloadedConfig.installationParams.height", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.height-type" => Some(("preloadedConfig.installationParams.heightType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.horizontal-accuracy" => Some(("preloadedConfig.installationParams.horizontalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.indoor-deployment" => Some(("preloadedConfig.installationParams.indoorDeployment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.latitude" => Some(("preloadedConfig.installationParams.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.longitude" => Some(("preloadedConfig.installationParams.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.vertical-accuracy" => Some(("preloadedConfig.installationParams.verticalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.is-signed" => Some(("preloadedConfig.isSigned", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "preloaded-config.measurement-capabilities" => Some(("preloadedConfig.measurementCapabilities", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "preloaded-config.model.firmware-version" => Some(("preloadedConfig.model.firmwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.model.hardware-version" => Some(("preloadedConfig.model.hardwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.model.name" => Some(("preloadedConfig.model.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.model.software-version" => Some(("preloadedConfig.model.softwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.model.vendor" => Some(("preloadedConfig.model.vendor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.state" => Some(("preloadedConfig.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.update-time" => Some(("preloadedConfig.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.user-id" => Some(("preloadedConfig.userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "serial-number" => Some(("serialNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["active-config", "air-interface", "antenna-azimuth", "antenna-beamwidth", "antenna-downtilt", "antenna-gain", "antenna-model", "call-sign", "case-id", "category", "common-channel-group", "cpe-cbsd-indication", "cpi-id", "device-metadata", "display-name", "eirp-capability", "fcc-id", "firmware-version", "hardware-version", "height", "height-type", "horizontal-accuracy", "indoor-deployment", "installation-params", "interference-coordination-group", "is-signed", "latitude", "longitude", "measurement-capabilities", "model", "name", "nrqz-validated", "nrqz-validation", "preloaded-config", "radio-technology", "serial-number", "software-version", "state", "supported-spec", "update-time", "user-id", "vendor", "vertical-accuracy"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalDevice = json::value::from_value(object).unwrap();
        let mut call = self.hub.nodes().devices_patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
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
                                                                           v.extend(["update-mask"].iter().map(|v|*v));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _nodes_devices_sign_device(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "device.active-config.air-interface.radio-technology" => Some(("device.activeConfig.airInterface.radioTechnology", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.active-config.air-interface.supported-spec" => Some(("device.activeConfig.airInterface.supportedSpec", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.active-config.call-sign" => Some(("device.activeConfig.callSign", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.active-config.category" => Some(("device.activeConfig.category", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.antenna-azimuth" => Some(("device.activeConfig.installationParams.antennaAzimuth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.antenna-beamwidth" => Some(("device.activeConfig.installationParams.antennaBeamwidth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.antenna-downtilt" => Some(("device.activeConfig.installationParams.antennaDowntilt", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.antenna-gain" => Some(("device.activeConfig.installationParams.antennaGain", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.antenna-model" => Some(("device.activeConfig.installationParams.antennaModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.cpe-cbsd-indication" => Some(("device.activeConfig.installationParams.cpeCbsdIndication", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.eirp-capability" => Some(("device.activeConfig.installationParams.eirpCapability", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.height" => Some(("device.activeConfig.installationParams.height", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.height-type" => Some(("device.activeConfig.installationParams.heightType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.horizontal-accuracy" => Some(("device.activeConfig.installationParams.horizontalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.indoor-deployment" => Some(("device.activeConfig.installationParams.indoorDeployment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.latitude" => Some(("device.activeConfig.installationParams.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.longitude" => Some(("device.activeConfig.installationParams.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.active-config.installation-params.vertical-accuracy" => Some(("device.activeConfig.installationParams.verticalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.active-config.is-signed" => Some(("device.activeConfig.isSigned", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "device.active-config.measurement-capabilities" => Some(("device.activeConfig.measurementCapabilities", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "device.active-config.model.firmware-version" => Some(("device.activeConfig.model.firmwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.active-config.model.hardware-version" => Some(("device.activeConfig.model.hardwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.active-config.model.name" => Some(("device.activeConfig.model.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.active-config.model.software-version" => Some(("device.activeConfig.model.softwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.active-config.model.vendor" => Some(("device.activeConfig.model.vendor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.active-config.state" => Some(("device.activeConfig.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.active-config.update-time" => Some(("device.activeConfig.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.active-config.user-id" => Some(("device.activeConfig.userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.device-metadata.antenna-model" => Some(("device.deviceMetadata.antennaModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.device-metadata.common-channel-group" => Some(("device.deviceMetadata.commonChannelGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.device-metadata.interference-coordination-group" => Some(("device.deviceMetadata.interferenceCoordinationGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.device-metadata.nrqz-validated" => Some(("device.deviceMetadata.nrqzValidated", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "device.device-metadata.nrqz-validation.case-id" => Some(("device.deviceMetadata.nrqzValidation.caseId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.device-metadata.nrqz-validation.cpi-id" => Some(("device.deviceMetadata.nrqzValidation.cpiId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.device-metadata.nrqz-validation.latitude" => Some(("device.deviceMetadata.nrqzValidation.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.device-metadata.nrqz-validation.longitude" => Some(("device.deviceMetadata.nrqzValidation.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.device-metadata.nrqz-validation.state" => Some(("device.deviceMetadata.nrqzValidation.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.display-name" => Some(("device.displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.fcc-id" => Some(("device.fccId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.name" => Some(("device.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.air-interface.radio-technology" => Some(("device.preloadedConfig.airInterface.radioTechnology", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.air-interface.supported-spec" => Some(("device.preloadedConfig.airInterface.supportedSpec", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.call-sign" => Some(("device.preloadedConfig.callSign", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.category" => Some(("device.preloadedConfig.category", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.antenna-azimuth" => Some(("device.preloadedConfig.installationParams.antennaAzimuth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.antenna-beamwidth" => Some(("device.preloadedConfig.installationParams.antennaBeamwidth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.antenna-downtilt" => Some(("device.preloadedConfig.installationParams.antennaDowntilt", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.antenna-gain" => Some(("device.preloadedConfig.installationParams.antennaGain", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.antenna-model" => Some(("device.preloadedConfig.installationParams.antennaModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.cpe-cbsd-indication" => Some(("device.preloadedConfig.installationParams.cpeCbsdIndication", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.eirp-capability" => Some(("device.preloadedConfig.installationParams.eirpCapability", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.height" => Some(("device.preloadedConfig.installationParams.height", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.height-type" => Some(("device.preloadedConfig.installationParams.heightType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.horizontal-accuracy" => Some(("device.preloadedConfig.installationParams.horizontalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.indoor-deployment" => Some(("device.preloadedConfig.installationParams.indoorDeployment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.latitude" => Some(("device.preloadedConfig.installationParams.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.longitude" => Some(("device.preloadedConfig.installationParams.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.preloaded-config.installation-params.vertical-accuracy" => Some(("device.preloadedConfig.installationParams.verticalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device.preloaded-config.is-signed" => Some(("device.preloadedConfig.isSigned", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "device.preloaded-config.measurement-capabilities" => Some(("device.preloadedConfig.measurementCapabilities", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "device.preloaded-config.model.firmware-version" => Some(("device.preloadedConfig.model.firmwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.model.hardware-version" => Some(("device.preloadedConfig.model.hardwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.model.name" => Some(("device.preloadedConfig.model.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.model.software-version" => Some(("device.preloadedConfig.model.softwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.model.vendor" => Some(("device.preloadedConfig.model.vendor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.state" => Some(("device.preloadedConfig.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.update-time" => Some(("device.preloadedConfig.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.preloaded-config.user-id" => Some(("device.preloadedConfig.userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.serial-number" => Some(("device.serialNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device.state" => Some(("device.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["active-config", "air-interface", "antenna-azimuth", "antenna-beamwidth", "antenna-downtilt", "antenna-gain", "antenna-model", "call-sign", "case-id", "category", "common-channel-group", "cpe-cbsd-indication", "cpi-id", "device", "device-metadata", "display-name", "eirp-capability", "fcc-id", "firmware-version", "hardware-version", "height", "height-type", "horizontal-accuracy", "indoor-deployment", "installation-params", "interference-coordination-group", "is-signed", "latitude", "longitude", "measurement-capabilities", "model", "name", "nrqz-validated", "nrqz-validation", "preloaded-config", "radio-technology", "serial-number", "software-version", "state", "supported-spec", "update-time", "user-id", "vendor", "vertical-accuracy"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalSignDeviceRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.nodes().devices_sign_device(request, opt.value_of("name").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _nodes_devices_update_signed(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "encoded-device" => Some(("encodedDevice", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "installer-id" => Some(("installerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["encoded-device", "installer-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalUpdateSignedDeviceRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.nodes().devices_update_signed(request, opt.value_of("name").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _nodes_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.nodes().get(opt.value_of("name").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _nodes_nodes_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sas-user-ids" => Some(("sasUserIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["display-name", "name", "sas-user-ids"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalNode = json::value::from_value(object).unwrap();
        let mut call = self.hub.nodes().nodes_create(request, opt.value_of("parent").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _nodes_nodes_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.nodes().nodes_delete(opt.value_of("name").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _nodes_nodes_deployments_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "frns" => Some(("frns", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sas-user-ids" => Some(("sasUserIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["display-name", "frns", "name", "sas-user-ids"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalDeployment = json::value::from_value(object).unwrap();
        let mut call = self.hub.nodes().nodes_deployments_create(request, opt.value_of("parent").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _nodes_nodes_deployments_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.nodes().nodes_deployments_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
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
                                                                           v.extend(["filter", "page-size", "page-token"].iter().map(|v|*v));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _nodes_nodes_devices_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "active-config.air-interface.radio-technology" => Some(("activeConfig.airInterface.radioTechnology", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.air-interface.supported-spec" => Some(("activeConfig.airInterface.supportedSpec", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.call-sign" => Some(("activeConfig.callSign", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.category" => Some(("activeConfig.category", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-azimuth" => Some(("activeConfig.installationParams.antennaAzimuth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-beamwidth" => Some(("activeConfig.installationParams.antennaBeamwidth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-downtilt" => Some(("activeConfig.installationParams.antennaDowntilt", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-gain" => Some(("activeConfig.installationParams.antennaGain", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.antenna-model" => Some(("activeConfig.installationParams.antennaModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.installation-params.cpe-cbsd-indication" => Some(("activeConfig.installationParams.cpeCbsdIndication", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "active-config.installation-params.eirp-capability" => Some(("activeConfig.installationParams.eirpCapability", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "active-config.installation-params.height" => Some(("activeConfig.installationParams.height", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.height-type" => Some(("activeConfig.installationParams.heightType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.installation-params.horizontal-accuracy" => Some(("activeConfig.installationParams.horizontalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.indoor-deployment" => Some(("activeConfig.installationParams.indoorDeployment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "active-config.installation-params.latitude" => Some(("activeConfig.installationParams.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.longitude" => Some(("activeConfig.installationParams.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.installation-params.vertical-accuracy" => Some(("activeConfig.installationParams.verticalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "active-config.is-signed" => Some(("activeConfig.isSigned", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "active-config.measurement-capabilities" => Some(("activeConfig.measurementCapabilities", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "active-config.model.firmware-version" => Some(("activeConfig.model.firmwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.model.hardware-version" => Some(("activeConfig.model.hardwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.model.name" => Some(("activeConfig.model.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.model.software-version" => Some(("activeConfig.model.softwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.model.vendor" => Some(("activeConfig.model.vendor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.state" => Some(("activeConfig.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.update-time" => Some(("activeConfig.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "active-config.user-id" => Some(("activeConfig.userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.antenna-model" => Some(("deviceMetadata.antennaModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.common-channel-group" => Some(("deviceMetadata.commonChannelGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.interference-coordination-group" => Some(("deviceMetadata.interferenceCoordinationGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validated" => Some(("deviceMetadata.nrqzValidated", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.case-id" => Some(("deviceMetadata.nrqzValidation.caseId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.cpi-id" => Some(("deviceMetadata.nrqzValidation.cpiId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.latitude" => Some(("deviceMetadata.nrqzValidation.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.longitude" => Some(("deviceMetadata.nrqzValidation.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "device-metadata.nrqz-validation.state" => Some(("deviceMetadata.nrqzValidation.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fcc-id" => Some(("fccId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.air-interface.radio-technology" => Some(("preloadedConfig.airInterface.radioTechnology", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.air-interface.supported-spec" => Some(("preloadedConfig.airInterface.supportedSpec", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.call-sign" => Some(("preloadedConfig.callSign", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.category" => Some(("preloadedConfig.category", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-azimuth" => Some(("preloadedConfig.installationParams.antennaAzimuth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-beamwidth" => Some(("preloadedConfig.installationParams.antennaBeamwidth", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-downtilt" => Some(("preloadedConfig.installationParams.antennaDowntilt", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-gain" => Some(("preloadedConfig.installationParams.antennaGain", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.antenna-model" => Some(("preloadedConfig.installationParams.antennaModel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.cpe-cbsd-indication" => Some(("preloadedConfig.installationParams.cpeCbsdIndication", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.eirp-capability" => Some(("preloadedConfig.installationParams.eirpCapability", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.height" => Some(("preloadedConfig.installationParams.height", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.height-type" => Some(("preloadedConfig.installationParams.heightType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.horizontal-accuracy" => Some(("preloadedConfig.installationParams.horizontalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.indoor-deployment" => Some(("preloadedConfig.installationParams.indoorDeployment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.latitude" => Some(("preloadedConfig.installationParams.latitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.longitude" => Some(("preloadedConfig.installationParams.longitude", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.installation-params.vertical-accuracy" => Some(("preloadedConfig.installationParams.verticalAccuracy", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "preloaded-config.is-signed" => Some(("preloadedConfig.isSigned", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "preloaded-config.measurement-capabilities" => Some(("preloadedConfig.measurementCapabilities", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "preloaded-config.model.firmware-version" => Some(("preloadedConfig.model.firmwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.model.hardware-version" => Some(("preloadedConfig.model.hardwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.model.name" => Some(("preloadedConfig.model.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.model.software-version" => Some(("preloadedConfig.model.softwareVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.model.vendor" => Some(("preloadedConfig.model.vendor", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.state" => Some(("preloadedConfig.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.update-time" => Some(("preloadedConfig.updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "preloaded-config.user-id" => Some(("preloadedConfig.userId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "serial-number" => Some(("serialNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["active-config", "air-interface", "antenna-azimuth", "antenna-beamwidth", "antenna-downtilt", "antenna-gain", "antenna-model", "call-sign", "case-id", "category", "common-channel-group", "cpe-cbsd-indication", "cpi-id", "device-metadata", "display-name", "eirp-capability", "fcc-id", "firmware-version", "hardware-version", "height", "height-type", "horizontal-accuracy", "indoor-deployment", "installation-params", "interference-coordination-group", "is-signed", "latitude", "longitude", "measurement-capabilities", "model", "name", "nrqz-validated", "nrqz-validation", "preloaded-config", "radio-technology", "serial-number", "software-version", "state", "supported-spec", "update-time", "user-id", "vendor", "vertical-accuracy"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalDevice = json::value::from_value(object).unwrap();
        let mut call = self.hub.nodes().nodes_devices_create(request, opt.value_of("parent").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _nodes_nodes_devices_create_signed(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "encoded-device" => Some(("encodedDevice", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "installer-id" => Some(("installerId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["encoded-device", "installer-id"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalCreateSignedDeviceRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.nodes().nodes_devices_create_signed(request, opt.value_of("parent").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _nodes_nodes_devices_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.nodes().nodes_devices_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
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
                                                                           v.extend(["filter", "page-size", "page-token"].iter().map(|v|*v));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _nodes_nodes_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.nodes().nodes_get(opt.value_of("name").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _nodes_nodes_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.nodes().nodes_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
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
                                                                           v.extend(["filter", "page-size", "page-token"].iter().map(|v|*v));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _nodes_nodes_move(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "destination" => Some(("destination", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["destination"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalMoveNodeRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.nodes().nodes_move(request, opt.value_of("name").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _nodes_nodes_nodes_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sas-user-ids" => Some(("sasUserIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["display-name", "name", "sas-user-ids"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalNode = json::value::from_value(object).unwrap();
        let mut call = self.hub.nodes().nodes_nodes_create(request, opt.value_of("parent").unwrap_or(""));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _nodes_nodes_nodes_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.nodes().nodes_nodes_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
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
                                                                           v.extend(["filter", "page-size", "page-token"].iter().map(|v|*v));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _nodes_nodes_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "display-name" => Some(("displayName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "sas-user-ids" => Some(("sasUserIds", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["display-name", "name", "sas-user-ids"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalNode = json::value::from_value(object).unwrap();
        let mut call = self.hub.nodes().nodes_patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
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
                                                                           v.extend(["update-mask"].iter().map(|v|*v));
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _policies_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "resource" => Some(("resource", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["resource"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalGetPolicyRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.policies().get(request);
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _policies_set(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "disable-notification" => Some(("disableNotification", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "policy.etag" => Some(("policy.etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "resource" => Some(("resource", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["disable-notification", "etag", "policy", "resource"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalSetPolicyRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.policies().set(request);
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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

    async fn _policies_test(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "permissions" => Some(("permissions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "resource" => Some(("resource", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["permissions", "resource"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SasPortalTestPermissionsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.policies().test(request);
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
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
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
            ("customers", Some(opt)) => {
                match opt.subcommand() {
                    ("deployments-create", Some(opt)) => {
                        call_result = self._customers_deployments_create(opt, dry_run, &mut err).await;
                    },
                    ("deployments-delete", Some(opt)) => {
                        call_result = self._customers_deployments_delete(opt, dry_run, &mut err).await;
                    },
                    ("deployments-devices-create", Some(opt)) => {
                        call_result = self._customers_deployments_devices_create(opt, dry_run, &mut err).await;
                    },
                    ("deployments-devices-create-signed", Some(opt)) => {
                        call_result = self._customers_deployments_devices_create_signed(opt, dry_run, &mut err).await;
                    },
                    ("deployments-devices-list", Some(opt)) => {
                        call_result = self._customers_deployments_devices_list(opt, dry_run, &mut err).await;
                    },
                    ("deployments-get", Some(opt)) => {
                        call_result = self._customers_deployments_get(opt, dry_run, &mut err).await;
                    },
                    ("deployments-list", Some(opt)) => {
                        call_result = self._customers_deployments_list(opt, dry_run, &mut err).await;
                    },
                    ("deployments-move", Some(opt)) => {
                        call_result = self._customers_deployments_move(opt, dry_run, &mut err).await;
                    },
                    ("deployments-patch", Some(opt)) => {
                        call_result = self._customers_deployments_patch(opt, dry_run, &mut err).await;
                    },
                    ("devices-create", Some(opt)) => {
                        call_result = self._customers_devices_create(opt, dry_run, &mut err).await;
                    },
                    ("devices-create-signed", Some(opt)) => {
                        call_result = self._customers_devices_create_signed(opt, dry_run, &mut err).await;
                    },
                    ("devices-delete", Some(opt)) => {
                        call_result = self._customers_devices_delete(opt, dry_run, &mut err).await;
                    },
                    ("devices-get", Some(opt)) => {
                        call_result = self._customers_devices_get(opt, dry_run, &mut err).await;
                    },
                    ("devices-list", Some(opt)) => {
                        call_result = self._customers_devices_list(opt, dry_run, &mut err).await;
                    },
                    ("devices-move", Some(opt)) => {
                        call_result = self._customers_devices_move(opt, dry_run, &mut err).await;
                    },
                    ("devices-patch", Some(opt)) => {
                        call_result = self._customers_devices_patch(opt, dry_run, &mut err).await;
                    },
                    ("devices-sign-device", Some(opt)) => {
                        call_result = self._customers_devices_sign_device(opt, dry_run, &mut err).await;
                    },
                    ("devices-update-signed", Some(opt)) => {
                        call_result = self._customers_devices_update_signed(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._customers_get(opt, dry_run, &mut err).await;
                    },
                    ("list", Some(opt)) => {
                        call_result = self._customers_list(opt, dry_run, &mut err).await;
                    },
                    ("list-gcp-project-deployments", Some(opt)) => {
                        call_result = self._customers_list_gcp_project_deployments(opt, dry_run, &mut err).await;
                    },
                    ("list-legacy-organizations", Some(opt)) => {
                        call_result = self._customers_list_legacy_organizations(opt, dry_run, &mut err).await;
                    },
                    ("migrate-organization", Some(opt)) => {
                        call_result = self._customers_migrate_organization(opt, dry_run, &mut err).await;
                    },
                    ("nodes-create", Some(opt)) => {
                        call_result = self._customers_nodes_create(opt, dry_run, &mut err).await;
                    },
                    ("nodes-delete", Some(opt)) => {
                        call_result = self._customers_nodes_delete(opt, dry_run, &mut err).await;
                    },
                    ("nodes-deployments-create", Some(opt)) => {
                        call_result = self._customers_nodes_deployments_create(opt, dry_run, &mut err).await;
                    },
                    ("nodes-deployments-list", Some(opt)) => {
                        call_result = self._customers_nodes_deployments_list(opt, dry_run, &mut err).await;
                    },
                    ("nodes-devices-create", Some(opt)) => {
                        call_result = self._customers_nodes_devices_create(opt, dry_run, &mut err).await;
                    },
                    ("nodes-devices-create-signed", Some(opt)) => {
                        call_result = self._customers_nodes_devices_create_signed(opt, dry_run, &mut err).await;
                    },
                    ("nodes-devices-list", Some(opt)) => {
                        call_result = self._customers_nodes_devices_list(opt, dry_run, &mut err).await;
                    },
                    ("nodes-get", Some(opt)) => {
                        call_result = self._customers_nodes_get(opt, dry_run, &mut err).await;
                    },
                    ("nodes-list", Some(opt)) => {
                        call_result = self._customers_nodes_list(opt, dry_run, &mut err).await;
                    },
                    ("nodes-move", Some(opt)) => {
                        call_result = self._customers_nodes_move(opt, dry_run, &mut err).await;
                    },
                    ("nodes-nodes-create", Some(opt)) => {
                        call_result = self._customers_nodes_nodes_create(opt, dry_run, &mut err).await;
                    },
                    ("nodes-nodes-list", Some(opt)) => {
                        call_result = self._customers_nodes_nodes_list(opt, dry_run, &mut err).await;
                    },
                    ("nodes-patch", Some(opt)) => {
                        call_result = self._customers_nodes_patch(opt, dry_run, &mut err).await;
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._customers_patch(opt, dry_run, &mut err).await;
                    },
                    ("provision-deployment", Some(opt)) => {
                        call_result = self._customers_provision_deployment(opt, dry_run, &mut err).await;
                    },
                    ("setup-sas-analytics", Some(opt)) => {
                        call_result = self._customers_setup_sas_analytics(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("customers".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("deployments", Some(opt)) => {
                match opt.subcommand() {
                    ("devices-delete", Some(opt)) => {
                        call_result = self._deployments_devices_delete(opt, dry_run, &mut err).await;
                    },
                    ("devices-get", Some(opt)) => {
                        call_result = self._deployments_devices_get(opt, dry_run, &mut err).await;
                    },
                    ("devices-move", Some(opt)) => {
                        call_result = self._deployments_devices_move(opt, dry_run, &mut err).await;
                    },
                    ("devices-patch", Some(opt)) => {
                        call_result = self._deployments_devices_patch(opt, dry_run, &mut err).await;
                    },
                    ("devices-sign-device", Some(opt)) => {
                        call_result = self._deployments_devices_sign_device(opt, dry_run, &mut err).await;
                    },
                    ("devices-update-signed", Some(opt)) => {
                        call_result = self._deployments_devices_update_signed(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._deployments_get(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("deployments".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("installer", Some(opt)) => {
                match opt.subcommand() {
                    ("generate-secret", Some(opt)) => {
                        call_result = self._installer_generate_secret(opt, dry_run, &mut err).await;
                    },
                    ("validate", Some(opt)) => {
                        call_result = self._installer_validate(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("installer".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("nodes", Some(opt)) => {
                match opt.subcommand() {
                    ("deployments-delete", Some(opt)) => {
                        call_result = self._nodes_deployments_delete(opt, dry_run, &mut err).await;
                    },
                    ("deployments-devices-create", Some(opt)) => {
                        call_result = self._nodes_deployments_devices_create(opt, dry_run, &mut err).await;
                    },
                    ("deployments-devices-create-signed", Some(opt)) => {
                        call_result = self._nodes_deployments_devices_create_signed(opt, dry_run, &mut err).await;
                    },
                    ("deployments-devices-list", Some(opt)) => {
                        call_result = self._nodes_deployments_devices_list(opt, dry_run, &mut err).await;
                    },
                    ("deployments-get", Some(opt)) => {
                        call_result = self._nodes_deployments_get(opt, dry_run, &mut err).await;
                    },
                    ("deployments-list", Some(opt)) => {
                        call_result = self._nodes_deployments_list(opt, dry_run, &mut err).await;
                    },
                    ("deployments-move", Some(opt)) => {
                        call_result = self._nodes_deployments_move(opt, dry_run, &mut err).await;
                    },
                    ("deployments-patch", Some(opt)) => {
                        call_result = self._nodes_deployments_patch(opt, dry_run, &mut err).await;
                    },
                    ("devices-create", Some(opt)) => {
                        call_result = self._nodes_devices_create(opt, dry_run, &mut err).await;
                    },
                    ("devices-create-signed", Some(opt)) => {
                        call_result = self._nodes_devices_create_signed(opt, dry_run, &mut err).await;
                    },
                    ("devices-delete", Some(opt)) => {
                        call_result = self._nodes_devices_delete(opt, dry_run, &mut err).await;
                    },
                    ("devices-get", Some(opt)) => {
                        call_result = self._nodes_devices_get(opt, dry_run, &mut err).await;
                    },
                    ("devices-list", Some(opt)) => {
                        call_result = self._nodes_devices_list(opt, dry_run, &mut err).await;
                    },
                    ("devices-move", Some(opt)) => {
                        call_result = self._nodes_devices_move(opt, dry_run, &mut err).await;
                    },
                    ("devices-patch", Some(opt)) => {
                        call_result = self._nodes_devices_patch(opt, dry_run, &mut err).await;
                    },
                    ("devices-sign-device", Some(opt)) => {
                        call_result = self._nodes_devices_sign_device(opt, dry_run, &mut err).await;
                    },
                    ("devices-update-signed", Some(opt)) => {
                        call_result = self._nodes_devices_update_signed(opt, dry_run, &mut err).await;
                    },
                    ("get", Some(opt)) => {
                        call_result = self._nodes_get(opt, dry_run, &mut err).await;
                    },
                    ("nodes-create", Some(opt)) => {
                        call_result = self._nodes_nodes_create(opt, dry_run, &mut err).await;
                    },
                    ("nodes-delete", Some(opt)) => {
                        call_result = self._nodes_nodes_delete(opt, dry_run, &mut err).await;
                    },
                    ("nodes-deployments-create", Some(opt)) => {
                        call_result = self._nodes_nodes_deployments_create(opt, dry_run, &mut err).await;
                    },
                    ("nodes-deployments-list", Some(opt)) => {
                        call_result = self._nodes_nodes_deployments_list(opt, dry_run, &mut err).await;
                    },
                    ("nodes-devices-create", Some(opt)) => {
                        call_result = self._nodes_nodes_devices_create(opt, dry_run, &mut err).await;
                    },
                    ("nodes-devices-create-signed", Some(opt)) => {
                        call_result = self._nodes_nodes_devices_create_signed(opt, dry_run, &mut err).await;
                    },
                    ("nodes-devices-list", Some(opt)) => {
                        call_result = self._nodes_nodes_devices_list(opt, dry_run, &mut err).await;
                    },
                    ("nodes-get", Some(opt)) => {
                        call_result = self._nodes_nodes_get(opt, dry_run, &mut err).await;
                    },
                    ("nodes-list", Some(opt)) => {
                        call_result = self._nodes_nodes_list(opt, dry_run, &mut err).await;
                    },
                    ("nodes-move", Some(opt)) => {
                        call_result = self._nodes_nodes_move(opt, dry_run, &mut err).await;
                    },
                    ("nodes-nodes-create", Some(opt)) => {
                        call_result = self._nodes_nodes_nodes_create(opt, dry_run, &mut err).await;
                    },
                    ("nodes-nodes-list", Some(opt)) => {
                        call_result = self._nodes_nodes_nodes_list(opt, dry_run, &mut err).await;
                    },
                    ("nodes-patch", Some(opt)) => {
                        call_result = self._nodes_nodes_patch(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("nodes".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            ("policies", Some(opt)) => {
                match opt.subcommand() {
                    ("get", Some(opt)) => {
                        call_result = self._policies_get(opt, dry_run, &mut err).await;
                    },
                    ("set", Some(opt)) => {
                        call_result = self._policies_set(opt, dry_run, &mut err).await;
                    },
                    ("test", Some(opt)) => {
                        call_result = self._policies_test(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("policies".to_string()));
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

            match client::application_secret_from_directory(&config_dir, "sasportal1-alpha1-secret.json",
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
        ).persist_tokens_to_disk(format!("{}/sasportal1-alpha1", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::Sasportal::new(client, auth),
            gp: vec!["$-xgafv", "access-token", "alt", "callback", "fields", "key", "oauth-token", "pretty-print", "quota-user", "upload-type", "upload-protocol"],
            gpm: vec![
                    ("$-xgafv", "$.xgafv"),
                    ("access-token", "access_token"),
                    ("oauth-token", "oauth_token"),
                    ("pretty-print", "prettyPrint"),
                    ("quota-user", "quotaUser"),
                    ("upload-type", "uploadType"),
                    ("upload-protocol", "upload_protocol"),
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
        ("customers", "methods: 'deployments-create', 'deployments-delete', 'deployments-devices-create', 'deployments-devices-create-signed', 'deployments-devices-list', 'deployments-get', 'deployments-list', 'deployments-move', 'deployments-patch', 'devices-create', 'devices-create-signed', 'devices-delete', 'devices-get', 'devices-list', 'devices-move', 'devices-patch', 'devices-sign-device', 'devices-update-signed', 'get', 'list', 'list-gcp-project-deployments', 'list-legacy-organizations', 'migrate-organization', 'nodes-create', 'nodes-delete', 'nodes-deployments-create', 'nodes-deployments-list', 'nodes-devices-create', 'nodes-devices-create-signed', 'nodes-devices-list', 'nodes-get', 'nodes-list', 'nodes-move', 'nodes-nodes-create', 'nodes-nodes-list', 'nodes-patch', 'patch', 'provision-deployment' and 'setup-sas-analytics'", vec![
            ("deployments-create",
                    Some(r##"Creates a new deployment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/customers_deployments-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource name where the deployment is to be created."##),
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
            ("deployments-delete",
                    Some(r##"Deletes a deployment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/customers_deployments-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the deployment."##),
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
            ("deployments-devices-create",
                    Some(r##"Creates a device under a node or customer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/customers_deployments-devices-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the parent resource."##),
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
            ("deployments-devices-create-signed",
                    Some(r##"Creates a signed device under a node or customer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/customers_deployments-devices-create-signed",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the parent resource."##),
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
            ("deployments-devices-list",
                    Some(r##"Lists devices under a node or customer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/customers_deployments-devices-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the parent resource."##),
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
            ("deployments-get",
                    Some(r##"Returns a requested deployment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/customers_deployments-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the deployment."##),
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
            ("deployments-list",
                    Some(r##"Lists deployments."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/customers_deployments-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource name, for example, "nodes/1", customer/1/nodes/2."##),
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
            ("deployments-move",
                    Some(r##"Moves a deployment under another node or customer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/customers_deployments-move",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the deployment to move."##),
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
            ("deployments-patch",
                    Some(r##"Updates an existing deployment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/customers_deployments-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Output only. Resource name."##),
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
            ("devices-create",
                    Some(r##"Creates a device under a node or customer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/customers_devices-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the parent resource."##),
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
            ("devices-create-signed",
                    Some(r##"Creates a signed device under a node or customer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/customers_devices-create-signed",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the parent resource."##),
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
            ("devices-delete",
                    Some(r##"Deletes a device."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/customers_devices-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the device."##),
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
            ("devices-get",
                    Some(r##"Gets details about a device."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/customers_devices-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the device."##),
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
            ("devices-list",
                    Some(r##"Lists devices under a node or customer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/customers_devices-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the parent resource."##),
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
            ("devices-move",
                    Some(r##"Moves a device under another node or customer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/customers_devices-move",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the device to move."##),
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
            ("devices-patch",
                    Some(r##"Updates a device."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/customers_devices-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Output only. The resource path name."##),
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
            ("devices-sign-device",
                    Some(r##"Signs a device."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/customers_devices-sign-device",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Output only. The resource path name."##),
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
            ("devices-update-signed",
                    Some(r##"Updates a signed device."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/customers_devices-update-signed",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the device to update."##),
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
            ("get",
                    Some(r##"Returns a requested customer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/customers_get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the customer."##),
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
                    Some(r##"Returns a list of requested customers."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/customers_list",
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
            ("list-gcp-project-deployments",
                    Some(r##"Returns a list of SAS deployments associated with current GCP project. Includes whether SAS analytics has been enabled or not."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/customers_list-gcp-project-deployments",
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
            ("list-legacy-organizations",
                    Some(r##"Returns a list of legacy organizations."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/customers_list-legacy-organizations",
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
            ("migrate-organization",
                    Some(r##"Migrates a SAS organization to the cloud. This will create GCP projects for each deployment and associate them. The SAS Organization is linked to the gcp project that called the command. go/sas-legacy-customer-migration"##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/customers_migrate-organization",
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
            ("nodes-create",
                    Some(r##"Creates a new node."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/customers_nodes-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource name where the node is to be created."##),
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
            ("nodes-delete",
                    Some(r##"Deletes a node."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/customers_nodes-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the node."##),
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
            ("nodes-deployments-create",
                    Some(r##"Creates a new deployment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/customers_nodes-deployments-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource name where the deployment is to be created."##),
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
            ("nodes-deployments-list",
                    Some(r##"Lists deployments."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/customers_nodes-deployments-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource name, for example, "nodes/1", customer/1/nodes/2."##),
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
            ("nodes-devices-create",
                    Some(r##"Creates a device under a node or customer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/customers_nodes-devices-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the parent resource."##),
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
            ("nodes-devices-create-signed",
                    Some(r##"Creates a signed device under a node or customer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/customers_nodes-devices-create-signed",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the parent resource."##),
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
            ("nodes-devices-list",
                    Some(r##"Lists devices under a node or customer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/customers_nodes-devices-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the parent resource."##),
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
            ("nodes-get",
                    Some(r##"Returns a requested node."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/customers_nodes-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the node."##),
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
            ("nodes-list",
                    Some(r##"Lists nodes."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/customers_nodes-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource name, for example, "nodes/1"."##),
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
            ("nodes-move",
                    Some(r##"Moves a node under another node or customer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/customers_nodes-move",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the node to move."##),
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
            ("nodes-nodes-create",
                    Some(r##"Creates a new node."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/customers_nodes-nodes-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource name where the node is to be created."##),
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
            ("nodes-nodes-list",
                    Some(r##"Lists nodes."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/customers_nodes-nodes-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource name, for example, "nodes/1"."##),
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
            ("nodes-patch",
                    Some(r##"Updates an existing node."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/customers_nodes-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Output only. Resource name."##),
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
            ("patch",
                    Some(r##"Updates an existing customer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/customers_patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Output only. Resource name of the customer."##),
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
            ("provision-deployment",
                    Some(r##"Creates a new SAS deployment through the GCP workflow. Creates a SAS organization if an organization match is not found."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/customers_provision-deployment",
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
            ("setup-sas-analytics",
                    Some(r##"Setups the a GCP Project to receive SAS Analytics messages via GCP Pub/Sub with a subscription to BigQuery. All the Pub/Sub topics and BigQuery tables are created automatically as part of this service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/customers_setup-sas-analytics",
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
        
        ("deployments", "methods: 'devices-delete', 'devices-get', 'devices-move', 'devices-patch', 'devices-sign-device', 'devices-update-signed' and 'get'", vec![
            ("devices-delete",
                    Some(r##"Deletes a device."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/deployments_devices-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the device."##),
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
            ("devices-get",
                    Some(r##"Gets details about a device."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/deployments_devices-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the device."##),
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
            ("devices-move",
                    Some(r##"Moves a device under another node or customer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/deployments_devices-move",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the device to move."##),
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
            ("devices-patch",
                    Some(r##"Updates a device."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/deployments_devices-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Output only. The resource path name."##),
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
            ("devices-sign-device",
                    Some(r##"Signs a device."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/deployments_devices-sign-device",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Output only. The resource path name."##),
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
            ("devices-update-signed",
                    Some(r##"Updates a signed device."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/deployments_devices-update-signed",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the device to update."##),
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
            ("get",
                    Some(r##"Returns a requested deployment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/deployments_get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the deployment."##),
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
        
        ("installer", "methods: 'generate-secret' and 'validate'", vec![
            ("generate-secret",
                    Some(r##"Generates a secret to be used with the ValidateInstaller."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/installer_generate-secret",
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
            ("validate",
                    Some(r##"Validates the identity of a Certified Professional Installer (CPI)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/installer_validate",
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
        
        ("nodes", "methods: 'deployments-delete', 'deployments-devices-create', 'deployments-devices-create-signed', 'deployments-devices-list', 'deployments-get', 'deployments-list', 'deployments-move', 'deployments-patch', 'devices-create', 'devices-create-signed', 'devices-delete', 'devices-get', 'devices-list', 'devices-move', 'devices-patch', 'devices-sign-device', 'devices-update-signed', 'get', 'nodes-create', 'nodes-delete', 'nodes-deployments-create', 'nodes-deployments-list', 'nodes-devices-create', 'nodes-devices-create-signed', 'nodes-devices-list', 'nodes-get', 'nodes-list', 'nodes-move', 'nodes-nodes-create', 'nodes-nodes-list' and 'nodes-patch'", vec![
            ("deployments-delete",
                    Some(r##"Deletes a deployment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/nodes_deployments-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the deployment."##),
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
            ("deployments-devices-create",
                    Some(r##"Creates a device under a node or customer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/nodes_deployments-devices-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the parent resource."##),
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
            ("deployments-devices-create-signed",
                    Some(r##"Creates a signed device under a node or customer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/nodes_deployments-devices-create-signed",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the parent resource."##),
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
            ("deployments-devices-list",
                    Some(r##"Lists devices under a node or customer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/nodes_deployments-devices-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the parent resource."##),
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
            ("deployments-get",
                    Some(r##"Returns a requested deployment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/nodes_deployments-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the deployment."##),
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
            ("deployments-list",
                    Some(r##"Lists deployments."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/nodes_deployments-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource name, for example, "nodes/1", customer/1/nodes/2."##),
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
            ("deployments-move",
                    Some(r##"Moves a deployment under another node or customer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/nodes_deployments-move",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the deployment to move."##),
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
            ("deployments-patch",
                    Some(r##"Updates an existing deployment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/nodes_deployments-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Output only. Resource name."##),
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
            ("devices-create",
                    Some(r##"Creates a device under a node or customer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/nodes_devices-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the parent resource."##),
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
            ("devices-create-signed",
                    Some(r##"Creates a signed device under a node or customer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/nodes_devices-create-signed",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the parent resource."##),
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
            ("devices-delete",
                    Some(r##"Deletes a device."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/nodes_devices-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the device."##),
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
            ("devices-get",
                    Some(r##"Gets details about a device."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/nodes_devices-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the device."##),
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
            ("devices-list",
                    Some(r##"Lists devices under a node or customer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/nodes_devices-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the parent resource."##),
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
            ("devices-move",
                    Some(r##"Moves a device under another node or customer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/nodes_devices-move",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the device to move."##),
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
            ("devices-patch",
                    Some(r##"Updates a device."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/nodes_devices-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Output only. The resource path name."##),
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
            ("devices-sign-device",
                    Some(r##"Signs a device."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/nodes_devices-sign-device",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Output only. The resource path name."##),
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
            ("devices-update-signed",
                    Some(r##"Updates a signed device."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/nodes_devices-update-signed",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the device to update."##),
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
            ("get",
                    Some(r##"Returns a requested node."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/nodes_get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the node."##),
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
            ("nodes-create",
                    Some(r##"Creates a new node."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/nodes_nodes-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource name where the node is to be created."##),
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
            ("nodes-delete",
                    Some(r##"Deletes a node."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/nodes_nodes-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the node."##),
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
            ("nodes-deployments-create",
                    Some(r##"Creates a new deployment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/nodes_nodes-deployments-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource name where the deployment is to be created."##),
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
            ("nodes-deployments-list",
                    Some(r##"Lists deployments."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/nodes_nodes-deployments-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource name, for example, "nodes/1", customer/1/nodes/2."##),
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
            ("nodes-devices-create",
                    Some(r##"Creates a device under a node or customer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/nodes_nodes-devices-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the parent resource."##),
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
            ("nodes-devices-create-signed",
                    Some(r##"Creates a signed device under a node or customer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/nodes_nodes-devices-create-signed",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the parent resource."##),
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
            ("nodes-devices-list",
                    Some(r##"Lists devices under a node or customer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/nodes_nodes-devices-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The name of the parent resource."##),
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
            ("nodes-get",
                    Some(r##"Returns a requested node."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/nodes_nodes-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the node."##),
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
            ("nodes-list",
                    Some(r##"Lists nodes."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/nodes_nodes-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource name, for example, "nodes/1"."##),
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
            ("nodes-move",
                    Some(r##"Moves a node under another node or customer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/nodes_nodes-move",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the node to move."##),
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
            ("nodes-nodes-create",
                    Some(r##"Creates a new node."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/nodes_nodes-nodes-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource name where the node is to be created."##),
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
            ("nodes-nodes-list",
                    Some(r##"Lists nodes."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/nodes_nodes-nodes-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource name, for example, "nodes/1"."##),
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
            ("nodes-patch",
                    Some(r##"Updates an existing node."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/nodes_nodes-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Output only. Resource name."##),
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
        
        ("policies", "methods: 'get', 'set' and 'test'", vec![
            ("get",
                    Some(r##"Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/policies_get",
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
            ("set",
                    Some(r##"Sets the access control policy on the specified resource. Replaces any existing policy."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/policies_set",
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
            ("test",
                    Some(r##"Returns permissions that a caller has on the specified resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli/policies_test",
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
    
    let mut app = App::new("sasportal1-alpha1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+20240615")
           .about("")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_sasportal1_alpha1_cli")
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
