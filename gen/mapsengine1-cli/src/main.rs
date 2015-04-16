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
extern crate google_mapsengine1 as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  mapsengine1 [options] assets get <id> [-p <v>]... [-o <out>]
  mapsengine1 [options] assets list [-p <v>]... [-o <out>]
  mapsengine1 [options] assets parents-list <id> [-p <v>]... [-o <out>]
  mapsengine1 [options] assets permissions-list <id> [-p <v>]... [-o <out>]
  mapsengine1 [options] layers cancel-processing <id> [-p <v>]... [-o <out>]
  mapsengine1 [options] layers create -r <kv>... [-p <v>]... [-o <out>]
  mapsengine1 [options] layers delete <id> [-p <v>]...
  mapsengine1 [options] layers get <id> [-p <v>]... [-o <out>]
  mapsengine1 [options] layers get-published <id> [-p <v>]... [-o <out>]
  mapsengine1 [options] layers list [-p <v>]... [-o <out>]
  mapsengine1 [options] layers list-published [-p <v>]... [-o <out>]
  mapsengine1 [options] layers parents-list <id> [-p <v>]... [-o <out>]
  mapsengine1 [options] layers patch <id> -r <kv>... [-p <v>]...
  mapsengine1 [options] layers permissions-batch-delete <id> -r <kv>... [-p <v>]... [-o <out>]
  mapsengine1 [options] layers permissions-batch-update <id> -r <kv>... [-p <v>]... [-o <out>]
  mapsengine1 [options] layers permissions-list <id> [-p <v>]... [-o <out>]
  mapsengine1 [options] layers process <id> [-p <v>]... [-o <out>]
  mapsengine1 [options] layers publish <id> [-p <v>]... [-o <out>]
  mapsengine1 [options] layers unpublish <id> [-p <v>]... [-o <out>]
  mapsengine1 [options] maps create -r <kv>... [-p <v>]... [-o <out>]
  mapsengine1 [options] maps delete <id> [-p <v>]...
  mapsengine1 [options] maps get <id> [-p <v>]... [-o <out>]
  mapsengine1 [options] maps get-published <id> [-p <v>]... [-o <out>]
  mapsengine1 [options] maps list [-p <v>]... [-o <out>]
  mapsengine1 [options] maps list-published [-p <v>]... [-o <out>]
  mapsengine1 [options] maps patch <id> -r <kv>... [-p <v>]...
  mapsengine1 [options] maps permissions-batch-delete <id> -r <kv>... [-p <v>]... [-o <out>]
  mapsengine1 [options] maps permissions-batch-update <id> -r <kv>... [-p <v>]... [-o <out>]
  mapsengine1 [options] maps permissions-list <id> [-p <v>]... [-o <out>]
  mapsengine1 [options] maps publish <id> [-p <v>]... [-o <out>]
  mapsengine1 [options] maps unpublish <id> [-p <v>]... [-o <out>]
  mapsengine1 [options] projects icons-create <project-id> -r <kv>... -u (simple|resumable) <file> <mime> [-p <v>]... [-o <out>]
  mapsengine1 [options] projects icons-get <project-id> <id> [-p <v>]... [-o <out>]
  mapsengine1 [options] projects icons-list <project-id> [-p <v>]... [-o <out>]
  mapsengine1 [options] projects list [-p <v>]... [-o <out>]
  mapsengine1 [options] raster-collections cancel-processing <id> [-p <v>]... [-o <out>]
  mapsengine1 [options] raster-collections create -r <kv>... [-p <v>]... [-o <out>]
  mapsengine1 [options] raster-collections delete <id> [-p <v>]...
  mapsengine1 [options] raster-collections get <id> [-p <v>]... [-o <out>]
  mapsengine1 [options] raster-collections list [-p <v>]... [-o <out>]
  mapsengine1 [options] raster-collections parents-list <id> [-p <v>]... [-o <out>]
  mapsengine1 [options] raster-collections patch <id> -r <kv>... [-p <v>]...
  mapsengine1 [options] raster-collections permissions-batch-delete <id> -r <kv>... [-p <v>]... [-o <out>]
  mapsengine1 [options] raster-collections permissions-batch-update <id> -r <kv>... [-p <v>]... [-o <out>]
  mapsengine1 [options] raster-collections permissions-list <id> [-p <v>]... [-o <out>]
  mapsengine1 [options] raster-collections process <id> [-p <v>]... [-o <out>]
  mapsengine1 [options] raster-collections rasters-batch-delete <id> -r <kv>... [-p <v>]... [-o <out>]
  mapsengine1 [options] raster-collections rasters-batch-insert <id> -r <kv>... [-p <v>]... [-o <out>]
  mapsengine1 [options] raster-collections rasters-list <id> [-p <v>]... [-o <out>]
  mapsengine1 [options] rasters delete <id> [-p <v>]...
  mapsengine1 [options] rasters files-insert <id> <filename> -u (simple|resumable) <file> <mime> [-p <v>]...
  mapsengine1 [options] rasters get <id> [-p <v>]... [-o <out>]
  mapsengine1 [options] rasters list <project-id> [-p <v>]... [-o <out>]
  mapsengine1 [options] rasters parents-list <id> [-p <v>]... [-o <out>]
  mapsengine1 [options] rasters patch <id> -r <kv>... [-p <v>]...
  mapsengine1 [options] rasters permissions-batch-delete <id> -r <kv>... [-p <v>]... [-o <out>]
  mapsengine1 [options] rasters permissions-batch-update <id> -r <kv>... [-p <v>]... [-o <out>]
  mapsengine1 [options] rasters permissions-list <id> [-p <v>]... [-o <out>]
  mapsengine1 [options] rasters process <id> [-p <v>]... [-o <out>]
  mapsengine1 [options] rasters upload -r <kv>... [-p <v>]... [-o <out>]
  mapsengine1 [options] tables create -r <kv>... [-p <v>]... [-o <out>]
  mapsengine1 [options] tables delete <id> [-p <v>]...
  mapsengine1 [options] tables features-batch-delete <id> -r <kv>... [-p <v>]...
  mapsengine1 [options] tables features-batch-insert <id> -r <kv>... [-p <v>]...
  mapsengine1 [options] tables features-batch-patch <id> -r <kv>... [-p <v>]...
  mapsengine1 [options] tables features-get <table-id> <id> [-p <v>]... [-o <out>]
  mapsengine1 [options] tables features-list <id> [-p <v>]... [-o <out>]
  mapsengine1 [options] tables files-insert <id> <filename> -u (simple|resumable) <file> <mime> [-p <v>]...
  mapsengine1 [options] tables get <id> [-p <v>]... [-o <out>]
  mapsengine1 [options] tables list [-p <v>]... [-o <out>]
  mapsengine1 [options] tables parents-list <id> [-p <v>]... [-o <out>]
  mapsengine1 [options] tables patch <id> -r <kv>... [-p <v>]...
  mapsengine1 [options] tables permissions-batch-delete <id> -r <kv>... [-p <v>]... [-o <out>]
  mapsengine1 [options] tables permissions-batch-update <id> -r <kv>... [-p <v>]... [-o <out>]
  mapsengine1 [options] tables permissions-list <id> [-p <v>]... [-o <out>]
  mapsengine1 [options] tables process <id> [-p <v>]... [-o <out>]
  mapsengine1 [options] tables upload -r <kv>... [-p <v>]... [-o <out>]
  mapsengine1 --help

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
    hub: api::MapsEngine<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _assets_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.assets().get(&self.opt.arg_id);
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

    fn _assets_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.assets().list();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "type" => {
                    call = call.type_(value.unwrap_or(""));
                },
                "tags" => {
                    call = call.tags(value.unwrap_or(""));
                },
                "search" => {
                    call = call.search(value.unwrap_or(""));
                },
                "role" => {
                    call = call.role(value.unwrap_or(""));
                },
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "modified-before" => {
                    call = call.modified_before(value.unwrap_or(""));
                },
                "modified-after" => {
                    call = call.modified_after(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "creator-email" => {
                    call = call.creator_email(value.unwrap_or(""));
                },
                "created-before" => {
                    call = call.created_before(value.unwrap_or(""));
                },
                "created-after" => {
                    call = call.created_after(value.unwrap_or(""));
                },
                "bbox" => {
                    call = call.bbox(value.unwrap_or(""));
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

    fn _assets_parents_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.assets().parents_list(&self.opt.arg_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _assets_permissions_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.assets().permissions_list(&self.opt.arg_id);
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

    fn _layers_cancel_processing(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.layers().cancel_processing(&self.opt.arg_id);
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

    fn _layers_create(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Layer = Default::default();
        let mut call = self.hub.layers().create(&request);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "process" => {
                    call = call.process(arg_from_str(value.unwrap_or("false"), err, "process", "boolean"));
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
            fn request_style_init(request: &mut api::Layer) {
                if request.style.is_none() {
                    request.style = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "style.feature-info.content" => {
                        request_style_init(&mut request);
                        request.style.as_mut().unwrap().feature_info.content = value.unwrap_or("").to_string();
                    },
                "style.type" => {
                        request_style_init(&mut request);
                        request.style.as_mut().unwrap().type_ = value.unwrap_or("").to_string();
                    },
                "description" => {
                        request_style_init(&mut request);
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "processing-status" => {
                        request_style_init(&mut request);
                        request.processing_status = Some(value.unwrap_or("").to_string());
                    },
                "draft-access-list" => {
                        request_style_init(&mut request);
                        request.draft_access_list = Some(value.unwrap_or("").to_string());
                    },
                "datasource-type" => {
                        request_style_init(&mut request);
                        request.datasource_type = Some(value.unwrap_or("").to_string());
                    },
                "creation-time" => {
                        request_style_init(&mut request);
                        request.creation_time = Some(value.unwrap_or("").to_string());
                    },
                "publishing-status" => {
                        request_style_init(&mut request);
                        request.publishing_status = Some(value.unwrap_or("").to_string());
                    },
                "writers-can-edit-permissions" => {
                        request_style_init(&mut request);
                        request.writers_can_edit_permissions = Some(arg_from_str(value.unwrap_or("false"), err, "writers-can-edit-permissions", "boolean"));
                    },
                "etag" => {
                        request_style_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "creator-email" => {
                        request_style_init(&mut request);
                        request.creator_email = Some(value.unwrap_or("").to_string());
                    },
                "bbox" => {
                        request_style_init(&mut request);
                        if request.bbox.is_none() {
                            request.bbox = Some(Default::default());
                        }
                        request.bbox.as_mut().unwrap().push(arg_from_str(value.unwrap_or("0.0"), err, "bbox", "number"));
                    },
                "layer-type" => {
                        request_style_init(&mut request);
                        request.layer_type = Some(value.unwrap_or("").to_string());
                    },
                "project-id" => {
                        request_style_init(&mut request);
                        request.project_id = Some(value.unwrap_or("").to_string());
                    },
                "last-modifier-email" => {
                        request_style_init(&mut request);
                        request.last_modifier_email = Some(value.unwrap_or("").to_string());
                    },
                "last-modified-time" => {
                        request_style_init(&mut request);
                        request.last_modified_time = Some(value.unwrap_or("").to_string());
                    },
                "published-access-list" => {
                        request_style_init(&mut request);
                        request.published_access_list = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_style_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_style_init(&mut request);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _layers_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.layers().delete(&self.opt.arg_id);
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

    fn _layers_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.layers().get(&self.opt.arg_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "version" => {
                    call = call.version(value.unwrap_or(""));
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

    fn _layers_get_published(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.layers().get_published(&self.opt.arg_id);
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

    fn _layers_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.layers().list();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "tags" => {
                    call = call.tags(value.unwrap_or(""));
                },
                "search" => {
                    call = call.search(value.unwrap_or(""));
                },
                "role" => {
                    call = call.role(value.unwrap_or(""));
                },
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
                },
                "processing-status" => {
                    call = call.processing_status(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "modified-before" => {
                    call = call.modified_before(value.unwrap_or(""));
                },
                "modified-after" => {
                    call = call.modified_after(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "creator-email" => {
                    call = call.creator_email(value.unwrap_or(""));
                },
                "created-before" => {
                    call = call.created_before(value.unwrap_or(""));
                },
                "created-after" => {
                    call = call.created_after(value.unwrap_or(""));
                },
                "bbox" => {
                    call = call.bbox(value.unwrap_or(""));
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

    fn _layers_list_published(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.layers().list_published();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
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

    fn _layers_parents_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.layers().parents_list(&self.opt.arg_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _layers_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Layer = Default::default();
        let mut call = self.hub.layers().patch(&request, &self.opt.arg_id);
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
            fn request_style_init(request: &mut api::Layer) {
                if request.style.is_none() {
                    request.style = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "style.feature-info.content" => {
                        request_style_init(&mut request);
                        request.style.as_mut().unwrap().feature_info.content = value.unwrap_or("").to_string();
                    },
                "style.type" => {
                        request_style_init(&mut request);
                        request.style.as_mut().unwrap().type_ = value.unwrap_or("").to_string();
                    },
                "description" => {
                        request_style_init(&mut request);
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "processing-status" => {
                        request_style_init(&mut request);
                        request.processing_status = Some(value.unwrap_or("").to_string());
                    },
                "draft-access-list" => {
                        request_style_init(&mut request);
                        request.draft_access_list = Some(value.unwrap_or("").to_string());
                    },
                "datasource-type" => {
                        request_style_init(&mut request);
                        request.datasource_type = Some(value.unwrap_or("").to_string());
                    },
                "creation-time" => {
                        request_style_init(&mut request);
                        request.creation_time = Some(value.unwrap_or("").to_string());
                    },
                "publishing-status" => {
                        request_style_init(&mut request);
                        request.publishing_status = Some(value.unwrap_or("").to_string());
                    },
                "writers-can-edit-permissions" => {
                        request_style_init(&mut request);
                        request.writers_can_edit_permissions = Some(arg_from_str(value.unwrap_or("false"), err, "writers-can-edit-permissions", "boolean"));
                    },
                "etag" => {
                        request_style_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "creator-email" => {
                        request_style_init(&mut request);
                        request.creator_email = Some(value.unwrap_or("").to_string());
                    },
                "bbox" => {
                        request_style_init(&mut request);
                        if request.bbox.is_none() {
                            request.bbox = Some(Default::default());
                        }
                        request.bbox.as_mut().unwrap().push(arg_from_str(value.unwrap_or("0.0"), err, "bbox", "number"));
                    },
                "layer-type" => {
                        request_style_init(&mut request);
                        request.layer_type = Some(value.unwrap_or("").to_string());
                    },
                "project-id" => {
                        request_style_init(&mut request);
                        request.project_id = Some(value.unwrap_or("").to_string());
                    },
                "last-modifier-email" => {
                        request_style_init(&mut request);
                        request.last_modifier_email = Some(value.unwrap_or("").to_string());
                    },
                "last-modified-time" => {
                        request_style_init(&mut request);
                        request.last_modified_time = Some(value.unwrap_or("").to_string());
                    },
                "published-access-list" => {
                        request_style_init(&mut request);
                        request.published_access_list = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_style_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_style_init(&mut request);
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

    fn _layers_permissions_batch_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::PermissionsBatchDeleteRequest = Default::default();
        let mut call = self.hub.layers().permissions_batch_delete(&request, &self.opt.arg_id);
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
                "ids" => {
                        if request.ids.is_none() {
                            request.ids = Some(Default::default());
                        }
                        request.ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
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

    fn _layers_permissions_batch_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::PermissionsBatchUpdateRequest = Default::default();
        let mut call = self.hub.layers().permissions_batch_update(&request, &self.opt.arg_id);
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

    fn _layers_permissions_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.layers().permissions_list(&self.opt.arg_id);
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

    fn _layers_process(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.layers().process(&self.opt.arg_id);
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

    fn _layers_publish(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.layers().publish(&self.opt.arg_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "force" => {
                    call = call.force(arg_from_str(value.unwrap_or("false"), err, "force", "boolean"));
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

    fn _layers_unpublish(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.layers().unpublish(&self.opt.arg_id);
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

    fn _maps_create(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Map = Default::default();
        let mut call = self.hub.maps().create(&request);
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
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "processing-status" => {
                        request.processing_status = Some(value.unwrap_or("").to_string());
                    },
                "draft-access-list" => {
                        request.draft_access_list = Some(value.unwrap_or("").to_string());
                    },
                "project-id" => {
                        request.project_id = Some(value.unwrap_or("").to_string());
                    },
                "versions" => {
                        if request.versions.is_none() {
                            request.versions = Some(Default::default());
                        }
                        request.versions.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "creation-time" => {
                        request.creation_time = Some(value.unwrap_or("").to_string());
                    },
                "publishing-status" => {
                        request.publishing_status = Some(value.unwrap_or("").to_string());
                    },
                "writers-can-edit-permissions" => {
                        request.writers_can_edit_permissions = Some(arg_from_str(value.unwrap_or("false"), err, "writers-can-edit-permissions", "boolean"));
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "creator-email" => {
                        request.creator_email = Some(value.unwrap_or("").to_string());
                    },
                "bbox" => {
                        if request.bbox.is_none() {
                            request.bbox = Some(Default::default());
                        }
                        request.bbox.as_mut().unwrap().push(arg_from_str(value.unwrap_or("0.0"), err, "bbox", "number"));
                    },
                "last-modifier-email" => {
                        request.last_modifier_email = Some(value.unwrap_or("").to_string());
                    },
                "last-modified-time" => {
                        request.last_modified_time = Some(value.unwrap_or("").to_string());
                    },
                "published-access-list" => {
                        request.published_access_list = Some(value.unwrap_or("").to_string());
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _maps_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.maps().delete(&self.opt.arg_id);
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

    fn _maps_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.maps().get(&self.opt.arg_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "version" => {
                    call = call.version(value.unwrap_or(""));
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

    fn _maps_get_published(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.maps().get_published(&self.opt.arg_id);
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

    fn _maps_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.maps().list();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "tags" => {
                    call = call.tags(value.unwrap_or(""));
                },
                "search" => {
                    call = call.search(value.unwrap_or(""));
                },
                "role" => {
                    call = call.role(value.unwrap_or(""));
                },
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
                },
                "processing-status" => {
                    call = call.processing_status(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "modified-before" => {
                    call = call.modified_before(value.unwrap_or(""));
                },
                "modified-after" => {
                    call = call.modified_after(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "creator-email" => {
                    call = call.creator_email(value.unwrap_or(""));
                },
                "created-before" => {
                    call = call.created_before(value.unwrap_or(""));
                },
                "created-after" => {
                    call = call.created_after(value.unwrap_or(""));
                },
                "bbox" => {
                    call = call.bbox(value.unwrap_or(""));
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

    fn _maps_list_published(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.maps().list_published();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
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

    fn _maps_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Map = Default::default();
        let mut call = self.hub.maps().patch(&request, &self.opt.arg_id);
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
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "processing-status" => {
                        request.processing_status = Some(value.unwrap_or("").to_string());
                    },
                "draft-access-list" => {
                        request.draft_access_list = Some(value.unwrap_or("").to_string());
                    },
                "project-id" => {
                        request.project_id = Some(value.unwrap_or("").to_string());
                    },
                "versions" => {
                        if request.versions.is_none() {
                            request.versions = Some(Default::default());
                        }
                        request.versions.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "creation-time" => {
                        request.creation_time = Some(value.unwrap_or("").to_string());
                    },
                "publishing-status" => {
                        request.publishing_status = Some(value.unwrap_or("").to_string());
                    },
                "writers-can-edit-permissions" => {
                        request.writers_can_edit_permissions = Some(arg_from_str(value.unwrap_or("false"), err, "writers-can-edit-permissions", "boolean"));
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "creator-email" => {
                        request.creator_email = Some(value.unwrap_or("").to_string());
                    },
                "bbox" => {
                        if request.bbox.is_none() {
                            request.bbox = Some(Default::default());
                        }
                        request.bbox.as_mut().unwrap().push(arg_from_str(value.unwrap_or("0.0"), err, "bbox", "number"));
                    },
                "last-modifier-email" => {
                        request.last_modifier_email = Some(value.unwrap_or("").to_string());
                    },
                "last-modified-time" => {
                        request.last_modified_time = Some(value.unwrap_or("").to_string());
                    },
                "published-access-list" => {
                        request.published_access_list = Some(value.unwrap_or("").to_string());
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

    fn _maps_permissions_batch_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::PermissionsBatchDeleteRequest = Default::default();
        let mut call = self.hub.maps().permissions_batch_delete(&request, &self.opt.arg_id);
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
                "ids" => {
                        if request.ids.is_none() {
                            request.ids = Some(Default::default());
                        }
                        request.ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
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

    fn _maps_permissions_batch_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::PermissionsBatchUpdateRequest = Default::default();
        let mut call = self.hub.maps().permissions_batch_update(&request, &self.opt.arg_id);
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

    fn _maps_permissions_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.maps().permissions_list(&self.opt.arg_id);
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

    fn _maps_publish(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.maps().publish(&self.opt.arg_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "force" => {
                    call = call.force(arg_from_str(value.unwrap_or("false"), err, "force", "boolean"));
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

    fn _maps_unpublish(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.maps().unpublish(&self.opt.arg_id);
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

    fn _projects_icons_create(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Icon = Default::default();
        let mut call = self.hub.projects().icons_create(&request, &self.opt.arg_project_id);
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
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request.name = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _projects_icons_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut download_mode = false;
        let mut call = self.hub.projects().icons_get(&self.opt.arg_project_id, &self.opt.arg_id);
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
                    if key == "alt" && value.unwrap_or("unset") == "media" {
                        download_mode = true;
                    }
                    let map = [
                        ("oauth-token", "oauth_token"),
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
                    if !download_mode {
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    } else {
                    io::copy(&mut response, &mut ostream).unwrap();
                    }
                    None
                }
            }
        }
    }

    fn _projects_icons_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.projects().icons_list(&self.opt.arg_project_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _projects_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.projects().list();
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

    fn _raster_collections_cancel_processing(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.raster_collections().cancel_processing(&self.opt.arg_id);
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

    fn _raster_collections_create(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::RasterCollection = Default::default();
        let mut call = self.hub.raster_collections().create(&request);
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
                "attribution" => {
                        request.attribution = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "processing-status" => {
                        request.processing_status = Some(value.unwrap_or("").to_string());
                    },
                "draft-access-list" => {
                        request.draft_access_list = Some(value.unwrap_or("").to_string());
                    },
                "project-id" => {
                        request.project_id = Some(value.unwrap_or("").to_string());
                    },
                "creation-time" => {
                        request.creation_time = Some(value.unwrap_or("").to_string());
                    },
                "writers-can-edit-permissions" => {
                        request.writers_can_edit_permissions = Some(arg_from_str(value.unwrap_or("false"), err, "writers-can-edit-permissions", "boolean"));
                    },
                "raster-type" => {
                        request.raster_type = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "creator-email" => {
                        request.creator_email = Some(value.unwrap_or("").to_string());
                    },
                "bbox" => {
                        if request.bbox.is_none() {
                            request.bbox = Some(Default::default());
                        }
                        request.bbox.as_mut().unwrap().push(arg_from_str(value.unwrap_or("0.0"), err, "bbox", "number"));
                    },
                "last-modifier-email" => {
                        request.last_modifier_email = Some(value.unwrap_or("").to_string());
                    },
                "last-modified-time" => {
                        request.last_modified_time = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "mosaic" => {
                        request.mosaic = Some(arg_from_str(value.unwrap_or("false"), err, "mosaic", "boolean"));
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _raster_collections_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.raster_collections().delete(&self.opt.arg_id);
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

    fn _raster_collections_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.raster_collections().get(&self.opt.arg_id);
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

    fn _raster_collections_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.raster_collections().list();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "tags" => {
                    call = call.tags(value.unwrap_or(""));
                },
                "search" => {
                    call = call.search(value.unwrap_or(""));
                },
                "role" => {
                    call = call.role(value.unwrap_or(""));
                },
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
                },
                "processing-status" => {
                    call = call.processing_status(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "modified-before" => {
                    call = call.modified_before(value.unwrap_or(""));
                },
                "modified-after" => {
                    call = call.modified_after(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "creator-email" => {
                    call = call.creator_email(value.unwrap_or(""));
                },
                "created-before" => {
                    call = call.created_before(value.unwrap_or(""));
                },
                "created-after" => {
                    call = call.created_after(value.unwrap_or(""));
                },
                "bbox" => {
                    call = call.bbox(value.unwrap_or(""));
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

    fn _raster_collections_parents_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.raster_collections().parents_list(&self.opt.arg_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _raster_collections_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::RasterCollection = Default::default();
        let mut call = self.hub.raster_collections().patch(&request, &self.opt.arg_id);
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
                "attribution" => {
                        request.attribution = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "processing-status" => {
                        request.processing_status = Some(value.unwrap_or("").to_string());
                    },
                "draft-access-list" => {
                        request.draft_access_list = Some(value.unwrap_or("").to_string());
                    },
                "project-id" => {
                        request.project_id = Some(value.unwrap_or("").to_string());
                    },
                "creation-time" => {
                        request.creation_time = Some(value.unwrap_or("").to_string());
                    },
                "writers-can-edit-permissions" => {
                        request.writers_can_edit_permissions = Some(arg_from_str(value.unwrap_or("false"), err, "writers-can-edit-permissions", "boolean"));
                    },
                "raster-type" => {
                        request.raster_type = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "creator-email" => {
                        request.creator_email = Some(value.unwrap_or("").to_string());
                    },
                "bbox" => {
                        if request.bbox.is_none() {
                            request.bbox = Some(Default::default());
                        }
                        request.bbox.as_mut().unwrap().push(arg_from_str(value.unwrap_or("0.0"), err, "bbox", "number"));
                    },
                "last-modifier-email" => {
                        request.last_modifier_email = Some(value.unwrap_or("").to_string());
                    },
                "last-modified-time" => {
                        request.last_modified_time = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "mosaic" => {
                        request.mosaic = Some(arg_from_str(value.unwrap_or("false"), err, "mosaic", "boolean"));
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

    fn _raster_collections_permissions_batch_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::PermissionsBatchDeleteRequest = Default::default();
        let mut call = self.hub.raster_collections().permissions_batch_delete(&request, &self.opt.arg_id);
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
                "ids" => {
                        if request.ids.is_none() {
                            request.ids = Some(Default::default());
                        }
                        request.ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
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

    fn _raster_collections_permissions_batch_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::PermissionsBatchUpdateRequest = Default::default();
        let mut call = self.hub.raster_collections().permissions_batch_update(&request, &self.opt.arg_id);
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

    fn _raster_collections_permissions_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.raster_collections().permissions_list(&self.opt.arg_id);
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

    fn _raster_collections_process(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.raster_collections().process(&self.opt.arg_id);
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

    fn _raster_collections_rasters_batch_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::RasterCollectionsRasterBatchDeleteRequest = Default::default();
        let mut call = self.hub.raster_collections().rasters_batch_delete(&request, &self.opt.arg_id);
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
                "ids" => {
                        if request.ids.is_none() {
                            request.ids = Some(Default::default());
                        }
                        request.ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
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

    fn _raster_collections_rasters_batch_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::RasterCollectionsRastersBatchInsertRequest = Default::default();
        let mut call = self.hub.raster_collections().rasters_batch_insert(&request, &self.opt.arg_id);
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
                "ids" => {
                        if request.ids.is_none() {
                            request.ids = Some(Default::default());
                        }
                        request.ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
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

    fn _raster_collections_rasters_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.raster_collections().rasters_list(&self.opt.arg_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "tags" => {
                    call = call.tags(value.unwrap_or(""));
                },
                "search" => {
                    call = call.search(value.unwrap_or(""));
                },
                "role" => {
                    call = call.role(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "modified-before" => {
                    call = call.modified_before(value.unwrap_or(""));
                },
                "modified-after" => {
                    call = call.modified_after(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "creator-email" => {
                    call = call.creator_email(value.unwrap_or(""));
                },
                "created-before" => {
                    call = call.created_before(value.unwrap_or(""));
                },
                "created-after" => {
                    call = call.created_after(value.unwrap_or(""));
                },
                "bbox" => {
                    call = call.bbox(value.unwrap_or(""));
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

    fn _rasters_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.rasters().delete(&self.opt.arg_id);
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

    fn _rasters_files_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.rasters().files_insert(&self.opt.arg_id, &self.opt.arg_filename);
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
            match match protocol {
                "simple" => call.upload(input_file.unwrap(), mime_type.unwrap()),
                "resumable" => call.upload_resumable(input_file.unwrap(), mime_type.unwrap()),
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

    fn _rasters_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.rasters().get(&self.opt.arg_id);
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

    fn _rasters_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.rasters().list(&self.opt.arg_project_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "tags" => {
                    call = call.tags(value.unwrap_or(""));
                },
                "search" => {
                    call = call.search(value.unwrap_or(""));
                },
                "role" => {
                    call = call.role(value.unwrap_or(""));
                },
                "processing-status" => {
                    call = call.processing_status(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "modified-before" => {
                    call = call.modified_before(value.unwrap_or(""));
                },
                "modified-after" => {
                    call = call.modified_after(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "creator-email" => {
                    call = call.creator_email(value.unwrap_or(""));
                },
                "created-before" => {
                    call = call.created_before(value.unwrap_or(""));
                },
                "created-after" => {
                    call = call.created_after(value.unwrap_or(""));
                },
                "bbox" => {
                    call = call.bbox(value.unwrap_or(""));
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

    fn _rasters_parents_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.rasters().parents_list(&self.opt.arg_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _rasters_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Raster = Default::default();
        let mut call = self.hub.rasters().patch(&request, &self.opt.arg_id);
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
            fn request_acquisition_time_init(request: &mut api::Raster) {
                if request.acquisition_time.is_none() {
                    request.acquisition_time = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "acquisition-time.start" => {
                        request_acquisition_time_init(&mut request);
                        request.acquisition_time.as_mut().unwrap().start = value.unwrap_or("").to_string();
                    },
                "acquisition-time.end" => {
                        request_acquisition_time_init(&mut request);
                        request.acquisition_time.as_mut().unwrap().end = value.unwrap_or("").to_string();
                    },
                "acquisition-time.precision" => {
                        request_acquisition_time_init(&mut request);
                        request.acquisition_time.as_mut().unwrap().precision = value.unwrap_or("").to_string();
                    },
                "attribution" => {
                        request_acquisition_time_init(&mut request);
                        request.attribution = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request_acquisition_time_init(&mut request);
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "processing-status" => {
                        request_acquisition_time_init(&mut request);
                        request.processing_status = Some(value.unwrap_or("").to_string());
                    },
                "draft-access-list" => {
                        request_acquisition_time_init(&mut request);
                        request.draft_access_list = Some(value.unwrap_or("").to_string());
                    },
                "project-id" => {
                        request_acquisition_time_init(&mut request);
                        request.project_id = Some(value.unwrap_or("").to_string());
                    },
                "creation-time" => {
                        request_acquisition_time_init(&mut request);
                        request.creation_time = Some(value.unwrap_or("").to_string());
                    },
                "writers-can-edit-permissions" => {
                        request_acquisition_time_init(&mut request);
                        request.writers_can_edit_permissions = Some(arg_from_str(value.unwrap_or("false"), err, "writers-can-edit-permissions", "boolean"));
                    },
                "mask-type" => {
                        request_acquisition_time_init(&mut request);
                        request.mask_type = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_acquisition_time_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "creator-email" => {
                        request_acquisition_time_init(&mut request);
                        request.creator_email = Some(value.unwrap_or("").to_string());
                    },
                "bbox" => {
                        request_acquisition_time_init(&mut request);
                        if request.bbox.is_none() {
                            request.bbox = Some(Default::default());
                        }
                        request.bbox.as_mut().unwrap().push(arg_from_str(value.unwrap_or("0.0"), err, "bbox", "number"));
                    },
                "last-modifier-email" => {
                        request_acquisition_time_init(&mut request);
                        request.last_modifier_email = Some(value.unwrap_or("").to_string());
                    },
                "last-modified-time" => {
                        request_acquisition_time_init(&mut request);
                        request.last_modified_time = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_acquisition_time_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "raster-type" => {
                        request_acquisition_time_init(&mut request);
                        request.raster_type = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_acquisition_time_init(&mut request);
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

    fn _rasters_permissions_batch_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::PermissionsBatchDeleteRequest = Default::default();
        let mut call = self.hub.rasters().permissions_batch_delete(&request, &self.opt.arg_id);
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
                "ids" => {
                        if request.ids.is_none() {
                            request.ids = Some(Default::default());
                        }
                        request.ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
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

    fn _rasters_permissions_batch_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::PermissionsBatchUpdateRequest = Default::default();
        let mut call = self.hub.rasters().permissions_batch_update(&request, &self.opt.arg_id);
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

    fn _rasters_permissions_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.rasters().permissions_list(&self.opt.arg_id);
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

    fn _rasters_process(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.rasters().process(&self.opt.arg_id);
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

    fn _rasters_upload(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Raster = Default::default();
        let mut call = self.hub.rasters().upload(&request);
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
            fn request_acquisition_time_init(request: &mut api::Raster) {
                if request.acquisition_time.is_none() {
                    request.acquisition_time = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "acquisition-time.start" => {
                        request_acquisition_time_init(&mut request);
                        request.acquisition_time.as_mut().unwrap().start = value.unwrap_or("").to_string();
                    },
                "acquisition-time.end" => {
                        request_acquisition_time_init(&mut request);
                        request.acquisition_time.as_mut().unwrap().end = value.unwrap_or("").to_string();
                    },
                "acquisition-time.precision" => {
                        request_acquisition_time_init(&mut request);
                        request.acquisition_time.as_mut().unwrap().precision = value.unwrap_or("").to_string();
                    },
                "attribution" => {
                        request_acquisition_time_init(&mut request);
                        request.attribution = Some(value.unwrap_or("").to_string());
                    },
                "description" => {
                        request_acquisition_time_init(&mut request);
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "processing-status" => {
                        request_acquisition_time_init(&mut request);
                        request.processing_status = Some(value.unwrap_or("").to_string());
                    },
                "draft-access-list" => {
                        request_acquisition_time_init(&mut request);
                        request.draft_access_list = Some(value.unwrap_or("").to_string());
                    },
                "project-id" => {
                        request_acquisition_time_init(&mut request);
                        request.project_id = Some(value.unwrap_or("").to_string());
                    },
                "creation-time" => {
                        request_acquisition_time_init(&mut request);
                        request.creation_time = Some(value.unwrap_or("").to_string());
                    },
                "writers-can-edit-permissions" => {
                        request_acquisition_time_init(&mut request);
                        request.writers_can_edit_permissions = Some(arg_from_str(value.unwrap_or("false"), err, "writers-can-edit-permissions", "boolean"));
                    },
                "mask-type" => {
                        request_acquisition_time_init(&mut request);
                        request.mask_type = Some(value.unwrap_or("").to_string());
                    },
                "etag" => {
                        request_acquisition_time_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "creator-email" => {
                        request_acquisition_time_init(&mut request);
                        request.creator_email = Some(value.unwrap_or("").to_string());
                    },
                "bbox" => {
                        request_acquisition_time_init(&mut request);
                        if request.bbox.is_none() {
                            request.bbox = Some(Default::default());
                        }
                        request.bbox.as_mut().unwrap().push(arg_from_str(value.unwrap_or("0.0"), err, "bbox", "number"));
                    },
                "last-modifier-email" => {
                        request_acquisition_time_init(&mut request);
                        request.last_modifier_email = Some(value.unwrap_or("").to_string());
                    },
                "last-modified-time" => {
                        request_acquisition_time_init(&mut request);
                        request.last_modified_time = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_acquisition_time_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "raster-type" => {
                        request_acquisition_time_init(&mut request);
                        request.raster_type = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_acquisition_time_init(&mut request);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _tables_create(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Table = Default::default();
        let mut call = self.hub.tables().create(&request);
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
            fn request_schema_init(request: &mut api::Table) {
                if request.schema.is_none() {
                    request.schema = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "schema.primary-geometry" => {
                        request_schema_init(&mut request);
                        request.schema.as_mut().unwrap().primary_geometry = value.unwrap_or("").to_string();
                    },
                "schema.primary-key" => {
                        request_schema_init(&mut request);
                        request.schema.as_mut().unwrap().primary_key = value.unwrap_or("").to_string();
                    },
                "description" => {
                        request_schema_init(&mut request);
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "processing-status" => {
                        request_schema_init(&mut request);
                        request.processing_status = Some(value.unwrap_or("").to_string());
                    },
                "draft-access-list" => {
                        request_schema_init(&mut request);
                        request.draft_access_list = Some(value.unwrap_or("").to_string());
                    },
                "project-id" => {
                        request_schema_init(&mut request);
                        request.project_id = Some(value.unwrap_or("").to_string());
                    },
                "creation-time" => {
                        request_schema_init(&mut request);
                        request.creation_time = Some(value.unwrap_or("").to_string());
                    },
                "source-encoding" => {
                        request_schema_init(&mut request);
                        request.source_encoding = Some(value.unwrap_or("").to_string());
                    },
                "writers-can-edit-permissions" => {
                        request_schema_init(&mut request);
                        request.writers_can_edit_permissions = Some(arg_from_str(value.unwrap_or("false"), err, "writers-can-edit-permissions", "boolean"));
                    },
                "etag" => {
                        request_schema_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "creator-email" => {
                        request_schema_init(&mut request);
                        request.creator_email = Some(value.unwrap_or("").to_string());
                    },
                "bbox" => {
                        request_schema_init(&mut request);
                        if request.bbox.is_none() {
                            request.bbox = Some(Default::default());
                        }
                        request.bbox.as_mut().unwrap().push(arg_from_str(value.unwrap_or("0.0"), err, "bbox", "number"));
                    },
                "last-modifier-email" => {
                        request_schema_init(&mut request);
                        request.last_modifier_email = Some(value.unwrap_or("").to_string());
                    },
                "last-modified-time" => {
                        request_schema_init(&mut request);
                        request.last_modified_time = Some(value.unwrap_or("").to_string());
                    },
                "published-access-list" => {
                        request_schema_init(&mut request);
                        request.published_access_list = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_schema_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_schema_init(&mut request);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _tables_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.tables().delete(&self.opt.arg_id);
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

    fn _tables_features_batch_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::FeaturesBatchDeleteRequest = Default::default();
        let mut call = self.hub.tables().features_batch_delete(&request, &self.opt.arg_id);
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
                "gx-ids" => {
                        if request.gx_ids.is_none() {
                            request.gx_ids = Some(Default::default());
                        }
                        request.gx_ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
                    },
                "primary-keys" => {
                        if request.primary_keys.is_none() {
                            request.primary_keys = Some(Default::default());
                        }
                        request.primary_keys.as_mut().unwrap().push(value.unwrap_or("").to_string());
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

    fn _tables_features_batch_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::FeaturesBatchInsertRequest = Default::default();
        let mut call = self.hub.tables().features_batch_insert(&request, &self.opt.arg_id);
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
                "normalize-geometries" => {
                        request.normalize_geometries = Some(arg_from_str(value.unwrap_or("false"), err, "normalize-geometries", "boolean"));
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

    fn _tables_features_batch_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::FeaturesBatchPatchRequest = Default::default();
        let mut call = self.hub.tables().features_batch_patch(&request, &self.opt.arg_id);
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
                "normalize-geometries" => {
                        request.normalize_geometries = Some(arg_from_str(value.unwrap_or("false"), err, "normalize-geometries", "boolean"));
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

    fn _tables_features_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.tables().features_get(&self.opt.arg_table_id, &self.opt.arg_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "version" => {
                    call = call.version(value.unwrap_or(""));
                },
                "select" => {
                    call = call.select(value.unwrap_or(""));
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

    fn _tables_features_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.tables().features_list(&self.opt.arg_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "where" => {
                    call = call.where_(value.unwrap_or(""));
                },
                "version" => {
                    call = call.version(value.unwrap_or(""));
                },
                "select" => {
                    call = call.select(value.unwrap_or(""));
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
                "limit" => {
                    call = call.limit(arg_from_str(value.unwrap_or("-0"), err, "limit", "integer"));
                },
                "intersects" => {
                    call = call.intersects(value.unwrap_or(""));
                },
                "include" => {
                    call = call.include(value.unwrap_or(""));
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

    fn _tables_files_insert(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.tables().files_insert(&self.opt.arg_id, &self.opt.arg_filename);
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
            match match protocol {
                "simple" => call.upload(input_file.unwrap(), mime_type.unwrap()),
                "resumable" => call.upload_resumable(input_file.unwrap(), mime_type.unwrap()),
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

    fn _tables_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.tables().get(&self.opt.arg_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "version" => {
                    call = call.version(value.unwrap_or(""));
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

    fn _tables_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.tables().list();
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
            match key {
                "tags" => {
                    call = call.tags(value.unwrap_or(""));
                },
                "search" => {
                    call = call.search(value.unwrap_or(""));
                },
                "role" => {
                    call = call.role(value.unwrap_or(""));
                },
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
                },
                "processing-status" => {
                    call = call.processing_status(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "modified-before" => {
                    call = call.modified_before(value.unwrap_or(""));
                },
                "modified-after" => {
                    call = call.modified_after(value.unwrap_or(""));
                },
                "max-results" => {
                    call = call.max_results(arg_from_str(value.unwrap_or("-0"), err, "max-results", "integer"));
                },
                "creator-email" => {
                    call = call.creator_email(value.unwrap_or(""));
                },
                "created-before" => {
                    call = call.created_before(value.unwrap_or(""));
                },
                "created-after" => {
                    call = call.created_after(value.unwrap_or(""));
                },
                "bbox" => {
                    call = call.bbox(value.unwrap_or(""));
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

    fn _tables_parents_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.tables().parents_list(&self.opt.arg_id);
        for parg in self.opt.arg_v.iter() {
            let (key, value) = parse_kv_arg(&*parg, err);
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
                    println!("DEBUG: REMOVE ME {:?}", response);
                    serde::json::to_writer(&mut ostream, &output_schema).unwrap();
                    None
                }
            }
        }
    }

    fn _tables_patch(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Table = Default::default();
        let mut call = self.hub.tables().patch(&request, &self.opt.arg_id);
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
            fn request_schema_init(request: &mut api::Table) {
                if request.schema.is_none() {
                    request.schema = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "schema.primary-geometry" => {
                        request_schema_init(&mut request);
                        request.schema.as_mut().unwrap().primary_geometry = value.unwrap_or("").to_string();
                    },
                "schema.primary-key" => {
                        request_schema_init(&mut request);
                        request.schema.as_mut().unwrap().primary_key = value.unwrap_or("").to_string();
                    },
                "description" => {
                        request_schema_init(&mut request);
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "processing-status" => {
                        request_schema_init(&mut request);
                        request.processing_status = Some(value.unwrap_or("").to_string());
                    },
                "draft-access-list" => {
                        request_schema_init(&mut request);
                        request.draft_access_list = Some(value.unwrap_or("").to_string());
                    },
                "project-id" => {
                        request_schema_init(&mut request);
                        request.project_id = Some(value.unwrap_or("").to_string());
                    },
                "creation-time" => {
                        request_schema_init(&mut request);
                        request.creation_time = Some(value.unwrap_or("").to_string());
                    },
                "source-encoding" => {
                        request_schema_init(&mut request);
                        request.source_encoding = Some(value.unwrap_or("").to_string());
                    },
                "writers-can-edit-permissions" => {
                        request_schema_init(&mut request);
                        request.writers_can_edit_permissions = Some(arg_from_str(value.unwrap_or("false"), err, "writers-can-edit-permissions", "boolean"));
                    },
                "etag" => {
                        request_schema_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "creator-email" => {
                        request_schema_init(&mut request);
                        request.creator_email = Some(value.unwrap_or("").to_string());
                    },
                "bbox" => {
                        request_schema_init(&mut request);
                        if request.bbox.is_none() {
                            request.bbox = Some(Default::default());
                        }
                        request.bbox.as_mut().unwrap().push(arg_from_str(value.unwrap_or("0.0"), err, "bbox", "number"));
                    },
                "last-modifier-email" => {
                        request_schema_init(&mut request);
                        request.last_modifier_email = Some(value.unwrap_or("").to_string());
                    },
                "last-modified-time" => {
                        request_schema_init(&mut request);
                        request.last_modified_time = Some(value.unwrap_or("").to_string());
                    },
                "published-access-list" => {
                        request_schema_init(&mut request);
                        request.published_access_list = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_schema_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_schema_init(&mut request);
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

    fn _tables_permissions_batch_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::PermissionsBatchDeleteRequest = Default::default();
        let mut call = self.hub.tables().permissions_batch_delete(&request, &self.opt.arg_id);
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
                "ids" => {
                        if request.ids.is_none() {
                            request.ids = Some(Default::default());
                        }
                        request.ids.as_mut().unwrap().push(value.unwrap_or("").to_string());
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

    fn _tables_permissions_batch_update(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::PermissionsBatchUpdateRequest = Default::default();
        let mut call = self.hub.tables().permissions_batch_update(&request, &self.opt.arg_id);
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

    fn _tables_permissions_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.tables().permissions_list(&self.opt.arg_id);
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

    fn _tables_process(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.tables().process(&self.opt.arg_id);
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

    fn _tables_upload(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::Table = Default::default();
        let mut call = self.hub.tables().upload(&request);
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
            fn request_schema_init(request: &mut api::Table) {
                if request.schema.is_none() {
                    request.schema = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "schema.primary-geometry" => {
                        request_schema_init(&mut request);
                        request.schema.as_mut().unwrap().primary_geometry = value.unwrap_or("").to_string();
                    },
                "schema.primary-key" => {
                        request_schema_init(&mut request);
                        request.schema.as_mut().unwrap().primary_key = value.unwrap_or("").to_string();
                    },
                "description" => {
                        request_schema_init(&mut request);
                        request.description = Some(value.unwrap_or("").to_string());
                    },
                "processing-status" => {
                        request_schema_init(&mut request);
                        request.processing_status = Some(value.unwrap_or("").to_string());
                    },
                "draft-access-list" => {
                        request_schema_init(&mut request);
                        request.draft_access_list = Some(value.unwrap_or("").to_string());
                    },
                "project-id" => {
                        request_schema_init(&mut request);
                        request.project_id = Some(value.unwrap_or("").to_string());
                    },
                "creation-time" => {
                        request_schema_init(&mut request);
                        request.creation_time = Some(value.unwrap_or("").to_string());
                    },
                "source-encoding" => {
                        request_schema_init(&mut request);
                        request.source_encoding = Some(value.unwrap_or("").to_string());
                    },
                "writers-can-edit-permissions" => {
                        request_schema_init(&mut request);
                        request.writers_can_edit_permissions = Some(arg_from_str(value.unwrap_or("false"), err, "writers-can-edit-permissions", "boolean"));
                    },
                "etag" => {
                        request_schema_init(&mut request);
                        request.etag = Some(value.unwrap_or("").to_string());
                    },
                "creator-email" => {
                        request_schema_init(&mut request);
                        request.creator_email = Some(value.unwrap_or("").to_string());
                    },
                "bbox" => {
                        request_schema_init(&mut request);
                        if request.bbox.is_none() {
                            request.bbox = Some(Default::default());
                        }
                        request.bbox.as_mut().unwrap().push(arg_from_str(value.unwrap_or("0.0"), err, "bbox", "number"));
                    },
                "last-modifier-email" => {
                        request_schema_init(&mut request);
                        request.last_modifier_email = Some(value.unwrap_or("").to_string());
                    },
                "last-modified-time" => {
                        request_schema_init(&mut request);
                        request.last_modified_time = Some(value.unwrap_or("").to_string());
                    },
                "published-access-list" => {
                        request_schema_init(&mut request);
                        request.published_access_list = Some(value.unwrap_or("").to_string());
                    },
                "id" => {
                        request_schema_init(&mut request);
                        request.id = Some(value.unwrap_or("").to_string());
                    },
                "name" => {
                        request_schema_init(&mut request);
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

        if self.opt.cmd_assets {
            if self.opt.cmd_get {
                call_result = self._assets_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._assets_list(dry_run, &mut err);
            } else if self.opt.cmd_parents_list {
                call_result = self._assets_parents_list(dry_run, &mut err);
            } else if self.opt.cmd_permissions_list {
                call_result = self._assets_permissions_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_layers {
            if self.opt.cmd_cancel_processing {
                call_result = self._layers_cancel_processing(dry_run, &mut err);
            } else if self.opt.cmd_create {
                call_result = self._layers_create(dry_run, &mut err);
            } else if self.opt.cmd_delete {
                call_result = self._layers_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._layers_get(dry_run, &mut err);
            } else if self.opt.cmd_get_published {
                call_result = self._layers_get_published(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._layers_list(dry_run, &mut err);
            } else if self.opt.cmd_list_published {
                call_result = self._layers_list_published(dry_run, &mut err);
            } else if self.opt.cmd_parents_list {
                call_result = self._layers_parents_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._layers_patch(dry_run, &mut err);
            } else if self.opt.cmd_permissions_batch_delete {
                call_result = self._layers_permissions_batch_delete(dry_run, &mut err);
            } else if self.opt.cmd_permissions_batch_update {
                call_result = self._layers_permissions_batch_update(dry_run, &mut err);
            } else if self.opt.cmd_permissions_list {
                call_result = self._layers_permissions_list(dry_run, &mut err);
            } else if self.opt.cmd_process {
                call_result = self._layers_process(dry_run, &mut err);
            } else if self.opt.cmd_publish {
                call_result = self._layers_publish(dry_run, &mut err);
            } else if self.opt.cmd_unpublish {
                call_result = self._layers_unpublish(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_maps {
            if self.opt.cmd_create {
                call_result = self._maps_create(dry_run, &mut err);
            } else if self.opt.cmd_delete {
                call_result = self._maps_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._maps_get(dry_run, &mut err);
            } else if self.opt.cmd_get_published {
                call_result = self._maps_get_published(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._maps_list(dry_run, &mut err);
            } else if self.opt.cmd_list_published {
                call_result = self._maps_list_published(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._maps_patch(dry_run, &mut err);
            } else if self.opt.cmd_permissions_batch_delete {
                call_result = self._maps_permissions_batch_delete(dry_run, &mut err);
            } else if self.opt.cmd_permissions_batch_update {
                call_result = self._maps_permissions_batch_update(dry_run, &mut err);
            } else if self.opt.cmd_permissions_list {
                call_result = self._maps_permissions_list(dry_run, &mut err);
            } else if self.opt.cmd_publish {
                call_result = self._maps_publish(dry_run, &mut err);
            } else if self.opt.cmd_unpublish {
                call_result = self._maps_unpublish(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_projects {
            if self.opt.cmd_icons_create {
                call_result = self._projects_icons_create(dry_run, &mut err);
            } else if self.opt.cmd_icons_get {
                call_result = self._projects_icons_get(dry_run, &mut err);
            } else if self.opt.cmd_icons_list {
                call_result = self._projects_icons_list(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._projects_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_raster_collections {
            if self.opt.cmd_cancel_processing {
                call_result = self._raster_collections_cancel_processing(dry_run, &mut err);
            } else if self.opt.cmd_create {
                call_result = self._raster_collections_create(dry_run, &mut err);
            } else if self.opt.cmd_delete {
                call_result = self._raster_collections_delete(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._raster_collections_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._raster_collections_list(dry_run, &mut err);
            } else if self.opt.cmd_parents_list {
                call_result = self._raster_collections_parents_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._raster_collections_patch(dry_run, &mut err);
            } else if self.opt.cmd_permissions_batch_delete {
                call_result = self._raster_collections_permissions_batch_delete(dry_run, &mut err);
            } else if self.opt.cmd_permissions_batch_update {
                call_result = self._raster_collections_permissions_batch_update(dry_run, &mut err);
            } else if self.opt.cmd_permissions_list {
                call_result = self._raster_collections_permissions_list(dry_run, &mut err);
            } else if self.opt.cmd_process {
                call_result = self._raster_collections_process(dry_run, &mut err);
            } else if self.opt.cmd_rasters_batch_delete {
                call_result = self._raster_collections_rasters_batch_delete(dry_run, &mut err);
            } else if self.opt.cmd_rasters_batch_insert {
                call_result = self._raster_collections_rasters_batch_insert(dry_run, &mut err);
            } else if self.opt.cmd_rasters_list {
                call_result = self._raster_collections_rasters_list(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_rasters {
            if self.opt.cmd_delete {
                call_result = self._rasters_delete(dry_run, &mut err);
            } else if self.opt.cmd_files_insert {
                call_result = self._rasters_files_insert(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._rasters_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._rasters_list(dry_run, &mut err);
            } else if self.opt.cmd_parents_list {
                call_result = self._rasters_parents_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._rasters_patch(dry_run, &mut err);
            } else if self.opt.cmd_permissions_batch_delete {
                call_result = self._rasters_permissions_batch_delete(dry_run, &mut err);
            } else if self.opt.cmd_permissions_batch_update {
                call_result = self._rasters_permissions_batch_update(dry_run, &mut err);
            } else if self.opt.cmd_permissions_list {
                call_result = self._rasters_permissions_list(dry_run, &mut err);
            } else if self.opt.cmd_process {
                call_result = self._rasters_process(dry_run, &mut err);
            } else if self.opt.cmd_upload {
                call_result = self._rasters_upload(dry_run, &mut err);
            } else {
                unreachable!();
            }
        } else if self.opt.cmd_tables {
            if self.opt.cmd_create {
                call_result = self._tables_create(dry_run, &mut err);
            } else if self.opt.cmd_delete {
                call_result = self._tables_delete(dry_run, &mut err);
            } else if self.opt.cmd_features_batch_delete {
                call_result = self._tables_features_batch_delete(dry_run, &mut err);
            } else if self.opt.cmd_features_batch_insert {
                call_result = self._tables_features_batch_insert(dry_run, &mut err);
            } else if self.opt.cmd_features_batch_patch {
                call_result = self._tables_features_batch_patch(dry_run, &mut err);
            } else if self.opt.cmd_features_get {
                call_result = self._tables_features_get(dry_run, &mut err);
            } else if self.opt.cmd_features_list {
                call_result = self._tables_features_list(dry_run, &mut err);
            } else if self.opt.cmd_files_insert {
                call_result = self._tables_files_insert(dry_run, &mut err);
            } else if self.opt.cmd_get {
                call_result = self._tables_get(dry_run, &mut err);
            } else if self.opt.cmd_list {
                call_result = self._tables_list(dry_run, &mut err);
            } else if self.opt.cmd_parents_list {
                call_result = self._tables_parents_list(dry_run, &mut err);
            } else if self.opt.cmd_patch {
                call_result = self._tables_patch(dry_run, &mut err);
            } else if self.opt.cmd_permissions_batch_delete {
                call_result = self._tables_permissions_batch_delete(dry_run, &mut err);
            } else if self.opt.cmd_permissions_batch_update {
                call_result = self._tables_permissions_batch_update(dry_run, &mut err);
            } else if self.opt.cmd_permissions_list {
                call_result = self._tables_permissions_list(dry_run, &mut err);
            } else if self.opt.cmd_process {
                call_result = self._tables_process(dry_run, &mut err);
            } else if self.opt.cmd_upload {
                call_result = self._tables_upload(dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "mapsengine1-secret.json") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                                      hyper::Client::new(),
                                      JsonTokenStorage {
                                        program_name: "mapsengine1",
                                        db_dir: config_dir.clone(),
                                      }, None);
        let engine = Engine {
            opt: opt,
            hub: api::MapsEngine::new(hyper::Client::new(), auth),
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