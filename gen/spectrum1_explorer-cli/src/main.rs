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
extern crate google_spectrum1_explorer as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  spectrum1-explorer [options] paws get-spectrum -r <kv>... [-p <v>]... [-o <out>]
  spectrum1-explorer [options] paws get-spectrum-batch -r <kv>... [-p <v>]... [-o <out>]
  spectrum1-explorer [options] paws init -r <kv>... [-p <v>]... [-o <out>]
  spectrum1-explorer [options] paws notify-spectrum-use -r <kv>... [-p <v>]... [-o <out>]
  spectrum1-explorer [options] paws register -r <kv>... [-p <v>]... [-o <out>]
  spectrum1-explorer [options] paws verify-device -r <kv>... [-p <v>]... [-o <out>]
  spectrum1-explorer --help

All documentation details can be found TODO: <URL to github.io docs here, see #51>

Configuration:
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
    hub: api::Spectrum<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _paws_get_spectrum(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::PawsGetSpectrumRequest = Default::default();
        let mut call = self.hub.paws().get_spectrum(&request);
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
            fn request_antenna_init(request: &mut api::PawsGetSpectrumRequest) {
                if request.antenna.is_none() {
                    request.antenna = Some(Default::default());
                }
            }
            
            fn request_device_desc_init(request: &mut api::PawsGetSpectrumRequest) {
                if request.device_desc.is_none() {
                    request.device_desc = Some(Default::default());
                }
            }
            
            fn request_location_init(request: &mut api::PawsGetSpectrumRequest) {
                if request.location.is_none() {
                    request.location = Some(Default::default());
                }
            }
            
            fn request_master_device_desc_init(request: &mut api::PawsGetSpectrumRequest) {
                if request.master_device_desc.is_none() {
                    request.master_device_desc = Some(Default::default());
                }
            }
            
            fn request_owner_init(request: &mut api::PawsGetSpectrumRequest) {
                if request.owner.is_none() {
                    request.owner = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "device-desc.etsi-en-device-type" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().etsi_en_device_type = value.unwrap_or("").to_string();
                    },
                "device-desc.fcc-id" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().fcc_id = value.unwrap_or("").to_string();
                    },
                "device-desc.serial-number" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().serial_number = value.unwrap_or("").to_string();
                    },
                "device-desc.etsi-en-technology-id" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().etsi_en_technology_id = value.unwrap_or("").to_string();
                    },
                "device-desc.fcc-tvbd-device-type" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().fcc_tvbd_device_type = value.unwrap_or("").to_string();
                    },
                "device-desc.etsi-en-device-category" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().etsi_en_device_category = value.unwrap_or("").to_string();
                    },
                "device-desc.ruleset-ids" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().ruleset_ids.push(value.unwrap_or("").to_string());
                    },
                "device-desc.etsi-en-device-emissions-class" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().etsi_en_device_emissions_class = value.unwrap_or("").to_string();
                    },
                "device-desc.manufacturer-id" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().manufacturer_id = value.unwrap_or("").to_string();
                    },
                "device-desc.model-id" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().model_id = value.unwrap_or("").to_string();
                    },
                "version" => {
                        request_device_desc_init(&mut request);
                        request.version = Some(value.unwrap_or("").to_string());
                    },
                "antenna.height-type" => {
                        request_antenna_init(&mut request);
                        request.antenna.as_mut().unwrap().height_type = value.unwrap_or("").to_string();
                    },
                "antenna.height-uncertainty" => {
                        request_antenna_init(&mut request);
                        request.antenna.as_mut().unwrap().height_uncertainty = arg_from_str(value.unwrap_or("0.0"), err, "antenna.height-uncertainty", "number");
                    },
                "antenna.height" => {
                        request_antenna_init(&mut request);
                        request.antenna.as_mut().unwrap().height = arg_from_str(value.unwrap_or("0.0"), err, "antenna.height", "number");
                    },
                "request-type" => {
                        request_antenna_init(&mut request);
                        request.request_type = Some(value.unwrap_or("").to_string());
                    },
                "master-device-desc.etsi-en-device-type" => {
                        request_master_device_desc_init(&mut request);
                        request.master_device_desc.as_mut().unwrap().etsi_en_device_type = value.unwrap_or("").to_string();
                    },
                "master-device-desc.fcc-id" => {
                        request_master_device_desc_init(&mut request);
                        request.master_device_desc.as_mut().unwrap().fcc_id = value.unwrap_or("").to_string();
                    },
                "master-device-desc.serial-number" => {
                        request_master_device_desc_init(&mut request);
                        request.master_device_desc.as_mut().unwrap().serial_number = value.unwrap_or("").to_string();
                    },
                "master-device-desc.etsi-en-technology-id" => {
                        request_master_device_desc_init(&mut request);
                        request.master_device_desc.as_mut().unwrap().etsi_en_technology_id = value.unwrap_or("").to_string();
                    },
                "master-device-desc.fcc-tvbd-device-type" => {
                        request_master_device_desc_init(&mut request);
                        request.master_device_desc.as_mut().unwrap().fcc_tvbd_device_type = value.unwrap_or("").to_string();
                    },
                "master-device-desc.etsi-en-device-category" => {
                        request_master_device_desc_init(&mut request);
                        request.master_device_desc.as_mut().unwrap().etsi_en_device_category = value.unwrap_or("").to_string();
                    },
                "master-device-desc.ruleset-ids" => {
                        request_master_device_desc_init(&mut request);
                        request.master_device_desc.as_mut().unwrap().ruleset_ids.push(value.unwrap_or("").to_string());
                    },
                "master-device-desc.etsi-en-device-emissions-class" => {
                        request_master_device_desc_init(&mut request);
                        request.master_device_desc.as_mut().unwrap().etsi_en_device_emissions_class = value.unwrap_or("").to_string();
                    },
                "master-device-desc.manufacturer-id" => {
                        request_master_device_desc_init(&mut request);
                        request.master_device_desc.as_mut().unwrap().manufacturer_id = value.unwrap_or("").to_string();
                    },
                "master-device-desc.model-id" => {
                        request_master_device_desc_init(&mut request);
                        request.master_device_desc.as_mut().unwrap().model_id = value.unwrap_or("").to_string();
                    },
                "location.confidence" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().confidence = arg_from_str(value.unwrap_or("-0"), err, "location.confidence", "integer");
                    },
                "location.point.semi-minor-axis" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().point.semi_minor_axis = arg_from_str(value.unwrap_or("0.0"), err, "location.point.semi-minor-axis", "number");
                    },
                "location.point.center.latitude" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().point.center.latitude = arg_from_str(value.unwrap_or("0.0"), err, "location.point.center.latitude", "number");
                    },
                "location.point.center.longitude" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().point.center.longitude = arg_from_str(value.unwrap_or("0.0"), err, "location.point.center.longitude", "number");
                    },
                "location.point.semi-major-axis" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().point.semi_major_axis = arg_from_str(value.unwrap_or("0.0"), err, "location.point.semi-major-axis", "number");
                    },
                "location.point.orientation" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().point.orientation = arg_from_str(value.unwrap_or("0.0"), err, "location.point.orientation", "number");
                    },
                "owner.operator.org.text" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().operator.org.text = value.unwrap_or("").to_string();
                    },
                "owner.operator.tel.uri" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().operator.tel.uri = value.unwrap_or("").to_string();
                    },
                "owner.operator.email.text" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().operator.email.text = value.unwrap_or("").to_string();
                    },
                "owner.operator.fn" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().operator.fn_ = value.unwrap_or("").to_string();
                    },
                "owner.operator.adr.code" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().operator.adr.code = value.unwrap_or("").to_string();
                    },
                "owner.operator.adr.locality" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().operator.adr.locality = value.unwrap_or("").to_string();
                    },
                "owner.operator.adr.country" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().operator.adr.country = value.unwrap_or("").to_string();
                    },
                "owner.operator.adr.region" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().operator.adr.region = value.unwrap_or("").to_string();
                    },
                "owner.operator.adr.pobox" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().operator.adr.pobox = value.unwrap_or("").to_string();
                    },
                "owner.operator.adr.street" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().operator.adr.street = value.unwrap_or("").to_string();
                    },
                "owner.owner.org.text" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().owner.org.text = value.unwrap_or("").to_string();
                    },
                "owner.owner.tel.uri" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().owner.tel.uri = value.unwrap_or("").to_string();
                    },
                "owner.owner.email.text" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().owner.email.text = value.unwrap_or("").to_string();
                    },
                "owner.owner.fn" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().owner.fn_ = value.unwrap_or("").to_string();
                    },
                "owner.owner.adr.code" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().owner.adr.code = value.unwrap_or("").to_string();
                    },
                "owner.owner.adr.locality" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().owner.adr.locality = value.unwrap_or("").to_string();
                    },
                "owner.owner.adr.country" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().owner.adr.country = value.unwrap_or("").to_string();
                    },
                "owner.owner.adr.region" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().owner.adr.region = value.unwrap_or("").to_string();
                    },
                "owner.owner.adr.pobox" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().owner.adr.pobox = value.unwrap_or("").to_string();
                    },
                "owner.owner.adr.street" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().owner.adr.street = value.unwrap_or("").to_string();
                    },
                "type" => {
                        request_owner_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
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

    fn _paws_get_spectrum_batch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::PawsGetSpectrumBatchRequest = Default::default();
        let mut call = self.hub.paws().get_spectrum_batch(&request);
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
            fn request_antenna_init(request: &mut api::PawsGetSpectrumBatchRequest) {
                if request.antenna.is_none() {
                    request.antenna = Some(Default::default());
                }
            }
            
            fn request_device_desc_init(request: &mut api::PawsGetSpectrumBatchRequest) {
                if request.device_desc.is_none() {
                    request.device_desc = Some(Default::default());
                }
            }
            
            fn request_master_device_desc_init(request: &mut api::PawsGetSpectrumBatchRequest) {
                if request.master_device_desc.is_none() {
                    request.master_device_desc = Some(Default::default());
                }
            }
            
            fn request_owner_init(request: &mut api::PawsGetSpectrumBatchRequest) {
                if request.owner.is_none() {
                    request.owner = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "device-desc.etsi-en-device-type" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().etsi_en_device_type = value.unwrap_or("").to_string();
                    },
                "device-desc.fcc-id" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().fcc_id = value.unwrap_or("").to_string();
                    },
                "device-desc.serial-number" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().serial_number = value.unwrap_or("").to_string();
                    },
                "device-desc.etsi-en-technology-id" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().etsi_en_technology_id = value.unwrap_or("").to_string();
                    },
                "device-desc.fcc-tvbd-device-type" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().fcc_tvbd_device_type = value.unwrap_or("").to_string();
                    },
                "device-desc.etsi-en-device-category" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().etsi_en_device_category = value.unwrap_or("").to_string();
                    },
                "device-desc.ruleset-ids" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().ruleset_ids.push(value.unwrap_or("").to_string());
                    },
                "device-desc.etsi-en-device-emissions-class" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().etsi_en_device_emissions_class = value.unwrap_or("").to_string();
                    },
                "device-desc.manufacturer-id" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().manufacturer_id = value.unwrap_or("").to_string();
                    },
                "device-desc.model-id" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().model_id = value.unwrap_or("").to_string();
                    },
                "version" => {
                        request_device_desc_init(&mut request);
                        request.version = Some(value.unwrap_or("").to_string());
                    },
                "antenna.height-type" => {
                        request_antenna_init(&mut request);
                        request.antenna.as_mut().unwrap().height_type = value.unwrap_or("").to_string();
                    },
                "antenna.height-uncertainty" => {
                        request_antenna_init(&mut request);
                        request.antenna.as_mut().unwrap().height_uncertainty = arg_from_str(value.unwrap_or("0.0"), err, "antenna.height-uncertainty", "number");
                    },
                "antenna.height" => {
                        request_antenna_init(&mut request);
                        request.antenna.as_mut().unwrap().height = arg_from_str(value.unwrap_or("0.0"), err, "antenna.height", "number");
                    },
                "request-type" => {
                        request_antenna_init(&mut request);
                        request.request_type = Some(value.unwrap_or("").to_string());
                    },
                "master-device-desc.etsi-en-device-type" => {
                        request_master_device_desc_init(&mut request);
                        request.master_device_desc.as_mut().unwrap().etsi_en_device_type = value.unwrap_or("").to_string();
                    },
                "master-device-desc.fcc-id" => {
                        request_master_device_desc_init(&mut request);
                        request.master_device_desc.as_mut().unwrap().fcc_id = value.unwrap_or("").to_string();
                    },
                "master-device-desc.serial-number" => {
                        request_master_device_desc_init(&mut request);
                        request.master_device_desc.as_mut().unwrap().serial_number = value.unwrap_or("").to_string();
                    },
                "master-device-desc.etsi-en-technology-id" => {
                        request_master_device_desc_init(&mut request);
                        request.master_device_desc.as_mut().unwrap().etsi_en_technology_id = value.unwrap_or("").to_string();
                    },
                "master-device-desc.fcc-tvbd-device-type" => {
                        request_master_device_desc_init(&mut request);
                        request.master_device_desc.as_mut().unwrap().fcc_tvbd_device_type = value.unwrap_or("").to_string();
                    },
                "master-device-desc.etsi-en-device-category" => {
                        request_master_device_desc_init(&mut request);
                        request.master_device_desc.as_mut().unwrap().etsi_en_device_category = value.unwrap_or("").to_string();
                    },
                "master-device-desc.ruleset-ids" => {
                        request_master_device_desc_init(&mut request);
                        request.master_device_desc.as_mut().unwrap().ruleset_ids.push(value.unwrap_or("").to_string());
                    },
                "master-device-desc.etsi-en-device-emissions-class" => {
                        request_master_device_desc_init(&mut request);
                        request.master_device_desc.as_mut().unwrap().etsi_en_device_emissions_class = value.unwrap_or("").to_string();
                    },
                "master-device-desc.manufacturer-id" => {
                        request_master_device_desc_init(&mut request);
                        request.master_device_desc.as_mut().unwrap().manufacturer_id = value.unwrap_or("").to_string();
                    },
                "master-device-desc.model-id" => {
                        request_master_device_desc_init(&mut request);
                        request.master_device_desc.as_mut().unwrap().model_id = value.unwrap_or("").to_string();
                    },
                "owner.operator.org.text" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().operator.org.text = value.unwrap_or("").to_string();
                    },
                "owner.operator.tel.uri" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().operator.tel.uri = value.unwrap_or("").to_string();
                    },
                "owner.operator.email.text" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().operator.email.text = value.unwrap_or("").to_string();
                    },
                "owner.operator.fn" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().operator.fn_ = value.unwrap_or("").to_string();
                    },
                "owner.operator.adr.code" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().operator.adr.code = value.unwrap_or("").to_string();
                    },
                "owner.operator.adr.locality" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().operator.adr.locality = value.unwrap_or("").to_string();
                    },
                "owner.operator.adr.country" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().operator.adr.country = value.unwrap_or("").to_string();
                    },
                "owner.operator.adr.region" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().operator.adr.region = value.unwrap_or("").to_string();
                    },
                "owner.operator.adr.pobox" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().operator.adr.pobox = value.unwrap_or("").to_string();
                    },
                "owner.operator.adr.street" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().operator.adr.street = value.unwrap_or("").to_string();
                    },
                "owner.owner.org.text" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().owner.org.text = value.unwrap_or("").to_string();
                    },
                "owner.owner.tel.uri" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().owner.tel.uri = value.unwrap_or("").to_string();
                    },
                "owner.owner.email.text" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().owner.email.text = value.unwrap_or("").to_string();
                    },
                "owner.owner.fn" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().owner.fn_ = value.unwrap_or("").to_string();
                    },
                "owner.owner.adr.code" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().owner.adr.code = value.unwrap_or("").to_string();
                    },
                "owner.owner.adr.locality" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().owner.adr.locality = value.unwrap_or("").to_string();
                    },
                "owner.owner.adr.country" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().owner.adr.country = value.unwrap_or("").to_string();
                    },
                "owner.owner.adr.region" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().owner.adr.region = value.unwrap_or("").to_string();
                    },
                "owner.owner.adr.pobox" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().owner.adr.pobox = value.unwrap_or("").to_string();
                    },
                "owner.owner.adr.street" => {
                        request_owner_init(&mut request);
                        request.owner.as_mut().unwrap().owner.adr.street = value.unwrap_or("").to_string();
                    },
                "type" => {
                        request_owner_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
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

    fn _paws_init(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::PawsInitRequest = Default::default();
        let mut call = self.hub.paws().init(&request);
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
            fn request_device_desc_init(request: &mut api::PawsInitRequest) {
                if request.device_desc.is_none() {
                    request.device_desc = Some(Default::default());
                }
            }
            
            fn request_location_init(request: &mut api::PawsInitRequest) {
                if request.location.is_none() {
                    request.location = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "device-desc.etsi-en-device-type" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().etsi_en_device_type = value.unwrap_or("").to_string();
                    },
                "device-desc.fcc-id" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().fcc_id = value.unwrap_or("").to_string();
                    },
                "device-desc.serial-number" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().serial_number = value.unwrap_or("").to_string();
                    },
                "device-desc.etsi-en-technology-id" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().etsi_en_technology_id = value.unwrap_or("").to_string();
                    },
                "device-desc.fcc-tvbd-device-type" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().fcc_tvbd_device_type = value.unwrap_or("").to_string();
                    },
                "device-desc.etsi-en-device-category" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().etsi_en_device_category = value.unwrap_or("").to_string();
                    },
                "device-desc.ruleset-ids" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().ruleset_ids.push(value.unwrap_or("").to_string());
                    },
                "device-desc.etsi-en-device-emissions-class" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().etsi_en_device_emissions_class = value.unwrap_or("").to_string();
                    },
                "device-desc.manufacturer-id" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().manufacturer_id = value.unwrap_or("").to_string();
                    },
                "device-desc.model-id" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().model_id = value.unwrap_or("").to_string();
                    },
                "type" => {
                        request_device_desc_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "location.confidence" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().confidence = arg_from_str(value.unwrap_or("-0"), err, "location.confidence", "integer");
                    },
                "location.point.semi-minor-axis" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().point.semi_minor_axis = arg_from_str(value.unwrap_or("0.0"), err, "location.point.semi-minor-axis", "number");
                    },
                "location.point.center.latitude" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().point.center.latitude = arg_from_str(value.unwrap_or("0.0"), err, "location.point.center.latitude", "number");
                    },
                "location.point.center.longitude" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().point.center.longitude = arg_from_str(value.unwrap_or("0.0"), err, "location.point.center.longitude", "number");
                    },
                "location.point.semi-major-axis" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().point.semi_major_axis = arg_from_str(value.unwrap_or("0.0"), err, "location.point.semi-major-axis", "number");
                    },
                "location.point.orientation" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().point.orientation = arg_from_str(value.unwrap_or("0.0"), err, "location.point.orientation", "number");
                    },
                "version" => {
                        request_location_init(&mut request);
                        request.version = Some(value.unwrap_or("").to_string());
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

    fn _paws_notify_spectrum_use(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::PawsNotifySpectrumUseRequest = Default::default();
        let mut call = self.hub.paws().notify_spectrum_use(&request);
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
            fn request_device_desc_init(request: &mut api::PawsNotifySpectrumUseRequest) {
                if request.device_desc.is_none() {
                    request.device_desc = Some(Default::default());
                }
            }
            
            fn request_location_init(request: &mut api::PawsNotifySpectrumUseRequest) {
                if request.location.is_none() {
                    request.location = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "device-desc.etsi-en-device-type" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().etsi_en_device_type = value.unwrap_or("").to_string();
                    },
                "device-desc.fcc-id" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().fcc_id = value.unwrap_or("").to_string();
                    },
                "device-desc.serial-number" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().serial_number = value.unwrap_or("").to_string();
                    },
                "device-desc.etsi-en-technology-id" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().etsi_en_technology_id = value.unwrap_or("").to_string();
                    },
                "device-desc.fcc-tvbd-device-type" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().fcc_tvbd_device_type = value.unwrap_or("").to_string();
                    },
                "device-desc.etsi-en-device-category" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().etsi_en_device_category = value.unwrap_or("").to_string();
                    },
                "device-desc.ruleset-ids" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().ruleset_ids.push(value.unwrap_or("").to_string());
                    },
                "device-desc.etsi-en-device-emissions-class" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().etsi_en_device_emissions_class = value.unwrap_or("").to_string();
                    },
                "device-desc.manufacturer-id" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().manufacturer_id = value.unwrap_or("").to_string();
                    },
                "device-desc.model-id" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().model_id = value.unwrap_or("").to_string();
                    },
                "type" => {
                        request_device_desc_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "location.confidence" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().confidence = arg_from_str(value.unwrap_or("-0"), err, "location.confidence", "integer");
                    },
                "location.point.semi-minor-axis" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().point.semi_minor_axis = arg_from_str(value.unwrap_or("0.0"), err, "location.point.semi-minor-axis", "number");
                    },
                "location.point.center.latitude" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().point.center.latitude = arg_from_str(value.unwrap_or("0.0"), err, "location.point.center.latitude", "number");
                    },
                "location.point.center.longitude" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().point.center.longitude = arg_from_str(value.unwrap_or("0.0"), err, "location.point.center.longitude", "number");
                    },
                "location.point.semi-major-axis" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().point.semi_major_axis = arg_from_str(value.unwrap_or("0.0"), err, "location.point.semi-major-axis", "number");
                    },
                "location.point.orientation" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().point.orientation = arg_from_str(value.unwrap_or("0.0"), err, "location.point.orientation", "number");
                    },
                "version" => {
                        request_location_init(&mut request);
                        request.version = Some(value.unwrap_or("").to_string());
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

    fn _paws_register(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::PawsRegisterRequest = Default::default();
        let mut call = self.hub.paws().register(&request);
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
            fn request_antenna_init(request: &mut api::PawsRegisterRequest) {
                if request.antenna.is_none() {
                    request.antenna = Some(Default::default());
                }
            }
            
            fn request_device_desc_init(request: &mut api::PawsRegisterRequest) {
                if request.device_desc.is_none() {
                    request.device_desc = Some(Default::default());
                }
            }
            
            fn request_device_owner_init(request: &mut api::PawsRegisterRequest) {
                if request.device_owner.is_none() {
                    request.device_owner = Some(Default::default());
                }
            }
            
            fn request_location_init(request: &mut api::PawsRegisterRequest) {
                if request.location.is_none() {
                    request.location = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "device-desc.etsi-en-device-type" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().etsi_en_device_type = value.unwrap_or("").to_string();
                    },
                "device-desc.fcc-id" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().fcc_id = value.unwrap_or("").to_string();
                    },
                "device-desc.serial-number" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().serial_number = value.unwrap_or("").to_string();
                    },
                "device-desc.etsi-en-technology-id" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().etsi_en_technology_id = value.unwrap_or("").to_string();
                    },
                "device-desc.fcc-tvbd-device-type" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().fcc_tvbd_device_type = value.unwrap_or("").to_string();
                    },
                "device-desc.etsi-en-device-category" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().etsi_en_device_category = value.unwrap_or("").to_string();
                    },
                "device-desc.ruleset-ids" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().ruleset_ids.push(value.unwrap_or("").to_string());
                    },
                "device-desc.etsi-en-device-emissions-class" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().etsi_en_device_emissions_class = value.unwrap_or("").to_string();
                    },
                "device-desc.manufacturer-id" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().manufacturer_id = value.unwrap_or("").to_string();
                    },
                "device-desc.model-id" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().model_id = value.unwrap_or("").to_string();
                    },
                "antenna.height-type" => {
                        request_antenna_init(&mut request);
                        request.antenna.as_mut().unwrap().height_type = value.unwrap_or("").to_string();
                    },
                "antenna.height-uncertainty" => {
                        request_antenna_init(&mut request);
                        request.antenna.as_mut().unwrap().height_uncertainty = arg_from_str(value.unwrap_or("0.0"), err, "antenna.height-uncertainty", "number");
                    },
                "antenna.height" => {
                        request_antenna_init(&mut request);
                        request.antenna.as_mut().unwrap().height = arg_from_str(value.unwrap_or("0.0"), err, "antenna.height", "number");
                    },
                "device-owner.operator.org.text" => {
                        request_device_owner_init(&mut request);
                        request.device_owner.as_mut().unwrap().operator.org.text = value.unwrap_or("").to_string();
                    },
                "device-owner.operator.tel.uri" => {
                        request_device_owner_init(&mut request);
                        request.device_owner.as_mut().unwrap().operator.tel.uri = value.unwrap_or("").to_string();
                    },
                "device-owner.operator.email.text" => {
                        request_device_owner_init(&mut request);
                        request.device_owner.as_mut().unwrap().operator.email.text = value.unwrap_or("").to_string();
                    },
                "device-owner.operator.fn" => {
                        request_device_owner_init(&mut request);
                        request.device_owner.as_mut().unwrap().operator.fn_ = value.unwrap_or("").to_string();
                    },
                "device-owner.operator.adr.code" => {
                        request_device_owner_init(&mut request);
                        request.device_owner.as_mut().unwrap().operator.adr.code = value.unwrap_or("").to_string();
                    },
                "device-owner.operator.adr.locality" => {
                        request_device_owner_init(&mut request);
                        request.device_owner.as_mut().unwrap().operator.adr.locality = value.unwrap_or("").to_string();
                    },
                "device-owner.operator.adr.country" => {
                        request_device_owner_init(&mut request);
                        request.device_owner.as_mut().unwrap().operator.adr.country = value.unwrap_or("").to_string();
                    },
                "device-owner.operator.adr.region" => {
                        request_device_owner_init(&mut request);
                        request.device_owner.as_mut().unwrap().operator.adr.region = value.unwrap_or("").to_string();
                    },
                "device-owner.operator.adr.pobox" => {
                        request_device_owner_init(&mut request);
                        request.device_owner.as_mut().unwrap().operator.adr.pobox = value.unwrap_or("").to_string();
                    },
                "device-owner.operator.adr.street" => {
                        request_device_owner_init(&mut request);
                        request.device_owner.as_mut().unwrap().operator.adr.street = value.unwrap_or("").to_string();
                    },
                "device-owner.owner.org.text" => {
                        request_device_owner_init(&mut request);
                        request.device_owner.as_mut().unwrap().owner.org.text = value.unwrap_or("").to_string();
                    },
                "device-owner.owner.tel.uri" => {
                        request_device_owner_init(&mut request);
                        request.device_owner.as_mut().unwrap().owner.tel.uri = value.unwrap_or("").to_string();
                    },
                "device-owner.owner.email.text" => {
                        request_device_owner_init(&mut request);
                        request.device_owner.as_mut().unwrap().owner.email.text = value.unwrap_or("").to_string();
                    },
                "device-owner.owner.fn" => {
                        request_device_owner_init(&mut request);
                        request.device_owner.as_mut().unwrap().owner.fn_ = value.unwrap_or("").to_string();
                    },
                "device-owner.owner.adr.code" => {
                        request_device_owner_init(&mut request);
                        request.device_owner.as_mut().unwrap().owner.adr.code = value.unwrap_or("").to_string();
                    },
                "device-owner.owner.adr.locality" => {
                        request_device_owner_init(&mut request);
                        request.device_owner.as_mut().unwrap().owner.adr.locality = value.unwrap_or("").to_string();
                    },
                "device-owner.owner.adr.country" => {
                        request_device_owner_init(&mut request);
                        request.device_owner.as_mut().unwrap().owner.adr.country = value.unwrap_or("").to_string();
                    },
                "device-owner.owner.adr.region" => {
                        request_device_owner_init(&mut request);
                        request.device_owner.as_mut().unwrap().owner.adr.region = value.unwrap_or("").to_string();
                    },
                "device-owner.owner.adr.pobox" => {
                        request_device_owner_init(&mut request);
                        request.device_owner.as_mut().unwrap().owner.adr.pobox = value.unwrap_or("").to_string();
                    },
                "device-owner.owner.adr.street" => {
                        request_device_owner_init(&mut request);
                        request.device_owner.as_mut().unwrap().owner.adr.street = value.unwrap_or("").to_string();
                    },
                "version" => {
                        request_device_owner_init(&mut request);
                        request.version = Some(value.unwrap_or("").to_string());
                    },
                "location.confidence" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().confidence = arg_from_str(value.unwrap_or("-0"), err, "location.confidence", "integer");
                    },
                "location.point.semi-minor-axis" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().point.semi_minor_axis = arg_from_str(value.unwrap_or("0.0"), err, "location.point.semi-minor-axis", "number");
                    },
                "location.point.center.latitude" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().point.center.latitude = arg_from_str(value.unwrap_or("0.0"), err, "location.point.center.latitude", "number");
                    },
                "location.point.center.longitude" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().point.center.longitude = arg_from_str(value.unwrap_or("0.0"), err, "location.point.center.longitude", "number");
                    },
                "location.point.semi-major-axis" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().point.semi_major_axis = arg_from_str(value.unwrap_or("0.0"), err, "location.point.semi-major-axis", "number");
                    },
                "location.point.orientation" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().point.orientation = arg_from_str(value.unwrap_or("0.0"), err, "location.point.orientation", "number");
                    },
                "type" => {
                        request_location_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
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

    fn _paws_verify_device(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::PawsVerifyDeviceRequest = Default::default();
        let mut call = self.hub.paws().verify_device(&request);
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
                "version" => {
                        request.version = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
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

        if self.opt.cmd_paws {
            if self.opt.cmd_get_spectrum {
                call_result = self._paws_get_spectrum(dry_run, &mut err);
            } else if self.opt.cmd_get_spectrum_batch {
                call_result = self._paws_get_spectrum_batch(dry_run, &mut err);
            } else if self.opt.cmd_init {
                call_result = self._paws_init(dry_run, &mut err);
            } else if self.opt.cmd_notify_spectrum_use {
                call_result = self._paws_notify_spectrum_use(dry_run, &mut err);
            } else if self.opt.cmd_register {
                call_result = self._paws_register(dry_run, &mut err);
            } else if self.opt.cmd_verify_device {
                call_result = self._paws_verify_device(dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "spectrum1-explorer-secret.json") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                                      hyper::Client::new(),
                                      JsonTokenStorage {
                                        program_name: "spectrum1-explorer",
                                        db_dir: config_dir.clone(),
                                      }, None);
        let engine = Engine {
            opt: opt,
            hub: api::Spectrum::new(hyper::Client::new(), auth),
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