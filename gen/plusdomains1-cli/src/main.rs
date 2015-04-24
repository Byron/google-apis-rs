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
extern crate google_plusdomains1 as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  plusdomains1 [options] activities get <activity-id> [-p <v>]... [-o <out>]
  plusdomains1 [options] activities insert <user-id> -r <kv>... [-p <v>]... [-o <out>]
  plusdomains1 [options] activities list <user-id> <collection> [-p <v>]... [-o <out>]
  plusdomains1 [options] audiences list <user-id> [-p <v>]... [-o <out>]
  plusdomains1 [options] circles add-people <circle-id> [-p <v>]... [-o <out>]
  plusdomains1 [options] circles get <circle-id> [-p <v>]... [-o <out>]
  plusdomains1 [options] circles insert <user-id> -r <kv>... [-p <v>]... [-o <out>]
  plusdomains1 [options] circles list <user-id> [-p <v>]... [-o <out>]
  plusdomains1 [options] circles patch <circle-id> -r <kv>... [-p <v>]... [-o <out>]
  plusdomains1 [options] circles remove <circle-id> [-p <v>]...
  plusdomains1 [options] circles remove-people <circle-id> [-p <v>]...
  plusdomains1 [options] circles update <circle-id> -r <kv>... [-p <v>]... [-o <out>]
  plusdomains1 [options] comments get <comment-id> [-p <v>]... [-o <out>]
  plusdomains1 [options] comments insert <activity-id> -r <kv>... [-p <v>]... [-o <out>]
  plusdomains1 [options] comments list <activity-id> [-p <v>]... [-o <out>]
  plusdomains1 [options] media insert <user-id> <collection> -r <kv>... -u (simple|resumable) <file> <mime> [-p <v>]... [-o <out>]
  plusdomains1 [options] people get <user-id> [-p <v>]... [-o <out>]
  plusdomains1 [options] people list <user-id> <collection> [-p <v>]... [-o <out>]
  plusdomains1 [options] people list-by-activity <activity-id> <collection> [-p <v>]... [-o <out>]
  plusdomains1 [options] people list-by-circle <circle-id> [-p <v>]... [-o <out>]
  plusdomains1 --help

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
    hub: api::PlusDomains<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
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

    fn _activities_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Activity::default();
        let mut call = self.hub.activities().insert(&request, &self.opt.arg_user_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "preview" => {
                    call = call.preview(arg_from_str(value.unwrap_or("false"), err, "preview", "boolean"));
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
        
        let mut field_name = FieldCursor::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_access_init(request: &mut api::Activity) {
                if request.access.is_none() {
                    request.access = Some(Default::default());
                }
            }
            
            fn request_actor_image_init(request: &mut api::Activity) {
                request_actor_init(request);
                if request.actor.as_mut().unwrap().image.is_none() {
                    request.actor.as_mut().unwrap().image = Some(Default::default());
                }
            }
            
            fn request_actor_init(request: &mut api::Activity) {
                if request.actor.is_none() {
                    request.actor = Some(Default::default());
                }
            }
            
            fn request_actor_name_init(request: &mut api::Activity) {
                request_actor_init(request);
                if request.actor.as_mut().unwrap().name.is_none() {
                    request.actor.as_mut().unwrap().name = Some(Default::default());
                }
            }
            
            fn request_location_address_init(request: &mut api::Activity) {
                request_location_init(request);
                if request.location.as_mut().unwrap().address.is_none() {
                    request.location.as_mut().unwrap().address = Some(Default::default());
                }
            }
            
            fn request_location_init(request: &mut api::Activity) {
                if request.location.is_none() {
                    request.location = Some(Default::default());
                }
            }
            
            fn request_location_position_init(request: &mut api::Activity) {
                request_location_init(request);
                if request.location.as_mut().unwrap().position.is_none() {
                    request.location.as_mut().unwrap().position = Some(Default::default());
                }
            }
            
            fn request_object_actor_image_init(request: &mut api::Activity) {
                request_object_actor_init(request);
                if request.object.as_mut().unwrap().actor.as_mut().unwrap().image.is_none() {
                    request.object.as_mut().unwrap().actor.as_mut().unwrap().image = Some(Default::default());
                }
            }
            
            fn request_object_actor_init(request: &mut api::Activity) {
                request_object_init(request);
                if request.object.as_mut().unwrap().actor.is_none() {
                    request.object.as_mut().unwrap().actor = Some(Default::default());
                }
            }
            
            fn request_object_init(request: &mut api::Activity) {
                if request.object.is_none() {
                    request.object = Some(Default::default());
                }
            }
            
            fn request_object_plusoners_init(request: &mut api::Activity) {
                request_object_init(request);
                if request.object.as_mut().unwrap().plusoners.is_none() {
                    request.object.as_mut().unwrap().plusoners = Some(Default::default());
                }
            }
            
            fn request_object_replies_init(request: &mut api::Activity) {
                request_object_init(request);
                if request.object.as_mut().unwrap().replies.is_none() {
                    request.object.as_mut().unwrap().replies = Some(Default::default());
                }
            }
            
            fn request_object_resharers_init(request: &mut api::Activity) {
                request_object_init(request);
                if request.object.as_mut().unwrap().resharers.is_none() {
                    request.object.as_mut().unwrap().resharers = Some(Default::default());
                }
            }
            
            fn request_object_status_for_viewer_init(request: &mut api::Activity) {
                request_object_init(request);
                if request.object.as_mut().unwrap().status_for_viewer.is_none() {
                    request.object.as_mut().unwrap().status_for_viewer = Some(Default::default());
                }
            }
            
            fn request_provider_init(request: &mut api::Activity) {
                if request.provider.is_none() {
                    request.provider = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "place-name" => {
                        request.place_name = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "provider.title" => {
                        request_provider_init(&mut request);
                        request.provider.as_mut().unwrap().title = Some(value.unwrap_or("").to_string());
                    },
                "title" => {
                        request_provider_init(&mut request);
                        request.title = Some(value.unwrap_or("").to_string());
                    },
                "url" => {
                        request_provider_init(&mut request);
                        request.url = Some(value.unwrap_or("").to_string());
                    },
                "geocode" => {
                        request_provider_init(&mut request);
                        request.geocode = Some(value.unwrap_or("").to_string());
                    },
                "object.resharers.total-items" => {
                        request_object_resharers_init(&mut request);
                        request.object.as_mut().unwrap().resharers.as_mut().unwrap().total_items = Some(arg_from_str(value.unwrap_or("-0"), err, "object.resharers.total-items", "integer"));
                    },
                "object.resharers.self-link" => {
                        request_object_resharers_init(&mut request);
                        request.object.as_mut().unwrap().resharers.as_mut().unwrap().self_link = Some(value.unwrap_or("").to_string());
                    },
                "object.original-content" => {
                        request_object_resharers_init(&mut request);
                        request.object.as_mut().unwrap().original_content = Some(value.unwrap_or("").to_string());
                    },
                "object.plusoners.total-items" => {
                        request_object_plusoners_init(&mut request);
                        request.object.as_mut().unwrap().plusoners.as_mut().unwrap().total_items = Some(arg_from_str(value.unwrap_or("-0"), err, "object.plusoners.total-items", "integer"));
                    },
                "object.plusoners.self-link" => {
                        request_object_plusoners_init(&mut request);
                        request.object.as_mut().unwrap().plusoners.as_mut().unwrap().self_link = Some(value.unwrap_or("").to_string());
                    },
                "object.actor.url" => {
                        request_object_actor_init(&mut request);
                        request.object.as_mut().unwrap().actor.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "object.actor.image.url" => {
                        request_object_actor_image_init(&mut request);
                        request.object.as_mut().unwrap().actor.as_mut().unwrap().image.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "object.actor.display-name" => {
                        request_object_actor_image_init(&mut request);
                        request.object.as_mut().unwrap().actor.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "object.actor.id" => {
                        request_object_actor_image_init(&mut request);
                        request.object.as_mut().unwrap().actor.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "object.content" => {
                        request_object_actor_init(&mut request);
                        request.object.as_mut().unwrap().content = Some(value.unwrap_or("").to_string());
                    },
                "object.url" => {
                        request_object_actor_init(&mut request);
                        request.object.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "object.status-for-viewer.can-plusone" => {
                        request_object_status_for_viewer_init(&mut request);
                        request.object.as_mut().unwrap().status_for_viewer.as_mut().unwrap().can_plusone = Some(arg_from_str(value.unwrap_or("false"), err, "object.status-for-viewer.can-plusone", "boolean"));
                    },
                "object.status-for-viewer.can-update" => {
                        request_object_status_for_viewer_init(&mut request);
                        request.object.as_mut().unwrap().status_for_viewer.as_mut().unwrap().can_update = Some(arg_from_str(value.unwrap_or("false"), err, "object.status-for-viewer.can-update", "boolean"));
                    },
                "object.status-for-viewer.is-plus-oned" => {
                        request_object_status_for_viewer_init(&mut request);
                        request.object.as_mut().unwrap().status_for_viewer.as_mut().unwrap().is_plus_oned = Some(arg_from_str(value.unwrap_or("false"), err, "object.status-for-viewer.is-plus-oned", "boolean"));
                    },
                "object.status-for-viewer.resharing-disabled" => {
                        request_object_status_for_viewer_init(&mut request);
                        request.object.as_mut().unwrap().status_for_viewer.as_mut().unwrap().resharing_disabled = Some(arg_from_str(value.unwrap_or("false"), err, "object.status-for-viewer.resharing-disabled", "boolean"));
                    },
                "object.status-for-viewer.can-comment" => {
                        request_object_status_for_viewer_init(&mut request);
                        request.object.as_mut().unwrap().status_for_viewer.as_mut().unwrap().can_comment = Some(arg_from_str(value.unwrap_or("false"), err, "object.status-for-viewer.can-comment", "boolean"));
                    },
                "object.replies.total-items" => {
                        request_object_replies_init(&mut request);
                        request.object.as_mut().unwrap().replies.as_mut().unwrap().total_items = Some(arg_from_str(value.unwrap_or("-0"), err, "object.replies.total-items", "integer"));
                    },
                "object.replies.self-link" => {
                        request_object_replies_init(&mut request);
                        request.object.as_mut().unwrap().replies.as_mut().unwrap().self_link = Some(value.unwrap_or("").to_string());
                    },
                "object.id" => {
                        request_object_replies_init(&mut request);
                        request.object.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "object.object-type" => {
                        request_object_replies_init(&mut request);
                        request.object.as_mut().unwrap().object_type = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_object_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "actor.url" => {
                        request_actor_init(&mut request);
                        request.actor.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "actor.image.url" => {
                        request_actor_image_init(&mut request);
                        request.actor.as_mut().unwrap().image.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "actor.display-name" => {
                        request_actor_image_init(&mut request);
                        request.actor.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "actor.id" => {
                        request_actor_image_init(&mut request);
                        request.actor.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "actor.name.given-name" => {
                        request_actor_name_init(&mut request);
                        request.actor.as_mut().unwrap().name.as_mut().unwrap().given_name = Some(value.unwrap_or("").to_string());
                    },
                "actor.name.family-name" => {
                        request_actor_name_init(&mut request);
                        request.actor.as_mut().unwrap().name.as_mut().unwrap().family_name = Some(value.unwrap_or("").to_string());
                    },
                "place-id" => {
                        request_actor_init(&mut request);
                        request.place_id = Some(value.unwrap_or("").to_string());
                    },
                "access.kind" => {
                        request_access_init(&mut request);
                        request.access.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "access.description" => {
                        request_access_init(&mut request);
                        request.access.as_mut().unwrap().description = Some(value.unwrap_or("").to_string());
                    },
                "access.domain-restricted" => {
                        request_access_init(&mut request);
                        request.access.as_mut().unwrap().domain_restricted = Some(arg_from_str(value.unwrap_or("false"), err, "access.domain-restricted", "boolean"));
                    },
                "etag" => {
                        request_access_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "radius" => {
                        request_access_init(&mut request);
                        request.radius = Some(value.unwrap_or("").to_string());
                    },
                "location.position.latitude" => {
                        request_location_position_init(&mut request);
                        request.location.as_mut().unwrap().position.as_mut().unwrap().latitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "location.position.latitude", "number"));
                    },
                "location.position.longitude" => {
                        request_location_position_init(&mut request);
                        request.location.as_mut().unwrap().position.as_mut().unwrap().longitude = Some(arg_from_str(value.unwrap_or("0.0"), err, "location.position.longitude", "number"));
                    },
                "location.kind" => {
                        request_location_position_init(&mut request);
                        request.location.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "location.display-name" => {
                        request_location_position_init(&mut request);
                        request.location.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "location.id" => {
                        request_location_position_init(&mut request);
                        request.location.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "location.address.formatted" => {
                        request_location_address_init(&mut request);
                        request.location.as_mut().unwrap().address.as_mut().unwrap().formatted = Some(value.unwrap_or("").to_string());
                    },
                "verb" => {
                        request_location_init(&mut request);
                        request.verb = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_location_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "crosspost-source" => {
                        request_location_init(&mut request);
                        request.crosspost_source = Some(value.unwrap_or("").to_string());
                    },
                "annotation" => {
                        request_location_init(&mut request);
                        request.annotation = Some(value.unwrap_or("").to_string());
                    },
                "address" => {
                        request_location_init(&mut request);
                        request.address = Some(value.unwrap_or("").to_string());
                    },
                "published" => {
                        request_location_init(&mut request);
                        request.published = Some(value.unwrap_or("").to_string());
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

    fn _audiences_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.audiences().list(&self.opt.arg_user_id);
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

    fn _circles_add_people(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.circles().add_people(&self.opt.arg_circle_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-id" => {
                    call = call.add_user_id(value.unwrap_or(""));
                },
                "email" => {
                    call = call.add_email(value.unwrap_or(""));
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

    fn _circles_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.circles().get(&self.opt.arg_circle_id);
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

    fn _circles_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Circle::default();
        let mut call = self.hub.circles().insert(&request, &self.opt.arg_user_id);
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
            fn request_people_init(request: &mut api::Circle) {
                if request.people.is_none() {
                    request.people = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "display-name" => {
                        request.display_name = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "people.total-items" => {
                        request_people_init(&mut request);
                        request.people.as_mut().unwrap().total_items = Some(arg_from_str(value.unwrap_or("-0"), err, "people.total-items", "integer"));
                    },
                "etag" => {
                        request_people_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_people_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_people_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
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

    fn _circles_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.circles().list(&self.opt.arg_user_id);
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

    fn _circles_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Circle::default();
        let mut call = self.hub.circles().patch(&request, &self.opt.arg_circle_id);
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
            fn request_people_init(request: &mut api::Circle) {
                if request.people.is_none() {
                    request.people = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "display-name" => {
                        request.display_name = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "people.total-items" => {
                        request_people_init(&mut request);
                        request.people.as_mut().unwrap().total_items = Some(arg_from_str(value.unwrap_or("-0"), err, "people.total-items", "integer"));
                    },
                "etag" => {
                        request_people_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_people_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_people_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
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

    fn _circles_remove(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.circles().remove(&self.opt.arg_circle_id);
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

    fn _circles_remove_people(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.circles().remove_people(&self.opt.arg_circle_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "user-id" => {
                    call = call.add_user_id(value.unwrap_or(""));
                },
                "email" => {
                    call = call.add_email(value.unwrap_or(""));
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

    fn _circles_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Circle::default();
        let mut call = self.hub.circles().update(&request, &self.opt.arg_circle_id);
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
            fn request_people_init(request: &mut api::Circle) {
                if request.people.is_none() {
                    request.people = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "display-name" => {
                        request.display_name = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "people.total-items" => {
                        request_people_init(&mut request);
                        request.people.as_mut().unwrap().total_items = Some(arg_from_str(value.unwrap_or("-0"), err, "people.total-items", "integer"));
                    },
                "etag" => {
                        request_people_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_people_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_people_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
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

    fn _comments_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Comment::default();
        let mut call = self.hub.comments().insert(&request, &self.opt.arg_activity_id);
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
            fn request_actor_image_init(request: &mut api::Comment) {
                request_actor_init(request);
                if request.actor.as_mut().unwrap().image.is_none() {
                    request.actor.as_mut().unwrap().image = Some(Default::default());
                }
            }
            
            fn request_actor_init(request: &mut api::Comment) {
                if request.actor.is_none() {
                    request.actor = Some(Default::default());
                }
            }
            
            fn request_object_init(request: &mut api::Comment) {
                if request.object.is_none() {
                    request.object = Some(Default::default());
                }
            }
            
            fn request_plusoners_init(request: &mut api::Comment) {
                if request.plusoners.is_none() {
                    request.plusoners = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "plusoners.total-items" => {
                        request_plusoners_init(&mut request);
                        request.plusoners.as_mut().unwrap().total_items = Some(arg_from_str(value.unwrap_or("-0"), err, "plusoners.total-items", "integer"));
                    },
                "object.content" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().content = Some(value.unwrap_or("").to_string());
                    },
                "object.original-content" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().original_content = Some(value.unwrap_or("").to_string());
                    },
                "object.object-type" => {
                        request_object_init(&mut request);
                        request.object.as_mut().unwrap().object_type = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request_object_init(&mut request);
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "actor.url" => {
                        request_actor_init(&mut request);
                        request.actor.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "actor.image.url" => {
                        request_actor_image_init(&mut request);
                        request.actor.as_mut().unwrap().image.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "actor.display-name" => {
                        request_actor_image_init(&mut request);
                        request.actor.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "actor.id" => {
                        request_actor_image_init(&mut request);
                        request.actor.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "verb" => {
                        request_actor_init(&mut request);
                        request.verb = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_actor_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "published" => {
                        request_actor_init(&mut request);
                        request.published = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_actor_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_actor_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
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

    fn _media_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut request = api::Media::default();
        let mut call = self.hub.media().insert(&request, &self.opt.arg_user_id, &self.opt.arg_collection);
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
            fn request_author_image_init(request: &mut api::Media) {
                request_author_init(request);
                if request.author.as_mut().unwrap().image.is_none() {
                    request.author.as_mut().unwrap().image = Some(Default::default());
                }
            }
            
            fn request_author_init(request: &mut api::Media) {
                if request.author.is_none() {
                    request.author = Some(Default::default());
                }
            }
            
            fn request_exif_init(request: &mut api::Media) {
                if request.exif.is_none() {
                    request.exif = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "display-name" => {
                        request.display_name = Some(value.unwrap_or("").to_string());
                    },
                "exif.time" => {
                        request_exif_init(&mut request);
                        request.exif.as_mut().unwrap().time = Some(value.unwrap_or("").to_string());
                    },
                "author.url" => {
                        request_author_init(&mut request);
                        request.author.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "author.image.url" => {
                        request_author_image_init(&mut request);
                        request.author.as_mut().unwrap().image.as_mut().unwrap().url = Some(value.unwrap_or("").to_string());
                    },
                "author.display-name" => {
                        request_author_image_init(&mut request);
                        request.author.as_mut().unwrap().display_name = Some(value.unwrap_or("").to_string());
                    },
                "author.id" => {
                        request_author_image_init(&mut request);
                        request.author.as_mut().unwrap().id = Some(value.unwrap_or("").to_string());
                    },
                "url" => {
                        request_author_init(&mut request);
                        request.url = Some(value.unwrap_or("").to_string());
                    },
                "media-url" => {
                        request_author_init(&mut request);
                        request.media_url = Some(value.unwrap_or("").to_string());
                    },
                "video-status" => {
                        request_author_init(&mut request);
                        request.video_status = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_author_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "height" => {
                        request_author_init(&mut request);
                        request.height = Some(arg_from_str(value.unwrap_or("-0"), err, "height", "integer"));
                    },
                "video-duration" => {
                        request_author_init(&mut request);
                        request.video_duration = Some(value.unwrap_or("").to_string());
                    },
                "size-bytes" => {
                        request_author_init(&mut request);
                        request.size_bytes = Some(value.unwrap_or("").to_string());
                    },
                "width" => {
                        request_author_init(&mut request);
                        request.width = Some(arg_from_str(value.unwrap_or("-0"), err, "width", "integer"));
                    },
                "summary" => {
                        request_author_init(&mut request);
                        request.summary = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_author_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "published" => {
                        request_author_init(&mut request);
                        request.published = Some(value.unwrap_or("").to_string());
                    },
                "media-created-time" => {
                        request_author_init(&mut request);
                        request.media_created_time = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_author_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(field_name.to_string())));
                }
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
                    serde::json::to_writer_pretty(&mut ostream, &output_schema).unwrap();
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

    fn _people_list_by_circle(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.people().list_by_circle(&self.opt.arg_circle_id);
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

        if self.opt.cmd_activities {
            if self.opt.cmd_get {
                call_result = self._activities_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._activities_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._activities_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_audiences {
            if self.opt.cmd_list {
                call_result = self._audiences_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_circles {
            if self.opt.cmd_add_people {
                call_result = self._circles_add_people(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._circles_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._circles_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._circles_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._circles_patch(dry_run, &mut err);
            } else if self.opt.cmd_remove {
                call_result = self._circles_remove(dry_run, &mut err);
            } else if self.opt.cmd_remove_people {
                call_result = self._circles_remove_people(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._circles_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_comments {
            if self.opt.cmd_get {
                call_result = self._comments_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._comments_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._comments_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_media {
            if self.opt.cmd_insert {
                call_result = self._media_insert(dry_run, &mut err);
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
            } else if self.opt.cmd_list_by_circle {
                call_result = self._people_list_by_circle(dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "plusdomains1-secret.json", 
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
                                          program_name: "plusdomains1",
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
            hub: api::PlusDomains::new(client, auth),
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