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
extern crate google_androidpublisher2 as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  androidpublisher2 [options] edits apklistings-delete <package-name> <edit-id> <apk-version-code> <language> [-p <v>]...
  androidpublisher2 [options] edits apklistings-deleteall <package-name> <edit-id> <apk-version-code> [-p <v>]...
  androidpublisher2 [options] edits apklistings-get <package-name> <edit-id> <apk-version-code> <language> [-p <v>]... [-o <out>]
  androidpublisher2 [options] edits apklistings-list <package-name> <edit-id> <apk-version-code> [-p <v>]... [-o <out>]
  androidpublisher2 [options] edits apklistings-patch <package-name> <edit-id> <apk-version-code> <language> -r <kv>... [-p <v>]... [-o <out>]
  androidpublisher2 [options] edits apklistings-update <package-name> <edit-id> <apk-version-code> <language> -r <kv>... [-p <v>]... [-o <out>]
  androidpublisher2 [options] edits apks-addexternallyhosted <package-name> <edit-id> -r <kv>... [-p <v>]... [-o <out>]
  androidpublisher2 [options] edits apks-list <package-name> <edit-id> [-p <v>]... [-o <out>]
  androidpublisher2 [options] edits apks-upload <package-name> <edit-id> -u (simple|resumable) <file> <mime> [-p <v>]... [-o <out>]
  androidpublisher2 [options] edits commit <package-name> <edit-id> [-p <v>]... [-o <out>]
  androidpublisher2 [options] edits delete <package-name> <edit-id> [-p <v>]...
  androidpublisher2 [options] edits details-get <package-name> <edit-id> [-p <v>]... [-o <out>]
  androidpublisher2 [options] edits details-patch <package-name> <edit-id> -r <kv>... [-p <v>]... [-o <out>]
  androidpublisher2 [options] edits details-update <package-name> <edit-id> -r <kv>... [-p <v>]... [-o <out>]
  androidpublisher2 [options] edits expansionfiles-get <package-name> <edit-id> <apk-version-code> <expansion-file-type> [-p <v>]... [-o <out>]
  androidpublisher2 [options] edits expansionfiles-patch <package-name> <edit-id> <apk-version-code> <expansion-file-type> -r <kv>... [-p <v>]... [-o <out>]
  androidpublisher2 [options] edits expansionfiles-update <package-name> <edit-id> <apk-version-code> <expansion-file-type> -r <kv>... [-p <v>]... [-o <out>]
  androidpublisher2 [options] edits expansionfiles-upload <package-name> <edit-id> <apk-version-code> <expansion-file-type> -u (simple|resumable) <file> <mime> [-p <v>]... [-o <out>]
  androidpublisher2 [options] edits get <package-name> <edit-id> [-p <v>]... [-o <out>]
  androidpublisher2 [options] edits images-delete <package-name> <edit-id> <language> <image-type> <image-id> [-p <v>]...
  androidpublisher2 [options] edits images-deleteall <package-name> <edit-id> <language> <image-type> [-p <v>]... [-o <out>]
  androidpublisher2 [options] edits images-list <package-name> <edit-id> <language> <image-type> [-p <v>]... [-o <out>]
  androidpublisher2 [options] edits images-upload <package-name> <edit-id> <language> <image-type> -u (simple|resumable) <file> <mime> [-p <v>]... [-o <out>]
  androidpublisher2 [options] edits insert <package-name> -r <kv>... [-p <v>]... [-o <out>]
  androidpublisher2 [options] edits listings-delete <package-name> <edit-id> <language> [-p <v>]...
  androidpublisher2 [options] edits listings-deleteall <package-name> <edit-id> [-p <v>]...
  androidpublisher2 [options] edits listings-get <package-name> <edit-id> <language> [-p <v>]... [-o <out>]
  androidpublisher2 [options] edits listings-list <package-name> <edit-id> [-p <v>]... [-o <out>]
  androidpublisher2 [options] edits listings-patch <package-name> <edit-id> <language> -r <kv>... [-p <v>]... [-o <out>]
  androidpublisher2 [options] edits listings-update <package-name> <edit-id> <language> -r <kv>... [-p <v>]... [-o <out>]
  androidpublisher2 [options] edits testers-get <package-name> <edit-id> <track> [-p <v>]... [-o <out>]
  androidpublisher2 [options] edits testers-patch <package-name> <edit-id> <track> -r <kv>... [-p <v>]... [-o <out>]
  androidpublisher2 [options] edits testers-update <package-name> <edit-id> <track> -r <kv>... [-p <v>]... [-o <out>]
  androidpublisher2 [options] edits tracks-get <package-name> <edit-id> <track> [-p <v>]... [-o <out>]
  androidpublisher2 [options] edits tracks-list <package-name> <edit-id> [-p <v>]... [-o <out>]
  androidpublisher2 [options] edits tracks-patch <package-name> <edit-id> <track> -r <kv>... [-p <v>]... [-o <out>]
  androidpublisher2 [options] edits tracks-update <package-name> <edit-id> <track> -r <kv>... [-p <v>]... [-o <out>]
  androidpublisher2 [options] edits validate <package-name> <edit-id> [-p <v>]... [-o <out>]
  androidpublisher2 [options] inappproducts batch -r <kv>... [-p <v>]... [-o <out>]
  androidpublisher2 [options] inappproducts delete <package-name> <sku> [-p <v>]...
  androidpublisher2 [options] inappproducts get <package-name> <sku> [-p <v>]... [-o <out>]
  androidpublisher2 [options] inappproducts insert <package-name> -r <kv>... [-p <v>]... [-o <out>]
  androidpublisher2 [options] inappproducts list <package-name> [-p <v>]... [-o <out>]
  androidpublisher2 [options] inappproducts patch <package-name> <sku> -r <kv>... [-p <v>]... [-o <out>]
  androidpublisher2 [options] inappproducts update <package-name> <sku> -r <kv>... [-p <v>]... [-o <out>]
  androidpublisher2 [options] purchases products-get <package-name> <product-id> <token> [-p <v>]... [-o <out>]
  androidpublisher2 [options] purchases subscriptions-cancel <package-name> <subscription-id> <token> [-p <v>]...
  androidpublisher2 [options] purchases subscriptions-defer <package-name> <subscription-id> <token> -r <kv>... [-p <v>]... [-o <out>]
  androidpublisher2 [options] purchases subscriptions-get <package-name> <subscription-id> <token> [-p <v>]... [-o <out>]
  androidpublisher2 [options] purchases subscriptions-refund <package-name> <subscription-id> <token> [-p <v>]...
  androidpublisher2 [options] purchases subscriptions-revoke <package-name> <subscription-id> <token> [-p <v>]...
  androidpublisher2 --help

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
    hub: api::AndroidPublisher<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _edits_apklistings_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let apk_version_code: i32 = arg_from_str(&self.opt.arg_apk_version_code, err, "<apk-version-code>", "integer");
        let mut call = self.hub.edits().apklistings_delete(&self.opt.arg_package_name, &self.opt.arg_edit_id, apk_version_code, &self.opt.arg_language);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    None
                }
            }
        }
    }

    fn _edits_apklistings_deleteall(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let apk_version_code: i32 = arg_from_str(&self.opt.arg_apk_version_code, err, "<apk-version-code>", "integer");
        let mut call = self.hub.edits().apklistings_deleteall(&self.opt.arg_package_name, &self.opt.arg_edit_id, apk_version_code);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    None
                }
            }
        }
    }

    fn _edits_apklistings_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let apk_version_code: i32 = arg_from_str(&self.opt.arg_apk_version_code, err, "<apk-version-code>", "integer");
        let mut call = self.hub.edits().apklistings_get(&self.opt.arg_package_name, &self.opt.arg_edit_id, apk_version_code, &self.opt.arg_language);
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

    fn _edits_apklistings_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let apk_version_code: i32 = arg_from_str(&self.opt.arg_apk_version_code, err, "<apk-version-code>", "integer");
        let mut call = self.hub.edits().apklistings_list(&self.opt.arg_package_name, &self.opt.arg_edit_id, apk_version_code);
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

    fn _edits_apklistings_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::ApkListing = Default::default();
        let apk_version_code: i32 = arg_from_str(&self.opt.arg_apk_version_code, err, "<apk-version-code>", "integer");
        let mut call = self.hub.edits().apklistings_patch(&request, &self.opt.arg_package_name, &self.opt.arg_edit_id, apk_version_code, &self.opt.arg_language);
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
                "recent-changes" => {
                        request.recent_changes = Some(value.unwrap_or("").to_string());
                    },
                "language" => {
                        request.language = Some(value.unwrap_or("").to_string());
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

    fn _edits_apklistings_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::ApkListing = Default::default();
        let apk_version_code: i32 = arg_from_str(&self.opt.arg_apk_version_code, err, "<apk-version-code>", "integer");
        let mut call = self.hub.edits().apklistings_update(&request, &self.opt.arg_package_name, &self.opt.arg_edit_id, apk_version_code, &self.opt.arg_language);
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
                "recent-changes" => {
                        request.recent_changes = Some(value.unwrap_or("").to_string());
                    },
                "language" => {
                        request.language = Some(value.unwrap_or("").to_string());
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

    fn _edits_apks_addexternallyhosted(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::ApksAddExternallyHostedRequest = Default::default();
        let mut call = self.hub.edits().apks_addexternallyhosted(&request, &self.opt.arg_package_name, &self.opt.arg_edit_id);
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
            fn request_externally_hosted_apk_init(request: &mut api::ApksAddExternallyHostedRequest) {
                if request.externally_hosted_apk.is_none() {
                    request.externally_hosted_apk = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "externally-hosted-apk.icon-base64" => {
                        request_externally_hosted_apk_init(&mut request);
                        request.externally_hosted_apk.as_mut().unwrap().icon_base64 = value.unwrap_or("").to_string();
                    },
                "externally-hosted-apk.certificate-base64s" => {
                        request_externally_hosted_apk_init(&mut request);
                        request.externally_hosted_apk.as_mut().unwrap().certificate_base64s.push(value.unwrap_or("").to_string());
                    },
                "externally-hosted-apk.externally-hosted-url" => {
                        request_externally_hosted_apk_init(&mut request);
                        request.externally_hosted_apk.as_mut().unwrap().externally_hosted_url = value.unwrap_or("").to_string();
                    },
                "externally-hosted-apk.maximum-sdk" => {
                        request_externally_hosted_apk_init(&mut request);
                        request.externally_hosted_apk.as_mut().unwrap().maximum_sdk = arg_from_str(value.unwrap_or("-0"), err, "externally-hosted-apk.maximum-sdk", "integer");
                    },
                "externally-hosted-apk.file-sha256-base64" => {
                        request_externally_hosted_apk_init(&mut request);
                        request.externally_hosted_apk.as_mut().unwrap().file_sha256_base64 = value.unwrap_or("").to_string();
                    },
                "externally-hosted-apk.file-sha1-base64" => {
                        request_externally_hosted_apk_init(&mut request);
                        request.externally_hosted_apk.as_mut().unwrap().file_sha1_base64 = value.unwrap_or("").to_string();
                    },
                "externally-hosted-apk.uses-features" => {
                        request_externally_hosted_apk_init(&mut request);
                        request.externally_hosted_apk.as_mut().unwrap().uses_features.push(value.unwrap_or("").to_string());
                    },
                "externally-hosted-apk.file-size" => {
                        request_externally_hosted_apk_init(&mut request);
                        request.externally_hosted_apk.as_mut().unwrap().file_size = value.unwrap_or("").to_string();
                    },
                "externally-hosted-apk.version-name" => {
                        request_externally_hosted_apk_init(&mut request);
                        request.externally_hosted_apk.as_mut().unwrap().version_name = value.unwrap_or("").to_string();
                    },
                "externally-hosted-apk.version-code" => {
                        request_externally_hosted_apk_init(&mut request);
                        request.externally_hosted_apk.as_mut().unwrap().version_code = arg_from_str(value.unwrap_or("-0"), err, "externally-hosted-apk.version-code", "integer");
                    },
                "externally-hosted-apk.package-name" => {
                        request_externally_hosted_apk_init(&mut request);
                        request.externally_hosted_apk.as_mut().unwrap().package_name = value.unwrap_or("").to_string();
                    },
                "externally-hosted-apk.minimum-sdk" => {
                        request_externally_hosted_apk_init(&mut request);
                        request.externally_hosted_apk.as_mut().unwrap().minimum_sdk = arg_from_str(value.unwrap_or("-0"), err, "externally-hosted-apk.minimum-sdk", "integer");
                    },
                "externally-hosted-apk.application-label" => {
                        request_externally_hosted_apk_init(&mut request);
                        request.externally_hosted_apk.as_mut().unwrap().application_label = value.unwrap_or("").to_string();
                    },
                "externally-hosted-apk.native-codes" => {
                        request_externally_hosted_apk_init(&mut request);
                        request.externally_hosted_apk.as_mut().unwrap().native_codes.push(value.unwrap_or("").to_string());
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

    fn _edits_apks_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.edits().apks_list(&self.opt.arg_package_name, &self.opt.arg_edit_id);
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

    fn _edits_apks_upload(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.edits().apks_upload(&self.opt.arg_package_name, &self.opt.arg_edit_id);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _edits_commit(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.edits().commit(&self.opt.arg_package_name, &self.opt.arg_edit_id);
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

    fn _edits_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.edits().delete(&self.opt.arg_package_name, &self.opt.arg_edit_id);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    None
                }
            }
        }
    }

    fn _edits_details_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.edits().details_get(&self.opt.arg_package_name, &self.opt.arg_edit_id);
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

    fn _edits_details_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::AppDetails = Default::default();
        let mut call = self.hub.edits().details_patch(&request, &self.opt.arg_package_name, &self.opt.arg_edit_id);
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
                "contact-email" => {
                        request.contact_email = Some(value.unwrap_or("").to_string());
                    },
                "contact-phone" => {
                        request.contact_phone = Some(value.unwrap_or("").to_string());
                    },
                "contact-website" => {
                        request.contact_website = Some(value.unwrap_or("").to_string());
                    },
                "default-language" => {
                        request.default_language = Some(value.unwrap_or("").to_string());
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

    fn _edits_details_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::AppDetails = Default::default();
        let mut call = self.hub.edits().details_update(&request, &self.opt.arg_package_name, &self.opt.arg_edit_id);
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
                "contact-email" => {
                        request.contact_email = Some(value.unwrap_or("").to_string());
                    },
                "contact-phone" => {
                        request.contact_phone = Some(value.unwrap_or("").to_string());
                    },
                "contact-website" => {
                        request.contact_website = Some(value.unwrap_or("").to_string());
                    },
                "default-language" => {
                        request.default_language = Some(value.unwrap_or("").to_string());
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

    fn _edits_expansionfiles_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let apk_version_code: i32 = arg_from_str(&self.opt.arg_apk_version_code, err, "<apk-version-code>", "integer");
        let mut call = self.hub.edits().expansionfiles_get(&self.opt.arg_package_name, &self.opt.arg_edit_id, apk_version_code, &self.opt.arg_expansion_file_type);
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

    fn _edits_expansionfiles_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::ExpansionFile = Default::default();
        let apk_version_code: i32 = arg_from_str(&self.opt.arg_apk_version_code, err, "<apk-version-code>", "integer");
        let mut call = self.hub.edits().expansionfiles_patch(&request, &self.opt.arg_package_name, &self.opt.arg_edit_id, apk_version_code, &self.opt.arg_expansion_file_type);
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
                "references-version" => {
                        request.references_version = Some(arg_from_str(value.unwrap_or("-0"), err, "references-version", "integer"));
                    },
                "file-size" => {
                        request.file_size = Some(value.unwrap_or("").to_string());
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

    fn _edits_expansionfiles_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::ExpansionFile = Default::default();
        let apk_version_code: i32 = arg_from_str(&self.opt.arg_apk_version_code, err, "<apk-version-code>", "integer");
        let mut call = self.hub.edits().expansionfiles_update(&request, &self.opt.arg_package_name, &self.opt.arg_edit_id, apk_version_code, &self.opt.arg_expansion_file_type);
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
                "references-version" => {
                        request.references_version = Some(arg_from_str(value.unwrap_or("-0"), err, "references-version", "integer"));
                    },
                "file-size" => {
                        request.file_size = Some(value.unwrap_or("").to_string());
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

    fn _edits_expansionfiles_upload(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let apk_version_code: i32 = arg_from_str(&self.opt.arg_apk_version_code, err, "<apk-version-code>", "integer");
        let mut call = self.hub.edits().expansionfiles_upload(&self.opt.arg_package_name, &self.opt.arg_edit_id, apk_version_code, &self.opt.arg_expansion_file_type);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _edits_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.edits().get(&self.opt.arg_package_name, &self.opt.arg_edit_id);
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

    fn _edits_images_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.edits().images_delete(&self.opt.arg_package_name, &self.opt.arg_edit_id, &self.opt.arg_language, &self.opt.arg_image_type, &self.opt.arg_image_id);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    None
                }
            }
        }
    }

    fn _edits_images_deleteall(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.edits().images_deleteall(&self.opt.arg_package_name, &self.opt.arg_edit_id, &self.opt.arg_language, &self.opt.arg_image_type);
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

    fn _edits_images_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.edits().images_list(&self.opt.arg_package_name, &self.opt.arg_edit_id, &self.opt.arg_language, &self.opt.arg_image_type);
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

    fn _edits_images_upload(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.edits().images_upload(&self.opt.arg_package_name, &self.opt.arg_edit_id, &self.opt.arg_language, &self.opt.arg_image_type);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _edits_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::AppEdit = Default::default();
        let mut call = self.hub.edits().insert(&request, &self.opt.arg_package_name);
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
                "expiry-time-seconds" => {
                        request.expiry_time_seconds = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
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

    fn _edits_listings_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.edits().listings_delete(&self.opt.arg_package_name, &self.opt.arg_edit_id, &self.opt.arg_language);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    None
                }
            }
        }
    }

    fn _edits_listings_deleteall(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.edits().listings_deleteall(&self.opt.arg_package_name, &self.opt.arg_edit_id);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    None
                }
            }
        }
    }

    fn _edits_listings_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.edits().listings_get(&self.opt.arg_package_name, &self.opt.arg_edit_id, &self.opt.arg_language);
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

    fn _edits_listings_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.edits().listings_list(&self.opt.arg_package_name, &self.opt.arg_edit_id);
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

    fn _edits_listings_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Listing = Default::default();
        let mut call = self.hub.edits().listings_patch(&request, &self.opt.arg_package_name, &self.opt.arg_edit_id, &self.opt.arg_language);
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
                "short-description" => {
                        request.short_description = Some(value.unwrap_or("").to_string());
                    },
                "video" => {
                        request.video = Some(value.unwrap_or("").to_string());
                    },
                "full-description" => {
                        request.full_description = Some(value.unwrap_or("").to_string());
                    },
                "language" => {
                        request.language = Some(value.unwrap_or("").to_string());
                    },
                "title" => {
                        request.title = Some(value.unwrap_or("").to_string());
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

    fn _edits_listings_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Listing = Default::default();
        let mut call = self.hub.edits().listings_update(&request, &self.opt.arg_package_name, &self.opt.arg_edit_id, &self.opt.arg_language);
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
                "short-description" => {
                        request.short_description = Some(value.unwrap_or("").to_string());
                    },
                "video" => {
                        request.video = Some(value.unwrap_or("").to_string());
                    },
                "full-description" => {
                        request.full_description = Some(value.unwrap_or("").to_string());
                    },
                "language" => {
                        request.language = Some(value.unwrap_or("").to_string());
                    },
                "title" => {
                        request.title = Some(value.unwrap_or("").to_string());
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

    fn _edits_testers_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.edits().testers_get(&self.opt.arg_package_name, &self.opt.arg_edit_id, &self.opt.arg_track);
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

    fn _edits_testers_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Testers = Default::default();
        let mut call = self.hub.edits().testers_patch(&request, &self.opt.arg_package_name, &self.opt.arg_edit_id, &self.opt.arg_track);
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
                "google-groups" => {
                        if request.google_groups.is_none() {
                            request.google_groups = Some(Default::default());
                        }
                        request.google_groups.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "google-plus-communities" => {
                        if request.google_plus_communities.is_none() {
                            request.google_plus_communities = Some(Default::default());
                        }
                        request.google_plus_communities.as_mut().unwrap().push(value.unwrap_or("").to_string());
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

    fn _edits_testers_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Testers = Default::default();
        let mut call = self.hub.edits().testers_update(&request, &self.opt.arg_package_name, &self.opt.arg_edit_id, &self.opt.arg_track);
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
                "google-groups" => {
                        if request.google_groups.is_none() {
                            request.google_groups = Some(Default::default());
                        }
                        request.google_groups.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "google-plus-communities" => {
                        if request.google_plus_communities.is_none() {
                            request.google_plus_communities = Some(Default::default());
                        }
                        request.google_plus_communities.as_mut().unwrap().push(value.unwrap_or("").to_string());
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

    fn _edits_tracks_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.edits().tracks_get(&self.opt.arg_package_name, &self.opt.arg_edit_id, &self.opt.arg_track);
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

    fn _edits_tracks_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.edits().tracks_list(&self.opt.arg_package_name, &self.opt.arg_edit_id);
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

    fn _edits_tracks_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Track = Default::default();
        let mut call = self.hub.edits().tracks_patch(&request, &self.opt.arg_package_name, &self.opt.arg_edit_id, &self.opt.arg_track);
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
                "track" => {
                        request.track = Some(value.unwrap_or("").to_string());
                    },
                "user-fraction" => {
                        request.user_fraction = Some(arg_from_str(value.unwrap_or("0.0"), err, "user-fraction", "number"));
                    },
                "version-codes" => {
                        if request.version_codes.is_none() {
                            request.version_codes = Some(Default::default());
                        }
                        request.version_codes.as_mut().unwrap().push(arg_from_str(value.unwrap_or("-0"), err, "version-codes", "integer"));
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

    fn _edits_tracks_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Track = Default::default();
        let mut call = self.hub.edits().tracks_update(&request, &self.opt.arg_package_name, &self.opt.arg_edit_id, &self.opt.arg_track);
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
                "track" => {
                        request.track = Some(value.unwrap_or("").to_string());
                    },
                "user-fraction" => {
                        request.user_fraction = Some(arg_from_str(value.unwrap_or("0.0"), err, "user-fraction", "number"));
                    },
                "version-codes" => {
                        if request.version_codes.is_none() {
                            request.version_codes = Some(Default::default());
                        }
                        request.version_codes.as_mut().unwrap().push(arg_from_str(value.unwrap_or("-0"), err, "version-codes", "integer"));
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

    fn _edits_validate(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.edits().validate(&self.opt.arg_package_name, &self.opt.arg_edit_id);
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

    fn _inappproducts_batch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::InappproductsBatchRequest = Default::default();
        let mut call = self.hub.inappproducts().batch(&request);
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

    fn _inappproducts_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.inappproducts().delete(&self.opt.arg_package_name, &self.opt.arg_sku);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    None
                }
            }
        }
    }

    fn _inappproducts_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.inappproducts().get(&self.opt.arg_package_name, &self.opt.arg_sku);
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

    fn _inappproducts_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::InAppProduct = Default::default();
        let mut call = self.hub.inappproducts().insert(&request, &self.opt.arg_package_name);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "auto-convert-missing-prices" => {
                    call = call.auto_convert_missing_prices(arg_from_str(value.unwrap_or("false"), err, "auto-convert-missing-prices", "boolean"));
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
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_default_price_init(request: &mut api::InAppProduct) {
                if request.default_price.is_none() {
                    request.default_price = Some(Default::default());
                }
            }
            
            fn request_season_init(request: &mut api::InAppProduct) {
                if request.season.is_none() {
                    request.season = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "sku" => {
                        request.sku = Some(value.unwrap_or("").to_string());
                    },
                "status" => {
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "subscription-period" => {
                        request.subscription_period = Some(value.unwrap_or("").to_string());
                    },
                "season.start.day" => {
                        request_season_init(&mut request);
                        request.season.as_mut().unwrap().start.day = arg_from_str(value.unwrap_or("-0"), err, "season.start.day", "integer");
                    },
                "season.start.month" => {
                        request_season_init(&mut request);
                        request.season.as_mut().unwrap().start.month = arg_from_str(value.unwrap_or("-0"), err, "season.start.month", "integer");
                    },
                "season.end.day" => {
                        request_season_init(&mut request);
                        request.season.as_mut().unwrap().end.day = arg_from_str(value.unwrap_or("-0"), err, "season.end.day", "integer");
                    },
                "season.end.month" => {
                        request_season_init(&mut request);
                        request.season.as_mut().unwrap().end.month = arg_from_str(value.unwrap_or("-0"), err, "season.end.month", "integer");
                    },
                "package-name" => {
                        request_season_init(&mut request);
                        request.package_name = Some(value.unwrap_or("").to_string());
                    },
                "trial-period" => {
                        request_season_init(&mut request);
                        request.trial_period = Some(value.unwrap_or("").to_string());
                    },
                "purchase-type" => {
                        request_season_init(&mut request);
                        request.purchase_type = Some(value.unwrap_or("").to_string());
                    },
                "default-language" => {
                        request_season_init(&mut request);
                        request.default_language = Some(value.unwrap_or("").to_string());
                    },
                "default-price.currency" => {
                        request_default_price_init(&mut request);
                        request.default_price.as_mut().unwrap().currency = value.unwrap_or("").to_string();
                    },
                "default-price.price-micros" => {
                        request_default_price_init(&mut request);
                        request.default_price.as_mut().unwrap().price_micros = value.unwrap_or("").to_string();
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

    fn _inappproducts_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.inappproducts().list(&self.opt.arg_package_name);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "token" => {
                    call = call.token(value.unwrap_or(""));
                },
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _inappproducts_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::InAppProduct = Default::default();
        let mut call = self.hub.inappproducts().patch(&request, &self.opt.arg_package_name, &self.opt.arg_sku);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "auto-convert-missing-prices" => {
                    call = call.auto_convert_missing_prices(arg_from_str(value.unwrap_or("false"), err, "auto-convert-missing-prices", "boolean"));
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
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_default_price_init(request: &mut api::InAppProduct) {
                if request.default_price.is_none() {
                    request.default_price = Some(Default::default());
                }
            }
            
            fn request_season_init(request: &mut api::InAppProduct) {
                if request.season.is_none() {
                    request.season = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "sku" => {
                        request.sku = Some(value.unwrap_or("").to_string());
                    },
                "status" => {
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "subscription-period" => {
                        request.subscription_period = Some(value.unwrap_or("").to_string());
                    },
                "season.start.day" => {
                        request_season_init(&mut request);
                        request.season.as_mut().unwrap().start.day = arg_from_str(value.unwrap_or("-0"), err, "season.start.day", "integer");
                    },
                "season.start.month" => {
                        request_season_init(&mut request);
                        request.season.as_mut().unwrap().start.month = arg_from_str(value.unwrap_or("-0"), err, "season.start.month", "integer");
                    },
                "season.end.day" => {
                        request_season_init(&mut request);
                        request.season.as_mut().unwrap().end.day = arg_from_str(value.unwrap_or("-0"), err, "season.end.day", "integer");
                    },
                "season.end.month" => {
                        request_season_init(&mut request);
                        request.season.as_mut().unwrap().end.month = arg_from_str(value.unwrap_or("-0"), err, "season.end.month", "integer");
                    },
                "package-name" => {
                        request_season_init(&mut request);
                        request.package_name = Some(value.unwrap_or("").to_string());
                    },
                "trial-period" => {
                        request_season_init(&mut request);
                        request.trial_period = Some(value.unwrap_or("").to_string());
                    },
                "purchase-type" => {
                        request_season_init(&mut request);
                        request.purchase_type = Some(value.unwrap_or("").to_string());
                    },
                "default-language" => {
                        request_season_init(&mut request);
                        request.default_language = Some(value.unwrap_or("").to_string());
                    },
                "default-price.currency" => {
                        request_default_price_init(&mut request);
                        request.default_price.as_mut().unwrap().currency = value.unwrap_or("").to_string();
                    },
                "default-price.price-micros" => {
                        request_default_price_init(&mut request);
                        request.default_price.as_mut().unwrap().price_micros = value.unwrap_or("").to_string();
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

    fn _inappproducts_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::InAppProduct = Default::default();
        let mut call = self.hub.inappproducts().update(&request, &self.opt.arg_package_name, &self.opt.arg_sku);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "auto-convert-missing-prices" => {
                    call = call.auto_convert_missing_prices(arg_from_str(value.unwrap_or("false"), err, "auto-convert-missing-prices", "boolean"));
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
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_default_price_init(request: &mut api::InAppProduct) {
                if request.default_price.is_none() {
                    request.default_price = Some(Default::default());
                }
            }
            
            fn request_season_init(request: &mut api::InAppProduct) {
                if request.season.is_none() {
                    request.season = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "sku" => {
                        request.sku = Some(value.unwrap_or("").to_string());
                    },
                "status" => {
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "subscription-period" => {
                        request.subscription_period = Some(value.unwrap_or("").to_string());
                    },
                "season.start.day" => {
                        request_season_init(&mut request);
                        request.season.as_mut().unwrap().start.day = arg_from_str(value.unwrap_or("-0"), err, "season.start.day", "integer");
                    },
                "season.start.month" => {
                        request_season_init(&mut request);
                        request.season.as_mut().unwrap().start.month = arg_from_str(value.unwrap_or("-0"), err, "season.start.month", "integer");
                    },
                "season.end.day" => {
                        request_season_init(&mut request);
                        request.season.as_mut().unwrap().end.day = arg_from_str(value.unwrap_or("-0"), err, "season.end.day", "integer");
                    },
                "season.end.month" => {
                        request_season_init(&mut request);
                        request.season.as_mut().unwrap().end.month = arg_from_str(value.unwrap_or("-0"), err, "season.end.month", "integer");
                    },
                "package-name" => {
                        request_season_init(&mut request);
                        request.package_name = Some(value.unwrap_or("").to_string());
                    },
                "trial-period" => {
                        request_season_init(&mut request);
                        request.trial_period = Some(value.unwrap_or("").to_string());
                    },
                "purchase-type" => {
                        request_season_init(&mut request);
                        request.purchase_type = Some(value.unwrap_or("").to_string());
                    },
                "default-language" => {
                        request_season_init(&mut request);
                        request.default_language = Some(value.unwrap_or("").to_string());
                    },
                "default-price.currency" => {
                        request_default_price_init(&mut request);
                        request.default_price.as_mut().unwrap().currency = value.unwrap_or("").to_string();
                    },
                "default-price.price-micros" => {
                        request_default_price_init(&mut request);
                        request.default_price.as_mut().unwrap().price_micros = value.unwrap_or("").to_string();
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

    fn _purchases_products_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.purchases().products_get(&self.opt.arg_package_name, &self.opt.arg_product_id, &self.opt.arg_token);
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

    fn _purchases_subscriptions_cancel(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.purchases().subscriptions_cancel(&self.opt.arg_package_name, &self.opt.arg_subscription_id, &self.opt.arg_token);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    None
                }
            }
        }
    }

    fn _purchases_subscriptions_defer(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::SubscriptionPurchasesDeferRequest = Default::default();
        let mut call = self.hub.purchases().subscriptions_defer(&request, &self.opt.arg_package_name, &self.opt.arg_subscription_id, &self.opt.arg_token);
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
            fn request_deferral_info_init(request: &mut api::SubscriptionPurchasesDeferRequest) {
                if request.deferral_info.is_none() {
                    request.deferral_info = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "deferral-info.expected-expiry-time-millis" => {
                        request_deferral_info_init(&mut request);
                        request.deferral_info.as_mut().unwrap().expected_expiry_time_millis = value.unwrap_or("").to_string();
                    },
                "deferral-info.desired-expiry-time-millis" => {
                        request_deferral_info_init(&mut request);
                        request.deferral_info.as_mut().unwrap().desired_expiry_time_millis = value.unwrap_or("").to_string();
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

    fn _purchases_subscriptions_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.purchases().subscriptions_get(&self.opt.arg_package_name, &self.opt.arg_subscription_id, &self.opt.arg_token);
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

    fn _purchases_subscriptions_refund(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.purchases().subscriptions_refund(&self.opt.arg_package_name, &self.opt.arg_subscription_id, &self.opt.arg_token);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    None
                }
            }
        }
    }

    fn _purchases_subscriptions_revoke(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.purchases().subscriptions_revoke(&self.opt.arg_package_name, &self.opt.arg_subscription_id, &self.opt.arg_token);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    None
                }
            }
        }
    }

    fn _doit(&self, dry_run: bool) -> (Option<api::Error>, Option<InvalidOptionsError>) {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Option<api::Error>;
        let mut err_opt: Option<InvalidOptionsError> = None;

        if self.opt.cmd_edits {
            if self.opt.cmd_apklistings_delete {
                call_result = self._edits_apklistings_delete(dry_run, &mut err);
            } else if self.opt.cmd_apklistings_deleteall {
                call_result = self._edits_apklistings_deleteall(dry_run, &mut err);
            } else if self.opt.cmd_apklistings_get {
                call_result = self._edits_apklistings_get(dry_run, &mut err);
            } else if self.opt.cmd_apklistings_list {
                call_result = self._edits_apklistings_list(dry_run, &mut err);
            } else if self.opt.cmd_apklistings_patch {
                call_result = self._edits_apklistings_patch(dry_run, &mut err);
            } else if self.opt.cmd_apklistings_update {
                call_result = self._edits_apklistings_update(dry_run, &mut err);
            } else if self.opt.cmd_apks_addexternallyhosted {
                call_result = self._edits_apks_addexternallyhosted(dry_run, &mut err);
            } else if self.opt.cmd_apks_list {
                call_result = self._edits_apks_list(dry_run, &mut err);
            } else if self.opt.cmd_apks_upload {
                call_result = self._edits_apks_upload(dry_run, &mut err);
            } else if self.opt.cmd_commit {
                call_result = self._edits_commit(dry_run, &mut err);
            } else if self.opt.cmd_delete {
                call_result = self._edits_delete(dry_run, &mut err);
            } else if self.opt.cmd_details_get {
                call_result = self._edits_details_get(dry_run, &mut err);
            } else if self.opt.cmd_details_patch {
                call_result = self._edits_details_patch(dry_run, &mut err);
            } else if self.opt.cmd_details_update {
                call_result = self._edits_details_update(dry_run, &mut err);
            } else if self.opt.cmd_expansionfiles_get {
                call_result = self._edits_expansionfiles_get(dry_run, &mut err);
            } else if self.opt.cmd_expansionfiles_patch {
                call_result = self._edits_expansionfiles_patch(dry_run, &mut err);
            } else if self.opt.cmd_expansionfiles_update {
                call_result = self._edits_expansionfiles_update(dry_run, &mut err);
            } else if self.opt.cmd_expansionfiles_upload {
                call_result = self._edits_expansionfiles_upload(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._edits_get(dry_run, &mut err);
            } else if self.opt.cmd_images_delete {
                call_result = self._edits_images_delete(dry_run, &mut err);
            } else if self.opt.cmd_images_deleteall {
                call_result = self._edits_images_deleteall(dry_run, &mut err);
            } else if self.opt.cmd_images_list {
                call_result = self._edits_images_list(dry_run, &mut err);
            } else if self.opt.cmd_images_upload {
                call_result = self._edits_images_upload(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._edits_insert(dry_run, &mut err);
            } else if self.opt.cmd_listings_delete {
                call_result = self._edits_listings_delete(dry_run, &mut err);
            } else if self.opt.cmd_listings_deleteall {
                call_result = self._edits_listings_deleteall(dry_run, &mut err);
            } else if self.opt.cmd_listings_get {
                call_result = self._edits_listings_get(dry_run, &mut err);
            } else if self.opt.cmd_listings_list {
                call_result = self._edits_listings_list(dry_run, &mut err);
            } else if self.opt.cmd_listings_patch {
                call_result = self._edits_listings_patch(dry_run, &mut err);
            } else if self.opt.cmd_listings_update {
                call_result = self._edits_listings_update(dry_run, &mut err);
            } else if self.opt.cmd_testers_get {
                call_result = self._edits_testers_get(dry_run, &mut err);
            } else if self.opt.cmd_testers_patch {
                call_result = self._edits_testers_patch(dry_run, &mut err);
            } else if self.opt.cmd_testers_update {
                call_result = self._edits_testers_update(dry_run, &mut err);
            } else if self.opt.cmd_tracks_get {
                call_result = self._edits_tracks_get(dry_run, &mut err);
            } else if self.opt.cmd_tracks_list {
                call_result = self._edits_tracks_list(dry_run, &mut err);
            } else if self.opt.cmd_tracks_patch {
                call_result = self._edits_tracks_patch(dry_run, &mut err);
            } else if self.opt.cmd_tracks_update {
                call_result = self._edits_tracks_update(dry_run, &mut err);
            } else if self.opt.cmd_validate {
                call_result = self._edits_validate(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_inappproducts {
            if self.opt.cmd_batch {
                call_result = self._inappproducts_batch(dry_run, &mut err);
            } else if self.opt.cmd_delete {
                call_result = self._inappproducts_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._inappproducts_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._inappproducts_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._inappproducts_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._inappproducts_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._inappproducts_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_purchases {
            if self.opt.cmd_products_get {
                call_result = self._purchases_products_get(dry_run, &mut err);
            } else if self.opt.cmd_subscriptions_cancel {
                call_result = self._purchases_subscriptions_cancel(dry_run, &mut err);
            } else if self.opt.cmd_subscriptions_defer {
                call_result = self._purchases_subscriptions_defer(dry_run, &mut err);
            } else if self.opt.cmd_subscriptions_get {
                call_result = self._purchases_subscriptions_get(dry_run, &mut err);
            } else if self.opt.cmd_subscriptions_refund {
                call_result = self._purchases_subscriptions_refund(dry_run, &mut err);
            } else if self.opt.cmd_subscriptions_revoke {
                call_result = self._purchases_subscriptions_revoke(dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "androidpublisher2-secret.json") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                                      hyper::Client::new(),
                                      JsonTokenStorage {
                                        program_name: "androidpublisher2",
                                        db_dir: config_dir.clone(),
                                      }, None);
        let engine = Engine {
            opt: opt,
            hub: api::AndroidPublisher::new(hyper::Client::new(), auth),
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