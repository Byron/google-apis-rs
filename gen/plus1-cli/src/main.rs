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
extern crate google_plus1 as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  plus1 [options] activities get <activity-id> [-p <v>...] [-o <out>]
  plus1 [options] activities list <user-id> <collection> [-p <v>...] [-o <out>]
  plus1 [options] activities search <query> [-p <v>...] [-o <out>]
  plus1 [options] comments get <comment-id> [-p <v>...] [-o <out>]
  plus1 [options] comments list <activity-id> [-p <v>...] [-o <out>]
  plus1 [options] moments insert <user-id> <collection> -r <kv>... [-p <v>...] [-o <out>]
  plus1 [options] moments list <user-id> <collection> [-p <v>...] [-o <out>]
  plus1 [options] moments remove <id> [-p <v>...]
  plus1 [options] people get <user-id> [-p <v>...] [-o <out>]
  plus1 [options] people list <user-id> <collection> [-p <v>...] [-o <out>]
  plus1 [options] people list-by-activity <activity-id> <collection> [-p <v>...] [-o <out>]
  plus1 [options] people search <query> [-p <v>...] [-o <out>]
  plus1 --help

All documentation details can be found at
http://byron.github.io/google-apis-rs/google_plus1_cli/index.html

Configuration:
  --scope <url>  
            Specify the authentication a method should be executed in. Each scope 
            requires the user to grant this application permission to use it.
            If unset, it defaults to the shortest scope url for a particular method.
  --config-dir <folder>
            A directory into which we will store our persistent data. Defaults to 
            a user-writable directory that we will create during the first invocation.
            [default: ~/.google-service-cli]
  --debug
            Output all server communication to standard error. `tx` and `rx` are placed 
            into the same stream.
  --debug-auth
            Output all communication related to authentication to standard error. `tx` 
            and `rx` are placed into the same stream.
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
    hub: api::Plus<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _activities_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.activities().get(&self.opt.arg_activity_id);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _activities_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.activities().list(&self.opt.arg_user_id, &self.opt.arg_collection);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _activities_search(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.activities().search(&self.opt.arg_query);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _comments_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.comments().get(&self.opt.arg_comment_id);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _comments_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.comments().list(&self.opt.arg_activity_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "sort-order" => {
                    call = call.sort_order(value.unwrap_or(""));
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _moments_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Moment::default();
        let mut field_cursor = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
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
            fn request_object_init(request: &mut api::Moment) {
                if request.object.is_none() {
                    request.object = Some(Default::default());
                }
            }
            
            fn request_result_init(request: &mut api::Moment) {
                if request.result.is_none() {
                    request.result = Some(Default::default());
                }
            }
            
            fn request_target_init(request: &mut api::Moment) {
                if request.target.is_none() {
                    request.target = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "start-date" => {
                        request.start_date = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "target.start-date" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().start_date = Some(value.unwrap_or("").to_string());
                    },
                "target.end-date" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().end_date = Some(value.unwrap_or("").to_string());
                    },
                "target.text" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().text = Some(value.unwrap_or("").to_string());
                    },
                "target.image" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().image = Some(value.unwrap_or("").to_string());
                    },
                "target.birth-date" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().birth_date = Some(value.unwrap_or("").to_string());
                    },
                "target.date-published" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().date_published = Some(value.unwrap_or("").to_string());
                    },
                "target.address-locality" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().address_locality = Some(value.unwrap_or("").to_string());
                    },
                "target.additional-name" => {
                        request_target_init(&mut request);
                        if request.target.as_mut().unwrap().additional_name.is_none() {
                           request.target.as_mut().unwrap().additional_name = Some(Default::default());
                        }
                                        request.target.as_mut().unwrap().additional_name.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "target.worst-rating" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().worst_rating = Some(value.unwrap_or("").to_string());
                    },
                "target.duration" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().duration = Some(value.unwrap_or("").to_string());
                    },
                "target.thumbnail-url" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().thumbnail_url = Some(value.unwrap_or("").to_string());
                    },
                "target.id" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "target.post-office-box-number" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().post_office_box_number = Some(value.unwrap_or("").to_string());
                    },
                "target.caption" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().caption = Some(value.unwrap_or("").to_string());
                    },
                "target.best-rating" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().best_rating = Some(value.unwrap_or("").to_string());
                    },
                "target.address-country" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().address_country = Some(arg_from_str(value.unwrap_or("-0"), err, "target.address-country", "int64"));
                    },
                "target.width" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().width = Some(value.unwrap_or("").to_string());
                    },
                "target.street-address" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().street_address = Some(value.unwrap_or("").to_string());
                    },
                "target.latitude" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().latitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "target.latitude", "number"));
                    },
                "target.embed-url" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().embed_url = Some(value.unwrap_or("").to_string());
                    },
                "target.type" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "target.date-modified" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().date_modified = Some(value.unwrap_or("").to_string());
                    },
                "target.content-size" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().content_size = Some(value.unwrap_or("").to_string());
                    },
                "target.content-url" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().content_url = Some(value.unwrap_or("").to_string());
                    },
                "target.description" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().description = Some(value.unwrap_or("").to_string());
                    },
                "target.family-name" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().family_name = Some(value.unwrap_or("").to_string());
                    },
                "target.date-created" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().date_created = Some(value.unwrap_or("").to_string());
                    },
                "target.postal-code" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().postal_code = Some(value.unwrap_or("").to_string());
                    },
                "target.attendee-count" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().attendee_count = Some(arg_from_str(value.unwrap_or("-0"), err, "target.attendee-count", "integer"));
                    },
                "target.height" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().height = Some(value.unwrap_or("").to_string());
                    },
                "target.ticker-symbol" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().ticker_symbol = Some(value.unwrap_or("").to_string());
                    },
                "target.player-type" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().player_type = Some(value.unwrap_or("").to_string());
                    },
                "target.kind" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "target.name" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "target.url" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "target.gender" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().gender = Some(value.unwrap_or("").to_string());
                    },
                "target.longitude" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().longitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "target.longitude", "number"));
                    },
                "target.address-region" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().address_region = Some(value.unwrap_or("").to_string());
                    },
                "target.rating-value" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().rating_value = Some(value.unwrap_or("").to_string());
                    },
                "target.given-name" => {
                        request_target_init(&mut request);
                        request.target.as_mut().unwrap().given_name = Some(value.unwrap_or("").to_string());
                    },
                "object.start-date" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().start_date = Some(value.unwrap_or("").to_string());
                    },
                "object.end-date" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().end_date = Some(value.unwrap_or("").to_string());
                    },
                "object.text" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().text = Some(value.unwrap_or("").to_string());
                    },
                "object.image" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().image = Some(value.unwrap_or("").to_string());
                    },
                "object.birth-date" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().birth_date = Some(value.unwrap_or("").to_string());
                    },
                "object.date-published" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().date_published = Some(value.unwrap_or("").to_string());
                    },
                "object.address-locality" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().address_locality = Some(value.unwrap_or("").to_string());
                    },
                "object.additional-name" => {
                        request_object_init(&mut request);
                        if request.object.as_mut().unwrap().additional_name.is_none() {
                           request.object.as_mut().unwrap().additional_name = Some(Default::default());
                        }
                                        request.object.as_mut().unwrap().additional_name.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "object.worst-rating" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().worst_rating = Some(value.unwrap_or("").to_string());
                    },
                "object.duration" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().duration = Some(value.unwrap_or("").to_string());
                    },
                "object.thumbnail-url" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().thumbnail_url = Some(value.unwrap_or("").to_string());
                    },
                "object.id" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "object.post-office-box-number" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().post_office_box_number = Some(value.unwrap_or("").to_string());
                    },
                "object.caption" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().caption = Some(value.unwrap_or("").to_string());
                    },
                "object.best-rating" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().best_rating = Some(value.unwrap_or("").to_string());
                    },
                "object.address-country" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().address_country = Some(arg_from_str(value.unwrap_or("-0"), err, "object.address-country", "int64"));
                    },
                "object.width" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().width = Some(value.unwrap_or("").to_string());
                    },
                "object.street-address" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().street_address = Some(value.unwrap_or("").to_string());
                    },
                "object.latitude" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().latitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "object.latitude", "number"));
                    },
                "object.embed-url" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().embed_url = Some(value.unwrap_or("").to_string());
                    },
                "object.type" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "object.date-modified" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().date_modified = Some(value.unwrap_or("").to_string());
                    },
                "object.content-size" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().content_size = Some(value.unwrap_or("").to_string());
                    },
                "object.content-url" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().content_url = Some(value.unwrap_or("").to_string());
                    },
                "object.description" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().description = Some(value.unwrap_or("").to_string());
                    },
                "object.family-name" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().family_name = Some(value.unwrap_or("").to_string());
                    },
                "object.date-created" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().date_created = Some(value.unwrap_or("").to_string());
                    },
                "object.postal-code" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().postal_code = Some(value.unwrap_or("").to_string());
                    },
                "object.attendee-count" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().attendee_count = Some(arg_from_str(value.unwrap_or("-0"), err, "object.attendee-count", "integer"));
                    },
                "object.height" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().height = Some(value.unwrap_or("").to_string());
                    },
                "object.ticker-symbol" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().ticker_symbol = Some(value.unwrap_or("").to_string());
                    },
                "object.player-type" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().player_type = Some(value.unwrap_or("").to_string());
                    },
                "object.kind" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "object.name" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "object.url" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "object.gender" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().gender = Some(value.unwrap_or("").to_string());
                    },
                "object.longitude" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().longitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "object.longitude", "number"));
                    },
                "object.address-region" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().address_region = Some(value.unwrap_or("").to_string());
                    },
                "object.rating-value" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().rating_value = Some(value.unwrap_or("").to_string());
                    },
                "object.given-name" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().given_name = Some(value.unwrap_or("").to_string());
                    },
                "result.start-date" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().start_date = Some(value.unwrap_or("").to_string());
                    },
                "result.end-date" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().end_date = Some(value.unwrap_or("").to_string());
                    },
                "result.text" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().text = Some(value.unwrap_or("").to_string());
                    },
                "result.image" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().image = Some(value.unwrap_or("").to_string());
                    },
                "result.birth-date" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().birth_date = Some(value.unwrap_or("").to_string());
                    },
                "result.date-published" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().date_published = Some(value.unwrap_or("").to_string());
                    },
                "result.address-locality" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().address_locality = Some(value.unwrap_or("").to_string());
                    },
                "result.additional-name" => {
                        request_result_init(&mut request);
                        if request.result.as_mut().unwrap().additional_name.is_none() {
                           request.result.as_mut().unwrap().additional_name = Some(Default::default());
                        }
                                        request.result.as_mut().unwrap().additional_name.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "result.worst-rating" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().worst_rating = Some(value.unwrap_or("").to_string());
                    },
                "result.duration" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().duration = Some(value.unwrap_or("").to_string());
                    },
                "result.thumbnail-url" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().thumbnail_url = Some(value.unwrap_or("").to_string());
                    },
                "result.id" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "result.post-office-box-number" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().post_office_box_number = Some(value.unwrap_or("").to_string());
                    },
                "result.caption" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().caption = Some(value.unwrap_or("").to_string());
                    },
                "result.best-rating" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().best_rating = Some(value.unwrap_or("").to_string());
                    },
                "result.address-country" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().address_country = Some(arg_from_str(value.unwrap_or("-0"), err, "result.address-country", "int64"));
                    },
                "result.width" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().width = Some(value.unwrap_or("").to_string());
                    },
                "result.street-address" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().street_address = Some(value.unwrap_or("").to_string());
                    },
                "result.latitude" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().latitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "result.latitude", "number"));
                    },
                "result.embed-url" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().embed_url = Some(value.unwrap_or("").to_string());
                    },
                "result.type" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().type_ = Some(value.unwrap_or("").to_string());
                    },
                "result.date-modified" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().date_modified = Some(value.unwrap_or("").to_string());
                    },
                "result.content-size" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().content_size = Some(value.unwrap_or("").to_string());
                    },
                "result.content-url" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().content_url = Some(value.unwrap_or("").to_string());
                    },
                "result.description" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().description = Some(value.unwrap_or("").to_string());
                    },
                "result.family-name" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().family_name = Some(value.unwrap_or("").to_string());
                    },
                "result.date-created" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().date_created = Some(value.unwrap_or("").to_string());
                    },
                "result.postal-code" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().postal_code = Some(value.unwrap_or("").to_string());
                    },
                "result.attendee-count" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().attendee_count = Some(arg_from_str(value.unwrap_or("-0"), err, "result.attendee-count", "integer"));
                    },
                "result.height" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().height = Some(value.unwrap_or("").to_string());
                    },
                "result.ticker-symbol" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().ticker_symbol = Some(value.unwrap_or("").to_string());
                    },
                "result.player-type" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().player_type = Some(value.unwrap_or("").to_string());
                    },
                "result.kind" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "result.name" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().name = Some(value.unwrap_or("").to_string());
                    },
                "result.url" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "result.gender" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().gender = Some(value.unwrap_or("").to_string());
                    },
                "result.longitude" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().longitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "result.longitude", "number"));
                    },
                "result.address-region" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().address_region = Some(value.unwrap_or("").to_string());
                    },
                "result.rating-value" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().rating_value = Some(value.unwrap_or("").to_string());
                    },
                "result.given-name" => {
                        request_result_init(&mut request);
                        request.result.as_mut().unwrap().given_name = Some(value.unwrap_or("").to_string());
                    },
                "type" => {
                        request_result_init(&mut request);
                        request.type_ = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_result_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.moments().insert(request, &self.opt.arg_user_id, &self.opt.arg_collection);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "debug" => {
                    call = call.debug(arg_from_str(value.unwrap_or("false"), err, "debug", "boolean"));
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _moments_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.moments().list(&self.opt.arg_user_id, &self.opt.arg_collection);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "type" => {
                    call = call.type_(value.unwrap_or(""));
                },
                "target-url" => {
                    call = call.target_url(value.unwrap_or(""));
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _moments_remove(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.moments().remove(&self.opt.arg_id);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
            match match protocol {
                "standard-request" => call.doit(),
                _ => unreachable!(),
            } {
                Err(api_err) => Some(api_err),
                Ok(mut response) => {
                    None
                }
            }
        }
    }

    fn _people_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.people().get(&self.opt.arg_user_id);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _people_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.people().list(&self.opt.arg_user_id, &self.opt.arg_collection);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _people_list_by_activity(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.people().list_by_activity(&self.opt.arg_activity_id, &self.opt.arg_collection);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _people_search(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.people().search(&self.opt.arg_query);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

        if self.opt.cmd_activities {
            if self.opt.cmd_get {
                call_result = self._activities_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._activities_list(dry_run, &mut err);
            } else if self.opt.cmd_search {
                call_result = self._activities_search(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_comments {
            if self.opt.cmd_get {
                call_result = self._comments_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._comments_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_moments {
            if self.opt.cmd_insert {
                call_result = self._moments_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._moments_list(dry_run, &mut err);
            } else if self.opt.cmd_remove {
                call_result = self._moments_remove(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_people {
            if self.opt.cmd_get {
                call_result = self._people_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._people_list(dry_run, &mut err);
            } else if self.opt.cmd_list_by_activity {
                call_result = self._people_list_by_activity(dry_run, &mut err);
            } else if self.opt.cmd_search {
                call_result = self._people_search(dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "plus1-secret.json", 
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
                                          program_name: "plus1",
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
            hub: api::Plus::new(client, auth),
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
    let debug = opts.flag_debug;
    match Engine::new(opts) {
        Err(err) => {
            writeln!(io::stderr(), "{}", err).ok();
            env::set_exit_status(err.exit_code);
        },
        Ok(engine) => {
            if let Some(err) = engine.doit() {
                if debug {
                    writeln!(io::stderr(), "{:?}", err).ok();
                } else {
                    writeln!(io::stderr(), "{}", err).ok();
                }
                env::set_exit_status(1);
            }
        }
    }
}