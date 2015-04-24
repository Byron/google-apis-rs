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
    hub: api::Spectrum<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _paws_get_spectrum(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::PawsGetSpectrumRequest::default();
        let mut call = self.hub.paws().get_spectrum(&request);
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
            
            fn request_location_point_center_init(request: &mut api::PawsGetSpectrumRequest) {
                request_location_point_init(request);
                if request.location.as_mut().unwrap().point.as_mut().unwrap().center.is_none() {
                    request.location.as_mut().unwrap().point.as_mut().unwrap().center = Some(Default::default());
                }
            }
            
            fn request_location_point_init(request: &mut api::PawsGetSpectrumRequest) {
                request_location_init(request);
                if request.location.as_mut().unwrap().point.is_none() {
                    request.location.as_mut().unwrap().point = Some(Default::default());
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
            
            fn request_owner_operator_adr_init(request: &mut api::PawsGetSpectrumRequest) {
                request_owner_operator_init(request);
                if request.owner.as_mut().unwrap().operator.as_mut().unwrap().adr.is_none() {
                    request.owner.as_mut().unwrap().operator.as_mut().unwrap().adr = Some(Default::default());
                }
            }
            
            fn request_owner_operator_email_init(request: &mut api::PawsGetSpectrumRequest) {
                request_owner_operator_init(request);
                if request.owner.as_mut().unwrap().operator.as_mut().unwrap().email.is_none() {
                    request.owner.as_mut().unwrap().operator.as_mut().unwrap().email = Some(Default::default());
                }
            }
            
            fn request_owner_operator_init(request: &mut api::PawsGetSpectrumRequest) {
                request_owner_init(request);
                if request.owner.as_mut().unwrap().operator.is_none() {
                    request.owner.as_mut().unwrap().operator = Some(Default::default());
                }
            }
            
            fn request_owner_operator_org_init(request: &mut api::PawsGetSpectrumRequest) {
                request_owner_operator_init(request);
                if request.owner.as_mut().unwrap().operator.as_mut().unwrap().org.is_none() {
                    request.owner.as_mut().unwrap().operator.as_mut().unwrap().org = Some(Default::default());
                }
            }
            
            fn request_owner_operator_tel_init(request: &mut api::PawsGetSpectrumRequest) {
                request_owner_operator_init(request);
                if request.owner.as_mut().unwrap().operator.as_mut().unwrap().tel.is_none() {
                    request.owner.as_mut().unwrap().operator.as_mut().unwrap().tel = Some(Default::default());
                }
            }
            
            fn request_owner_owner_adr_init(request: &mut api::PawsGetSpectrumRequest) {
                request_owner_owner_init(request);
                if request.owner.as_mut().unwrap().owner.as_mut().unwrap().adr.is_none() {
                    request.owner.as_mut().unwrap().owner.as_mut().unwrap().adr = Some(Default::default());
                }
            }
            
            fn request_owner_owner_email_init(request: &mut api::PawsGetSpectrumRequest) {
                request_owner_owner_init(request);
                if request.owner.as_mut().unwrap().owner.as_mut().unwrap().email.is_none() {
                    request.owner.as_mut().unwrap().owner.as_mut().unwrap().email = Some(Default::default());
                }
            }
            
            fn request_owner_owner_init(request: &mut api::PawsGetSpectrumRequest) {
                request_owner_init(request);
                if request.owner.as_mut().unwrap().owner.is_none() {
                    request.owner.as_mut().unwrap().owner = Some(Default::default());
                }
            }
            
            fn request_owner_owner_org_init(request: &mut api::PawsGetSpectrumRequest) {
                request_owner_owner_init(request);
                if request.owner.as_mut().unwrap().owner.as_mut().unwrap().org.is_none() {
                    request.owner.as_mut().unwrap().owner.as_mut().unwrap().org = Some(Default::default());
                }
            }
            
            fn request_owner_owner_tel_init(request: &mut api::PawsGetSpectrumRequest) {
                request_owner_owner_init(request);
                if request.owner.as_mut().unwrap().owner.as_mut().unwrap().tel.is_none() {
                    request.owner.as_mut().unwrap().owner.as_mut().unwrap().tel = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "device-desc.etsi-en-device-type" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().etsi_en_device_type = Some(value.unwrap_or("").to_string());
                    },
                "device-desc.fcc-id" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().fcc_id = Some(value.unwrap_or("").to_string());
                    },
                "device-desc.serial-number" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().serial_number = Some(value.unwrap_or("").to_string());
                    },
                "device-desc.etsi-en-technology-id" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().etsi_en_technology_id = Some(value.unwrap_or("").to_string());
                    },
                "device-desc.fcc-tvbd-device-type" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().fcc_tvbd_device_type = Some(value.unwrap_or("").to_string());
                    },
                "device-desc.etsi-en-device-category" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().etsi_en_device_category = Some(value.unwrap_or("").to_string());
                    },
                "device-desc.ruleset-ids" => {
                        request_device_desc_init(&mut request);
                        if request.device_desc.as_mut().unwrap().ruleset_ids.is_none() {
                           request.device_desc.as_mut().unwrap().ruleset_ids = Some(Default::default());
                        }
                                        request.device_desc.as_mut().unwrap().ruleset_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "device-desc.etsi-en-device-emissions-class" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().etsi_en_device_emissions_class = Some(value.unwrap_or("").to_string());
                    },
                "device-desc.manufacturer-id" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().manufacturer_id = Some(value.unwrap_or("").to_string());
                    },
                "device-desc.model-id" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().model_id = Some(value.unwrap_or("").to_string());
                    },
                "version" => {
                        request_device_desc_init(&mut request);
                        request.version = Some(value.unwrap_or("").to_string());
                    },
                "antenna.height-type" => {
                        request_antenna_init(&mut request);
                        request.antenna.as_mut().unwrap().height_type = Some(value.unwrap_or("").to_string());
                    },
                "antenna.height-uncertainty" => {
                        request_antenna_init(&mut request);
                        request.antenna.as_mut().unwrap().height_uncertainty = Some(arg_from_str(value.unwrap_or("0.0"), err, "antenna.height-uncertainty", "number"));
                    },
                "antenna.height" => {
                        request_antenna_init(&mut request);
                        request.antenna.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("0.0"), err, "antenna.height", "number"));
                    },
                "request-type" => {
                        request_antenna_init(&mut request);
                        request.request_type = Some(value.unwrap_or("").to_string());
                    },
                "master-device-desc.etsi-en-device-type" => {
                        request_master_device_desc_init(&mut request);
                        request.master_device_desc.as_mut().unwrap().etsi_en_device_type = Some(value.unwrap_or("").to_string());
                    },
                "master-device-desc.fcc-id" => {
                        request_master_device_desc_init(&mut request);
                        request.master_device_desc.as_mut().unwrap().fcc_id = Some(value.unwrap_or("").to_string());
                    },
                "master-device-desc.serial-number" => {
                        request_master_device_desc_init(&mut request);
                        request.master_device_desc.as_mut().unwrap().serial_number = Some(value.unwrap_or("").to_string());
                    },
                "master-device-desc.etsi-en-technology-id" => {
                        request_master_device_desc_init(&mut request);
                        request.master_device_desc.as_mut().unwrap().etsi_en_technology_id = Some(value.unwrap_or("").to_string());
                    },
                "master-device-desc.fcc-tvbd-device-type" => {
                        request_master_device_desc_init(&mut request);
                        request.master_device_desc.as_mut().unwrap().fcc_tvbd_device_type = Some(value.unwrap_or("").to_string());
                    },
                "master-device-desc.etsi-en-device-category" => {
                        request_master_device_desc_init(&mut request);
                        request.master_device_desc.as_mut().unwrap().etsi_en_device_category = Some(value.unwrap_or("").to_string());
                    },
                "master-device-desc.ruleset-ids" => {
                        request_master_device_desc_init(&mut request);
                        if request.master_device_desc.as_mut().unwrap().ruleset_ids.is_none() {
                           request.master_device_desc.as_mut().unwrap().ruleset_ids = Some(Default::default());
                        }
                                        request.master_device_desc.as_mut().unwrap().ruleset_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "master-device-desc.etsi-en-device-emissions-class" => {
                        request_master_device_desc_init(&mut request);
                        request.master_device_desc.as_mut().unwrap().etsi_en_device_emissions_class = Some(value.unwrap_or("").to_string());
                    },
                "master-device-desc.manufacturer-id" => {
                        request_master_device_desc_init(&mut request);
                        request.master_device_desc.as_mut().unwrap().manufacturer_id = Some(value.unwrap_or("").to_string());
                    },
                "master-device-desc.model-id" => {
                        request_master_device_desc_init(&mut request);
                        request.master_device_desc.as_mut().unwrap().model_id = Some(value.unwrap_or("").to_string());
                    },
                "location.confidence" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().confidence = Some(arg_from_str(value.unwrap_or("-0"), err, "location.confidence", "integer"));
                    },
                "location.point.semi-minor-axis" => {
                        request_location_point_init(&mut request);
                        request.location.as_mut().unwrap().point.as_mut().unwrap().semi_minor_axis = Some(arg_from_str(value.unwrap_or("0.0"), err, "location.point.semi-minor-axis", "number"));
                    },
                "location.point.center.latitude" => {
                        request_location_point_center_init(&mut request);
                        request.location.as_mut().unwrap().point.as_mut().unwrap().center.as_mut().unwrap().latitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "location.point.center.latitude", "number"));
                    },
                "location.point.center.longitude" => {
                        request_location_point_center_init(&mut request);
                        request.location.as_mut().unwrap().point.as_mut().unwrap().center.as_mut().unwrap().longitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "location.point.center.longitude", "number"));
                    },
                "location.point.semi-major-axis" => {
                        request_location_point_center_init(&mut request);
                        request.location.as_mut().unwrap().point.as_mut().unwrap().semi_major_axis = Some(arg_from_str(value.unwrap_or("0.0"), err, "location.point.semi-major-axis", "number"));
                    },
                "location.point.orientation" => {
                        request_location_point_center_init(&mut request);
                        request.location.as_mut().unwrap().point.as_mut().unwrap().orientation = Some(arg_from_str(value.unwrap_or("0.0"), err, "location.point.orientation", "number"));
                    },
                "owner.operator.org.text" => {
                        request_owner_operator_org_init(&mut request);
                        request.owner.as_mut().unwrap().operator.as_mut().unwrap().org.as_mut().unwrap().text = Some(value.unwrap_or("").to_string());
                    },
                "owner.operator.tel.uri" => {
                        request_owner_operator_tel_init(&mut request);
                        request.owner.as_mut().unwrap().operator.as_mut().unwrap().tel.as_mut().unwrap().uri = Some(value.unwrap_or("").to_string());
                    },
                "owner.operator.email.text" => {
                        request_owner_operator_email_init(&mut request);
                        request.owner.as_mut().unwrap().operator.as_mut().unwrap().email.as_mut().unwrap().text = Some(value.unwrap_or("").to_string());
                    },
                "owner.operator.fn" => {
                        request_owner_operator_email_init(&mut request);
                        request.owner.as_mut().unwrap().operator.as_mut().unwrap().fn_ = Some(value.unwrap_or("").to_string());
                    },
                "owner.operator.adr.code" => {
                        request_owner_operator_adr_init(&mut request);
                        request.owner.as_mut().unwrap().operator.as_mut().unwrap().adr.as_mut().unwrap().code = Some(value.unwrap_or("").to_string());
                    },
                "owner.operator.adr.locality" => {
                        request_owner_operator_adr_init(&mut request);
                        request.owner.as_mut().unwrap().operator.as_mut().unwrap().adr.as_mut().unwrap().locality = Some(value.unwrap_or("").to_string());
                    },
                "owner.operator.adr.country" => {
                        request_owner_operator_adr_init(&mut request);
                        request.owner.as_mut().unwrap().operator.as_mut().unwrap().adr.as_mut().unwrap().country = Some(value.unwrap_or("").to_string());
                    },
                "owner.operator.adr.region" => {
                        request_owner_operator_adr_init(&mut request);
                        request.owner.as_mut().unwrap().operator.as_mut().unwrap().adr.as_mut().unwrap().region = Some(value.unwrap_or("").to_string());
                    },
                "owner.operator.adr.pobox" => {
                        request_owner_operator_adr_init(&mut request);
                        request.owner.as_mut().unwrap().operator.as_mut().unwrap().adr.as_mut().unwrap().pobox = Some(value.unwrap_or("").to_string());
                    },
                "owner.operator.adr.street" => {
                        request_owner_operator_adr_init(&mut request);
                        request.owner.as_mut().unwrap().operator.as_mut().unwrap().adr.as_mut().unwrap().street = Some(value.unwrap_or("").to_string());
                    },
                "owner.owner.org.text" => {
                        request_owner_owner_org_init(&mut request);
                        request.owner.as_mut().unwrap().owner.as_mut().unwrap().org.as_mut().unwrap().text = Some(value.unwrap_or("").to_string());
                    },
                "owner.owner.tel.uri" => {
                        request_owner_owner_tel_init(&mut request);
                        request.owner.as_mut().unwrap().owner.as_mut().unwrap().tel.as_mut().unwrap().uri = Some(value.unwrap_or("").to_string());
                    },
                "owner.owner.email.text" => {
                        request_owner_owner_email_init(&mut request);
                        request.owner.as_mut().unwrap().owner.as_mut().unwrap().email.as_mut().unwrap().text = Some(value.unwrap_or("").to_string());
                    },
                "owner.owner.fn" => {
                        request_owner_owner_email_init(&mut request);
                        request.owner.as_mut().unwrap().owner.as_mut().unwrap().fn_ = Some(value.unwrap_or("").to_string());
                    },
                "owner.owner.adr.code" => {
                        request_owner_owner_adr_init(&mut request);
                        request.owner.as_mut().unwrap().owner.as_mut().unwrap().adr.as_mut().unwrap().code = Some(value.unwrap_or("").to_string());
                    },
                "owner.owner.adr.locality" => {
                        request_owner_owner_adr_init(&mut request);
                        request.owner.as_mut().unwrap().owner.as_mut().unwrap().adr.as_mut().unwrap().locality = Some(value.unwrap_or("").to_string());
                    },
                "owner.owner.adr.country" => {
                        request_owner_owner_adr_init(&mut request);
                        request.owner.as_mut().unwrap().owner.as_mut().unwrap().adr.as_mut().unwrap().country = Some(value.unwrap_or("").to_string());
                    },
                "owner.owner.adr.region" => {
                        request_owner_owner_adr_init(&mut request);
                        request.owner.as_mut().unwrap().owner.as_mut().unwrap().adr.as_mut().unwrap().region = Some(value.unwrap_or("").to_string());
                    },
                "owner.owner.adr.pobox" => {
                        request_owner_owner_adr_init(&mut request);
                        request.owner.as_mut().unwrap().owner.as_mut().unwrap().adr.as_mut().unwrap().pobox = Some(value.unwrap_or("").to_string());
                    },
                "owner.owner.adr.street" => {
                        request_owner_owner_adr_init(&mut request);
                        request.owner.as_mut().unwrap().owner.as_mut().unwrap().adr.as_mut().unwrap().street = Some(value.unwrap_or("").to_string());
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _paws_get_spectrum_batch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::PawsGetSpectrumBatchRequest::default();
        let mut call = self.hub.paws().get_spectrum_batch(&request);
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
            
            fn request_owner_operator_adr_init(request: &mut api::PawsGetSpectrumBatchRequest) {
                request_owner_operator_init(request);
                if request.owner.as_mut().unwrap().operator.as_mut().unwrap().adr.is_none() {
                    request.owner.as_mut().unwrap().operator.as_mut().unwrap().adr = Some(Default::default());
                }
            }
            
            fn request_owner_operator_email_init(request: &mut api::PawsGetSpectrumBatchRequest) {
                request_owner_operator_init(request);
                if request.owner.as_mut().unwrap().operator.as_mut().unwrap().email.is_none() {
                    request.owner.as_mut().unwrap().operator.as_mut().unwrap().email = Some(Default::default());
                }
            }
            
            fn request_owner_operator_init(request: &mut api::PawsGetSpectrumBatchRequest) {
                request_owner_init(request);
                if request.owner.as_mut().unwrap().operator.is_none() {
                    request.owner.as_mut().unwrap().operator = Some(Default::default());
                }
            }
            
            fn request_owner_operator_org_init(request: &mut api::PawsGetSpectrumBatchRequest) {
                request_owner_operator_init(request);
                if request.owner.as_mut().unwrap().operator.as_mut().unwrap().org.is_none() {
                    request.owner.as_mut().unwrap().operator.as_mut().unwrap().org = Some(Default::default());
                }
            }
            
            fn request_owner_operator_tel_init(request: &mut api::PawsGetSpectrumBatchRequest) {
                request_owner_operator_init(request);
                if request.owner.as_mut().unwrap().operator.as_mut().unwrap().tel.is_none() {
                    request.owner.as_mut().unwrap().operator.as_mut().unwrap().tel = Some(Default::default());
                }
            }
            
            fn request_owner_owner_adr_init(request: &mut api::PawsGetSpectrumBatchRequest) {
                request_owner_owner_init(request);
                if request.owner.as_mut().unwrap().owner.as_mut().unwrap().adr.is_none() {
                    request.owner.as_mut().unwrap().owner.as_mut().unwrap().adr = Some(Default::default());
                }
            }
            
            fn request_owner_owner_email_init(request: &mut api::PawsGetSpectrumBatchRequest) {
                request_owner_owner_init(request);
                if request.owner.as_mut().unwrap().owner.as_mut().unwrap().email.is_none() {
                    request.owner.as_mut().unwrap().owner.as_mut().unwrap().email = Some(Default::default());
                }
            }
            
            fn request_owner_owner_init(request: &mut api::PawsGetSpectrumBatchRequest) {
                request_owner_init(request);
                if request.owner.as_mut().unwrap().owner.is_none() {
                    request.owner.as_mut().unwrap().owner = Some(Default::default());
                }
            }
            
            fn request_owner_owner_org_init(request: &mut api::PawsGetSpectrumBatchRequest) {
                request_owner_owner_init(request);
                if request.owner.as_mut().unwrap().owner.as_mut().unwrap().org.is_none() {
                    request.owner.as_mut().unwrap().owner.as_mut().unwrap().org = Some(Default::default());
                }
            }
            
            fn request_owner_owner_tel_init(request: &mut api::PawsGetSpectrumBatchRequest) {
                request_owner_owner_init(request);
                if request.owner.as_mut().unwrap().owner.as_mut().unwrap().tel.is_none() {
                    request.owner.as_mut().unwrap().owner.as_mut().unwrap().tel = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "device-desc.etsi-en-device-type" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().etsi_en_device_type = Some(value.unwrap_or("").to_string());
                    },
                "device-desc.fcc-id" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().fcc_id = Some(value.unwrap_or("").to_string());
                    },
                "device-desc.serial-number" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().serial_number = Some(value.unwrap_or("").to_string());
                    },
                "device-desc.etsi-en-technology-id" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().etsi_en_technology_id = Some(value.unwrap_or("").to_string());
                    },
                "device-desc.fcc-tvbd-device-type" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().fcc_tvbd_device_type = Some(value.unwrap_or("").to_string());
                    },
                "device-desc.etsi-en-device-category" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().etsi_en_device_category = Some(value.unwrap_or("").to_string());
                    },
                "device-desc.ruleset-ids" => {
                        request_device_desc_init(&mut request);
                        if request.device_desc.as_mut().unwrap().ruleset_ids.is_none() {
                           request.device_desc.as_mut().unwrap().ruleset_ids = Some(Default::default());
                        }
                                        request.device_desc.as_mut().unwrap().ruleset_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "device-desc.etsi-en-device-emissions-class" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().etsi_en_device_emissions_class = Some(value.unwrap_or("").to_string());
                    },
                "device-desc.manufacturer-id" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().manufacturer_id = Some(value.unwrap_or("").to_string());
                    },
                "device-desc.model-id" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().model_id = Some(value.unwrap_or("").to_string());
                    },
                "version" => {
                        request_device_desc_init(&mut request);
                        request.version = Some(value.unwrap_or("").to_string());
                    },
                "antenna.height-type" => {
                        request_antenna_init(&mut request);
                        request.antenna.as_mut().unwrap().height_type = Some(value.unwrap_or("").to_string());
                    },
                "antenna.height-uncertainty" => {
                        request_antenna_init(&mut request);
                        request.antenna.as_mut().unwrap().height_uncertainty = Some(arg_from_str(value.unwrap_or("0.0"), err, "antenna.height-uncertainty", "number"));
                    },
                "antenna.height" => {
                        request_antenna_init(&mut request);
                        request.antenna.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("0.0"), err, "antenna.height", "number"));
                    },
                "request-type" => {
                        request_antenna_init(&mut request);
                        request.request_type = Some(value.unwrap_or("").to_string());
                    },
                "master-device-desc.etsi-en-device-type" => {
                        request_master_device_desc_init(&mut request);
                        request.master_device_desc.as_mut().unwrap().etsi_en_device_type = Some(value.unwrap_or("").to_string());
                    },
                "master-device-desc.fcc-id" => {
                        request_master_device_desc_init(&mut request);
                        request.master_device_desc.as_mut().unwrap().fcc_id = Some(value.unwrap_or("").to_string());
                    },
                "master-device-desc.serial-number" => {
                        request_master_device_desc_init(&mut request);
                        request.master_device_desc.as_mut().unwrap().serial_number = Some(value.unwrap_or("").to_string());
                    },
                "master-device-desc.etsi-en-technology-id" => {
                        request_master_device_desc_init(&mut request);
                        request.master_device_desc.as_mut().unwrap().etsi_en_technology_id = Some(value.unwrap_or("").to_string());
                    },
                "master-device-desc.fcc-tvbd-device-type" => {
                        request_master_device_desc_init(&mut request);
                        request.master_device_desc.as_mut().unwrap().fcc_tvbd_device_type = Some(value.unwrap_or("").to_string());
                    },
                "master-device-desc.etsi-en-device-category" => {
                        request_master_device_desc_init(&mut request);
                        request.master_device_desc.as_mut().unwrap().etsi_en_device_category = Some(value.unwrap_or("").to_string());
                    },
                "master-device-desc.ruleset-ids" => {
                        request_master_device_desc_init(&mut request);
                        if request.master_device_desc.as_mut().unwrap().ruleset_ids.is_none() {
                           request.master_device_desc.as_mut().unwrap().ruleset_ids = Some(Default::default());
                        }
                                        request.master_device_desc.as_mut().unwrap().ruleset_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "master-device-desc.etsi-en-device-emissions-class" => {
                        request_master_device_desc_init(&mut request);
                        request.master_device_desc.as_mut().unwrap().etsi_en_device_emissions_class = Some(value.unwrap_or("").to_string());
                    },
                "master-device-desc.manufacturer-id" => {
                        request_master_device_desc_init(&mut request);
                        request.master_device_desc.as_mut().unwrap().manufacturer_id = Some(value.unwrap_or("").to_string());
                    },
                "master-device-desc.model-id" => {
                        request_master_device_desc_init(&mut request);
                        request.master_device_desc.as_mut().unwrap().model_id = Some(value.unwrap_or("").to_string());
                    },
                "owner.operator.org.text" => {
                        request_owner_operator_org_init(&mut request);
                        request.owner.as_mut().unwrap().operator.as_mut().unwrap().org.as_mut().unwrap().text = Some(value.unwrap_or("").to_string());
                    },
                "owner.operator.tel.uri" => {
                        request_owner_operator_tel_init(&mut request);
                        request.owner.as_mut().unwrap().operator.as_mut().unwrap().tel.as_mut().unwrap().uri = Some(value.unwrap_or("").to_string());
                    },
                "owner.operator.email.text" => {
                        request_owner_operator_email_init(&mut request);
                        request.owner.as_mut().unwrap().operator.as_mut().unwrap().email.as_mut().unwrap().text = Some(value.unwrap_or("").to_string());
                    },
                "owner.operator.fn" => {
                        request_owner_operator_email_init(&mut request);
                        request.owner.as_mut().unwrap().operator.as_mut().unwrap().fn_ = Some(value.unwrap_or("").to_string());
                    },
                "owner.operator.adr.code" => {
                        request_owner_operator_adr_init(&mut request);
                        request.owner.as_mut().unwrap().operator.as_mut().unwrap().adr.as_mut().unwrap().code = Some(value.unwrap_or("").to_string());
                    },
                "owner.operator.adr.locality" => {
                        request_owner_operator_adr_init(&mut request);
                        request.owner.as_mut().unwrap().operator.as_mut().unwrap().adr.as_mut().unwrap().locality = Some(value.unwrap_or("").to_string());
                    },
                "owner.operator.adr.country" => {
                        request_owner_operator_adr_init(&mut request);
                        request.owner.as_mut().unwrap().operator.as_mut().unwrap().adr.as_mut().unwrap().country = Some(value.unwrap_or("").to_string());
                    },
                "owner.operator.adr.region" => {
                        request_owner_operator_adr_init(&mut request);
                        request.owner.as_mut().unwrap().operator.as_mut().unwrap().adr.as_mut().unwrap().region = Some(value.unwrap_or("").to_string());
                    },
                "owner.operator.adr.pobox" => {
                        request_owner_operator_adr_init(&mut request);
                        request.owner.as_mut().unwrap().operator.as_mut().unwrap().adr.as_mut().unwrap().pobox = Some(value.unwrap_or("").to_string());
                    },
                "owner.operator.adr.street" => {
                        request_owner_operator_adr_init(&mut request);
                        request.owner.as_mut().unwrap().operator.as_mut().unwrap().adr.as_mut().unwrap().street = Some(value.unwrap_or("").to_string());
                    },
                "owner.owner.org.text" => {
                        request_owner_owner_org_init(&mut request);
                        request.owner.as_mut().unwrap().owner.as_mut().unwrap().org.as_mut().unwrap().text = Some(value.unwrap_or("").to_string());
                    },
                "owner.owner.tel.uri" => {
                        request_owner_owner_tel_init(&mut request);
                        request.owner.as_mut().unwrap().owner.as_mut().unwrap().tel.as_mut().unwrap().uri = Some(value.unwrap_or("").to_string());
                    },
                "owner.owner.email.text" => {
                        request_owner_owner_email_init(&mut request);
                        request.owner.as_mut().unwrap().owner.as_mut().unwrap().email.as_mut().unwrap().text = Some(value.unwrap_or("").to_string());
                    },
                "owner.owner.fn" => {
                        request_owner_owner_email_init(&mut request);
                        request.owner.as_mut().unwrap().owner.as_mut().unwrap().fn_ = Some(value.unwrap_or("").to_string());
                    },
                "owner.owner.adr.code" => {
                        request_owner_owner_adr_init(&mut request);
                        request.owner.as_mut().unwrap().owner.as_mut().unwrap().adr.as_mut().unwrap().code = Some(value.unwrap_or("").to_string());
                    },
                "owner.owner.adr.locality" => {
                        request_owner_owner_adr_init(&mut request);
                        request.owner.as_mut().unwrap().owner.as_mut().unwrap().adr.as_mut().unwrap().locality = Some(value.unwrap_or("").to_string());
                    },
                "owner.owner.adr.country" => {
                        request_owner_owner_adr_init(&mut request);
                        request.owner.as_mut().unwrap().owner.as_mut().unwrap().adr.as_mut().unwrap().country = Some(value.unwrap_or("").to_string());
                    },
                "owner.owner.adr.region" => {
                        request_owner_owner_adr_init(&mut request);
                        request.owner.as_mut().unwrap().owner.as_mut().unwrap().adr.as_mut().unwrap().region = Some(value.unwrap_or("").to_string());
                    },
                "owner.owner.adr.pobox" => {
                        request_owner_owner_adr_init(&mut request);
                        request.owner.as_mut().unwrap().owner.as_mut().unwrap().adr.as_mut().unwrap().pobox = Some(value.unwrap_or("").to_string());
                    },
                "owner.owner.adr.street" => {
                        request_owner_owner_adr_init(&mut request);
                        request.owner.as_mut().unwrap().owner.as_mut().unwrap().adr.as_mut().unwrap().street = Some(value.unwrap_or("").to_string());
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _paws_init(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::PawsInitRequest::default();
        let mut call = self.hub.paws().init(&request);
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
            
            fn request_location_point_center_init(request: &mut api::PawsInitRequest) {
                request_location_point_init(request);
                if request.location.as_mut().unwrap().point.as_mut().unwrap().center.is_none() {
                    request.location.as_mut().unwrap().point.as_mut().unwrap().center = Some(Default::default());
                }
            }
            
            fn request_location_point_init(request: &mut api::PawsInitRequest) {
                request_location_init(request);
                if request.location.as_mut().unwrap().point.is_none() {
                    request.location.as_mut().unwrap().point = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "device-desc.etsi-en-device-type" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().etsi_en_device_type = Some(value.unwrap_or("").to_string());
                    },
                "device-desc.fcc-id" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().fcc_id = Some(value.unwrap_or("").to_string());
                    },
                "device-desc.serial-number" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().serial_number = Some(value.unwrap_or("").to_string());
                    },
                "device-desc.etsi-en-technology-id" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().etsi_en_technology_id = Some(value.unwrap_or("").to_string());
                    },
                "device-desc.fcc-tvbd-device-type" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().fcc_tvbd_device_type = Some(value.unwrap_or("").to_string());
                    },
                "device-desc.etsi-en-device-category" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().etsi_en_device_category = Some(value.unwrap_or("").to_string());
                    },
                "device-desc.ruleset-ids" => {
                        request_device_desc_init(&mut request);
                        if request.device_desc.as_mut().unwrap().ruleset_ids.is_none() {
                           request.device_desc.as_mut().unwrap().ruleset_ids = Some(Default::default());
                        }
                                        request.device_desc.as_mut().unwrap().ruleset_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "device-desc.etsi-en-device-emissions-class" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().etsi_en_device_emissions_class = Some(value.unwrap_or("").to_string());
                    },
                "device-desc.manufacturer-id" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().manufacturer_id = Some(value.unwrap_or("").to_string());
                    },
                "device-desc.model-id" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().model_id = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_device_desc_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "location.confidence" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().confidence = Some(arg_from_str(value.unwrap_or("-0"), err, "location.confidence", "integer"));
                    },
                "location.point.semi-minor-axis" => {
                        request_location_point_init(&mut request);
                        request.location.as_mut().unwrap().point.as_mut().unwrap().semi_minor_axis = Some(arg_from_str(value.unwrap_or("0.0"), err, "location.point.semi-minor-axis", "number"));
                    },
                "location.point.center.latitude" => {
                        request_location_point_center_init(&mut request);
                        request.location.as_mut().unwrap().point.as_mut().unwrap().center.as_mut().unwrap().latitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "location.point.center.latitude", "number"));
                    },
                "location.point.center.longitude" => {
                        request_location_point_center_init(&mut request);
                        request.location.as_mut().unwrap().point.as_mut().unwrap().center.as_mut().unwrap().longitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "location.point.center.longitude", "number"));
                    },
                "location.point.semi-major-axis" => {
                        request_location_point_center_init(&mut request);
                        request.location.as_mut().unwrap().point.as_mut().unwrap().semi_major_axis = Some(arg_from_str(value.unwrap_or("0.0"), err, "location.point.semi-major-axis", "number"));
                    },
                "location.point.orientation" => {
                        request_location_point_center_init(&mut request);
                        request.location.as_mut().unwrap().point.as_mut().unwrap().orientation = Some(arg_from_str(value.unwrap_or("0.0"), err, "location.point.orientation", "number"));
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _paws_notify_spectrum_use(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::PawsNotifySpectrumUseRequest::default();
        let mut call = self.hub.paws().notify_spectrum_use(&request);
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
            
            fn request_location_point_center_init(request: &mut api::PawsNotifySpectrumUseRequest) {
                request_location_point_init(request);
                if request.location.as_mut().unwrap().point.as_mut().unwrap().center.is_none() {
                    request.location.as_mut().unwrap().point.as_mut().unwrap().center = Some(Default::default());
                }
            }
            
            fn request_location_point_init(request: &mut api::PawsNotifySpectrumUseRequest) {
                request_location_init(request);
                if request.location.as_mut().unwrap().point.is_none() {
                    request.location.as_mut().unwrap().point = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "device-desc.etsi-en-device-type" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().etsi_en_device_type = Some(value.unwrap_or("").to_string());
                    },
                "device-desc.fcc-id" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().fcc_id = Some(value.unwrap_or("").to_string());
                    },
                "device-desc.serial-number" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().serial_number = Some(value.unwrap_or("").to_string());
                    },
                "device-desc.etsi-en-technology-id" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().etsi_en_technology_id = Some(value.unwrap_or("").to_string());
                    },
                "device-desc.fcc-tvbd-device-type" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().fcc_tvbd_device_type = Some(value.unwrap_or("").to_string());
                    },
                "device-desc.etsi-en-device-category" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().etsi_en_device_category = Some(value.unwrap_or("").to_string());
                    },
                "device-desc.ruleset-ids" => {
                        request_device_desc_init(&mut request);
                        if request.device_desc.as_mut().unwrap().ruleset_ids.is_none() {
                           request.device_desc.as_mut().unwrap().ruleset_ids = Some(Default::default());
                        }
                                        request.device_desc.as_mut().unwrap().ruleset_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "device-desc.etsi-en-device-emissions-class" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().etsi_en_device_emissions_class = Some(value.unwrap_or("").to_string());
                    },
                "device-desc.manufacturer-id" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().manufacturer_id = Some(value.unwrap_or("").to_string());
                    },
                "device-desc.model-id" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().model_id = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_device_desc_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "location.confidence" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().confidence = Some(arg_from_str(value.unwrap_or("-0"), err, "location.confidence", "integer"));
                    },
                "location.point.semi-minor-axis" => {
                        request_location_point_init(&mut request);
                        request.location.as_mut().unwrap().point.as_mut().unwrap().semi_minor_axis = Some(arg_from_str(value.unwrap_or("0.0"), err, "location.point.semi-minor-axis", "number"));
                    },
                "location.point.center.latitude" => {
                        request_location_point_center_init(&mut request);
                        request.location.as_mut().unwrap().point.as_mut().unwrap().center.as_mut().unwrap().latitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "location.point.center.latitude", "number"));
                    },
                "location.point.center.longitude" => {
                        request_location_point_center_init(&mut request);
                        request.location.as_mut().unwrap().point.as_mut().unwrap().center.as_mut().unwrap().longitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "location.point.center.longitude", "number"));
                    },
                "location.point.semi-major-axis" => {
                        request_location_point_center_init(&mut request);
                        request.location.as_mut().unwrap().point.as_mut().unwrap().semi_major_axis = Some(arg_from_str(value.unwrap_or("0.0"), err, "location.point.semi-major-axis", "number"));
                    },
                "location.point.orientation" => {
                        request_location_point_center_init(&mut request);
                        request.location.as_mut().unwrap().point.as_mut().unwrap().orientation = Some(arg_from_str(value.unwrap_or("0.0"), err, "location.point.orientation", "number"));
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _paws_register(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::PawsRegisterRequest::default();
        let mut call = self.hub.paws().register(&request);
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
            
            fn request_device_owner_operator_adr_init(request: &mut api::PawsRegisterRequest) {
                request_device_owner_operator_init(request);
                if request.device_owner.as_mut().unwrap().operator.as_mut().unwrap().adr.is_none() {
                    request.device_owner.as_mut().unwrap().operator.as_mut().unwrap().adr = Some(Default::default());
                }
            }
            
            fn request_device_owner_operator_email_init(request: &mut api::PawsRegisterRequest) {
                request_device_owner_operator_init(request);
                if request.device_owner.as_mut().unwrap().operator.as_mut().unwrap().email.is_none() {
                    request.device_owner.as_mut().unwrap().operator.as_mut().unwrap().email = Some(Default::default());
                }
            }
            
            fn request_device_owner_operator_init(request: &mut api::PawsRegisterRequest) {
                request_device_owner_init(request);
                if request.device_owner.as_mut().unwrap().operator.is_none() {
                    request.device_owner.as_mut().unwrap().operator = Some(Default::default());
                }
            }
            
            fn request_device_owner_operator_org_init(request: &mut api::PawsRegisterRequest) {
                request_device_owner_operator_init(request);
                if request.device_owner.as_mut().unwrap().operator.as_mut().unwrap().org.is_none() {
                    request.device_owner.as_mut().unwrap().operator.as_mut().unwrap().org = Some(Default::default());
                }
            }
            
            fn request_device_owner_operator_tel_init(request: &mut api::PawsRegisterRequest) {
                request_device_owner_operator_init(request);
                if request.device_owner.as_mut().unwrap().operator.as_mut().unwrap().tel.is_none() {
                    request.device_owner.as_mut().unwrap().operator.as_mut().unwrap().tel = Some(Default::default());
                }
            }
            
            fn request_device_owner_owner_adr_init(request: &mut api::PawsRegisterRequest) {
                request_device_owner_owner_init(request);
                if request.device_owner.as_mut().unwrap().owner.as_mut().unwrap().adr.is_none() {
                    request.device_owner.as_mut().unwrap().owner.as_mut().unwrap().adr = Some(Default::default());
                }
            }
            
            fn request_device_owner_owner_email_init(request: &mut api::PawsRegisterRequest) {
                request_device_owner_owner_init(request);
                if request.device_owner.as_mut().unwrap().owner.as_mut().unwrap().email.is_none() {
                    request.device_owner.as_mut().unwrap().owner.as_mut().unwrap().email = Some(Default::default());
                }
            }
            
            fn request_device_owner_owner_init(request: &mut api::PawsRegisterRequest) {
                request_device_owner_init(request);
                if request.device_owner.as_mut().unwrap().owner.is_none() {
                    request.device_owner.as_mut().unwrap().owner = Some(Default::default());
                }
            }
            
            fn request_device_owner_owner_org_init(request: &mut api::PawsRegisterRequest) {
                request_device_owner_owner_init(request);
                if request.device_owner.as_mut().unwrap().owner.as_mut().unwrap().org.is_none() {
                    request.device_owner.as_mut().unwrap().owner.as_mut().unwrap().org = Some(Default::default());
                }
            }
            
            fn request_device_owner_owner_tel_init(request: &mut api::PawsRegisterRequest) {
                request_device_owner_owner_init(request);
                if request.device_owner.as_mut().unwrap().owner.as_mut().unwrap().tel.is_none() {
                    request.device_owner.as_mut().unwrap().owner.as_mut().unwrap().tel = Some(Default::default());
                }
            }
            
            fn request_location_init(request: &mut api::PawsRegisterRequest) {
                if request.location.is_none() {
                    request.location = Some(Default::default());
                }
            }
            
            fn request_location_point_center_init(request: &mut api::PawsRegisterRequest) {
                request_location_point_init(request);
                if request.location.as_mut().unwrap().point.as_mut().unwrap().center.is_none() {
                    request.location.as_mut().unwrap().point.as_mut().unwrap().center = Some(Default::default());
                }
            }
            
            fn request_location_point_init(request: &mut api::PawsRegisterRequest) {
                request_location_init(request);
                if request.location.as_mut().unwrap().point.is_none() {
                    request.location.as_mut().unwrap().point = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "device-desc.etsi-en-device-type" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().etsi_en_device_type = Some(value.unwrap_or("").to_string());
                    },
                "device-desc.fcc-id" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().fcc_id = Some(value.unwrap_or("").to_string());
                    },
                "device-desc.serial-number" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().serial_number = Some(value.unwrap_or("").to_string());
                    },
                "device-desc.etsi-en-technology-id" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().etsi_en_technology_id = Some(value.unwrap_or("").to_string());
                    },
                "device-desc.fcc-tvbd-device-type" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().fcc_tvbd_device_type = Some(value.unwrap_or("").to_string());
                    },
                "device-desc.etsi-en-device-category" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().etsi_en_device_category = Some(value.unwrap_or("").to_string());
                    },
                "device-desc.ruleset-ids" => {
                        request_device_desc_init(&mut request);
                        if request.device_desc.as_mut().unwrap().ruleset_ids.is_none() {
                           request.device_desc.as_mut().unwrap().ruleset_ids = Some(Default::default());
                        }
                                        request.device_desc.as_mut().unwrap().ruleset_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "device-desc.etsi-en-device-emissions-class" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().etsi_en_device_emissions_class = Some(value.unwrap_or("").to_string());
                    },
                "device-desc.manufacturer-id" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().manufacturer_id = Some(value.unwrap_or("").to_string());
                    },
                "device-desc.model-id" => {
                        request_device_desc_init(&mut request);
                        request.device_desc.as_mut().unwrap().model_id = Some(value.unwrap_or("").to_string());
                    },
                "antenna.height-type" => {
                        request_antenna_init(&mut request);
                        request.antenna.as_mut().unwrap().height_type = Some(value.unwrap_or("").to_string());
                    },
                "antenna.height-uncertainty" => {
                        request_antenna_init(&mut request);
                        request.antenna.as_mut().unwrap().height_uncertainty = Some(arg_from_str(value.unwrap_or("0.0"), err, "antenna.height-uncertainty", "number"));
                    },
                "antenna.height" => {
                        request_antenna_init(&mut request);
                        request.antenna.as_mut().unwrap().height = Some(arg_from_str(value.unwrap_or("0.0"), err, "antenna.height", "number"));
                    },
                "device-owner.operator.org.text" => {
                        request_device_owner_operator_org_init(&mut request);
                        request.device_owner.as_mut().unwrap().operator.as_mut().unwrap().org.as_mut().unwrap().text = Some(value.unwrap_or("").to_string());
                    },
                "device-owner.operator.tel.uri" => {
                        request_device_owner_operator_tel_init(&mut request);
                        request.device_owner.as_mut().unwrap().operator.as_mut().unwrap().tel.as_mut().unwrap().uri = Some(value.unwrap_or("").to_string());
                    },
                "device-owner.operator.email.text" => {
                        request_device_owner_operator_email_init(&mut request);
                        request.device_owner.as_mut().unwrap().operator.as_mut().unwrap().email.as_mut().unwrap().text = Some(value.unwrap_or("").to_string());
                    },
                "device-owner.operator.fn" => {
                        request_device_owner_operator_email_init(&mut request);
                        request.device_owner.as_mut().unwrap().operator.as_mut().unwrap().fn_ = Some(value.unwrap_or("").to_string());
                    },
                "device-owner.operator.adr.code" => {
                        request_device_owner_operator_adr_init(&mut request);
                        request.device_owner.as_mut().unwrap().operator.as_mut().unwrap().adr.as_mut().unwrap().code = Some(value.unwrap_or("").to_string());
                    },
                "device-owner.operator.adr.locality" => {
                        request_device_owner_operator_adr_init(&mut request);
                        request.device_owner.as_mut().unwrap().operator.as_mut().unwrap().adr.as_mut().unwrap().locality = Some(value.unwrap_or("").to_string());
                    },
                "device-owner.operator.adr.country" => {
                        request_device_owner_operator_adr_init(&mut request);
                        request.device_owner.as_mut().unwrap().operator.as_mut().unwrap().adr.as_mut().unwrap().country = Some(value.unwrap_or("").to_string());
                    },
                "device-owner.operator.adr.region" => {
                        request_device_owner_operator_adr_init(&mut request);
                        request.device_owner.as_mut().unwrap().operator.as_mut().unwrap().adr.as_mut().unwrap().region = Some(value.unwrap_or("").to_string());
                    },
                "device-owner.operator.adr.pobox" => {
                        request_device_owner_operator_adr_init(&mut request);
                        request.device_owner.as_mut().unwrap().operator.as_mut().unwrap().adr.as_mut().unwrap().pobox = Some(value.unwrap_or("").to_string());
                    },
                "device-owner.operator.adr.street" => {
                        request_device_owner_operator_adr_init(&mut request);
                        request.device_owner.as_mut().unwrap().operator.as_mut().unwrap().adr.as_mut().unwrap().street = Some(value.unwrap_or("").to_string());
                    },
                "device-owner.owner.org.text" => {
                        request_device_owner_owner_org_init(&mut request);
                        request.device_owner.as_mut().unwrap().owner.as_mut().unwrap().org.as_mut().unwrap().text = Some(value.unwrap_or("").to_string());
                    },
                "device-owner.owner.tel.uri" => {
                        request_device_owner_owner_tel_init(&mut request);
                        request.device_owner.as_mut().unwrap().owner.as_mut().unwrap().tel.as_mut().unwrap().uri = Some(value.unwrap_or("").to_string());
                    },
                "device-owner.owner.email.text" => {
                        request_device_owner_owner_email_init(&mut request);
                        request.device_owner.as_mut().unwrap().owner.as_mut().unwrap().email.as_mut().unwrap().text = Some(value.unwrap_or("").to_string());
                    },
                "device-owner.owner.fn" => {
                        request_device_owner_owner_email_init(&mut request);
                        request.device_owner.as_mut().unwrap().owner.as_mut().unwrap().fn_ = Some(value.unwrap_or("").to_string());
                    },
                "device-owner.owner.adr.code" => {
                        request_device_owner_owner_adr_init(&mut request);
                        request.device_owner.as_mut().unwrap().owner.as_mut().unwrap().adr.as_mut().unwrap().code = Some(value.unwrap_or("").to_string());
                    },
                "device-owner.owner.adr.locality" => {
                        request_device_owner_owner_adr_init(&mut request);
                        request.device_owner.as_mut().unwrap().owner.as_mut().unwrap().adr.as_mut().unwrap().locality = Some(value.unwrap_or("").to_string());
                    },
                "device-owner.owner.adr.country" => {
                        request_device_owner_owner_adr_init(&mut request);
                        request.device_owner.as_mut().unwrap().owner.as_mut().unwrap().adr.as_mut().unwrap().country = Some(value.unwrap_or("").to_string());
                    },
                "device-owner.owner.adr.region" => {
                        request_device_owner_owner_adr_init(&mut request);
                        request.device_owner.as_mut().unwrap().owner.as_mut().unwrap().adr.as_mut().unwrap().region = Some(value.unwrap_or("").to_string());
                    },
                "device-owner.owner.adr.pobox" => {
                        request_device_owner_owner_adr_init(&mut request);
                        request.device_owner.as_mut().unwrap().owner.as_mut().unwrap().adr.as_mut().unwrap().pobox = Some(value.unwrap_or("").to_string());
                    },
                "device-owner.owner.adr.street" => {
                        request_device_owner_owner_adr_init(&mut request);
                        request.device_owner.as_mut().unwrap().owner.as_mut().unwrap().adr.as_mut().unwrap().street = Some(value.unwrap_or("").to_string());
                    },
                "version" => {
                        request_device_owner_init(&mut request);
                        request.version = Some(value.unwrap_or("").to_string());
                    },
                "location.confidence" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().confidence = Some(arg_from_str(value.unwrap_or("-0"), err, "location.confidence", "integer"));
                    },
                "location.point.semi-minor-axis" => {
                        request_location_point_init(&mut request);
                        request.location.as_mut().unwrap().point.as_mut().unwrap().semi_minor_axis = Some(arg_from_str(value.unwrap_or("0.0"), err, "location.point.semi-minor-axis", "number"));
                    },
                "location.point.center.latitude" => {
                        request_location_point_center_init(&mut request);
                        request.location.as_mut().unwrap().point.as_mut().unwrap().center.as_mut().unwrap().latitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "location.point.center.latitude", "number"));
                    },
                "location.point.center.longitude" => {
                        request_location_point_center_init(&mut request);
                        request.location.as_mut().unwrap().point.as_mut().unwrap().center.as_mut().unwrap().longitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "location.point.center.longitude", "number"));
                    },
                "location.point.semi-major-axis" => {
                        request_location_point_center_init(&mut request);
                        request.location.as_mut().unwrap().point.as_mut().unwrap().semi_major_axis = Some(arg_from_str(value.unwrap_or("0.0"), err, "location.point.semi-major-axis", "number"));
                    },
                "location.point.orientation" => {
                        request_location_point_center_init(&mut request);
                        request.location.as_mut().unwrap().point.as_mut().unwrap().orientation = Some(arg_from_str(value.unwrap_or("0.0"), err, "location.point.orientation", "number"));
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _paws_verify_device(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::PawsVerifyDeviceRequest::default();
        let mut call = self.hub.paws().verify_device(&request);
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

            match cmn::application_secret_from_directory(&config_dir, "spectrum1-explorer-secret.json", 
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
                                          program_name: "spectrum1-explorer",
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
            hub: api::Spectrum::new(client, auth),
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