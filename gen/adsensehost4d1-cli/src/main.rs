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
extern crate google_adsensehost4d1 as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  adsensehost4d1 [options] accounts adclients-get <account-id> <ad-client-id> [-p <v>]... [-o <out>]
  adsensehost4d1 [options] accounts adclients-list <account-id> [-p <v>]... [-o <out>]
  adsensehost4d1 [options] accounts adunits-delete <account-id> <ad-client-id> <ad-unit-id> [-p <v>]... [-o <out>]
  adsensehost4d1 [options] accounts adunits-get <account-id> <ad-client-id> <ad-unit-id> [-p <v>]... [-o <out>]
  adsensehost4d1 [options] accounts adunits-get-ad-code <account-id> <ad-client-id> <ad-unit-id> [-p <v>]... [-o <out>]
  adsensehost4d1 [options] accounts adunits-insert <account-id> <ad-client-id> -r <kv>... [-p <v>]... [-o <out>]
  adsensehost4d1 [options] accounts adunits-list <account-id> <ad-client-id> [-p <v>]... [-o <out>]
  adsensehost4d1 [options] accounts adunits-patch <account-id> <ad-client-id> <ad-unit-id> -r <kv>... [-p <v>]... [-o <out>]
  adsensehost4d1 [options] accounts adunits-update <account-id> <ad-client-id> -r <kv>... [-p <v>]... [-o <out>]
  adsensehost4d1 [options] accounts get <account-id> [-p <v>]... [-o <out>]
  adsensehost4d1 [options] accounts list <filter-ad-client-id>... [-p <v>]... [-o <out>]
  adsensehost4d1 [options] accounts reports-generate <account-id> <start-date> <end-date> [-p <v>]... [-o <out>]
  adsensehost4d1 [options] adclients get <ad-client-id> [-p <v>]... [-o <out>]
  adsensehost4d1 [options] adclients list [-p <v>]... [-o <out>]
  adsensehost4d1 [options] associationsessions start <product-code>... <website-url> [-p <v>]... [-o <out>]
  adsensehost4d1 [options] associationsessions verify <token> [-p <v>]... [-o <out>]
  adsensehost4d1 [options] customchannels delete <ad-client-id> <custom-channel-id> [-p <v>]... [-o <out>]
  adsensehost4d1 [options] customchannels get <ad-client-id> <custom-channel-id> [-p <v>]... [-o <out>]
  adsensehost4d1 [options] customchannels insert <ad-client-id> -r <kv>... [-p <v>]... [-o <out>]
  adsensehost4d1 [options] customchannels list <ad-client-id> [-p <v>]... [-o <out>]
  adsensehost4d1 [options] customchannels patch <ad-client-id> <custom-channel-id> -r <kv>... [-p <v>]... [-o <out>]
  adsensehost4d1 [options] customchannels update <ad-client-id> -r <kv>... [-p <v>]... [-o <out>]
  adsensehost4d1 [options] reports generate <start-date> <end-date> [-p <v>]... [-o <out>]
  adsensehost4d1 [options] urlchannels delete <ad-client-id> <url-channel-id> [-p <v>]... [-o <out>]
  adsensehost4d1 [options] urlchannels insert <ad-client-id> -r <kv>... [-p <v>]... [-o <out>]
  adsensehost4d1 [options] urlchannels list <ad-client-id> [-p <v>]... [-o <out>]
  adsensehost4d1 --help

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
    hub: api::AdSenseHost<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _accounts_adclients_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().adclients_get(&self.opt.arg_account_id, &self.opt.arg_ad_client_id);
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

    fn _accounts_adclients_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().adclients_list(&self.opt.arg_account_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _accounts_adunits_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().adunits_delete(&self.opt.arg_account_id, &self.opt.arg_ad_client_id, &self.opt.arg_ad_unit_id);
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

    fn _accounts_adunits_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().adunits_get(&self.opt.arg_account_id, &self.opt.arg_ad_client_id, &self.opt.arg_ad_unit_id);
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

    fn _accounts_adunits_get_ad_code(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().adunits_get_ad_code(&self.opt.arg_account_id, &self.opt.arg_ad_client_id, &self.opt.arg_ad_unit_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "host-custom-channel-id" => {
                    call = call.add_host_custom_channel_id(value.unwrap_or(""));
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _accounts_adunits_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::AdUnit::default();
        let mut call = self.hub.accounts().adunits_insert(&request, &self.opt.arg_account_id, &self.opt.arg_ad_client_id);
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
            fn request_content_ads_settings_backup_option_init(request: &mut api::AdUnit) {
                request_content_ads_settings_init(request);
                if request.content_ads_settings.as_mut().unwrap().backup_option.is_none() {
                    request.content_ads_settings.as_mut().unwrap().backup_option = Some(Default::default());
                }
            }
            
            fn request_content_ads_settings_init(request: &mut api::AdUnit) {
                if request.content_ads_settings.is_none() {
                    request.content_ads_settings = Some(Default::default());
                }
            }
            
            fn request_custom_style_colors_init(request: &mut api::AdUnit) {
                request_custom_style_init(request);
                if request.custom_style.as_mut().unwrap().colors.is_none() {
                    request.custom_style.as_mut().unwrap().colors = Some(Default::default());
                }
            }
            
            fn request_custom_style_font_init(request: &mut api::AdUnit) {
                request_custom_style_init(request);
                if request.custom_style.as_mut().unwrap().font.is_none() {
                    request.custom_style.as_mut().unwrap().font = Some(Default::default());
                }
            }
            
            fn request_custom_style_init(request: &mut api::AdUnit) {
                if request.custom_style.is_none() {
                    request.custom_style = Some(Default::default());
                }
            }
            
            fn request_mobile_content_ads_settings_init(request: &mut api::AdUnit) {
                if request.mobile_content_ads_settings.is_none() {
                    request.mobile_content_ads_settings = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "status" => {
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "code" => {
                        request.code = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "content-ads-settings.type" => {
                        request_content_ads_settings_init(&mut request);
                        request.content_ads_settings.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "content-ads-settings.backup-option.color" => {
                        request_content_ads_settings_backup_option_init(&mut request);
                        request.content_ads_settings.as_mut().unwrap().backup_option.as_mut().unwrap().color = Some(value.unwrap_or("").to_string());
                    },
                "content-ads-settings.backup-option.url" => {
                        request_content_ads_settings_backup_option_init(&mut request);
                        request.content_ads_settings.as_mut().unwrap().backup_option.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "content-ads-settings.backup-option.type" => {
                        request_content_ads_settings_backup_option_init(&mut request);
                        request.content_ads_settings.as_mut().unwrap().backup_option.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "content-ads-settings.size" => {
                        request_content_ads_settings_backup_option_init(&mut request);
                        request.content_ads_settings.as_mut().unwrap().size = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_content_ads_settings_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "mobile-content-ads-settings.scripting-language" => {
                        request_mobile_content_ads_settings_init(&mut request);
                        request.mobile_content_ads_settings.as_mut().unwrap().scripting_language = Some(value.unwrap_or("").to_string());
                    },
                "mobile-content-ads-settings.type" => {
                        request_mobile_content_ads_settings_init(&mut request);
                        request.mobile_content_ads_settings.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "mobile-content-ads-settings.markup-language" => {
                        request_mobile_content_ads_settings_init(&mut request);
                        request.mobile_content_ads_settings.as_mut().unwrap().markup_language = Some(value.unwrap_or("").to_string());
                    },
                "mobile-content-ads-settings.size" => {
                        request_mobile_content_ads_settings_init(&mut request);
                        request.mobile_content_ads_settings.as_mut().unwrap().size = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.corners" => {
                        request_custom_style_init(&mut request);
                        request.custom_style.as_mut().unwrap().corners = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.colors.url" => {
                        request_custom_style_colors_init(&mut request);
                        request.custom_style.as_mut().unwrap().colors.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.colors.text" => {
                        request_custom_style_colors_init(&mut request);
                        request.custom_style.as_mut().unwrap().colors.as_mut().unwrap().text = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.colors.border" => {
                        request_custom_style_colors_init(&mut request);
                        request.custom_style.as_mut().unwrap().colors.as_mut().unwrap().border = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.colors.background" => {
                        request_custom_style_colors_init(&mut request);
                        request.custom_style.as_mut().unwrap().colors.as_mut().unwrap().background = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.colors.title" => {
                        request_custom_style_colors_init(&mut request);
                        request.custom_style.as_mut().unwrap().colors.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.font.family" => {
                        request_custom_style_font_init(&mut request);
                        request.custom_style.as_mut().unwrap().font.as_mut().unwrap().family = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.font.size" => {
                        request_custom_style_font_init(&mut request);
                        request.custom_style.as_mut().unwrap().font.as_mut().unwrap().size = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.kind" => {
                        request_custom_style_font_init(&mut request);
                        request.custom_style.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
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

    fn _accounts_adunits_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().adunits_list(&self.opt.arg_account_id, &self.opt.arg_ad_client_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "include-inactive" => {
                    call = call.include_inactive(arg_from_str(value.unwrap_or("false"), err, "include-inactive", "boolean"));
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _accounts_adunits_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::AdUnit::default();
        let mut call = self.hub.accounts().adunits_patch(&request, &self.opt.arg_account_id, &self.opt.arg_ad_client_id, &self.opt.arg_ad_unit_id);
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
            fn request_content_ads_settings_backup_option_init(request: &mut api::AdUnit) {
                request_content_ads_settings_init(request);
                if request.content_ads_settings.as_mut().unwrap().backup_option.is_none() {
                    request.content_ads_settings.as_mut().unwrap().backup_option = Some(Default::default());
                }
            }
            
            fn request_content_ads_settings_init(request: &mut api::AdUnit) {
                if request.content_ads_settings.is_none() {
                    request.content_ads_settings = Some(Default::default());
                }
            }
            
            fn request_custom_style_colors_init(request: &mut api::AdUnit) {
                request_custom_style_init(request);
                if request.custom_style.as_mut().unwrap().colors.is_none() {
                    request.custom_style.as_mut().unwrap().colors = Some(Default::default());
                }
            }
            
            fn request_custom_style_font_init(request: &mut api::AdUnit) {
                request_custom_style_init(request);
                if request.custom_style.as_mut().unwrap().font.is_none() {
                    request.custom_style.as_mut().unwrap().font = Some(Default::default());
                }
            }
            
            fn request_custom_style_init(request: &mut api::AdUnit) {
                if request.custom_style.is_none() {
                    request.custom_style = Some(Default::default());
                }
            }
            
            fn request_mobile_content_ads_settings_init(request: &mut api::AdUnit) {
                if request.mobile_content_ads_settings.is_none() {
                    request.mobile_content_ads_settings = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "status" => {
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "code" => {
                        request.code = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "content-ads-settings.type" => {
                        request_content_ads_settings_init(&mut request);
                        request.content_ads_settings.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "content-ads-settings.backup-option.color" => {
                        request_content_ads_settings_backup_option_init(&mut request);
                        request.content_ads_settings.as_mut().unwrap().backup_option.as_mut().unwrap().color = Some(value.unwrap_or("").to_string());
                    },
                "content-ads-settings.backup-option.url" => {
                        request_content_ads_settings_backup_option_init(&mut request);
                        request.content_ads_settings.as_mut().unwrap().backup_option.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "content-ads-settings.backup-option.type" => {
                        request_content_ads_settings_backup_option_init(&mut request);
                        request.content_ads_settings.as_mut().unwrap().backup_option.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "content-ads-settings.size" => {
                        request_content_ads_settings_backup_option_init(&mut request);
                        request.content_ads_settings.as_mut().unwrap().size = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_content_ads_settings_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "mobile-content-ads-settings.scripting-language" => {
                        request_mobile_content_ads_settings_init(&mut request);
                        request.mobile_content_ads_settings.as_mut().unwrap().scripting_language = Some(value.unwrap_or("").to_string());
                    },
                "mobile-content-ads-settings.type" => {
                        request_mobile_content_ads_settings_init(&mut request);
                        request.mobile_content_ads_settings.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "mobile-content-ads-settings.markup-language" => {
                        request_mobile_content_ads_settings_init(&mut request);
                        request.mobile_content_ads_settings.as_mut().unwrap().markup_language = Some(value.unwrap_or("").to_string());
                    },
                "mobile-content-ads-settings.size" => {
                        request_mobile_content_ads_settings_init(&mut request);
                        request.mobile_content_ads_settings.as_mut().unwrap().size = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.corners" => {
                        request_custom_style_init(&mut request);
                        request.custom_style.as_mut().unwrap().corners = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.colors.url" => {
                        request_custom_style_colors_init(&mut request);
                        request.custom_style.as_mut().unwrap().colors.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.colors.text" => {
                        request_custom_style_colors_init(&mut request);
                        request.custom_style.as_mut().unwrap().colors.as_mut().unwrap().text = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.colors.border" => {
                        request_custom_style_colors_init(&mut request);
                        request.custom_style.as_mut().unwrap().colors.as_mut().unwrap().border = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.colors.background" => {
                        request_custom_style_colors_init(&mut request);
                        request.custom_style.as_mut().unwrap().colors.as_mut().unwrap().background = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.colors.title" => {
                        request_custom_style_colors_init(&mut request);
                        request.custom_style.as_mut().unwrap().colors.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.font.family" => {
                        request_custom_style_font_init(&mut request);
                        request.custom_style.as_mut().unwrap().font.as_mut().unwrap().family = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.font.size" => {
                        request_custom_style_font_init(&mut request);
                        request.custom_style.as_mut().unwrap().font.as_mut().unwrap().size = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.kind" => {
                        request_custom_style_font_init(&mut request);
                        request.custom_style.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
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

    fn _accounts_adunits_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::AdUnit::default();
        let mut call = self.hub.accounts().adunits_update(&request, &self.opt.arg_account_id, &self.opt.arg_ad_client_id);
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
            fn request_content_ads_settings_backup_option_init(request: &mut api::AdUnit) {
                request_content_ads_settings_init(request);
                if request.content_ads_settings.as_mut().unwrap().backup_option.is_none() {
                    request.content_ads_settings.as_mut().unwrap().backup_option = Some(Default::default());
                }
            }
            
            fn request_content_ads_settings_init(request: &mut api::AdUnit) {
                if request.content_ads_settings.is_none() {
                    request.content_ads_settings = Some(Default::default());
                }
            }
            
            fn request_custom_style_colors_init(request: &mut api::AdUnit) {
                request_custom_style_init(request);
                if request.custom_style.as_mut().unwrap().colors.is_none() {
                    request.custom_style.as_mut().unwrap().colors = Some(Default::default());
                }
            }
            
            fn request_custom_style_font_init(request: &mut api::AdUnit) {
                request_custom_style_init(request);
                if request.custom_style.as_mut().unwrap().font.is_none() {
                    request.custom_style.as_mut().unwrap().font = Some(Default::default());
                }
            }
            
            fn request_custom_style_init(request: &mut api::AdUnit) {
                if request.custom_style.is_none() {
                    request.custom_style = Some(Default::default());
                }
            }
            
            fn request_mobile_content_ads_settings_init(request: &mut api::AdUnit) {
                if request.mobile_content_ads_settings.is_none() {
                    request.mobile_content_ads_settings = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "status" => {
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "code" => {
                        request.code = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "content-ads-settings.type" => {
                        request_content_ads_settings_init(&mut request);
                        request.content_ads_settings.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "content-ads-settings.backup-option.color" => {
                        request_content_ads_settings_backup_option_init(&mut request);
                        request.content_ads_settings.as_mut().unwrap().backup_option.as_mut().unwrap().color = Some(value.unwrap_or("").to_string());
                    },
                "content-ads-settings.backup-option.url" => {
                        request_content_ads_settings_backup_option_init(&mut request);
                        request.content_ads_settings.as_mut().unwrap().backup_option.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "content-ads-settings.backup-option.type" => {
                        request_content_ads_settings_backup_option_init(&mut request);
                        request.content_ads_settings.as_mut().unwrap().backup_option.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "content-ads-settings.size" => {
                        request_content_ads_settings_backup_option_init(&mut request);
                        request.content_ads_settings.as_mut().unwrap().size = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_content_ads_settings_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "mobile-content-ads-settings.scripting-language" => {
                        request_mobile_content_ads_settings_init(&mut request);
                        request.mobile_content_ads_settings.as_mut().unwrap().scripting_language = Some(value.unwrap_or("").to_string());
                    },
                "mobile-content-ads-settings.type" => {
                        request_mobile_content_ads_settings_init(&mut request);
                        request.mobile_content_ads_settings.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "mobile-content-ads-settings.markup-language" => {
                        request_mobile_content_ads_settings_init(&mut request);
                        request.mobile_content_ads_settings.as_mut().unwrap().markup_language = Some(value.unwrap_or("").to_string());
                    },
                "mobile-content-ads-settings.size" => {
                        request_mobile_content_ads_settings_init(&mut request);
                        request.mobile_content_ads_settings.as_mut().unwrap().size = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.corners" => {
                        request_custom_style_init(&mut request);
                        request.custom_style.as_mut().unwrap().corners = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.colors.url" => {
                        request_custom_style_colors_init(&mut request);
                        request.custom_style.as_mut().unwrap().colors.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.colors.text" => {
                        request_custom_style_colors_init(&mut request);
                        request.custom_style.as_mut().unwrap().colors.as_mut().unwrap().text = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.colors.border" => {
                        request_custom_style_colors_init(&mut request);
                        request.custom_style.as_mut().unwrap().colors.as_mut().unwrap().border = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.colors.background" => {
                        request_custom_style_colors_init(&mut request);
                        request.custom_style.as_mut().unwrap().colors.as_mut().unwrap().background = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.colors.title" => {
                        request_custom_style_colors_init(&mut request);
                        request.custom_style.as_mut().unwrap().colors.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.font.family" => {
                        request_custom_style_font_init(&mut request);
                        request.custom_style.as_mut().unwrap().font.as_mut().unwrap().family = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.font.size" => {
                        request_custom_style_font_init(&mut request);
                        request.custom_style.as_mut().unwrap().font.as_mut().unwrap().size = Some(value.unwrap_or("").to_string());
                    },
                "custom-style.kind" => {
                        request_custom_style_font_init(&mut request);
                        request.custom_style.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
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

    fn _accounts_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().get(&self.opt.arg_account_id);
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

    fn _accounts_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().list(&self.opt.arg_filter_ad_client_id);
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

    fn _accounts_reports_generate(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.accounts().reports_generate(&self.opt.arg_account_id, &self.opt.arg_start_date, &self.opt.arg_end_date);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
                },
                "sort" => {
                    call = call.add_sort(value.unwrap_or(""));
                },
                "metric" => {
                    call = call.add_metric(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "locale" => {
                    call = call.locale(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.add_filter(value.unwrap_or(""));
                },
                "dimension" => {
                    call = call.add_dimension(value.unwrap_or(""));
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _adclients_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.adclients().get(&self.opt.arg_ad_client_id);
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

    fn _adclients_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.adclients().list();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _associationsessions_start(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.associationsessions().start(&self.opt.arg_product_code, &self.opt.arg_website_url);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "website-locale" => {
                    call = call.website_locale(value.unwrap_or(""));
                },
                "user-locale" => {
                    call = call.user_locale(value.unwrap_or(""));
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _associationsessions_verify(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.associationsessions().verify(&self.opt.arg_token);
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

    fn _customchannels_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.customchannels().delete(&self.opt.arg_ad_client_id, &self.opt.arg_custom_channel_id);
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

    fn _customchannels_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.customchannels().get(&self.opt.arg_ad_client_id, &self.opt.arg_custom_channel_id);
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

    fn _customchannels_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::CustomChannel::default();
        let mut call = self.hub.customchannels().insert(&request, &self.opt.arg_ad_client_id);
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
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "code" => {
                        request.code = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
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

    fn _customchannels_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.customchannels().list(&self.opt.arg_ad_client_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _customchannels_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::CustomChannel::default();
        let mut call = self.hub.customchannels().patch(&request, &self.opt.arg_ad_client_id, &self.opt.arg_custom_channel_id);
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
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "code" => {
                        request.code = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
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

    fn _customchannels_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::CustomChannel::default();
        let mut call = self.hub.customchannels().update(&request, &self.opt.arg_ad_client_id);
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
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "code" => {
                        request.code = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
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

    fn _reports_generate(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.reports().generate(&self.opt.arg_start_date, &self.opt.arg_end_date);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
                },
                "sort" => {
                    call = call.add_sort(value.unwrap_or(""));
                },
                "metric" => {
                    call = call.add_metric(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "locale" => {
                    call = call.locale(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.add_filter(value.unwrap_or(""));
                },
                "dimension" => {
                    call = call.add_dimension(value.unwrap_or(""));
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _urlchannels_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.urlchannels().delete(&self.opt.arg_ad_client_id, &self.opt.arg_url_channel_id);
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

    fn _urlchannels_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::UrlChannel::default();
        let mut call = self.hub.urlchannels().insert(&request, &self.opt.arg_ad_client_id);
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
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "url-pattern" => {
                        request.url_pattern = Some(value.unwrap_or("").to_string());
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

    fn _urlchannels_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.urlchannels().list(&self.opt.arg_ad_client_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
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

        if self.opt.cmd_accounts {
            if self.opt.cmd_adclients_get {
                call_result = self._accounts_adclients_get(dry_run, &mut err);
            } else if self.opt.cmd_adclients_list {
                call_result = self._accounts_adclients_list(dry_run, &mut err);
            } else if self.opt.cmd_adunits_delete {
                call_result = self._accounts_adunits_delete(dry_run, &mut err);
            } else if self.opt.cmd_adunits_get {
                call_result = self._accounts_adunits_get(dry_run, &mut err);
            } else if self.opt.cmd_adunits_get_ad_code {
                call_result = self._accounts_adunits_get_ad_code(dry_run, &mut err);
            } else if self.opt.cmd_adunits_insert {
                call_result = self._accounts_adunits_insert(dry_run, &mut err);
            } else if self.opt.cmd_adunits_list {
                call_result = self._accounts_adunits_list(dry_run, &mut err);
            } else if self.opt.cmd_adunits_patch {
                call_result = self._accounts_adunits_patch(dry_run, &mut err);
            } else if self.opt.cmd_adunits_update {
                call_result = self._accounts_adunits_update(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._accounts_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._accounts_list(dry_run, &mut err);
            } else if self.opt.cmd_reports_generate {
                call_result = self._accounts_reports_generate(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_adclients {
            if self.opt.cmd_get {
                call_result = self._adclients_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._adclients_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_associationsessions {
            if self.opt.cmd_start {
                call_result = self._associationsessions_start(dry_run, &mut err);
            } else if self.opt.cmd_verify {
                call_result = self._associationsessions_verify(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_customchannels {
            if self.opt.cmd_delete {
                call_result = self._customchannels_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._customchannels_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._customchannels_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._customchannels_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._customchannels_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._customchannels_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_reports {
            if self.opt.cmd_generate {
                call_result = self._reports_generate(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_urlchannels {
            if self.opt.cmd_delete {
                call_result = self._urlchannels_delete(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._urlchannels_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._urlchannels_list(dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "adsensehost4d1-secret.json", 
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
                                          program_name: "adsensehost4d1",
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
            hub: api::AdSenseHost::new(client, auth),
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