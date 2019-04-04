// DO NOT EDIT !
// This file was generated automatically from 'src/mako/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;
extern crate yup_oauth2 as oauth2;
extern crate yup_hyper_mock as mock;
extern crate hyper_rustls;
extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate mime;
extern crate strsim;
extern crate google_container1 as api;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

mod cmn;

use cmn::{InvalidOptionsError, CLIError, JsonTokenStorage, arg_from_str, writer_from_opts, parse_kv_arg,
          input_file_from_opts, input_mime_from_opts, FieldCursor, FieldError, CallType, UploadProtocol,
          calltype_from_str, remove_json_null_values, ComplexType, JsonType, JsonTypeInfo};

use std::default::Default;
use std::str::FromStr;

use oauth2::{Authenticator, DefaultAuthenticatorDelegate, FlowType};
use serde_json as json;
use clap::ArgMatches;

enum DoitError {
    IoError(String, io::Error),
    ApiError(api::Error),
}

struct Engine<'n> {
    opt: ArgMatches<'n>,
    hub: api::Container<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n> Engine<'n> {
    fn _projects_locations_clusters_complete_ip_rotation(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cluster-id", "name", "project-id", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CompleteIPRotationRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_clusters_complete_ip_rotation(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_locations_clusters_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "cluster.node-ipv4-cidr-size" => Some(("cluster.nodeIpv4CidrSize", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.maintenance-policy.window.daily-maintenance-window.duration" => Some(("cluster.maintenancePolicy.window.dailyMaintenanceWindow.duration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.maintenance-policy.window.daily-maintenance-window.start-time" => Some(("cluster.maintenancePolicy.window.dailyMaintenanceWindow.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.network-policy.enabled" => Some(("cluster.networkPolicy.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.network-policy.provider" => Some(("cluster.networkPolicy.provider", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.master-authorized-networks-config.enabled" => Some(("cluster.masterAuthorizedNetworksConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.http-load-balancing.disabled" => Some(("cluster.addonsConfig.httpLoadBalancing.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.network-policy-config.disabled" => Some(("cluster.addonsConfig.networkPolicyConfig.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.kubernetes-dashboard.disabled" => Some(("cluster.addonsConfig.kubernetesDashboard.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.horizontal-pod-autoscaling.disabled" => Some(("cluster.addonsConfig.horizontalPodAutoscaling.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.locations" => Some(("cluster.locations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.enable-kubernetes-alpha" => Some(("cluster.enableKubernetesAlpha", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.network" => Some(("cluster.network", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.logging-service" => Some(("cluster.loggingService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.instance-group-urls" => Some(("cluster.instanceGroupUrls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.network-config.subnetwork" => Some(("cluster.networkConfig.subnetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.network-config.network" => Some(("cluster.networkConfig.network", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.location" => Some(("cluster.location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.enable-private-endpoint" => Some(("cluster.privateClusterConfig.enablePrivateEndpoint", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.master-ipv4-cidr-block" => Some(("cluster.privateClusterConfig.masterIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.private-endpoint" => Some(("cluster.privateClusterConfig.privateEndpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.public-endpoint" => Some(("cluster.privateClusterConfig.publicEndpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.enable-private-nodes" => Some(("cluster.privateClusterConfig.enablePrivateNodes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.services-ipv4-cidr" => Some(("cluster.servicesIpv4Cidr", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.status-message" => Some(("cluster.statusMessage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.node-ipv4-cidr" => Some(("cluster.ipAllocationPolicy.nodeIpv4Cidr", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.use-ip-aliases" => Some(("cluster.ipAllocationPolicy.useIpAliases", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.cluster-secondary-range-name" => Some(("cluster.ipAllocationPolicy.clusterSecondaryRangeName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.cluster-ipv4-cidr-block" => Some(("cluster.ipAllocationPolicy.clusterIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.services-ipv4-cidr-block" => Some(("cluster.ipAllocationPolicy.servicesIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.create-subnetwork" => Some(("cluster.ipAllocationPolicy.createSubnetwork", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.node-ipv4-cidr-block" => Some(("cluster.ipAllocationPolicy.nodeIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.tpu-ipv4-cidr-block" => Some(("cluster.ipAllocationPolicy.tpuIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.cluster-ipv4-cidr" => Some(("cluster.ipAllocationPolicy.clusterIpv4Cidr", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.subnetwork-name" => Some(("cluster.ipAllocationPolicy.subnetworkName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.services-secondary-range-name" => Some(("cluster.ipAllocationPolicy.servicesSecondaryRangeName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.services-ipv4-cidr" => Some(("cluster.ipAllocationPolicy.servicesIpv4Cidr", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.status" => Some(("cluster.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.description" => Some(("cluster.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.current-node-version" => Some(("cluster.currentNodeVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.label-fingerprint" => Some(("cluster.labelFingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.current-master-version" => Some(("cluster.currentMasterVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.master-auth.username" => Some(("cluster.masterAuth.username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.master-auth.client-key" => Some(("cluster.masterAuth.clientKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.master-auth.client-certificate" => Some(("cluster.masterAuth.clientCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.master-auth.client-certificate-config.issue-client-certificate" => Some(("cluster.masterAuth.clientCertificateConfig.issueClientCertificate", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.master-auth.password" => Some(("cluster.masterAuth.password", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.master-auth.cluster-ca-certificate" => Some(("cluster.masterAuth.clusterCaCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.expire-time" => Some(("cluster.expireTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.zone" => Some(("cluster.zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.initial-node-count" => Some(("cluster.initialNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.monitoring-service" => Some(("cluster.monitoringService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.create-time" => Some(("cluster.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.enable-tpu" => Some(("cluster.enableTpu", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.endpoint" => Some(("cluster.endpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.current-node-count" => Some(("cluster.currentNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.name" => Some(("cluster.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.legacy-abac.enabled" => Some(("cluster.legacyAbac.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.initial-cluster-version" => Some(("cluster.initialClusterVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.machine-type" => Some(("cluster.nodeConfig.machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.tags" => Some(("cluster.nodeConfig.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.node-config.preemptible" => Some(("cluster.nodeConfig.preemptible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.labels" => Some(("cluster.nodeConfig.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "cluster.node-config.service-account" => Some(("cluster.nodeConfig.serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.oauth-scopes" => Some(("cluster.nodeConfig.oauthScopes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.node-config.disk-type" => Some(("cluster.nodeConfig.diskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.disk-size-gb" => Some(("cluster.nodeConfig.diskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-config.min-cpu-platform" => Some(("cluster.nodeConfig.minCpuPlatform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.metadata" => Some(("cluster.nodeConfig.metadata", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "cluster.node-config.image-type" => Some(("cluster.nodeConfig.imageType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.local-ssd-count" => Some(("cluster.nodeConfig.localSsdCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.cluster-ipv4-cidr" => Some(("cluster.clusterIpv4Cidr", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.subnetwork" => Some(("cluster.subnetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.resource-labels" => Some(("cluster.resourceLabels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "cluster.self-link" => Some(("cluster.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.tpu-ipv4-cidr-block" => Some(("cluster.tpuIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent" => Some(("parent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["addons-config", "client-certificate", "client-certificate-config", "client-key", "cluster", "cluster-ca-certificate", "cluster-ipv4-cidr", "cluster-ipv4-cidr-block", "cluster-secondary-range-name", "create-subnetwork", "create-time", "current-master-version", "current-node-count", "current-node-version", "daily-maintenance-window", "description", "disabled", "disk-size-gb", "disk-type", "duration", "enable-kubernetes-alpha", "enable-private-endpoint", "enable-private-nodes", "enable-tpu", "enabled", "endpoint", "expire-time", "horizontal-pod-autoscaling", "http-load-balancing", "image-type", "initial-cluster-version", "initial-node-count", "instance-group-urls", "ip-allocation-policy", "issue-client-certificate", "kubernetes-dashboard", "label-fingerprint", "labels", "legacy-abac", "local-ssd-count", "location", "locations", "logging-service", "machine-type", "maintenance-policy", "master-auth", "master-authorized-networks-config", "master-ipv4-cidr-block", "metadata", "min-cpu-platform", "monitoring-service", "name", "network", "network-config", "network-policy", "network-policy-config", "node-config", "node-ipv4-cidr", "node-ipv4-cidr-block", "node-ipv4-cidr-size", "oauth-scopes", "parent", "password", "preemptible", "private-cluster-config", "private-endpoint", "project-id", "provider", "public-endpoint", "resource-labels", "self-link", "service-account", "services-ipv4-cidr", "services-ipv4-cidr-block", "services-secondary-range-name", "start-time", "status", "status-message", "subnetwork", "subnetwork-name", "tags", "tpu-ipv4-cidr-block", "use-ip-aliases", "username", "window", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CreateClusterRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_clusters_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_locations_clusters_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_clusters_delete(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "zone" => {
                    call = call.zone(value.unwrap_or(""));
                },
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
                },
                "cluster-id" => {
                    call = call.cluster_id(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["project-id", "cluster-id", "zone"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_locations_clusters_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_clusters_get(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "zone" => {
                    call = call.zone(value.unwrap_or(""));
                },
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
                },
                "cluster-id" => {
                    call = call.cluster_id(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["project-id", "cluster-id", "zone"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_locations_clusters_get_jwks(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_clusters_get_jwks(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_locations_clusters_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_clusters_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "zone" => {
                    call = call.zone(value.unwrap_or(""));
                },
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["project-id", "zone"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_locations_clusters_node_pools_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent" => Some(("parent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.status" => Some(("nodePool.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.management.auto-repair" => Some(("nodePool.management.autoRepair", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.management.upgrade-options.description" => Some(("nodePool.management.upgradeOptions.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.management.upgrade-options.auto-upgrade-start-time" => Some(("nodePool.management.upgradeOptions.autoUpgradeStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.management.auto-upgrade" => Some(("nodePool.management.autoUpgrade", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.name" => Some(("nodePool.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.instance-group-urls" => Some(("nodePool.instanceGroupUrls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "node-pool.autoscaling.min-node-count" => Some(("nodePool.autoscaling.minNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.autoscaling.enabled" => Some(("nodePool.autoscaling.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.autoscaling.max-node-count" => Some(("nodePool.autoscaling.maxNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.version" => Some(("nodePool.version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.initial-node-count" => Some(("nodePool.initialNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.status-message" => Some(("nodePool.statusMessage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.machine-type" => Some(("nodePool.config.machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.tags" => Some(("nodePool.config.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "node-pool.config.preemptible" => Some(("nodePool.config.preemptible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.labels" => Some(("nodePool.config.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "node-pool.config.service-account" => Some(("nodePool.config.serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.oauth-scopes" => Some(("nodePool.config.oauthScopes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "node-pool.config.disk-type" => Some(("nodePool.config.diskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.disk-size-gb" => Some(("nodePool.config.diskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.min-cpu-platform" => Some(("nodePool.config.minCpuPlatform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.metadata" => Some(("nodePool.config.metadata", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "node-pool.config.image-type" => Some(("nodePool.config.imageType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.local-ssd-count" => Some(("nodePool.config.localSsdCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.self-link" => Some(("nodePool.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["auto-repair", "auto-upgrade", "auto-upgrade-start-time", "autoscaling", "cluster-id", "config", "description", "disk-size-gb", "disk-type", "enabled", "image-type", "initial-node-count", "instance-group-urls", "labels", "local-ssd-count", "machine-type", "management", "max-node-count", "metadata", "min-cpu-platform", "min-node-count", "name", "node-pool", "oauth-scopes", "parent", "preemptible", "project-id", "self-link", "service-account", "status", "status-message", "tags", "upgrade-options", "version", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CreateNodePoolRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_clusters_node_pools_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_locations_clusters_node_pools_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_clusters_node_pools_delete(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "zone" => {
                    call = call.zone(value.unwrap_or(""));
                },
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
                },
                "node-pool-id" => {
                    call = call.node_pool_id(value.unwrap_or(""));
                },
                "cluster-id" => {
                    call = call.cluster_id(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["project-id", "cluster-id", "zone", "node-pool-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_locations_clusters_node_pools_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_clusters_node_pools_get(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "zone" => {
                    call = call.zone(value.unwrap_or(""));
                },
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
                },
                "node-pool-id" => {
                    call = call.node_pool_id(value.unwrap_or(""));
                },
                "cluster-id" => {
                    call = call.cluster_id(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["project-id", "cluster-id", "zone", "node-pool-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_locations_clusters_node_pools_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_clusters_node_pools_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "zone" => {
                    call = call.zone(value.unwrap_or(""));
                },
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
                },
                "cluster-id" => {
                    call = call.cluster_id(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["project-id", "cluster-id", "zone"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_locations_clusters_node_pools_rollback(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool-id" => Some(("nodePoolId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cluster-id", "name", "node-pool-id", "project-id", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::RollbackNodePoolUpgradeRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_clusters_node_pools_rollback(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_locations_clusters_node_pools_set_autoscaling(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "autoscaling.min-node-count" => Some(("autoscaling.minNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "autoscaling.enabled" => Some(("autoscaling.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "autoscaling.max-node-count" => Some(("autoscaling.maxNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool-id" => Some(("nodePoolId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["autoscaling", "cluster-id", "enabled", "max-node-count", "min-node-count", "name", "node-pool-id", "project-id", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SetNodePoolAutoscalingRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_clusters_node_pools_set_autoscaling(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_locations_clusters_node_pools_set_management(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "management.auto-repair" => Some(("management.autoRepair", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "management.upgrade-options.description" => Some(("management.upgradeOptions.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "management.upgrade-options.auto-upgrade-start-time" => Some(("management.upgradeOptions.autoUpgradeStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "management.auto-upgrade" => Some(("management.autoUpgrade", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool-id" => Some(("nodePoolId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["auto-repair", "auto-upgrade", "auto-upgrade-start-time", "cluster-id", "description", "management", "name", "node-pool-id", "project-id", "upgrade-options", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SetNodePoolManagementRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_clusters_node_pools_set_management(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_locations_clusters_node_pools_set_size(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool-id" => Some(("nodePoolId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-count" => Some(("nodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cluster-id", "name", "node-count", "node-pool-id", "project-id", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SetNodePoolSizeRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_clusters_node_pools_set_size(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_locations_clusters_node_pools_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool-id" => Some(("nodePoolId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-version" => Some(("nodeVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-type" => Some(("imageType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cluster-id", "image-type", "name", "node-pool-id", "node-version", "project-id", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::UpdateNodePoolRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_clusters_node_pools_update(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_locations_clusters_set_addons(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "addons-config.http-load-balancing.disabled" => Some(("addonsConfig.httpLoadBalancing.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "addons-config.network-policy-config.disabled" => Some(("addonsConfig.networkPolicyConfig.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "addons-config.kubernetes-dashboard.disabled" => Some(("addonsConfig.kubernetesDashboard.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "addons-config.horizontal-pod-autoscaling.disabled" => Some(("addonsConfig.horizontalPodAutoscaling.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["addons-config", "cluster-id", "disabled", "horizontal-pod-autoscaling", "http-load-balancing", "kubernetes-dashboard", "name", "network-policy-config", "project-id", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SetAddonsConfigRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_clusters_set_addons(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_locations_clusters_set_legacy_abac(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "enabled" => Some(("enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cluster-id", "enabled", "name", "project-id", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SetLegacyAbacRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_clusters_set_legacy_abac(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_locations_clusters_set_locations(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "locations" => Some(("locations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cluster-id", "locations", "name", "project-id", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SetLocationsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_clusters_set_locations(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_locations_clusters_set_logging(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "logging-service" => Some(("loggingService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cluster-id", "logging-service", "name", "project-id", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SetLoggingServiceRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_clusters_set_logging(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_locations_clusters_set_maintenance_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "maintenance-policy.window.daily-maintenance-window.duration" => Some(("maintenancePolicy.window.dailyMaintenanceWindow.duration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "maintenance-policy.window.daily-maintenance-window.start-time" => Some(("maintenancePolicy.window.dailyMaintenanceWindow.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cluster-id", "daily-maintenance-window", "duration", "maintenance-policy", "name", "project-id", "start-time", "window", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SetMaintenancePolicyRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_clusters_set_maintenance_policy(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_locations_clusters_set_master_auth(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.username" => Some(("update.username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.client-key" => Some(("update.clientKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.client-certificate" => Some(("update.clientCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.client-certificate-config.issue-client-certificate" => Some(("update.clientCertificateConfig.issueClientCertificate", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.password" => Some(("update.password", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.cluster-ca-certificate" => Some(("update.clusterCaCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "action" => Some(("action", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["action", "client-certificate", "client-certificate-config", "client-key", "cluster-ca-certificate", "cluster-id", "issue-client-certificate", "name", "password", "project-id", "update", "username", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SetMasterAuthRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_clusters_set_master_auth(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_locations_clusters_set_monitoring(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "monitoring-service" => Some(("monitoringService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cluster-id", "monitoring-service", "name", "project-id", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SetMonitoringServiceRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_clusters_set_monitoring(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_locations_clusters_set_network_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "network-policy.enabled" => Some(("networkPolicy.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "network-policy.provider" => Some(("networkPolicy.provider", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cluster-id", "enabled", "name", "network-policy", "project-id", "provider", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SetNetworkPolicyRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_clusters_set_network_policy(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_locations_clusters_set_resource_labels(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "label-fingerprint" => Some(("labelFingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "resource-labels" => Some(("resourceLabels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cluster-id", "label-fingerprint", "name", "project-id", "resource-labels", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SetLabelsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_clusters_set_resource_labels(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_locations_clusters_start_ip_rotation(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rotate-credentials" => Some(("rotateCredentials", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cluster-id", "name", "project-id", "rotate-credentials", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::StartIPRotationRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_clusters_start_ip_rotation(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_locations_clusters_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-master-authorized-networks-config.enabled" => Some(("update.desiredMasterAuthorizedNetworksConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-id" => Some(("update.desiredNodePoolId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.http-load-balancing.disabled" => Some(("update.desiredAddonsConfig.httpLoadBalancing.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.network-policy-config.disabled" => Some(("update.desiredAddonsConfig.networkPolicyConfig.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.kubernetes-dashboard.disabled" => Some(("update.desiredAddonsConfig.kubernetesDashboard.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.horizontal-pod-autoscaling.disabled" => Some(("update.desiredAddonsConfig.horizontalPodAutoscaling.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-master-version" => Some(("update.desiredMasterVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-locations" => Some(("update.desiredLocations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.desired-node-version" => Some(("update.desiredNodeVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-image-type" => Some(("update.desiredImageType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-autoscaling.min-node-count" => Some(("update.desiredNodePoolAutoscaling.minNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-autoscaling.enabled" => Some(("update.desiredNodePoolAutoscaling.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-autoscaling.max-node-count" => Some(("update.desiredNodePoolAutoscaling.maxNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-monitoring-service" => Some(("update.desiredMonitoringService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cluster-id", "desired-addons-config", "desired-image-type", "desired-locations", "desired-master-authorized-networks-config", "desired-master-version", "desired-monitoring-service", "desired-node-pool-autoscaling", "desired-node-pool-id", "desired-node-version", "disabled", "enabled", "horizontal-pod-autoscaling", "http-load-balancing", "kubernetes-dashboard", "max-node-count", "min-node-count", "name", "network-policy-config", "project-id", "update", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::UpdateClusterRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_clusters_update(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_locations_clusters_update_master(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "master-version" => Some(("masterVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cluster-id", "master-version", "name", "project-id", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::UpdateMasterRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_clusters_update_master(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_locations_clusters_well_known_get_openid_configuration(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_clusters_well_known_get_openid_configuration(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_locations_get_server_config(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_get_server_config(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "zone" => {
                    call = call.zone(value.unwrap_or(""));
                },
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["project-id", "zone"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_locations_operations_cancel(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "operation-id" => Some(("operationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["name", "operation-id", "project-id", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CancelOperationRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_operations_cancel(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_locations_operations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_operations_get(opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "zone" => {
                    call = call.zone(value.unwrap_or(""));
                },
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
                },
                "operation-id" => {
                    call = call.operation_id(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["project-id", "zone", "operation-id"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_locations_operations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_operations_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "zone" => {
                    call = call.zone(value.unwrap_or(""));
                },
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["project-id", "zone"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_zones_clusters_addons(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "addons-config.http-load-balancing.disabled" => Some(("addonsConfig.httpLoadBalancing.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "addons-config.network-policy-config.disabled" => Some(("addonsConfig.networkPolicyConfig.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "addons-config.kubernetes-dashboard.disabled" => Some(("addonsConfig.kubernetesDashboard.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "addons-config.horizontal-pod-autoscaling.disabled" => Some(("addonsConfig.horizontalPodAutoscaling.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["addons-config", "cluster-id", "disabled", "horizontal-pod-autoscaling", "http-load-balancing", "kubernetes-dashboard", "name", "network-policy-config", "project-id", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SetAddonsConfigRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_clusters_addons(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("zone").unwrap_or(""), opt.value_of("cluster-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_zones_clusters_complete_ip_rotation(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cluster-id", "name", "project-id", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CompleteIPRotationRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_clusters_complete_ip_rotation(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("zone").unwrap_or(""), opt.value_of("cluster-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_zones_clusters_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "cluster.node-ipv4-cidr-size" => Some(("cluster.nodeIpv4CidrSize", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.maintenance-policy.window.daily-maintenance-window.duration" => Some(("cluster.maintenancePolicy.window.dailyMaintenanceWindow.duration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.maintenance-policy.window.daily-maintenance-window.start-time" => Some(("cluster.maintenancePolicy.window.dailyMaintenanceWindow.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.network-policy.enabled" => Some(("cluster.networkPolicy.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.network-policy.provider" => Some(("cluster.networkPolicy.provider", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.master-authorized-networks-config.enabled" => Some(("cluster.masterAuthorizedNetworksConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.http-load-balancing.disabled" => Some(("cluster.addonsConfig.httpLoadBalancing.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.network-policy-config.disabled" => Some(("cluster.addonsConfig.networkPolicyConfig.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.kubernetes-dashboard.disabled" => Some(("cluster.addonsConfig.kubernetesDashboard.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.horizontal-pod-autoscaling.disabled" => Some(("cluster.addonsConfig.horizontalPodAutoscaling.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.locations" => Some(("cluster.locations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.enable-kubernetes-alpha" => Some(("cluster.enableKubernetesAlpha", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.network" => Some(("cluster.network", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.logging-service" => Some(("cluster.loggingService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.instance-group-urls" => Some(("cluster.instanceGroupUrls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.network-config.subnetwork" => Some(("cluster.networkConfig.subnetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.network-config.network" => Some(("cluster.networkConfig.network", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.location" => Some(("cluster.location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.enable-private-endpoint" => Some(("cluster.privateClusterConfig.enablePrivateEndpoint", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.master-ipv4-cidr-block" => Some(("cluster.privateClusterConfig.masterIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.private-endpoint" => Some(("cluster.privateClusterConfig.privateEndpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.public-endpoint" => Some(("cluster.privateClusterConfig.publicEndpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.enable-private-nodes" => Some(("cluster.privateClusterConfig.enablePrivateNodes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.services-ipv4-cidr" => Some(("cluster.servicesIpv4Cidr", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.status-message" => Some(("cluster.statusMessage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.node-ipv4-cidr" => Some(("cluster.ipAllocationPolicy.nodeIpv4Cidr", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.use-ip-aliases" => Some(("cluster.ipAllocationPolicy.useIpAliases", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.cluster-secondary-range-name" => Some(("cluster.ipAllocationPolicy.clusterSecondaryRangeName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.cluster-ipv4-cidr-block" => Some(("cluster.ipAllocationPolicy.clusterIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.services-ipv4-cidr-block" => Some(("cluster.ipAllocationPolicy.servicesIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.create-subnetwork" => Some(("cluster.ipAllocationPolicy.createSubnetwork", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.node-ipv4-cidr-block" => Some(("cluster.ipAllocationPolicy.nodeIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.tpu-ipv4-cidr-block" => Some(("cluster.ipAllocationPolicy.tpuIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.cluster-ipv4-cidr" => Some(("cluster.ipAllocationPolicy.clusterIpv4Cidr", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.subnetwork-name" => Some(("cluster.ipAllocationPolicy.subnetworkName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.services-secondary-range-name" => Some(("cluster.ipAllocationPolicy.servicesSecondaryRangeName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.services-ipv4-cidr" => Some(("cluster.ipAllocationPolicy.servicesIpv4Cidr", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.status" => Some(("cluster.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.description" => Some(("cluster.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.current-node-version" => Some(("cluster.currentNodeVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.label-fingerprint" => Some(("cluster.labelFingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.current-master-version" => Some(("cluster.currentMasterVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.master-auth.username" => Some(("cluster.masterAuth.username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.master-auth.client-key" => Some(("cluster.masterAuth.clientKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.master-auth.client-certificate" => Some(("cluster.masterAuth.clientCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.master-auth.client-certificate-config.issue-client-certificate" => Some(("cluster.masterAuth.clientCertificateConfig.issueClientCertificate", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.master-auth.password" => Some(("cluster.masterAuth.password", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.master-auth.cluster-ca-certificate" => Some(("cluster.masterAuth.clusterCaCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.expire-time" => Some(("cluster.expireTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.zone" => Some(("cluster.zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.initial-node-count" => Some(("cluster.initialNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.monitoring-service" => Some(("cluster.monitoringService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.create-time" => Some(("cluster.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.enable-tpu" => Some(("cluster.enableTpu", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.endpoint" => Some(("cluster.endpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.current-node-count" => Some(("cluster.currentNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.name" => Some(("cluster.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.legacy-abac.enabled" => Some(("cluster.legacyAbac.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.initial-cluster-version" => Some(("cluster.initialClusterVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.machine-type" => Some(("cluster.nodeConfig.machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.tags" => Some(("cluster.nodeConfig.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.node-config.preemptible" => Some(("cluster.nodeConfig.preemptible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.labels" => Some(("cluster.nodeConfig.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "cluster.node-config.service-account" => Some(("cluster.nodeConfig.serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.oauth-scopes" => Some(("cluster.nodeConfig.oauthScopes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.node-config.disk-type" => Some(("cluster.nodeConfig.diskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.disk-size-gb" => Some(("cluster.nodeConfig.diskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-config.min-cpu-platform" => Some(("cluster.nodeConfig.minCpuPlatform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.metadata" => Some(("cluster.nodeConfig.metadata", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "cluster.node-config.image-type" => Some(("cluster.nodeConfig.imageType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.local-ssd-count" => Some(("cluster.nodeConfig.localSsdCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.cluster-ipv4-cidr" => Some(("cluster.clusterIpv4Cidr", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.subnetwork" => Some(("cluster.subnetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.resource-labels" => Some(("cluster.resourceLabels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "cluster.self-link" => Some(("cluster.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.tpu-ipv4-cidr-block" => Some(("cluster.tpuIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent" => Some(("parent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["addons-config", "client-certificate", "client-certificate-config", "client-key", "cluster", "cluster-ca-certificate", "cluster-ipv4-cidr", "cluster-ipv4-cidr-block", "cluster-secondary-range-name", "create-subnetwork", "create-time", "current-master-version", "current-node-count", "current-node-version", "daily-maintenance-window", "description", "disabled", "disk-size-gb", "disk-type", "duration", "enable-kubernetes-alpha", "enable-private-endpoint", "enable-private-nodes", "enable-tpu", "enabled", "endpoint", "expire-time", "horizontal-pod-autoscaling", "http-load-balancing", "image-type", "initial-cluster-version", "initial-node-count", "instance-group-urls", "ip-allocation-policy", "issue-client-certificate", "kubernetes-dashboard", "label-fingerprint", "labels", "legacy-abac", "local-ssd-count", "location", "locations", "logging-service", "machine-type", "maintenance-policy", "master-auth", "master-authorized-networks-config", "master-ipv4-cidr-block", "metadata", "min-cpu-platform", "monitoring-service", "name", "network", "network-config", "network-policy", "network-policy-config", "node-config", "node-ipv4-cidr", "node-ipv4-cidr-block", "node-ipv4-cidr-size", "oauth-scopes", "parent", "password", "preemptible", "private-cluster-config", "private-endpoint", "project-id", "provider", "public-endpoint", "resource-labels", "self-link", "service-account", "services-ipv4-cidr", "services-ipv4-cidr-block", "services-secondary-range-name", "start-time", "status", "status-message", "subnetwork", "subnetwork-name", "tags", "tpu-ipv4-cidr-block", "use-ip-aliases", "username", "window", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CreateClusterRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_clusters_create(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("zone").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_zones_clusters_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().zones_clusters_delete(opt.value_of("project-id").unwrap_or(""), opt.value_of("zone").unwrap_or(""), opt.value_of("cluster-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "name" => {
                    call = call.name(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["name"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_zones_clusters_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().zones_clusters_get(opt.value_of("project-id").unwrap_or(""), opt.value_of("zone").unwrap_or(""), opt.value_of("cluster-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "name" => {
                    call = call.name(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["name"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_zones_clusters_legacy_abac(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "enabled" => Some(("enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cluster-id", "enabled", "name", "project-id", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SetLegacyAbacRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_clusters_legacy_abac(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("zone").unwrap_or(""), opt.value_of("cluster-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_zones_clusters_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().zones_clusters_list(opt.value_of("project-id").unwrap_or(""), opt.value_of("zone").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "parent" => {
                    call = call.parent(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["parent"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_zones_clusters_locations(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "locations" => Some(("locations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cluster-id", "locations", "name", "project-id", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SetLocationsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_clusters_locations(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("zone").unwrap_or(""), opt.value_of("cluster-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_zones_clusters_logging(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "logging-service" => Some(("loggingService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cluster-id", "logging-service", "name", "project-id", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SetLoggingServiceRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_clusters_logging(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("zone").unwrap_or(""), opt.value_of("cluster-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_zones_clusters_master(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "master-version" => Some(("masterVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cluster-id", "master-version", "name", "project-id", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::UpdateMasterRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_clusters_master(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("zone").unwrap_or(""), opt.value_of("cluster-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_zones_clusters_monitoring(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "monitoring-service" => Some(("monitoringService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cluster-id", "monitoring-service", "name", "project-id", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SetMonitoringServiceRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_clusters_monitoring(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("zone").unwrap_or(""), opt.value_of("cluster-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_zones_clusters_node_pools_autoscaling(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "autoscaling.min-node-count" => Some(("autoscaling.minNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "autoscaling.enabled" => Some(("autoscaling.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "autoscaling.max-node-count" => Some(("autoscaling.maxNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool-id" => Some(("nodePoolId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["autoscaling", "cluster-id", "enabled", "max-node-count", "min-node-count", "name", "node-pool-id", "project-id", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SetNodePoolAutoscalingRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_clusters_node_pools_autoscaling(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("zone").unwrap_or(""), opt.value_of("cluster-id").unwrap_or(""), opt.value_of("node-pool-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_zones_clusters_node_pools_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent" => Some(("parent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.status" => Some(("nodePool.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.management.auto-repair" => Some(("nodePool.management.autoRepair", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.management.upgrade-options.description" => Some(("nodePool.management.upgradeOptions.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.management.upgrade-options.auto-upgrade-start-time" => Some(("nodePool.management.upgradeOptions.autoUpgradeStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.management.auto-upgrade" => Some(("nodePool.management.autoUpgrade", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.name" => Some(("nodePool.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.instance-group-urls" => Some(("nodePool.instanceGroupUrls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "node-pool.autoscaling.min-node-count" => Some(("nodePool.autoscaling.minNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.autoscaling.enabled" => Some(("nodePool.autoscaling.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.autoscaling.max-node-count" => Some(("nodePool.autoscaling.maxNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.version" => Some(("nodePool.version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.initial-node-count" => Some(("nodePool.initialNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.status-message" => Some(("nodePool.statusMessage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.machine-type" => Some(("nodePool.config.machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.tags" => Some(("nodePool.config.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "node-pool.config.preemptible" => Some(("nodePool.config.preemptible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.labels" => Some(("nodePool.config.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "node-pool.config.service-account" => Some(("nodePool.config.serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.oauth-scopes" => Some(("nodePool.config.oauthScopes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "node-pool.config.disk-type" => Some(("nodePool.config.diskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.disk-size-gb" => Some(("nodePool.config.diskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.min-cpu-platform" => Some(("nodePool.config.minCpuPlatform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.metadata" => Some(("nodePool.config.metadata", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "node-pool.config.image-type" => Some(("nodePool.config.imageType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.local-ssd-count" => Some(("nodePool.config.localSsdCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.self-link" => Some(("nodePool.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["auto-repair", "auto-upgrade", "auto-upgrade-start-time", "autoscaling", "cluster-id", "config", "description", "disk-size-gb", "disk-type", "enabled", "image-type", "initial-node-count", "instance-group-urls", "labels", "local-ssd-count", "machine-type", "management", "max-node-count", "metadata", "min-cpu-platform", "min-node-count", "name", "node-pool", "oauth-scopes", "parent", "preemptible", "project-id", "self-link", "service-account", "status", "status-message", "tags", "upgrade-options", "version", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CreateNodePoolRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_clusters_node_pools_create(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("zone").unwrap_or(""), opt.value_of("cluster-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_zones_clusters_node_pools_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().zones_clusters_node_pools_delete(opt.value_of("project-id").unwrap_or(""), opt.value_of("zone").unwrap_or(""), opt.value_of("cluster-id").unwrap_or(""), opt.value_of("node-pool-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "name" => {
                    call = call.name(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["name"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_zones_clusters_node_pools_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().zones_clusters_node_pools_get(opt.value_of("project-id").unwrap_or(""), opt.value_of("zone").unwrap_or(""), opt.value_of("cluster-id").unwrap_or(""), opt.value_of("node-pool-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "name" => {
                    call = call.name(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["name"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_zones_clusters_node_pools_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().zones_clusters_node_pools_list(opt.value_of("project-id").unwrap_or(""), opt.value_of("zone").unwrap_or(""), opt.value_of("cluster-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "parent" => {
                    call = call.parent(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["parent"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_zones_clusters_node_pools_rollback(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool-id" => Some(("nodePoolId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cluster-id", "name", "node-pool-id", "project-id", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::RollbackNodePoolUpgradeRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_clusters_node_pools_rollback(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("zone").unwrap_or(""), opt.value_of("cluster-id").unwrap_or(""), opt.value_of("node-pool-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_zones_clusters_node_pools_set_management(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "management.auto-repair" => Some(("management.autoRepair", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "management.upgrade-options.description" => Some(("management.upgradeOptions.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "management.upgrade-options.auto-upgrade-start-time" => Some(("management.upgradeOptions.autoUpgradeStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "management.auto-upgrade" => Some(("management.autoUpgrade", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool-id" => Some(("nodePoolId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["auto-repair", "auto-upgrade", "auto-upgrade-start-time", "cluster-id", "description", "management", "name", "node-pool-id", "project-id", "upgrade-options", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SetNodePoolManagementRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_clusters_node_pools_set_management(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("zone").unwrap_or(""), opt.value_of("cluster-id").unwrap_or(""), opt.value_of("node-pool-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_zones_clusters_node_pools_set_size(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool-id" => Some(("nodePoolId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-count" => Some(("nodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cluster-id", "name", "node-count", "node-pool-id", "project-id", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SetNodePoolSizeRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_clusters_node_pools_set_size(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("zone").unwrap_or(""), opt.value_of("cluster-id").unwrap_or(""), opt.value_of("node-pool-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_zones_clusters_node_pools_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool-id" => Some(("nodePoolId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-version" => Some(("nodeVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "image-type" => Some(("imageType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cluster-id", "image-type", "name", "node-pool-id", "node-version", "project-id", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::UpdateNodePoolRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_clusters_node_pools_update(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("zone").unwrap_or(""), opt.value_of("cluster-id").unwrap_or(""), opt.value_of("node-pool-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_zones_clusters_resource_labels(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "label-fingerprint" => Some(("labelFingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "resource-labels" => Some(("resourceLabels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cluster-id", "label-fingerprint", "name", "project-id", "resource-labels", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SetLabelsRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_clusters_resource_labels(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("zone").unwrap_or(""), opt.value_of("cluster-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_zones_clusters_set_maintenance_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "maintenance-policy.window.daily-maintenance-window.duration" => Some(("maintenancePolicy.window.dailyMaintenanceWindow.duration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "maintenance-policy.window.daily-maintenance-window.start-time" => Some(("maintenancePolicy.window.dailyMaintenanceWindow.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cluster-id", "daily-maintenance-window", "duration", "maintenance-policy", "name", "project-id", "start-time", "window", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SetMaintenancePolicyRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_clusters_set_maintenance_policy(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("zone").unwrap_or(""), opt.value_of("cluster-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_zones_clusters_set_master_auth(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.username" => Some(("update.username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.client-key" => Some(("update.clientKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.client-certificate" => Some(("update.clientCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.client-certificate-config.issue-client-certificate" => Some(("update.clientCertificateConfig.issueClientCertificate", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.password" => Some(("update.password", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.cluster-ca-certificate" => Some(("update.clusterCaCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "action" => Some(("action", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["action", "client-certificate", "client-certificate-config", "client-key", "cluster-ca-certificate", "cluster-id", "issue-client-certificate", "name", "password", "project-id", "update", "username", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SetMasterAuthRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_clusters_set_master_auth(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("zone").unwrap_or(""), opt.value_of("cluster-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_zones_clusters_set_network_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "network-policy.enabled" => Some(("networkPolicy.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "network-policy.provider" => Some(("networkPolicy.provider", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cluster-id", "enabled", "name", "network-policy", "project-id", "provider", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SetNetworkPolicyRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_clusters_set_network_policy(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("zone").unwrap_or(""), opt.value_of("cluster-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_zones_clusters_start_ip_rotation(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rotate-credentials" => Some(("rotateCredentials", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cluster-id", "name", "project-id", "rotate-credentials", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::StartIPRotationRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_clusters_start_ip_rotation(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("zone").unwrap_or(""), opt.value_of("cluster-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_zones_clusters_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-master-authorized-networks-config.enabled" => Some(("update.desiredMasterAuthorizedNetworksConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-id" => Some(("update.desiredNodePoolId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.http-load-balancing.disabled" => Some(("update.desiredAddonsConfig.httpLoadBalancing.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.network-policy-config.disabled" => Some(("update.desiredAddonsConfig.networkPolicyConfig.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.kubernetes-dashboard.disabled" => Some(("update.desiredAddonsConfig.kubernetesDashboard.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.horizontal-pod-autoscaling.disabled" => Some(("update.desiredAddonsConfig.horizontalPodAutoscaling.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-master-version" => Some(("update.desiredMasterVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-locations" => Some(("update.desiredLocations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.desired-node-version" => Some(("update.desiredNodeVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-image-type" => Some(("update.desiredImageType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-autoscaling.min-node-count" => Some(("update.desiredNodePoolAutoscaling.minNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-autoscaling.enabled" => Some(("update.desiredNodePoolAutoscaling.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-autoscaling.max-node-count" => Some(("update.desiredNodePoolAutoscaling.maxNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-monitoring-service" => Some(("update.desiredMonitoringService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cluster-id", "desired-addons-config", "desired-image-type", "desired-locations", "desired-master-authorized-networks-config", "desired-master-version", "desired-monitoring-service", "desired-node-pool-autoscaling", "desired-node-pool-id", "desired-node-version", "disabled", "enabled", "horizontal-pod-autoscaling", "http-load-balancing", "kubernetes-dashboard", "max-node-count", "min-node-count", "name", "network-policy-config", "project-id", "update", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::UpdateClusterRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_clusters_update(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("zone").unwrap_or(""), opt.value_of("cluster-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_zones_get_serverconfig(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().zones_get_serverconfig(opt.value_of("project-id").unwrap_or(""), opt.value_of("zone").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "name" => {
                    call = call.name(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["name"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_zones_operations_cancel(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
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
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "operation-id" => Some(("operationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["name", "operation-id", "project-id", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CancelOperationRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_operations_cancel(request, opt.value_of("project-id").unwrap_or(""), opt.value_of("zone").unwrap_or(""), opt.value_of("operation-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_zones_operations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().zones_operations_get(opt.value_of("project-id").unwrap_or(""), opt.value_of("zone").unwrap_or(""), opt.value_of("operation-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "name" => {
                    call = call.name(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["name"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _projects_zones_operations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().zones_operations_list(opt.value_of("project-id").unwrap_or(""), opt.value_of("zone").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "parent" => {
                    call = call.parent(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["parent"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _doit(&self, dry_run: bool) -> Result<Result<(), DoitError>, Option<InvalidOptionsError>> {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Result<(), DoitError> = Ok(());
        let mut err_opt: Option<InvalidOptionsError> = None;
        match self.opt.subcommand() {
            ("projects", Some(opt)) => {
                match opt.subcommand() {
                    ("locations-clusters-complete-ip-rotation", Some(opt)) => {
                        call_result = self._projects_locations_clusters_complete_ip_rotation(opt, dry_run, &mut err);
                    },
                    ("locations-clusters-create", Some(opt)) => {
                        call_result = self._projects_locations_clusters_create(opt, dry_run, &mut err);
                    },
                    ("locations-clusters-delete", Some(opt)) => {
                        call_result = self._projects_locations_clusters_delete(opt, dry_run, &mut err);
                    },
                    ("locations-clusters-get", Some(opt)) => {
                        call_result = self._projects_locations_clusters_get(opt, dry_run, &mut err);
                    },
                    ("locations-clusters-get-jwks", Some(opt)) => {
                        call_result = self._projects_locations_clusters_get_jwks(opt, dry_run, &mut err);
                    },
                    ("locations-clusters-list", Some(opt)) => {
                        call_result = self._projects_locations_clusters_list(opt, dry_run, &mut err);
                    },
                    ("locations-clusters-node-pools-create", Some(opt)) => {
                        call_result = self._projects_locations_clusters_node_pools_create(opt, dry_run, &mut err);
                    },
                    ("locations-clusters-node-pools-delete", Some(opt)) => {
                        call_result = self._projects_locations_clusters_node_pools_delete(opt, dry_run, &mut err);
                    },
                    ("locations-clusters-node-pools-get", Some(opt)) => {
                        call_result = self._projects_locations_clusters_node_pools_get(opt, dry_run, &mut err);
                    },
                    ("locations-clusters-node-pools-list", Some(opt)) => {
                        call_result = self._projects_locations_clusters_node_pools_list(opt, dry_run, &mut err);
                    },
                    ("locations-clusters-node-pools-rollback", Some(opt)) => {
                        call_result = self._projects_locations_clusters_node_pools_rollback(opt, dry_run, &mut err);
                    },
                    ("locations-clusters-node-pools-set-autoscaling", Some(opt)) => {
                        call_result = self._projects_locations_clusters_node_pools_set_autoscaling(opt, dry_run, &mut err);
                    },
                    ("locations-clusters-node-pools-set-management", Some(opt)) => {
                        call_result = self._projects_locations_clusters_node_pools_set_management(opt, dry_run, &mut err);
                    },
                    ("locations-clusters-node-pools-set-size", Some(opt)) => {
                        call_result = self._projects_locations_clusters_node_pools_set_size(opt, dry_run, &mut err);
                    },
                    ("locations-clusters-node-pools-update", Some(opt)) => {
                        call_result = self._projects_locations_clusters_node_pools_update(opt, dry_run, &mut err);
                    },
                    ("locations-clusters-set-addons", Some(opt)) => {
                        call_result = self._projects_locations_clusters_set_addons(opt, dry_run, &mut err);
                    },
                    ("locations-clusters-set-legacy-abac", Some(opt)) => {
                        call_result = self._projects_locations_clusters_set_legacy_abac(opt, dry_run, &mut err);
                    },
                    ("locations-clusters-set-locations", Some(opt)) => {
                        call_result = self._projects_locations_clusters_set_locations(opt, dry_run, &mut err);
                    },
                    ("locations-clusters-set-logging", Some(opt)) => {
                        call_result = self._projects_locations_clusters_set_logging(opt, dry_run, &mut err);
                    },
                    ("locations-clusters-set-maintenance-policy", Some(opt)) => {
                        call_result = self._projects_locations_clusters_set_maintenance_policy(opt, dry_run, &mut err);
                    },
                    ("locations-clusters-set-master-auth", Some(opt)) => {
                        call_result = self._projects_locations_clusters_set_master_auth(opt, dry_run, &mut err);
                    },
                    ("locations-clusters-set-monitoring", Some(opt)) => {
                        call_result = self._projects_locations_clusters_set_monitoring(opt, dry_run, &mut err);
                    },
                    ("locations-clusters-set-network-policy", Some(opt)) => {
                        call_result = self._projects_locations_clusters_set_network_policy(opt, dry_run, &mut err);
                    },
                    ("locations-clusters-set-resource-labels", Some(opt)) => {
                        call_result = self._projects_locations_clusters_set_resource_labels(opt, dry_run, &mut err);
                    },
                    ("locations-clusters-start-ip-rotation", Some(opt)) => {
                        call_result = self._projects_locations_clusters_start_ip_rotation(opt, dry_run, &mut err);
                    },
                    ("locations-clusters-update", Some(opt)) => {
                        call_result = self._projects_locations_clusters_update(opt, dry_run, &mut err);
                    },
                    ("locations-clusters-update-master", Some(opt)) => {
                        call_result = self._projects_locations_clusters_update_master(opt, dry_run, &mut err);
                    },
                    ("locations-clusters-well-known-get-openid-configuration", Some(opt)) => {
                        call_result = self._projects_locations_clusters_well_known_get_openid_configuration(opt, dry_run, &mut err);
                    },
                    ("locations-get-server-config", Some(opt)) => {
                        call_result = self._projects_locations_get_server_config(opt, dry_run, &mut err);
                    },
                    ("locations-operations-cancel", Some(opt)) => {
                        call_result = self._projects_locations_operations_cancel(opt, dry_run, &mut err);
                    },
                    ("locations-operations-get", Some(opt)) => {
                        call_result = self._projects_locations_operations_get(opt, dry_run, &mut err);
                    },
                    ("locations-operations-list", Some(opt)) => {
                        call_result = self._projects_locations_operations_list(opt, dry_run, &mut err);
                    },
                    ("zones-clusters-addons", Some(opt)) => {
                        call_result = self._projects_zones_clusters_addons(opt, dry_run, &mut err);
                    },
                    ("zones-clusters-complete-ip-rotation", Some(opt)) => {
                        call_result = self._projects_zones_clusters_complete_ip_rotation(opt, dry_run, &mut err);
                    },
                    ("zones-clusters-create", Some(opt)) => {
                        call_result = self._projects_zones_clusters_create(opt, dry_run, &mut err);
                    },
                    ("zones-clusters-delete", Some(opt)) => {
                        call_result = self._projects_zones_clusters_delete(opt, dry_run, &mut err);
                    },
                    ("zones-clusters-get", Some(opt)) => {
                        call_result = self._projects_zones_clusters_get(opt, dry_run, &mut err);
                    },
                    ("zones-clusters-legacy-abac", Some(opt)) => {
                        call_result = self._projects_zones_clusters_legacy_abac(opt, dry_run, &mut err);
                    },
                    ("zones-clusters-list", Some(opt)) => {
                        call_result = self._projects_zones_clusters_list(opt, dry_run, &mut err);
                    },
                    ("zones-clusters-locations", Some(opt)) => {
                        call_result = self._projects_zones_clusters_locations(opt, dry_run, &mut err);
                    },
                    ("zones-clusters-logging", Some(opt)) => {
                        call_result = self._projects_zones_clusters_logging(opt, dry_run, &mut err);
                    },
                    ("zones-clusters-master", Some(opt)) => {
                        call_result = self._projects_zones_clusters_master(opt, dry_run, &mut err);
                    },
                    ("zones-clusters-monitoring", Some(opt)) => {
                        call_result = self._projects_zones_clusters_monitoring(opt, dry_run, &mut err);
                    },
                    ("zones-clusters-node-pools-autoscaling", Some(opt)) => {
                        call_result = self._projects_zones_clusters_node_pools_autoscaling(opt, dry_run, &mut err);
                    },
                    ("zones-clusters-node-pools-create", Some(opt)) => {
                        call_result = self._projects_zones_clusters_node_pools_create(opt, dry_run, &mut err);
                    },
                    ("zones-clusters-node-pools-delete", Some(opt)) => {
                        call_result = self._projects_zones_clusters_node_pools_delete(opt, dry_run, &mut err);
                    },
                    ("zones-clusters-node-pools-get", Some(opt)) => {
                        call_result = self._projects_zones_clusters_node_pools_get(opt, dry_run, &mut err);
                    },
                    ("zones-clusters-node-pools-list", Some(opt)) => {
                        call_result = self._projects_zones_clusters_node_pools_list(opt, dry_run, &mut err);
                    },
                    ("zones-clusters-node-pools-rollback", Some(opt)) => {
                        call_result = self._projects_zones_clusters_node_pools_rollback(opt, dry_run, &mut err);
                    },
                    ("zones-clusters-node-pools-set-management", Some(opt)) => {
                        call_result = self._projects_zones_clusters_node_pools_set_management(opt, dry_run, &mut err);
                    },
                    ("zones-clusters-node-pools-set-size", Some(opt)) => {
                        call_result = self._projects_zones_clusters_node_pools_set_size(opt, dry_run, &mut err);
                    },
                    ("zones-clusters-node-pools-update", Some(opt)) => {
                        call_result = self._projects_zones_clusters_node_pools_update(opt, dry_run, &mut err);
                    },
                    ("zones-clusters-resource-labels", Some(opt)) => {
                        call_result = self._projects_zones_clusters_resource_labels(opt, dry_run, &mut err);
                    },
                    ("zones-clusters-set-maintenance-policy", Some(opt)) => {
                        call_result = self._projects_zones_clusters_set_maintenance_policy(opt, dry_run, &mut err);
                    },
                    ("zones-clusters-set-master-auth", Some(opt)) => {
                        call_result = self._projects_zones_clusters_set_master_auth(opt, dry_run, &mut err);
                    },
                    ("zones-clusters-set-network-policy", Some(opt)) => {
                        call_result = self._projects_zones_clusters_set_network_policy(opt, dry_run, &mut err);
                    },
                    ("zones-clusters-start-ip-rotation", Some(opt)) => {
                        call_result = self._projects_zones_clusters_start_ip_rotation(opt, dry_run, &mut err);
                    },
                    ("zones-clusters-update", Some(opt)) => {
                        call_result = self._projects_zones_clusters_update(opt, dry_run, &mut err);
                    },
                    ("zones-get-serverconfig", Some(opt)) => {
                        call_result = self._projects_zones_get_serverconfig(opt, dry_run, &mut err);
                    },
                    ("zones-operations-cancel", Some(opt)) => {
                        call_result = self._projects_zones_operations_cancel(opt, dry_run, &mut err);
                    },
                    ("zones-operations-get", Some(opt)) => {
                        call_result = self._projects_zones_operations_get(opt, dry_run, &mut err);
                    },
                    ("zones-operations-list", Some(opt)) => {
                        call_result = self._projects_zones_operations_list(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("projects".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            _ => {
                err.issues.push(CLIError::MissingCommandError);
                writeln!(io::stderr(), "{}\n", self.opt.usage()).ok();
            }
        }

        if dry_run {
            if err.issues.len() > 0 {
                err_opt = Some(err);
            }
            Err(err_opt)
        } else {
            Ok(call_result)
        }
    }

    // Please note that this call will fail if any part of the opt can't be handled
    fn new(opt: ArgMatches<'n>) -> Result<Engine<'n>, InvalidOptionsError> {
        let (config_dir, secret) = {
            let config_dir = match cmn::assure_config_dir_exists(opt.value_of("folder").unwrap_or("~/.google-service-cli")) {
                Err(e) => return Err(InvalidOptionsError::single(e, 3)),
                Ok(p) => p,
            };

            match cmn::application_secret_from_directory(&config_dir, "container1-secret.json",
                                                         "{\"installed\":{\"auth_uri\":\"https://accounts.google.com/o/oauth2/auth\",\"client_secret\":\"hCsslbCUyfehWMmbkG8vTYxG\",\"token_uri\":\"https://accounts.google.com/o/oauth2/token\",\"client_email\":\"\",\"redirect_uris\":[\"urn:ietf:wg:oauth:2.0:oob\",\"oob\"],\"client_x509_cert_url\":\"\",\"client_id\":\"620010449518-9ngf7o4dhs0dka470npqvor6dc5lqb9b.apps.googleusercontent.com\",\"auth_provider_x509_cert_url\":\"https://www.googleapis.com/oauth2/v1/certs\"}}") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let auth = Authenticator::new(  &secret, DefaultAuthenticatorDelegate,
                                        if opt.is_present("debug-auth") {
                                            hyper::Client::with_connector(mock::TeeConnector {
                                                    connector: hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())
                                                })
                                        } else {
                                            hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new()))
                                        },
                                        JsonTokenStorage {
                                          program_name: "container1",
                                          db_dir: config_dir.clone(),
                                        }, Some(FlowType::InstalledRedirect(54324)));

        let client =
            if opt.is_present("debug") {
                hyper::Client::with_connector(mock::TeeConnector {
                        connector: hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())
                    })
            } else {
                hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new()))
            };
        let engine = Engine {
            opt: opt,
            hub: api::Container::new(client, auth),
            gp: vec!["$-xgafv", "access-token", "alt", "callback", "fields", "key", "oauth-token", "pretty-print", "quota-user", "upload-type", "upload-protocol"],
            gpm: vec![
                    ("$-xgafv", "$.xgafv"),
                    ("access-token", "access_token"),
                    ("oauth-token", "oauth_token"),
                    ("pretty-print", "prettyPrint"),
                    ("quota-user", "quotaUser"),
                    ("upload-type", "uploadType"),
                    ("upload-protocol", "upload_protocol"),
                ]
        };

        match engine._doit(true) {
            Err(Some(err)) => Err(err),
            Err(None)      => Ok(engine),
            Ok(_)          => unreachable!(),
        }
    }

    fn doit(&self) -> Result<(), DoitError> {
        match self._doit(false) {
            Ok(res) => res,
            Err(_) => unreachable!(),
        }
    }
}

fn main() {
    let mut exit_status = 0i32;
    let arg_data = [
        ("projects", "methods: 'locations-clusters-complete-ip-rotation', 'locations-clusters-create', 'locations-clusters-delete', 'locations-clusters-get', 'locations-clusters-get-jwks', 'locations-clusters-list', 'locations-clusters-node-pools-create', 'locations-clusters-node-pools-delete', 'locations-clusters-node-pools-get', 'locations-clusters-node-pools-list', 'locations-clusters-node-pools-rollback', 'locations-clusters-node-pools-set-autoscaling', 'locations-clusters-node-pools-set-management', 'locations-clusters-node-pools-set-size', 'locations-clusters-node-pools-update', 'locations-clusters-set-addons', 'locations-clusters-set-legacy-abac', 'locations-clusters-set-locations', 'locations-clusters-set-logging', 'locations-clusters-set-maintenance-policy', 'locations-clusters-set-master-auth', 'locations-clusters-set-monitoring', 'locations-clusters-set-network-policy', 'locations-clusters-set-resource-labels', 'locations-clusters-start-ip-rotation', 'locations-clusters-update', 'locations-clusters-update-master', 'locations-clusters-well-known-get-openid-configuration', 'locations-get-server-config', 'locations-operations-cancel', 'locations-operations-get', 'locations-operations-list', 'zones-clusters-addons', 'zones-clusters-complete-ip-rotation', 'zones-clusters-create', 'zones-clusters-delete', 'zones-clusters-get', 'zones-clusters-legacy-abac', 'zones-clusters-list', 'zones-clusters-locations', 'zones-clusters-logging', 'zones-clusters-master', 'zones-clusters-monitoring', 'zones-clusters-node-pools-autoscaling', 'zones-clusters-node-pools-create', 'zones-clusters-node-pools-delete', 'zones-clusters-node-pools-get', 'zones-clusters-node-pools-list', 'zones-clusters-node-pools-rollback', 'zones-clusters-node-pools-set-management', 'zones-clusters-node-pools-set-size', 'zones-clusters-node-pools-update', 'zones-clusters-resource-labels', 'zones-clusters-set-maintenance-policy', 'zones-clusters-set-master-auth', 'zones-clusters-set-network-policy', 'zones-clusters-start-ip-rotation', 'zones-clusters-update', 'zones-get-serverconfig', 'zones-operations-cancel', 'zones-operations-get' and 'zones-operations-list'", vec![
            ("locations-clusters-complete-ip-rotation",
                    Some(r##"Completes master IP rotation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-complete-ip-rotation",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name (project, location, cluster id) of the cluster to complete IP
        rotation. Specified in the format 'projects/*/locations/*/clusters/*'."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("locations-clusters-create",
                    Some(r##"Creates a cluster, consisting of the specified number and type of Google
        Compute Engine instances.
        
        By default, the cluster is created in the project's
        [default network](/compute/docs/networks-and-firewalls#networks).
        
        One firewall is added for the cluster. After cluster creation,
        the cluster creates routes for each node to allow the containers
        on that node to communicate with all other instances in the
        cluster.
        
        Finally, an entry is added to the project's global metadata indicating
        which CIDR range is being used by the cluster."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The parent (project and location) where the cluster will be created.
        Specified in the format 'projects/*/locations/*'."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("locations-clusters-delete",
                    Some(r##"Deletes the cluster, including the Kubernetes endpoint and all worker
        nodes.
        
        Firewalls and routes that were configured during cluster creation
        are also deleted.
        
        Other Google Compute Engine resources that might be in use by the cluster
        (e.g. load balancer resources) will not be deleted if they weren't present
        at the initial create time."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name (project, location, cluster) of the cluster to delete.
        Specified in the format 'projects/*/locations/*/clusters/*'."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("locations-clusters-get",
                    Some(r##"Gets the details of a specific cluster."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name (project, location, cluster) of the cluster to retrieve.
        Specified in the format 'projects/*/locations/*/clusters/*'."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("locations-clusters-get-jwks",
                    Some(r##"GetJSONWebKeys gets the public component of the cluster signing keys in
        JSON Web Key format.
        This API is not yet intended for general use, and is not available for all
        clusters."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-get-jwks",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The cluster (project, location, cluster id) to get keys for. Specified in
        the format 'projects/*/locations/*/clusters/*'."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("locations-clusters-list",
                    Some(r##"Lists all clusters owned by a project in either the specified zone or all
        zones."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The parent (project and location) where the clusters will be listed.
        Specified in the format 'projects/*/locations/*'.
        Location "-" matches all zones and all regions."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("locations-clusters-node-pools-create",
                    Some(r##"Creates a node pool for a cluster."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-node-pools-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The parent (project, location, cluster id) where the node pool will be
        created. Specified in the format
        'projects/*/locations/*/clusters/*'."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("locations-clusters-node-pools-delete",
                    Some(r##"Deletes a node pool from a cluster."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-node-pools-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name (project, location, cluster, node pool id) of the node pool to
        delete. Specified in the format
        'projects/*/locations/*/clusters/*/nodePools/*'."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("locations-clusters-node-pools-get",
                    Some(r##"Retrieves the node pool requested."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-node-pools-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name (project, location, cluster, node pool id) of the node pool to
        get. Specified in the format
        'projects/*/locations/*/clusters/*/nodePools/*'."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("locations-clusters-node-pools-list",
                    Some(r##"Lists the node pools for a cluster."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-node-pools-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The parent (project, location, cluster id) where the node pools will be
        listed. Specified in the format 'projects/*/locations/*/clusters/*'."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("locations-clusters-node-pools-rollback",
                    Some(r##"Roll back the previously Aborted or Failed NodePool upgrade.
        This will be an no-op if the last upgrade successfully completed."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-node-pools-rollback",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name (project, location, cluster, node pool id) of the node poll to
        rollback upgrade.
        Specified in the format 'projects/*/locations/*/clusters/*/nodePools/*'."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("locations-clusters-node-pools-set-autoscaling",
                    Some(r##"Sets the autoscaling settings for a specific node pool."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-node-pools-set-autoscaling",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name (project, location, cluster, node pool) of the node pool to set
        autoscaler settings. Specified in the format
        'projects/*/locations/*/clusters/*/nodePools/*'."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("locations-clusters-node-pools-set-management",
                    Some(r##"Sets the NodeManagement options for a node pool."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-node-pools-set-management",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name (project, location, cluster, node pool id) of the node pool to set
        management properties. Specified in the format
        'projects/*/locations/*/clusters/*/nodePools/*'."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("locations-clusters-node-pools-set-size",
                    Some(r##"Sets the size for a specific node pool."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-node-pools-set-size",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name (project, location, cluster, node pool id) of the node pool to set
        size.
        Specified in the format 'projects/*/locations/*/clusters/*/nodePools/*'."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("locations-clusters-node-pools-update",
                    Some(r##"Updates the version and/or image type for a specific node pool."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-node-pools-update",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name (project, location, cluster, node pool) of the node pool to
        update. Specified in the format
        'projects/*/locations/*/clusters/*/nodePools/*'."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("locations-clusters-set-addons",
                    Some(r##"Sets the addons for a specific cluster."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-set-addons",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name (project, location, cluster) of the cluster to set addons.
        Specified in the format 'projects/*/locations/*/clusters/*'."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("locations-clusters-set-legacy-abac",
                    Some(r##"Enables or disables the ABAC authorization mechanism on a cluster."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-set-legacy-abac",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name (project, location, cluster id) of the cluster to set legacy abac.
        Specified in the format 'projects/*/locations/*/clusters/*'."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("locations-clusters-set-locations",
                    Some(r##"Sets the locations for a specific cluster."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-set-locations",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name (project, location, cluster) of the cluster to set locations.
        Specified in the format 'projects/*/locations/*/clusters/*'."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("locations-clusters-set-logging",
                    Some(r##"Sets the logging service for a specific cluster."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-set-logging",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name (project, location, cluster) of the cluster to set logging.
        Specified in the format 'projects/*/locations/*/clusters/*'."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("locations-clusters-set-maintenance-policy",
                    Some(r##"Sets the maintenance policy for a cluster."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-set-maintenance-policy",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name (project, location, cluster id) of the cluster to set maintenance
        policy.
        Specified in the format 'projects/*/locations/*/clusters/*'."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("locations-clusters-set-master-auth",
                    Some(r##"Used to set master auth materials. Currently supports :-
        Changing the admin password for a specific cluster.
        This can be either via password generation or explicitly set the password."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-set-master-auth",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name (project, location, cluster) of the cluster to set auth.
        Specified in the format 'projects/*/locations/*/clusters/*'."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("locations-clusters-set-monitoring",
                    Some(r##"Sets the monitoring service for a specific cluster."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-set-monitoring",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name (project, location, cluster) of the cluster to set monitoring.
        Specified in the format 'projects/*/locations/*/clusters/*'."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("locations-clusters-set-network-policy",
                    Some(r##"Enables/Disables Network Policy for a cluster."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-set-network-policy",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name (project, location, cluster id) of the cluster to set networking
        policy. Specified in the format 'projects/*/locations/*/clusters/*'."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("locations-clusters-set-resource-labels",
                    Some(r##"Sets labels on a cluster."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-set-resource-labels",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name (project, location, cluster id) of the cluster to set labels.
        Specified in the format 'projects/*/locations/*/clusters/*'."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("locations-clusters-start-ip-rotation",
                    Some(r##"Start master IP rotation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-start-ip-rotation",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name (project, location, cluster id) of the cluster to start IP
        rotation. Specified in the format 'projects/*/locations/*/clusters/*'."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("locations-clusters-update",
                    Some(r##"Updates the settings of a specific cluster."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-update",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name (project, location, cluster) of the cluster to update.
        Specified in the format 'projects/*/locations/*/clusters/*'."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("locations-clusters-update-master",
                    Some(r##"Updates the master for a specific cluster."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-update-master",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name (project, location, cluster) of the cluster to update.
        Specified in the format 'projects/*/locations/*/clusters/*'."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("locations-clusters-well-known-get-openid-configuration",
                    Some(r##"GetOpenIDConfig gets the OIDC discovery document for the cluster.
        See the OpenID Connect Discovery 1.0 specification for details.
        https://openid.net/specs/openid-connect-discovery-1_0.html
        This API is not yet intended for general use, and is not available for all
        clusters."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-well-known-get-openid-configuration",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The cluster (project, location, cluster id) to get the discovery document
        for. Specified in the format 'projects/*/locations/*/clusters/*'."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("locations-get-server-config",
                    Some(r##"Returns configuration info about the Kubernetes Engine service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-get-server-config",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name (project and location) of the server config to get,
        specified in the format 'projects/*/locations/*'."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("locations-operations-cancel",
                    Some(r##"Cancels the specified operation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-operations-cancel",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name (project, location, operation id) of the operation to cancel.
        Specified in the format 'projects/*/locations/*/operations/*'."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("locations-operations-get",
                    Some(r##"Gets the specified operation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-operations-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name (project, location, operation id) of the operation to get.
        Specified in the format 'projects/*/locations/*/operations/*'."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("locations-operations-list",
                    Some(r##"Lists all operations in a project in a specific zone or all zones."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-operations-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The parent (project and location) where the operations will be listed.
        Specified in the format 'projects/*/locations/*'.
        Location "-" matches all zones and all regions."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("zones-clusters-addons",
                    Some(r##"Sets the addons for a specific cluster."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-clusters-addons",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project
        number](https://support.google.com/cloud/answer/6158840).
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine
        [zone](/compute/docs/zones#available) in which the cluster
        resides.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster to upgrade.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("zones-clusters-complete-ip-rotation",
                    Some(r##"Completes master IP rotation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-clusters-complete-ip-rotation",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project
        number](https://developers.google.com/console/help/new/#projectnumber).
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine
        [zone](/compute/docs/zones#available) in which the cluster
        resides.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("zones-clusters-create",
                    Some(r##"Creates a cluster, consisting of the specified number and type of Google
        Compute Engine instances.
        
        By default, the cluster is created in the project's
        [default network](/compute/docs/networks-and-firewalls#networks).
        
        One firewall is added for the cluster. After cluster creation,
        the cluster creates routes for each node to allow the containers
        on that node to communicate with all other instances in the
        cluster.
        
        Finally, an entry is added to the project's global metadata indicating
        which CIDR range is being used by the cluster."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-clusters-create",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project
        number](https://support.google.com/cloud/answer/6158840).
        This field has been deprecated and replaced by the parent field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine
        [zone](/compute/docs/zones#available) in which the cluster
        resides.
        This field has been deprecated and replaced by the parent field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("zones-clusters-delete",
                    Some(r##"Deletes the cluster, including the Kubernetes endpoint and all worker
        nodes.
        
        Firewalls and routes that were configured during cluster creation
        are also deleted.
        
        Other Google Compute Engine resources that might be in use by the cluster
        (e.g. load balancer resources) will not be deleted if they weren't present
        at the initial create time."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-clusters-delete",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project
        number](https://support.google.com/cloud/answer/6158840).
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine
        [zone](/compute/docs/zones#available) in which the cluster
        resides.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster to delete.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("zones-clusters-get",
                    Some(r##"Gets the details of a specific cluster."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-clusters-get",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project
        number](https://support.google.com/cloud/answer/6158840).
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine
        [zone](/compute/docs/zones#available) in which the cluster
        resides.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster to retrieve.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("zones-clusters-legacy-abac",
                    Some(r##"Enables or disables the ABAC authorization mechanism on a cluster."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-clusters-legacy-abac",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project
        number](https://support.google.com/cloud/answer/6158840).
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine
        [zone](/compute/docs/zones#available) in which the cluster
        resides.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster to update.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("zones-clusters-list",
                    Some(r##"Lists all clusters owned by a project in either the specified zone or all
        zones."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-clusters-list",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project
        number](https://support.google.com/cloud/answer/6158840).
        This field has been deprecated and replaced by the parent field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine
        [zone](/compute/docs/zones#available) in which the cluster
        resides, or "-" for all zones.
        This field has been deprecated and replaced by the parent field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("zones-clusters-locations",
                    Some(r##"Sets the locations for a specific cluster."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-clusters-locations",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project
        number](https://support.google.com/cloud/answer/6158840).
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine
        [zone](/compute/docs/zones#available) in which the cluster
        resides.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster to upgrade.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("zones-clusters-logging",
                    Some(r##"Sets the logging service for a specific cluster."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-clusters-logging",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project
        number](https://support.google.com/cloud/answer/6158840).
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine
        [zone](/compute/docs/zones#available) in which the cluster
        resides.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster to upgrade.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("zones-clusters-master",
                    Some(r##"Updates the master for a specific cluster."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-clusters-master",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project
        number](https://support.google.com/cloud/answer/6158840).
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine
        [zone](/compute/docs/zones#available) in which the cluster
        resides.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster to upgrade.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("zones-clusters-monitoring",
                    Some(r##"Sets the monitoring service for a specific cluster."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-clusters-monitoring",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project
        number](https://support.google.com/cloud/answer/6158840).
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine
        [zone](/compute/docs/zones#available) in which the cluster
        resides.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster to upgrade.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("zones-clusters-node-pools-autoscaling",
                    Some(r##"Sets the autoscaling settings for a specific node pool."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-clusters-node-pools-autoscaling",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project
        number](https://support.google.com/cloud/answer/6158840).
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine
        [zone](/compute/docs/zones#available) in which the cluster
        resides.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster to upgrade.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"node-pool-id"##),
                     None,
                     Some(r##"Deprecated. The name of the node pool to upgrade.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("zones-clusters-node-pools-create",
                    Some(r##"Creates a node pool for a cluster."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-clusters-node-pools-create",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project
        number](https://developers.google.com/console/help/new/#projectnumber).
        This field has been deprecated and replaced by the parent field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine
        [zone](/compute/docs/zones#available) in which the cluster
        resides.
        This field has been deprecated and replaced by the parent field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster.
        This field has been deprecated and replaced by the parent field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("zones-clusters-node-pools-delete",
                    Some(r##"Deletes a node pool from a cluster."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-clusters-node-pools-delete",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project
        number](https://developers.google.com/console/help/new/#projectnumber).
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine
        [zone](/compute/docs/zones#available) in which the cluster
        resides.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"node-pool-id"##),
                     None,
                     Some(r##"Deprecated. The name of the node pool to delete.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("zones-clusters-node-pools-get",
                    Some(r##"Retrieves the node pool requested."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-clusters-node-pools-get",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project
        number](https://developers.google.com/console/help/new/#projectnumber).
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine
        [zone](/compute/docs/zones#available) in which the cluster
        resides.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"node-pool-id"##),
                     None,
                     Some(r##"Deprecated. The name of the node pool.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("zones-clusters-node-pools-list",
                    Some(r##"Lists the node pools for a cluster."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-clusters-node-pools-list",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project
        number](https://developers.google.com/console/help/new/#projectnumber).
        This field has been deprecated and replaced by the parent field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine
        [zone](/compute/docs/zones#available) in which the cluster
        resides.
        This field has been deprecated and replaced by the parent field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster.
        This field has been deprecated and replaced by the parent field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("zones-clusters-node-pools-rollback",
                    Some(r##"Roll back the previously Aborted or Failed NodePool upgrade.
        This will be an no-op if the last upgrade successfully completed."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-clusters-node-pools-rollback",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project
        number](https://support.google.com/cloud/answer/6158840).
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine
        [zone](/compute/docs/zones#available) in which the cluster
        resides.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster to rollback.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"node-pool-id"##),
                     None,
                     Some(r##"Deprecated. The name of the node pool to rollback.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("zones-clusters-node-pools-set-management",
                    Some(r##"Sets the NodeManagement options for a node pool."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-clusters-node-pools-set-management",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project
        number](https://support.google.com/cloud/answer/6158840).
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine
        [zone](/compute/docs/zones#available) in which the cluster
        resides.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster to update.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"node-pool-id"##),
                     None,
                     Some(r##"Deprecated. The name of the node pool to update.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("zones-clusters-node-pools-set-size",
                    Some(r##"Sets the size for a specific node pool."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-clusters-node-pools-set-size",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project
        number](https://support.google.com/cloud/answer/6158840).
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine
        [zone](/compute/docs/zones#available) in which the cluster
        resides.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster to update.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"node-pool-id"##),
                     None,
                     Some(r##"Deprecated. The name of the node pool to update.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("zones-clusters-node-pools-update",
                    Some(r##"Updates the version and/or image type for a specific node pool."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-clusters-node-pools-update",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project
        number](https://support.google.com/cloud/answer/6158840).
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine
        [zone](/compute/docs/zones#available) in which the cluster
        resides.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster to upgrade.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"node-pool-id"##),
                     None,
                     Some(r##"Deprecated. The name of the node pool to upgrade.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("zones-clusters-resource-labels",
                    Some(r##"Sets labels on a cluster."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-clusters-resource-labels",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project
        number](https://developers.google.com/console/help/new/#projectnumber).
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine
        [zone](/compute/docs/zones#available) in which the cluster
        resides.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("zones-clusters-set-maintenance-policy",
                    Some(r##"Sets the maintenance policy for a cluster."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-clusters-set-maintenance-policy",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"The Google Developers Console [project ID or project
        number](https://support.google.com/cloud/answer/6158840)."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"The name of the Google Compute Engine
        [zone](/compute/docs/zones#available) in which the cluster
        resides."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"The name of the cluster to update."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("zones-clusters-set-master-auth",
                    Some(r##"Used to set master auth materials. Currently supports :-
        Changing the admin password for a specific cluster.
        This can be either via password generation or explicitly set the password."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-clusters-set-master-auth",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project
        number](https://support.google.com/cloud/answer/6158840).
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine
        [zone](/compute/docs/zones#available) in which the cluster
        resides.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster to upgrade.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("zones-clusters-set-network-policy",
                    Some(r##"Enables/Disables Network Policy for a cluster."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-clusters-set-network-policy",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project
        number](https://developers.google.com/console/help/new/#projectnumber).
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine
        [zone](/compute/docs/zones#available) in which the cluster
        resides.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("zones-clusters-start-ip-rotation",
                    Some(r##"Start master IP rotation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-clusters-start-ip-rotation",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project
        number](https://developers.google.com/console/help/new/#projectnumber).
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine
        [zone](/compute/docs/zones#available) in which the cluster
        resides.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("zones-clusters-update",
                    Some(r##"Updates the settings of a specific cluster."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-clusters-update",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project
        number](https://support.google.com/cloud/answer/6158840).
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine
        [zone](/compute/docs/zones#available) in which the cluster
        resides.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster to upgrade.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("zones-get-serverconfig",
                    Some(r##"Returns configuration info about the Kubernetes Engine service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-get-serverconfig",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project
        number](https://support.google.com/cloud/answer/6158840).
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine
        [zone](/compute/docs/zones#available) to return operations for.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("zones-operations-cancel",
                    Some(r##"Cancels the specified operation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-operations-cancel",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project
        number](https://support.google.com/cloud/answer/6158840).
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine
        [zone](/compute/docs/zones#available) in which the operation resides.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"operation-id"##),
                     None,
                     Some(r##"Deprecated. The server-assigned `name` of the operation.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("zones-operations-get",
                    Some(r##"Gets the specified operation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-operations-get",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project
        number](https://support.google.com/cloud/answer/6158840).
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine
        [zone](/compute/docs/zones#available) in which the cluster
        resides.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"operation-id"##),
                     None,
                     Some(r##"Deprecated. The server-assigned `name` of the operation.
        This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("zones-operations-list",
                    Some(r##"Lists all operations in a project in a specific zone or all zones."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-operations-list",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project
        number](https://support.google.com/cloud/answer/6158840).
        This field has been deprecated and replaced by the parent field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine
        [zone](/compute/docs/zones#available) to return operations for, or `-` for
        all zones. This field has been deprecated and replaced by the parent field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
    ];
    
    let mut app = App::new("container1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("1.0.8+20190316")
           .about("Builds and manages container-based applications, powered by the open source Kubernetes technology.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_container1_cli")
           .arg(Arg::with_name("url")
                   .long("scope")
                   .help("Specify the authentication a method should be executed in. Each scope requires the user to grant this application permission to use it.If unset, it defaults to the shortest scope url for a particular method.")
                   .multiple(true)
                   .takes_value(true))
           .arg(Arg::with_name("folder")
                   .long("config-dir")
                   .help("A directory into which we will store our persistent data. Defaults to a user-writable directory that we will create during the first invocation.[default: ~/.google-service-cli")
                   .multiple(false)
                   .takes_value(true))
           .arg(Arg::with_name("debug")
                   .long("debug")
                   .help("Output all server communication to standard error. `tx` and `rx` are placed into the same stream.")
                   .multiple(false)
                   .takes_value(false))
           .arg(Arg::with_name("debug-auth")
                   .long("debug-auth")
                   .help("Output all communication related to authentication to standard error. `tx` and `rx` are placed into the same stream.")
                   .multiple(false)
                   .takes_value(false));
           
           for &(main_command_name, about, ref subcommands) in arg_data.iter() {
               let mut mcmd = SubCommand::with_name(main_command_name).about(about);
           
               for &(sub_command_name, ref desc, url_info, ref args) in subcommands {
                   let mut scmd = SubCommand::with_name(sub_command_name);
                   if let &Some(desc) = desc {
                       scmd = scmd.about(desc);
                   }
                   scmd = scmd.after_help(url_info);
           
                   for &(ref arg_name, ref flag, ref desc, ref required, ref multi) in args {
                       let arg_name_str =
                           match (arg_name, flag) {
                                   (&Some(an), _       ) => an,
                                   (_        , &Some(f)) => f,
                                    _                    => unreachable!(),
                            };
                       let mut arg = Arg::with_name(arg_name_str)
                                         .empty_values(false);
                       if let &Some(short_flag) = flag {
                           arg = arg.short(short_flag);
                       }
                       if let &Some(desc) = desc {
                           arg = arg.help(desc);
                       }
                       if arg_name.is_some() && flag.is_some() {
                           arg = arg.takes_value(true);
                       }
                       if let &Some(required) = required {
                           arg = arg.required(required);
                       }
                       if let &Some(multi) = multi {
                           arg = arg.multiple(multi);
                       }
                       scmd = scmd.arg(arg);
                   }
                   mcmd = mcmd.subcommand(scmd);
               }
               app = app.subcommand(mcmd);
           }
           
        let matches = app.get_matches();

    let debug = matches.is_present("debug");
    match Engine::new(matches) {
        Err(err) => {
            exit_status = err.exit_code;
            writeln!(io::stderr(), "{}", err).ok();
        },
        Ok(engine) => {
            if let Err(doit_err) = engine.doit() {
                exit_status = 1;
                match doit_err {
                    DoitError::IoError(path, err) => {
                        writeln!(io::stderr(), "Failed to open output file '{}': {}", path, err).ok();
                    },
                    DoitError::ApiError(err) => {
                        if debug {
                            writeln!(io::stderr(), "{:#?}", err).ok();
                        } else {
                            writeln!(io::stderr(), "{}", err).ok();
                        }
                    }
                }
            }
        }
    }

    std::process::exit(exit_status);
}