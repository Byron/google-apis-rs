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
extern crate google_container1_beta1 as api;

use std::env;
use std::io::{self, Write};

docopt!(Options derive Debug, "
Usage: 
  container1-beta1 [options] projects clusters-list <project-id> [-p <v>]... [-o <out>]
  container1-beta1 [options] projects operations-list <project-id> [-p <v>]... [-o <out>]
  container1-beta1 [options] projects zones-clusters-create <project-id> <zone-id> -r <kv>... [-p <v>]... [-o <out>]
  container1-beta1 [options] projects zones-clusters-delete <project-id> <zone-id> <cluster-id> [-p <v>]... [-o <out>]
  container1-beta1 [options] projects zones-clusters-get <project-id> <zone-id> <cluster-id> [-p <v>]... [-o <out>]
  container1-beta1 [options] projects zones-clusters-list <project-id> <zone-id> [-p <v>]... [-o <out>]
  container1-beta1 [options] projects zones-operations-get <project-id> <zone-id> <operation-id> [-p <v>]... [-o <out>]
  container1-beta1 [options] projects zones-operations-list <project-id> <zone-id> [-p <v>]... [-o <out>]
  container1-beta1 --help

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
    hub: api::Container<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
}


impl Engine {
    fn _projects_clusters_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.projects().clusters_list(&self.opt.arg_project_id);
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

    fn _projects_operations_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.projects().operations_list(&self.opt.arg_project_id);
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

    fn _projects_zones_clusters_create(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
            let mut request: api::CreateClusterRequest = Default::default();
        let mut call = self.hub.projects().zones_clusters_create(&request, &self.opt.arg_project_id, &self.opt.arg_zone_id);
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
            fn request_cluster_init(request: &mut api::CreateClusterRequest) {
                if request.cluster.is_none() {
                    request.cluster = Some(Default::default());
                }
            }
            
            match &field_name.to_string()[..] {
                "cluster.status" => {
                        request_cluster_init(&mut request);
                        request.cluster.as_mut().unwrap().status = value.unwrap_or("").to_string();
                    },
                "cluster.container-ipv4-cidr" => {
                        request_cluster_init(&mut request);
                        request.cluster.as_mut().unwrap().container_ipv4_cidr = value.unwrap_or("").to_string();
                    },
                "cluster.description" => {
                        request_cluster_init(&mut request);
                        request.cluster.as_mut().unwrap().description = value.unwrap_or("").to_string();
                    },
                "cluster.zone" => {
                        request_cluster_init(&mut request);
                        request.cluster.as_mut().unwrap().zone = value.unwrap_or("").to_string();
                    },
                "cluster.num-nodes" => {
                        request_cluster_init(&mut request);
                        request.cluster.as_mut().unwrap().num_nodes = arg_from_str(value.unwrap_or("-0"), err, "cluster.num-nodes", "integer");
                    },
                "cluster.node-routing-prefix-size" => {
                        request_cluster_init(&mut request);
                        request.cluster.as_mut().unwrap().node_routing_prefix_size = arg_from_str(value.unwrap_or("-0"), err, "cluster.node-routing-prefix-size", "integer");
                    },
                "cluster.master-auth.password" => {
                        request_cluster_init(&mut request);
                        request.cluster.as_mut().unwrap().master_auth.password = value.unwrap_or("").to_string();
                    },
                "cluster.master-auth.user" => {
                        request_cluster_init(&mut request);
                        request.cluster.as_mut().unwrap().master_auth.user = value.unwrap_or("").to_string();
                    },
                "cluster.cluster-api-version" => {
                        request_cluster_init(&mut request);
                        request.cluster.as_mut().unwrap().cluster_api_version = value.unwrap_or("").to_string();
                    },
                "cluster.network" => {
                        request_cluster_init(&mut request);
                        request.cluster.as_mut().unwrap().network = value.unwrap_or("").to_string();
                    },
                "cluster.endpoint" => {
                        request_cluster_init(&mut request);
                        request.cluster.as_mut().unwrap().endpoint = value.unwrap_or("").to_string();
                    },
                "cluster.node-config.machine-type" => {
                        request_cluster_init(&mut request);
                        request.cluster.as_mut().unwrap().node_config.machine_type = value.unwrap_or("").to_string();
                    },
                "cluster.node-config.source-image" => {
                        request_cluster_init(&mut request);
                        request.cluster.as_mut().unwrap().node_config.source_image = value.unwrap_or("").to_string();
                    },
                "cluster.status-message" => {
                        request_cluster_init(&mut request);
                        request.cluster.as_mut().unwrap().status_message = value.unwrap_or("").to_string();
                    },
                "cluster.services-ipv4-cidr" => {
                        request_cluster_init(&mut request);
                        request.cluster.as_mut().unwrap().services_ipv4_cidr = value.unwrap_or("").to_string();
                    },
                "cluster.creation-timestamp" => {
                        request_cluster_init(&mut request);
                        request.cluster.as_mut().unwrap().creation_timestamp = value.unwrap_or("").to_string();
                    },
                "cluster.enable-cloud-logging" => {
                        request_cluster_init(&mut request);
                        request.cluster.as_mut().unwrap().enable_cloud_logging = arg_from_str(value.unwrap_or("false"), err, "cluster.enable-cloud-logging", "boolean");
                    },
                "cluster.self-link" => {
                        request_cluster_init(&mut request);
                        request.cluster.as_mut().unwrap().self_link = value.unwrap_or("").to_string();
                    },
                "cluster.name" => {
                        request_cluster_init(&mut request);
                        request.cluster.as_mut().unwrap().name = value.unwrap_or("").to_string();
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

    fn _projects_zones_clusters_delete(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.projects().zones_clusters_delete(&self.opt.arg_project_id, &self.opt.arg_zone_id, &self.opt.arg_cluster_id);
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

    fn _projects_zones_clusters_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.projects().zones_clusters_get(&self.opt.arg_project_id, &self.opt.arg_zone_id, &self.opt.arg_cluster_id);
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

    fn _projects_zones_clusters_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.projects().zones_clusters_list(&self.opt.arg_project_id, &self.opt.arg_zone_id);
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

    fn _projects_zones_operations_get(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.projects().zones_operations_get(&self.opt.arg_project_id, &self.opt.arg_zone_id, &self.opt.arg_operation_id);
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

    fn _projects_zones_operations_list(&self, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Option<api::Error> {
        let mut call = self.hub.projects().zones_operations_list(&self.opt.arg_project_id, &self.opt.arg_zone_id);
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

        if self.opt.cmd_projects {
            if self.opt.cmd_clusters_list {
                call_result = self._projects_clusters_list(dry_run, &mut err);
            } else if self.opt.cmd_operations_list {
                call_result = self._projects_operations_list(dry_run, &mut err);
            } else if self.opt.cmd_zones_clusters_create {
                call_result = self._projects_zones_clusters_create(dry_run, &mut err);
            } else if self.opt.cmd_zones_clusters_delete {
                call_result = self._projects_zones_clusters_delete(dry_run, &mut err);
            } else if self.opt.cmd_zones_clusters_get {
                call_result = self._projects_zones_clusters_get(dry_run, &mut err);
            } else if self.opt.cmd_zones_clusters_list {
                call_result = self._projects_zones_clusters_list(dry_run, &mut err);
            } else if self.opt.cmd_zones_operations_get {
                call_result = self._projects_zones_operations_get(dry_run, &mut err);
            } else if self.opt.cmd_zones_operations_list {
                call_result = self._projects_zones_operations_list(dry_run, &mut err);
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

            match cmn::application_secret_from_directory(&config_dir, "container1-beta1-secret.json") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                                      hyper::Client::new(),
                                      JsonTokenStorage {
                                        program_name: "container1-beta1",
                                        db_dir: config_dir.clone(),
                                      }, None);
        let engine = Engine {
            opt: opt,
            hub: api::Container::new(hyper::Client::new(), auth),
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