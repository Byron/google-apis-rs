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
extern crate google_gamesconfiguration1_configuration as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  gamesconfiguration1-configuration [options] achievement-configurations delete <achievement-id> [-p <v>...]
  gamesconfiguration1-configuration [options] achievement-configurations get <achievement-id> [-p <v>...] [-o <out>]
  gamesconfiguration1-configuration [options] achievement-configurations insert <application-id> -r <kv>... [-p <v>...] [-o <out>]
  gamesconfiguration1-configuration [options] achievement-configurations list <application-id> [-p <v>...] [-o <out>]
  gamesconfiguration1-configuration [options] achievement-configurations patch <achievement-id> -r <kv>... [-p <v>...] [-o <out>]
  gamesconfiguration1-configuration [options] achievement-configurations update <achievement-id> -r <kv>... [-p <v>...] [-o <out>]
  gamesconfiguration1-configuration [options] image-configurations upload <resource-id> <image-type> -u (simple|resumable) <file> <mime> [-p <v>...] [-o <out>]
  gamesconfiguration1-configuration [options] leaderboard-configurations delete <leaderboard-id> [-p <v>...]
  gamesconfiguration1-configuration [options] leaderboard-configurations get <leaderboard-id> [-p <v>...] [-o <out>]
  gamesconfiguration1-configuration [options] leaderboard-configurations insert <application-id> -r <kv>... [-p <v>...] [-o <out>]
  gamesconfiguration1-configuration [options] leaderboard-configurations list <application-id> [-p <v>...] [-o <out>]
  gamesconfiguration1-configuration [options] leaderboard-configurations patch <leaderboard-id> -r <kv>... [-p <v>...] [-o <out>]
  gamesconfiguration1-configuration [options] leaderboard-configurations update <leaderboard-id> -r <kv>... [-p <v>...] [-o <out>]
  gamesconfiguration1-configuration --help

All documentation details can be found at
http://byron.github.io/google-apis-rs/google_gamesconfiguration1_configuration_cli/index.html

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
    hub: api::GamesConfiguration<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _achievement_configurations_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.achievement_configurations().delete(&self.opt.arg_achievement_id);
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

    fn _achievement_configurations_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.achievement_configurations().get(&self.opt.arg_achievement_id);
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

    fn _achievement_configurations_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::AchievementConfiguration::default();
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
            fn request_draft_description_init(request: &mut api::AchievementConfiguration) {
                request_draft_init(request);
                if request.draft.as_mut().unwrap().description.is_none() {
                    request.draft.as_mut().unwrap().description = Some(Default::default());
                }
            }
            
            fn request_draft_init(request: &mut api::AchievementConfiguration) {
                if request.draft.is_none() {
                    request.draft = Some(Default::default());
                }
            }
            
            fn request_draft_name_init(request: &mut api::AchievementConfiguration) {
                request_draft_init(request);
                if request.draft.as_mut().unwrap().name.is_none() {
                    request.draft.as_mut().unwrap().name = Some(Default::default());
                }
            }
            
            fn request_published_description_init(request: &mut api::AchievementConfiguration) {
                request_published_init(request);
                if request.published.as_mut().unwrap().description.is_none() {
                    request.published.as_mut().unwrap().description = Some(Default::default());
                }
            }
            
            fn request_published_init(request: &mut api::AchievementConfiguration) {
                if request.published.is_none() {
                    request.published = Some(Default::default());
                }
            }
            
            fn request_published_name_init(request: &mut api::AchievementConfiguration) {
                request_published_init(request);
                if request.published.as_mut().unwrap().name.is_none() {
                    request.published.as_mut().unwrap().name = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "achievement-type" => {
                        request.achievement_type = Some(value.unwrap_or("").to_string());
                    },
                "steps-to-unlock" => {
                        request.steps_to_unlock = Some(arg_from_str(value.unwrap_or("-0"), err, "steps-to-unlock", "integer"));
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "initial-state" => {
                        request.initial_state = Some(value.unwrap_or("").to_string());
                    },
                "token" => {
                        request.token = Some(value.unwrap_or("").to_string());
                    },
                "draft.kind" => {
                        request_draft_init(&mut request);
                        request.draft.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "draft.description.kind" => {
                        request_draft_description_init(&mut request);
                        request.draft.as_mut().unwrap().description.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "draft.icon-url" => {
                        request_draft_description_init(&mut request);
                        request.draft.as_mut().unwrap().icon_url = Some(value.unwrap_or("").to_string());
                    },
                "draft.point-value" => {
                        request_draft_description_init(&mut request);
                        request.draft.as_mut().unwrap().point_value = Some(arg_from_str(value.unwrap_or("-0"), err, "draft.point-value", "integer"));
                    },
                "draft.sort-rank" => {
                        request_draft_description_init(&mut request);
                        request.draft.as_mut().unwrap().sort_rank = Some(arg_from_str(value.unwrap_or("-0"), err, "draft.sort-rank", "integer"));
                    },
                "draft.name.kind" => {
                        request_draft_name_init(&mut request);
                        request.draft.as_mut().unwrap().name.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "published.kind" => {
                        request_published_init(&mut request);
                        request.published.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "published.description.kind" => {
                        request_published_description_init(&mut request);
                        request.published.as_mut().unwrap().description.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "published.icon-url" => {
                        request_published_description_init(&mut request);
                        request.published.as_mut().unwrap().icon_url = Some(value.unwrap_or("").to_string());
                    },
                "published.point-value" => {
                        request_published_description_init(&mut request);
                        request.published.as_mut().unwrap().point_value = Some(arg_from_str(value.unwrap_or("-0"), err, "published.point-value", "integer"));
                    },
                "published.sort-rank" => {
                        request_published_description_init(&mut request);
                        request.published.as_mut().unwrap().sort_rank = Some(arg_from_str(value.unwrap_or("-0"), err, "published.sort-rank", "integer"));
                    },
                "published.name.kind" => {
                        request_published_name_init(&mut request);
                        request.published.as_mut().unwrap().name.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_published_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.achievement_configurations().insert(request, &self.opt.arg_application_id);
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

    fn _achievement_configurations_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.achievement_configurations().list(&self.opt.arg_application_id);
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

    fn _achievement_configurations_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::AchievementConfiguration::default();
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
            fn request_draft_description_init(request: &mut api::AchievementConfiguration) {
                request_draft_init(request);
                if request.draft.as_mut().unwrap().description.is_none() {
                    request.draft.as_mut().unwrap().description = Some(Default::default());
                }
            }
            
            fn request_draft_init(request: &mut api::AchievementConfiguration) {
                if request.draft.is_none() {
                    request.draft = Some(Default::default());
                }
            }
            
            fn request_draft_name_init(request: &mut api::AchievementConfiguration) {
                request_draft_init(request);
                if request.draft.as_mut().unwrap().name.is_none() {
                    request.draft.as_mut().unwrap().name = Some(Default::default());
                }
            }
            
            fn request_published_description_init(request: &mut api::AchievementConfiguration) {
                request_published_init(request);
                if request.published.as_mut().unwrap().description.is_none() {
                    request.published.as_mut().unwrap().description = Some(Default::default());
                }
            }
            
            fn request_published_init(request: &mut api::AchievementConfiguration) {
                if request.published.is_none() {
                    request.published = Some(Default::default());
                }
            }
            
            fn request_published_name_init(request: &mut api::AchievementConfiguration) {
                request_published_init(request);
                if request.published.as_mut().unwrap().name.is_none() {
                    request.published.as_mut().unwrap().name = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "achievement-type" => {
                        request.achievement_type = Some(value.unwrap_or("").to_string());
                    },
                "steps-to-unlock" => {
                        request.steps_to_unlock = Some(arg_from_str(value.unwrap_or("-0"), err, "steps-to-unlock", "integer"));
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "initial-state" => {
                        request.initial_state = Some(value.unwrap_or("").to_string());
                    },
                "token" => {
                        request.token = Some(value.unwrap_or("").to_string());
                    },
                "draft.kind" => {
                        request_draft_init(&mut request);
                        request.draft.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "draft.description.kind" => {
                        request_draft_description_init(&mut request);
                        request.draft.as_mut().unwrap().description.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "draft.icon-url" => {
                        request_draft_description_init(&mut request);
                        request.draft.as_mut().unwrap().icon_url = Some(value.unwrap_or("").to_string());
                    },
                "draft.point-value" => {
                        request_draft_description_init(&mut request);
                        request.draft.as_mut().unwrap().point_value = Some(arg_from_str(value.unwrap_or("-0"), err, "draft.point-value", "integer"));
                    },
                "draft.sort-rank" => {
                        request_draft_description_init(&mut request);
                        request.draft.as_mut().unwrap().sort_rank = Some(arg_from_str(value.unwrap_or("-0"), err, "draft.sort-rank", "integer"));
                    },
                "draft.name.kind" => {
                        request_draft_name_init(&mut request);
                        request.draft.as_mut().unwrap().name.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "published.kind" => {
                        request_published_init(&mut request);
                        request.published.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "published.description.kind" => {
                        request_published_description_init(&mut request);
                        request.published.as_mut().unwrap().description.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "published.icon-url" => {
                        request_published_description_init(&mut request);
                        request.published.as_mut().unwrap().icon_url = Some(value.unwrap_or("").to_string());
                    },
                "published.point-value" => {
                        request_published_description_init(&mut request);
                        request.published.as_mut().unwrap().point_value = Some(arg_from_str(value.unwrap_or("-0"), err, "published.point-value", "integer"));
                    },
                "published.sort-rank" => {
                        request_published_description_init(&mut request);
                        request.published.as_mut().unwrap().sort_rank = Some(arg_from_str(value.unwrap_or("-0"), err, "published.sort-rank", "integer"));
                    },
                "published.name.kind" => {
                        request_published_name_init(&mut request);
                        request.published.as_mut().unwrap().name.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_published_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.achievement_configurations().patch(request, &self.opt.arg_achievement_id);
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

    fn _achievement_configurations_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::AchievementConfiguration::default();
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
            fn request_draft_description_init(request: &mut api::AchievementConfiguration) {
                request_draft_init(request);
                if request.draft.as_mut().unwrap().description.is_none() {
                    request.draft.as_mut().unwrap().description = Some(Default::default());
                }
            }
            
            fn request_draft_init(request: &mut api::AchievementConfiguration) {
                if request.draft.is_none() {
                    request.draft = Some(Default::default());
                }
            }
            
            fn request_draft_name_init(request: &mut api::AchievementConfiguration) {
                request_draft_init(request);
                if request.draft.as_mut().unwrap().name.is_none() {
                    request.draft.as_mut().unwrap().name = Some(Default::default());
                }
            }
            
            fn request_published_description_init(request: &mut api::AchievementConfiguration) {
                request_published_init(request);
                if request.published.as_mut().unwrap().description.is_none() {
                    request.published.as_mut().unwrap().description = Some(Default::default());
                }
            }
            
            fn request_published_init(request: &mut api::AchievementConfiguration) {
                if request.published.is_none() {
                    request.published = Some(Default::default());
                }
            }
            
            fn request_published_name_init(request: &mut api::AchievementConfiguration) {
                request_published_init(request);
                if request.published.as_mut().unwrap().name.is_none() {
                    request.published.as_mut().unwrap().name = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "achievement-type" => {
                        request.achievement_type = Some(value.unwrap_or("").to_string());
                    },
                "steps-to-unlock" => {
                        request.steps_to_unlock = Some(arg_from_str(value.unwrap_or("-0"), err, "steps-to-unlock", "integer"));
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "initial-state" => {
                        request.initial_state = Some(value.unwrap_or("").to_string());
                    },
                "token" => {
                        request.token = Some(value.unwrap_or("").to_string());
                    },
                "draft.kind" => {
                        request_draft_init(&mut request);
                        request.draft.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "draft.description.kind" => {
                        request_draft_description_init(&mut request);
                        request.draft.as_mut().unwrap().description.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "draft.icon-url" => {
                        request_draft_description_init(&mut request);
                        request.draft.as_mut().unwrap().icon_url = Some(value.unwrap_or("").to_string());
                    },
                "draft.point-value" => {
                        request_draft_description_init(&mut request);
                        request.draft.as_mut().unwrap().point_value = Some(arg_from_str(value.unwrap_or("-0"), err, "draft.point-value", "integer"));
                    },
                "draft.sort-rank" => {
                        request_draft_description_init(&mut request);
                        request.draft.as_mut().unwrap().sort_rank = Some(arg_from_str(value.unwrap_or("-0"), err, "draft.sort-rank", "integer"));
                    },
                "draft.name.kind" => {
                        request_draft_name_init(&mut request);
                        request.draft.as_mut().unwrap().name.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "published.kind" => {
                        request_published_init(&mut request);
                        request.published.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "published.description.kind" => {
                        request_published_description_init(&mut request);
                        request.published.as_mut().unwrap().description.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "published.icon-url" => {
                        request_published_description_init(&mut request);
                        request.published.as_mut().unwrap().icon_url = Some(value.unwrap_or("").to_string());
                    },
                "published.point-value" => {
                        request_published_description_init(&mut request);
                        request.published.as_mut().unwrap().point_value = Some(arg_from_str(value.unwrap_or("-0"), err, "published.point-value", "integer"));
                    },
                "published.sort-rank" => {
                        request_published_description_init(&mut request);
                        request.published.as_mut().unwrap().sort_rank = Some(arg_from_str(value.unwrap_or("-0"), err, "published.sort-rank", "integer"));
                    },
                "published.name.kind" => {
                        request_published_name_init(&mut request);
                        request.published.as_mut().unwrap().name.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_published_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.achievement_configurations().update(request, &self.opt.arg_achievement_id);
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

    fn _image_configurations_upload(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.image_configurations().upload(&self.opt.arg_resource_id, &self.opt.arg_image_type);
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
            if self.opt.flag_scope.len() > 0 {
                call = call.add_scope(&self.opt.flag_scope);
            }
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

    fn _leaderboard_configurations_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.leaderboard_configurations().delete(&self.opt.arg_leaderboard_id);
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

    fn _leaderboard_configurations_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.leaderboard_configurations().get(&self.opt.arg_leaderboard_id);
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

    fn _leaderboard_configurations_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::LeaderboardConfiguration::default();
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
            fn request_draft_init(request: &mut api::LeaderboardConfiguration) {
                if request.draft.is_none() {
                    request.draft = Some(Default::default());
                }
            }
            
            fn request_draft_name_init(request: &mut api::LeaderboardConfiguration) {
                request_draft_init(request);
                if request.draft.as_mut().unwrap().name.is_none() {
                    request.draft.as_mut().unwrap().name = Some(Default::default());
                }
            }
            
            fn request_draft_score_format_init(request: &mut api::LeaderboardConfiguration) {
                request_draft_init(request);
                if request.draft.as_mut().unwrap().score_format.is_none() {
                    request.draft.as_mut().unwrap().score_format = Some(Default::default());
                }
            }
            
            fn request_draft_score_format_suffix_few_init(request: &mut api::LeaderboardConfiguration) {
                request_draft_score_format_suffix_init(request);
                if request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().few.is_none() {
                    request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().few = Some(Default::default());
                }
            }
            
            fn request_draft_score_format_suffix_init(request: &mut api::LeaderboardConfiguration) {
                request_draft_score_format_init(request);
                if request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.is_none() {
                    request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix = Some(Default::default());
                }
            }
            
            fn request_draft_score_format_suffix_many_init(request: &mut api::LeaderboardConfiguration) {
                request_draft_score_format_suffix_init(request);
                if request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().many.is_none() {
                    request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().many = Some(Default::default());
                }
            }
            
            fn request_draft_score_format_suffix_one_init(request: &mut api::LeaderboardConfiguration) {
                request_draft_score_format_suffix_init(request);
                if request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().one.is_none() {
                    request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().one = Some(Default::default());
                }
            }
            
            fn request_draft_score_format_suffix_other_init(request: &mut api::LeaderboardConfiguration) {
                request_draft_score_format_suffix_init(request);
                if request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().other.is_none() {
                    request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().other = Some(Default::default());
                }
            }
            
            fn request_draft_score_format_suffix_two_init(request: &mut api::LeaderboardConfiguration) {
                request_draft_score_format_suffix_init(request);
                if request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().two.is_none() {
                    request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().two = Some(Default::default());
                }
            }
            
            fn request_draft_score_format_suffix_zero_init(request: &mut api::LeaderboardConfiguration) {
                request_draft_score_format_suffix_init(request);
                if request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().zero.is_none() {
                    request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().zero = Some(Default::default());
                }
            }
            
            fn request_published_init(request: &mut api::LeaderboardConfiguration) {
                if request.published.is_none() {
                    request.published = Some(Default::default());
                }
            }
            
            fn request_published_name_init(request: &mut api::LeaderboardConfiguration) {
                request_published_init(request);
                if request.published.as_mut().unwrap().name.is_none() {
                    request.published.as_mut().unwrap().name = Some(Default::default());
                }
            }
            
            fn request_published_score_format_init(request: &mut api::LeaderboardConfiguration) {
                request_published_init(request);
                if request.published.as_mut().unwrap().score_format.is_none() {
                    request.published.as_mut().unwrap().score_format = Some(Default::default());
                }
            }
            
            fn request_published_score_format_suffix_few_init(request: &mut api::LeaderboardConfiguration) {
                request_published_score_format_suffix_init(request);
                if request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().few.is_none() {
                    request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().few = Some(Default::default());
                }
            }
            
            fn request_published_score_format_suffix_init(request: &mut api::LeaderboardConfiguration) {
                request_published_score_format_init(request);
                if request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.is_none() {
                    request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix = Some(Default::default());
                }
            }
            
            fn request_published_score_format_suffix_many_init(request: &mut api::LeaderboardConfiguration) {
                request_published_score_format_suffix_init(request);
                if request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().many.is_none() {
                    request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().many = Some(Default::default());
                }
            }
            
            fn request_published_score_format_suffix_one_init(request: &mut api::LeaderboardConfiguration) {
                request_published_score_format_suffix_init(request);
                if request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().one.is_none() {
                    request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().one = Some(Default::default());
                }
            }
            
            fn request_published_score_format_suffix_other_init(request: &mut api::LeaderboardConfiguration) {
                request_published_score_format_suffix_init(request);
                if request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().other.is_none() {
                    request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().other = Some(Default::default());
                }
            }
            
            fn request_published_score_format_suffix_two_init(request: &mut api::LeaderboardConfiguration) {
                request_published_score_format_suffix_init(request);
                if request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().two.is_none() {
                    request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().two = Some(Default::default());
                }
            }
            
            fn request_published_score_format_suffix_zero_init(request: &mut api::LeaderboardConfiguration) {
                request_published_score_format_suffix_init(request);
                if request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().zero.is_none() {
                    request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().zero = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "score-order" => {
                        request.score_order = Some(value.unwrap_or("").to_string());
                    },
                "score-min" => {
                        request.score_min = Some(value.unwrap_or("").to_string());
                    },
                "token" => {
                        request.token = Some(value.unwrap_or("").to_string());
                    },
                "score-max" => {
                        request.score_max = Some(value.unwrap_or("").to_string());
                    },
                "published.score-format.currency-code" => {
                        request_published_score_format_init(&mut request);
                        request.published.as_mut().unwrap().score_format.as_mut().unwrap().currency_code = Some(value.unwrap_or("").to_string());
                    },
                "published.score-format.suffix.many.kind" => {
                        request_published_score_format_suffix_many_init(&mut request);
                        request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().many.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "published.score-format.suffix.two.kind" => {
                        request_published_score_format_suffix_two_init(&mut request);
                        request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().two.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "published.score-format.suffix.one.kind" => {
                        request_published_score_format_suffix_one_init(&mut request);
                        request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().one.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "published.score-format.suffix.few.kind" => {
                        request_published_score_format_suffix_few_init(&mut request);
                        request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().few.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "published.score-format.suffix.zero.kind" => {
                        request_published_score_format_suffix_zero_init(&mut request);
                        request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().zero.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "published.score-format.suffix.other.kind" => {
                        request_published_score_format_suffix_other_init(&mut request);
                        request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().other.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "published.score-format.number-format-type" => {
                        request_published_score_format_suffix_init(&mut request);
                        request.published.as_mut().unwrap().score_format.as_mut().unwrap().number_format_type = Some(value.unwrap_or("").to_string());
                    },
                "published.score-format.num-decimal-places" => {
                        request_published_score_format_suffix_init(&mut request);
                        request.published.as_mut().unwrap().score_format.as_mut().unwrap().num_decimal_places = Some(arg_from_str(value.unwrap_or("-0"), err, "published.score-format.num-decimal-places", "integer"));
                    },
                "published.icon-url" => {
                        request_published_score_format_init(&mut request);
                        request.published.as_mut().unwrap().icon_url = Some(value.unwrap_or("").to_string());
                    },
                "published.kind" => {
                        request_published_score_format_init(&mut request);
                        request.published.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "published.name.kind" => {
                        request_published_name_init(&mut request);
                        request.published.as_mut().unwrap().name.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "published.sort-rank" => {
                        request_published_name_init(&mut request);
                        request.published.as_mut().unwrap().sort_rank = Some(arg_from_str(value.unwrap_or("-0"), err, "published.sort-rank", "integer"));
                    },
                "draft.score-format.currency-code" => {
                        request_draft_score_format_init(&mut request);
                        request.draft.as_mut().unwrap().score_format.as_mut().unwrap().currency_code = Some(value.unwrap_or("").to_string());
                    },
                "draft.score-format.suffix.many.kind" => {
                        request_draft_score_format_suffix_many_init(&mut request);
                        request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().many.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "draft.score-format.suffix.two.kind" => {
                        request_draft_score_format_suffix_two_init(&mut request);
                        request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().two.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "draft.score-format.suffix.one.kind" => {
                        request_draft_score_format_suffix_one_init(&mut request);
                        request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().one.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "draft.score-format.suffix.few.kind" => {
                        request_draft_score_format_suffix_few_init(&mut request);
                        request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().few.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "draft.score-format.suffix.zero.kind" => {
                        request_draft_score_format_suffix_zero_init(&mut request);
                        request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().zero.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "draft.score-format.suffix.other.kind" => {
                        request_draft_score_format_suffix_other_init(&mut request);
                        request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().other.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "draft.score-format.number-format-type" => {
                        request_draft_score_format_suffix_init(&mut request);
                        request.draft.as_mut().unwrap().score_format.as_mut().unwrap().number_format_type = Some(value.unwrap_or("").to_string());
                    },
                "draft.score-format.num-decimal-places" => {
                        request_draft_score_format_suffix_init(&mut request);
                        request.draft.as_mut().unwrap().score_format.as_mut().unwrap().num_decimal_places = Some(arg_from_str(value.unwrap_or("-0"), err, "draft.score-format.num-decimal-places", "integer"));
                    },
                "draft.icon-url" => {
                        request_draft_score_format_init(&mut request);
                        request.draft.as_mut().unwrap().icon_url = Some(value.unwrap_or("").to_string());
                    },
                "draft.kind" => {
                        request_draft_score_format_init(&mut request);
                        request.draft.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "draft.name.kind" => {
                        request_draft_name_init(&mut request);
                        request.draft.as_mut().unwrap().name.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "draft.sort-rank" => {
                        request_draft_name_init(&mut request);
                        request.draft.as_mut().unwrap().sort_rank = Some(arg_from_str(value.unwrap_or("-0"), err, "draft.sort-rank", "integer"));
                    },
                "id" => {
                        request_draft_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.leaderboard_configurations().insert(request, &self.opt.arg_application_id);
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

    fn _leaderboard_configurations_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.leaderboard_configurations().list(&self.opt.arg_application_id);
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

    fn _leaderboard_configurations_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::LeaderboardConfiguration::default();
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
            fn request_draft_init(request: &mut api::LeaderboardConfiguration) {
                if request.draft.is_none() {
                    request.draft = Some(Default::default());
                }
            }
            
            fn request_draft_name_init(request: &mut api::LeaderboardConfiguration) {
                request_draft_init(request);
                if request.draft.as_mut().unwrap().name.is_none() {
                    request.draft.as_mut().unwrap().name = Some(Default::default());
                }
            }
            
            fn request_draft_score_format_init(request: &mut api::LeaderboardConfiguration) {
                request_draft_init(request);
                if request.draft.as_mut().unwrap().score_format.is_none() {
                    request.draft.as_mut().unwrap().score_format = Some(Default::default());
                }
            }
            
            fn request_draft_score_format_suffix_few_init(request: &mut api::LeaderboardConfiguration) {
                request_draft_score_format_suffix_init(request);
                if request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().few.is_none() {
                    request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().few = Some(Default::default());
                }
            }
            
            fn request_draft_score_format_suffix_init(request: &mut api::LeaderboardConfiguration) {
                request_draft_score_format_init(request);
                if request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.is_none() {
                    request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix = Some(Default::default());
                }
            }
            
            fn request_draft_score_format_suffix_many_init(request: &mut api::LeaderboardConfiguration) {
                request_draft_score_format_suffix_init(request);
                if request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().many.is_none() {
                    request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().many = Some(Default::default());
                }
            }
            
            fn request_draft_score_format_suffix_one_init(request: &mut api::LeaderboardConfiguration) {
                request_draft_score_format_suffix_init(request);
                if request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().one.is_none() {
                    request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().one = Some(Default::default());
                }
            }
            
            fn request_draft_score_format_suffix_other_init(request: &mut api::LeaderboardConfiguration) {
                request_draft_score_format_suffix_init(request);
                if request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().other.is_none() {
                    request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().other = Some(Default::default());
                }
            }
            
            fn request_draft_score_format_suffix_two_init(request: &mut api::LeaderboardConfiguration) {
                request_draft_score_format_suffix_init(request);
                if request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().two.is_none() {
                    request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().two = Some(Default::default());
                }
            }
            
            fn request_draft_score_format_suffix_zero_init(request: &mut api::LeaderboardConfiguration) {
                request_draft_score_format_suffix_init(request);
                if request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().zero.is_none() {
                    request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().zero = Some(Default::default());
                }
            }
            
            fn request_published_init(request: &mut api::LeaderboardConfiguration) {
                if request.published.is_none() {
                    request.published = Some(Default::default());
                }
            }
            
            fn request_published_name_init(request: &mut api::LeaderboardConfiguration) {
                request_published_init(request);
                if request.published.as_mut().unwrap().name.is_none() {
                    request.published.as_mut().unwrap().name = Some(Default::default());
                }
            }
            
            fn request_published_score_format_init(request: &mut api::LeaderboardConfiguration) {
                request_published_init(request);
                if request.published.as_mut().unwrap().score_format.is_none() {
                    request.published.as_mut().unwrap().score_format = Some(Default::default());
                }
            }
            
            fn request_published_score_format_suffix_few_init(request: &mut api::LeaderboardConfiguration) {
                request_published_score_format_suffix_init(request);
                if request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().few.is_none() {
                    request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().few = Some(Default::default());
                }
            }
            
            fn request_published_score_format_suffix_init(request: &mut api::LeaderboardConfiguration) {
                request_published_score_format_init(request);
                if request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.is_none() {
                    request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix = Some(Default::default());
                }
            }
            
            fn request_published_score_format_suffix_many_init(request: &mut api::LeaderboardConfiguration) {
                request_published_score_format_suffix_init(request);
                if request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().many.is_none() {
                    request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().many = Some(Default::default());
                }
            }
            
            fn request_published_score_format_suffix_one_init(request: &mut api::LeaderboardConfiguration) {
                request_published_score_format_suffix_init(request);
                if request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().one.is_none() {
                    request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().one = Some(Default::default());
                }
            }
            
            fn request_published_score_format_suffix_other_init(request: &mut api::LeaderboardConfiguration) {
                request_published_score_format_suffix_init(request);
                if request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().other.is_none() {
                    request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().other = Some(Default::default());
                }
            }
            
            fn request_published_score_format_suffix_two_init(request: &mut api::LeaderboardConfiguration) {
                request_published_score_format_suffix_init(request);
                if request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().two.is_none() {
                    request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().two = Some(Default::default());
                }
            }
            
            fn request_published_score_format_suffix_zero_init(request: &mut api::LeaderboardConfiguration) {
                request_published_score_format_suffix_init(request);
                if request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().zero.is_none() {
                    request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().zero = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "score-order" => {
                        request.score_order = Some(value.unwrap_or("").to_string());
                    },
                "score-min" => {
                        request.score_min = Some(value.unwrap_or("").to_string());
                    },
                "token" => {
                        request.token = Some(value.unwrap_or("").to_string());
                    },
                "score-max" => {
                        request.score_max = Some(value.unwrap_or("").to_string());
                    },
                "published.score-format.currency-code" => {
                        request_published_score_format_init(&mut request);
                        request.published.as_mut().unwrap().score_format.as_mut().unwrap().currency_code = Some(value.unwrap_or("").to_string());
                    },
                "published.score-format.suffix.many.kind" => {
                        request_published_score_format_suffix_many_init(&mut request);
                        request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().many.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "published.score-format.suffix.two.kind" => {
                        request_published_score_format_suffix_two_init(&mut request);
                        request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().two.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "published.score-format.suffix.one.kind" => {
                        request_published_score_format_suffix_one_init(&mut request);
                        request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().one.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "published.score-format.suffix.few.kind" => {
                        request_published_score_format_suffix_few_init(&mut request);
                        request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().few.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "published.score-format.suffix.zero.kind" => {
                        request_published_score_format_suffix_zero_init(&mut request);
                        request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().zero.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "published.score-format.suffix.other.kind" => {
                        request_published_score_format_suffix_other_init(&mut request);
                        request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().other.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "published.score-format.number-format-type" => {
                        request_published_score_format_suffix_init(&mut request);
                        request.published.as_mut().unwrap().score_format.as_mut().unwrap().number_format_type = Some(value.unwrap_or("").to_string());
                    },
                "published.score-format.num-decimal-places" => {
                        request_published_score_format_suffix_init(&mut request);
                        request.published.as_mut().unwrap().score_format.as_mut().unwrap().num_decimal_places = Some(arg_from_str(value.unwrap_or("-0"), err, "published.score-format.num-decimal-places", "integer"));
                    },
                "published.icon-url" => {
                        request_published_score_format_init(&mut request);
                        request.published.as_mut().unwrap().icon_url = Some(value.unwrap_or("").to_string());
                    },
                "published.kind" => {
                        request_published_score_format_init(&mut request);
                        request.published.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "published.name.kind" => {
                        request_published_name_init(&mut request);
                        request.published.as_mut().unwrap().name.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "published.sort-rank" => {
                        request_published_name_init(&mut request);
                        request.published.as_mut().unwrap().sort_rank = Some(arg_from_str(value.unwrap_or("-0"), err, "published.sort-rank", "integer"));
                    },
                "draft.score-format.currency-code" => {
                        request_draft_score_format_init(&mut request);
                        request.draft.as_mut().unwrap().score_format.as_mut().unwrap().currency_code = Some(value.unwrap_or("").to_string());
                    },
                "draft.score-format.suffix.many.kind" => {
                        request_draft_score_format_suffix_many_init(&mut request);
                        request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().many.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "draft.score-format.suffix.two.kind" => {
                        request_draft_score_format_suffix_two_init(&mut request);
                        request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().two.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "draft.score-format.suffix.one.kind" => {
                        request_draft_score_format_suffix_one_init(&mut request);
                        request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().one.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "draft.score-format.suffix.few.kind" => {
                        request_draft_score_format_suffix_few_init(&mut request);
                        request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().few.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "draft.score-format.suffix.zero.kind" => {
                        request_draft_score_format_suffix_zero_init(&mut request);
                        request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().zero.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "draft.score-format.suffix.other.kind" => {
                        request_draft_score_format_suffix_other_init(&mut request);
                        request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().other.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "draft.score-format.number-format-type" => {
                        request_draft_score_format_suffix_init(&mut request);
                        request.draft.as_mut().unwrap().score_format.as_mut().unwrap().number_format_type = Some(value.unwrap_or("").to_string());
                    },
                "draft.score-format.num-decimal-places" => {
                        request_draft_score_format_suffix_init(&mut request);
                        request.draft.as_mut().unwrap().score_format.as_mut().unwrap().num_decimal_places = Some(arg_from_str(value.unwrap_or("-0"), err, "draft.score-format.num-decimal-places", "integer"));
                    },
                "draft.icon-url" => {
                        request_draft_score_format_init(&mut request);
                        request.draft.as_mut().unwrap().icon_url = Some(value.unwrap_or("").to_string());
                    },
                "draft.kind" => {
                        request_draft_score_format_init(&mut request);
                        request.draft.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "draft.name.kind" => {
                        request_draft_name_init(&mut request);
                        request.draft.as_mut().unwrap().name.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "draft.sort-rank" => {
                        request_draft_name_init(&mut request);
                        request.draft.as_mut().unwrap().sort_rank = Some(arg_from_str(value.unwrap_or("-0"), err, "draft.sort-rank", "integer"));
                    },
                "id" => {
                        request_draft_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.leaderboard_configurations().patch(request, &self.opt.arg_leaderboard_id);
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

    fn _leaderboard_configurations_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::LeaderboardConfiguration::default();
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
            fn request_draft_init(request: &mut api::LeaderboardConfiguration) {
                if request.draft.is_none() {
                    request.draft = Some(Default::default());
                }
            }
            
            fn request_draft_name_init(request: &mut api::LeaderboardConfiguration) {
                request_draft_init(request);
                if request.draft.as_mut().unwrap().name.is_none() {
                    request.draft.as_mut().unwrap().name = Some(Default::default());
                }
            }
            
            fn request_draft_score_format_init(request: &mut api::LeaderboardConfiguration) {
                request_draft_init(request);
                if request.draft.as_mut().unwrap().score_format.is_none() {
                    request.draft.as_mut().unwrap().score_format = Some(Default::default());
                }
            }
            
            fn request_draft_score_format_suffix_few_init(request: &mut api::LeaderboardConfiguration) {
                request_draft_score_format_suffix_init(request);
                if request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().few.is_none() {
                    request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().few = Some(Default::default());
                }
            }
            
            fn request_draft_score_format_suffix_init(request: &mut api::LeaderboardConfiguration) {
                request_draft_score_format_init(request);
                if request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.is_none() {
                    request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix = Some(Default::default());
                }
            }
            
            fn request_draft_score_format_suffix_many_init(request: &mut api::LeaderboardConfiguration) {
                request_draft_score_format_suffix_init(request);
                if request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().many.is_none() {
                    request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().many = Some(Default::default());
                }
            }
            
            fn request_draft_score_format_suffix_one_init(request: &mut api::LeaderboardConfiguration) {
                request_draft_score_format_suffix_init(request);
                if request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().one.is_none() {
                    request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().one = Some(Default::default());
                }
            }
            
            fn request_draft_score_format_suffix_other_init(request: &mut api::LeaderboardConfiguration) {
                request_draft_score_format_suffix_init(request);
                if request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().other.is_none() {
                    request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().other = Some(Default::default());
                }
            }
            
            fn request_draft_score_format_suffix_two_init(request: &mut api::LeaderboardConfiguration) {
                request_draft_score_format_suffix_init(request);
                if request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().two.is_none() {
                    request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().two = Some(Default::default());
                }
            }
            
            fn request_draft_score_format_suffix_zero_init(request: &mut api::LeaderboardConfiguration) {
                request_draft_score_format_suffix_init(request);
                if request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().zero.is_none() {
                    request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().zero = Some(Default::default());
                }
            }
            
            fn request_published_init(request: &mut api::LeaderboardConfiguration) {
                if request.published.is_none() {
                    request.published = Some(Default::default());
                }
            }
            
            fn request_published_name_init(request: &mut api::LeaderboardConfiguration) {
                request_published_init(request);
                if request.published.as_mut().unwrap().name.is_none() {
                    request.published.as_mut().unwrap().name = Some(Default::default());
                }
            }
            
            fn request_published_score_format_init(request: &mut api::LeaderboardConfiguration) {
                request_published_init(request);
                if request.published.as_mut().unwrap().score_format.is_none() {
                    request.published.as_mut().unwrap().score_format = Some(Default::default());
                }
            }
            
            fn request_published_score_format_suffix_few_init(request: &mut api::LeaderboardConfiguration) {
                request_published_score_format_suffix_init(request);
                if request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().few.is_none() {
                    request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().few = Some(Default::default());
                }
            }
            
            fn request_published_score_format_suffix_init(request: &mut api::LeaderboardConfiguration) {
                request_published_score_format_init(request);
                if request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.is_none() {
                    request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix = Some(Default::default());
                }
            }
            
            fn request_published_score_format_suffix_many_init(request: &mut api::LeaderboardConfiguration) {
                request_published_score_format_suffix_init(request);
                if request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().many.is_none() {
                    request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().many = Some(Default::default());
                }
            }
            
            fn request_published_score_format_suffix_one_init(request: &mut api::LeaderboardConfiguration) {
                request_published_score_format_suffix_init(request);
                if request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().one.is_none() {
                    request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().one = Some(Default::default());
                }
            }
            
            fn request_published_score_format_suffix_other_init(request: &mut api::LeaderboardConfiguration) {
                request_published_score_format_suffix_init(request);
                if request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().other.is_none() {
                    request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().other = Some(Default::default());
                }
            }
            
            fn request_published_score_format_suffix_two_init(request: &mut api::LeaderboardConfiguration) {
                request_published_score_format_suffix_init(request);
                if request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().two.is_none() {
                    request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().two = Some(Default::default());
                }
            }
            
            fn request_published_score_format_suffix_zero_init(request: &mut api::LeaderboardConfiguration) {
                request_published_score_format_suffix_init(request);
                if request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().zero.is_none() {
                    request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().zero = Some(Default::default());
                }
            }
            
            match &temp_cursor.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "score-order" => {
                        request.score_order = Some(value.unwrap_or("").to_string());
                    },
                "score-min" => {
                        request.score_min = Some(value.unwrap_or("").to_string());
                    },
                "token" => {
                        request.token = Some(value.unwrap_or("").to_string());
                    },
                "score-max" => {
                        request.score_max = Some(value.unwrap_or("").to_string());
                    },
                "published.score-format.currency-code" => {
                        request_published_score_format_init(&mut request);
                        request.published.as_mut().unwrap().score_format.as_mut().unwrap().currency_code = Some(value.unwrap_or("").to_string());
                    },
                "published.score-format.suffix.many.kind" => {
                        request_published_score_format_suffix_many_init(&mut request);
                        request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().many.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "published.score-format.suffix.two.kind" => {
                        request_published_score_format_suffix_two_init(&mut request);
                        request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().two.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "published.score-format.suffix.one.kind" => {
                        request_published_score_format_suffix_one_init(&mut request);
                        request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().one.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "published.score-format.suffix.few.kind" => {
                        request_published_score_format_suffix_few_init(&mut request);
                        request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().few.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "published.score-format.suffix.zero.kind" => {
                        request_published_score_format_suffix_zero_init(&mut request);
                        request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().zero.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "published.score-format.suffix.other.kind" => {
                        request_published_score_format_suffix_other_init(&mut request);
                        request.published.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().other.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "published.score-format.number-format-type" => {
                        request_published_score_format_suffix_init(&mut request);
                        request.published.as_mut().unwrap().score_format.as_mut().unwrap().number_format_type = Some(value.unwrap_or("").to_string());
                    },
                "published.score-format.num-decimal-places" => {
                        request_published_score_format_suffix_init(&mut request);
                        request.published.as_mut().unwrap().score_format.as_mut().unwrap().num_decimal_places = Some(arg_from_str(value.unwrap_or("-0"), err, "published.score-format.num-decimal-places", "integer"));
                    },
                "published.icon-url" => {
                        request_published_score_format_init(&mut request);
                        request.published.as_mut().unwrap().icon_url = Some(value.unwrap_or("").to_string());
                    },
                "published.kind" => {
                        request_published_score_format_init(&mut request);
                        request.published.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "published.name.kind" => {
                        request_published_name_init(&mut request);
                        request.published.as_mut().unwrap().name.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "published.sort-rank" => {
                        request_published_name_init(&mut request);
                        request.published.as_mut().unwrap().sort_rank = Some(arg_from_str(value.unwrap_or("-0"), err, "published.sort-rank", "integer"));
                    },
                "draft.score-format.currency-code" => {
                        request_draft_score_format_init(&mut request);
                        request.draft.as_mut().unwrap().score_format.as_mut().unwrap().currency_code = Some(value.unwrap_or("").to_string());
                    },
                "draft.score-format.suffix.many.kind" => {
                        request_draft_score_format_suffix_many_init(&mut request);
                        request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().many.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "draft.score-format.suffix.two.kind" => {
                        request_draft_score_format_suffix_two_init(&mut request);
                        request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().two.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "draft.score-format.suffix.one.kind" => {
                        request_draft_score_format_suffix_one_init(&mut request);
                        request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().one.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "draft.score-format.suffix.few.kind" => {
                        request_draft_score_format_suffix_few_init(&mut request);
                        request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().few.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "draft.score-format.suffix.zero.kind" => {
                        request_draft_score_format_suffix_zero_init(&mut request);
                        request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().zero.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "draft.score-format.suffix.other.kind" => {
                        request_draft_score_format_suffix_other_init(&mut request);
                        request.draft.as_mut().unwrap().score_format.as_mut().unwrap().suffix.as_mut().unwrap().other.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "draft.score-format.number-format-type" => {
                        request_draft_score_format_suffix_init(&mut request);
                        request.draft.as_mut().unwrap().score_format.as_mut().unwrap().number_format_type = Some(value.unwrap_or("").to_string());
                    },
                "draft.score-format.num-decimal-places" => {
                        request_draft_score_format_suffix_init(&mut request);
                        request.draft.as_mut().unwrap().score_format.as_mut().unwrap().num_decimal_places = Some(arg_from_str(value.unwrap_or("-0"), err, "draft.score-format.num-decimal-places", "integer"));
                    },
                "draft.icon-url" => {
                        request_draft_score_format_init(&mut request);
                        request.draft.as_mut().unwrap().icon_url = Some(value.unwrap_or("").to_string());
                    },
                "draft.kind" => {
                        request_draft_score_format_init(&mut request);
                        request.draft.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "draft.name.kind" => {
                        request_draft_name_init(&mut request);
                        request.draft.as_mut().unwrap().name.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "draft.sort-rank" => {
                        request_draft_name_init(&mut request);
                        request.draft.as_mut().unwrap().sort_rank = Some(arg_from_str(value.unwrap_or("-0"), err, "draft.sort-rank", "integer"));
                    },
                "id" => {
                        request_draft_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.leaderboard_configurations().update(request, &self.opt.arg_leaderboard_id);
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

    fn _doit(&self, dry_run: bool) -> (Option<api::Error>, Option<InvalidOptionsError>) {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Option<api::Error>;
        let mut err_opt: Option<InvalidOptionsError> = None;

        if self.opt.cmd_achievement_configurations {
            if self.opt.cmd_delete {
                call_result = self._achievement_configurations_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._achievement_configurations_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._achievement_configurations_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._achievement_configurations_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._achievement_configurations_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._achievement_configurations_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_image_configurations {
            if self.opt.cmd_upload {
                call_result = self._image_configurations_upload(dry_run, &mut err);
            } else {
                unreachable!();
            }
        }
 else if self.opt.cmd_leaderboard_configurations {
            if self.opt.cmd_delete {
                call_result = self._leaderboard_configurations_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._leaderboard_configurations_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._leaderboard_configurations_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._leaderboard_configurations_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._leaderboard_configurations_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._leaderboard_configurations_update(dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "gamesconfiguration1-configuration-secret.json", 
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
                                          program_name: "gamesconfiguration1-configuration",
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
            hub: api::GamesConfiguration::new(client, auth),
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