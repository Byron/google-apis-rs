// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_container1::{api, Error, oauth2, client::chrono, FieldMask};


use google_clis_common as client;

use client::{InvalidOptionsError, CLIError, arg_from_str, writer_from_opts, parse_kv_arg,
          input_file_from_opts, input_mime_from_opts, FieldCursor, FieldError, CallType, UploadProtocol,
          calltype_from_str, remove_json_null_values, ComplexType, JsonType, JsonTypeInfo};

use std::default::Default;
use std::error::Error as StdError;
use std::str::FromStr;

use serde_json as json;
use clap::ArgMatches;
use http::Uri;
use hyper::client::connect;
use tokio::io::{AsyncRead, AsyncWrite};
use tower_service;

enum DoitError {
    IoError(String, io::Error),
    ApiError(Error),
}

struct Engine<'n, S> {
    opt: ArgMatches<'n>,
    hub: api::Container<S>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n, S> Engine<'n, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    async fn _projects_aggregated_usable_subnetworks_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().aggregated_usable_subnetworks_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
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
                                                                           v.extend(["filter", "page-size", "page-token"].iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_clusters_check_autopilot_compatibility(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_clusters_check_autopilot_compatibility(opt.value_of("name").unwrap_or(""));
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_clusters_complete_ip_rotation(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_clusters_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cluster.addons-config.cloud-run-config.disabled" => Some(("cluster.addonsConfig.cloudRunConfig.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.cloud-run-config.load-balancer-type" => Some(("cluster.addonsConfig.cloudRunConfig.loadBalancerType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.addons-config.config-connector-config.enabled" => Some(("cluster.addonsConfig.configConnectorConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.dns-cache-config.enabled" => Some(("cluster.addonsConfig.dnsCacheConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.gce-persistent-disk-csi-driver-config.enabled" => Some(("cluster.addonsConfig.gcePersistentDiskCsiDriverConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.gcp-filestore-csi-driver-config.enabled" => Some(("cluster.addonsConfig.gcpFilestoreCsiDriverConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.gcs-fuse-csi-driver-config.enabled" => Some(("cluster.addonsConfig.gcsFuseCsiDriverConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.gke-backup-agent-config.enabled" => Some(("cluster.addonsConfig.gkeBackupAgentConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.horizontal-pod-autoscaling.disabled" => Some(("cluster.addonsConfig.horizontalPodAutoscaling.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.http-load-balancing.disabled" => Some(("cluster.addonsConfig.httpLoadBalancing.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.kubernetes-dashboard.disabled" => Some(("cluster.addonsConfig.kubernetesDashboard.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.network-policy-config.disabled" => Some(("cluster.addonsConfig.networkPolicyConfig.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.stateful-ha-config.enabled" => Some(("cluster.addonsConfig.statefulHaConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.authenticator-groups-config.enabled" => Some(("cluster.authenticatorGroupsConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.authenticator-groups-config.security-group" => Some(("cluster.authenticatorGroupsConfig.securityGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.autopilot.enabled" => Some(("cluster.autopilot.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.autopilot.workload-policy-config.allow-net-admin" => Some(("cluster.autopilot.workloadPolicyConfig.allowNetAdmin", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-locations" => Some(("cluster.autoscaling.autoprovisioningLocations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.boot-disk-kms-key" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.bootDiskKmsKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.disk-size-gb" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.diskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.disk-type" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.diskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.image-type" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.imageType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.insecure-kubelet-readonly-port-enabled" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.insecureKubeletReadonlyPortEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.management.auto-repair" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.management.autoRepair", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.management.auto-upgrade" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.management.autoUpgrade", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.management.upgrade-options.auto-upgrade-start-time" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.management.upgradeOptions.autoUpgradeStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.management.upgrade-options.description" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.management.upgradeOptions.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.min-cpu-platform" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.minCpuPlatform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.oauth-scopes" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.oauthScopes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.service-account" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.shielded-instance-config.enable-integrity-monitoring" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.shieldedInstanceConfig.enableIntegrityMonitoring", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.shielded-instance-config.enable-secure-boot" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.shieldedInstanceConfig.enableSecureBoot", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.blue-green-settings.node-pool-soak-duration" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.blueGreenSettings.nodePoolSoakDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.blue-green-settings.standard-rollout-policy.batch-node-count" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.blueGreenSettings.standardRolloutPolicy.batchNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.blue-green-settings.standard-rollout-policy.batch-percentage" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.blueGreenSettings.standardRolloutPolicy.batchPercentage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.blue-green-settings.standard-rollout-policy.batch-soak-duration" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.blueGreenSettings.standardRolloutPolicy.batchSoakDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.max-surge" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.maxSurge", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.max-unavailable" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.maxUnavailable", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.strategy" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.strategy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoscaling-profile" => Some(("cluster.autoscaling.autoscalingProfile", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.enable-node-autoprovisioning" => Some(("cluster.autoscaling.enableNodeAutoprovisioning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.binary-authorization.enabled" => Some(("cluster.binaryAuthorization.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.binary-authorization.evaluation-mode" => Some(("cluster.binaryAuthorization.evaluationMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.cluster-ipv4-cidr" => Some(("cluster.clusterIpv4Cidr", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.confidential-nodes.enabled" => Some(("cluster.confidentialNodes.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.cost-management-config.enabled" => Some(("cluster.costManagementConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.create-time" => Some(("cluster.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.current-master-version" => Some(("cluster.currentMasterVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.current-node-count" => Some(("cluster.currentNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.current-node-version" => Some(("cluster.currentNodeVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.database-encryption.current-state" => Some(("cluster.databaseEncryption.currentState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.database-encryption.decryption-keys" => Some(("cluster.databaseEncryption.decryptionKeys", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.database-encryption.key-name" => Some(("cluster.databaseEncryption.keyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.database-encryption.state" => Some(("cluster.databaseEncryption.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.default-max-pods-constraint.max-pods-per-node" => Some(("cluster.defaultMaxPodsConstraint.maxPodsPerNode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.description" => Some(("cluster.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.enable-k8s-beta-apis.enabled-apis" => Some(("cluster.enableK8sBetaApis.enabledApis", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.enable-kubernetes-alpha" => Some(("cluster.enableKubernetesAlpha", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.enable-tpu" => Some(("cluster.enableTpu", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.endpoint" => Some(("cluster.endpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.enterprise-config.cluster-tier" => Some(("cluster.enterpriseConfig.clusterTier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.etag" => Some(("cluster.etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.expire-time" => Some(("cluster.expireTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.fleet.membership" => Some(("cluster.fleet.membership", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.fleet.pre-registered" => Some(("cluster.fleet.preRegistered", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.fleet.project" => Some(("cluster.fleet.project", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.id" => Some(("cluster.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.identity-service-config.enabled" => Some(("cluster.identityServiceConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.initial-cluster-version" => Some(("cluster.initialClusterVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.initial-node-count" => Some(("cluster.initialNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.instance-group-urls" => Some(("cluster.instanceGroupUrls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.ip-allocation-policy.additional-pod-ranges-config.pod-range-names" => Some(("cluster.ipAllocationPolicy.additionalPodRangesConfig.podRangeNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.ip-allocation-policy.cluster-ipv4-cidr" => Some(("cluster.ipAllocationPolicy.clusterIpv4Cidr", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.cluster-ipv4-cidr-block" => Some(("cluster.ipAllocationPolicy.clusterIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.cluster-secondary-range-name" => Some(("cluster.ipAllocationPolicy.clusterSecondaryRangeName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.create-subnetwork" => Some(("cluster.ipAllocationPolicy.createSubnetwork", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.default-pod-ipv4-range-utilization" => Some(("cluster.ipAllocationPolicy.defaultPodIpv4RangeUtilization", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.ipv6-access-type" => Some(("cluster.ipAllocationPolicy.ipv6AccessType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.node-ipv4-cidr" => Some(("cluster.ipAllocationPolicy.nodeIpv4Cidr", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.node-ipv4-cidr-block" => Some(("cluster.ipAllocationPolicy.nodeIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.pod-cidr-overprovision-config.disable" => Some(("cluster.ipAllocationPolicy.podCidrOverprovisionConfig.disable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.services-ipv4-cidr" => Some(("cluster.ipAllocationPolicy.servicesIpv4Cidr", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.services-ipv4-cidr-block" => Some(("cluster.ipAllocationPolicy.servicesIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.services-ipv6-cidr-block" => Some(("cluster.ipAllocationPolicy.servicesIpv6CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.services-secondary-range-name" => Some(("cluster.ipAllocationPolicy.servicesSecondaryRangeName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.stack-type" => Some(("cluster.ipAllocationPolicy.stackType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.subnet-ipv6-cidr-block" => Some(("cluster.ipAllocationPolicy.subnetIpv6CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.subnetwork-name" => Some(("cluster.ipAllocationPolicy.subnetworkName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.tpu-ipv4-cidr-block" => Some(("cluster.ipAllocationPolicy.tpuIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.use-ip-aliases" => Some(("cluster.ipAllocationPolicy.useIpAliases", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.use-routes" => Some(("cluster.ipAllocationPolicy.useRoutes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.label-fingerprint" => Some(("cluster.labelFingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.legacy-abac.enabled" => Some(("cluster.legacyAbac.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.location" => Some(("cluster.location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.locations" => Some(("cluster.locations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.logging-config.component-config.enable-components" => Some(("cluster.loggingConfig.componentConfig.enableComponents", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.logging-service" => Some(("cluster.loggingService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.maintenance-policy.resource-version" => Some(("cluster.maintenancePolicy.resourceVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.maintenance-policy.window.daily-maintenance-window.duration" => Some(("cluster.maintenancePolicy.window.dailyMaintenanceWindow.duration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.maintenance-policy.window.daily-maintenance-window.start-time" => Some(("cluster.maintenancePolicy.window.dailyMaintenanceWindow.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.maintenance-policy.window.recurring-window.recurrence" => Some(("cluster.maintenancePolicy.window.recurringWindow.recurrence", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.maintenance-policy.window.recurring-window.window.end-time" => Some(("cluster.maintenancePolicy.window.recurringWindow.window.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.maintenance-policy.window.recurring-window.window.maintenance-exclusion-options.scope" => Some(("cluster.maintenancePolicy.window.recurringWindow.window.maintenanceExclusionOptions.scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.maintenance-policy.window.recurring-window.window.start-time" => Some(("cluster.maintenancePolicy.window.recurringWindow.window.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.master-auth.client-certificate" => Some(("cluster.masterAuth.clientCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.master-auth.client-certificate-config.issue-client-certificate" => Some(("cluster.masterAuth.clientCertificateConfig.issueClientCertificate", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.master-auth.client-key" => Some(("cluster.masterAuth.clientKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.master-auth.cluster-ca-certificate" => Some(("cluster.masterAuth.clusterCaCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.master-auth.password" => Some(("cluster.masterAuth.password", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.master-auth.username" => Some(("cluster.masterAuth.username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.master-authorized-networks-config.enabled" => Some(("cluster.masterAuthorizedNetworksConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.master-authorized-networks-config.gcp-public-cidrs-access-enabled" => Some(("cluster.masterAuthorizedNetworksConfig.gcpPublicCidrsAccessEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.mesh-certificates.enable-certificates" => Some(("cluster.meshCertificates.enableCertificates", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.monitoring-config.advanced-datapath-observability-config.enable-metrics" => Some(("cluster.monitoringConfig.advancedDatapathObservabilityConfig.enableMetrics", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.monitoring-config.advanced-datapath-observability-config.enable-relay" => Some(("cluster.monitoringConfig.advancedDatapathObservabilityConfig.enableRelay", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.monitoring-config.advanced-datapath-observability-config.relay-mode" => Some(("cluster.monitoringConfig.advancedDatapathObservabilityConfig.relayMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.monitoring-config.component-config.enable-components" => Some(("cluster.monitoringConfig.componentConfig.enableComponents", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.monitoring-config.managed-prometheus-config.enabled" => Some(("cluster.monitoringConfig.managedPrometheusConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.monitoring-service" => Some(("cluster.monitoringService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.name" => Some(("cluster.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.network" => Some(("cluster.network", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.network-config.datapath-provider" => Some(("cluster.networkConfig.datapathProvider", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.network-config.default-snat-status.disabled" => Some(("cluster.networkConfig.defaultSnatStatus.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.network-config.dns-config.additive-vpc-scope-dns-domain" => Some(("cluster.networkConfig.dnsConfig.additiveVpcScopeDnsDomain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.network-config.dns-config.cluster-dns" => Some(("cluster.networkConfig.dnsConfig.clusterDns", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.network-config.dns-config.cluster-dns-domain" => Some(("cluster.networkConfig.dnsConfig.clusterDnsDomain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.network-config.dns-config.cluster-dns-scope" => Some(("cluster.networkConfig.dnsConfig.clusterDnsScope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.network-config.enable-cilium-clusterwide-network-policy" => Some(("cluster.networkConfig.enableCiliumClusterwideNetworkPolicy", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.network-config.enable-fqdn-network-policy" => Some(("cluster.networkConfig.enableFqdnNetworkPolicy", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.network-config.enable-intra-node-visibility" => Some(("cluster.networkConfig.enableIntraNodeVisibility", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.network-config.enable-l4ilb-subsetting" => Some(("cluster.networkConfig.enableL4ilbSubsetting", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.network-config.enable-multi-networking" => Some(("cluster.networkConfig.enableMultiNetworking", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.network-config.gateway-api-config.channel" => Some(("cluster.networkConfig.gatewayApiConfig.channel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.network-config.in-transit-encryption-config" => Some(("cluster.networkConfig.inTransitEncryptionConfig", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.network-config.network" => Some(("cluster.networkConfig.network", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.network-config.network-performance-config.total-egress-bandwidth-tier" => Some(("cluster.networkConfig.networkPerformanceConfig.totalEgressBandwidthTier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.network-config.private-ipv6-google-access" => Some(("cluster.networkConfig.privateIpv6GoogleAccess", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.network-config.service-external-ips-config.enabled" => Some(("cluster.networkConfig.serviceExternalIpsConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.network-config.subnetwork" => Some(("cluster.networkConfig.subnetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.network-policy.enabled" => Some(("cluster.networkPolicy.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.network-policy.provider" => Some(("cluster.networkPolicy.provider", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.advanced-machine-features.enable-nested-virtualization" => Some(("cluster.nodeConfig.advancedMachineFeatures.enableNestedVirtualization", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.advanced-machine-features.threads-per-core" => Some(("cluster.nodeConfig.advancedMachineFeatures.threadsPerCore", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.boot-disk-kms-key" => Some(("cluster.nodeConfig.bootDiskKmsKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.confidential-nodes.enabled" => Some(("cluster.nodeConfig.confidentialNodes.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.containerd-config.private-registry-access-config.enabled" => Some(("cluster.nodeConfig.containerdConfig.privateRegistryAccessConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.disk-size-gb" => Some(("cluster.nodeConfig.diskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-config.disk-type" => Some(("cluster.nodeConfig.diskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.enable-confidential-storage" => Some(("cluster.nodeConfig.enableConfidentialStorage", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.ephemeral-storage-local-ssd-config.local-ssd-count" => Some(("cluster.nodeConfig.ephemeralStorageLocalSsdConfig.localSsdCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-config.fast-socket.enabled" => Some(("cluster.nodeConfig.fastSocket.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.gcfs-config.enabled" => Some(("cluster.nodeConfig.gcfsConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.gvnic.enabled" => Some(("cluster.nodeConfig.gvnic.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.image-type" => Some(("cluster.nodeConfig.imageType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.cpu-cfs-quota" => Some(("cluster.nodeConfig.kubeletConfig.cpuCfsQuota", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.cpu-cfs-quota-period" => Some(("cluster.nodeConfig.kubeletConfig.cpuCfsQuotaPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.cpu-manager-policy" => Some(("cluster.nodeConfig.kubeletConfig.cpuManagerPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.insecure-kubelet-readonly-port-enabled" => Some(("cluster.nodeConfig.kubeletConfig.insecureKubeletReadonlyPortEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.pod-pids-limit" => Some(("cluster.nodeConfig.kubeletConfig.podPidsLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.labels" => Some(("cluster.nodeConfig.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "cluster.node-config.linux-node-config.cgroup-mode" => Some(("cluster.nodeConfig.linuxNodeConfig.cgroupMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.linux-node-config.hugepages.hugepage-size1g" => Some(("cluster.nodeConfig.linuxNodeConfig.hugepages.hugepageSize1g", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-config.linux-node-config.hugepages.hugepage-size2m" => Some(("cluster.nodeConfig.linuxNodeConfig.hugepages.hugepageSize2m", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-config.linux-node-config.sysctls" => Some(("cluster.nodeConfig.linuxNodeConfig.sysctls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "cluster.node-config.local-nvme-ssd-block-config.local-ssd-count" => Some(("cluster.nodeConfig.localNvmeSsdBlockConfig.localSsdCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-config.local-ssd-count" => Some(("cluster.nodeConfig.localSsdCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-config.logging-config.variant-config.variant" => Some(("cluster.nodeConfig.loggingConfig.variantConfig.variant", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.machine-type" => Some(("cluster.nodeConfig.machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.metadata" => Some(("cluster.nodeConfig.metadata", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "cluster.node-config.min-cpu-platform" => Some(("cluster.nodeConfig.minCpuPlatform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.node-group" => Some(("cluster.nodeConfig.nodeGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.oauth-scopes" => Some(("cluster.nodeConfig.oauthScopes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.node-config.preemptible" => Some(("cluster.nodeConfig.preemptible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.reservation-affinity.consume-reservation-type" => Some(("cluster.nodeConfig.reservationAffinity.consumeReservationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.reservation-affinity.key" => Some(("cluster.nodeConfig.reservationAffinity.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.reservation-affinity.values" => Some(("cluster.nodeConfig.reservationAffinity.values", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.node-config.resource-labels" => Some(("cluster.nodeConfig.resourceLabels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "cluster.node-config.resource-manager-tags.tags" => Some(("cluster.nodeConfig.resourceManagerTags.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "cluster.node-config.sandbox-config.type" => Some(("cluster.nodeConfig.sandboxConfig.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.service-account" => Some(("cluster.nodeConfig.serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.shielded-instance-config.enable-integrity-monitoring" => Some(("cluster.nodeConfig.shieldedInstanceConfig.enableIntegrityMonitoring", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.shielded-instance-config.enable-secure-boot" => Some(("cluster.nodeConfig.shieldedInstanceConfig.enableSecureBoot", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.spot" => Some(("cluster.nodeConfig.spot", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.tags" => Some(("cluster.nodeConfig.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.node-config.windows-node-config.os-version" => Some(("cluster.nodeConfig.windowsNodeConfig.osVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.workload-metadata-config.mode" => Some(("cluster.nodeConfig.workloadMetadataConfig.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-ipv4-cidr-size" => Some(("cluster.nodeIpv4CidrSize", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.network-tags.tags" => Some(("cluster.nodePoolAutoConfig.networkTags.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.node-pool-auto-config.node-kubelet-config.cpu-cfs-quota" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.cpuCfsQuota", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.cpu-cfs-quota-period" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.cpuCfsQuotaPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.cpu-manager-policy" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.cpuManagerPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.insecure-kubelet-readonly-port-enabled" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.insecureKubeletReadonlyPortEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.pod-pids-limit" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.podPidsLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.resource-manager-tags.tags" => Some(("cluster.nodePoolAutoConfig.resourceManagerTags.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "cluster.node-pool-defaults.node-config-defaults.containerd-config.private-registry-access-config.enabled" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.containerdConfig.privateRegistryAccessConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.gcfs-config.enabled" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.gcfsConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.logging-config.variant-config.variant" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.loggingConfig.variantConfig.variant", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.cpu-cfs-quota" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.cpuCfsQuota", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.cpu-cfs-quota-period" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.cpuCfsQuotaPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.cpu-manager-policy" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.cpuManagerPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.insecure-kubelet-readonly-port-enabled" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.insecureKubeletReadonlyPortEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.pod-pids-limit" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.podPidsLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.notification-config.pubsub.enabled" => Some(("cluster.notificationConfig.pubsub.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.notification-config.pubsub.filter.event-type" => Some(("cluster.notificationConfig.pubsub.filter.eventType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.notification-config.pubsub.topic" => Some(("cluster.notificationConfig.pubsub.topic", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.parent-product-config.labels" => Some(("cluster.parentProductConfig.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "cluster.parent-product-config.product-name" => Some(("cluster.parentProductConfig.productName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.enable-private-endpoint" => Some(("cluster.privateClusterConfig.enablePrivateEndpoint", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.enable-private-nodes" => Some(("cluster.privateClusterConfig.enablePrivateNodes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.master-global-access-config.enabled" => Some(("cluster.privateClusterConfig.masterGlobalAccessConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.master-ipv4-cidr-block" => Some(("cluster.privateClusterConfig.masterIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.peering-name" => Some(("cluster.privateClusterConfig.peeringName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.private-endpoint" => Some(("cluster.privateClusterConfig.privateEndpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.private-endpoint-subnetwork" => Some(("cluster.privateClusterConfig.privateEndpointSubnetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.public-endpoint" => Some(("cluster.privateClusterConfig.publicEndpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.release-channel.channel" => Some(("cluster.releaseChannel.channel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.resource-labels" => Some(("cluster.resourceLabels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "cluster.resource-usage-export-config.bigquery-destination.dataset-id" => Some(("cluster.resourceUsageExportConfig.bigqueryDestination.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.resource-usage-export-config.consumption-metering-config.enabled" => Some(("cluster.resourceUsageExportConfig.consumptionMeteringConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.resource-usage-export-config.enable-network-egress-metering" => Some(("cluster.resourceUsageExportConfig.enableNetworkEgressMetering", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.satisfies-pzi" => Some(("cluster.satisfiesPzi", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.satisfies-pzs" => Some(("cluster.satisfiesPzs", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.security-posture-config.mode" => Some(("cluster.securityPostureConfig.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.security-posture-config.vulnerability-mode" => Some(("cluster.securityPostureConfig.vulnerabilityMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.self-link" => Some(("cluster.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.services-ipv4-cidr" => Some(("cluster.servicesIpv4Cidr", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.shielded-nodes.enabled" => Some(("cluster.shieldedNodes.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.status" => Some(("cluster.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.status-message" => Some(("cluster.statusMessage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.subnetwork" => Some(("cluster.subnetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.tpu-ipv4-cidr-block" => Some(("cluster.tpuIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.vertical-pod-autoscaling.enabled" => Some(("cluster.verticalPodAutoscaling.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.workload-identity-config.workload-pool" => Some(("cluster.workloadIdentityConfig.workloadPool", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.zone" => Some(("cluster.zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent" => Some(("parent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["additional-pod-ranges-config", "additive-vpc-scope-dns-domain", "addons-config", "advanced-datapath-observability-config", "advanced-machine-features", "allow-net-admin", "authenticator-groups-config", "auto-repair", "auto-upgrade", "auto-upgrade-start-time", "autopilot", "autoprovisioning-locations", "autoprovisioning-node-pool-defaults", "autoscaling", "autoscaling-profile", "batch-node-count", "batch-percentage", "batch-soak-duration", "bigquery-destination", "binary-authorization", "blue-green-settings", "boot-disk-kms-key", "cgroup-mode", "channel", "client-certificate", "client-certificate-config", "client-key", "cloud-run-config", "cluster", "cluster-ca-certificate", "cluster-dns", "cluster-dns-domain", "cluster-dns-scope", "cluster-ipv4-cidr", "cluster-ipv4-cidr-block", "cluster-secondary-range-name", "cluster-tier", "component-config", "confidential-nodes", "config-connector-config", "consume-reservation-type", "consumption-metering-config", "containerd-config", "cost-management-config", "cpu-cfs-quota", "cpu-cfs-quota-period", "cpu-manager-policy", "create-subnetwork", "create-time", "current-master-version", "current-node-count", "current-node-version", "current-state", "daily-maintenance-window", "database-encryption", "datapath-provider", "dataset-id", "decryption-keys", "default-max-pods-constraint", "default-pod-ipv4-range-utilization", "default-snat-status", "description", "disable", "disabled", "disk-size-gb", "disk-type", "dns-cache-config", "dns-config", "duration", "enable-certificates", "enable-cilium-clusterwide-network-policy", "enable-components", "enable-confidential-storage", "enable-fqdn-network-policy", "enable-integrity-monitoring", "enable-intra-node-visibility", "enable-k8s-beta-apis", "enable-kubernetes-alpha", "enable-l4ilb-subsetting", "enable-metrics", "enable-multi-networking", "enable-nested-virtualization", "enable-network-egress-metering", "enable-node-autoprovisioning", "enable-private-endpoint", "enable-private-nodes", "enable-relay", "enable-secure-boot", "enable-tpu", "enabled", "enabled-apis", "end-time", "endpoint", "enterprise-config", "ephemeral-storage-local-ssd-config", "etag", "evaluation-mode", "event-type", "expire-time", "fast-socket", "filter", "fleet", "gateway-api-config", "gce-persistent-disk-csi-driver-config", "gcfs-config", "gcp-filestore-csi-driver-config", "gcp-public-cidrs-access-enabled", "gcs-fuse-csi-driver-config", "gke-backup-agent-config", "gvnic", "horizontal-pod-autoscaling", "http-load-balancing", "hugepage-size1g", "hugepage-size2m", "hugepages", "id", "identity-service-config", "image-type", "in-transit-encryption-config", "initial-cluster-version", "initial-node-count", "insecure-kubelet-readonly-port-enabled", "instance-group-urls", "ip-allocation-policy", "ipv6-access-type", "issue-client-certificate", "key", "key-name", "kubelet-config", "kubernetes-dashboard", "label-fingerprint", "labels", "legacy-abac", "linux-node-config", "load-balancer-type", "local-nvme-ssd-block-config", "local-ssd-count", "location", "locations", "logging-config", "logging-service", "machine-type", "maintenance-exclusion-options", "maintenance-policy", "managed-prometheus-config", "management", "master-auth", "master-authorized-networks-config", "master-global-access-config", "master-ipv4-cidr-block", "max-pods-per-node", "max-surge", "max-unavailable", "membership", "mesh-certificates", "metadata", "min-cpu-platform", "mode", "monitoring-config", "monitoring-service", "name", "network", "network-config", "network-performance-config", "network-policy", "network-policy-config", "network-tags", "node-config", "node-config-defaults", "node-group", "node-ipv4-cidr", "node-ipv4-cidr-block", "node-ipv4-cidr-size", "node-kubelet-config", "node-pool-auto-config", "node-pool-defaults", "node-pool-soak-duration", "notification-config", "oauth-scopes", "os-version", "parent", "parent-product-config", "password", "peering-name", "pod-cidr-overprovision-config", "pod-pids-limit", "pod-range-names", "pre-registered", "preemptible", "private-cluster-config", "private-endpoint", "private-endpoint-subnetwork", "private-ipv6-google-access", "private-registry-access-config", "product-name", "project", "project-id", "provider", "public-endpoint", "pubsub", "recurrence", "recurring-window", "relay-mode", "release-channel", "reservation-affinity", "resource-labels", "resource-manager-tags", "resource-usage-export-config", "resource-version", "sandbox-config", "satisfies-pzi", "satisfies-pzs", "scope", "security-group", "security-posture-config", "self-link", "service-account", "service-external-ips-config", "services-ipv4-cidr", "services-ipv4-cidr-block", "services-ipv6-cidr-block", "services-secondary-range-name", "shielded-instance-config", "shielded-nodes", "spot", "stack-type", "standard-rollout-policy", "start-time", "state", "stateful-ha-config", "status", "status-message", "strategy", "subnet-ipv6-cidr-block", "subnetwork", "subnetwork-name", "sysctls", "tags", "threads-per-core", "topic", "total-egress-bandwidth-tier", "tpu-ipv4-cidr-block", "type", "upgrade-options", "upgrade-settings", "use-ip-aliases", "use-routes", "username", "values", "variant", "variant-config", "vertical-pod-autoscaling", "vulnerability-mode", "window", "windows-node-config", "workload-identity-config", "workload-metadata-config", "workload-policy-config", "workload-pool", "zone"]);
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_clusters_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                                                                           v.extend(["cluster-id", "project-id", "zone"].iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_clusters_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                                                                           v.extend(["cluster-id", "project-id", "zone"].iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_clusters_get_jwks(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_clusters_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_clusters_node_pools_complete_upgrade(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec![]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CompleteNodePoolUpgradeRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_clusters_node_pools_complete_upgrade(request, opt.value_of("name").unwrap_or(""));
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_clusters_node_pools_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.autoscaling.autoprovisioned" => Some(("nodePool.autoscaling.autoprovisioned", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.autoscaling.enabled" => Some(("nodePool.autoscaling.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.autoscaling.location-policy" => Some(("nodePool.autoscaling.locationPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.autoscaling.max-node-count" => Some(("nodePool.autoscaling.maxNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.autoscaling.min-node-count" => Some(("nodePool.autoscaling.minNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.autoscaling.total-max-node-count" => Some(("nodePool.autoscaling.totalMaxNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.autoscaling.total-min-node-count" => Some(("nodePool.autoscaling.totalMinNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.best-effort-provisioning.enabled" => Some(("nodePool.bestEffortProvisioning.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.best-effort-provisioning.min-provision-nodes" => Some(("nodePool.bestEffortProvisioning.minProvisionNodes", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.advanced-machine-features.enable-nested-virtualization" => Some(("nodePool.config.advancedMachineFeatures.enableNestedVirtualization", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.advanced-machine-features.threads-per-core" => Some(("nodePool.config.advancedMachineFeatures.threadsPerCore", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.boot-disk-kms-key" => Some(("nodePool.config.bootDiskKmsKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.confidential-nodes.enabled" => Some(("nodePool.config.confidentialNodes.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.containerd-config.private-registry-access-config.enabled" => Some(("nodePool.config.containerdConfig.privateRegistryAccessConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.disk-size-gb" => Some(("nodePool.config.diskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.disk-type" => Some(("nodePool.config.diskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.enable-confidential-storage" => Some(("nodePool.config.enableConfidentialStorage", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.ephemeral-storage-local-ssd-config.local-ssd-count" => Some(("nodePool.config.ephemeralStorageLocalSsdConfig.localSsdCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.fast-socket.enabled" => Some(("nodePool.config.fastSocket.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.gcfs-config.enabled" => Some(("nodePool.config.gcfsConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.gvnic.enabled" => Some(("nodePool.config.gvnic.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.image-type" => Some(("nodePool.config.imageType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.cpu-cfs-quota" => Some(("nodePool.config.kubeletConfig.cpuCfsQuota", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.cpu-cfs-quota-period" => Some(("nodePool.config.kubeletConfig.cpuCfsQuotaPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.cpu-manager-policy" => Some(("nodePool.config.kubeletConfig.cpuManagerPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.insecure-kubelet-readonly-port-enabled" => Some(("nodePool.config.kubeletConfig.insecureKubeletReadonlyPortEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.pod-pids-limit" => Some(("nodePool.config.kubeletConfig.podPidsLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.labels" => Some(("nodePool.config.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "node-pool.config.linux-node-config.cgroup-mode" => Some(("nodePool.config.linuxNodeConfig.cgroupMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.linux-node-config.hugepages.hugepage-size1g" => Some(("nodePool.config.linuxNodeConfig.hugepages.hugepageSize1g", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.linux-node-config.hugepages.hugepage-size2m" => Some(("nodePool.config.linuxNodeConfig.hugepages.hugepageSize2m", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.linux-node-config.sysctls" => Some(("nodePool.config.linuxNodeConfig.sysctls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "node-pool.config.local-nvme-ssd-block-config.local-ssd-count" => Some(("nodePool.config.localNvmeSsdBlockConfig.localSsdCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.local-ssd-count" => Some(("nodePool.config.localSsdCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.logging-config.variant-config.variant" => Some(("nodePool.config.loggingConfig.variantConfig.variant", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.machine-type" => Some(("nodePool.config.machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.metadata" => Some(("nodePool.config.metadata", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "node-pool.config.min-cpu-platform" => Some(("nodePool.config.minCpuPlatform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.node-group" => Some(("nodePool.config.nodeGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.oauth-scopes" => Some(("nodePool.config.oauthScopes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "node-pool.config.preemptible" => Some(("nodePool.config.preemptible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.reservation-affinity.consume-reservation-type" => Some(("nodePool.config.reservationAffinity.consumeReservationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.reservation-affinity.key" => Some(("nodePool.config.reservationAffinity.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.reservation-affinity.values" => Some(("nodePool.config.reservationAffinity.values", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "node-pool.config.resource-labels" => Some(("nodePool.config.resourceLabels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "node-pool.config.resource-manager-tags.tags" => Some(("nodePool.config.resourceManagerTags.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "node-pool.config.sandbox-config.type" => Some(("nodePool.config.sandboxConfig.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.service-account" => Some(("nodePool.config.serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.shielded-instance-config.enable-integrity-monitoring" => Some(("nodePool.config.shieldedInstanceConfig.enableIntegrityMonitoring", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.shielded-instance-config.enable-secure-boot" => Some(("nodePool.config.shieldedInstanceConfig.enableSecureBoot", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.spot" => Some(("nodePool.config.spot", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.tags" => Some(("nodePool.config.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "node-pool.config.windows-node-config.os-version" => Some(("nodePool.config.windowsNodeConfig.osVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.workload-metadata-config.mode" => Some(("nodePool.config.workloadMetadataConfig.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.etag" => Some(("nodePool.etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.initial-node-count" => Some(("nodePool.initialNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.instance-group-urls" => Some(("nodePool.instanceGroupUrls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "node-pool.locations" => Some(("nodePool.locations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "node-pool.management.auto-repair" => Some(("nodePool.management.autoRepair", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.management.auto-upgrade" => Some(("nodePool.management.autoUpgrade", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.management.upgrade-options.auto-upgrade-start-time" => Some(("nodePool.management.upgradeOptions.autoUpgradeStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.management.upgrade-options.description" => Some(("nodePool.management.upgradeOptions.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.max-pods-constraint.max-pods-per-node" => Some(("nodePool.maxPodsConstraint.maxPodsPerNode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.name" => Some(("nodePool.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.network-config.create-pod-range" => Some(("nodePool.networkConfig.createPodRange", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.network-config.enable-private-nodes" => Some(("nodePool.networkConfig.enablePrivateNodes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.network-config.network-performance-config.total-egress-bandwidth-tier" => Some(("nodePool.networkConfig.networkPerformanceConfig.totalEgressBandwidthTier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.network-config.pod-cidr-overprovision-config.disable" => Some(("nodePool.networkConfig.podCidrOverprovisionConfig.disable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.network-config.pod-ipv4-cidr-block" => Some(("nodePool.networkConfig.podIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.network-config.pod-ipv4-range-utilization" => Some(("nodePool.networkConfig.podIpv4RangeUtilization", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "node-pool.network-config.pod-range" => Some(("nodePool.networkConfig.podRange", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.placement-policy.policy-name" => Some(("nodePool.placementPolicy.policyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.placement-policy.tpu-topology" => Some(("nodePool.placementPolicy.tpuTopology", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.placement-policy.type" => Some(("nodePool.placementPolicy.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.pod-ipv4-cidr-size" => Some(("nodePool.podIpv4CidrSize", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.queued-provisioning.enabled" => Some(("nodePool.queuedProvisioning.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.self-link" => Some(("nodePool.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.status" => Some(("nodePool.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.status-message" => Some(("nodePool.statusMessage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.update-info.blue-green-info.blue-instance-group-urls" => Some(("nodePool.updateInfo.blueGreenInfo.blueInstanceGroupUrls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "node-pool.update-info.blue-green-info.blue-pool-deletion-start-time" => Some(("nodePool.updateInfo.blueGreenInfo.bluePoolDeletionStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.update-info.blue-green-info.green-instance-group-urls" => Some(("nodePool.updateInfo.blueGreenInfo.greenInstanceGroupUrls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "node-pool.update-info.blue-green-info.green-pool-version" => Some(("nodePool.updateInfo.blueGreenInfo.greenPoolVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.update-info.blue-green-info.phase" => Some(("nodePool.updateInfo.blueGreenInfo.phase", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.upgrade-settings.blue-green-settings.node-pool-soak-duration" => Some(("nodePool.upgradeSettings.blueGreenSettings.nodePoolSoakDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.upgrade-settings.blue-green-settings.standard-rollout-policy.batch-node-count" => Some(("nodePool.upgradeSettings.blueGreenSettings.standardRolloutPolicy.batchNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.upgrade-settings.blue-green-settings.standard-rollout-policy.batch-percentage" => Some(("nodePool.upgradeSettings.blueGreenSettings.standardRolloutPolicy.batchPercentage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "node-pool.upgrade-settings.blue-green-settings.standard-rollout-policy.batch-soak-duration" => Some(("nodePool.upgradeSettings.blueGreenSettings.standardRolloutPolicy.batchSoakDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.upgrade-settings.max-surge" => Some(("nodePool.upgradeSettings.maxSurge", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.upgrade-settings.max-unavailable" => Some(("nodePool.upgradeSettings.maxUnavailable", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.upgrade-settings.strategy" => Some(("nodePool.upgradeSettings.strategy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.version" => Some(("nodePool.version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent" => Some(("parent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["advanced-machine-features", "auto-repair", "auto-upgrade", "auto-upgrade-start-time", "autoprovisioned", "autoscaling", "batch-node-count", "batch-percentage", "batch-soak-duration", "best-effort-provisioning", "blue-green-info", "blue-green-settings", "blue-instance-group-urls", "blue-pool-deletion-start-time", "boot-disk-kms-key", "cgroup-mode", "cluster-id", "confidential-nodes", "config", "consume-reservation-type", "containerd-config", "cpu-cfs-quota", "cpu-cfs-quota-period", "cpu-manager-policy", "create-pod-range", "description", "disable", "disk-size-gb", "disk-type", "enable-confidential-storage", "enable-integrity-monitoring", "enable-nested-virtualization", "enable-private-nodes", "enable-secure-boot", "enabled", "ephemeral-storage-local-ssd-config", "etag", "fast-socket", "gcfs-config", "green-instance-group-urls", "green-pool-version", "gvnic", "hugepage-size1g", "hugepage-size2m", "hugepages", "image-type", "initial-node-count", "insecure-kubelet-readonly-port-enabled", "instance-group-urls", "key", "kubelet-config", "labels", "linux-node-config", "local-nvme-ssd-block-config", "local-ssd-count", "location-policy", "locations", "logging-config", "machine-type", "management", "max-node-count", "max-pods-constraint", "max-pods-per-node", "max-surge", "max-unavailable", "metadata", "min-cpu-platform", "min-node-count", "min-provision-nodes", "mode", "name", "network-config", "network-performance-config", "node-group", "node-pool", "node-pool-soak-duration", "oauth-scopes", "os-version", "parent", "phase", "placement-policy", "pod-cidr-overprovision-config", "pod-ipv4-cidr-block", "pod-ipv4-cidr-size", "pod-ipv4-range-utilization", "pod-pids-limit", "pod-range", "policy-name", "preemptible", "private-registry-access-config", "project-id", "queued-provisioning", "reservation-affinity", "resource-labels", "resource-manager-tags", "sandbox-config", "self-link", "service-account", "shielded-instance-config", "spot", "standard-rollout-policy", "status", "status-message", "strategy", "sysctls", "tags", "threads-per-core", "total-egress-bandwidth-tier", "total-max-node-count", "total-min-node-count", "tpu-topology", "type", "update-info", "upgrade-options", "upgrade-settings", "values", "variant", "variant-config", "version", "windows-node-config", "workload-metadata-config", "zone"]);
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_clusters_node_pools_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                                                                           v.extend(["cluster-id", "node-pool-id", "project-id", "zone"].iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_clusters_node_pools_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                                                                           v.extend(["cluster-id", "node-pool-id", "project-id", "zone"].iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_clusters_node_pools_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                                                                           v.extend(["cluster-id", "project-id", "zone"].iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_clusters_node_pools_rollback(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool-id" => Some(("nodePoolId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "respect-pdb" => Some(("respectPdb", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cluster-id", "name", "node-pool-id", "project-id", "respect-pdb", "zone"]);
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_clusters_node_pools_set_autoscaling(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "autoscaling.autoprovisioned" => Some(("autoscaling.autoprovisioned", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "autoscaling.enabled" => Some(("autoscaling.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "autoscaling.location-policy" => Some(("autoscaling.locationPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "autoscaling.max-node-count" => Some(("autoscaling.maxNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "autoscaling.min-node-count" => Some(("autoscaling.minNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "autoscaling.total-max-node-count" => Some(("autoscaling.totalMaxNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "autoscaling.total-min-node-count" => Some(("autoscaling.totalMinNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool-id" => Some(("nodePoolId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["autoprovisioned", "autoscaling", "cluster-id", "enabled", "location-policy", "max-node-count", "min-node-count", "name", "node-pool-id", "project-id", "total-max-node-count", "total-min-node-count", "zone"]);
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_clusters_node_pools_set_management(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "management.auto-repair" => Some(("management.autoRepair", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "management.auto-upgrade" => Some(("management.autoUpgrade", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "management.upgrade-options.auto-upgrade-start-time" => Some(("management.upgradeOptions.autoUpgradeStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "management.upgrade-options.description" => Some(("management.upgradeOptions.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool-id" => Some(("nodePoolId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_clusters_node_pools_set_size(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-count" => Some(("nodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool-id" => Some(("nodePoolId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_clusters_node_pools_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "confidential-nodes.enabled" => Some(("confidentialNodes.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "containerd-config.private-registry-access-config.enabled" => Some(("containerdConfig.privateRegistryAccessConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "disk-size-gb" => Some(("diskSizeGb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "disk-type" => Some(("diskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fast-socket.enabled" => Some(("fastSocket.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "gcfs-config.enabled" => Some(("gcfsConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "gvnic.enabled" => Some(("gvnic.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "image-type" => Some(("imageType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.cpu-cfs-quota" => Some(("kubeletConfig.cpuCfsQuota", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kubelet-config.cpu-cfs-quota-period" => Some(("kubeletConfig.cpuCfsQuotaPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.cpu-manager-policy" => Some(("kubeletConfig.cpuManagerPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.insecure-kubelet-readonly-port-enabled" => Some(("kubeletConfig.insecureKubeletReadonlyPortEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kubelet-config.pod-pids-limit" => Some(("kubeletConfig.podPidsLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels.labels" => Some(("labels.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "linux-node-config.cgroup-mode" => Some(("linuxNodeConfig.cgroupMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "linux-node-config.hugepages.hugepage-size1g" => Some(("linuxNodeConfig.hugepages.hugepageSize1g", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "linux-node-config.hugepages.hugepage-size2m" => Some(("linuxNodeConfig.hugepages.hugepageSize2m", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "linux-node-config.sysctls" => Some(("linuxNodeConfig.sysctls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "locations" => Some(("locations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "logging-config.variant-config.variant" => Some(("loggingConfig.variantConfig.variant", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "machine-type" => Some(("machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-network-config.create-pod-range" => Some(("nodeNetworkConfig.createPodRange", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-network-config.enable-private-nodes" => Some(("nodeNetworkConfig.enablePrivateNodes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-network-config.network-performance-config.total-egress-bandwidth-tier" => Some(("nodeNetworkConfig.networkPerformanceConfig.totalEgressBandwidthTier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-network-config.pod-cidr-overprovision-config.disable" => Some(("nodeNetworkConfig.podCidrOverprovisionConfig.disable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-network-config.pod-ipv4-cidr-block" => Some(("nodeNetworkConfig.podIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-network-config.pod-ipv4-range-utilization" => Some(("nodeNetworkConfig.podIpv4RangeUtilization", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "node-network-config.pod-range" => Some(("nodeNetworkConfig.podRange", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool-id" => Some(("nodePoolId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-version" => Some(("nodeVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "queued-provisioning.enabled" => Some(("queuedProvisioning.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "resource-labels.labels" => Some(("resourceLabels.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "resource-manager-tags.tags" => Some(("resourceManagerTags.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "tags.tags" => Some(("tags.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "upgrade-settings.blue-green-settings.node-pool-soak-duration" => Some(("upgradeSettings.blueGreenSettings.nodePoolSoakDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade-settings.blue-green-settings.standard-rollout-policy.batch-node-count" => Some(("upgradeSettings.blueGreenSettings.standardRolloutPolicy.batchNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "upgrade-settings.blue-green-settings.standard-rollout-policy.batch-percentage" => Some(("upgradeSettings.blueGreenSettings.standardRolloutPolicy.batchPercentage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "upgrade-settings.blue-green-settings.standard-rollout-policy.batch-soak-duration" => Some(("upgradeSettings.blueGreenSettings.standardRolloutPolicy.batchSoakDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade-settings.max-surge" => Some(("upgradeSettings.maxSurge", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "upgrade-settings.max-unavailable" => Some(("upgradeSettings.maxUnavailable", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "upgrade-settings.strategy" => Some(("upgradeSettings.strategy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "windows-node-config.os-version" => Some(("windowsNodeConfig.osVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload-metadata-config.mode" => Some(("workloadMetadataConfig.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["batch-node-count", "batch-percentage", "batch-soak-duration", "blue-green-settings", "cgroup-mode", "cluster-id", "confidential-nodes", "containerd-config", "cpu-cfs-quota", "cpu-cfs-quota-period", "cpu-manager-policy", "create-pod-range", "disable", "disk-size-gb", "disk-type", "enable-private-nodes", "enabled", "etag", "fast-socket", "gcfs-config", "gvnic", "hugepage-size1g", "hugepage-size2m", "hugepages", "image-type", "insecure-kubelet-readonly-port-enabled", "kubelet-config", "labels", "linux-node-config", "locations", "logging-config", "machine-type", "max-surge", "max-unavailable", "mode", "name", "network-performance-config", "node-network-config", "node-pool-id", "node-pool-soak-duration", "node-version", "os-version", "pod-cidr-overprovision-config", "pod-ipv4-cidr-block", "pod-ipv4-range-utilization", "pod-pids-limit", "pod-range", "private-registry-access-config", "project-id", "queued-provisioning", "resource-labels", "resource-manager-tags", "standard-rollout-policy", "strategy", "sysctls", "tags", "total-egress-bandwidth-tier", "upgrade-settings", "variant", "variant-config", "windows-node-config", "workload-metadata-config", "zone"]);
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_clusters_set_addons(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "addons-config.cloud-run-config.disabled" => Some(("addonsConfig.cloudRunConfig.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "addons-config.cloud-run-config.load-balancer-type" => Some(("addonsConfig.cloudRunConfig.loadBalancerType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "addons-config.config-connector-config.enabled" => Some(("addonsConfig.configConnectorConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "addons-config.dns-cache-config.enabled" => Some(("addonsConfig.dnsCacheConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "addons-config.gce-persistent-disk-csi-driver-config.enabled" => Some(("addonsConfig.gcePersistentDiskCsiDriverConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "addons-config.gcp-filestore-csi-driver-config.enabled" => Some(("addonsConfig.gcpFilestoreCsiDriverConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "addons-config.gcs-fuse-csi-driver-config.enabled" => Some(("addonsConfig.gcsFuseCsiDriverConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "addons-config.gke-backup-agent-config.enabled" => Some(("addonsConfig.gkeBackupAgentConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "addons-config.horizontal-pod-autoscaling.disabled" => Some(("addonsConfig.horizontalPodAutoscaling.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "addons-config.http-load-balancing.disabled" => Some(("addonsConfig.httpLoadBalancing.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "addons-config.kubernetes-dashboard.disabled" => Some(("addonsConfig.kubernetesDashboard.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "addons-config.network-policy-config.disabled" => Some(("addonsConfig.networkPolicyConfig.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "addons-config.stateful-ha-config.enabled" => Some(("addonsConfig.statefulHaConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["addons-config", "cloud-run-config", "cluster-id", "config-connector-config", "disabled", "dns-cache-config", "enabled", "gce-persistent-disk-csi-driver-config", "gcp-filestore-csi-driver-config", "gcs-fuse-csi-driver-config", "gke-backup-agent-config", "horizontal-pod-autoscaling", "http-load-balancing", "kubernetes-dashboard", "load-balancer-type", "name", "network-policy-config", "project-id", "stateful-ha-config", "zone"]);
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_clusters_set_legacy_abac(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "enabled" => Some(("enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_clusters_set_locations(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "locations" => Some(("locations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_clusters_set_logging(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "logging-service" => Some(("loggingService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_clusters_set_maintenance_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "maintenance-policy.resource-version" => Some(("maintenancePolicy.resourceVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "maintenance-policy.window.daily-maintenance-window.duration" => Some(("maintenancePolicy.window.dailyMaintenanceWindow.duration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "maintenance-policy.window.daily-maintenance-window.start-time" => Some(("maintenancePolicy.window.dailyMaintenanceWindow.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "maintenance-policy.window.recurring-window.recurrence" => Some(("maintenancePolicy.window.recurringWindow.recurrence", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "maintenance-policy.window.recurring-window.window.end-time" => Some(("maintenancePolicy.window.recurringWindow.window.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "maintenance-policy.window.recurring-window.window.maintenance-exclusion-options.scope" => Some(("maintenancePolicy.window.recurringWindow.window.maintenanceExclusionOptions.scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "maintenance-policy.window.recurring-window.window.start-time" => Some(("maintenancePolicy.window.recurringWindow.window.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cluster-id", "daily-maintenance-window", "duration", "end-time", "maintenance-exclusion-options", "maintenance-policy", "name", "project-id", "recurrence", "recurring-window", "resource-version", "scope", "start-time", "window", "zone"]);
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_clusters_set_master_auth(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "action" => Some(("action", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.client-certificate" => Some(("update.clientCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.client-certificate-config.issue-client-certificate" => Some(("update.clientCertificateConfig.issueClientCertificate", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.client-key" => Some(("update.clientKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.cluster-ca-certificate" => Some(("update.clusterCaCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.password" => Some(("update.password", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.username" => Some(("update.username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_clusters_set_monitoring(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "monitoring-service" => Some(("monitoringService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_clusters_set_network_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "network-policy.enabled" => Some(("networkPolicy.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "network-policy.provider" => Some(("networkPolicy.provider", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_clusters_set_resource_labels(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "label-fingerprint" => Some(("labelFingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "resource-labels" => Some(("resourceLabels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_clusters_start_ip_rotation(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rotate-credentials" => Some(("rotateCredentials", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_clusters_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.additional-pod-ranges-config.pod-range-names" => Some(("update.additionalPodRangesConfig.podRangeNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.desired-addons-config.cloud-run-config.disabled" => Some(("update.desiredAddonsConfig.cloudRunConfig.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.cloud-run-config.load-balancer-type" => Some(("update.desiredAddonsConfig.cloudRunConfig.loadBalancerType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.config-connector-config.enabled" => Some(("update.desiredAddonsConfig.configConnectorConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.dns-cache-config.enabled" => Some(("update.desiredAddonsConfig.dnsCacheConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.gce-persistent-disk-csi-driver-config.enabled" => Some(("update.desiredAddonsConfig.gcePersistentDiskCsiDriverConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.gcp-filestore-csi-driver-config.enabled" => Some(("update.desiredAddonsConfig.gcpFilestoreCsiDriverConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.gcs-fuse-csi-driver-config.enabled" => Some(("update.desiredAddonsConfig.gcsFuseCsiDriverConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.gke-backup-agent-config.enabled" => Some(("update.desiredAddonsConfig.gkeBackupAgentConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.horizontal-pod-autoscaling.disabled" => Some(("update.desiredAddonsConfig.horizontalPodAutoscaling.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.http-load-balancing.disabled" => Some(("update.desiredAddonsConfig.httpLoadBalancing.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.kubernetes-dashboard.disabled" => Some(("update.desiredAddonsConfig.kubernetesDashboard.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.network-policy-config.disabled" => Some(("update.desiredAddonsConfig.networkPolicyConfig.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.stateful-ha-config.enabled" => Some(("update.desiredAddonsConfig.statefulHaConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-authenticator-groups-config.enabled" => Some(("update.desiredAuthenticatorGroupsConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-authenticator-groups-config.security-group" => Some(("update.desiredAuthenticatorGroupsConfig.securityGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-autopilot-workload-policy-config.allow-net-admin" => Some(("update.desiredAutopilotWorkloadPolicyConfig.allowNetAdmin", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-binary-authorization.enabled" => Some(("update.desiredBinaryAuthorization.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-binary-authorization.evaluation-mode" => Some(("update.desiredBinaryAuthorization.evaluationMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-locations" => Some(("update.desiredClusterAutoscaling.autoprovisioningLocations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.boot-disk-kms-key" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.bootDiskKmsKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.disk-size-gb" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.diskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.disk-type" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.diskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.image-type" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.imageType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.insecure-kubelet-readonly-port-enabled" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.insecureKubeletReadonlyPortEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.management.auto-repair" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.management.autoRepair", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.management.auto-upgrade" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.management.autoUpgrade", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.management.upgrade-options.auto-upgrade-start-time" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.management.upgradeOptions.autoUpgradeStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.management.upgrade-options.description" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.management.upgradeOptions.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.min-cpu-platform" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.minCpuPlatform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.oauth-scopes" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.oauthScopes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.service-account" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.shielded-instance-config.enable-integrity-monitoring" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.shieldedInstanceConfig.enableIntegrityMonitoring", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.shielded-instance-config.enable-secure-boot" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.shieldedInstanceConfig.enableSecureBoot", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.blue-green-settings.node-pool-soak-duration" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.blueGreenSettings.nodePoolSoakDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.blue-green-settings.standard-rollout-policy.batch-node-count" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.blueGreenSettings.standardRolloutPolicy.batchNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.blue-green-settings.standard-rollout-policy.batch-percentage" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.blueGreenSettings.standardRolloutPolicy.batchPercentage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.blue-green-settings.standard-rollout-policy.batch-soak-duration" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.blueGreenSettings.standardRolloutPolicy.batchSoakDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.max-surge" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.maxSurge", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.max-unavailable" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.maxUnavailable", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.strategy" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.strategy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoscaling-profile" => Some(("update.desiredClusterAutoscaling.autoscalingProfile", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.enable-node-autoprovisioning" => Some(("update.desiredClusterAutoscaling.enableNodeAutoprovisioning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-containerd-config.private-registry-access-config.enabled" => Some(("update.desiredContainerdConfig.privateRegistryAccessConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-cost-management-config.enabled" => Some(("update.desiredCostManagementConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-database-encryption.current-state" => Some(("update.desiredDatabaseEncryption.currentState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-database-encryption.decryption-keys" => Some(("update.desiredDatabaseEncryption.decryptionKeys", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.desired-database-encryption.key-name" => Some(("update.desiredDatabaseEncryption.keyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-database-encryption.state" => Some(("update.desiredDatabaseEncryption.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-datapath-provider" => Some(("update.desiredDatapathProvider", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-default-snat-status.disabled" => Some(("update.desiredDefaultSnatStatus.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-dns-config.additive-vpc-scope-dns-domain" => Some(("update.desiredDnsConfig.additiveVpcScopeDnsDomain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-dns-config.cluster-dns" => Some(("update.desiredDnsConfig.clusterDns", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-dns-config.cluster-dns-domain" => Some(("update.desiredDnsConfig.clusterDnsDomain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-dns-config.cluster-dns-scope" => Some(("update.desiredDnsConfig.clusterDnsScope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-enable-cilium-clusterwide-network-policy" => Some(("update.desiredEnableCiliumClusterwideNetworkPolicy", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-enable-fqdn-network-policy" => Some(("update.desiredEnableFqdnNetworkPolicy", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-enable-multi-networking" => Some(("update.desiredEnableMultiNetworking", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-enable-private-endpoint" => Some(("update.desiredEnablePrivateEndpoint", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-fleet.membership" => Some(("update.desiredFleet.membership", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-fleet.pre-registered" => Some(("update.desiredFleet.preRegistered", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-fleet.project" => Some(("update.desiredFleet.project", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-gateway-api-config.channel" => Some(("update.desiredGatewayApiConfig.channel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-gcfs-config.enabled" => Some(("update.desiredGcfsConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-identity-service-config.enabled" => Some(("update.desiredIdentityServiceConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-image-type" => Some(("update.desiredImageType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-in-transit-encryption-config" => Some(("update.desiredInTransitEncryptionConfig", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-intra-node-visibility-config.enabled" => Some(("update.desiredIntraNodeVisibilityConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-k8s-beta-apis.enabled-apis" => Some(("update.desiredK8sBetaApis.enabledApis", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.desired-l4ilb-subsetting-config.enabled" => Some(("update.desiredL4ilbSubsettingConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-locations" => Some(("update.desiredLocations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.desired-logging-config.component-config.enable-components" => Some(("update.desiredLoggingConfig.componentConfig.enableComponents", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.desired-logging-service" => Some(("update.desiredLoggingService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-master-authorized-networks-config.enabled" => Some(("update.desiredMasterAuthorizedNetworksConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-master-authorized-networks-config.gcp-public-cidrs-access-enabled" => Some(("update.desiredMasterAuthorizedNetworksConfig.gcpPublicCidrsAccessEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-master-version" => Some(("update.desiredMasterVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-mesh-certificates.enable-certificates" => Some(("update.desiredMeshCertificates.enableCertificates", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-monitoring-config.advanced-datapath-observability-config.enable-metrics" => Some(("update.desiredMonitoringConfig.advancedDatapathObservabilityConfig.enableMetrics", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-monitoring-config.advanced-datapath-observability-config.enable-relay" => Some(("update.desiredMonitoringConfig.advancedDatapathObservabilityConfig.enableRelay", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-monitoring-config.advanced-datapath-observability-config.relay-mode" => Some(("update.desiredMonitoringConfig.advancedDatapathObservabilityConfig.relayMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-monitoring-config.component-config.enable-components" => Some(("update.desiredMonitoringConfig.componentConfig.enableComponents", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.desired-monitoring-config.managed-prometheus-config.enabled" => Some(("update.desiredMonitoringConfig.managedPrometheusConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-monitoring-service" => Some(("update.desiredMonitoringService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-network-performance-config.total-egress-bandwidth-tier" => Some(("update.desiredNetworkPerformanceConfig.totalEgressBandwidthTier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.cpu-cfs-quota" => Some(("update.desiredNodeKubeletConfig.cpuCfsQuota", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.cpu-cfs-quota-period" => Some(("update.desiredNodeKubeletConfig.cpuCfsQuotaPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.cpu-manager-policy" => Some(("update.desiredNodeKubeletConfig.cpuManagerPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.insecure-kubelet-readonly-port-enabled" => Some(("update.desiredNodeKubeletConfig.insecureKubeletReadonlyPortEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.pod-pids-limit" => Some(("update.desiredNodeKubeletConfig.podPidsLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.cpu-cfs-quota" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.cpuCfsQuota", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.cpu-cfs-quota-period" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.cpuCfsQuotaPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.cpu-manager-policy" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.cpuManagerPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.insecure-kubelet-readonly-port-enabled" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.insecureKubeletReadonlyPortEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.pod-pids-limit" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.podPidsLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-network-tags.tags" => Some(("update.desiredNodePoolAutoConfigNetworkTags.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.desired-node-pool-auto-config-resource-manager-tags.tags" => Some(("update.desiredNodePoolAutoConfigResourceManagerTags.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "update.desired-node-pool-autoscaling.autoprovisioned" => Some(("update.desiredNodePoolAutoscaling.autoprovisioned", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-autoscaling.enabled" => Some(("update.desiredNodePoolAutoscaling.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-autoscaling.location-policy" => Some(("update.desiredNodePoolAutoscaling.locationPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-autoscaling.max-node-count" => Some(("update.desiredNodePoolAutoscaling.maxNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-autoscaling.min-node-count" => Some(("update.desiredNodePoolAutoscaling.minNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-autoscaling.total-max-node-count" => Some(("update.desiredNodePoolAutoscaling.totalMaxNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-autoscaling.total-min-node-count" => Some(("update.desiredNodePoolAutoscaling.totalMinNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-id" => Some(("update.desiredNodePoolId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-logging-config.variant-config.variant" => Some(("update.desiredNodePoolLoggingConfig.variantConfig.variant", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-version" => Some(("update.desiredNodeVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-notification-config.pubsub.enabled" => Some(("update.desiredNotificationConfig.pubsub.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-notification-config.pubsub.filter.event-type" => Some(("update.desiredNotificationConfig.pubsub.filter.eventType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.desired-notification-config.pubsub.topic" => Some(("update.desiredNotificationConfig.pubsub.topic", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-parent-product-config.labels" => Some(("update.desiredParentProductConfig.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "update.desired-parent-product-config.product-name" => Some(("update.desiredParentProductConfig.productName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-private-cluster-config.enable-private-endpoint" => Some(("update.desiredPrivateClusterConfig.enablePrivateEndpoint", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-private-cluster-config.enable-private-nodes" => Some(("update.desiredPrivateClusterConfig.enablePrivateNodes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-private-cluster-config.master-global-access-config.enabled" => Some(("update.desiredPrivateClusterConfig.masterGlobalAccessConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-private-cluster-config.master-ipv4-cidr-block" => Some(("update.desiredPrivateClusterConfig.masterIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-private-cluster-config.peering-name" => Some(("update.desiredPrivateClusterConfig.peeringName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-private-cluster-config.private-endpoint" => Some(("update.desiredPrivateClusterConfig.privateEndpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-private-cluster-config.private-endpoint-subnetwork" => Some(("update.desiredPrivateClusterConfig.privateEndpointSubnetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-private-cluster-config.public-endpoint" => Some(("update.desiredPrivateClusterConfig.publicEndpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-private-ipv6-google-access" => Some(("update.desiredPrivateIpv6GoogleAccess", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-release-channel.channel" => Some(("update.desiredReleaseChannel.channel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-resource-usage-export-config.bigquery-destination.dataset-id" => Some(("update.desiredResourceUsageExportConfig.bigqueryDestination.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-resource-usage-export-config.consumption-metering-config.enabled" => Some(("update.desiredResourceUsageExportConfig.consumptionMeteringConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-resource-usage-export-config.enable-network-egress-metering" => Some(("update.desiredResourceUsageExportConfig.enableNetworkEgressMetering", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-security-posture-config.mode" => Some(("update.desiredSecurityPostureConfig.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-security-posture-config.vulnerability-mode" => Some(("update.desiredSecurityPostureConfig.vulnerabilityMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-service-external-ips-config.enabled" => Some(("update.desiredServiceExternalIpsConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-shielded-nodes.enabled" => Some(("update.desiredShieldedNodes.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-stack-type" => Some(("update.desiredStackType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-vertical-pod-autoscaling.enabled" => Some(("update.desiredVerticalPodAutoscaling.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-workload-identity-config.workload-pool" => Some(("update.desiredWorkloadIdentityConfig.workloadPool", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.enable-k8s-beta-apis.enabled-apis" => Some(("update.enableK8sBetaApis.enabledApis", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.etag" => Some(("update.etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.removed-additional-pod-ranges-config.pod-range-names" => Some(("update.removedAdditionalPodRangesConfig.podRangeNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["additional-pod-ranges-config", "additive-vpc-scope-dns-domain", "advanced-datapath-observability-config", "allow-net-admin", "auto-repair", "auto-upgrade", "auto-upgrade-start-time", "autoprovisioned", "autoprovisioning-locations", "autoprovisioning-node-pool-defaults", "autoscaling-profile", "batch-node-count", "batch-percentage", "batch-soak-duration", "bigquery-destination", "blue-green-settings", "boot-disk-kms-key", "channel", "cloud-run-config", "cluster-dns", "cluster-dns-domain", "cluster-dns-scope", "cluster-id", "component-config", "config-connector-config", "consumption-metering-config", "cpu-cfs-quota", "cpu-cfs-quota-period", "cpu-manager-policy", "current-state", "dataset-id", "decryption-keys", "description", "desired-addons-config", "desired-authenticator-groups-config", "desired-autopilot-workload-policy-config", "desired-binary-authorization", "desired-cluster-autoscaling", "desired-containerd-config", "desired-cost-management-config", "desired-database-encryption", "desired-datapath-provider", "desired-default-snat-status", "desired-dns-config", "desired-enable-cilium-clusterwide-network-policy", "desired-enable-fqdn-network-policy", "desired-enable-multi-networking", "desired-enable-private-endpoint", "desired-fleet", "desired-gateway-api-config", "desired-gcfs-config", "desired-identity-service-config", "desired-image-type", "desired-in-transit-encryption-config", "desired-intra-node-visibility-config", "desired-k8s-beta-apis", "desired-l4ilb-subsetting-config", "desired-locations", "desired-logging-config", "desired-logging-service", "desired-master-authorized-networks-config", "desired-master-version", "desired-mesh-certificates", "desired-monitoring-config", "desired-monitoring-service", "desired-network-performance-config", "desired-node-kubelet-config", "desired-node-pool-auto-config-kubelet-config", "desired-node-pool-auto-config-network-tags", "desired-node-pool-auto-config-resource-manager-tags", "desired-node-pool-autoscaling", "desired-node-pool-id", "desired-node-pool-logging-config", "desired-node-version", "desired-notification-config", "desired-parent-product-config", "desired-private-cluster-config", "desired-private-ipv6-google-access", "desired-release-channel", "desired-resource-usage-export-config", "desired-security-posture-config", "desired-service-external-ips-config", "desired-shielded-nodes", "desired-stack-type", "desired-vertical-pod-autoscaling", "desired-workload-identity-config", "disabled", "disk-size-gb", "disk-type", "dns-cache-config", "enable-certificates", "enable-components", "enable-integrity-monitoring", "enable-k8s-beta-apis", "enable-metrics", "enable-network-egress-metering", "enable-node-autoprovisioning", "enable-private-endpoint", "enable-private-nodes", "enable-relay", "enable-secure-boot", "enabled", "enabled-apis", "etag", "evaluation-mode", "event-type", "filter", "gce-persistent-disk-csi-driver-config", "gcp-filestore-csi-driver-config", "gcp-public-cidrs-access-enabled", "gcs-fuse-csi-driver-config", "gke-backup-agent-config", "horizontal-pod-autoscaling", "http-load-balancing", "image-type", "insecure-kubelet-readonly-port-enabled", "key-name", "kubernetes-dashboard", "labels", "load-balancer-type", "location-policy", "managed-prometheus-config", "management", "master-global-access-config", "master-ipv4-cidr-block", "max-node-count", "max-surge", "max-unavailable", "membership", "min-cpu-platform", "min-node-count", "mode", "name", "network-policy-config", "node-pool-soak-duration", "oauth-scopes", "peering-name", "pod-pids-limit", "pod-range-names", "pre-registered", "private-endpoint", "private-endpoint-subnetwork", "private-registry-access-config", "product-name", "project", "project-id", "public-endpoint", "pubsub", "relay-mode", "removed-additional-pod-ranges-config", "security-group", "service-account", "shielded-instance-config", "standard-rollout-policy", "state", "stateful-ha-config", "strategy", "tags", "topic", "total-egress-bandwidth-tier", "total-max-node-count", "total-min-node-count", "update", "upgrade-options", "upgrade-settings", "variant", "variant-config", "vulnerability-mode", "workload-pool", "zone"]);
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_clusters_update_master(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "master-version" => Some(("masterVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_clusters_well_known_get_openid_configuration(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_get_server_config(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_operations_cancel(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "operation-id" => Some(("operationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_operations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                                                                           v.extend(["operation-id", "project-id", "zone"].iter().map(|v|*v));
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_locations_operations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_zones_clusters_addons(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "addons-config.cloud-run-config.disabled" => Some(("addonsConfig.cloudRunConfig.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "addons-config.cloud-run-config.load-balancer-type" => Some(("addonsConfig.cloudRunConfig.loadBalancerType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "addons-config.config-connector-config.enabled" => Some(("addonsConfig.configConnectorConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "addons-config.dns-cache-config.enabled" => Some(("addonsConfig.dnsCacheConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "addons-config.gce-persistent-disk-csi-driver-config.enabled" => Some(("addonsConfig.gcePersistentDiskCsiDriverConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "addons-config.gcp-filestore-csi-driver-config.enabled" => Some(("addonsConfig.gcpFilestoreCsiDriverConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "addons-config.gcs-fuse-csi-driver-config.enabled" => Some(("addonsConfig.gcsFuseCsiDriverConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "addons-config.gke-backup-agent-config.enabled" => Some(("addonsConfig.gkeBackupAgentConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "addons-config.horizontal-pod-autoscaling.disabled" => Some(("addonsConfig.horizontalPodAutoscaling.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "addons-config.http-load-balancing.disabled" => Some(("addonsConfig.httpLoadBalancing.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "addons-config.kubernetes-dashboard.disabled" => Some(("addonsConfig.kubernetesDashboard.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "addons-config.network-policy-config.disabled" => Some(("addonsConfig.networkPolicyConfig.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "addons-config.stateful-ha-config.enabled" => Some(("addonsConfig.statefulHaConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["addons-config", "cloud-run-config", "cluster-id", "config-connector-config", "disabled", "dns-cache-config", "enabled", "gce-persistent-disk-csi-driver-config", "gcp-filestore-csi-driver-config", "gcs-fuse-csi-driver-config", "gke-backup-agent-config", "horizontal-pod-autoscaling", "http-load-balancing", "kubernetes-dashboard", "load-balancer-type", "name", "network-policy-config", "project-id", "stateful-ha-config", "zone"]);
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_zones_clusters_complete_ip_rotation(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_zones_clusters_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cluster.addons-config.cloud-run-config.disabled" => Some(("cluster.addonsConfig.cloudRunConfig.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.cloud-run-config.load-balancer-type" => Some(("cluster.addonsConfig.cloudRunConfig.loadBalancerType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.addons-config.config-connector-config.enabled" => Some(("cluster.addonsConfig.configConnectorConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.dns-cache-config.enabled" => Some(("cluster.addonsConfig.dnsCacheConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.gce-persistent-disk-csi-driver-config.enabled" => Some(("cluster.addonsConfig.gcePersistentDiskCsiDriverConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.gcp-filestore-csi-driver-config.enabled" => Some(("cluster.addonsConfig.gcpFilestoreCsiDriverConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.gcs-fuse-csi-driver-config.enabled" => Some(("cluster.addonsConfig.gcsFuseCsiDriverConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.gke-backup-agent-config.enabled" => Some(("cluster.addonsConfig.gkeBackupAgentConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.horizontal-pod-autoscaling.disabled" => Some(("cluster.addonsConfig.horizontalPodAutoscaling.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.http-load-balancing.disabled" => Some(("cluster.addonsConfig.httpLoadBalancing.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.kubernetes-dashboard.disabled" => Some(("cluster.addonsConfig.kubernetesDashboard.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.network-policy-config.disabled" => Some(("cluster.addonsConfig.networkPolicyConfig.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.stateful-ha-config.enabled" => Some(("cluster.addonsConfig.statefulHaConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.authenticator-groups-config.enabled" => Some(("cluster.authenticatorGroupsConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.authenticator-groups-config.security-group" => Some(("cluster.authenticatorGroupsConfig.securityGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.autopilot.enabled" => Some(("cluster.autopilot.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.autopilot.workload-policy-config.allow-net-admin" => Some(("cluster.autopilot.workloadPolicyConfig.allowNetAdmin", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-locations" => Some(("cluster.autoscaling.autoprovisioningLocations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.boot-disk-kms-key" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.bootDiskKmsKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.disk-size-gb" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.diskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.disk-type" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.diskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.image-type" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.imageType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.insecure-kubelet-readonly-port-enabled" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.insecureKubeletReadonlyPortEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.management.auto-repair" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.management.autoRepair", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.management.auto-upgrade" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.management.autoUpgrade", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.management.upgrade-options.auto-upgrade-start-time" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.management.upgradeOptions.autoUpgradeStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.management.upgrade-options.description" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.management.upgradeOptions.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.min-cpu-platform" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.minCpuPlatform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.oauth-scopes" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.oauthScopes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.service-account" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.shielded-instance-config.enable-integrity-monitoring" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.shieldedInstanceConfig.enableIntegrityMonitoring", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.shielded-instance-config.enable-secure-boot" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.shieldedInstanceConfig.enableSecureBoot", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.blue-green-settings.node-pool-soak-duration" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.blueGreenSettings.nodePoolSoakDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.blue-green-settings.standard-rollout-policy.batch-node-count" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.blueGreenSettings.standardRolloutPolicy.batchNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.blue-green-settings.standard-rollout-policy.batch-percentage" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.blueGreenSettings.standardRolloutPolicy.batchPercentage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.blue-green-settings.standard-rollout-policy.batch-soak-duration" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.blueGreenSettings.standardRolloutPolicy.batchSoakDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.max-surge" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.maxSurge", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.max-unavailable" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.maxUnavailable", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.strategy" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.strategy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoscaling-profile" => Some(("cluster.autoscaling.autoscalingProfile", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.enable-node-autoprovisioning" => Some(("cluster.autoscaling.enableNodeAutoprovisioning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.binary-authorization.enabled" => Some(("cluster.binaryAuthorization.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.binary-authorization.evaluation-mode" => Some(("cluster.binaryAuthorization.evaluationMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.cluster-ipv4-cidr" => Some(("cluster.clusterIpv4Cidr", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.confidential-nodes.enabled" => Some(("cluster.confidentialNodes.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.cost-management-config.enabled" => Some(("cluster.costManagementConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.create-time" => Some(("cluster.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.current-master-version" => Some(("cluster.currentMasterVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.current-node-count" => Some(("cluster.currentNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.current-node-version" => Some(("cluster.currentNodeVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.database-encryption.current-state" => Some(("cluster.databaseEncryption.currentState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.database-encryption.decryption-keys" => Some(("cluster.databaseEncryption.decryptionKeys", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.database-encryption.key-name" => Some(("cluster.databaseEncryption.keyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.database-encryption.state" => Some(("cluster.databaseEncryption.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.default-max-pods-constraint.max-pods-per-node" => Some(("cluster.defaultMaxPodsConstraint.maxPodsPerNode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.description" => Some(("cluster.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.enable-k8s-beta-apis.enabled-apis" => Some(("cluster.enableK8sBetaApis.enabledApis", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.enable-kubernetes-alpha" => Some(("cluster.enableKubernetesAlpha", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.enable-tpu" => Some(("cluster.enableTpu", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.endpoint" => Some(("cluster.endpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.enterprise-config.cluster-tier" => Some(("cluster.enterpriseConfig.clusterTier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.etag" => Some(("cluster.etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.expire-time" => Some(("cluster.expireTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.fleet.membership" => Some(("cluster.fleet.membership", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.fleet.pre-registered" => Some(("cluster.fleet.preRegistered", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.fleet.project" => Some(("cluster.fleet.project", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.id" => Some(("cluster.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.identity-service-config.enabled" => Some(("cluster.identityServiceConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.initial-cluster-version" => Some(("cluster.initialClusterVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.initial-node-count" => Some(("cluster.initialNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.instance-group-urls" => Some(("cluster.instanceGroupUrls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.ip-allocation-policy.additional-pod-ranges-config.pod-range-names" => Some(("cluster.ipAllocationPolicy.additionalPodRangesConfig.podRangeNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.ip-allocation-policy.cluster-ipv4-cidr" => Some(("cluster.ipAllocationPolicy.clusterIpv4Cidr", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.cluster-ipv4-cidr-block" => Some(("cluster.ipAllocationPolicy.clusterIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.cluster-secondary-range-name" => Some(("cluster.ipAllocationPolicy.clusterSecondaryRangeName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.create-subnetwork" => Some(("cluster.ipAllocationPolicy.createSubnetwork", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.default-pod-ipv4-range-utilization" => Some(("cluster.ipAllocationPolicy.defaultPodIpv4RangeUtilization", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.ipv6-access-type" => Some(("cluster.ipAllocationPolicy.ipv6AccessType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.node-ipv4-cidr" => Some(("cluster.ipAllocationPolicy.nodeIpv4Cidr", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.node-ipv4-cidr-block" => Some(("cluster.ipAllocationPolicy.nodeIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.pod-cidr-overprovision-config.disable" => Some(("cluster.ipAllocationPolicy.podCidrOverprovisionConfig.disable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.services-ipv4-cidr" => Some(("cluster.ipAllocationPolicy.servicesIpv4Cidr", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.services-ipv4-cidr-block" => Some(("cluster.ipAllocationPolicy.servicesIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.services-ipv6-cidr-block" => Some(("cluster.ipAllocationPolicy.servicesIpv6CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.services-secondary-range-name" => Some(("cluster.ipAllocationPolicy.servicesSecondaryRangeName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.stack-type" => Some(("cluster.ipAllocationPolicy.stackType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.subnet-ipv6-cidr-block" => Some(("cluster.ipAllocationPolicy.subnetIpv6CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.subnetwork-name" => Some(("cluster.ipAllocationPolicy.subnetworkName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.tpu-ipv4-cidr-block" => Some(("cluster.ipAllocationPolicy.tpuIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.use-ip-aliases" => Some(("cluster.ipAllocationPolicy.useIpAliases", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.use-routes" => Some(("cluster.ipAllocationPolicy.useRoutes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.label-fingerprint" => Some(("cluster.labelFingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.legacy-abac.enabled" => Some(("cluster.legacyAbac.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.location" => Some(("cluster.location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.locations" => Some(("cluster.locations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.logging-config.component-config.enable-components" => Some(("cluster.loggingConfig.componentConfig.enableComponents", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.logging-service" => Some(("cluster.loggingService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.maintenance-policy.resource-version" => Some(("cluster.maintenancePolicy.resourceVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.maintenance-policy.window.daily-maintenance-window.duration" => Some(("cluster.maintenancePolicy.window.dailyMaintenanceWindow.duration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.maintenance-policy.window.daily-maintenance-window.start-time" => Some(("cluster.maintenancePolicy.window.dailyMaintenanceWindow.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.maintenance-policy.window.recurring-window.recurrence" => Some(("cluster.maintenancePolicy.window.recurringWindow.recurrence", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.maintenance-policy.window.recurring-window.window.end-time" => Some(("cluster.maintenancePolicy.window.recurringWindow.window.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.maintenance-policy.window.recurring-window.window.maintenance-exclusion-options.scope" => Some(("cluster.maintenancePolicy.window.recurringWindow.window.maintenanceExclusionOptions.scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.maintenance-policy.window.recurring-window.window.start-time" => Some(("cluster.maintenancePolicy.window.recurringWindow.window.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.master-auth.client-certificate" => Some(("cluster.masterAuth.clientCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.master-auth.client-certificate-config.issue-client-certificate" => Some(("cluster.masterAuth.clientCertificateConfig.issueClientCertificate", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.master-auth.client-key" => Some(("cluster.masterAuth.clientKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.master-auth.cluster-ca-certificate" => Some(("cluster.masterAuth.clusterCaCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.master-auth.password" => Some(("cluster.masterAuth.password", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.master-auth.username" => Some(("cluster.masterAuth.username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.master-authorized-networks-config.enabled" => Some(("cluster.masterAuthorizedNetworksConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.master-authorized-networks-config.gcp-public-cidrs-access-enabled" => Some(("cluster.masterAuthorizedNetworksConfig.gcpPublicCidrsAccessEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.mesh-certificates.enable-certificates" => Some(("cluster.meshCertificates.enableCertificates", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.monitoring-config.advanced-datapath-observability-config.enable-metrics" => Some(("cluster.monitoringConfig.advancedDatapathObservabilityConfig.enableMetrics", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.monitoring-config.advanced-datapath-observability-config.enable-relay" => Some(("cluster.monitoringConfig.advancedDatapathObservabilityConfig.enableRelay", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.monitoring-config.advanced-datapath-observability-config.relay-mode" => Some(("cluster.monitoringConfig.advancedDatapathObservabilityConfig.relayMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.monitoring-config.component-config.enable-components" => Some(("cluster.monitoringConfig.componentConfig.enableComponents", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.monitoring-config.managed-prometheus-config.enabled" => Some(("cluster.monitoringConfig.managedPrometheusConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.monitoring-service" => Some(("cluster.monitoringService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.name" => Some(("cluster.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.network" => Some(("cluster.network", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.network-config.datapath-provider" => Some(("cluster.networkConfig.datapathProvider", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.network-config.default-snat-status.disabled" => Some(("cluster.networkConfig.defaultSnatStatus.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.network-config.dns-config.additive-vpc-scope-dns-domain" => Some(("cluster.networkConfig.dnsConfig.additiveVpcScopeDnsDomain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.network-config.dns-config.cluster-dns" => Some(("cluster.networkConfig.dnsConfig.clusterDns", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.network-config.dns-config.cluster-dns-domain" => Some(("cluster.networkConfig.dnsConfig.clusterDnsDomain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.network-config.dns-config.cluster-dns-scope" => Some(("cluster.networkConfig.dnsConfig.clusterDnsScope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.network-config.enable-cilium-clusterwide-network-policy" => Some(("cluster.networkConfig.enableCiliumClusterwideNetworkPolicy", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.network-config.enable-fqdn-network-policy" => Some(("cluster.networkConfig.enableFqdnNetworkPolicy", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.network-config.enable-intra-node-visibility" => Some(("cluster.networkConfig.enableIntraNodeVisibility", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.network-config.enable-l4ilb-subsetting" => Some(("cluster.networkConfig.enableL4ilbSubsetting", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.network-config.enable-multi-networking" => Some(("cluster.networkConfig.enableMultiNetworking", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.network-config.gateway-api-config.channel" => Some(("cluster.networkConfig.gatewayApiConfig.channel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.network-config.in-transit-encryption-config" => Some(("cluster.networkConfig.inTransitEncryptionConfig", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.network-config.network" => Some(("cluster.networkConfig.network", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.network-config.network-performance-config.total-egress-bandwidth-tier" => Some(("cluster.networkConfig.networkPerformanceConfig.totalEgressBandwidthTier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.network-config.private-ipv6-google-access" => Some(("cluster.networkConfig.privateIpv6GoogleAccess", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.network-config.service-external-ips-config.enabled" => Some(("cluster.networkConfig.serviceExternalIpsConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.network-config.subnetwork" => Some(("cluster.networkConfig.subnetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.network-policy.enabled" => Some(("cluster.networkPolicy.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.network-policy.provider" => Some(("cluster.networkPolicy.provider", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.advanced-machine-features.enable-nested-virtualization" => Some(("cluster.nodeConfig.advancedMachineFeatures.enableNestedVirtualization", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.advanced-machine-features.threads-per-core" => Some(("cluster.nodeConfig.advancedMachineFeatures.threadsPerCore", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.boot-disk-kms-key" => Some(("cluster.nodeConfig.bootDiskKmsKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.confidential-nodes.enabled" => Some(("cluster.nodeConfig.confidentialNodes.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.containerd-config.private-registry-access-config.enabled" => Some(("cluster.nodeConfig.containerdConfig.privateRegistryAccessConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.disk-size-gb" => Some(("cluster.nodeConfig.diskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-config.disk-type" => Some(("cluster.nodeConfig.diskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.enable-confidential-storage" => Some(("cluster.nodeConfig.enableConfidentialStorage", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.ephemeral-storage-local-ssd-config.local-ssd-count" => Some(("cluster.nodeConfig.ephemeralStorageLocalSsdConfig.localSsdCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-config.fast-socket.enabled" => Some(("cluster.nodeConfig.fastSocket.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.gcfs-config.enabled" => Some(("cluster.nodeConfig.gcfsConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.gvnic.enabled" => Some(("cluster.nodeConfig.gvnic.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.image-type" => Some(("cluster.nodeConfig.imageType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.cpu-cfs-quota" => Some(("cluster.nodeConfig.kubeletConfig.cpuCfsQuota", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.cpu-cfs-quota-period" => Some(("cluster.nodeConfig.kubeletConfig.cpuCfsQuotaPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.cpu-manager-policy" => Some(("cluster.nodeConfig.kubeletConfig.cpuManagerPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.insecure-kubelet-readonly-port-enabled" => Some(("cluster.nodeConfig.kubeletConfig.insecureKubeletReadonlyPortEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.pod-pids-limit" => Some(("cluster.nodeConfig.kubeletConfig.podPidsLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.labels" => Some(("cluster.nodeConfig.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "cluster.node-config.linux-node-config.cgroup-mode" => Some(("cluster.nodeConfig.linuxNodeConfig.cgroupMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.linux-node-config.hugepages.hugepage-size1g" => Some(("cluster.nodeConfig.linuxNodeConfig.hugepages.hugepageSize1g", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-config.linux-node-config.hugepages.hugepage-size2m" => Some(("cluster.nodeConfig.linuxNodeConfig.hugepages.hugepageSize2m", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-config.linux-node-config.sysctls" => Some(("cluster.nodeConfig.linuxNodeConfig.sysctls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "cluster.node-config.local-nvme-ssd-block-config.local-ssd-count" => Some(("cluster.nodeConfig.localNvmeSsdBlockConfig.localSsdCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-config.local-ssd-count" => Some(("cluster.nodeConfig.localSsdCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-config.logging-config.variant-config.variant" => Some(("cluster.nodeConfig.loggingConfig.variantConfig.variant", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.machine-type" => Some(("cluster.nodeConfig.machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.metadata" => Some(("cluster.nodeConfig.metadata", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "cluster.node-config.min-cpu-platform" => Some(("cluster.nodeConfig.minCpuPlatform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.node-group" => Some(("cluster.nodeConfig.nodeGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.oauth-scopes" => Some(("cluster.nodeConfig.oauthScopes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.node-config.preemptible" => Some(("cluster.nodeConfig.preemptible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.reservation-affinity.consume-reservation-type" => Some(("cluster.nodeConfig.reservationAffinity.consumeReservationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.reservation-affinity.key" => Some(("cluster.nodeConfig.reservationAffinity.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.reservation-affinity.values" => Some(("cluster.nodeConfig.reservationAffinity.values", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.node-config.resource-labels" => Some(("cluster.nodeConfig.resourceLabels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "cluster.node-config.resource-manager-tags.tags" => Some(("cluster.nodeConfig.resourceManagerTags.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "cluster.node-config.sandbox-config.type" => Some(("cluster.nodeConfig.sandboxConfig.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.service-account" => Some(("cluster.nodeConfig.serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.shielded-instance-config.enable-integrity-monitoring" => Some(("cluster.nodeConfig.shieldedInstanceConfig.enableIntegrityMonitoring", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.shielded-instance-config.enable-secure-boot" => Some(("cluster.nodeConfig.shieldedInstanceConfig.enableSecureBoot", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.spot" => Some(("cluster.nodeConfig.spot", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.tags" => Some(("cluster.nodeConfig.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.node-config.windows-node-config.os-version" => Some(("cluster.nodeConfig.windowsNodeConfig.osVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.workload-metadata-config.mode" => Some(("cluster.nodeConfig.workloadMetadataConfig.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-ipv4-cidr-size" => Some(("cluster.nodeIpv4CidrSize", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.network-tags.tags" => Some(("cluster.nodePoolAutoConfig.networkTags.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.node-pool-auto-config.node-kubelet-config.cpu-cfs-quota" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.cpuCfsQuota", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.cpu-cfs-quota-period" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.cpuCfsQuotaPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.cpu-manager-policy" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.cpuManagerPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.insecure-kubelet-readonly-port-enabled" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.insecureKubeletReadonlyPortEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.pod-pids-limit" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.podPidsLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.resource-manager-tags.tags" => Some(("cluster.nodePoolAutoConfig.resourceManagerTags.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "cluster.node-pool-defaults.node-config-defaults.containerd-config.private-registry-access-config.enabled" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.containerdConfig.privateRegistryAccessConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.gcfs-config.enabled" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.gcfsConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.logging-config.variant-config.variant" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.loggingConfig.variantConfig.variant", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.cpu-cfs-quota" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.cpuCfsQuota", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.cpu-cfs-quota-period" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.cpuCfsQuotaPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.cpu-manager-policy" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.cpuManagerPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.insecure-kubelet-readonly-port-enabled" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.insecureKubeletReadonlyPortEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.pod-pids-limit" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.podPidsLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.notification-config.pubsub.enabled" => Some(("cluster.notificationConfig.pubsub.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.notification-config.pubsub.filter.event-type" => Some(("cluster.notificationConfig.pubsub.filter.eventType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.notification-config.pubsub.topic" => Some(("cluster.notificationConfig.pubsub.topic", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.parent-product-config.labels" => Some(("cluster.parentProductConfig.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "cluster.parent-product-config.product-name" => Some(("cluster.parentProductConfig.productName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.enable-private-endpoint" => Some(("cluster.privateClusterConfig.enablePrivateEndpoint", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.enable-private-nodes" => Some(("cluster.privateClusterConfig.enablePrivateNodes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.master-global-access-config.enabled" => Some(("cluster.privateClusterConfig.masterGlobalAccessConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.master-ipv4-cidr-block" => Some(("cluster.privateClusterConfig.masterIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.peering-name" => Some(("cluster.privateClusterConfig.peeringName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.private-endpoint" => Some(("cluster.privateClusterConfig.privateEndpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.private-endpoint-subnetwork" => Some(("cluster.privateClusterConfig.privateEndpointSubnetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.public-endpoint" => Some(("cluster.privateClusterConfig.publicEndpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.release-channel.channel" => Some(("cluster.releaseChannel.channel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.resource-labels" => Some(("cluster.resourceLabels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "cluster.resource-usage-export-config.bigquery-destination.dataset-id" => Some(("cluster.resourceUsageExportConfig.bigqueryDestination.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.resource-usage-export-config.consumption-metering-config.enabled" => Some(("cluster.resourceUsageExportConfig.consumptionMeteringConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.resource-usage-export-config.enable-network-egress-metering" => Some(("cluster.resourceUsageExportConfig.enableNetworkEgressMetering", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.satisfies-pzi" => Some(("cluster.satisfiesPzi", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.satisfies-pzs" => Some(("cluster.satisfiesPzs", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.security-posture-config.mode" => Some(("cluster.securityPostureConfig.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.security-posture-config.vulnerability-mode" => Some(("cluster.securityPostureConfig.vulnerabilityMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.self-link" => Some(("cluster.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.services-ipv4-cidr" => Some(("cluster.servicesIpv4Cidr", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.shielded-nodes.enabled" => Some(("cluster.shieldedNodes.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.status" => Some(("cluster.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.status-message" => Some(("cluster.statusMessage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.subnetwork" => Some(("cluster.subnetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.tpu-ipv4-cidr-block" => Some(("cluster.tpuIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.vertical-pod-autoscaling.enabled" => Some(("cluster.verticalPodAutoscaling.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.workload-identity-config.workload-pool" => Some(("cluster.workloadIdentityConfig.workloadPool", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.zone" => Some(("cluster.zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent" => Some(("parent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["additional-pod-ranges-config", "additive-vpc-scope-dns-domain", "addons-config", "advanced-datapath-observability-config", "advanced-machine-features", "allow-net-admin", "authenticator-groups-config", "auto-repair", "auto-upgrade", "auto-upgrade-start-time", "autopilot", "autoprovisioning-locations", "autoprovisioning-node-pool-defaults", "autoscaling", "autoscaling-profile", "batch-node-count", "batch-percentage", "batch-soak-duration", "bigquery-destination", "binary-authorization", "blue-green-settings", "boot-disk-kms-key", "cgroup-mode", "channel", "client-certificate", "client-certificate-config", "client-key", "cloud-run-config", "cluster", "cluster-ca-certificate", "cluster-dns", "cluster-dns-domain", "cluster-dns-scope", "cluster-ipv4-cidr", "cluster-ipv4-cidr-block", "cluster-secondary-range-name", "cluster-tier", "component-config", "confidential-nodes", "config-connector-config", "consume-reservation-type", "consumption-metering-config", "containerd-config", "cost-management-config", "cpu-cfs-quota", "cpu-cfs-quota-period", "cpu-manager-policy", "create-subnetwork", "create-time", "current-master-version", "current-node-count", "current-node-version", "current-state", "daily-maintenance-window", "database-encryption", "datapath-provider", "dataset-id", "decryption-keys", "default-max-pods-constraint", "default-pod-ipv4-range-utilization", "default-snat-status", "description", "disable", "disabled", "disk-size-gb", "disk-type", "dns-cache-config", "dns-config", "duration", "enable-certificates", "enable-cilium-clusterwide-network-policy", "enable-components", "enable-confidential-storage", "enable-fqdn-network-policy", "enable-integrity-monitoring", "enable-intra-node-visibility", "enable-k8s-beta-apis", "enable-kubernetes-alpha", "enable-l4ilb-subsetting", "enable-metrics", "enable-multi-networking", "enable-nested-virtualization", "enable-network-egress-metering", "enable-node-autoprovisioning", "enable-private-endpoint", "enable-private-nodes", "enable-relay", "enable-secure-boot", "enable-tpu", "enabled", "enabled-apis", "end-time", "endpoint", "enterprise-config", "ephemeral-storage-local-ssd-config", "etag", "evaluation-mode", "event-type", "expire-time", "fast-socket", "filter", "fleet", "gateway-api-config", "gce-persistent-disk-csi-driver-config", "gcfs-config", "gcp-filestore-csi-driver-config", "gcp-public-cidrs-access-enabled", "gcs-fuse-csi-driver-config", "gke-backup-agent-config", "gvnic", "horizontal-pod-autoscaling", "http-load-balancing", "hugepage-size1g", "hugepage-size2m", "hugepages", "id", "identity-service-config", "image-type", "in-transit-encryption-config", "initial-cluster-version", "initial-node-count", "insecure-kubelet-readonly-port-enabled", "instance-group-urls", "ip-allocation-policy", "ipv6-access-type", "issue-client-certificate", "key", "key-name", "kubelet-config", "kubernetes-dashboard", "label-fingerprint", "labels", "legacy-abac", "linux-node-config", "load-balancer-type", "local-nvme-ssd-block-config", "local-ssd-count", "location", "locations", "logging-config", "logging-service", "machine-type", "maintenance-exclusion-options", "maintenance-policy", "managed-prometheus-config", "management", "master-auth", "master-authorized-networks-config", "master-global-access-config", "master-ipv4-cidr-block", "max-pods-per-node", "max-surge", "max-unavailable", "membership", "mesh-certificates", "metadata", "min-cpu-platform", "mode", "monitoring-config", "monitoring-service", "name", "network", "network-config", "network-performance-config", "network-policy", "network-policy-config", "network-tags", "node-config", "node-config-defaults", "node-group", "node-ipv4-cidr", "node-ipv4-cidr-block", "node-ipv4-cidr-size", "node-kubelet-config", "node-pool-auto-config", "node-pool-defaults", "node-pool-soak-duration", "notification-config", "oauth-scopes", "os-version", "parent", "parent-product-config", "password", "peering-name", "pod-cidr-overprovision-config", "pod-pids-limit", "pod-range-names", "pre-registered", "preemptible", "private-cluster-config", "private-endpoint", "private-endpoint-subnetwork", "private-ipv6-google-access", "private-registry-access-config", "product-name", "project", "project-id", "provider", "public-endpoint", "pubsub", "recurrence", "recurring-window", "relay-mode", "release-channel", "reservation-affinity", "resource-labels", "resource-manager-tags", "resource-usage-export-config", "resource-version", "sandbox-config", "satisfies-pzi", "satisfies-pzs", "scope", "security-group", "security-posture-config", "self-link", "service-account", "service-external-ips-config", "services-ipv4-cidr", "services-ipv4-cidr-block", "services-ipv6-cidr-block", "services-secondary-range-name", "shielded-instance-config", "shielded-nodes", "spot", "stack-type", "standard-rollout-policy", "start-time", "state", "stateful-ha-config", "status", "status-message", "strategy", "subnet-ipv6-cidr-block", "subnetwork", "subnetwork-name", "sysctls", "tags", "threads-per-core", "topic", "total-egress-bandwidth-tier", "tpu-ipv4-cidr-block", "type", "upgrade-options", "upgrade-settings", "use-ip-aliases", "use-routes", "username", "values", "variant", "variant-config", "vertical-pod-autoscaling", "vulnerability-mode", "window", "windows-node-config", "workload-identity-config", "workload-metadata-config", "workload-policy-config", "workload-pool", "zone"]);
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_zones_clusters_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_zones_clusters_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_zones_clusters_legacy_abac(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "enabled" => Some(("enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_zones_clusters_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_zones_clusters_locations(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "locations" => Some(("locations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_zones_clusters_logging(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "logging-service" => Some(("loggingService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_zones_clusters_master(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "master-version" => Some(("masterVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_zones_clusters_monitoring(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "monitoring-service" => Some(("monitoringService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_zones_clusters_node_pools_autoscaling(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "autoscaling.autoprovisioned" => Some(("autoscaling.autoprovisioned", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "autoscaling.enabled" => Some(("autoscaling.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "autoscaling.location-policy" => Some(("autoscaling.locationPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "autoscaling.max-node-count" => Some(("autoscaling.maxNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "autoscaling.min-node-count" => Some(("autoscaling.minNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "autoscaling.total-max-node-count" => Some(("autoscaling.totalMaxNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "autoscaling.total-min-node-count" => Some(("autoscaling.totalMinNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool-id" => Some(("nodePoolId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["autoprovisioned", "autoscaling", "cluster-id", "enabled", "location-policy", "max-node-count", "min-node-count", "name", "node-pool-id", "project-id", "total-max-node-count", "total-min-node-count", "zone"]);
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_zones_clusters_node_pools_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.autoscaling.autoprovisioned" => Some(("nodePool.autoscaling.autoprovisioned", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.autoscaling.enabled" => Some(("nodePool.autoscaling.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.autoscaling.location-policy" => Some(("nodePool.autoscaling.locationPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.autoscaling.max-node-count" => Some(("nodePool.autoscaling.maxNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.autoscaling.min-node-count" => Some(("nodePool.autoscaling.minNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.autoscaling.total-max-node-count" => Some(("nodePool.autoscaling.totalMaxNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.autoscaling.total-min-node-count" => Some(("nodePool.autoscaling.totalMinNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.best-effort-provisioning.enabled" => Some(("nodePool.bestEffortProvisioning.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.best-effort-provisioning.min-provision-nodes" => Some(("nodePool.bestEffortProvisioning.minProvisionNodes", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.advanced-machine-features.enable-nested-virtualization" => Some(("nodePool.config.advancedMachineFeatures.enableNestedVirtualization", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.advanced-machine-features.threads-per-core" => Some(("nodePool.config.advancedMachineFeatures.threadsPerCore", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.boot-disk-kms-key" => Some(("nodePool.config.bootDiskKmsKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.confidential-nodes.enabled" => Some(("nodePool.config.confidentialNodes.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.containerd-config.private-registry-access-config.enabled" => Some(("nodePool.config.containerdConfig.privateRegistryAccessConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.disk-size-gb" => Some(("nodePool.config.diskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.disk-type" => Some(("nodePool.config.diskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.enable-confidential-storage" => Some(("nodePool.config.enableConfidentialStorage", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.ephemeral-storage-local-ssd-config.local-ssd-count" => Some(("nodePool.config.ephemeralStorageLocalSsdConfig.localSsdCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.fast-socket.enabled" => Some(("nodePool.config.fastSocket.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.gcfs-config.enabled" => Some(("nodePool.config.gcfsConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.gvnic.enabled" => Some(("nodePool.config.gvnic.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.image-type" => Some(("nodePool.config.imageType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.cpu-cfs-quota" => Some(("nodePool.config.kubeletConfig.cpuCfsQuota", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.cpu-cfs-quota-period" => Some(("nodePool.config.kubeletConfig.cpuCfsQuotaPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.cpu-manager-policy" => Some(("nodePool.config.kubeletConfig.cpuManagerPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.insecure-kubelet-readonly-port-enabled" => Some(("nodePool.config.kubeletConfig.insecureKubeletReadonlyPortEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.pod-pids-limit" => Some(("nodePool.config.kubeletConfig.podPidsLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.labels" => Some(("nodePool.config.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "node-pool.config.linux-node-config.cgroup-mode" => Some(("nodePool.config.linuxNodeConfig.cgroupMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.linux-node-config.hugepages.hugepage-size1g" => Some(("nodePool.config.linuxNodeConfig.hugepages.hugepageSize1g", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.linux-node-config.hugepages.hugepage-size2m" => Some(("nodePool.config.linuxNodeConfig.hugepages.hugepageSize2m", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.linux-node-config.sysctls" => Some(("nodePool.config.linuxNodeConfig.sysctls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "node-pool.config.local-nvme-ssd-block-config.local-ssd-count" => Some(("nodePool.config.localNvmeSsdBlockConfig.localSsdCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.local-ssd-count" => Some(("nodePool.config.localSsdCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.logging-config.variant-config.variant" => Some(("nodePool.config.loggingConfig.variantConfig.variant", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.machine-type" => Some(("nodePool.config.machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.metadata" => Some(("nodePool.config.metadata", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "node-pool.config.min-cpu-platform" => Some(("nodePool.config.minCpuPlatform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.node-group" => Some(("nodePool.config.nodeGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.oauth-scopes" => Some(("nodePool.config.oauthScopes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "node-pool.config.preemptible" => Some(("nodePool.config.preemptible", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.reservation-affinity.consume-reservation-type" => Some(("nodePool.config.reservationAffinity.consumeReservationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.reservation-affinity.key" => Some(("nodePool.config.reservationAffinity.key", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.reservation-affinity.values" => Some(("nodePool.config.reservationAffinity.values", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "node-pool.config.resource-labels" => Some(("nodePool.config.resourceLabels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "node-pool.config.resource-manager-tags.tags" => Some(("nodePool.config.resourceManagerTags.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "node-pool.config.sandbox-config.type" => Some(("nodePool.config.sandboxConfig.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.service-account" => Some(("nodePool.config.serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.shielded-instance-config.enable-integrity-monitoring" => Some(("nodePool.config.shieldedInstanceConfig.enableIntegrityMonitoring", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.shielded-instance-config.enable-secure-boot" => Some(("nodePool.config.shieldedInstanceConfig.enableSecureBoot", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.spot" => Some(("nodePool.config.spot", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.tags" => Some(("nodePool.config.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "node-pool.config.windows-node-config.os-version" => Some(("nodePool.config.windowsNodeConfig.osVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.workload-metadata-config.mode" => Some(("nodePool.config.workloadMetadataConfig.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.etag" => Some(("nodePool.etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.initial-node-count" => Some(("nodePool.initialNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.instance-group-urls" => Some(("nodePool.instanceGroupUrls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "node-pool.locations" => Some(("nodePool.locations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "node-pool.management.auto-repair" => Some(("nodePool.management.autoRepair", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.management.auto-upgrade" => Some(("nodePool.management.autoUpgrade", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.management.upgrade-options.auto-upgrade-start-time" => Some(("nodePool.management.upgradeOptions.autoUpgradeStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.management.upgrade-options.description" => Some(("nodePool.management.upgradeOptions.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.max-pods-constraint.max-pods-per-node" => Some(("nodePool.maxPodsConstraint.maxPodsPerNode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.name" => Some(("nodePool.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.network-config.create-pod-range" => Some(("nodePool.networkConfig.createPodRange", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.network-config.enable-private-nodes" => Some(("nodePool.networkConfig.enablePrivateNodes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.network-config.network-performance-config.total-egress-bandwidth-tier" => Some(("nodePool.networkConfig.networkPerformanceConfig.totalEgressBandwidthTier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.network-config.pod-cidr-overprovision-config.disable" => Some(("nodePool.networkConfig.podCidrOverprovisionConfig.disable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.network-config.pod-ipv4-cidr-block" => Some(("nodePool.networkConfig.podIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.network-config.pod-ipv4-range-utilization" => Some(("nodePool.networkConfig.podIpv4RangeUtilization", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "node-pool.network-config.pod-range" => Some(("nodePool.networkConfig.podRange", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.placement-policy.policy-name" => Some(("nodePool.placementPolicy.policyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.placement-policy.tpu-topology" => Some(("nodePool.placementPolicy.tpuTopology", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.placement-policy.type" => Some(("nodePool.placementPolicy.type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.pod-ipv4-cidr-size" => Some(("nodePool.podIpv4CidrSize", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.queued-provisioning.enabled" => Some(("nodePool.queuedProvisioning.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.self-link" => Some(("nodePool.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.status" => Some(("nodePool.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.status-message" => Some(("nodePool.statusMessage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.update-info.blue-green-info.blue-instance-group-urls" => Some(("nodePool.updateInfo.blueGreenInfo.blueInstanceGroupUrls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "node-pool.update-info.blue-green-info.blue-pool-deletion-start-time" => Some(("nodePool.updateInfo.blueGreenInfo.bluePoolDeletionStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.update-info.blue-green-info.green-instance-group-urls" => Some(("nodePool.updateInfo.blueGreenInfo.greenInstanceGroupUrls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "node-pool.update-info.blue-green-info.green-pool-version" => Some(("nodePool.updateInfo.blueGreenInfo.greenPoolVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.update-info.blue-green-info.phase" => Some(("nodePool.updateInfo.blueGreenInfo.phase", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.upgrade-settings.blue-green-settings.node-pool-soak-duration" => Some(("nodePool.upgradeSettings.blueGreenSettings.nodePoolSoakDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.upgrade-settings.blue-green-settings.standard-rollout-policy.batch-node-count" => Some(("nodePool.upgradeSettings.blueGreenSettings.standardRolloutPolicy.batchNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.upgrade-settings.blue-green-settings.standard-rollout-policy.batch-percentage" => Some(("nodePool.upgradeSettings.blueGreenSettings.standardRolloutPolicy.batchPercentage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "node-pool.upgrade-settings.blue-green-settings.standard-rollout-policy.batch-soak-duration" => Some(("nodePool.upgradeSettings.blueGreenSettings.standardRolloutPolicy.batchSoakDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.upgrade-settings.max-surge" => Some(("nodePool.upgradeSettings.maxSurge", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.upgrade-settings.max-unavailable" => Some(("nodePool.upgradeSettings.maxUnavailable", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.upgrade-settings.strategy" => Some(("nodePool.upgradeSettings.strategy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.version" => Some(("nodePool.version", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent" => Some(("parent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["advanced-machine-features", "auto-repair", "auto-upgrade", "auto-upgrade-start-time", "autoprovisioned", "autoscaling", "batch-node-count", "batch-percentage", "batch-soak-duration", "best-effort-provisioning", "blue-green-info", "blue-green-settings", "blue-instance-group-urls", "blue-pool-deletion-start-time", "boot-disk-kms-key", "cgroup-mode", "cluster-id", "confidential-nodes", "config", "consume-reservation-type", "containerd-config", "cpu-cfs-quota", "cpu-cfs-quota-period", "cpu-manager-policy", "create-pod-range", "description", "disable", "disk-size-gb", "disk-type", "enable-confidential-storage", "enable-integrity-monitoring", "enable-nested-virtualization", "enable-private-nodes", "enable-secure-boot", "enabled", "ephemeral-storage-local-ssd-config", "etag", "fast-socket", "gcfs-config", "green-instance-group-urls", "green-pool-version", "gvnic", "hugepage-size1g", "hugepage-size2m", "hugepages", "image-type", "initial-node-count", "insecure-kubelet-readonly-port-enabled", "instance-group-urls", "key", "kubelet-config", "labels", "linux-node-config", "local-nvme-ssd-block-config", "local-ssd-count", "location-policy", "locations", "logging-config", "machine-type", "management", "max-node-count", "max-pods-constraint", "max-pods-per-node", "max-surge", "max-unavailable", "metadata", "min-cpu-platform", "min-node-count", "min-provision-nodes", "mode", "name", "network-config", "network-performance-config", "node-group", "node-pool", "node-pool-soak-duration", "oauth-scopes", "os-version", "parent", "phase", "placement-policy", "pod-cidr-overprovision-config", "pod-ipv4-cidr-block", "pod-ipv4-cidr-size", "pod-ipv4-range-utilization", "pod-pids-limit", "pod-range", "policy-name", "preemptible", "private-registry-access-config", "project-id", "queued-provisioning", "reservation-affinity", "resource-labels", "resource-manager-tags", "sandbox-config", "self-link", "service-account", "shielded-instance-config", "spot", "standard-rollout-policy", "status", "status-message", "strategy", "sysctls", "tags", "threads-per-core", "total-egress-bandwidth-tier", "total-max-node-count", "total-min-node-count", "tpu-topology", "type", "update-info", "upgrade-options", "upgrade-settings", "values", "variant", "variant-config", "version", "windows-node-config", "workload-metadata-config", "zone"]);
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_zones_clusters_node_pools_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_zones_clusters_node_pools_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_zones_clusters_node_pools_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_zones_clusters_node_pools_rollback(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool-id" => Some(("nodePoolId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "respect-pdb" => Some(("respectPdb", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cluster-id", "name", "node-pool-id", "project-id", "respect-pdb", "zone"]);
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_zones_clusters_node_pools_set_management(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "management.auto-repair" => Some(("management.autoRepair", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "management.auto-upgrade" => Some(("management.autoUpgrade", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "management.upgrade-options.auto-upgrade-start-time" => Some(("management.upgradeOptions.autoUpgradeStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "management.upgrade-options.description" => Some(("management.upgradeOptions.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool-id" => Some(("nodePoolId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_zones_clusters_node_pools_set_size(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-count" => Some(("nodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool-id" => Some(("nodePoolId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_zones_clusters_node_pools_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "confidential-nodes.enabled" => Some(("confidentialNodes.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "containerd-config.private-registry-access-config.enabled" => Some(("containerdConfig.privateRegistryAccessConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "disk-size-gb" => Some(("diskSizeGb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "disk-type" => Some(("diskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fast-socket.enabled" => Some(("fastSocket.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "gcfs-config.enabled" => Some(("gcfsConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "gvnic.enabled" => Some(("gvnic.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "image-type" => Some(("imageType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.cpu-cfs-quota" => Some(("kubeletConfig.cpuCfsQuota", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kubelet-config.cpu-cfs-quota-period" => Some(("kubeletConfig.cpuCfsQuotaPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.cpu-manager-policy" => Some(("kubeletConfig.cpuManagerPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.insecure-kubelet-readonly-port-enabled" => Some(("kubeletConfig.insecureKubeletReadonlyPortEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kubelet-config.pod-pids-limit" => Some(("kubeletConfig.podPidsLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels.labels" => Some(("labels.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "linux-node-config.cgroup-mode" => Some(("linuxNodeConfig.cgroupMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "linux-node-config.hugepages.hugepage-size1g" => Some(("linuxNodeConfig.hugepages.hugepageSize1g", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "linux-node-config.hugepages.hugepage-size2m" => Some(("linuxNodeConfig.hugepages.hugepageSize2m", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "linux-node-config.sysctls" => Some(("linuxNodeConfig.sysctls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "locations" => Some(("locations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "logging-config.variant-config.variant" => Some(("loggingConfig.variantConfig.variant", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "machine-type" => Some(("machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-network-config.create-pod-range" => Some(("nodeNetworkConfig.createPodRange", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-network-config.enable-private-nodes" => Some(("nodeNetworkConfig.enablePrivateNodes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-network-config.network-performance-config.total-egress-bandwidth-tier" => Some(("nodeNetworkConfig.networkPerformanceConfig.totalEgressBandwidthTier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-network-config.pod-cidr-overprovision-config.disable" => Some(("nodeNetworkConfig.podCidrOverprovisionConfig.disable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-network-config.pod-ipv4-cidr-block" => Some(("nodeNetworkConfig.podIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-network-config.pod-ipv4-range-utilization" => Some(("nodeNetworkConfig.podIpv4RangeUtilization", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "node-network-config.pod-range" => Some(("nodeNetworkConfig.podRange", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool-id" => Some(("nodePoolId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-version" => Some(("nodeVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "queued-provisioning.enabled" => Some(("queuedProvisioning.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "resource-labels.labels" => Some(("resourceLabels.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "resource-manager-tags.tags" => Some(("resourceManagerTags.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "tags.tags" => Some(("tags.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "upgrade-settings.blue-green-settings.node-pool-soak-duration" => Some(("upgradeSettings.blueGreenSettings.nodePoolSoakDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade-settings.blue-green-settings.standard-rollout-policy.batch-node-count" => Some(("upgradeSettings.blueGreenSettings.standardRolloutPolicy.batchNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "upgrade-settings.blue-green-settings.standard-rollout-policy.batch-percentage" => Some(("upgradeSettings.blueGreenSettings.standardRolloutPolicy.batchPercentage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "upgrade-settings.blue-green-settings.standard-rollout-policy.batch-soak-duration" => Some(("upgradeSettings.blueGreenSettings.standardRolloutPolicy.batchSoakDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "upgrade-settings.max-surge" => Some(("upgradeSettings.maxSurge", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "upgrade-settings.max-unavailable" => Some(("upgradeSettings.maxUnavailable", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "upgrade-settings.strategy" => Some(("upgradeSettings.strategy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "windows-node-config.os-version" => Some(("windowsNodeConfig.osVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "workload-metadata-config.mode" => Some(("workloadMetadataConfig.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["batch-node-count", "batch-percentage", "batch-soak-duration", "blue-green-settings", "cgroup-mode", "cluster-id", "confidential-nodes", "containerd-config", "cpu-cfs-quota", "cpu-cfs-quota-period", "cpu-manager-policy", "create-pod-range", "disable", "disk-size-gb", "disk-type", "enable-private-nodes", "enabled", "etag", "fast-socket", "gcfs-config", "gvnic", "hugepage-size1g", "hugepage-size2m", "hugepages", "image-type", "insecure-kubelet-readonly-port-enabled", "kubelet-config", "labels", "linux-node-config", "locations", "logging-config", "machine-type", "max-surge", "max-unavailable", "mode", "name", "network-performance-config", "node-network-config", "node-pool-id", "node-pool-soak-duration", "node-version", "os-version", "pod-cidr-overprovision-config", "pod-ipv4-cidr-block", "pod-ipv4-range-utilization", "pod-pids-limit", "pod-range", "private-registry-access-config", "project-id", "queued-provisioning", "resource-labels", "resource-manager-tags", "standard-rollout-policy", "strategy", "sysctls", "tags", "total-egress-bandwidth-tier", "upgrade-settings", "variant", "variant-config", "windows-node-config", "workload-metadata-config", "zone"]);
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_zones_clusters_resource_labels(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "label-fingerprint" => Some(("labelFingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "resource-labels" => Some(("resourceLabels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_zones_clusters_set_maintenance_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "maintenance-policy.resource-version" => Some(("maintenancePolicy.resourceVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "maintenance-policy.window.daily-maintenance-window.duration" => Some(("maintenancePolicy.window.dailyMaintenanceWindow.duration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "maintenance-policy.window.daily-maintenance-window.start-time" => Some(("maintenancePolicy.window.dailyMaintenanceWindow.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "maintenance-policy.window.recurring-window.recurrence" => Some(("maintenancePolicy.window.recurringWindow.recurrence", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "maintenance-policy.window.recurring-window.window.end-time" => Some(("maintenancePolicy.window.recurringWindow.window.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "maintenance-policy.window.recurring-window.window.maintenance-exclusion-options.scope" => Some(("maintenancePolicy.window.recurringWindow.window.maintenanceExclusionOptions.scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "maintenance-policy.window.recurring-window.window.start-time" => Some(("maintenancePolicy.window.recurringWindow.window.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cluster-id", "daily-maintenance-window", "duration", "end-time", "maintenance-exclusion-options", "maintenance-policy", "name", "project-id", "recurrence", "recurring-window", "resource-version", "scope", "start-time", "window", "zone"]);
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_zones_clusters_set_master_auth(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "action" => Some(("action", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.client-certificate" => Some(("update.clientCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.client-certificate-config.issue-client-certificate" => Some(("update.clientCertificateConfig.issueClientCertificate", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.client-key" => Some(("update.clientKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.cluster-ca-certificate" => Some(("update.clusterCaCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.password" => Some(("update.password", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.username" => Some(("update.username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_zones_clusters_set_network_policy(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "network-policy.enabled" => Some(("networkPolicy.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "network-policy.provider" => Some(("networkPolicy.provider", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_zones_clusters_start_ip_rotation(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "rotate-credentials" => Some(("rotateCredentials", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_zones_clusters_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.additional-pod-ranges-config.pod-range-names" => Some(("update.additionalPodRangesConfig.podRangeNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.desired-addons-config.cloud-run-config.disabled" => Some(("update.desiredAddonsConfig.cloudRunConfig.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.cloud-run-config.load-balancer-type" => Some(("update.desiredAddonsConfig.cloudRunConfig.loadBalancerType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.config-connector-config.enabled" => Some(("update.desiredAddonsConfig.configConnectorConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.dns-cache-config.enabled" => Some(("update.desiredAddonsConfig.dnsCacheConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.gce-persistent-disk-csi-driver-config.enabled" => Some(("update.desiredAddonsConfig.gcePersistentDiskCsiDriverConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.gcp-filestore-csi-driver-config.enabled" => Some(("update.desiredAddonsConfig.gcpFilestoreCsiDriverConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.gcs-fuse-csi-driver-config.enabled" => Some(("update.desiredAddonsConfig.gcsFuseCsiDriverConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.gke-backup-agent-config.enabled" => Some(("update.desiredAddonsConfig.gkeBackupAgentConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.horizontal-pod-autoscaling.disabled" => Some(("update.desiredAddonsConfig.horizontalPodAutoscaling.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.http-load-balancing.disabled" => Some(("update.desiredAddonsConfig.httpLoadBalancing.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.kubernetes-dashboard.disabled" => Some(("update.desiredAddonsConfig.kubernetesDashboard.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.network-policy-config.disabled" => Some(("update.desiredAddonsConfig.networkPolicyConfig.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.stateful-ha-config.enabled" => Some(("update.desiredAddonsConfig.statefulHaConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-authenticator-groups-config.enabled" => Some(("update.desiredAuthenticatorGroupsConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-authenticator-groups-config.security-group" => Some(("update.desiredAuthenticatorGroupsConfig.securityGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-autopilot-workload-policy-config.allow-net-admin" => Some(("update.desiredAutopilotWorkloadPolicyConfig.allowNetAdmin", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-binary-authorization.enabled" => Some(("update.desiredBinaryAuthorization.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-binary-authorization.evaluation-mode" => Some(("update.desiredBinaryAuthorization.evaluationMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-locations" => Some(("update.desiredClusterAutoscaling.autoprovisioningLocations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.boot-disk-kms-key" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.bootDiskKmsKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.disk-size-gb" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.diskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.disk-type" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.diskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.image-type" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.imageType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.insecure-kubelet-readonly-port-enabled" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.insecureKubeletReadonlyPortEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.management.auto-repair" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.management.autoRepair", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.management.auto-upgrade" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.management.autoUpgrade", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.management.upgrade-options.auto-upgrade-start-time" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.management.upgradeOptions.autoUpgradeStartTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.management.upgrade-options.description" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.management.upgradeOptions.description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.min-cpu-platform" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.minCpuPlatform", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.oauth-scopes" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.oauthScopes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.service-account" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.shielded-instance-config.enable-integrity-monitoring" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.shieldedInstanceConfig.enableIntegrityMonitoring", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.shielded-instance-config.enable-secure-boot" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.shieldedInstanceConfig.enableSecureBoot", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.blue-green-settings.node-pool-soak-duration" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.blueGreenSettings.nodePoolSoakDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.blue-green-settings.standard-rollout-policy.batch-node-count" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.blueGreenSettings.standardRolloutPolicy.batchNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.blue-green-settings.standard-rollout-policy.batch-percentage" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.blueGreenSettings.standardRolloutPolicy.batchPercentage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.blue-green-settings.standard-rollout-policy.batch-soak-duration" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.blueGreenSettings.standardRolloutPolicy.batchSoakDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.max-surge" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.maxSurge", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.max-unavailable" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.maxUnavailable", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.strategy" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.strategy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoscaling-profile" => Some(("update.desiredClusterAutoscaling.autoscalingProfile", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.enable-node-autoprovisioning" => Some(("update.desiredClusterAutoscaling.enableNodeAutoprovisioning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-containerd-config.private-registry-access-config.enabled" => Some(("update.desiredContainerdConfig.privateRegistryAccessConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-cost-management-config.enabled" => Some(("update.desiredCostManagementConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-database-encryption.current-state" => Some(("update.desiredDatabaseEncryption.currentState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-database-encryption.decryption-keys" => Some(("update.desiredDatabaseEncryption.decryptionKeys", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.desired-database-encryption.key-name" => Some(("update.desiredDatabaseEncryption.keyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-database-encryption.state" => Some(("update.desiredDatabaseEncryption.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-datapath-provider" => Some(("update.desiredDatapathProvider", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-default-snat-status.disabled" => Some(("update.desiredDefaultSnatStatus.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-dns-config.additive-vpc-scope-dns-domain" => Some(("update.desiredDnsConfig.additiveVpcScopeDnsDomain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-dns-config.cluster-dns" => Some(("update.desiredDnsConfig.clusterDns", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-dns-config.cluster-dns-domain" => Some(("update.desiredDnsConfig.clusterDnsDomain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-dns-config.cluster-dns-scope" => Some(("update.desiredDnsConfig.clusterDnsScope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-enable-cilium-clusterwide-network-policy" => Some(("update.desiredEnableCiliumClusterwideNetworkPolicy", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-enable-fqdn-network-policy" => Some(("update.desiredEnableFqdnNetworkPolicy", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-enable-multi-networking" => Some(("update.desiredEnableMultiNetworking", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-enable-private-endpoint" => Some(("update.desiredEnablePrivateEndpoint", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-fleet.membership" => Some(("update.desiredFleet.membership", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-fleet.pre-registered" => Some(("update.desiredFleet.preRegistered", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-fleet.project" => Some(("update.desiredFleet.project", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-gateway-api-config.channel" => Some(("update.desiredGatewayApiConfig.channel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-gcfs-config.enabled" => Some(("update.desiredGcfsConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-identity-service-config.enabled" => Some(("update.desiredIdentityServiceConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-image-type" => Some(("update.desiredImageType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-in-transit-encryption-config" => Some(("update.desiredInTransitEncryptionConfig", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-intra-node-visibility-config.enabled" => Some(("update.desiredIntraNodeVisibilityConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-k8s-beta-apis.enabled-apis" => Some(("update.desiredK8sBetaApis.enabledApis", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.desired-l4ilb-subsetting-config.enabled" => Some(("update.desiredL4ilbSubsettingConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-locations" => Some(("update.desiredLocations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.desired-logging-config.component-config.enable-components" => Some(("update.desiredLoggingConfig.componentConfig.enableComponents", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.desired-logging-service" => Some(("update.desiredLoggingService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-master-authorized-networks-config.enabled" => Some(("update.desiredMasterAuthorizedNetworksConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-master-authorized-networks-config.gcp-public-cidrs-access-enabled" => Some(("update.desiredMasterAuthorizedNetworksConfig.gcpPublicCidrsAccessEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-master-version" => Some(("update.desiredMasterVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-mesh-certificates.enable-certificates" => Some(("update.desiredMeshCertificates.enableCertificates", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-monitoring-config.advanced-datapath-observability-config.enable-metrics" => Some(("update.desiredMonitoringConfig.advancedDatapathObservabilityConfig.enableMetrics", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-monitoring-config.advanced-datapath-observability-config.enable-relay" => Some(("update.desiredMonitoringConfig.advancedDatapathObservabilityConfig.enableRelay", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-monitoring-config.advanced-datapath-observability-config.relay-mode" => Some(("update.desiredMonitoringConfig.advancedDatapathObservabilityConfig.relayMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-monitoring-config.component-config.enable-components" => Some(("update.desiredMonitoringConfig.componentConfig.enableComponents", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.desired-monitoring-config.managed-prometheus-config.enabled" => Some(("update.desiredMonitoringConfig.managedPrometheusConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-monitoring-service" => Some(("update.desiredMonitoringService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-network-performance-config.total-egress-bandwidth-tier" => Some(("update.desiredNetworkPerformanceConfig.totalEgressBandwidthTier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.cpu-cfs-quota" => Some(("update.desiredNodeKubeletConfig.cpuCfsQuota", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.cpu-cfs-quota-period" => Some(("update.desiredNodeKubeletConfig.cpuCfsQuotaPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.cpu-manager-policy" => Some(("update.desiredNodeKubeletConfig.cpuManagerPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.insecure-kubelet-readonly-port-enabled" => Some(("update.desiredNodeKubeletConfig.insecureKubeletReadonlyPortEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.pod-pids-limit" => Some(("update.desiredNodeKubeletConfig.podPidsLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.cpu-cfs-quota" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.cpuCfsQuota", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.cpu-cfs-quota-period" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.cpuCfsQuotaPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.cpu-manager-policy" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.cpuManagerPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.insecure-kubelet-readonly-port-enabled" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.insecureKubeletReadonlyPortEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.pod-pids-limit" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.podPidsLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-network-tags.tags" => Some(("update.desiredNodePoolAutoConfigNetworkTags.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.desired-node-pool-auto-config-resource-manager-tags.tags" => Some(("update.desiredNodePoolAutoConfigResourceManagerTags.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "update.desired-node-pool-autoscaling.autoprovisioned" => Some(("update.desiredNodePoolAutoscaling.autoprovisioned", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-autoscaling.enabled" => Some(("update.desiredNodePoolAutoscaling.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-autoscaling.location-policy" => Some(("update.desiredNodePoolAutoscaling.locationPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-autoscaling.max-node-count" => Some(("update.desiredNodePoolAutoscaling.maxNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-autoscaling.min-node-count" => Some(("update.desiredNodePoolAutoscaling.minNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-autoscaling.total-max-node-count" => Some(("update.desiredNodePoolAutoscaling.totalMaxNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-autoscaling.total-min-node-count" => Some(("update.desiredNodePoolAutoscaling.totalMinNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-id" => Some(("update.desiredNodePoolId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-logging-config.variant-config.variant" => Some(("update.desiredNodePoolLoggingConfig.variantConfig.variant", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-version" => Some(("update.desiredNodeVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-notification-config.pubsub.enabled" => Some(("update.desiredNotificationConfig.pubsub.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-notification-config.pubsub.filter.event-type" => Some(("update.desiredNotificationConfig.pubsub.filter.eventType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.desired-notification-config.pubsub.topic" => Some(("update.desiredNotificationConfig.pubsub.topic", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-parent-product-config.labels" => Some(("update.desiredParentProductConfig.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "update.desired-parent-product-config.product-name" => Some(("update.desiredParentProductConfig.productName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-private-cluster-config.enable-private-endpoint" => Some(("update.desiredPrivateClusterConfig.enablePrivateEndpoint", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-private-cluster-config.enable-private-nodes" => Some(("update.desiredPrivateClusterConfig.enablePrivateNodes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-private-cluster-config.master-global-access-config.enabled" => Some(("update.desiredPrivateClusterConfig.masterGlobalAccessConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-private-cluster-config.master-ipv4-cidr-block" => Some(("update.desiredPrivateClusterConfig.masterIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-private-cluster-config.peering-name" => Some(("update.desiredPrivateClusterConfig.peeringName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-private-cluster-config.private-endpoint" => Some(("update.desiredPrivateClusterConfig.privateEndpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-private-cluster-config.private-endpoint-subnetwork" => Some(("update.desiredPrivateClusterConfig.privateEndpointSubnetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-private-cluster-config.public-endpoint" => Some(("update.desiredPrivateClusterConfig.publicEndpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-private-ipv6-google-access" => Some(("update.desiredPrivateIpv6GoogleAccess", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-release-channel.channel" => Some(("update.desiredReleaseChannel.channel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-resource-usage-export-config.bigquery-destination.dataset-id" => Some(("update.desiredResourceUsageExportConfig.bigqueryDestination.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-resource-usage-export-config.consumption-metering-config.enabled" => Some(("update.desiredResourceUsageExportConfig.consumptionMeteringConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-resource-usage-export-config.enable-network-egress-metering" => Some(("update.desiredResourceUsageExportConfig.enableNetworkEgressMetering", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-security-posture-config.mode" => Some(("update.desiredSecurityPostureConfig.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-security-posture-config.vulnerability-mode" => Some(("update.desiredSecurityPostureConfig.vulnerabilityMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-service-external-ips-config.enabled" => Some(("update.desiredServiceExternalIpsConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-shielded-nodes.enabled" => Some(("update.desiredShieldedNodes.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-stack-type" => Some(("update.desiredStackType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-vertical-pod-autoscaling.enabled" => Some(("update.desiredVerticalPodAutoscaling.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-workload-identity-config.workload-pool" => Some(("update.desiredWorkloadIdentityConfig.workloadPool", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.enable-k8s-beta-apis.enabled-apis" => Some(("update.enableK8sBetaApis.enabledApis", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.etag" => Some(("update.etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.removed-additional-pod-ranges-config.pod-range-names" => Some(("update.removedAdditionalPodRangesConfig.podRangeNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["additional-pod-ranges-config", "additive-vpc-scope-dns-domain", "advanced-datapath-observability-config", "allow-net-admin", "auto-repair", "auto-upgrade", "auto-upgrade-start-time", "autoprovisioned", "autoprovisioning-locations", "autoprovisioning-node-pool-defaults", "autoscaling-profile", "batch-node-count", "batch-percentage", "batch-soak-duration", "bigquery-destination", "blue-green-settings", "boot-disk-kms-key", "channel", "cloud-run-config", "cluster-dns", "cluster-dns-domain", "cluster-dns-scope", "cluster-id", "component-config", "config-connector-config", "consumption-metering-config", "cpu-cfs-quota", "cpu-cfs-quota-period", "cpu-manager-policy", "current-state", "dataset-id", "decryption-keys", "description", "desired-addons-config", "desired-authenticator-groups-config", "desired-autopilot-workload-policy-config", "desired-binary-authorization", "desired-cluster-autoscaling", "desired-containerd-config", "desired-cost-management-config", "desired-database-encryption", "desired-datapath-provider", "desired-default-snat-status", "desired-dns-config", "desired-enable-cilium-clusterwide-network-policy", "desired-enable-fqdn-network-policy", "desired-enable-multi-networking", "desired-enable-private-endpoint", "desired-fleet", "desired-gateway-api-config", "desired-gcfs-config", "desired-identity-service-config", "desired-image-type", "desired-in-transit-encryption-config", "desired-intra-node-visibility-config", "desired-k8s-beta-apis", "desired-l4ilb-subsetting-config", "desired-locations", "desired-logging-config", "desired-logging-service", "desired-master-authorized-networks-config", "desired-master-version", "desired-mesh-certificates", "desired-monitoring-config", "desired-monitoring-service", "desired-network-performance-config", "desired-node-kubelet-config", "desired-node-pool-auto-config-kubelet-config", "desired-node-pool-auto-config-network-tags", "desired-node-pool-auto-config-resource-manager-tags", "desired-node-pool-autoscaling", "desired-node-pool-id", "desired-node-pool-logging-config", "desired-node-version", "desired-notification-config", "desired-parent-product-config", "desired-private-cluster-config", "desired-private-ipv6-google-access", "desired-release-channel", "desired-resource-usage-export-config", "desired-security-posture-config", "desired-service-external-ips-config", "desired-shielded-nodes", "desired-stack-type", "desired-vertical-pod-autoscaling", "desired-workload-identity-config", "disabled", "disk-size-gb", "disk-type", "dns-cache-config", "enable-certificates", "enable-components", "enable-integrity-monitoring", "enable-k8s-beta-apis", "enable-metrics", "enable-network-egress-metering", "enable-node-autoprovisioning", "enable-private-endpoint", "enable-private-nodes", "enable-relay", "enable-secure-boot", "enabled", "enabled-apis", "etag", "evaluation-mode", "event-type", "filter", "gce-persistent-disk-csi-driver-config", "gcp-filestore-csi-driver-config", "gcp-public-cidrs-access-enabled", "gcs-fuse-csi-driver-config", "gke-backup-agent-config", "horizontal-pod-autoscaling", "http-load-balancing", "image-type", "insecure-kubelet-readonly-port-enabled", "key-name", "kubernetes-dashboard", "labels", "load-balancer-type", "location-policy", "managed-prometheus-config", "management", "master-global-access-config", "master-ipv4-cidr-block", "max-node-count", "max-surge", "max-unavailable", "membership", "min-cpu-platform", "min-node-count", "mode", "name", "network-policy-config", "node-pool-soak-duration", "oauth-scopes", "peering-name", "pod-pids-limit", "pod-range-names", "pre-registered", "private-endpoint", "private-endpoint-subnetwork", "private-registry-access-config", "product-name", "project", "project-id", "public-endpoint", "pubsub", "relay-mode", "removed-additional-pod-ranges-config", "security-group", "service-account", "shielded-instance-config", "standard-rollout-policy", "state", "stateful-ha-config", "strategy", "tags", "topic", "total-egress-bandwidth-tier", "total-max-node-count", "total-min-node-count", "update", "upgrade-options", "upgrade-settings", "variant", "variant-config", "vulnerability-mode", "workload-pool", "zone"]);
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_zones_get_serverconfig(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_zones_operations_cancel(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "operation-id" => Some(("operationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_zones_operations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit().await,
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

    async fn _projects_zones_operations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                CallType::Standard => call.doit().await,
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

    async fn _doit(&self, dry_run: bool) -> Result<Result<(), DoitError>, Option<InvalidOptionsError>> {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Result<(), DoitError> = Ok(());
        let mut err_opt: Option<InvalidOptionsError> = None;
        match self.opt.subcommand() {
            ("projects", Some(opt)) => {
                match opt.subcommand() {
                    ("aggregated-usable-subnetworks-list", Some(opt)) => {
                        call_result = self._projects_aggregated_usable_subnetworks_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-clusters-check-autopilot-compatibility", Some(opt)) => {
                        call_result = self._projects_locations_clusters_check_autopilot_compatibility(opt, dry_run, &mut err).await;
                    },
                    ("locations-clusters-complete-ip-rotation", Some(opt)) => {
                        call_result = self._projects_locations_clusters_complete_ip_rotation(opt, dry_run, &mut err).await;
                    },
                    ("locations-clusters-create", Some(opt)) => {
                        call_result = self._projects_locations_clusters_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-clusters-delete", Some(opt)) => {
                        call_result = self._projects_locations_clusters_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-clusters-get", Some(opt)) => {
                        call_result = self._projects_locations_clusters_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-clusters-get-jwks", Some(opt)) => {
                        call_result = self._projects_locations_clusters_get_jwks(opt, dry_run, &mut err).await;
                    },
                    ("locations-clusters-list", Some(opt)) => {
                        call_result = self._projects_locations_clusters_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-clusters-node-pools-complete-upgrade", Some(opt)) => {
                        call_result = self._projects_locations_clusters_node_pools_complete_upgrade(opt, dry_run, &mut err).await;
                    },
                    ("locations-clusters-node-pools-create", Some(opt)) => {
                        call_result = self._projects_locations_clusters_node_pools_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-clusters-node-pools-delete", Some(opt)) => {
                        call_result = self._projects_locations_clusters_node_pools_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-clusters-node-pools-get", Some(opt)) => {
                        call_result = self._projects_locations_clusters_node_pools_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-clusters-node-pools-list", Some(opt)) => {
                        call_result = self._projects_locations_clusters_node_pools_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-clusters-node-pools-rollback", Some(opt)) => {
                        call_result = self._projects_locations_clusters_node_pools_rollback(opt, dry_run, &mut err).await;
                    },
                    ("locations-clusters-node-pools-set-autoscaling", Some(opt)) => {
                        call_result = self._projects_locations_clusters_node_pools_set_autoscaling(opt, dry_run, &mut err).await;
                    },
                    ("locations-clusters-node-pools-set-management", Some(opt)) => {
                        call_result = self._projects_locations_clusters_node_pools_set_management(opt, dry_run, &mut err).await;
                    },
                    ("locations-clusters-node-pools-set-size", Some(opt)) => {
                        call_result = self._projects_locations_clusters_node_pools_set_size(opt, dry_run, &mut err).await;
                    },
                    ("locations-clusters-node-pools-update", Some(opt)) => {
                        call_result = self._projects_locations_clusters_node_pools_update(opt, dry_run, &mut err).await;
                    },
                    ("locations-clusters-set-addons", Some(opt)) => {
                        call_result = self._projects_locations_clusters_set_addons(opt, dry_run, &mut err).await;
                    },
                    ("locations-clusters-set-legacy-abac", Some(opt)) => {
                        call_result = self._projects_locations_clusters_set_legacy_abac(opt, dry_run, &mut err).await;
                    },
                    ("locations-clusters-set-locations", Some(opt)) => {
                        call_result = self._projects_locations_clusters_set_locations(opt, dry_run, &mut err).await;
                    },
                    ("locations-clusters-set-logging", Some(opt)) => {
                        call_result = self._projects_locations_clusters_set_logging(opt, dry_run, &mut err).await;
                    },
                    ("locations-clusters-set-maintenance-policy", Some(opt)) => {
                        call_result = self._projects_locations_clusters_set_maintenance_policy(opt, dry_run, &mut err).await;
                    },
                    ("locations-clusters-set-master-auth", Some(opt)) => {
                        call_result = self._projects_locations_clusters_set_master_auth(opt, dry_run, &mut err).await;
                    },
                    ("locations-clusters-set-monitoring", Some(opt)) => {
                        call_result = self._projects_locations_clusters_set_monitoring(opt, dry_run, &mut err).await;
                    },
                    ("locations-clusters-set-network-policy", Some(opt)) => {
                        call_result = self._projects_locations_clusters_set_network_policy(opt, dry_run, &mut err).await;
                    },
                    ("locations-clusters-set-resource-labels", Some(opt)) => {
                        call_result = self._projects_locations_clusters_set_resource_labels(opt, dry_run, &mut err).await;
                    },
                    ("locations-clusters-start-ip-rotation", Some(opt)) => {
                        call_result = self._projects_locations_clusters_start_ip_rotation(opt, dry_run, &mut err).await;
                    },
                    ("locations-clusters-update", Some(opt)) => {
                        call_result = self._projects_locations_clusters_update(opt, dry_run, &mut err).await;
                    },
                    ("locations-clusters-update-master", Some(opt)) => {
                        call_result = self._projects_locations_clusters_update_master(opt, dry_run, &mut err).await;
                    },
                    ("locations-clusters-well-known-get-openid-configuration", Some(opt)) => {
                        call_result = self._projects_locations_clusters_well_known_get_openid_configuration(opt, dry_run, &mut err).await;
                    },
                    ("locations-get-server-config", Some(opt)) => {
                        call_result = self._projects_locations_get_server_config(opt, dry_run, &mut err).await;
                    },
                    ("locations-operations-cancel", Some(opt)) => {
                        call_result = self._projects_locations_operations_cancel(opt, dry_run, &mut err).await;
                    },
                    ("locations-operations-get", Some(opt)) => {
                        call_result = self._projects_locations_operations_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-operations-list", Some(opt)) => {
                        call_result = self._projects_locations_operations_list(opt, dry_run, &mut err).await;
                    },
                    ("zones-clusters-addons", Some(opt)) => {
                        call_result = self._projects_zones_clusters_addons(opt, dry_run, &mut err).await;
                    },
                    ("zones-clusters-complete-ip-rotation", Some(opt)) => {
                        call_result = self._projects_zones_clusters_complete_ip_rotation(opt, dry_run, &mut err).await;
                    },
                    ("zones-clusters-create", Some(opt)) => {
                        call_result = self._projects_zones_clusters_create(opt, dry_run, &mut err).await;
                    },
                    ("zones-clusters-delete", Some(opt)) => {
                        call_result = self._projects_zones_clusters_delete(opt, dry_run, &mut err).await;
                    },
                    ("zones-clusters-get", Some(opt)) => {
                        call_result = self._projects_zones_clusters_get(opt, dry_run, &mut err).await;
                    },
                    ("zones-clusters-legacy-abac", Some(opt)) => {
                        call_result = self._projects_zones_clusters_legacy_abac(opt, dry_run, &mut err).await;
                    },
                    ("zones-clusters-list", Some(opt)) => {
                        call_result = self._projects_zones_clusters_list(opt, dry_run, &mut err).await;
                    },
                    ("zones-clusters-locations", Some(opt)) => {
                        call_result = self._projects_zones_clusters_locations(opt, dry_run, &mut err).await;
                    },
                    ("zones-clusters-logging", Some(opt)) => {
                        call_result = self._projects_zones_clusters_logging(opt, dry_run, &mut err).await;
                    },
                    ("zones-clusters-master", Some(opt)) => {
                        call_result = self._projects_zones_clusters_master(opt, dry_run, &mut err).await;
                    },
                    ("zones-clusters-monitoring", Some(opt)) => {
                        call_result = self._projects_zones_clusters_monitoring(opt, dry_run, &mut err).await;
                    },
                    ("zones-clusters-node-pools-autoscaling", Some(opt)) => {
                        call_result = self._projects_zones_clusters_node_pools_autoscaling(opt, dry_run, &mut err).await;
                    },
                    ("zones-clusters-node-pools-create", Some(opt)) => {
                        call_result = self._projects_zones_clusters_node_pools_create(opt, dry_run, &mut err).await;
                    },
                    ("zones-clusters-node-pools-delete", Some(opt)) => {
                        call_result = self._projects_zones_clusters_node_pools_delete(opt, dry_run, &mut err).await;
                    },
                    ("zones-clusters-node-pools-get", Some(opt)) => {
                        call_result = self._projects_zones_clusters_node_pools_get(opt, dry_run, &mut err).await;
                    },
                    ("zones-clusters-node-pools-list", Some(opt)) => {
                        call_result = self._projects_zones_clusters_node_pools_list(opt, dry_run, &mut err).await;
                    },
                    ("zones-clusters-node-pools-rollback", Some(opt)) => {
                        call_result = self._projects_zones_clusters_node_pools_rollback(opt, dry_run, &mut err).await;
                    },
                    ("zones-clusters-node-pools-set-management", Some(opt)) => {
                        call_result = self._projects_zones_clusters_node_pools_set_management(opt, dry_run, &mut err).await;
                    },
                    ("zones-clusters-node-pools-set-size", Some(opt)) => {
                        call_result = self._projects_zones_clusters_node_pools_set_size(opt, dry_run, &mut err).await;
                    },
                    ("zones-clusters-node-pools-update", Some(opt)) => {
                        call_result = self._projects_zones_clusters_node_pools_update(opt, dry_run, &mut err).await;
                    },
                    ("zones-clusters-resource-labels", Some(opt)) => {
                        call_result = self._projects_zones_clusters_resource_labels(opt, dry_run, &mut err).await;
                    },
                    ("zones-clusters-set-maintenance-policy", Some(opt)) => {
                        call_result = self._projects_zones_clusters_set_maintenance_policy(opt, dry_run, &mut err).await;
                    },
                    ("zones-clusters-set-master-auth", Some(opt)) => {
                        call_result = self._projects_zones_clusters_set_master_auth(opt, dry_run, &mut err).await;
                    },
                    ("zones-clusters-set-network-policy", Some(opt)) => {
                        call_result = self._projects_zones_clusters_set_network_policy(opt, dry_run, &mut err).await;
                    },
                    ("zones-clusters-start-ip-rotation", Some(opt)) => {
                        call_result = self._projects_zones_clusters_start_ip_rotation(opt, dry_run, &mut err).await;
                    },
                    ("zones-clusters-update", Some(opt)) => {
                        call_result = self._projects_zones_clusters_update(opt, dry_run, &mut err).await;
                    },
                    ("zones-get-serverconfig", Some(opt)) => {
                        call_result = self._projects_zones_get_serverconfig(opt, dry_run, &mut err).await;
                    },
                    ("zones-operations-cancel", Some(opt)) => {
                        call_result = self._projects_zones_operations_cancel(opt, dry_run, &mut err).await;
                    },
                    ("zones-operations-get", Some(opt)) => {
                        call_result = self._projects_zones_operations_get(opt, dry_run, &mut err).await;
                    },
                    ("zones-operations-list", Some(opt)) => {
                        call_result = self._projects_zones_operations_list(opt, dry_run, &mut err).await;
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
    async fn new(opt: ArgMatches<'n>, connector: S) -> Result<Engine<'n, S>, InvalidOptionsError> {
        let (config_dir, secret) = {
            let config_dir = match client::assure_config_dir_exists(opt.value_of("folder").unwrap_or("~/.google-service-cli")) {
                Err(e) => return Err(InvalidOptionsError::single(e, 3)),
                Ok(p) => p,
            };

            match client::application_secret_from_directory(&config_dir, "container1-secret.json",
                                                         "{\"installed\":{\"auth_uri\":\"https://accounts.google.com/o/oauth2/auth\",\"client_secret\":\"hCsslbCUyfehWMmbkG8vTYxG\",\"token_uri\":\"https://accounts.google.com/o/oauth2/token\",\"client_email\":\"\",\"redirect_uris\":[\"urn:ietf:wg:oauth:2.0:oob\",\"oob\"],\"client_x509_cert_url\":\"\",\"client_id\":\"620010449518-9ngf7o4dhs0dka470npqvor6dc5lqb9b.apps.googleusercontent.com\",\"auth_provider_x509_cert_url\":\"https://www.googleapis.com/oauth2/v1/certs\"}}") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let client = hyper::Client::builder().build(connector);

        let auth = oauth2::InstalledFlowAuthenticator::with_client(
            secret,
            oauth2::InstalledFlowReturnMethod::HTTPRedirect,
            client.clone(),
        ).persist_tokens_to_disk(format!("{}/container1", config_dir)).build().await.unwrap();

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

        match engine._doit(true).await {
            Err(Some(err)) => Err(err),
            Err(None)      => Ok(engine),
            Ok(_)          => unreachable!(),
        }
    }

    async fn doit(&self) -> Result<(), DoitError> {
        match self._doit(false).await {
            Ok(res) => res,
            Err(_) => unreachable!(),
        }
    }
}

#[tokio::main]
async fn main() {
    let mut exit_status = 0i32;
    let arg_data = [
        ("projects", "methods: 'aggregated-usable-subnetworks-list', 'locations-clusters-check-autopilot-compatibility', 'locations-clusters-complete-ip-rotation', 'locations-clusters-create', 'locations-clusters-delete', 'locations-clusters-get', 'locations-clusters-get-jwks', 'locations-clusters-list', 'locations-clusters-node-pools-complete-upgrade', 'locations-clusters-node-pools-create', 'locations-clusters-node-pools-delete', 'locations-clusters-node-pools-get', 'locations-clusters-node-pools-list', 'locations-clusters-node-pools-rollback', 'locations-clusters-node-pools-set-autoscaling', 'locations-clusters-node-pools-set-management', 'locations-clusters-node-pools-set-size', 'locations-clusters-node-pools-update', 'locations-clusters-set-addons', 'locations-clusters-set-legacy-abac', 'locations-clusters-set-locations', 'locations-clusters-set-logging', 'locations-clusters-set-maintenance-policy', 'locations-clusters-set-master-auth', 'locations-clusters-set-monitoring', 'locations-clusters-set-network-policy', 'locations-clusters-set-resource-labels', 'locations-clusters-start-ip-rotation', 'locations-clusters-update', 'locations-clusters-update-master', 'locations-clusters-well-known-get-openid-configuration', 'locations-get-server-config', 'locations-operations-cancel', 'locations-operations-get', 'locations-operations-list', 'zones-clusters-addons', 'zones-clusters-complete-ip-rotation', 'zones-clusters-create', 'zones-clusters-delete', 'zones-clusters-get', 'zones-clusters-legacy-abac', 'zones-clusters-list', 'zones-clusters-locations', 'zones-clusters-logging', 'zones-clusters-master', 'zones-clusters-monitoring', 'zones-clusters-node-pools-autoscaling', 'zones-clusters-node-pools-create', 'zones-clusters-node-pools-delete', 'zones-clusters-node-pools-get', 'zones-clusters-node-pools-list', 'zones-clusters-node-pools-rollback', 'zones-clusters-node-pools-set-management', 'zones-clusters-node-pools-set-size', 'zones-clusters-node-pools-update', 'zones-clusters-resource-labels', 'zones-clusters-set-maintenance-policy', 'zones-clusters-set-master-auth', 'zones-clusters-set-network-policy', 'zones-clusters-start-ip-rotation', 'zones-clusters-update', 'zones-get-serverconfig', 'zones-operations-cancel', 'zones-operations-get' and 'zones-operations-list'", vec![
            ("aggregated-usable-subnetworks-list",
                    Some(r##"Lists subnetworks that are usable for creating clusters in a project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_aggregated-usable-subnetworks-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The parent project where subnetworks are usable. Specified in the format `projects/*`."##),
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
            ("locations-clusters-check-autopilot-compatibility",
                    Some(r##"Checks the cluster compatibility with Autopilot mode, and returns a list of compatibility issues."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-check-autopilot-compatibility",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name (project, location, cluster) of the cluster to retrieve. Specified in the format `projects/*/locations/*/clusters/*`."##),
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
            ("locations-clusters-complete-ip-rotation",
                    Some(r##"Completes master IP rotation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-complete-ip-rotation",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name (project, location, cluster name) of the cluster to complete IP rotation. Specified in the format `projects/*/locations/*/clusters/*`."##),
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
                    Some(r##"Creates a cluster, consisting of the specified number and type of Google Compute Engine instances. By default, the cluster is created in the project's [default network](https://cloud.google.com/compute/docs/networks-and-firewalls#networks). One firewall is added for the cluster. After cluster creation, the Kubelet creates routes for each node to allow the containers on that node to communicate with all other instances in the cluster. Finally, an entry is added to the project's global metadata indicating which CIDR range the cluster is using."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The parent (project and location) where the cluster will be created. Specified in the format `projects/*/locations/*`."##),
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
                    Some(r##"Deletes the cluster, including the Kubernetes endpoint and all worker nodes. Firewalls and routes that were configured during cluster creation are also deleted. Other Google Compute Engine resources that might be in use by the cluster, such as load balancer resources, are not deleted if they weren't present when the cluster was initially created."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name (project, location, cluster) of the cluster to delete. Specified in the format `projects/*/locations/*/clusters/*`."##),
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
                     Some(r##"The name (project, location, cluster) of the cluster to retrieve. Specified in the format `projects/*/locations/*/clusters/*`."##),
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
                    Some(r##"Gets the public component of the cluster signing keys in JSON Web Key format."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-get-jwks",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The cluster (project, location, cluster name) to get keys for. Specified in the format `projects/*/locations/*/clusters/*`."##),
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
                    Some(r##"Lists all clusters owned by a project in either the specified zone or all zones."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The parent (project and location) where the clusters will be listed. Specified in the format `projects/*/locations/*`. Location "-" matches all zones and all regions."##),
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
            ("locations-clusters-node-pools-complete-upgrade",
                    Some(r##"CompleteNodePoolUpgrade will signal an on-going node pool upgrade to complete."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-node-pools-complete-upgrade",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name (project, location, cluster, node pool id) of the node pool to complete upgrade. Specified in the format `projects/*/locations/*/clusters/*/nodePools/*`."##),
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
            ("locations-clusters-node-pools-create",
                    Some(r##"Creates a node pool for a cluster."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-node-pools-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The parent (project, location, cluster name) where the node pool will be created. Specified in the format `projects/*/locations/*/clusters/*`."##),
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
                     Some(r##"The name (project, location, cluster, node pool id) of the node pool to delete. Specified in the format `projects/*/locations/*/clusters/*/nodePools/*`."##),
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
                    Some(r##"Retrieves the requested node pool."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-node-pools-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name (project, location, cluster, node pool id) of the node pool to get. Specified in the format `projects/*/locations/*/clusters/*/nodePools/*`."##),
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
                     Some(r##"The parent (project, location, cluster name) where the node pools will be listed. Specified in the format `projects/*/locations/*/clusters/*`."##),
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
                    Some(r##"Rolls back a previously Aborted or Failed NodePool upgrade. This makes no changes if the last upgrade successfully completed."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-node-pools-rollback",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name (project, location, cluster, node pool id) of the node poll to rollback upgrade. Specified in the format `projects/*/locations/*/clusters/*/nodePools/*`."##),
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
                    Some(r##"Sets the autoscaling settings for the specified node pool."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-node-pools-set-autoscaling",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name (project, location, cluster, node pool) of the node pool to set autoscaler settings. Specified in the format `projects/*/locations/*/clusters/*/nodePools/*`."##),
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
                     Some(r##"The name (project, location, cluster, node pool id) of the node pool to set management properties. Specified in the format `projects/*/locations/*/clusters/*/nodePools/*`."##),
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
                    Some(r##"Sets the size for a specific node pool. The new size will be used for all replicas, including future replicas created by modifying NodePool.locations."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-node-pools-set-size",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name (project, location, cluster, node pool id) of the node pool to set size. Specified in the format `projects/*/locations/*/clusters/*/nodePools/*`."##),
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
                    Some(r##"Updates the version and/or image type for the specified node pool."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-node-pools-update",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name (project, location, cluster, node pool) of the node pool to update. Specified in the format `projects/*/locations/*/clusters/*/nodePools/*`."##),
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
                     Some(r##"The name (project, location, cluster) of the cluster to set addons. Specified in the format `projects/*/locations/*/clusters/*`."##),
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
                     Some(r##"The name (project, location, cluster name) of the cluster to set legacy abac. Specified in the format `projects/*/locations/*/clusters/*`."##),
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
                    Some(r##"Sets the locations for a specific cluster. Deprecated. Use [projects.locations.clusters.update](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters/update) instead."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-set-locations",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name (project, location, cluster) of the cluster to set locations. Specified in the format `projects/*/locations/*/clusters/*`."##),
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
                     Some(r##"The name (project, location, cluster) of the cluster to set logging. Specified in the format `projects/*/locations/*/clusters/*`."##),
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
                     Some(r##"The name (project, location, cluster name) of the cluster to set maintenance policy. Specified in the format `projects/*/locations/*/clusters/*`."##),
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
                    Some(r##"Sets master auth materials. Currently supports changing the admin password or a specific cluster, either via password generation or explicitly setting the password."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-set-master-auth",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name (project, location, cluster) of the cluster to set auth. Specified in the format `projects/*/locations/*/clusters/*`."##),
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
                     Some(r##"The name (project, location, cluster) of the cluster to set monitoring. Specified in the format `projects/*/locations/*/clusters/*`."##),
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
                    Some(r##"Enables or disables Network Policy for a cluster."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-set-network-policy",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name (project, location, cluster name) of the cluster to set networking policy. Specified in the format `projects/*/locations/*/clusters/*`."##),
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
                     Some(r##"The name (project, location, cluster name) of the cluster to set labels. Specified in the format `projects/*/locations/*/clusters/*`."##),
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
                    Some(r##"Starts master IP rotation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-start-ip-rotation",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name (project, location, cluster name) of the cluster to start IP rotation. Specified in the format `projects/*/locations/*/clusters/*`."##),
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
                     Some(r##"The name (project, location, cluster) of the cluster to update. Specified in the format `projects/*/locations/*/clusters/*`."##),
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
                     Some(r##"The name (project, location, cluster) of the cluster to update. Specified in the format `projects/*/locations/*/clusters/*`."##),
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
                    Some(r##"Gets the OIDC discovery document for the cluster. See the [OpenID Connect Discovery 1.0 specification](https://openid.net/specs/openid-connect-discovery-1_0.html) for details."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-well-known-get-openid-configuration",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The cluster (project, location, cluster name) to get the discovery document for. Specified in the format `projects/*/locations/*/clusters/*`."##),
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
                    Some(r##"Returns configuration info about the Google Kubernetes Engine service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-get-server-config",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name (project and location) of the server config to get, specified in the format `projects/*/locations/*`."##),
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
                     Some(r##"The name (project, location, operation id) of the operation to cancel. Specified in the format `projects/*/locations/*/operations/*`."##),
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
                     Some(r##"The name (project, location, operation id) of the operation to get. Specified in the format `projects/*/locations/*/operations/*`."##),
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
                     Some(r##"The parent (project and location) where the operations will be listed. Specified in the format `projects/*/locations/*`. Location "-" matches all zones and all regions."##),
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
                     Some(r##"Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field."##),
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
                     Some(r##"Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster. This field has been deprecated and replaced by the name field."##),
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
                    Some(r##"Creates a cluster, consisting of the specified number and type of Google Compute Engine instances. By default, the cluster is created in the project's [default network](https://cloud.google.com/compute/docs/networks-and-firewalls#networks). One firewall is added for the cluster. After cluster creation, the Kubelet creates routes for each node to allow the containers on that node to communicate with all other instances in the cluster. Finally, an entry is added to the project's global metadata indicating which CIDR range the cluster is using."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-clusters-create",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the parent field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the parent field."##),
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
                    Some(r##"Deletes the cluster, including the Kubernetes endpoint and all worker nodes. Firewalls and routes that were configured during cluster creation are also deleted. Other Google Compute Engine resources that might be in use by the cluster, such as load balancer resources, are not deleted if they weren't present when the cluster was initially created."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-clusters-delete",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster to delete. This field has been deprecated and replaced by the name field."##),
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
                     Some(r##"Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster to retrieve. This field has been deprecated and replaced by the name field."##),
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
                     Some(r##"Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster to update. This field has been deprecated and replaced by the name field."##),
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
                    Some(r##"Lists all clusters owned by a project in either the specified zone or all zones."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-clusters-list",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the parent field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides, or "-" for all zones. This field has been deprecated and replaced by the parent field."##),
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
                    Some(r##"Sets the locations for a specific cluster. Deprecated. Use [projects.locations.clusters.update](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters/update) instead."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-clusters-locations",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field."##),
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
                     Some(r##"Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field."##),
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
                     Some(r##"Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field."##),
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
                     Some(r##"Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field."##),
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
                    Some(r##"Sets the autoscaling settings for the specified node pool."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-clusters-node-pools-autoscaling",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"node-pool-id"##),
                     None,
                     Some(r##"Deprecated. The name of the node pool to upgrade. This field has been deprecated and replaced by the name field."##),
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
                     Some(r##"Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the parent field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the parent field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster. This field has been deprecated and replaced by the parent field."##),
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
                     Some(r##"Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster. This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"node-pool-id"##),
                     None,
                     Some(r##"Deprecated. The name of the node pool to delete. This field has been deprecated and replaced by the name field."##),
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
                    Some(r##"Retrieves the requested node pool."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-clusters-node-pools-get",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster. This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"node-pool-id"##),
                     None,
                     Some(r##"Deprecated. The name of the node pool. This field has been deprecated and replaced by the name field."##),
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
                     Some(r##"Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the parent field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the parent field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster. This field has been deprecated and replaced by the parent field."##),
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
                    Some(r##"Rolls back a previously Aborted or Failed NodePool upgrade. This makes no changes if the last upgrade successfully completed."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-clusters-node-pools-rollback",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster to rollback. This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"node-pool-id"##),
                     None,
                     Some(r##"Deprecated. The name of the node pool to rollback. This field has been deprecated and replaced by the name field."##),
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
                     Some(r##"Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster to update. This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"node-pool-id"##),
                     None,
                     Some(r##"Deprecated. The name of the node pool to update. This field has been deprecated and replaced by the name field."##),
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
                    Some(r##"Sets the size for a specific node pool. The new size will be used for all replicas, including future replicas created by modifying NodePool.locations."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-clusters-node-pools-set-size",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster to update. This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"node-pool-id"##),
                     None,
                     Some(r##"Deprecated. The name of the node pool to update. This field has been deprecated and replaced by the name field."##),
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
                    Some(r##"Updates the version and/or image type for the specified node pool."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-clusters-node-pools-update",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"node-pool-id"##),
                     None,
                     Some(r##"Deprecated. The name of the node pool to upgrade. This field has been deprecated and replaced by the name field."##),
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
                     Some(r##"Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster. This field has been deprecated and replaced by the name field."##),
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
                     Some(r##"Required. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects)."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Required. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Required. The name of the cluster to update."##),
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
                    Some(r##"Sets master auth materials. Currently supports changing the admin password or a specific cluster, either via password generation or explicitly setting the password."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-clusters-set-master-auth",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field."##),
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
                    Some(r##"Enables or disables Network Policy for a cluster."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-clusters-set-network-policy",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster. This field has been deprecated and replaced by the name field."##),
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
                    Some(r##"Starts master IP rotation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-clusters-start-ip-rotation",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster. This field has been deprecated and replaced by the name field."##),
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
                     Some(r##"Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"cluster-id"##),
                     None,
                     Some(r##"Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field."##),
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
                    Some(r##"Returns configuration info about the Google Kubernetes Engine service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-get-serverconfig",
                  vec![
                    (Some(r##"project-id"##),
                     None,
                     Some(r##"Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) to return operations for. This field has been deprecated and replaced by the name field."##),
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
                     Some(r##"Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the operation resides. This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"operation-id"##),
                     None,
                     Some(r##"Deprecated. The server-assigned `name` of the operation. This field has been deprecated and replaced by the name field."##),
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
                     Some(r##"Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"operation-id"##),
                     None,
                     Some(r##"Deprecated. The server-assigned `name` of the operation. This field has been deprecated and replaced by the name field."##),
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
                     Some(r##"Deprecated. The Google Developers Console [project ID or project number](https://cloud.google.com/resource-manager/docs/creating-managing-projects). This field has been deprecated and replaced by the parent field."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"zone"##),
                     None,
                     Some(r##"Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) to return operations for, or `-` for all zones. This field has been deprecated and replaced by the parent field."##),
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
           .version("5.0.5+20240608")
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
                   .help("Debug print all errors")
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

    let debug = matches.is_present("adebug");
    let connector = hyper_rustls::HttpsConnectorBuilder::new().with_native_roots()
        .unwrap()
        .https_or_http()
        .enable_http1()
        .build();

    match Engine::new(matches, connector).await {
        Err(err) => {
            exit_status = err.exit_code;
            writeln!(io::stderr(), "{}", err).ok();
        },
        Ok(engine) => {
            if let Err(doit_err) = engine.doit().await {
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
