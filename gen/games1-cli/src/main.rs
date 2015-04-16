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
extern crate google_games1 as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  games1 [options] achievement-definitions list [-p <v>]... [-o <out>]
  games1 [options] achievements increment <achievement-id> <steps-to-increment> [-p <v>]... [-o <out>]
  games1 [options] achievements list <player-id> [-p <v>]... [-o <out>]
  games1 [options] achievements reveal <achievement-id> [-p <v>]... [-o <out>]
  games1 [options] achievements set-steps-at-least <achievement-id> <steps> [-p <v>]... [-o <out>]
  games1 [options] achievements unlock <achievement-id> [-p <v>]... [-o <out>]
  games1 [options] achievements update-multiple -r <kv>... [-p <v>]... [-o <out>]
  games1 [options] applications get <application-id> [-p <v>]... [-o <out>]
  games1 [options] applications played [-p <v>]...
  games1 [options] events list-by-player [-p <v>]... [-o <out>]
  games1 [options] events list-definitions [-p <v>]... [-o <out>]
  games1 [options] events record -r <kv>... [-p <v>]... [-o <out>]
  games1 [options] leaderboards get <leaderboard-id> [-p <v>]... [-o <out>]
  games1 [options] leaderboards list [-p <v>]... [-o <out>]
  games1 [options] metagame get-metagame-config [-p <v>]... [-o <out>]
  games1 [options] metagame list-categories-by-player <player-id> <collection> [-p <v>]... [-o <out>]
  games1 [options] players get <player-id> [-p <v>]... [-o <out>]
  games1 [options] players list <collection> [-p <v>]... [-o <out>]
  games1 [options] pushtokens remove -r <kv>... [-p <v>]...
  games1 [options] pushtokens update -r <kv>... [-p <v>]...
  games1 [options] quest-milestones claim <quest-id> <milestone-id> <request-id> [-p <v>]...
  games1 [options] quests accept <quest-id> [-p <v>]... [-o <out>]
  games1 [options] quests list <player-id> [-p <v>]... [-o <out>]
  games1 [options] revisions check <client-revision> [-p <v>]... [-o <out>]
  games1 [options] rooms create -r <kv>... [-p <v>]... [-o <out>]
  games1 [options] rooms decline <room-id> [-p <v>]... [-o <out>]
  games1 [options] rooms dismiss <room-id> [-p <v>]...
  games1 [options] rooms get <room-id> [-p <v>]... [-o <out>]
  games1 [options] rooms join <room-id> -r <kv>... [-p <v>]... [-o <out>]
  games1 [options] rooms leave <room-id> -r <kv>... [-p <v>]... [-o <out>]
  games1 [options] rooms list [-p <v>]... [-o <out>]
  games1 [options] rooms report-status <room-id> -r <kv>... [-p <v>]... [-o <out>]
  games1 [options] scores get <player-id> <leaderboard-id> <time-span> [-p <v>]... [-o <out>]
  games1 [options] scores list <leaderboard-id> <collection> <time-span> [-p <v>]... [-o <out>]
  games1 [options] scores list-window <leaderboard-id> <collection> <time-span> [-p <v>]... [-o <out>]
  games1 [options] scores submit <leaderboard-id> <score> [-p <v>]... [-o <out>]
  games1 [options] scores submit-multiple -r <kv>... [-p <v>]... [-o <out>]
  games1 [options] snapshots get <snapshot-id> [-p <v>]... [-o <out>]
  games1 [options] snapshots list <player-id> [-p <v>]... [-o <out>]
  games1 [options] turn-based-matches cancel <match-id> [-p <v>]...
  games1 [options] turn-based-matches create -r <kv>... [-p <v>]... [-o <out>]
  games1 [options] turn-based-matches decline <match-id> [-p <v>]... [-o <out>]
  games1 [options] turn-based-matches dismiss <match-id> [-p <v>]...
  games1 [options] turn-based-matches finish <match-id> -r <kv>... [-p <v>]... [-o <out>]
  games1 [options] turn-based-matches get <match-id> [-p <v>]... [-o <out>]
  games1 [options] turn-based-matches join <match-id> [-p <v>]... [-o <out>]
  games1 [options] turn-based-matches leave <match-id> [-p <v>]... [-o <out>]
  games1 [options] turn-based-matches leave-turn <match-id> <match-version> [-p <v>]... [-o <out>]
  games1 [options] turn-based-matches list [-p <v>]... [-o <out>]
  games1 [options] turn-based-matches rematch <match-id> [-p <v>]... [-o <out>]
  games1 [options] turn-based-matches sync [-p <v>]... [-o <out>]
  games1 [options] turn-based-matches take-turn <match-id> -r <kv>... [-p <v>]... [-o <out>]
  games1 --help

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
    hub: api::Games<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _achievement_definitions_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.achievement_definitions().list();
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

    fn _achievements_increment(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let steps_to_increment: i32 = arg_from_str(&self.opt.arg_steps_to_increment, err, "<steps-to-increment>", "integer");
        let mut call = self.hub.achievements().increment(&self.opt.arg_achievement_id, steps_to_increment);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "request-id" => {
                    call = call.request_id(value.unwrap_or(""));
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

    fn _achievements_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.achievements().list(&self.opt.arg_player_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "state" => {
                    call = call.state(value.unwrap_or(""));
                },
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

    fn _achievements_reveal(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.achievements().reveal(&self.opt.arg_achievement_id);
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

    fn _achievements_set_steps_at_least(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let steps: i32 = arg_from_str(&self.opt.arg_steps, err, "<steps>", "integer");
        let mut call = self.hub.achievements().set_steps_at_least(&self.opt.arg_achievement_id, steps);
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

    fn _achievements_unlock(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.achievements().unlock(&self.opt.arg_achievement_id);
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

    fn _achievements_update_multiple(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::AchievementUpdateMultipleRequest = Default::default();
        let mut call = self.hub.achievements().update_multiple(&request);
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

    fn _applications_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.applications().get(&self.opt.arg_application_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "platform-type" => {
                    call = call.platform_type(value.unwrap_or(""));
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

    fn _applications_played(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.applications().played();
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

    fn _events_list_by_player(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.events().list_by_player();
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

    fn _events_list_definitions(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.events().list_definitions();
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

    fn _events_record(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::EventRecordRequest = Default::default();
        let mut call = self.hub.events().record(&request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
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
                "request-id" => {
                        request.request_id = Some(value.unwrap_or("").to_string());
                    },
                "current-time-millis" => {
                        request.current_time_millis = Some(value.unwrap_or("").to_string());
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

    fn _leaderboards_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.leaderboards().get(&self.opt.arg_leaderboard_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
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

    fn _leaderboards_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.leaderboards().list();
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

    fn _metagame_get_metagame_config(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.metagame().get_metagame_config();
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

    fn _metagame_list_categories_by_player(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.metagame().list_categories_by_player(&self.opt.arg_player_id, &self.opt.arg_collection);
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

    fn _players_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.players().get(&self.opt.arg_player_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
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

    fn _players_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.players().list(&self.opt.arg_collection);
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

    fn _pushtokens_remove(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::PushTokenId = Default::default();
        let mut call = self.hub.pushtokens().remove(&request);
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
            fn request_ios_init(request: &mut api::PushTokenId) {
                if request.ios.is_none() {
                    request.ios = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "ios.apns-device-token" => {
                        request_ios_init(&mut request);
                        request.ios.as_mut().unwrap().apns_device_token = value.unwrap_or("").to_string();
                    },
                "ios.apns-environment" => {
                        request_ios_init(&mut request);
                        request.ios.as_mut().unwrap().apns_environment = value.unwrap_or("").to_string();
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

    fn _pushtokens_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::PushToken = Default::default();
        let mut call = self.hub.pushtokens().update(&request);
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
            fn request_id_init(request: &mut api::PushToken) {
                if request.id.is_none() {
                    request.id = Some(Default::default());
                }
            }
            
            fn request_id_ios_init(request: &mut api::PushToken) {
                request_id_init(request);
                if request.id.as_mut().unwrap().ios.is_none() {
                    request.id.as_mut().unwrap().ios = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "client-revision" => {
                        request.client_revision = Some(value.unwrap_or("").to_string());
                    },
                "id.kind" => {
                        request_id_init(&mut request);
                        request.id.as_mut().unwrap().kind = Some(value.unwrap_or("").to_string());
                    },
                "id.ios.apns-device-token" => {
                        request_id_ios_init(&mut request);
                        request.id.as_mut().unwrap().ios.as_mut().unwrap().apns_device_token = value.unwrap_or("").to_string();
                    },
                "id.ios.apns-environment" => {
                        request_id_ios_init(&mut request);
                        request.id.as_mut().unwrap().ios.as_mut().unwrap().apns_environment = value.unwrap_or("").to_string();
                    },
                "language" => {
                        request_id_init(&mut request);
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

    fn _quest_milestones_claim(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.quest_milestones().claim(&self.opt.arg_quest_id, &self.opt.arg_milestone_id, &self.opt.arg_request_id);
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

    fn _quests_accept(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.quests().accept(&self.opt.arg_quest_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
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

    fn _quests_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.quests().list(&self.opt.arg_player_id);
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

    fn _revisions_check(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.revisions().check(&self.opt.arg_client_revision);
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

    fn _rooms_create(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::RoomCreateRequest = Default::default();
        let mut call = self.hub.rooms().create(&request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
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
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_auto_matching_criteria_init(request: &mut api::RoomCreateRequest) {
                if request.auto_matching_criteria.is_none() {
                    request.auto_matching_criteria = Some(Default::default());
                }
            }
            
            fn request_client_address_init(request: &mut api::RoomCreateRequest) {
                if request.client_address.is_none() {
                    request.client_address = Some(Default::default());
                }
            }
            
            fn request_network_diagnostics_init(request: &mut api::RoomCreateRequest) {
                if request.network_diagnostics.is_none() {
                    request.network_diagnostics = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "auto-matching-criteria.kind" => {
                        request_auto_matching_criteria_init(&mut request);
                        request.auto_matching_criteria.as_mut().unwrap().kind = value.unwrap_or("").to_string();
                    },
                "auto-matching-criteria.min-auto-matching-players" => {
                        request_auto_matching_criteria_init(&mut request);
                        request.auto_matching_criteria.as_mut().unwrap().min_auto_matching_players = arg_from_str(value.unwrap_or("-0"), err, "auto-matching-criteria.min-auto-matching-players", "integer");
                    },
                "auto-matching-criteria.exclusive-bitmask" => {
                        request_auto_matching_criteria_init(&mut request);
                        request.auto_matching_criteria.as_mut().unwrap().exclusive_bitmask = value.unwrap_or("").to_string();
                    },
                "auto-matching-criteria.max-auto-matching-players" => {
                        request_auto_matching_criteria_init(&mut request);
                        request.auto_matching_criteria.as_mut().unwrap().max_auto_matching_players = arg_from_str(value.unwrap_or("-0"), err, "auto-matching-criteria.max-auto-matching-players", "integer");
                    },
                "invited-player-ids" => {
                        request_auto_matching_criteria_init(&mut request);
                        if request.invited_player_ids.is_none() {
                            request.invited_player_ids = Some(Default::default());
                        }
                        request.invited_player_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "variant" => {
                        request_auto_matching_criteria_init(&mut request);
                        request.variant = Some(arg_from_str(value.unwrap_or("-0"), err, "variant", "integer"));
                    },
                "capabilities" => {
                        request_auto_matching_criteria_init(&mut request);
                        if request.capabilities.is_none() {
                            request.capabilities = Some(Default::default());
                        }
                        request.capabilities.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "network-diagnostics.kind" => {
                        request_network_diagnostics_init(&mut request);
                        request.network_diagnostics.as_mut().unwrap().kind = value.unwrap_or("").to_string();
                    },
                "network-diagnostics.ios-network-type" => {
                        request_network_diagnostics_init(&mut request);
                        request.network_diagnostics.as_mut().unwrap().ios_network_type = arg_from_str(value.unwrap_or("-0"), err, "network-diagnostics.ios-network-type", "integer");
                    },
                "network-diagnostics.network-operator-code" => {
                        request_network_diagnostics_init(&mut request);
                        request.network_diagnostics.as_mut().unwrap().network_operator_code = value.unwrap_or("").to_string();
                    },
                "network-diagnostics.android-network-subtype" => {
                        request_network_diagnostics_init(&mut request);
                        request.network_diagnostics.as_mut().unwrap().android_network_subtype = arg_from_str(value.unwrap_or("-0"), err, "network-diagnostics.android-network-subtype", "integer");
                    },
                "network-diagnostics.network-operator-name" => {
                        request_network_diagnostics_init(&mut request);
                        request.network_diagnostics.as_mut().unwrap().network_operator_name = value.unwrap_or("").to_string();
                    },
                "network-diagnostics.registration-latency-millis" => {
                        request_network_diagnostics_init(&mut request);
                        request.network_diagnostics.as_mut().unwrap().registration_latency_millis = arg_from_str(value.unwrap_or("-0"), err, "network-diagnostics.registration-latency-millis", "integer");
                    },
                "network-diagnostics.android-network-type" => {
                        request_network_diagnostics_init(&mut request);
                        request.network_diagnostics.as_mut().unwrap().android_network_type = arg_from_str(value.unwrap_or("-0"), err, "network-diagnostics.android-network-type", "integer");
                    },
                "client-address.kind" => {
                        request_client_address_init(&mut request);
                        request.client_address.as_mut().unwrap().kind = value.unwrap_or("").to_string();
                    },
                "client-address.xmpp-address" => {
                        request_client_address_init(&mut request);
                        request.client_address.as_mut().unwrap().xmpp_address = value.unwrap_or("").to_string();
                    },
                "request-id" => {
                        request_client_address_init(&mut request);
                        request.request_id = Some(value.unwrap_or("").to_string());
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

    fn _rooms_decline(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.rooms().decline(&self.opt.arg_room_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
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

    fn _rooms_dismiss(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.rooms().dismiss(&self.opt.arg_room_id);
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

    fn _rooms_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.rooms().get(&self.opt.arg_room_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
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

    fn _rooms_join(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::RoomJoinRequest = Default::default();
        let mut call = self.hub.rooms().join(&request, &self.opt.arg_room_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
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
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_client_address_init(request: &mut api::RoomJoinRequest) {
                if request.client_address.is_none() {
                    request.client_address = Some(Default::default());
                }
            }
            
            fn request_network_diagnostics_init(request: &mut api::RoomJoinRequest) {
                if request.network_diagnostics.is_none() {
                    request.network_diagnostics = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "network-diagnostics.kind" => {
                        request_network_diagnostics_init(&mut request);
                        request.network_diagnostics.as_mut().unwrap().kind = value.unwrap_or("").to_string();
                    },
                "network-diagnostics.ios-network-type" => {
                        request_network_diagnostics_init(&mut request);
                        request.network_diagnostics.as_mut().unwrap().ios_network_type = arg_from_str(value.unwrap_or("-0"), err, "network-diagnostics.ios-network-type", "integer");
                    },
                "network-diagnostics.network-operator-code" => {
                        request_network_diagnostics_init(&mut request);
                        request.network_diagnostics.as_mut().unwrap().network_operator_code = value.unwrap_or("").to_string();
                    },
                "network-diagnostics.android-network-subtype" => {
                        request_network_diagnostics_init(&mut request);
                        request.network_diagnostics.as_mut().unwrap().android_network_subtype = arg_from_str(value.unwrap_or("-0"), err, "network-diagnostics.android-network-subtype", "integer");
                    },
                "network-diagnostics.network-operator-name" => {
                        request_network_diagnostics_init(&mut request);
                        request.network_diagnostics.as_mut().unwrap().network_operator_name = value.unwrap_or("").to_string();
                    },
                "network-diagnostics.registration-latency-millis" => {
                        request_network_diagnostics_init(&mut request);
                        request.network_diagnostics.as_mut().unwrap().registration_latency_millis = arg_from_str(value.unwrap_or("-0"), err, "network-diagnostics.registration-latency-millis", "integer");
                    },
                "network-diagnostics.android-network-type" => {
                        request_network_diagnostics_init(&mut request);
                        request.network_diagnostics.as_mut().unwrap().android_network_type = arg_from_str(value.unwrap_or("-0"), err, "network-diagnostics.android-network-type", "integer");
                    },
                "client-address.kind" => {
                        request_client_address_init(&mut request);
                        request.client_address.as_mut().unwrap().kind = value.unwrap_or("").to_string();
                    },
                "client-address.xmpp-address" => {
                        request_client_address_init(&mut request);
                        request.client_address.as_mut().unwrap().xmpp_address = value.unwrap_or("").to_string();
                    },
                "kind" => {
                        request_client_address_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "capabilities" => {
                        request_client_address_init(&mut request);
                        if request.capabilities.is_none() {
                            request.capabilities = Some(Default::default());
                        }
                        request.capabilities.as_mut().unwrap().push(value.unwrap_or("").to_string());
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

    fn _rooms_leave(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::RoomLeaveRequest = Default::default();
        let mut call = self.hub.rooms().leave(&request, &self.opt.arg_room_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
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
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_leave_diagnostics_init(request: &mut api::RoomLeaveRequest) {
                if request.leave_diagnostics.is_none() {
                    request.leave_diagnostics = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "reason" => {
                        request.reason = Some(value.unwrap_or("").to_string());
                    },
                "leave-diagnostics.kind" => {
                        request_leave_diagnostics_init(&mut request);
                        request.leave_diagnostics.as_mut().unwrap().kind = value.unwrap_or("").to_string();
                    },
                "leave-diagnostics.sockets-used" => {
                        request_leave_diagnostics_init(&mut request);
                        request.leave_diagnostics.as_mut().unwrap().sockets_used = arg_from_str(value.unwrap_or("false"), err, "leave-diagnostics.sockets-used", "boolean");
                    },
                "leave-diagnostics.ios-network-type" => {
                        request_leave_diagnostics_init(&mut request);
                        request.leave_diagnostics.as_mut().unwrap().ios_network_type = arg_from_str(value.unwrap_or("-0"), err, "leave-diagnostics.ios-network-type", "integer");
                    },
                "leave-diagnostics.network-operator-code" => {
                        request_leave_diagnostics_init(&mut request);
                        request.leave_diagnostics.as_mut().unwrap().network_operator_code = value.unwrap_or("").to_string();
                    },
                "leave-diagnostics.android-network-subtype" => {
                        request_leave_diagnostics_init(&mut request);
                        request.leave_diagnostics.as_mut().unwrap().android_network_subtype = arg_from_str(value.unwrap_or("-0"), err, "leave-diagnostics.android-network-subtype", "integer");
                    },
                "leave-diagnostics.network-operator-name" => {
                        request_leave_diagnostics_init(&mut request);
                        request.leave_diagnostics.as_mut().unwrap().network_operator_name = value.unwrap_or("").to_string();
                    },
                "leave-diagnostics.android-network-type" => {
                        request_leave_diagnostics_init(&mut request);
                        request.leave_diagnostics.as_mut().unwrap().android_network_type = arg_from_str(value.unwrap_or("-0"), err, "leave-diagnostics.android-network-type", "integer");
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

    fn _rooms_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.rooms().list();
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

    fn _rooms_report_status(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::RoomP2PStatuses = Default::default();
        let mut call = self.hub.rooms().report_status(&request, &self.opt.arg_room_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
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

    fn _scores_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.scores().get(&self.opt.arg_player_id, &self.opt.arg_leaderboard_id, &self.opt.arg_time_span);
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
                "include-rank-type" => {
                    call = call.include_rank_type(value.unwrap_or(""));
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

    fn _scores_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.scores().list(&self.opt.arg_leaderboard_id, &self.opt.arg_collection, &self.opt.arg_time_span);
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

    fn _scores_list_window(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.scores().list_window(&self.opt.arg_leaderboard_id, &self.opt.arg_collection, &self.opt.arg_time_span);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "return-top-if-absent" => {
                    call = call.return_top_if_absent(arg_from_str(value.unwrap_or("false"), err, "return-top-if-absent", "boolean"));
                },
                "results-above" => {
                    call = call.results_above(arg_from_str(value.unwrap_or("-0"), err, "results-above", "integer"));
                },
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

    fn _scores_submit(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.scores().submit(&self.opt.arg_leaderboard_id, &self.opt.arg_score);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "score-tag" => {
                    call = call.score_tag(value.unwrap_or(""));
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

    fn _scores_submit_multiple(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::PlayerScoreSubmissionList = Default::default();
        let mut call = self.hub.scores().submit_multiple(&request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
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

    fn _snapshots_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.snapshots().get(&self.opt.arg_snapshot_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
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

    fn _snapshots_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.snapshots().list(&self.opt.arg_player_id);
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

    fn _turn_based_matches_cancel(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.turn_based_matches().cancel(&self.opt.arg_match_id);
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

    fn _turn_based_matches_create(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::TurnBasedMatchCreateRequest = Default::default();
        let mut call = self.hub.turn_based_matches().create(&request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
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
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_auto_matching_criteria_init(request: &mut api::TurnBasedMatchCreateRequest) {
                if request.auto_matching_criteria.is_none() {
                    request.auto_matching_criteria = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "invited-player-ids" => {
                        if request.invited_player_ids.is_none() {
                            request.invited_player_ids = Some(Default::default());
                        }
                        request.invited_player_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "auto-matching-criteria.kind" => {
                        request_auto_matching_criteria_init(&mut request);
                        request.auto_matching_criteria.as_mut().unwrap().kind = value.unwrap_or("").to_string();
                    },
                "auto-matching-criteria.min-auto-matching-players" => {
                        request_auto_matching_criteria_init(&mut request);
                        request.auto_matching_criteria.as_mut().unwrap().min_auto_matching_players = arg_from_str(value.unwrap_or("-0"), err, "auto-matching-criteria.min-auto-matching-players", "integer");
                    },
                "auto-matching-criteria.exclusive-bitmask" => {
                        request_auto_matching_criteria_init(&mut request);
                        request.auto_matching_criteria.as_mut().unwrap().exclusive_bitmask = value.unwrap_or("").to_string();
                    },
                "auto-matching-criteria.max-auto-matching-players" => {
                        request_auto_matching_criteria_init(&mut request);
                        request.auto_matching_criteria.as_mut().unwrap().max_auto_matching_players = arg_from_str(value.unwrap_or("-0"), err, "auto-matching-criteria.max-auto-matching-players", "integer");
                    },
                "variant" => {
                        request_auto_matching_criteria_init(&mut request);
                        request.variant = Some(arg_from_str(value.unwrap_or("-0"), err, "variant", "integer"));
                    },
                "request-id" => {
                        request_auto_matching_criteria_init(&mut request);
                        request.request_id = Some(value.unwrap_or("").to_string());
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

    fn _turn_based_matches_decline(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.turn_based_matches().decline(&self.opt.arg_match_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
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

    fn _turn_based_matches_dismiss(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.turn_based_matches().dismiss(&self.opt.arg_match_id);
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

    fn _turn_based_matches_finish(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::TurnBasedMatchResults = Default::default();
        let mut call = self.hub.turn_based_matches().finish(&request, &self.opt.arg_match_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
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
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_data_init(request: &mut api::TurnBasedMatchResults) {
                if request.data.is_none() {
                    request.data = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "data.kind" => {
                        request_data_init(&mut request);
                        request.data.as_mut().unwrap().kind = value.unwrap_or("").to_string();
                    },
                "data.data" => {
                        request_data_init(&mut request);
                        request.data.as_mut().unwrap().data = value.unwrap_or("").to_string();
                    },
                "match-version" => {
                        request_data_init(&mut request);
                        request.match_version = Some(arg_from_str(value.unwrap_or("-0"), err, "match-version", "integer"));
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

    fn _turn_based_matches_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.turn_based_matches().get(&self.opt.arg_match_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "language" => {
                    call = call.language(value.unwrap_or(""));
                },
                "include-match-data" => {
                    call = call.include_match_data(arg_from_str(value.unwrap_or("false"), err, "include-match-data", "boolean"));
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

    fn _turn_based_matches_join(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.turn_based_matches().join(&self.opt.arg_match_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
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

    fn _turn_based_matches_leave(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.turn_based_matches().leave(&self.opt.arg_match_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
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

    fn _turn_based_matches_leave_turn(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let match_version: i32 = arg_from_str(&self.opt.arg_match_version, err, "<match-version>", "integer");
        let mut call = self.hub.turn_based_matches().leave_turn(&self.opt.arg_match_id, match_version);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "pending-participant-id" => {
                    call = call.pending_participant_id(value.unwrap_or(""));
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

    fn _turn_based_matches_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.turn_based_matches().list();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "max-completed-matches" => {
                    call = call.max_completed_matches(arg_from_str(value.unwrap_or("-0"), err, "max-completed-matches", "integer"));
                },
                "language" => {
                    call = call.language(value.unwrap_or(""));
                },
                "include-match-data" => {
                    call = call.include_match_data(arg_from_str(value.unwrap_or("false"), err, "include-match-data", "boolean"));
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

    fn _turn_based_matches_rematch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.turn_based_matches().rematch(&self.opt.arg_match_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "request-id" => {
                    call = call.request_id(value.unwrap_or(""));
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

    fn _turn_based_matches_sync(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.turn_based_matches().sync();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "max-completed-matches" => {
                    call = call.max_completed_matches(arg_from_str(value.unwrap_or("-0"), err, "max-completed-matches", "integer"));
                },
                "language" => {
                    call = call.language(value.unwrap_or(""));
                },
                "include-match-data" => {
                    call = call.include_match_data(arg_from_str(value.unwrap_or("false"), err, "include-match-data", "boolean"));
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

    fn _turn_based_matches_take_turn(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::TurnBasedMatchTurn = Default::default();
        let mut call = self.hub.turn_based_matches().take_turn(&request, &self.opt.arg_match_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
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
        let mut field_name: FieldCursor = Default::default();
        for kvarg in self.opt.arg_kv.iter() {
            let (key, value) = parse_kv_arg(&*kvarg, err);
            if let Err(field_err) = field_name.set(&*key) {
                err.issues.push(field_err);
            }
            fn request_data_init(request: &mut api::TurnBasedMatchTurn) {
                if request.data.is_none() {
                    request.data = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "data.kind" => {
                        request_data_init(&mut request);
                        request.data.as_mut().unwrap().kind = value.unwrap_or("").to_string();
                    },
                "data.data" => {
                        request_data_init(&mut request);
                        request.data.as_mut().unwrap().data = value.unwrap_or("").to_string();
                    },
                "match-version" => {
                        request_data_init(&mut request);
                        request.match_version = Some(arg_from_str(value.unwrap_or("-0"), err, "match-version", "integer"));
                    },
                "pending-participant-id" => {
                        request_data_init(&mut request);
                        request.pending_participant_id = Some(value.unwrap_or("").to_string());
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

        if self.opt.cmd_achievement_definitions {
            if self.opt.cmd_list {
                call_result = self._achievement_definitions_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_achievements {
            if self.opt.cmd_increment {
                call_result = self._achievements_increment(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._achievements_list(dry_run, &mut err);
            } else if self.opt.cmd_reveal {
                call_result = self._achievements_reveal(dry_run, &mut err);
            } else if self.opt.cmd_set_steps_at_least {
                call_result = self._achievements_set_steps_at_least(dry_run, &mut err);
            } else if self.opt.cmd_unlock {
                call_result = self._achievements_unlock(dry_run, &mut err);
            } else if self.opt.cmd_update_multiple {
                call_result = self._achievements_update_multiple(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_applications {
            if self.opt.cmd_get {
                call_result = self._applications_get(dry_run, &mut err);
            } else if self.opt.cmd_played {
                call_result = self._applications_played(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_events {
            if self.opt.cmd_list_by_player {
                call_result = self._events_list_by_player(dry_run, &mut err);
            } else if self.opt.cmd_list_definitions {
                call_result = self._events_list_definitions(dry_run, &mut err);
            } else if self.opt.cmd_record {
                call_result = self._events_record(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_leaderboards {
            if self.opt.cmd_get {
                call_result = self._leaderboards_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._leaderboards_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_metagame {
            if self.opt.cmd_get_metagame_config {
                call_result = self._metagame_get_metagame_config(dry_run, &mut err);
            } else if self.opt.cmd_list_categories_by_player {
                call_result = self._metagame_list_categories_by_player(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_players {
            if self.opt.cmd_get {
                call_result = self._players_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._players_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_pushtokens {
            if self.opt.cmd_remove {
                call_result = self._pushtokens_remove(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._pushtokens_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_quest_milestones {
            if self.opt.cmd_claim {
                call_result = self._quest_milestones_claim(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_quests {
            if self.opt.cmd_accept {
                call_result = self._quests_accept(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._quests_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_revisions {
            if self.opt.cmd_check {
                call_result = self._revisions_check(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_rooms {
            if self.opt.cmd_create {
                call_result = self._rooms_create(dry_run, &mut err);
            } else if self.opt.cmd_decline {
                call_result = self._rooms_decline(dry_run, &mut err);
            } else if self.opt.cmd_dismiss {
                call_result = self._rooms_dismiss(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._rooms_get(dry_run, &mut err);
            } else if self.opt.cmd_join {
                call_result = self._rooms_join(dry_run, &mut err);
            } else if self.opt.cmd_leave {
                call_result = self._rooms_leave(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._rooms_list(dry_run, &mut err);
            } else if self.opt.cmd_report_status {
                call_result = self._rooms_report_status(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_scores {
            if self.opt.cmd_get {
                call_result = self._scores_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._scores_list(dry_run, &mut err);
            } else if self.opt.cmd_list_window {
                call_result = self._scores_list_window(dry_run, &mut err);
            } else if self.opt.cmd_submit {
                call_result = self._scores_submit(dry_run, &mut err);
            } else if self.opt.cmd_submit_multiple {
                call_result = self._scores_submit_multiple(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_snapshots {
            if self.opt.cmd_get {
                call_result = self._snapshots_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._snapshots_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_turn_based_matches {
            if self.opt.cmd_cancel {
                call_result = self._turn_based_matches_cancel(dry_run, &mut err);
            } else if self.opt.cmd_create {
                call_result = self._turn_based_matches_create(dry_run, &mut err);
            } else if self.opt.cmd_decline {
                call_result = self._turn_based_matches_decline(dry_run, &mut err);
            } else if self.opt.cmd_dismiss {
                call_result = self._turn_based_matches_dismiss(dry_run, &mut err);
            } else if self.opt.cmd_finish {
                call_result = self._turn_based_matches_finish(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._turn_based_matches_get(dry_run, &mut err);
            } else if self.opt.cmd_join {
                call_result = self._turn_based_matches_join(dry_run, &mut err);
            } else if self.opt.cmd_leave {
                call_result = self._turn_based_matches_leave(dry_run, &mut err);
            } else if self.opt.cmd_leave_turn {
                call_result = self._turn_based_matches_leave_turn(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._turn_based_matches_list(dry_run, &mut err);
            } else if self.opt.cmd_rematch {
                call_result = self._turn_based_matches_rematch(dry_run, &mut err);
            } else if self.opt.cmd_sync {
                call_result = self._turn_based_matches_sync(dry_run, &mut err);
            } else if self.opt.cmd_take_turn {
                call_result = self._turn_based_matches_take_turn(dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "games1-secret.json") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                                      hyper::Client::new(),
                                      JsonTokenStorage {
                                        program_name: "games1",
                                        db_dir: config_dir.clone(),
                                      }, None);
        let engine = Engine {
            opt: opt,
            hub: api::Games::new(hyper::Client::new(), auth),
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