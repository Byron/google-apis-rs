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
extern crate google_groupssettings1 as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  groupssettings1 [options] groups get <group-unique-id> [-p <v>...] [-o <out>]
  groupssettings1 [options] groups patch <group-unique-id> -r <kv>... [-p <v>...] [-o <out>]
  groupssettings1 [options] groups update <group-unique-id> -r <kv>... [-p <v>...] [-o <out>]
  groupssettings1 --help

All documentation details can be found at
http://byron.github.io/google-apis-rs/google_groupssettings1_cli/index.html

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
    hub: api::Groupssettings<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _groups_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.groups().get(&self.opt.arg_group_unique_id);
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

    fn _groups_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Groups::default();
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
            match &temp_cursor.to_string()[..] {
                "allow-external-members" => {
                        request.allow_external_members = Some(value.unwrap_or("").to_string());
                    },
                "who-can-post-message" => {
                        request.who_can_post_message = Some(value.unwrap_or("").to_string());
                    },
                "primary-language" => {
                        request.primary_language = Some(value.unwrap_or("").to_string());
                    },
                "who-can-view-membership" => {
                        request.who_can_view_membership = Some(value.unwrap_or("").to_string());
                    },
                "default-message-deny-notification-text" => {
                        request.default_message_deny_notification_text = Some(value.unwrap_or("").to_string());
                    },
                "include-in-global-address-list" => {
                        request.include_in_global_address_list = Some(value.unwrap_or("").to_string());
                    },
                "archive-only" => {
                        request.archive_only = Some(value.unwrap_or("").to_string());
                    },
                "is-archived" => {
                        request.is_archived = Some(value.unwrap_or("").to_string());
                    },
                "members-can-post-as-the-group" => {
                        request.members_can_post_as_the_group = Some(value.unwrap_or("").to_string());
                    },
                "allow-web-posting" => {
                        request.allow_web_posting = Some(value.unwrap_or("").to_string());
                    },
                "email" => {
                        request.email = Some(value.unwrap_or("").to_string());
                    },
                "message-moderation-level" => {
                        request.message_moderation_level = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "reply-to" => {
                        request.reply_to = Some(value.unwrap_or("").to_string());
                    },
                "custom-reply-to" => {
                        request.custom_reply_to = Some(value.unwrap_or("").to_string());
                    },
                "send-message-deny-notification" => {
                        request.send_message_deny_notification = Some(value.unwrap_or("").to_string());
                    },
                "who-can-contact-owner" => {
                        request.who_can_contact_owner = Some(value.unwrap_or("").to_string());
                    },
                "message-display-font" => {
                        request.message_display_font = Some(value.unwrap_or("").to_string());
                    },
                "who-can-leave-group" => {
                        request.who_can_leave_group = Some(value.unwrap_or("").to_string());
                    },
                "who-can-join" => {
                        request.who_can_join = Some(value.unwrap_or("").to_string());
                    },
                "who-can-invite" => {
                        request.who_can_invite = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "spam-moderation-level" => {
                        request.spam_moderation_level = Some(value.unwrap_or("").to_string());
                    },
                "who-can-view-group" => {
                        request.who_can_view_group = Some(value.unwrap_or("").to_string());
                    },
                "show-in-group-directory" => {
                        request.show_in_group_directory = Some(value.unwrap_or("").to_string());
                    },
                "max-message-bytes" => {
                        request.max_message_bytes = Some(arg_from_str(value.unwrap_or("-0"), err, "max-message-bytes", "integer"));
                    },
                "allow-google-communication" => {
                        request.allow_google_communication = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.groups().patch(request, &self.opt.arg_group_unique_id);
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

    fn _groups_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        
        let mut request = api::Groups::default();
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
            match &temp_cursor.to_string()[..] {
                "allow-external-members" => {
                        request.allow_external_members = Some(value.unwrap_or("").to_string());
                    },
                "who-can-post-message" => {
                        request.who_can_post_message = Some(value.unwrap_or("").to_string());
                    },
                "primary-language" => {
                        request.primary_language = Some(value.unwrap_or("").to_string());
                    },
                "who-can-view-membership" => {
                        request.who_can_view_membership = Some(value.unwrap_or("").to_string());
                    },
                "default-message-deny-notification-text" => {
                        request.default_message_deny_notification_text = Some(value.unwrap_or("").to_string());
                    },
                "include-in-global-address-list" => {
                        request.include_in_global_address_list = Some(value.unwrap_or("").to_string());
                    },
                "archive-only" => {
                        request.archive_only = Some(value.unwrap_or("").to_string());
                    },
                "is-archived" => {
                        request.is_archived = Some(value.unwrap_or("").to_string());
                    },
                "members-can-post-as-the-group" => {
                        request.members_can_post_as_the_group = Some(value.unwrap_or("").to_string());
                    },
                "allow-web-posting" => {
                        request.allow_web_posting = Some(value.unwrap_or("").to_string());
                    },
                "email" => {
                        request.email = Some(value.unwrap_or("").to_string());
                    },
                "message-moderation-level" => {
                        request.message_moderation_level = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "reply-to" => {
                        request.reply_to = Some(value.unwrap_or("").to_string());
                    },
                "custom-reply-to" => {
                        request.custom_reply_to = Some(value.unwrap_or("").to_string());
                    },
                "send-message-deny-notification" => {
                        request.send_message_deny_notification = Some(value.unwrap_or("").to_string());
                    },
                "who-can-contact-owner" => {
                        request.who_can_contact_owner = Some(value.unwrap_or("").to_string());
                    },
                "message-display-font" => {
                        request.message_display_font = Some(value.unwrap_or("").to_string());
                    },
                "who-can-leave-group" => {
                        request.who_can_leave_group = Some(value.unwrap_or("").to_string());
                    },
                "who-can-join" => {
                        request.who_can_join = Some(value.unwrap_or("").to_string());
                    },
                "who-can-invite" => {
                        request.who_can_invite = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "spam-moderation-level" => {
                        request.spam_moderation_level = Some(value.unwrap_or("").to_string());
                    },
                "who-can-view-group" => {
                        request.who_can_view_group = Some(value.unwrap_or("").to_string());
                    },
                "show-in-group-directory" => {
                        request.show_in_group_directory = Some(value.unwrap_or("").to_string());
                    },
                "max-message-bytes" => {
                        request.max_message_bytes = Some(arg_from_str(value.unwrap_or("-0"), err, "max-message-bytes", "integer"));
                    },
                "allow-google-communication" => {
                        request.allow_google_communication = Some(value.unwrap_or("").to_string());
                    },
                _ => {
                    err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string())));
                }
            }
        }
        let mut call = self.hub.groups().update(request, &self.opt.arg_group_unique_id);
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

        if self.opt.cmd_groups {
            if self.opt.cmd_get {
                call_result = self._groups_get(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._groups_patch(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._groups_update(dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "groupssettings1-secret.json", 
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
                                          program_name: "groupssettings1",
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
            hub: api::Groupssettings::new(client, auth),
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