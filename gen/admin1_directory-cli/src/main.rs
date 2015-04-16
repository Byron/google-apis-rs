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
extern crate google_admin1_directory as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  admin1-directory [options] asps delete <user-key> <code-id> [-p <v>]...
  admin1-directory [options] asps get <user-key> <code-id> [-p <v>]... [-o <out>]
  admin1-directory [options] asps list <user-key> [-p <v>]... [-o <out>]
  admin1-directory [options] channels stop -r <kv>... [-p <v>]...
  admin1-directory [options] chromeosdevices get <customer-id> <device-id> [-p <v>]... [-o <out>]
  admin1-directory [options] chromeosdevices list <customer-id> [-p <v>]... [-o <out>]
  admin1-directory [options] chromeosdevices patch <customer-id> <device-id> -r <kv>... [-p <v>]... [-o <out>]
  admin1-directory [options] chromeosdevices update <customer-id> <device-id> -r <kv>... [-p <v>]... [-o <out>]
  admin1-directory [options] groups aliases-delete <group-key> <alias> [-p <v>]...
  admin1-directory [options] groups aliases-insert <group-key> -r <kv>... [-p <v>]... [-o <out>]
  admin1-directory [options] groups aliases-list <group-key> [-p <v>]... [-o <out>]
  admin1-directory [options] groups delete <group-key> [-p <v>]...
  admin1-directory [options] groups get <group-key> [-p <v>]... [-o <out>]
  admin1-directory [options] groups insert -r <kv>... [-p <v>]... [-o <out>]
  admin1-directory [options] groups list [-p <v>]... [-o <out>]
  admin1-directory [options] groups patch <group-key> -r <kv>... [-p <v>]... [-o <out>]
  admin1-directory [options] groups update <group-key> -r <kv>... [-p <v>]... [-o <out>]
  admin1-directory [options] members delete <group-key> <member-key> [-p <v>]...
  admin1-directory [options] members get <group-key> <member-key> [-p <v>]... [-o <out>]
  admin1-directory [options] members insert <group-key> -r <kv>... [-p <v>]... [-o <out>]
  admin1-directory [options] members list <group-key> [-p <v>]... [-o <out>]
  admin1-directory [options] members patch <group-key> <member-key> -r <kv>... [-p <v>]... [-o <out>]
  admin1-directory [options] members update <group-key> <member-key> -r <kv>... [-p <v>]... [-o <out>]
  admin1-directory [options] mobiledevices action <customer-id> <resource-id> -r <kv>... [-p <v>]...
  admin1-directory [options] mobiledevices delete <customer-id> <resource-id> [-p <v>]...
  admin1-directory [options] mobiledevices get <customer-id> <resource-id> [-p <v>]... [-o <out>]
  admin1-directory [options] mobiledevices list <customer-id> [-p <v>]... [-o <out>]
  admin1-directory [options] notifications delete <customer> <notification-id> [-p <v>]...
  admin1-directory [options] notifications get <customer> <notification-id> [-p <v>]... [-o <out>]
  admin1-directory [options] notifications list <customer> [-p <v>]... [-o <out>]
  admin1-directory [options] notifications patch <customer> <notification-id> -r <kv>... [-p <v>]... [-o <out>]
  admin1-directory [options] notifications update <customer> <notification-id> -r <kv>... [-p <v>]... [-o <out>]
  admin1-directory [options] orgunits delete <customer-id> <org-unit-path> [-p <v>]...
  admin1-directory [options] orgunits get <customer-id> <org-unit-path> [-p <v>]... [-o <out>]
  admin1-directory [options] orgunits insert <customer-id> -r <kv>... [-p <v>]... [-o <out>]
  admin1-directory [options] orgunits list <customer-id> [-p <v>]... [-o <out>]
  admin1-directory [options] orgunits patch <customer-id> <org-unit-path> -r <kv>... [-p <v>]... [-o <out>]
  admin1-directory [options] orgunits update <customer-id> <org-unit-path> -r <kv>... [-p <v>]... [-o <out>]
  admin1-directory [options] schemas delete <customer-id> <schema-key> [-p <v>]...
  admin1-directory [options] schemas get <customer-id> <schema-key> [-p <v>]... [-o <out>]
  admin1-directory [options] schemas insert <customer-id> -r <kv>... [-p <v>]... [-o <out>]
  admin1-directory [options] schemas list <customer-id> [-p <v>]... [-o <out>]
  admin1-directory [options] schemas patch <customer-id> <schema-key> -r <kv>... [-p <v>]... [-o <out>]
  admin1-directory [options] schemas update <customer-id> <schema-key> -r <kv>... [-p <v>]... [-o <out>]
  admin1-directory [options] tokens delete <user-key> <client-id> [-p <v>]...
  admin1-directory [options] tokens get <user-key> <client-id> [-p <v>]... [-o <out>]
  admin1-directory [options] tokens list <user-key> [-p <v>]... [-o <out>]
  admin1-directory [options] users aliases-delete <user-key> <alias> [-p <v>]...
  admin1-directory [options] users aliases-insert <user-key> -r <kv>... [-p <v>]... [-o <out>]
  admin1-directory [options] users aliases-list <user-key> [-p <v>]... [-o <out>]
  admin1-directory [options] users aliases-watch <user-key> -r <kv>... [-p <v>]... [-o <out>]
  admin1-directory [options] users delete <user-key> [-p <v>]...
  admin1-directory [options] users get <user-key> [-p <v>]... [-o <out>]
  admin1-directory [options] users insert -r <kv>... [-p <v>]... [-o <out>]
  admin1-directory [options] users list [-p <v>]... [-o <out>]
  admin1-directory [options] users make-admin <user-key> -r <kv>... [-p <v>]...
  admin1-directory [options] users patch <user-key> -r <kv>... [-p <v>]... [-o <out>]
  admin1-directory [options] users photos-delete <user-key> [-p <v>]...
  admin1-directory [options] users photos-get <user-key> [-p <v>]... [-o <out>]
  admin1-directory [options] users photos-patch <user-key> -r <kv>... [-p <v>]... [-o <out>]
  admin1-directory [options] users photos-update <user-key> -r <kv>... [-p <v>]... [-o <out>]
  admin1-directory [options] users undelete <user-key> -r <kv>... [-p <v>]...
  admin1-directory [options] users update <user-key> -r <kv>... [-p <v>]... [-o <out>]
  admin1-directory [options] users watch -r <kv>... [-p <v>]... [-o <out>]
  admin1-directory [options] verification-codes generate <user-key> [-p <v>]...
  admin1-directory [options] verification-codes invalidate <user-key> [-p <v>]...
  admin1-directory [options] verification-codes list <user-key> [-p <v>]... [-o <out>]
  admin1-directory --help

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
    hub: api::Directory<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _asps_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let code_id: i32 = arg_from_str(&self.opt.arg_code_id, err, "<code-id>", "integer");
        let mut call = self.hub.asps().delete(&self.opt.arg_user_key, code_id);
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

    fn _asps_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let code_id: i32 = arg_from_str(&self.opt.arg_code_id, err, "<code-id>", "integer");
        let mut call = self.hub.asps().get(&self.opt.arg_user_key, code_id);
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

    fn _asps_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.asps().list(&self.opt.arg_user_key);
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

    fn _channels_stop(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Channel = Default::default();
        let mut call = self.hub.channels().stop(&request);
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
                "resource-uri" => {
                        request.resource_uri = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "resource-id" => {
                        request.resource_id = Some(value.unwrap_or("").to_string());
                    },
                "payload" => {
                        request.payload = Some(arg_from_str(value.unwrap_or("false"), err, "payload", "boolean"));
                    },
                "token" => {
                        request.token = Some(value.unwrap_or("").to_string());
                    },
                "params" => {
                        if request.params.is_none() {
                            request.params = Some(Default::default());
                        }
                        request.params.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "expiration" => {
                        request.expiration = Some(value.unwrap_or("").to_string());
                    },
                "address" => {
                        request.address = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
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

    fn _chromeosdevices_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.chromeosdevices().get(&self.opt.arg_customer_id, &self.opt.arg_device_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
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

    fn _chromeosdevices_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.chromeosdevices().list(&self.opt.arg_customer_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "sort-order" => {
                    call = call.sort_order(value.unwrap_or(""));
                },
                "query" => {
                    call = call.query(value.unwrap_or(""));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
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

    fn _chromeosdevices_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::ChromeOsDevice = Default::default();
        let mut call = self.hub.chromeosdevices().patch(&request, &self.opt.arg_customer_id, &self.opt.arg_device_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
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
            match &field_name.to_string()[..] {
                "mac-address" => {
                        request.mac_address = Some(value.unwrap_or("").to_string());
                    },
                "last-sync" => {
                        request.last_sync = Some(value.unwrap_or("").to_string());
                    },
                "order-number" => {
                        request.order_number = Some(value.unwrap_or("").to_string());
                    },
                "annotated-user" => {
                        request.annotated_user = Some(value.unwrap_or("").to_string());
                    },
                "firmware-version" => {
                        request.firmware_version = Some(value.unwrap_or("").to_string());
                    },
                "boot-mode" => {
                        request.boot_mode = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "support-end-date" => {
                        request.support_end_date = Some(value.unwrap_or("").to_string());
                    },
                "status" => {
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "will-auto-renew" => {
                        request.will_auto_renew = Some(arg_from_str(value.unwrap_or("false"), err, "will-auto-renew", "boolean"));
                    },
                "ethernet-mac-address" => {
                        request.ethernet_mac_address = Some(value.unwrap_or("").to_string());
                    },
                "meid" => {
                        request.meid = Some(value.unwrap_or("").to_string());
                    },
                "annotated-location" => {
                        request.annotated_location = Some(value.unwrap_or("").to_string());
                    },
                "device-id" => {
                        request.device_id = Some(value.unwrap_or("").to_string());
                    },
                "platform-version" => {
                        request.platform_version = Some(value.unwrap_or("").to_string());
                    },
                "os-version" => {
                        request.os_version = Some(value.unwrap_or("").to_string());
                    },
                "last-enrollment-time" => {
                        request.last_enrollment_time = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "notes" => {
                        request.notes = Some(value.unwrap_or("").to_string());
                    },
                "serial-number" => {
                        request.serial_number = Some(value.unwrap_or("").to_string());
                    },
                "org-unit-path" => {
                        request.org_unit_path = Some(value.unwrap_or("").to_string());
                    },
                "model" => {
                        request.model = Some(value.unwrap_or("").to_string());
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

    fn _chromeosdevices_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::ChromeOsDevice = Default::default();
        let mut call = self.hub.chromeosdevices().update(&request, &self.opt.arg_customer_id, &self.opt.arg_device_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
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
            match &field_name.to_string()[..] {
                "mac-address" => {
                        request.mac_address = Some(value.unwrap_or("").to_string());
                    },
                "last-sync" => {
                        request.last_sync = Some(value.unwrap_or("").to_string());
                    },
                "order-number" => {
                        request.order_number = Some(value.unwrap_or("").to_string());
                    },
                "annotated-user" => {
                        request.annotated_user = Some(value.unwrap_or("").to_string());
                    },
                "firmware-version" => {
                        request.firmware_version = Some(value.unwrap_or("").to_string());
                    },
                "boot-mode" => {
                        request.boot_mode = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "support-end-date" => {
                        request.support_end_date = Some(value.unwrap_or("").to_string());
                    },
                "status" => {
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "will-auto-renew" => {
                        request.will_auto_renew = Some(arg_from_str(value.unwrap_or("false"), err, "will-auto-renew", "boolean"));
                    },
                "ethernet-mac-address" => {
                        request.ethernet_mac_address = Some(value.unwrap_or("").to_string());
                    },
                "meid" => {
                        request.meid = Some(value.unwrap_or("").to_string());
                    },
                "annotated-location" => {
                        request.annotated_location = Some(value.unwrap_or("").to_string());
                    },
                "device-id" => {
                        request.device_id = Some(value.unwrap_or("").to_string());
                    },
                "platform-version" => {
                        request.platform_version = Some(value.unwrap_or("").to_string());
                    },
                "os-version" => {
                        request.os_version = Some(value.unwrap_or("").to_string());
                    },
                "last-enrollment-time" => {
                        request.last_enrollment_time = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "notes" => {
                        request.notes = Some(value.unwrap_or("").to_string());
                    },
                "serial-number" => {
                        request.serial_number = Some(value.unwrap_or("").to_string());
                    },
                "org-unit-path" => {
                        request.org_unit_path = Some(value.unwrap_or("").to_string());
                    },
                "model" => {
                        request.model = Some(value.unwrap_or("").to_string());
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

    fn _groups_aliases_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.groups().aliases_delete(&self.opt.arg_group_key, &self.opt.arg_alias);
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

    fn _groups_aliases_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Alias = Default::default();
        let mut call = self.hub.groups().aliases_insert(&request, &self.opt.arg_group_key);
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
                "alias" => {
                        request.alias = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "primary-email" => {
                        request.primary_email = Some(value.unwrap_or("").to_string());
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

    fn _groups_aliases_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.groups().aliases_list(&self.opt.arg_group_key);
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

    fn _groups_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.groups().delete(&self.opt.arg_group_key);
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

    fn _groups_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.groups().get(&self.opt.arg_group_key);
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

    fn _groups_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Group = Default::default();
        let mut call = self.hub.groups().insert(&request);
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
                "non-editable-aliases" => {
                        if request.non_editable_aliases.is_none() {
                            request.non_editable_aliases = Some(Default::default());
                        }
                        request.non_editable_aliases.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "admin-created" => {
                        request.admin_created = Some(arg_from_str(value.unwrap_or("false"), err, "admin-created", "boolean"));
                    },
                "direct-members-count" => {
                        request.direct_members_count = Some(arg_from_str(value.unwrap_or("-0"), err, "direct-members-count", "int64"));
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "email" => {
                        request.email = Some(value.unwrap_or("").to_string());
                    },
                "aliases" => {
                        if request.aliases.is_none() {
                            request.aliases = Some(Default::default());
                        }
                        request.aliases.as_mut().unwrap().push(value.unwrap_or("").to_string());
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

    fn _groups_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.groups().list();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "user-key" => {
                    call = call.user_key(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "domain" => {
                    call = call.domain(value.unwrap_or(""));
                },
                "customer" => {
                    call = call.customer(value.unwrap_or(""));
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

    fn _groups_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Group = Default::default();
        let mut call = self.hub.groups().patch(&request, &self.opt.arg_group_key);
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
                "non-editable-aliases" => {
                        if request.non_editable_aliases.is_none() {
                            request.non_editable_aliases = Some(Default::default());
                        }
                        request.non_editable_aliases.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "admin-created" => {
                        request.admin_created = Some(arg_from_str(value.unwrap_or("false"), err, "admin-created", "boolean"));
                    },
                "direct-members-count" => {
                        request.direct_members_count = Some(arg_from_str(value.unwrap_or("-0"), err, "direct-members-count", "int64"));
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "email" => {
                        request.email = Some(value.unwrap_or("").to_string());
                    },
                "aliases" => {
                        if request.aliases.is_none() {
                            request.aliases = Some(Default::default());
                        }
                        request.aliases.as_mut().unwrap().push(value.unwrap_or("").to_string());
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

    fn _groups_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Group = Default::default();
        let mut call = self.hub.groups().update(&request, &self.opt.arg_group_key);
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
                "non-editable-aliases" => {
                        if request.non_editable_aliases.is_none() {
                            request.non_editable_aliases = Some(Default::default());
                        }
                        request.non_editable_aliases.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "admin-created" => {
                        request.admin_created = Some(arg_from_str(value.unwrap_or("false"), err, "admin-created", "boolean"));
                    },
                "direct-members-count" => {
                        request.direct_members_count = Some(arg_from_str(value.unwrap_or("-0"), err, "direct-members-count", "int64"));
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "email" => {
                        request.email = Some(value.unwrap_or("").to_string());
                    },
                "aliases" => {
                        if request.aliases.is_none() {
                            request.aliases = Some(Default::default());
                        }
                        request.aliases.as_mut().unwrap().push(value.unwrap_or("").to_string());
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

    fn _members_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.members().delete(&self.opt.arg_group_key, &self.opt.arg_member_key);
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

    fn _members_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.members().get(&self.opt.arg_group_key, &self.opt.arg_member_key);
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

    fn _members_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Member = Default::default();
        let mut call = self.hub.members().insert(&request, &self.opt.arg_group_key);
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
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "role" => {
                        request.role = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "email" => {
                        request.email = Some(value.unwrap_or("").to_string());
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

    fn _members_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.members().list(&self.opt.arg_group_key);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "roles" => {
                    call = call.roles(value.unwrap_or(""));
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _members_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Member = Default::default();
        let mut call = self.hub.members().patch(&request, &self.opt.arg_group_key, &self.opt.arg_member_key);
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
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "role" => {
                        request.role = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "email" => {
                        request.email = Some(value.unwrap_or("").to_string());
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

    fn _members_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Member = Default::default();
        let mut call = self.hub.members().update(&request, &self.opt.arg_group_key, &self.opt.arg_member_key);
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
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "role" => {
                        request.role = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "email" => {
                        request.email = Some(value.unwrap_or("").to_string());
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

    fn _mobiledevices_action(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::MobileDeviceAction = Default::default();
        let mut call = self.hub.mobiledevices().action(&request, &self.opt.arg_customer_id, &self.opt.arg_resource_id);
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
                "action" => {
                        request.action = Some(value.unwrap_or("").to_string());
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

    fn _mobiledevices_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.mobiledevices().delete(&self.opt.arg_customer_id, &self.opt.arg_resource_id);
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

    fn _mobiledevices_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.mobiledevices().get(&self.opt.arg_customer_id, &self.opt.arg_resource_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
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

    fn _mobiledevices_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.mobiledevices().list(&self.opt.arg_customer_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "sort-order" => {
                    call = call.sort_order(value.unwrap_or(""));
                },
                "query" => {
                    call = call.query(value.unwrap_or(""));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
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

    fn _notifications_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.notifications().delete(&self.opt.arg_customer, &self.opt.arg_notification_id);
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

    fn _notifications_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.notifications().get(&self.opt.arg_customer, &self.opt.arg_notification_id);
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

    fn _notifications_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.notifications().list(&self.opt.arg_customer);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "language" => {
                    call = call.language(value.unwrap_or(""));
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

    fn _notifications_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Notification = Default::default();
        let mut call = self.hub.notifications().patch(&request, &self.opt.arg_customer, &self.opt.arg_notification_id);
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
                "body" => {
                        request.body = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "send-time" => {
                        request.send_time = Some(value.unwrap_or("").to_string());
                    },
                "notification-id" => {
                        request.notification_id = Some(value.unwrap_or("").to_string());
                    },
                "from-address" => {
                        request.from_address = Some(value.unwrap_or("").to_string());
                    },
                "is-unread" => {
                        request.is_unread = Some(arg_from_str(value.unwrap_or("false"), err, "is-unread", "boolean"));
                    },
                "subject" => {
                        request.subject = Some(value.unwrap_or("").to_string());
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

    fn _notifications_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Notification = Default::default();
        let mut call = self.hub.notifications().update(&request, &self.opt.arg_customer, &self.opt.arg_notification_id);
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
                "body" => {
                        request.body = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "send-time" => {
                        request.send_time = Some(value.unwrap_or("").to_string());
                    },
                "notification-id" => {
                        request.notification_id = Some(value.unwrap_or("").to_string());
                    },
                "from-address" => {
                        request.from_address = Some(value.unwrap_or("").to_string());
                    },
                "is-unread" => {
                        request.is_unread = Some(arg_from_str(value.unwrap_or("false"), err, "is-unread", "boolean"));
                    },
                "subject" => {
                        request.subject = Some(value.unwrap_or("").to_string());
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

    fn _orgunits_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.orgunits().delete(&self.opt.arg_customer_id, &self.opt.arg_org_unit_path);
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

    fn _orgunits_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.orgunits().get(&self.opt.arg_customer_id, &self.opt.arg_org_unit_path);
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

    fn _orgunits_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::OrgUnit = Default::default();
        let mut call = self.hub.orgunits().insert(&request, &self.opt.arg_customer_id);
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
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "parent-org-unit-path" => {
                        request.parent_org_unit_path = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "org-unit-path" => {
                        request.org_unit_path = Some(value.unwrap_or("").to_string());
                    },
                "parent-org-unit-id" => {
                        request.parent_org_unit_id = Some(value.unwrap_or("").to_string());
                    },
                "block-inheritance" => {
                        request.block_inheritance = Some(arg_from_str(value.unwrap_or("false"), err, "block-inheritance", "boolean"));
                    },
                "org-unit-id" => {
                        request.org_unit_id = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
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

    fn _orgunits_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.orgunits().list(&self.opt.arg_customer_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "type" => {
                    call = call.type_(value.unwrap_or(""));
                },
                "org-unit-path" => {
                    call = call.org_unit_path(value.unwrap_or(""));
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

    fn _orgunits_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::OrgUnit = Default::default();
        let mut call = self.hub.orgunits().patch(&request, &self.opt.arg_customer_id, &self.opt.arg_org_unit_path);
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
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "parent-org-unit-path" => {
                        request.parent_org_unit_path = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "org-unit-path" => {
                        request.org_unit_path = Some(value.unwrap_or("").to_string());
                    },
                "parent-org-unit-id" => {
                        request.parent_org_unit_id = Some(value.unwrap_or("").to_string());
                    },
                "block-inheritance" => {
                        request.block_inheritance = Some(arg_from_str(value.unwrap_or("false"), err, "block-inheritance", "boolean"));
                    },
                "org-unit-id" => {
                        request.org_unit_id = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
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

    fn _orgunits_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::OrgUnit = Default::default();
        let mut call = self.hub.orgunits().update(&request, &self.opt.arg_customer_id, &self.opt.arg_org_unit_path);
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
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "parent-org-unit-path" => {
                        request.parent_org_unit_path = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "org-unit-path" => {
                        request.org_unit_path = Some(value.unwrap_or("").to_string());
                    },
                "parent-org-unit-id" => {
                        request.parent_org_unit_id = Some(value.unwrap_or("").to_string());
                    },
                "block-inheritance" => {
                        request.block_inheritance = Some(arg_from_str(value.unwrap_or("false"), err, "block-inheritance", "boolean"));
                    },
                "org-unit-id" => {
                        request.org_unit_id = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
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

    fn _schemas_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.schemas().delete(&self.opt.arg_customer_id, &self.opt.arg_schema_key);
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

    fn _schemas_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.schemas().get(&self.opt.arg_customer_id, &self.opt.arg_schema_key);
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

    fn _schemas_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Schema = Default::default();
        let mut call = self.hub.schemas().insert(&request, &self.opt.arg_customer_id);
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
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "schema-id" => {
                        request.schema_id = Some(value.unwrap_or("").to_string());
                    },
                "schema-name" => {
                        request.schema_name = Some(value.unwrap_or("").to_string());
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

    fn _schemas_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.schemas().list(&self.opt.arg_customer_id);
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

    fn _schemas_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Schema = Default::default();
        let mut call = self.hub.schemas().patch(&request, &self.opt.arg_customer_id, &self.opt.arg_schema_key);
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
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "schema-id" => {
                        request.schema_id = Some(value.unwrap_or("").to_string());
                    },
                "schema-name" => {
                        request.schema_name = Some(value.unwrap_or("").to_string());
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

    fn _schemas_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Schema = Default::default();
        let mut call = self.hub.schemas().update(&request, &self.opt.arg_customer_id, &self.opt.arg_schema_key);
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
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "schema-id" => {
                        request.schema_id = Some(value.unwrap_or("").to_string());
                    },
                "schema-name" => {
                        request.schema_name = Some(value.unwrap_or("").to_string());
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

    fn _tokens_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.tokens().delete(&self.opt.arg_user_key, &self.opt.arg_client_id);
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

    fn _tokens_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.tokens().get(&self.opt.arg_user_key, &self.opt.arg_client_id);
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

    fn _tokens_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.tokens().list(&self.opt.arg_user_key);
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

    fn _users_aliases_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.users().aliases_delete(&self.opt.arg_user_key, &self.opt.arg_alias);
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

    fn _users_aliases_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Alias = Default::default();
        let mut call = self.hub.users().aliases_insert(&request, &self.opt.arg_user_key);
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
                "alias" => {
                        request.alias = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "primary-email" => {
                        request.primary_email = Some(value.unwrap_or("").to_string());
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

    fn _users_aliases_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.users().aliases_list(&self.opt.arg_user_key);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "event" => {
                    call = call.event(value.unwrap_or(""));
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

    fn _users_aliases_watch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Channel = Default::default();
        let mut call = self.hub.users().aliases_watch(&request, &self.opt.arg_user_key);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "event" => {
                    call = call.event(value.unwrap_or(""));
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
            match &field_name.to_string()[..] {
                "resource-uri" => {
                        request.resource_uri = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "resource-id" => {
                        request.resource_id = Some(value.unwrap_or("").to_string());
                    },
                "payload" => {
                        request.payload = Some(arg_from_str(value.unwrap_or("false"), err, "payload", "boolean"));
                    },
                "token" => {
                        request.token = Some(value.unwrap_or("").to_string());
                    },
                "params" => {
                        if request.params.is_none() {
                            request.params = Some(Default::default());
                        }
                        request.params.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "expiration" => {
                        request.expiration = Some(value.unwrap_or("").to_string());
                    },
                "address" => {
                        request.address = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
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

    fn _users_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.users().delete(&self.opt.arg_user_key);
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

    fn _users_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.users().get(&self.opt.arg_user_key);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "view-type" => {
                    call = call.view_type(value.unwrap_or(""));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "custom-field-mask" => {
                    call = call.custom_field_mask(value.unwrap_or(""));
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

    fn _users_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::User = Default::default();
        let mut call = self.hub.users().insert(&request);
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
            fn request_name_init(request: &mut api::User) {
                if request.name.is_none() {
                    request.name = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "is-delegated-admin" => {
                        request.is_delegated_admin = Some(arg_from_str(value.unwrap_or("false"), err, "is-delegated-admin", "boolean"));
                    },
                "suspended" => {
                        request.suspended = Some(arg_from_str(value.unwrap_or("false"), err, "suspended", "boolean"));
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "aliases" => {
                        if request.aliases.is_none() {
                            request.aliases = Some(Default::default());
                        }
                        request.aliases.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "non-editable-aliases" => {
                        if request.non_editable_aliases.is_none() {
                            request.non_editable_aliases = Some(Default::default());
                        }
                        request.non_editable_aliases.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "deletion-time" => {
                        request.deletion_time = Some(value.unwrap_or("").to_string());
                    },
                "suspension-reason" => {
                        request.suspension_reason = Some(value.unwrap_or("").to_string());
                    },
                "thumbnail-photo-url" => {
                        request.thumbnail_photo_url = Some(value.unwrap_or("").to_string());
                    },
                "is-admin" => {
                        request.is_admin = Some(arg_from_str(value.unwrap_or("false"), err, "is-admin", "boolean"));
                    },
                "include-in-global-address-list" => {
                        request.include_in_global_address_list = Some(arg_from_str(value.unwrap_or("false"), err, "include-in-global-address-list", "boolean"));
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "last-login-time" => {
                        request.last_login_time = Some(value.unwrap_or("").to_string());
                    },
                "org-unit-path" => {
                        request.org_unit_path = Some(value.unwrap_or("").to_string());
                    },
                "agreed-to-terms" => {
                        request.agreed_to_terms = Some(arg_from_str(value.unwrap_or("false"), err, "agreed-to-terms", "boolean"));
                    },
                "ip-whitelisted" => {
                        request.ip_whitelisted = Some(arg_from_str(value.unwrap_or("false"), err, "ip-whitelisted", "boolean"));
                    },
                "primary-email" => {
                        request.primary_email = Some(value.unwrap_or("").to_string());
                    },
                "is-mailbox-setup" => {
                        request.is_mailbox_setup = Some(arg_from_str(value.unwrap_or("false"), err, "is-mailbox-setup", "boolean"));
                    },
                "password" => {
                        request.password = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "hash-function" => {
                        request.hash_function = Some(value.unwrap_or("").to_string());
                    },
                "name.full-name" => {
                        request_name_init(&mut request);
                        request.name.as_mut().unwrap().full_name = value.unwrap_or("").to_string();
                    },
                "name.given-name" => {
                        request_name_init(&mut request);
                        request.name.as_mut().unwrap().given_name = value.unwrap_or("").to_string();
                    },
                "name.family-name" => {
                        request_name_init(&mut request);
                        request.name.as_mut().unwrap().family_name = value.unwrap_or("").to_string();
                    },
                "creation-time" => {
                        request_name_init(&mut request);
                        request.creation_time = Some(value.unwrap_or("").to_string());
                    },
                "change-password-at-next-login" => {
                        request_name_init(&mut request);
                        request.change_password_at_next_login = Some(arg_from_str(value.unwrap_or("false"), err, "change-password-at-next-login", "boolean"));
                    },
                "customer-id" => {
                        request_name_init(&mut request);
                        request.customer_id = Some(value.unwrap_or("").to_string());
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

    fn _users_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.users().list();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "view-type" => {
                    call = call.view_type(value.unwrap_or(""));
                },
                "sort-order" => {
                    call = call.sort_order(value.unwrap_or(""));
                },
                "show-deleted" => {
                    call = call.show_deleted(value.unwrap_or(""));
                },
                "query" => {
                    call = call.query(value.unwrap_or(""));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "event" => {
                    call = call.event(value.unwrap_or(""));
                },
                "domain" => {
                    call = call.domain(value.unwrap_or(""));
                },
                "customer" => {
                    call = call.customer(value.unwrap_or(""));
                },
                "custom-field-mask" => {
                    call = call.custom_field_mask(value.unwrap_or(""));
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

    fn _users_make_admin(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::UserMakeAdmin = Default::default();
        let mut call = self.hub.users().make_admin(&request, &self.opt.arg_user_key);
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
                "status" => {
                        request.status = Some(arg_from_str(value.unwrap_or("false"), err, "status", "boolean"));
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

    fn _users_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::User = Default::default();
        let mut call = self.hub.users().patch(&request, &self.opt.arg_user_key);
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
            fn request_name_init(request: &mut api::User) {
                if request.name.is_none() {
                    request.name = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "is-delegated-admin" => {
                        request.is_delegated_admin = Some(arg_from_str(value.unwrap_or("false"), err, "is-delegated-admin", "boolean"));
                    },
                "suspended" => {
                        request.suspended = Some(arg_from_str(value.unwrap_or("false"), err, "suspended", "boolean"));
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "aliases" => {
                        if request.aliases.is_none() {
                            request.aliases = Some(Default::default());
                        }
                        request.aliases.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "non-editable-aliases" => {
                        if request.non_editable_aliases.is_none() {
                            request.non_editable_aliases = Some(Default::default());
                        }
                        request.non_editable_aliases.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "deletion-time" => {
                        request.deletion_time = Some(value.unwrap_or("").to_string());
                    },
                "suspension-reason" => {
                        request.suspension_reason = Some(value.unwrap_or("").to_string());
                    },
                "thumbnail-photo-url" => {
                        request.thumbnail_photo_url = Some(value.unwrap_or("").to_string());
                    },
                "is-admin" => {
                        request.is_admin = Some(arg_from_str(value.unwrap_or("false"), err, "is-admin", "boolean"));
                    },
                "include-in-global-address-list" => {
                        request.include_in_global_address_list = Some(arg_from_str(value.unwrap_or("false"), err, "include-in-global-address-list", "boolean"));
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "last-login-time" => {
                        request.last_login_time = Some(value.unwrap_or("").to_string());
                    },
                "org-unit-path" => {
                        request.org_unit_path = Some(value.unwrap_or("").to_string());
                    },
                "agreed-to-terms" => {
                        request.agreed_to_terms = Some(arg_from_str(value.unwrap_or("false"), err, "agreed-to-terms", "boolean"));
                    },
                "ip-whitelisted" => {
                        request.ip_whitelisted = Some(arg_from_str(value.unwrap_or("false"), err, "ip-whitelisted", "boolean"));
                    },
                "primary-email" => {
                        request.primary_email = Some(value.unwrap_or("").to_string());
                    },
                "is-mailbox-setup" => {
                        request.is_mailbox_setup = Some(arg_from_str(value.unwrap_or("false"), err, "is-mailbox-setup", "boolean"));
                    },
                "password" => {
                        request.password = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "hash-function" => {
                        request.hash_function = Some(value.unwrap_or("").to_string());
                    },
                "name.full-name" => {
                        request_name_init(&mut request);
                        request.name.as_mut().unwrap().full_name = value.unwrap_or("").to_string();
                    },
                "name.given-name" => {
                        request_name_init(&mut request);
                        request.name.as_mut().unwrap().given_name = value.unwrap_or("").to_string();
                    },
                "name.family-name" => {
                        request_name_init(&mut request);
                        request.name.as_mut().unwrap().family_name = value.unwrap_or("").to_string();
                    },
                "creation-time" => {
                        request_name_init(&mut request);
                        request.creation_time = Some(value.unwrap_or("").to_string());
                    },
                "change-password-at-next-login" => {
                        request_name_init(&mut request);
                        request.change_password_at_next_login = Some(arg_from_str(value.unwrap_or("false"), err, "change-password-at-next-login", "boolean"));
                    },
                "customer-id" => {
                        request_name_init(&mut request);
                        request.customer_id = Some(value.unwrap_or("").to_string());
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

    fn _users_photos_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.users().photos_delete(&self.opt.arg_user_key);
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

    fn _users_photos_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.users().photos_get(&self.opt.arg_user_key);
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

    fn _users_photos_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::UserPhoto = Default::default();
        let mut call = self.hub.users().photos_patch(&request, &self.opt.arg_user_key);
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
                "mime-type" => {
                        request.mime_type = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "photo-data" => {
                        request.photo_data = Some(value.unwrap_or("").to_string());
                    },
                "height" => {
                        request.height = Some(arg_from_str(value.unwrap_or("-0"), err, "height", "integer"));
                    },
                "width" => {
                        request.width = Some(arg_from_str(value.unwrap_or("-0"), err, "width", "integer"));
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "primary-email" => {
                        request.primary_email = Some(value.unwrap_or("").to_string());
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

    fn _users_photos_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::UserPhoto = Default::default();
        let mut call = self.hub.users().photos_update(&request, &self.opt.arg_user_key);
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
                "mime-type" => {
                        request.mime_type = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "photo-data" => {
                        request.photo_data = Some(value.unwrap_or("").to_string());
                    },
                "height" => {
                        request.height = Some(arg_from_str(value.unwrap_or("-0"), err, "height", "integer"));
                    },
                "width" => {
                        request.width = Some(arg_from_str(value.unwrap_or("-0"), err, "width", "integer"));
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "primary-email" => {
                        request.primary_email = Some(value.unwrap_or("").to_string());
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

    fn _users_undelete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::UserUndelete = Default::default();
        let mut call = self.hub.users().undelete(&request, &self.opt.arg_user_key);
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
                "org-unit-path" => {
                        request.org_unit_path = Some(value.unwrap_or("").to_string());
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

    fn _users_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::User = Default::default();
        let mut call = self.hub.users().update(&request, &self.opt.arg_user_key);
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
            fn request_name_init(request: &mut api::User) {
                if request.name.is_none() {
                    request.name = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "is-delegated-admin" => {
                        request.is_delegated_admin = Some(arg_from_str(value.unwrap_or("false"), err, "is-delegated-admin", "boolean"));
                    },
                "suspended" => {
                        request.suspended = Some(arg_from_str(value.unwrap_or("false"), err, "suspended", "boolean"));
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "aliases" => {
                        if request.aliases.is_none() {
                            request.aliases = Some(Default::default());
                        }
                        request.aliases.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "non-editable-aliases" => {
                        if request.non_editable_aliases.is_none() {
                            request.non_editable_aliases = Some(Default::default());
                        }
                        request.non_editable_aliases.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "deletion-time" => {
                        request.deletion_time = Some(value.unwrap_or("").to_string());
                    },
                "suspension-reason" => {
                        request.suspension_reason = Some(value.unwrap_or("").to_string());
                    },
                "thumbnail-photo-url" => {
                        request.thumbnail_photo_url = Some(value.unwrap_or("").to_string());
                    },
                "is-admin" => {
                        request.is_admin = Some(arg_from_str(value.unwrap_or("false"), err, "is-admin", "boolean"));
                    },
                "include-in-global-address-list" => {
                        request.include_in_global_address_list = Some(arg_from_str(value.unwrap_or("false"), err, "include-in-global-address-list", "boolean"));
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "last-login-time" => {
                        request.last_login_time = Some(value.unwrap_or("").to_string());
                    },
                "org-unit-path" => {
                        request.org_unit_path = Some(value.unwrap_or("").to_string());
                    },
                "agreed-to-terms" => {
                        request.agreed_to_terms = Some(arg_from_str(value.unwrap_or("false"), err, "agreed-to-terms", "boolean"));
                    },
                "ip-whitelisted" => {
                        request.ip_whitelisted = Some(arg_from_str(value.unwrap_or("false"), err, "ip-whitelisted", "boolean"));
                    },
                "primary-email" => {
                        request.primary_email = Some(value.unwrap_or("").to_string());
                    },
                "is-mailbox-setup" => {
                        request.is_mailbox_setup = Some(arg_from_str(value.unwrap_or("false"), err, "is-mailbox-setup", "boolean"));
                    },
                "password" => {
                        request.password = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "hash-function" => {
                        request.hash_function = Some(value.unwrap_or("").to_string());
                    },
                "name.full-name" => {
                        request_name_init(&mut request);
                        request.name.as_mut().unwrap().full_name = value.unwrap_or("").to_string();
                    },
                "name.given-name" => {
                        request_name_init(&mut request);
                        request.name.as_mut().unwrap().given_name = value.unwrap_or("").to_string();
                    },
                "name.family-name" => {
                        request_name_init(&mut request);
                        request.name.as_mut().unwrap().family_name = value.unwrap_or("").to_string();
                    },
                "creation-time" => {
                        request_name_init(&mut request);
                        request.creation_time = Some(value.unwrap_or("").to_string());
                    },
                "change-password-at-next-login" => {
                        request_name_init(&mut request);
                        request.change_password_at_next_login = Some(arg_from_str(value.unwrap_or("false"), err, "change-password-at-next-login", "boolean"));
                    },
                "customer-id" => {
                        request_name_init(&mut request);
                        request.customer_id = Some(value.unwrap_or("").to_string());
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

    fn _users_watch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Channel = Default::default();
        let mut call = self.hub.users().watch(&request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "view-type" => {
                    call = call.view_type(value.unwrap_or(""));
                },
                "sort-order" => {
                    call = call.sort_order(value.unwrap_or(""));
                },
                "show-deleted" => {
                    call = call.show_deleted(value.unwrap_or(""));
                },
                "query" => {
                    call = call.query(value.unwrap_or(""));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "event" => {
                    call = call.event(value.unwrap_or(""));
                },
                "domain" => {
                    call = call.domain(value.unwrap_or(""));
                },
                "customer" => {
                    call = call.customer(value.unwrap_or(""));
                },
                "custom-field-mask" => {
                    call = call.custom_field_mask(value.unwrap_or(""));
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
            match &field_name.to_string()[..] {
                "resource-uri" => {
                        request.resource_uri = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "resource-id" => {
                        request.resource_id = Some(value.unwrap_or("").to_string());
                    },
                "payload" => {
                        request.payload = Some(arg_from_str(value.unwrap_or("false"), err, "payload", "boolean"));
                    },
                "token" => {
                        request.token = Some(value.unwrap_or("").to_string());
                    },
                "params" => {
                        if request.params.is_none() {
                            request.params = Some(Default::default());
                        }
                        request.params.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "expiration" => {
                        request.expiration = Some(value.unwrap_or("").to_string());
                    },
                "address" => {
                        request.address = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request.type_ = Some(value.unwrap_or("").to_string());
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

    fn _verification_codes_generate(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.verification_codes().generate(&self.opt.arg_user_key);
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

    fn _verification_codes_invalidate(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.verification_codes().invalidate(&self.opt.arg_user_key);
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

    fn _verification_codes_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.verification_codes().list(&self.opt.arg_user_key);
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

    fn _doit(&self, dry_run: bool) -> (Option<api::Error>, Option<InvalidOptionsError>) {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Option<api::Error>;
        let mut err_opt: Option<InvalidOptionsError> = None;

        if self.opt.cmd_asps {
            if self.opt.cmd_delete {
                call_result = self._asps_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._asps_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._asps_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_channels {
            if self.opt.cmd_stop {
                call_result = self._channels_stop(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_chromeosdevices {
            if self.opt.cmd_get {
                call_result = self._chromeosdevices_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._chromeosdevices_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._chromeosdevices_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._chromeosdevices_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_groups {
            if self.opt.cmd_aliases_delete {
                call_result = self._groups_aliases_delete(dry_run, &mut err);
            } else if self.opt.cmd_aliases_insert {
                call_result = self._groups_aliases_insert(dry_run, &mut err);
            } else if self.opt.cmd_aliases_list {
                call_result = self._groups_aliases_list(dry_run, &mut err);
            } else if self.opt.cmd_delete {
                call_result = self._groups_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._groups_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._groups_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._groups_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._groups_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._groups_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_members {
            if self.opt.cmd_delete {
                call_result = self._members_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._members_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._members_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._members_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._members_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._members_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_mobiledevices {
            if self.opt.cmd_action {
                call_result = self._mobiledevices_action(dry_run, &mut err);
            } else if self.opt.cmd_delete {
                call_result = self._mobiledevices_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._mobiledevices_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._mobiledevices_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_notifications {
            if self.opt.cmd_delete {
                call_result = self._notifications_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._notifications_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._notifications_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._notifications_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._notifications_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_orgunits {
            if self.opt.cmd_delete {
                call_result = self._orgunits_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._orgunits_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._orgunits_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._orgunits_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._orgunits_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._orgunits_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_schemas {
            if self.opt.cmd_delete {
                call_result = self._schemas_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._schemas_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._schemas_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._schemas_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._schemas_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._schemas_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_tokens {
            if self.opt.cmd_delete {
                call_result = self._tokens_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._tokens_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._tokens_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_users {
            if self.opt.cmd_aliases_delete {
                call_result = self._users_aliases_delete(dry_run, &mut err);
            } else if self.opt.cmd_aliases_insert {
                call_result = self._users_aliases_insert(dry_run, &mut err);
            } else if self.opt.cmd_aliases_list {
                call_result = self._users_aliases_list(dry_run, &mut err);
            } else if self.opt.cmd_aliases_watch {
                call_result = self._users_aliases_watch(dry_run, &mut err);
            } else if self.opt.cmd_delete {
                call_result = self._users_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._users_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._users_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._users_list(dry_run, &mut err);
            } else if self.opt.cmd_make_admin {
                call_result = self._users_make_admin(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._users_patch(dry_run, &mut err);
            } else if self.opt.cmd_photos_delete {
                call_result = self._users_photos_delete(dry_run, &mut err);
            } else if self.opt.cmd_photos_get {
                call_result = self._users_photos_get(dry_run, &mut err);
            } else if self.opt.cmd_photos_patch {
                call_result = self._users_photos_patch(dry_run, &mut err);
            } else if self.opt.cmd_photos_update {
                call_result = self._users_photos_update(dry_run, &mut err);
            } else if self.opt.cmd_undelete {
                call_result = self._users_undelete(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._users_update(dry_run, &mut err);
            } else if self.opt.cmd_watch {
                call_result = self._users_watch(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_verification_codes {
            if self.opt.cmd_generate {
                call_result = self._verification_codes_generate(dry_run, &mut err);
            } else if self.opt.cmd_invalidate {
                call_result = self._verification_codes_invalidate(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._verification_codes_list(dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "admin1-directory-secret.json") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                                      hyper::Client::new(),
                                      JsonTokenStorage {
                                        program_name: "admin1-directory",
                                        db_dir: config_dir.clone(),
                                      }, None);
        let engine = Engine {
            opt: opt,
            hub: api::Directory::new(hyper::Client::new(), auth),
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