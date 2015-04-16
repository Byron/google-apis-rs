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
extern crate google_books1 as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  books1 [options] bookshelves get <user-id> <shelf> [-p <v>]... [-o <out>]
  books1 [options] bookshelves list <user-id> [-p <v>]... [-o <out>]
  books1 [options] bookshelves volumes-list <user-id> <shelf> [-p <v>]... [-o <out>]
  books1 [options] cloudloading add-book [-p <v>]... [-o <out>]
  books1 [options] cloudloading delete-book <volume-id> [-p <v>]...
  books1 [options] cloudloading update-book -r <kv>... [-p <v>]... [-o <out>]
  books1 [options] dictionary list-offline-metadata <cpksver> [-p <v>]... [-o <out>]
  books1 [options] layers annotation-data-get <volume-id> <layer-id> <annotation-data-id> <content-version> [-p <v>]... [-o <out>]
  books1 [options] layers annotation-data-list <volume-id> <layer-id> <content-version> [-p <v>]... [-o <out>]
  books1 [options] layers get <volume-id> <summary-id> [-p <v>]... [-o <out>]
  books1 [options] layers list <volume-id> [-p <v>]... [-o <out>]
  books1 [options] layers volume-annotations-get <volume-id> <layer-id> <annotation-id> [-p <v>]... [-o <out>]
  books1 [options] layers volume-annotations-list <volume-id> <layer-id> <content-version> [-p <v>]... [-o <out>]
  books1 [options] myconfig get-user-settings [-p <v>]... [-o <out>]
  books1 [options] myconfig release-download-access <volume-ids> <cpksver> [-p <v>]... [-o <out>]
  books1 [options] myconfig request-access <source> <volume-id> <nonce> <cpksver> [-p <v>]... [-o <out>]
  books1 [options] myconfig sync-volume-licenses <source> <nonce> <cpksver> [-p <v>]... [-o <out>]
  books1 [options] myconfig update-user-settings -r <kv>... [-p <v>]... [-o <out>]
  books1 [options] mylibrary annotations-delete <annotation-id> [-p <v>]...
  books1 [options] mylibrary annotations-insert -r <kv>... [-p <v>]... [-o <out>]
  books1 [options] mylibrary annotations-list [-p <v>]... [-o <out>]
  books1 [options] mylibrary annotations-summary <layer-ids> <volume-id> [-p <v>]... [-o <out>]
  books1 [options] mylibrary annotations-update <annotation-id> -r <kv>... [-p <v>]... [-o <out>]
  books1 [options] mylibrary bookshelves-add-volume <shelf> <volume-id> [-p <v>]...
  books1 [options] mylibrary bookshelves-clear-volumes <shelf> [-p <v>]...
  books1 [options] mylibrary bookshelves-get <shelf> [-p <v>]... [-o <out>]
  books1 [options] mylibrary bookshelves-list [-p <v>]... [-o <out>]
  books1 [options] mylibrary bookshelves-move-volume <shelf> <volume-id> <volume-position> [-p <v>]...
  books1 [options] mylibrary bookshelves-remove-volume <shelf> <volume-id> [-p <v>]...
  books1 [options] mylibrary bookshelves-volumes-list <shelf> [-p <v>]... [-o <out>]
  books1 [options] mylibrary readingpositions-get <volume-id> [-p <v>]... [-o <out>]
  books1 [options] mylibrary readingpositions-set-position <volume-id> <timestamp> <position> [-p <v>]...
  books1 [options] onboarding list-categories [-p <v>]... [-o <out>]
  books1 [options] onboarding list-category-volumes [-p <v>]... [-o <out>]
  books1 [options] promooffer accept [-p <v>]...
  books1 [options] promooffer dismiss [-p <v>]...
  books1 [options] promooffer get [-p <v>]... [-o <out>]
  books1 [options] volumes associated-list <volume-id> [-p <v>]... [-o <out>]
  books1 [options] volumes get <volume-id> [-p <v>]... [-o <out>]
  books1 [options] volumes list <q> [-p <v>]... [-o <out>]
  books1 [options] volumes mybooks-list [-p <v>]... [-o <out>]
  books1 [options] volumes recommended-list [-p <v>]... [-o <out>]
  books1 [options] volumes recommended-rate <rating> <volume-id> [-p <v>]... [-o <out>]
  books1 [options] volumes useruploaded-list [-p <v>]... [-o <out>]
  books1 --help

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
    hub: api::Books<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _bookshelves_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.bookshelves().get(&self.opt.arg_user_id, &self.opt.arg_shelf);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "source" => {
                    call = call.source(value.unwrap_or(""));
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

    fn _bookshelves_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.bookshelves().list(&self.opt.arg_user_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "source" => {
                    call = call.source(value.unwrap_or(""));
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

    fn _bookshelves_volumes_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.bookshelves().volumes_list(&self.opt.arg_user_id, &self.opt.arg_shelf);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
                },
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "show-preorders" => {
                    call = call.show_preorders(arg_from_str(value.unwrap_or("false"), err, "show-preorders", "boolean"));
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

    fn _cloudloading_add_book(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.cloudloading().add_book();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "upload-client-token" => {
                    call = call.upload_client_token(value.unwrap_or(""));
                },
                "name" => {
                    call = call.name(value.unwrap_or(""));
                },
                "mime-type" => {
                    call = call.mime_type(value.unwrap_or(""));
                },
                "drive-document-id" => {
                    call = call.drive_document_id(value.unwrap_or(""));
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

    fn _cloudloading_delete_book(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.cloudloading().delete_book(&self.opt.arg_volume_id);
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

    fn _cloudloading_update_book(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::BooksCloudloadingResource = Default::default();
        let mut call = self.hub.cloudloading().update_book(&request);
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
                "title" => {
                        request.title = Some(value.unwrap_or("").to_string());
                    },
                "processing-state" => {
                        request.processing_state = Some(value.unwrap_or("").to_string());
                    },
                "volume-id" => {
                        request.volume_id = Some(value.unwrap_or("").to_string());
                    },
                "author" => {
                        request.author = Some(value.unwrap_or("").to_string());
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

    fn _dictionary_list_offline_metadata(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.dictionary().list_offline_metadata(&self.opt.arg_cpksver);
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

    fn _layers_annotation_data_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.layers().annotation_data_get(&self.opt.arg_volume_id, &self.opt.arg_layer_id, &self.opt.arg_annotation_data_id, &self.opt.arg_content_version);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "w" => {
                    call = call.w(arg_from_str(value.unwrap_or("-0"), err, "w", "integer"));
                },
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "scale" => {
                    call = call.scale(arg_from_str(value.unwrap_or("-0"), err, "scale", "integer"));
                },
                "locale" => {
                    call = call.locale(value.unwrap_or(""));
                },
                "h" => {
                    call = call.h(arg_from_str(value.unwrap_or("-0"), err, "h", "integer"));
                },
                "allow-web-definitions" => {
                    call = call.allow_web_definitions(arg_from_str(value.unwrap_or("false"), err, "allow-web-definitions", "boolean"));
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

    fn _layers_annotation_data_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.layers().annotation_data_list(&self.opt.arg_volume_id, &self.opt.arg_layer_id, &self.opt.arg_content_version);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "w" => {
                    call = call.w(arg_from_str(value.unwrap_or("-0"), err, "w", "integer"));
                },
                "updated-min" => {
                    call = call.updated_min(value.unwrap_or(""));
                },
                "updated-max" => {
                    call = call.updated_max(value.unwrap_or(""));
                },
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "scale" => {
                    call = call.scale(arg_from_str(value.unwrap_or("-0"), err, "scale", "integer"));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "locale" => {
                    call = call.locale(value.unwrap_or(""));
                },
                "h" => {
                    call = call.h(arg_from_str(value.unwrap_or("-0"), err, "h", "integer"));
                },
                "annotation-data-id" => {
                    call = call.add_annotation_data_id(value.unwrap_or(""));
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

    fn _layers_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.layers().get(&self.opt.arg_volume_id, &self.opt.arg_summary_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "content-version" => {
                    call = call.content_version(value.unwrap_or(""));
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

    fn _layers_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.layers().list(&self.opt.arg_volume_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "content-version" => {
                    call = call.content_version(value.unwrap_or(""));
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

    fn _layers_volume_annotations_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.layers().volume_annotations_get(&self.opt.arg_volume_id, &self.opt.arg_layer_id, &self.opt.arg_annotation_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "locale" => {
                    call = call.locale(value.unwrap_or(""));
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

    fn _layers_volume_annotations_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.layers().volume_annotations_list(&self.opt.arg_volume_id, &self.opt.arg_layer_id, &self.opt.arg_content_version);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "volume-annotations-version" => {
                    call = call.volume_annotations_version(value.unwrap_or(""));
                },
                "updated-min" => {
                    call = call.updated_min(value.unwrap_or(""));
                },
                "updated-max" => {
                    call = call.updated_max(value.unwrap_or(""));
                },
                "start-position" => {
                    call = call.start_position(value.unwrap_or(""));
                },
                "start-offset" => {
                    call = call.start_offset(value.unwrap_or(""));
                },
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "show-deleted" => {
                    call = call.show_deleted(arg_from_str(value.unwrap_or("false"), err, "show-deleted", "boolean"));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "locale" => {
                    call = call.locale(value.unwrap_or(""));
                },
                "end-position" => {
                    call = call.end_position(value.unwrap_or(""));
                },
                "end-offset" => {
                    call = call.end_offset(value.unwrap_or(""));
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

    fn _myconfig_get_user_settings(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.myconfig().get_user_settings();
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

    fn _myconfig_release_download_access(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.myconfig().release_download_access(&self.opt.arg_volume_ids, &self.opt.arg_cpksver);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "locale" => {
                    call = call.locale(value.unwrap_or(""));
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

    fn _myconfig_request_access(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.myconfig().request_access(&self.opt.arg_source, &self.opt.arg_volume_id, &self.opt.arg_nonce, &self.opt.arg_cpksver);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "locale" => {
                    call = call.locale(value.unwrap_or(""));
                },
                "license-types" => {
                    call = call.license_types(value.unwrap_or(""));
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

    fn _myconfig_sync_volume_licenses(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.myconfig().sync_volume_licenses(&self.opt.arg_source, &self.opt.arg_nonce, &self.opt.arg_cpksver);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "volume-ids" => {
                    call = call.add_volume_ids(value.unwrap_or(""));
                },
                "show-preorders" => {
                    call = call.show_preorders(arg_from_str(value.unwrap_or("false"), err, "show-preorders", "boolean"));
                },
                "locale" => {
                    call = call.locale(value.unwrap_or(""));
                },
                "features" => {
                    call = call.add_features(value.unwrap_or(""));
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

    fn _myconfig_update_user_settings(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Usersettings = Default::default();
        let mut call = self.hub.myconfig().update_user_settings(&request);
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
            fn request_notes_export_init(request: &mut api::Usersettings) {
                if request.notes_export.is_none() {
                    request.notes_export = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "notes-export.is-enabled" => {
                        request_notes_export_init(&mut request);
                        request.notes_export.as_mut().unwrap().is_enabled = arg_from_str(value.unwrap_or("false"), err, "notes-export.is-enabled", "boolean");
                    },
                "notes-export.folder-name" => {
                        request_notes_export_init(&mut request);
                        request.notes_export.as_mut().unwrap().folder_name = value.unwrap_or("").to_string();
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

    fn _mylibrary_annotations_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.mylibrary().annotations_delete(&self.opt.arg_annotation_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "source" => {
                    call = call.source(value.unwrap_or(""));
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    None
                }
            }
        }
    }

    fn _mylibrary_annotations_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Annotation = Default::default();
        let mut call = self.hub.mylibrary().annotations_insert(&request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "show-only-summary-in-response" => {
                    call = call.show_only_summary_in_response(arg_from_str(value.unwrap_or("false"), err, "show-only-summary-in-response", "boolean"));
                },
                "country" => {
                    call = call.country(value.unwrap_or(""));
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
            fn request_client_version_ranges_init(request: &mut api::Annotation) {
                if request.client_version_ranges.is_none() {
                    request.client_version_ranges = Some(Default::default());
                }
            }
            
            fn request_current_version_ranges_init(request: &mut api::Annotation) {
                if request.current_version_ranges.is_none() {
                    request.current_version_ranges = Some(Default::default());
                }
            }
            
            fn request_layer_summary_init(request: &mut api::Annotation) {
                if request.layer_summary.is_none() {
                    request.layer_summary = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "page-ids" => {
                        if request.page_ids.is_none() {
                            request.page_ids = Some(Default::default());
                        }
                        request.page_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "deleted" => {
                        request.deleted = Some(arg_from_str(value.unwrap_or("false"), err, "deleted", "boolean"));
                    },
                "before-selected-text" => {
                        request.before_selected_text = Some(value.unwrap_or("").to_string());
                    },
                "current-version-ranges.image-cfi-range.start-position" => {
                        request_current_version_ranges_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().image_cfi_range.start_position = value.unwrap_or("").to_string();
                    },
                "current-version-ranges.image-cfi-range.end-position" => {
                        request_current_version_ranges_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().image_cfi_range.end_position = value.unwrap_or("").to_string();
                    },
                "current-version-ranges.image-cfi-range.start-offset" => {
                        request_current_version_ranges_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().image_cfi_range.start_offset = value.unwrap_or("").to_string();
                    },
                "current-version-ranges.image-cfi-range.end-offset" => {
                        request_current_version_ranges_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().image_cfi_range.end_offset = value.unwrap_or("").to_string();
                    },
                "current-version-ranges.gb-text-range.start-position" => {
                        request_current_version_ranges_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().gb_text_range.start_position = value.unwrap_or("").to_string();
                    },
                "current-version-ranges.gb-text-range.end-position" => {
                        request_current_version_ranges_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().gb_text_range.end_position = value.unwrap_or("").to_string();
                    },
                "current-version-ranges.gb-text-range.start-offset" => {
                        request_current_version_ranges_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().gb_text_range.start_offset = value.unwrap_or("").to_string();
                    },
                "current-version-ranges.gb-text-range.end-offset" => {
                        request_current_version_ranges_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().gb_text_range.end_offset = value.unwrap_or("").to_string();
                    },
                "current-version-ranges.content-version" => {
                        request_current_version_ranges_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().content_version = value.unwrap_or("").to_string();
                    },
                "current-version-ranges.cfi-range.start-position" => {
                        request_current_version_ranges_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().cfi_range.start_position = value.unwrap_or("").to_string();
                    },
                "current-version-ranges.cfi-range.end-position" => {
                        request_current_version_ranges_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().cfi_range.end_position = value.unwrap_or("").to_string();
                    },
                "current-version-ranges.cfi-range.start-offset" => {
                        request_current_version_ranges_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().cfi_range.start_offset = value.unwrap_or("").to_string();
                    },
                "current-version-ranges.cfi-range.end-offset" => {
                        request_current_version_ranges_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().cfi_range.end_offset = value.unwrap_or("").to_string();
                    },
                "current-version-ranges.gb-image-range.start-position" => {
                        request_current_version_ranges_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().gb_image_range.start_position = value.unwrap_or("").to_string();
                    },
                "current-version-ranges.gb-image-range.end-position" => {
                        request_current_version_ranges_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().gb_image_range.end_position = value.unwrap_or("").to_string();
                    },
                "current-version-ranges.gb-image-range.start-offset" => {
                        request_current_version_ranges_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().gb_image_range.start_offset = value.unwrap_or("").to_string();
                    },
                "current-version-ranges.gb-image-range.end-offset" => {
                        request_current_version_ranges_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().gb_image_range.end_offset = value.unwrap_or("").to_string();
                    },
                "after-selected-text" => {
                        request_current_version_ranges_init(&mut request);
                        request.after_selected_text = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_current_version_ranges_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "volume-id" => {
                        request_current_version_ranges_init(&mut request);
                        request.volume_id = Some(value.unwrap_or("").to_string());
                    },
                "layer-summary.limit-type" => {
                        request_layer_summary_init(&mut request);
                        request.layer_summary.as_mut().unwrap().limit_type = value.unwrap_or("").to_string();
                    },
                "layer-summary.remaining-character-count" => {
                        request_layer_summary_init(&mut request);
                        request.layer_summary.as_mut().unwrap().remaining_character_count = arg_from_str(value.unwrap_or("-0"), err, "layer-summary.remaining-character-count", "integer");
                    },
                "layer-summary.allowed-character-count" => {
                        request_layer_summary_init(&mut request);
                        request.layer_summary.as_mut().unwrap().allowed_character_count = arg_from_str(value.unwrap_or("-0"), err, "layer-summary.allowed-character-count", "integer");
                    },
                "selected-text" => {
                        request_layer_summary_init(&mut request);
                        request.selected_text = Some(value.unwrap_or("").to_string());
                    },
                "client-version-ranges.image-cfi-range.start-position" => {
                        request_client_version_ranges_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().image_cfi_range.start_position = value.unwrap_or("").to_string();
                    },
                "client-version-ranges.image-cfi-range.end-position" => {
                        request_client_version_ranges_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().image_cfi_range.end_position = value.unwrap_or("").to_string();
                    },
                "client-version-ranges.image-cfi-range.start-offset" => {
                        request_client_version_ranges_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().image_cfi_range.start_offset = value.unwrap_or("").to_string();
                    },
                "client-version-ranges.image-cfi-range.end-offset" => {
                        request_client_version_ranges_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().image_cfi_range.end_offset = value.unwrap_or("").to_string();
                    },
                "client-version-ranges.gb-text-range.start-position" => {
                        request_client_version_ranges_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().gb_text_range.start_position = value.unwrap_or("").to_string();
                    },
                "client-version-ranges.gb-text-range.end-position" => {
                        request_client_version_ranges_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().gb_text_range.end_position = value.unwrap_or("").to_string();
                    },
                "client-version-ranges.gb-text-range.start-offset" => {
                        request_client_version_ranges_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().gb_text_range.start_offset = value.unwrap_or("").to_string();
                    },
                "client-version-ranges.gb-text-range.end-offset" => {
                        request_client_version_ranges_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().gb_text_range.end_offset = value.unwrap_or("").to_string();
                    },
                "client-version-ranges.content-version" => {
                        request_client_version_ranges_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().content_version = value.unwrap_or("").to_string();
                    },
                "client-version-ranges.cfi-range.start-position" => {
                        request_client_version_ranges_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().cfi_range.start_position = value.unwrap_or("").to_string();
                    },
                "client-version-ranges.cfi-range.end-position" => {
                        request_client_version_ranges_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().cfi_range.end_position = value.unwrap_or("").to_string();
                    },
                "client-version-ranges.cfi-range.start-offset" => {
                        request_client_version_ranges_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().cfi_range.start_offset = value.unwrap_or("").to_string();
                    },
                "client-version-ranges.cfi-range.end-offset" => {
                        request_client_version_ranges_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().cfi_range.end_offset = value.unwrap_or("").to_string();
                    },
                "client-version-ranges.gb-image-range.start-position" => {
                        request_client_version_ranges_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().gb_image_range.start_position = value.unwrap_or("").to_string();
                    },
                "client-version-ranges.gb-image-range.end-position" => {
                        request_client_version_ranges_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().gb_image_range.end_position = value.unwrap_or("").to_string();
                    },
                "client-version-ranges.gb-image-range.start-offset" => {
                        request_client_version_ranges_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().gb_image_range.start_offset = value.unwrap_or("").to_string();
                    },
                "client-version-ranges.gb-image-range.end-offset" => {
                        request_client_version_ranges_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().gb_image_range.end_offset = value.unwrap_or("").to_string();
                    },
                "layer-id" => {
                        request_client_version_ranges_init(&mut request);
                        request.layer_id = Some(value.unwrap_or("").to_string());
                    },
                "highlight-style" => {
                        request_client_version_ranges_init(&mut request);
                        request.highlight_style = Some(value.unwrap_or("").to_string());
                    },
                "data" => {
                        request_client_version_ranges_init(&mut request);
                        request.data = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_client_version_ranges_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_client_version_ranges_init(&mut request);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _mylibrary_annotations_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.mylibrary().annotations_list();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "volume-id" => {
                    call = call.volume_id(value.unwrap_or(""));
                },
                "updated-min" => {
                    call = call.updated_min(value.unwrap_or(""));
                },
                "updated-max" => {
                    call = call.updated_max(value.unwrap_or(""));
                },
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "show-deleted" => {
                    call = call.show_deleted(arg_from_str(value.unwrap_or("false"), err, "show-deleted", "boolean"));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "layer-ids" => {
                    call = call.add_layer_ids(value.unwrap_or(""));
                },
                "layer-id" => {
                    call = call.layer_id(value.unwrap_or(""));
                },
                "content-version" => {
                    call = call.content_version(value.unwrap_or(""));
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

    fn _mylibrary_annotations_summary(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.mylibrary().annotations_summary(&self.opt.arg_layer_ids, &self.opt.arg_volume_id);
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

    fn _mylibrary_annotations_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Annotation = Default::default();
        let mut call = self.hub.mylibrary().annotations_update(&request, &self.opt.arg_annotation_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "source" => {
                    call = call.source(value.unwrap_or(""));
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
            fn request_client_version_ranges_init(request: &mut api::Annotation) {
                if request.client_version_ranges.is_none() {
                    request.client_version_ranges = Some(Default::default());
                }
            }
            
            fn request_current_version_ranges_init(request: &mut api::Annotation) {
                if request.current_version_ranges.is_none() {
                    request.current_version_ranges = Some(Default::default());
                }
            }
            
            fn request_layer_summary_init(request: &mut api::Annotation) {
                if request.layer_summary.is_none() {
                    request.layer_summary = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "page-ids" => {
                        if request.page_ids.is_none() {
                            request.page_ids = Some(Default::default());
                        }
                        request.page_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "created" => {
                        request.created = Some(value.unwrap_or("").to_string());
                    },
                "deleted" => {
                        request.deleted = Some(arg_from_str(value.unwrap_or("false"), err, "deleted", "boolean"));
                    },
                "before-selected-text" => {
                        request.before_selected_text = Some(value.unwrap_or("").to_string());
                    },
                "current-version-ranges.image-cfi-range.start-position" => {
                        request_current_version_ranges_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().image_cfi_range.start_position = value.unwrap_or("").to_string();
                    },
                "current-version-ranges.image-cfi-range.end-position" => {
                        request_current_version_ranges_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().image_cfi_range.end_position = value.unwrap_or("").to_string();
                    },
                "current-version-ranges.image-cfi-range.start-offset" => {
                        request_current_version_ranges_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().image_cfi_range.start_offset = value.unwrap_or("").to_string();
                    },
                "current-version-ranges.image-cfi-range.end-offset" => {
                        request_current_version_ranges_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().image_cfi_range.end_offset = value.unwrap_or("").to_string();
                    },
                "current-version-ranges.gb-text-range.start-position" => {
                        request_current_version_ranges_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().gb_text_range.start_position = value.unwrap_or("").to_string();
                    },
                "current-version-ranges.gb-text-range.end-position" => {
                        request_current_version_ranges_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().gb_text_range.end_position = value.unwrap_or("").to_string();
                    },
                "current-version-ranges.gb-text-range.start-offset" => {
                        request_current_version_ranges_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().gb_text_range.start_offset = value.unwrap_or("").to_string();
                    },
                "current-version-ranges.gb-text-range.end-offset" => {
                        request_current_version_ranges_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().gb_text_range.end_offset = value.unwrap_or("").to_string();
                    },
                "current-version-ranges.content-version" => {
                        request_current_version_ranges_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().content_version = value.unwrap_or("").to_string();
                    },
                "current-version-ranges.cfi-range.start-position" => {
                        request_current_version_ranges_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().cfi_range.start_position = value.unwrap_or("").to_string();
                    },
                "current-version-ranges.cfi-range.end-position" => {
                        request_current_version_ranges_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().cfi_range.end_position = value.unwrap_or("").to_string();
                    },
                "current-version-ranges.cfi-range.start-offset" => {
                        request_current_version_ranges_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().cfi_range.start_offset = value.unwrap_or("").to_string();
                    },
                "current-version-ranges.cfi-range.end-offset" => {
                        request_current_version_ranges_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().cfi_range.end_offset = value.unwrap_or("").to_string();
                    },
                "current-version-ranges.gb-image-range.start-position" => {
                        request_current_version_ranges_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().gb_image_range.start_position = value.unwrap_or("").to_string();
                    },
                "current-version-ranges.gb-image-range.end-position" => {
                        request_current_version_ranges_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().gb_image_range.end_position = value.unwrap_or("").to_string();
                    },
                "current-version-ranges.gb-image-range.start-offset" => {
                        request_current_version_ranges_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().gb_image_range.start_offset = value.unwrap_or("").to_string();
                    },
                "current-version-ranges.gb-image-range.end-offset" => {
                        request_current_version_ranges_init(&mut request);
                        request.current_version_ranges.as_mut().unwrap().gb_image_range.end_offset = value.unwrap_or("").to_string();
                    },
                "after-selected-text" => {
                        request_current_version_ranges_init(&mut request);
                        request.after_selected_text = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_current_version_ranges_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "volume-id" => {
                        request_current_version_ranges_init(&mut request);
                        request.volume_id = Some(value.unwrap_or("").to_string());
                    },
                "layer-summary.limit-type" => {
                        request_layer_summary_init(&mut request);
                        request.layer_summary.as_mut().unwrap().limit_type = value.unwrap_or("").to_string();
                    },
                "layer-summary.remaining-character-count" => {
                        request_layer_summary_init(&mut request);
                        request.layer_summary.as_mut().unwrap().remaining_character_count = arg_from_str(value.unwrap_or("-0"), err, "layer-summary.remaining-character-count", "integer");
                    },
                "layer-summary.allowed-character-count" => {
                        request_layer_summary_init(&mut request);
                        request.layer_summary.as_mut().unwrap().allowed_character_count = arg_from_str(value.unwrap_or("-0"), err, "layer-summary.allowed-character-count", "integer");
                    },
                "selected-text" => {
                        request_layer_summary_init(&mut request);
                        request.selected_text = Some(value.unwrap_or("").to_string());
                    },
                "client-version-ranges.image-cfi-range.start-position" => {
                        request_client_version_ranges_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().image_cfi_range.start_position = value.unwrap_or("").to_string();
                    },
                "client-version-ranges.image-cfi-range.end-position" => {
                        request_client_version_ranges_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().image_cfi_range.end_position = value.unwrap_or("").to_string();
                    },
                "client-version-ranges.image-cfi-range.start-offset" => {
                        request_client_version_ranges_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().image_cfi_range.start_offset = value.unwrap_or("").to_string();
                    },
                "client-version-ranges.image-cfi-range.end-offset" => {
                        request_client_version_ranges_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().image_cfi_range.end_offset = value.unwrap_or("").to_string();
                    },
                "client-version-ranges.gb-text-range.start-position" => {
                        request_client_version_ranges_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().gb_text_range.start_position = value.unwrap_or("").to_string();
                    },
                "client-version-ranges.gb-text-range.end-position" => {
                        request_client_version_ranges_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().gb_text_range.end_position = value.unwrap_or("").to_string();
                    },
                "client-version-ranges.gb-text-range.start-offset" => {
                        request_client_version_ranges_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().gb_text_range.start_offset = value.unwrap_or("").to_string();
                    },
                "client-version-ranges.gb-text-range.end-offset" => {
                        request_client_version_ranges_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().gb_text_range.end_offset = value.unwrap_or("").to_string();
                    },
                "client-version-ranges.content-version" => {
                        request_client_version_ranges_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().content_version = value.unwrap_or("").to_string();
                    },
                "client-version-ranges.cfi-range.start-position" => {
                        request_client_version_ranges_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().cfi_range.start_position = value.unwrap_or("").to_string();
                    },
                "client-version-ranges.cfi-range.end-position" => {
                        request_client_version_ranges_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().cfi_range.end_position = value.unwrap_or("").to_string();
                    },
                "client-version-ranges.cfi-range.start-offset" => {
                        request_client_version_ranges_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().cfi_range.start_offset = value.unwrap_or("").to_string();
                    },
                "client-version-ranges.cfi-range.end-offset" => {
                        request_client_version_ranges_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().cfi_range.end_offset = value.unwrap_or("").to_string();
                    },
                "client-version-ranges.gb-image-range.start-position" => {
                        request_client_version_ranges_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().gb_image_range.start_position = value.unwrap_or("").to_string();
                    },
                "client-version-ranges.gb-image-range.end-position" => {
                        request_client_version_ranges_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().gb_image_range.end_position = value.unwrap_or("").to_string();
                    },
                "client-version-ranges.gb-image-range.start-offset" => {
                        request_client_version_ranges_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().gb_image_range.start_offset = value.unwrap_or("").to_string();
                    },
                "client-version-ranges.gb-image-range.end-offset" => {
                        request_client_version_ranges_init(&mut request);
                        request.client_version_ranges.as_mut().unwrap().gb_image_range.end_offset = value.unwrap_or("").to_string();
                    },
                "layer-id" => {
                        request_client_version_ranges_init(&mut request);
                        request.layer_id = Some(value.unwrap_or("").to_string());
                    },
                "highlight-style" => {
                        request_client_version_ranges_init(&mut request);
                        request.highlight_style = Some(value.unwrap_or("").to_string());
                    },
                "data" => {
                        request_client_version_ranges_init(&mut request);
                        request.data = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_client_version_ranges_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_client_version_ranges_init(&mut request);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _mylibrary_bookshelves_add_volume(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.mylibrary().bookshelves_add_volume(&self.opt.arg_shelf, &self.opt.arg_volume_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "reason" => {
                    call = call.reason(value.unwrap_or(""));
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    None
                }
            }
        }
    }

    fn _mylibrary_bookshelves_clear_volumes(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.mylibrary().bookshelves_clear_volumes(&self.opt.arg_shelf);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "source" => {
                    call = call.source(value.unwrap_or(""));
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    None
                }
            }
        }
    }

    fn _mylibrary_bookshelves_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.mylibrary().bookshelves_get(&self.opt.arg_shelf);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "source" => {
                    call = call.source(value.unwrap_or(""));
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

    fn _mylibrary_bookshelves_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.mylibrary().bookshelves_list();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "source" => {
                    call = call.source(value.unwrap_or(""));
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

    fn _mylibrary_bookshelves_move_volume(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let volume_position: i32 = arg_from_str(&self.opt.arg_volume_position, err, "<volume-position>", "integer");
        let mut call = self.hub.mylibrary().bookshelves_move_volume(&self.opt.arg_shelf, &self.opt.arg_volume_id, volume_position);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "source" => {
                    call = call.source(value.unwrap_or(""));
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    None
                }
            }
        }
    }

    fn _mylibrary_bookshelves_remove_volume(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.mylibrary().bookshelves_remove_volume(&self.opt.arg_shelf, &self.opt.arg_volume_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "reason" => {
                    call = call.reason(value.unwrap_or(""));
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    None
                }
            }
        }
    }

    fn _mylibrary_bookshelves_volumes_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.mylibrary().bookshelves_volumes_list(&self.opt.arg_shelf);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
                },
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "show-preorders" => {
                    call = call.show_preorders(arg_from_str(value.unwrap_or("false"), err, "show-preorders", "boolean"));
                },
                "q" => {
                    call = call.q(value.unwrap_or(""));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "country" => {
                    call = call.country(value.unwrap_or(""));
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

    fn _mylibrary_readingpositions_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.mylibrary().readingpositions_get(&self.opt.arg_volume_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "content-version" => {
                    call = call.content_version(value.unwrap_or(""));
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

    fn _mylibrary_readingpositions_set_position(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.mylibrary().readingpositions_set_position(&self.opt.arg_volume_id, &self.opt.arg_timestamp, &self.opt.arg_position);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "device-cookie" => {
                    call = call.device_cookie(value.unwrap_or(""));
                },
                "content-version" => {
                    call = call.content_version(value.unwrap_or(""));
                },
                "action" => {
                    call = call.action(value.unwrap_or(""));
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    None
                }
            }
        }
    }

    fn _onboarding_list_categories(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.onboarding().list_categories();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "locale" => {
                    call = call.locale(value.unwrap_or(""));
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

    fn _onboarding_list_category_volumes(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.onboarding().list_category_volumes();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
                },
                "locale" => {
                    call = call.locale(value.unwrap_or(""));
                },
                "category-id" => {
                    call = call.add_category_id(value.unwrap_or(""));
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

    fn _promooffer_accept(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.promooffer().accept();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "volume-id" => {
                    call = call.volume_id(value.unwrap_or(""));
                },
                "serial" => {
                    call = call.serial(value.unwrap_or(""));
                },
                "product" => {
                    call = call.product(value.unwrap_or(""));
                },
                "offer-id" => {
                    call = call.offer_id(value.unwrap_or(""));
                },
                "model" => {
                    call = call.model(value.unwrap_or(""));
                },
                "manufacturer" => {
                    call = call.manufacturer(value.unwrap_or(""));
                },
                "device" => {
                    call = call.device(value.unwrap_or(""));
                },
                "android-id" => {
                    call = call.android_id(value.unwrap_or(""));
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    None
                }
            }
        }
    }

    fn _promooffer_dismiss(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.promooffer().dismiss();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "serial" => {
                    call = call.serial(value.unwrap_or(""));
                },
                "product" => {
                    call = call.product(value.unwrap_or(""));
                },
                "offer-id" => {
                    call = call.offer_id(value.unwrap_or(""));
                },
                "model" => {
                    call = call.model(value.unwrap_or(""));
                },
                "manufacturer" => {
                    call = call.manufacturer(value.unwrap_or(""));
                },
                "device" => {
                    call = call.device(value.unwrap_or(""));
                },
                "android-id" => {
                    call = call.android_id(value.unwrap_or(""));
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    None
                }
            }
        }
    }

    fn _promooffer_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.promooffer().get();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "serial" => {
                    call = call.serial(value.unwrap_or(""));
                },
                "product" => {
                    call = call.product(value.unwrap_or(""));
                },
                "model" => {
                    call = call.model(value.unwrap_or(""));
                },
                "manufacturer" => {
                    call = call.manufacturer(value.unwrap_or(""));
                },
                "device" => {
                    call = call.device(value.unwrap_or(""));
                },
                "android-id" => {
                    call = call.android_id(value.unwrap_or(""));
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

    fn _volumes_associated_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.volumes().associated_list(&self.opt.arg_volume_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "locale" => {
                    call = call.locale(value.unwrap_or(""));
                },
                "association" => {
                    call = call.association(value.unwrap_or(""));
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

    fn _volumes_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.volumes().get(&self.opt.arg_volume_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "user-library-consistent-read" => {
                    call = call.user_library_consistent_read(arg_from_str(value.unwrap_or("false"), err, "user-library-consistent-read", "boolean"));
                },
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "partner" => {
                    call = call.partner(value.unwrap_or(""));
                },
                "country" => {
                    call = call.country(value.unwrap_or(""));
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

    fn _volumes_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.volumes().list(&self.opt.arg_q);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
                },
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "show-preorders" => {
                    call = call.show_preorders(arg_from_str(value.unwrap_or("false"), err, "show-preorders", "boolean"));
                },
                "projection" => {
                    call = call.projection(value.unwrap_or(""));
                },
                "print-type" => {
                    call = call.print_type(value.unwrap_or(""));
                },
                "partner" => {
                    call = call.partner(value.unwrap_or(""));
                },
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "library-restrict" => {
                    call = call.library_restrict(value.unwrap_or(""));
                },
                "lang-restrict" => {
                    call = call.lang_restrict(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                "download" => {
                    call = call.download(value.unwrap_or(""));
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

    fn _volumes_mybooks_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.volumes().mybooks_list();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
                },
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "processing-state" => {
                    call = call.add_processing_state(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "locale" => {
                    call = call.locale(value.unwrap_or(""));
                },
                "acquire-method" => {
                    call = call.add_acquire_method(value.unwrap_or(""));
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

    fn _volumes_recommended_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.volumes().recommended_list();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "locale" => {
                    call = call.locale(value.unwrap_or(""));
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

    fn _volumes_recommended_rate(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.volumes().recommended_rate(&self.opt.arg_rating, &self.opt.arg_volume_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "locale" => {
                    call = call.locale(value.unwrap_or(""));
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

    fn _volumes_useruploaded_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.volumes().useruploaded_list();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "volume-id" => {
                    call = call.add_volume_id(value.unwrap_or(""));
                },
                "start-index" => {
                    call = call.start_index(arg_from_str(value.unwrap_or("-0"), err, "start-index", "integer"));
                },
                "source" => {
                    call = call.source(value.unwrap_or(""));
                },
                "processing-state" => {
                    call = call.add_processing_state(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "locale" => {
                    call = call.locale(value.unwrap_or(""));
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

    fn _doit(&self, dry_run: bool) -> (Option<api::Error>, Option<InvalidOptionsError>) {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Option<api::Error>;
        let mut err_opt: Option<InvalidOptionsError> = None;

        if self.opt.cmd_bookshelves {
            if self.opt.cmd_get {
                call_result = self._bookshelves_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._bookshelves_list(dry_run, &mut err);
            } else if self.opt.cmd_volumes_list {
                call_result = self._bookshelves_volumes_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_cloudloading {
            if self.opt.cmd_add_book {
                call_result = self._cloudloading_add_book(dry_run, &mut err);
            } else if self.opt.cmd_delete_book {
                call_result = self._cloudloading_delete_book(dry_run, &mut err);
            } else if self.opt.cmd_update_book {
                call_result = self._cloudloading_update_book(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_dictionary {
            if self.opt.cmd_list_offline_metadata {
                call_result = self._dictionary_list_offline_metadata(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_layers {
            if self.opt.cmd_annotation_data_get {
                call_result = self._layers_annotation_data_get(dry_run, &mut err);
            } else if self.opt.cmd_annotation_data_list {
                call_result = self._layers_annotation_data_list(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._layers_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._layers_list(dry_run, &mut err);
            } else if self.opt.cmd_volume_annotations_get {
                call_result = self._layers_volume_annotations_get(dry_run, &mut err);
            } else if self.opt.cmd_volume_annotations_list {
                call_result = self._layers_volume_annotations_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_myconfig {
            if self.opt.cmd_get_user_settings {
                call_result = self._myconfig_get_user_settings(dry_run, &mut err);
            } else if self.opt.cmd_release_download_access {
                call_result = self._myconfig_release_download_access(dry_run, &mut err);
            } else if self.opt.cmd_request_access {
                call_result = self._myconfig_request_access(dry_run, &mut err);
            } else if self.opt.cmd_sync_volume_licenses {
                call_result = self._myconfig_sync_volume_licenses(dry_run, &mut err);
            } else if self.opt.cmd_update_user_settings {
                call_result = self._myconfig_update_user_settings(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_mylibrary {
            if self.opt.cmd_annotations_delete {
                call_result = self._mylibrary_annotations_delete(dry_run, &mut err);
            } else if self.opt.cmd_annotations_insert {
                call_result = self._mylibrary_annotations_insert(dry_run, &mut err);
            } else if self.opt.cmd_annotations_list {
                call_result = self._mylibrary_annotations_list(dry_run, &mut err);
            } else if self.opt.cmd_annotations_summary {
                call_result = self._mylibrary_annotations_summary(dry_run, &mut err);
            } else if self.opt.cmd_annotations_update {
                call_result = self._mylibrary_annotations_update(dry_run, &mut err);
            } else if self.opt.cmd_bookshelves_add_volume {
                call_result = self._mylibrary_bookshelves_add_volume(dry_run, &mut err);
            } else if self.opt.cmd_bookshelves_clear_volumes {
                call_result = self._mylibrary_bookshelves_clear_volumes(dry_run, &mut err);
            } else if self.opt.cmd_bookshelves_get {
                call_result = self._mylibrary_bookshelves_get(dry_run, &mut err);
            } else if self.opt.cmd_bookshelves_list {
                call_result = self._mylibrary_bookshelves_list(dry_run, &mut err);
            } else if self.opt.cmd_bookshelves_move_volume {
                call_result = self._mylibrary_bookshelves_move_volume(dry_run, &mut err);
            } else if self.opt.cmd_bookshelves_remove_volume {
                call_result = self._mylibrary_bookshelves_remove_volume(dry_run, &mut err);
            } else if self.opt.cmd_bookshelves_volumes_list {
                call_result = self._mylibrary_bookshelves_volumes_list(dry_run, &mut err);
            } else if self.opt.cmd_readingpositions_get {
                call_result = self._mylibrary_readingpositions_get(dry_run, &mut err);
            } else if self.opt.cmd_readingpositions_set_position {
                call_result = self._mylibrary_readingpositions_set_position(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_onboarding {
            if self.opt.cmd_list_categories {
                call_result = self._onboarding_list_categories(dry_run, &mut err);
            } else if self.opt.cmd_list_category_volumes {
                call_result = self._onboarding_list_category_volumes(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_promooffer {
            if self.opt.cmd_accept {
                call_result = self._promooffer_accept(dry_run, &mut err);
            } else if self.opt.cmd_dismiss {
                call_result = self._promooffer_dismiss(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._promooffer_get(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_volumes {
            if self.opt.cmd_associated_list {
                call_result = self._volumes_associated_list(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._volumes_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._volumes_list(dry_run, &mut err);
            } else if self.opt.cmd_mybooks_list {
                call_result = self._volumes_mybooks_list(dry_run, &mut err);
            } else if self.opt.cmd_recommended_list {
                call_result = self._volumes_recommended_list(dry_run, &mut err);
            } else if self.opt.cmd_recommended_rate {
                call_result = self._volumes_recommended_rate(dry_run, &mut err);
            } else if self.opt.cmd_useruploaded_list {
                call_result = self._volumes_useruploaded_list(dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "books1-secret.json") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                                      hyper::Client::new(),
                                      JsonTokenStorage {
                                        program_name: "books1",
                                        db_dir: config_dir.clone(),
                                      }, None);
        let engine = Engine {
            opt: opt,
            hub: api::Books::new(hyper::Client::new(), auth),
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