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
extern crate google_blogger3 as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  blogger3 [options] blog-user-infos get <user-id> <blog-id> [-p <v>]... [-o <out>]
  blogger3 [options] blogs get <blog-id> [-p <v>]... [-o <out>]
  blogger3 [options] blogs get-by-url <url> [-p <v>]... [-o <out>]
  blogger3 [options] blogs list-by-user <user-id> [-p <v>]... [-o <out>]
  blogger3 [options] comments approve <blog-id> <post-id> <comment-id> [-p <v>]... [-o <out>]
  blogger3 [options] comments delete <blog-id> <post-id> <comment-id> [-p <v>]...
  blogger3 [options] comments get <blog-id> <post-id> <comment-id> [-p <v>]... [-o <out>]
  blogger3 [options] comments list <blog-id> <post-id> [-p <v>]... [-o <out>]
  blogger3 [options] comments list-by-blog <blog-id> [-p <v>]... [-o <out>]
  blogger3 [options] comments mark-as-spam <blog-id> <post-id> <comment-id> [-p <v>]... [-o <out>]
  blogger3 [options] comments remove-content <blog-id> <post-id> <comment-id> [-p <v>]... [-o <out>]
  blogger3 [options] page-views get <blog-id> [-p <v>]... [-o <out>]
  blogger3 [options] pages delete <blog-id> <page-id> [-p <v>]...
  blogger3 [options] pages get <blog-id> <page-id> [-p <v>]... [-o <out>]
  blogger3 [options] pages insert <blog-id> -r <kv>... [-p <v>]... [-o <out>]
  blogger3 [options] pages list <blog-id> [-p <v>]... [-o <out>]
  blogger3 [options] pages patch <blog-id> <page-id> -r <kv>... [-p <v>]... [-o <out>]
  blogger3 [options] pages publish <blog-id> <page-id> [-p <v>]... [-o <out>]
  blogger3 [options] pages revert <blog-id> <page-id> [-p <v>]... [-o <out>]
  blogger3 [options] pages update <blog-id> <page-id> -r <kv>... [-p <v>]... [-o <out>]
  blogger3 [options] post-user-infos get <user-id> <blog-id> <post-id> [-p <v>]... [-o <out>]
  blogger3 [options] post-user-infos list <user-id> <blog-id> [-p <v>]... [-o <out>]
  blogger3 [options] posts delete <blog-id> <post-id> [-p <v>]...
  blogger3 [options] posts get <blog-id> <post-id> [-p <v>]... [-o <out>]
  blogger3 [options] posts get-by-path <blog-id> <path> [-p <v>]... [-o <out>]
  blogger3 [options] posts insert <blog-id> -r <kv>... [-p <v>]... [-o <out>]
  blogger3 [options] posts list <blog-id> [-p <v>]... [-o <out>]
  blogger3 [options] posts patch <blog-id> <post-id> -r <kv>... [-p <v>]... [-o <out>]
  blogger3 [options] posts publish <blog-id> <post-id> [-p <v>]... [-o <out>]
  blogger3 [options] posts revert <blog-id> <post-id> [-p <v>]... [-o <out>]
  blogger3 [options] posts search <blog-id> <q> [-p <v>]... [-o <out>]
  blogger3 [options] posts update <blog-id> <post-id> -r <kv>... [-p <v>]... [-o <out>]
  blogger3 [options] users get <user-id> [-p <v>]... [-o <out>]
  blogger3 --help

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
    hub: api::Blogger<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _blog_user_infos_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.blog_user_infos().get(&self.opt.arg_user_id, &self.opt.arg_blog_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "max-posts" => {
                    call = call.max_posts(arg_from_str(value.unwrap_or("-0"), err, "max-posts", "integer"));
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

    fn _blogs_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.blogs().get(&self.opt.arg_blog_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "view" => {
                    call = call.view(value.unwrap_or(""));
                },
                "max-posts" => {
                    call = call.max_posts(arg_from_str(value.unwrap_or("-0"), err, "max-posts", "integer"));
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

    fn _blogs_get_by_url(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.blogs().get_by_url(&self.opt.arg_url);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "view" => {
                    call = call.view(value.unwrap_or(""));
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

    fn _blogs_list_by_user(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.blogs().list_by_user(&self.opt.arg_user_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "view" => {
                    call = call.view(value.unwrap_or(""));
                },
                "status" => {
                    call = call.add_status(value.unwrap_or(""));
                },
                "role" => {
                    call = call.add_role(value.unwrap_or(""));
                },
                "fetch-user-info" => {
                    call = call.fetch_user_info(arg_from_str(value.unwrap_or("false"), err, "fetch-user-info", "boolean"));
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

    fn _comments_approve(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.comments().approve(&self.opt.arg_blog_id, &self.opt.arg_post_id, &self.opt.arg_comment_id);
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

    fn _comments_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.comments().delete(&self.opt.arg_blog_id, &self.opt.arg_post_id, &self.opt.arg_comment_id);
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

    fn _comments_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.comments().get(&self.opt.arg_blog_id, &self.opt.arg_post_id, &self.opt.arg_comment_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "view" => {
                    call = call.view(value.unwrap_or(""));
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

    fn _comments_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.comments().list(&self.opt.arg_blog_id, &self.opt.arg_post_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "view" => {
                    call = call.view(value.unwrap_or(""));
                },
                "status" => {
                    call = call.add_status(value.unwrap_or(""));
                },
                "start-date" => {
                    call = call.start_date(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "fetch-bodies" => {
                    call = call.fetch_bodies(arg_from_str(value.unwrap_or("false"), err, "fetch-bodies", "boolean"));
                },
                "end-date" => {
                    call = call.end_date(value.unwrap_or(""));
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

    fn _comments_list_by_blog(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.comments().list_by_blog(&self.opt.arg_blog_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "status" => {
                    call = call.add_status(value.unwrap_or(""));
                },
                "start-date" => {
                    call = call.start_date(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "fetch-bodies" => {
                    call = call.fetch_bodies(arg_from_str(value.unwrap_or("false"), err, "fetch-bodies", "boolean"));
                },
                "end-date" => {
                    call = call.end_date(value.unwrap_or(""));
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

    fn _comments_mark_as_spam(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.comments().mark_as_spam(&self.opt.arg_blog_id, &self.opt.arg_post_id, &self.opt.arg_comment_id);
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

    fn _comments_remove_content(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.comments().remove_content(&self.opt.arg_blog_id, &self.opt.arg_post_id, &self.opt.arg_comment_id);
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

    fn _page_views_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.page_views().get(&self.opt.arg_blog_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "range" => {
                    call = call.add_range(value.unwrap_or(""));
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

    fn _pages_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.pages().delete(&self.opt.arg_blog_id, &self.opt.arg_page_id);
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

    fn _pages_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.pages().get(&self.opt.arg_blog_id, &self.opt.arg_page_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "view" => {
                    call = call.view(value.unwrap_or(""));
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

    fn _pages_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Page = Default::default();
        let mut call = self.hub.pages().insert(&request, &self.opt.arg_blog_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "is-draft" => {
                    call = call.is_draft(arg_from_str(value.unwrap_or("false"), err, "is-draft", "boolean"));
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
            fn request_author_init(request: &mut api::Page) {
                if request.author.is_none() {
                    request.author = Some(Default::default());
                }
            }
            
            fn request_blog_init(request: &mut api::Page) {
                if request.blog.is_none() {
                    request.blog = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "status" => {
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "content" => {
                        request.content = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "author.url" => {
                        request_author_init(&mut request);
                        request.author.as_mut().unwrap().url = value.unwrap_or("").to_string();
                    },
                "author.image.url" => {
                        request_author_init(&mut request);
                        request.author.as_mut().unwrap().image.url = value.unwrap_or("").to_string();
                    },
                "author.display-name" => {
                        request_author_init(&mut request);
                        request.author.as_mut().unwrap().display_name = value.unwrap_or("").to_string();
                    },
                "author.id" => {
                        request_author_init(&mut request);
                        request.author.as_mut().unwrap().id = value.unwrap_or("").to_string();
                    },
                "url" => {
                        request_author_init(&mut request);
                        request.url = Some(value.unwrap_or("").to_string());
                    },
                "title" => {
                        request_author_init(&mut request);
                        request.title = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request_author_init(&mut request);
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "blog.id" => {
                        request_blog_init(&mut request);
                        request.blog.as_mut().unwrap().id = value.unwrap_or("").to_string();
                    },
                "etag" => {
                        request_blog_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "published" => {
                        request_blog_init(&mut request);
                        request.published = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_blog_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_blog_init(&mut request);
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

    fn _pages_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.pages().list(&self.opt.arg_blog_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "view" => {
                    call = call.view(value.unwrap_or(""));
                },
                "status" => {
                    call = call.add_status(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "fetch-bodies" => {
                    call = call.fetch_bodies(arg_from_str(value.unwrap_or("false"), err, "fetch-bodies", "boolean"));
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

    fn _pages_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Page = Default::default();
        let mut call = self.hub.pages().patch(&request, &self.opt.arg_blog_id, &self.opt.arg_page_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "revert" => {
                    call = call.revert(arg_from_str(value.unwrap_or("false"), err, "revert", "boolean"));
                },
                "publish" => {
                    call = call.publish(arg_from_str(value.unwrap_or("false"), err, "publish", "boolean"));
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
            fn request_author_init(request: &mut api::Page) {
                if request.author.is_none() {
                    request.author = Some(Default::default());
                }
            }
            
            fn request_blog_init(request: &mut api::Page) {
                if request.blog.is_none() {
                    request.blog = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "status" => {
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "content" => {
                        request.content = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "author.url" => {
                        request_author_init(&mut request);
                        request.author.as_mut().unwrap().url = value.unwrap_or("").to_string();
                    },
                "author.image.url" => {
                        request_author_init(&mut request);
                        request.author.as_mut().unwrap().image.url = value.unwrap_or("").to_string();
                    },
                "author.display-name" => {
                        request_author_init(&mut request);
                        request.author.as_mut().unwrap().display_name = value.unwrap_or("").to_string();
                    },
                "author.id" => {
                        request_author_init(&mut request);
                        request.author.as_mut().unwrap().id = value.unwrap_or("").to_string();
                    },
                "url" => {
                        request_author_init(&mut request);
                        request.url = Some(value.unwrap_or("").to_string());
                    },
                "title" => {
                        request_author_init(&mut request);
                        request.title = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request_author_init(&mut request);
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "blog.id" => {
                        request_blog_init(&mut request);
                        request.blog.as_mut().unwrap().id = value.unwrap_or("").to_string();
                    },
                "etag" => {
                        request_blog_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "published" => {
                        request_blog_init(&mut request);
                        request.published = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_blog_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_blog_init(&mut request);
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

    fn _pages_publish(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.pages().publish(&self.opt.arg_blog_id, &self.opt.arg_page_id);
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

    fn _pages_revert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.pages().revert(&self.opt.arg_blog_id, &self.opt.arg_page_id);
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

    fn _pages_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Page = Default::default();
        let mut call = self.hub.pages().update(&request, &self.opt.arg_blog_id, &self.opt.arg_page_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "revert" => {
                    call = call.revert(arg_from_str(value.unwrap_or("false"), err, "revert", "boolean"));
                },
                "publish" => {
                    call = call.publish(arg_from_str(value.unwrap_or("false"), err, "publish", "boolean"));
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
            fn request_author_init(request: &mut api::Page) {
                if request.author.is_none() {
                    request.author = Some(Default::default());
                }
            }
            
            fn request_blog_init(request: &mut api::Page) {
                if request.blog.is_none() {
                    request.blog = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "status" => {
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "content" => {
                        request.content = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "author.url" => {
                        request_author_init(&mut request);
                        request.author.as_mut().unwrap().url = value.unwrap_or("").to_string();
                    },
                "author.image.url" => {
                        request_author_init(&mut request);
                        request.author.as_mut().unwrap().image.url = value.unwrap_or("").to_string();
                    },
                "author.display-name" => {
                        request_author_init(&mut request);
                        request.author.as_mut().unwrap().display_name = value.unwrap_or("").to_string();
                    },
                "author.id" => {
                        request_author_init(&mut request);
                        request.author.as_mut().unwrap().id = value.unwrap_or("").to_string();
                    },
                "url" => {
                        request_author_init(&mut request);
                        request.url = Some(value.unwrap_or("").to_string());
                    },
                "title" => {
                        request_author_init(&mut request);
                        request.title = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request_author_init(&mut request);
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "blog.id" => {
                        request_blog_init(&mut request);
                        request.blog.as_mut().unwrap().id = value.unwrap_or("").to_string();
                    },
                "etag" => {
                        request_blog_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "published" => {
                        request_blog_init(&mut request);
                        request.published = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_blog_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_blog_init(&mut request);
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

    fn _post_user_infos_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.post_user_infos().get(&self.opt.arg_user_id, &self.opt.arg_blog_id, &self.opt.arg_post_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "max-comments" => {
                    call = call.max_comments(arg_from_str(value.unwrap_or("-0"), err, "max-comments", "integer"));
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

    fn _post_user_infos_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.post_user_infos().list(&self.opt.arg_user_id, &self.opt.arg_blog_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "view" => {
                    call = call.view(value.unwrap_or(""));
                },
                "status" => {
                    call = call.add_status(value.unwrap_or(""));
                },
                "start-date" => {
                    call = call.start_date(value.unwrap_or(""));
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
                "labels" => {
                    call = call.labels(value.unwrap_or(""));
                },
                "fetch-bodies" => {
                    call = call.fetch_bodies(arg_from_str(value.unwrap_or("false"), err, "fetch-bodies", "boolean"));
                },
                "end-date" => {
                    call = call.end_date(value.unwrap_or(""));
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

    fn _posts_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.posts().delete(&self.opt.arg_blog_id, &self.opt.arg_post_id);
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

    fn _posts_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.posts().get(&self.opt.arg_blog_id, &self.opt.arg_post_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "view" => {
                    call = call.view(value.unwrap_or(""));
                },
                "max-comments" => {
                    call = call.max_comments(arg_from_str(value.unwrap_or("-0"), err, "max-comments", "integer"));
                },
                "fetch-images" => {
                    call = call.fetch_images(arg_from_str(value.unwrap_or("false"), err, "fetch-images", "boolean"));
                },
                "fetch-body" => {
                    call = call.fetch_body(arg_from_str(value.unwrap_or("false"), err, "fetch-body", "boolean"));
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

    fn _posts_get_by_path(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.posts().get_by_path(&self.opt.arg_blog_id, &self.opt.arg_path);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "view" => {
                    call = call.view(value.unwrap_or(""));
                },
                "max-comments" => {
                    call = call.max_comments(arg_from_str(value.unwrap_or("-0"), err, "max-comments", "integer"));
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

    fn _posts_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Post = Default::default();
        let mut call = self.hub.posts().insert(&request, &self.opt.arg_blog_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "is-draft" => {
                    call = call.is_draft(arg_from_str(value.unwrap_or("false"), err, "is-draft", "boolean"));
                },
                "fetch-images" => {
                    call = call.fetch_images(arg_from_str(value.unwrap_or("false"), err, "fetch-images", "boolean"));
                },
                "fetch-body" => {
                    call = call.fetch_body(arg_from_str(value.unwrap_or("false"), err, "fetch-body", "boolean"));
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
            fn request_author_init(request: &mut api::Post) {
                if request.author.is_none() {
                    request.author = Some(Default::default());
                }
            }
            
            fn request_blog_init(request: &mut api::Post) {
                if request.blog.is_none() {
                    request.blog = Some(Default::default());
                }
            }
            
            fn request_location_init(request: &mut api::Post) {
                if request.location.is_none() {
                    request.location = Some(Default::default());
                }
            }
            
            fn request_replies_init(request: &mut api::Post) {
                if request.replies.is_none() {
                    request.replies = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "status" => {
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "content" => {
                        request.content = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "title-link" => {
                        request.title_link = Some(value.unwrap_or("").to_string());
                    },
                "author.url" => {
                        request_author_init(&mut request);
                        request.author.as_mut().unwrap().url = value.unwrap_or("").to_string();
                    },
                "author.image.url" => {
                        request_author_init(&mut request);
                        request.author.as_mut().unwrap().image.url = value.unwrap_or("").to_string();
                    },
                "author.display-name" => {
                        request_author_init(&mut request);
                        request.author.as_mut().unwrap().display_name = value.unwrap_or("").to_string();
                    },
                "author.id" => {
                        request_author_init(&mut request);
                        request.author.as_mut().unwrap().id = value.unwrap_or("").to_string();
                    },
                "url" => {
                        request_author_init(&mut request);
                        request.url = Some(value.unwrap_or("").to_string());
                    },
                "reader-comments" => {
                        request_author_init(&mut request);
                        request.reader_comments = Some(value.unwrap_or("").to_string());
                    },
                "labels" => {
                        request_author_init(&mut request);
                        if request.labels.is_none() {
                            request.labels = Some(Default::default());
                        }
                        request.labels.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "custom-meta-data" => {
                        request_author_init(&mut request);
                        request.custom_meta_data = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_author_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "blog.id" => {
                        request_blog_init(&mut request);
                        request.blog.as_mut().unwrap().id = value.unwrap_or("").to_string();
                    },
                "etag" => {
                        request_blog_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "location.lat" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().lat = arg_from_str(value.unwrap_or("0.0"), err, "location.lat", "number");
                    },
                "location.lng" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().lng = arg_from_str(value.unwrap_or("0.0"), err, "location.lng", "number");
                    },
                "location.span" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().span = value.unwrap_or("").to_string();
                    },
                "location.name" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().name = value.unwrap_or("").to_string();
                    },
                "replies.total-items" => {
                        request_replies_init(&mut request);
                        request.replies.as_mut().unwrap().total_items = value.unwrap_or("").to_string();
                    },
                "replies.self-link" => {
                        request_replies_init(&mut request);
                        request.replies.as_mut().unwrap().self_link = value.unwrap_or("").to_string();
                    },
                "title" => {
                        request_replies_init(&mut request);
                        request.title = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_replies_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_replies_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "published" => {
                        request_replies_init(&mut request);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _posts_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.posts().list(&self.opt.arg_blog_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "view" => {
                    call = call.view(value.unwrap_or(""));
                },
                "status" => {
                    call = call.add_status(value.unwrap_or(""));
                },
                "start-date" => {
                    call = call.start_date(value.unwrap_or(""));
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
                "labels" => {
                    call = call.labels(value.unwrap_or(""));
                },
                "fetch-images" => {
                    call = call.fetch_images(arg_from_str(value.unwrap_or("false"), err, "fetch-images", "boolean"));
                },
                "fetch-bodies" => {
                    call = call.fetch_bodies(arg_from_str(value.unwrap_or("false"), err, "fetch-bodies", "boolean"));
                },
                "end-date" => {
                    call = call.end_date(value.unwrap_or(""));
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

    fn _posts_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Post = Default::default();
        let mut call = self.hub.posts().patch(&request, &self.opt.arg_blog_id, &self.opt.arg_post_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "revert" => {
                    call = call.revert(arg_from_str(value.unwrap_or("false"), err, "revert", "boolean"));
                },
                "publish" => {
                    call = call.publish(arg_from_str(value.unwrap_or("false"), err, "publish", "boolean"));
                },
                "max-comments" => {
                    call = call.max_comments(arg_from_str(value.unwrap_or("-0"), err, "max-comments", "integer"));
                },
                "fetch-images" => {
                    call = call.fetch_images(arg_from_str(value.unwrap_or("false"), err, "fetch-images", "boolean"));
                },
                "fetch-body" => {
                    call = call.fetch_body(arg_from_str(value.unwrap_or("false"), err, "fetch-body", "boolean"));
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
            fn request_author_init(request: &mut api::Post) {
                if request.author.is_none() {
                    request.author = Some(Default::default());
                }
            }
            
            fn request_blog_init(request: &mut api::Post) {
                if request.blog.is_none() {
                    request.blog = Some(Default::default());
                }
            }
            
            fn request_location_init(request: &mut api::Post) {
                if request.location.is_none() {
                    request.location = Some(Default::default());
                }
            }
            
            fn request_replies_init(request: &mut api::Post) {
                if request.replies.is_none() {
                    request.replies = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "status" => {
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "content" => {
                        request.content = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "title-link" => {
                        request.title_link = Some(value.unwrap_or("").to_string());
                    },
                "author.url" => {
                        request_author_init(&mut request);
                        request.author.as_mut().unwrap().url = value.unwrap_or("").to_string();
                    },
                "author.image.url" => {
                        request_author_init(&mut request);
                        request.author.as_mut().unwrap().image.url = value.unwrap_or("").to_string();
                    },
                "author.display-name" => {
                        request_author_init(&mut request);
                        request.author.as_mut().unwrap().display_name = value.unwrap_or("").to_string();
                    },
                "author.id" => {
                        request_author_init(&mut request);
                        request.author.as_mut().unwrap().id = value.unwrap_or("").to_string();
                    },
                "url" => {
                        request_author_init(&mut request);
                        request.url = Some(value.unwrap_or("").to_string());
                    },
                "reader-comments" => {
                        request_author_init(&mut request);
                        request.reader_comments = Some(value.unwrap_or("").to_string());
                    },
                "labels" => {
                        request_author_init(&mut request);
                        if request.labels.is_none() {
                            request.labels = Some(Default::default());
                        }
                        request.labels.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "custom-meta-data" => {
                        request_author_init(&mut request);
                        request.custom_meta_data = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_author_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "blog.id" => {
                        request_blog_init(&mut request);
                        request.blog.as_mut().unwrap().id = value.unwrap_or("").to_string();
                    },
                "etag" => {
                        request_blog_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "location.lat" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().lat = arg_from_str(value.unwrap_or("0.0"), err, "location.lat", "number");
                    },
                "location.lng" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().lng = arg_from_str(value.unwrap_or("0.0"), err, "location.lng", "number");
                    },
                "location.span" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().span = value.unwrap_or("").to_string();
                    },
                "location.name" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().name = value.unwrap_or("").to_string();
                    },
                "replies.total-items" => {
                        request_replies_init(&mut request);
                        request.replies.as_mut().unwrap().total_items = value.unwrap_or("").to_string();
                    },
                "replies.self-link" => {
                        request_replies_init(&mut request);
                        request.replies.as_mut().unwrap().self_link = value.unwrap_or("").to_string();
                    },
                "title" => {
                        request_replies_init(&mut request);
                        request.title = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_replies_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_replies_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "published" => {
                        request_replies_init(&mut request);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _posts_publish(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.posts().publish(&self.opt.arg_blog_id, &self.opt.arg_post_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "publish-date" => {
                    call = call.publish_date(value.unwrap_or(""));
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

    fn _posts_revert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.posts().revert(&self.opt.arg_blog_id, &self.opt.arg_post_id);
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

    fn _posts_search(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.posts().search(&self.opt.arg_blog_id, &self.opt.arg_q);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "order-by" => {
                    call = call.order_by(value.unwrap_or(""));
                },
                "fetch-bodies" => {
                    call = call.fetch_bodies(arg_from_str(value.unwrap_or("false"), err, "fetch-bodies", "boolean"));
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

    fn _posts_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Post = Default::default();
        let mut call = self.hub.posts().update(&request, &self.opt.arg_blog_id, &self.opt.arg_post_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "revert" => {
                    call = call.revert(arg_from_str(value.unwrap_or("false"), err, "revert", "boolean"));
                },
                "publish" => {
                    call = call.publish(arg_from_str(value.unwrap_or("false"), err, "publish", "boolean"));
                },
                "max-comments" => {
                    call = call.max_comments(arg_from_str(value.unwrap_or("-0"), err, "max-comments", "integer"));
                },
                "fetch-images" => {
                    call = call.fetch_images(arg_from_str(value.unwrap_or("false"), err, "fetch-images", "boolean"));
                },
                "fetch-body" => {
                    call = call.fetch_body(arg_from_str(value.unwrap_or("false"), err, "fetch-body", "boolean"));
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
            fn request_author_init(request: &mut api::Post) {
                if request.author.is_none() {
                    request.author = Some(Default::default());
                }
            }
            
            fn request_blog_init(request: &mut api::Post) {
                if request.blog.is_none() {
                    request.blog = Some(Default::default());
                }
            }
            
            fn request_location_init(request: &mut api::Post) {
                if request.location.is_none() {
                    request.location = Some(Default::default());
                }
            }
            
            fn request_replies_init(request: &mut api::Post) {
                if request.replies.is_none() {
                    request.replies = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "status" => {
                        request.status = Some(value.unwrap_or("").to_string());
                    },
                "content" => {
                        request.content = Some(value.unwrap_or("").to_string());
                    },
                "updated" => {
                        request.updated = Some(value.unwrap_or("").to_string());
                    },
                "title-link" => {
                        request.title_link = Some(value.unwrap_or("").to_string());
                    },
                "author.url" => {
                        request_author_init(&mut request);
                        request.author.as_mut().unwrap().url = value.unwrap_or("").to_string();
                    },
                "author.image.url" => {
                        request_author_init(&mut request);
                        request.author.as_mut().unwrap().image.url = value.unwrap_or("").to_string();
                    },
                "author.display-name" => {
                        request_author_init(&mut request);
                        request.author.as_mut().unwrap().display_name = value.unwrap_or("").to_string();
                    },
                "author.id" => {
                        request_author_init(&mut request);
                        request.author.as_mut().unwrap().id = value.unwrap_or("").to_string();
                    },
                "url" => {
                        request_author_init(&mut request);
                        request.url = Some(value.unwrap_or("").to_string());
                    },
                "reader-comments" => {
                        request_author_init(&mut request);
                        request.reader_comments = Some(value.unwrap_or("").to_string());
                    },
                "labels" => {
                        request_author_init(&mut request);
                        if request.labels.is_none() {
                            request.labels = Some(Default::default());
                        }
                        request.labels.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "custom-meta-data" => {
                        request_author_init(&mut request);
                        request.custom_meta_data = Some(value.unwrap_or("").to_string());
                    },
                "kind" => {
                        request_author_init(&mut request);
                        request.kind = Some(value.unwrap_or("").to_string());
                    },
                "blog.id" => {
                        request_blog_init(&mut request);
                        request.blog.as_mut().unwrap().id = value.unwrap_or("").to_string();
                    },
                "etag" => {
                        request_blog_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "location.lat" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().lat = arg_from_str(value.unwrap_or("0.0"), err, "location.lat", "number");
                    },
                "location.lng" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().lng = arg_from_str(value.unwrap_or("0.0"), err, "location.lng", "number");
                    },
                "location.span" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().span = value.unwrap_or("").to_string();
                    },
                "location.name" => {
                        request_location_init(&mut request);
                        request.location.as_mut().unwrap().name = value.unwrap_or("").to_string();
                    },
                "replies.total-items" => {
                        request_replies_init(&mut request);
                        request.replies.as_mut().unwrap().total_items = value.unwrap_or("").to_string();
                    },
                "replies.self-link" => {
                        request_replies_init(&mut request);
                        request.replies.as_mut().unwrap().self_link = value.unwrap_or("").to_string();
                    },
                "title" => {
                        request_replies_init(&mut request);
                        request.title = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_replies_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "self-link" => {
                        request_replies_init(&mut request);
                        request.self_link = Some(value.unwrap_or("").to_string());
                    },
                "published" => {
                        request_replies_init(&mut request);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _users_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.users().get(&self.opt.arg_user_id);
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

        if self.opt.cmd_blog_user_infos {
            if self.opt.cmd_get {
                call_result = self._blog_user_infos_get(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_blogs {
            if self.opt.cmd_get {
                call_result = self._blogs_get(dry_run, &mut err);
            } else if self.opt.cmd_get_by_url {
                call_result = self._blogs_get_by_url(dry_run, &mut err);
            } else if self.opt.cmd_list_by_user {
                call_result = self._blogs_list_by_user(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_comments {
            if self.opt.cmd_approve {
                call_result = self._comments_approve(dry_run, &mut err);
            } else if self.opt.cmd_delete {
                call_result = self._comments_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._comments_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._comments_list(dry_run, &mut err);
            } else if self.opt.cmd_list_by_blog {
                call_result = self._comments_list_by_blog(dry_run, &mut err);
            } else if self.opt.cmd_mark_as_spam {
                call_result = self._comments_mark_as_spam(dry_run, &mut err);
            } else if self.opt.cmd_remove_content {
                call_result = self._comments_remove_content(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_page_views {
            if self.opt.cmd_get {
                call_result = self._page_views_get(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_pages {
            if self.opt.cmd_delete {
                call_result = self._pages_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._pages_get(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._pages_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._pages_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._pages_patch(dry_run, &mut err);
            } else if self.opt.cmd_publish {
                call_result = self._pages_publish(dry_run, &mut err);
            } else if self.opt.cmd_revert {
                call_result = self._pages_revert(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._pages_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_post_user_infos {
            if self.opt.cmd_get {
                call_result = self._post_user_infos_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._post_user_infos_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_posts {
            if self.opt.cmd_delete {
                call_result = self._posts_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._posts_get(dry_run, &mut err);
            } else if self.opt.cmd_get_by_path {
                call_result = self._posts_get_by_path(dry_run, &mut err);
            } else if self.opt.cmd_insert {
                call_result = self._posts_insert(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._posts_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._posts_patch(dry_run, &mut err);
            } else if self.opt.cmd_publish {
                call_result = self._posts_publish(dry_run, &mut err);
            } else if self.opt.cmd_revert {
                call_result = self._posts_revert(dry_run, &mut err);
            } else if self.opt.cmd_search {
                call_result = self._posts_search(dry_run, &mut err);
            } else if self.opt.cmd_update {
                call_result = self._posts_update(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_users {
            if self.opt.cmd_get {
                call_result = self._users_get(dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "blogger3-secret.json") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                                      hyper::Client::new(),
                                      JsonTokenStorage {
                                        program_name: "blogger3",
                                        db_dir: config_dir.clone(),
                                      }, None);
        let engine = Engine {
            opt: opt,
            hub: api::Blogger::new(hyper::Client::new(), auth),
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