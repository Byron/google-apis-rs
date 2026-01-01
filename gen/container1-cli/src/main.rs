// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::io::Write;

use clap::{App, Arg, SubCommand};

use google_container1::{api, yup_oauth2, Error};

use google_apis_common as apis_common;
use google_clis_common as common;

use std::str::FromStr;

use clap::ArgMatches;
use http_body_util::BodyExt;

use common::{
    arg_from_str, calltype_from_str, input_file_from_opts, input_mime_from_opts, parse_kv_arg,
    remove_json_null_values, writer_from_opts, CLIError, CallType, ComplexType, FieldCursor,
    FieldError, InvalidOptionsError, JsonType, JsonTypeInfo, UploadProtocol,
};

enum DoitError {
    IoError(String, std::io::Error),
    ApiError(Error),
}

struct Engine<'n, C> {
    opt: ArgMatches<'n>,
    hub: api::Container<C>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}

impl<'n, C> Engine<'n, C>
where
    C: apis_common::Connector,
{
    async fn _projects_aggregated_usable_subnetworks_list(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .projects()
            .aggregated_usable_subnetworks_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                }
                "page-size" => {
                    call = call.page_size(
                        value
                            .map(|v| arg_from_str(v, err, "page-size", "int32"))
                            .unwrap_or(-0),
                    );
                }
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                }
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v.extend(["filter", "page-size", "page-token"].iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_clusters_check_autopilot_compatibility(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .projects()
            .locations_clusters_check_autopilot_compatibility(opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_clusters_complete_ip_rotation(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "cluster-id" => Some((
                    "clusterId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "name" => Some((
                    "name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "project-id" => Some((
                    "projectId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "zone" => Some((
                    "zone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec!["cluster-id", "name", "project-id", "zone"],
                    );
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::CompleteIPRotationRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self
            .hub
            .projects()
            .locations_clusters_complete_ip_rotation(request, opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_clusters_create(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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
                    "cluster.addons-config.high-scale-checkpointing-config.enabled" => Some(("cluster.addonsConfig.highScaleCheckpointingConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.horizontal-pod-autoscaling.disabled" => Some(("cluster.addonsConfig.horizontalPodAutoscaling.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.http-load-balancing.disabled" => Some(("cluster.addonsConfig.httpLoadBalancing.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.kubernetes-dashboard.disabled" => Some(("cluster.addonsConfig.kubernetesDashboard.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.lustre-csi-driver-config.enable-legacy-lustre-port" => Some(("cluster.addonsConfig.lustreCsiDriverConfig.enableLegacyLustrePort", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.lustre-csi-driver-config.enabled" => Some(("cluster.addonsConfig.lustreCsiDriverConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.network-policy-config.disabled" => Some(("cluster.addonsConfig.networkPolicyConfig.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.parallelstore-csi-driver-config.enabled" => Some(("cluster.addonsConfig.parallelstoreCsiDriverConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.ray-operator-config.enabled" => Some(("cluster.addonsConfig.rayOperatorConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.ray-operator-config.ray-cluster-logging-config.enabled" => Some(("cluster.addonsConfig.rayOperatorConfig.rayClusterLoggingConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.ray-operator-config.ray-cluster-monitoring-config.enabled" => Some(("cluster.addonsConfig.rayOperatorConfig.rayClusterMonitoringConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.stateful-ha-config.enabled" => Some(("cluster.addonsConfig.statefulHaConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.alpha-cluster-feature-gates" => Some(("cluster.alphaClusterFeatureGates", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.anonymous-authentication-config.mode" => Some(("cluster.anonymousAuthenticationConfig.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.authenticator-groups-config.enabled" => Some(("cluster.authenticatorGroupsConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.authenticator-groups-config.security-group" => Some(("cluster.authenticatorGroupsConfig.securityGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.autopilot.enabled" => Some(("cluster.autopilot.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.autopilot.privileged-admission-config.allowlist-paths" => Some(("cluster.autopilot.privilegedAdmissionConfig.allowlistPaths", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.autopilot.workload-policy-config.allow-net-admin" => Some(("cluster.autopilot.workloadPolicyConfig.allowNetAdmin", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.autopilot.workload-policy-config.autopilot-compatibility-auditing-enabled" => Some(("cluster.autopilot.workloadPolicyConfig.autopilotCompatibilityAuditingEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
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
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.blue-green-settings.autoscaled-rollout-policy.wait-for-drain-duration" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.blueGreenSettings.autoscaledRolloutPolicy.waitForDrainDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.blue-green-settings.node-pool-soak-duration" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.blueGreenSettings.nodePoolSoakDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.blue-green-settings.standard-rollout-policy.batch-node-count" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.blueGreenSettings.standardRolloutPolicy.batchNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.blue-green-settings.standard-rollout-policy.batch-percentage" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.blueGreenSettings.standardRolloutPolicy.batchPercentage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.blue-green-settings.standard-rollout-policy.batch-soak-duration" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.blueGreenSettings.standardRolloutPolicy.batchSoakDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.max-surge" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.maxSurge", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.max-unavailable" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.maxUnavailable", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.strategy" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.strategy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoscaling-profile" => Some(("cluster.autoscaling.autoscalingProfile", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.default-compute-class-config.enabled" => Some(("cluster.autoscaling.defaultComputeClassConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.enable-node-autoprovisioning" => Some(("cluster.autoscaling.enableNodeAutoprovisioning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.binary-authorization.enabled" => Some(("cluster.binaryAuthorization.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.binary-authorization.evaluation-mode" => Some(("cluster.binaryAuthorization.evaluationMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.cluster-ipv4-cidr" => Some(("cluster.clusterIpv4Cidr", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.compliance-posture-config.mode" => Some(("cluster.compliancePostureConfig.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.confidential-nodes.confidential-instance-type" => Some(("cluster.confidentialNodes.confidentialInstanceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.confidential-nodes.enabled" => Some(("cluster.confidentialNodes.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.control-plane-endpoints-config.dns-endpoint-config.allow-external-traffic" => Some(("cluster.controlPlaneEndpointsConfig.dnsEndpointConfig.allowExternalTraffic", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.control-plane-endpoints-config.dns-endpoint-config.enable-k8s-certs-via-dns" => Some(("cluster.controlPlaneEndpointsConfig.dnsEndpointConfig.enableK8sCertsViaDns", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.control-plane-endpoints-config.dns-endpoint-config.enable-k8s-tokens-via-dns" => Some(("cluster.controlPlaneEndpointsConfig.dnsEndpointConfig.enableK8sTokensViaDns", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.control-plane-endpoints-config.dns-endpoint-config.endpoint" => Some(("cluster.controlPlaneEndpointsConfig.dnsEndpointConfig.endpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.control-plane-endpoints-config.ip-endpoints-config.authorized-networks-config.enabled" => Some(("cluster.controlPlaneEndpointsConfig.ipEndpointsConfig.authorizedNetworksConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.control-plane-endpoints-config.ip-endpoints-config.authorized-networks-config.gcp-public-cidrs-access-enabled" => Some(("cluster.controlPlaneEndpointsConfig.ipEndpointsConfig.authorizedNetworksConfig.gcpPublicCidrsAccessEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.control-plane-endpoints-config.ip-endpoints-config.authorized-networks-config.private-endpoint-enforcement-enabled" => Some(("cluster.controlPlaneEndpointsConfig.ipEndpointsConfig.authorizedNetworksConfig.privateEndpointEnforcementEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.control-plane-endpoints-config.ip-endpoints-config.enable-public-endpoint" => Some(("cluster.controlPlaneEndpointsConfig.ipEndpointsConfig.enablePublicEndpoint", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.control-plane-endpoints-config.ip-endpoints-config.enabled" => Some(("cluster.controlPlaneEndpointsConfig.ipEndpointsConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.control-plane-endpoints-config.ip-endpoints-config.global-access" => Some(("cluster.controlPlaneEndpointsConfig.ipEndpointsConfig.globalAccess", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.control-plane-endpoints-config.ip-endpoints-config.private-endpoint" => Some(("cluster.controlPlaneEndpointsConfig.ipEndpointsConfig.privateEndpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.control-plane-endpoints-config.ip-endpoints-config.private-endpoint-subnetwork" => Some(("cluster.controlPlaneEndpointsConfig.ipEndpointsConfig.privateEndpointSubnetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.control-plane-endpoints-config.ip-endpoints-config.public-endpoint" => Some(("cluster.controlPlaneEndpointsConfig.ipEndpointsConfig.publicEndpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                    "cluster.enterprise-config.desired-tier" => Some(("cluster.enterpriseConfig.desiredTier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.etag" => Some(("cluster.etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.expire-time" => Some(("cluster.expireTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.fleet.membership" => Some(("cluster.fleet.membership", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.fleet.membership-type" => Some(("cluster.fleet.membershipType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.fleet.pre-registered" => Some(("cluster.fleet.preRegistered", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.fleet.project" => Some(("cluster.fleet.project", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.gke-auto-upgrade-config.patch-mode" => Some(("cluster.gkeAutoUpgradeConfig.patchMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.id" => Some(("cluster.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.identity-service-config.enabled" => Some(("cluster.identityServiceConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.initial-cluster-version" => Some(("cluster.initialClusterVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.initial-node-count" => Some(("cluster.initialNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.instance-group-urls" => Some(("cluster.instanceGroupUrls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.ip-allocation-policy.additional-pod-ranges-config.pod-range-names" => Some(("cluster.ipAllocationPolicy.additionalPodRangesConfig.podRangeNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.ip-allocation-policy.auto-ipam-config.enabled" => Some(("cluster.ipAllocationPolicy.autoIpamConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.cluster-ipv4-cidr" => Some(("cluster.ipAllocationPolicy.clusterIpv4Cidr", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.cluster-ipv4-cidr-block" => Some(("cluster.ipAllocationPolicy.clusterIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.cluster-secondary-range-name" => Some(("cluster.ipAllocationPolicy.clusterSecondaryRangeName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.create-subnetwork" => Some(("cluster.ipAllocationPolicy.createSubnetwork", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.default-pod-ipv4-range-utilization" => Some(("cluster.ipAllocationPolicy.defaultPodIpv4RangeUtilization", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.ipv6-access-type" => Some(("cluster.ipAllocationPolicy.ipv6AccessType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.network-tier-config.network-tier" => Some(("cluster.ipAllocationPolicy.networkTierConfig.networkTier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                    "cluster.maintenance-policy.window.recurring-window.window.maintenance-exclusion-options.end-time-behavior" => Some(("cluster.maintenancePolicy.window.recurringWindow.window.maintenanceExclusionOptions.endTimeBehavior", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.maintenance-policy.window.recurring-window.window.maintenance-exclusion-options.scope" => Some(("cluster.maintenancePolicy.window.recurringWindow.window.maintenanceExclusionOptions.scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.maintenance-policy.window.recurring-window.window.start-time" => Some(("cluster.maintenancePolicy.window.recurringWindow.window.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.managed-opentelemetry-config.scope" => Some(("cluster.managedOpentelemetryConfig.scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.master-auth.client-certificate" => Some(("cluster.masterAuth.clientCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.master-auth.client-certificate-config.issue-client-certificate" => Some(("cluster.masterAuth.clientCertificateConfig.issueClientCertificate", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.master-auth.client-key" => Some(("cluster.masterAuth.clientKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.master-auth.cluster-ca-certificate" => Some(("cluster.masterAuth.clusterCaCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.master-auth.password" => Some(("cluster.masterAuth.password", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.master-auth.username" => Some(("cluster.masterAuth.username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.master-authorized-networks-config.enabled" => Some(("cluster.masterAuthorizedNetworksConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.master-authorized-networks-config.gcp-public-cidrs-access-enabled" => Some(("cluster.masterAuthorizedNetworksConfig.gcpPublicCidrsAccessEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.master-authorized-networks-config.private-endpoint-enforcement-enabled" => Some(("cluster.masterAuthorizedNetworksConfig.privateEndpointEnforcementEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.mesh-certificates.enable-certificates" => Some(("cluster.meshCertificates.enableCertificates", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.monitoring-config.advanced-datapath-observability-config.enable-metrics" => Some(("cluster.monitoringConfig.advancedDatapathObservabilityConfig.enableMetrics", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.monitoring-config.advanced-datapath-observability-config.enable-relay" => Some(("cluster.monitoringConfig.advancedDatapathObservabilityConfig.enableRelay", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.monitoring-config.advanced-datapath-observability-config.relay-mode" => Some(("cluster.monitoringConfig.advancedDatapathObservabilityConfig.relayMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.monitoring-config.component-config.enable-components" => Some(("cluster.monitoringConfig.componentConfig.enableComponents", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.monitoring-config.managed-prometheus-config.auto-monitoring-config.scope" => Some(("cluster.monitoringConfig.managedPrometheusConfig.autoMonitoringConfig.scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.monitoring-config.managed-prometheus-config.enabled" => Some(("cluster.monitoringConfig.managedPrometheusConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.monitoring-service" => Some(("cluster.monitoringService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.name" => Some(("cluster.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.network" => Some(("cluster.network", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.network-config.datapath-provider" => Some(("cluster.networkConfig.datapathProvider", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.network-config.default-enable-private-nodes" => Some(("cluster.networkConfig.defaultEnablePrivateNodes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.network-config.default-snat-status.disabled" => Some(("cluster.networkConfig.defaultSnatStatus.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.network-config.disable-l4-lb-firewall-reconciliation" => Some(("cluster.networkConfig.disableL4LbFirewallReconciliation", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
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
                    "cluster.node-config.advanced-machine-features.performance-monitoring-unit" => Some(("cluster.nodeConfig.advancedMachineFeatures.performanceMonitoringUnit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.advanced-machine-features.threads-per-core" => Some(("cluster.nodeConfig.advancedMachineFeatures.threadsPerCore", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.boot-disk.disk-type" => Some(("cluster.nodeConfig.bootDisk.diskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.boot-disk.provisioned-iops" => Some(("cluster.nodeConfig.bootDisk.provisionedIops", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.boot-disk.provisioned-throughput" => Some(("cluster.nodeConfig.bootDisk.provisionedThroughput", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.boot-disk.size-gb" => Some(("cluster.nodeConfig.bootDisk.sizeGb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.boot-disk-kms-key" => Some(("cluster.nodeConfig.bootDiskKmsKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.confidential-nodes.confidential-instance-type" => Some(("cluster.nodeConfig.confidentialNodes.confidentialInstanceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.confidential-nodes.enabled" => Some(("cluster.nodeConfig.confidentialNodes.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.containerd-config.private-registry-access-config.enabled" => Some(("cluster.nodeConfig.containerdConfig.privateRegistryAccessConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.containerd-config.writable-cgroups.enabled" => Some(("cluster.nodeConfig.containerdConfig.writableCgroups.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.disk-size-gb" => Some(("cluster.nodeConfig.diskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-config.disk-type" => Some(("cluster.nodeConfig.diskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.effective-cgroup-mode" => Some(("cluster.nodeConfig.effectiveCgroupMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.enable-confidential-storage" => Some(("cluster.nodeConfig.enableConfidentialStorage", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.ephemeral-storage-local-ssd-config.data-cache-count" => Some(("cluster.nodeConfig.ephemeralStorageLocalSsdConfig.dataCacheCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-config.ephemeral-storage-local-ssd-config.local-ssd-count" => Some(("cluster.nodeConfig.ephemeralStorageLocalSsdConfig.localSsdCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-config.fast-socket.enabled" => Some(("cluster.nodeConfig.fastSocket.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.flex-start" => Some(("cluster.nodeConfig.flexStart", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.gcfs-config.enabled" => Some(("cluster.nodeConfig.gcfsConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.gvnic.enabled" => Some(("cluster.nodeConfig.gvnic.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.image-type" => Some(("cluster.nodeConfig.imageType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.allowed-unsafe-sysctls" => Some(("cluster.nodeConfig.kubeletConfig.allowedUnsafeSysctls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.node-config.kubelet-config.container-log-max-files" => Some(("cluster.nodeConfig.kubeletConfig.containerLogMaxFiles", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.container-log-max-size" => Some(("cluster.nodeConfig.kubeletConfig.containerLogMaxSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.cpu-cfs-quota" => Some(("cluster.nodeConfig.kubeletConfig.cpuCfsQuota", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.cpu-cfs-quota-period" => Some(("cluster.nodeConfig.kubeletConfig.cpuCfsQuotaPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.cpu-manager-policy" => Some(("cluster.nodeConfig.kubeletConfig.cpuManagerPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.eviction-max-pod-grace-period-seconds" => Some(("cluster.nodeConfig.kubeletConfig.evictionMaxPodGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.eviction-minimum-reclaim.imagefs-available" => Some(("cluster.nodeConfig.kubeletConfig.evictionMinimumReclaim.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.eviction-minimum-reclaim.imagefs-inodes-free" => Some(("cluster.nodeConfig.kubeletConfig.evictionMinimumReclaim.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.eviction-minimum-reclaim.memory-available" => Some(("cluster.nodeConfig.kubeletConfig.evictionMinimumReclaim.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.eviction-minimum-reclaim.nodefs-available" => Some(("cluster.nodeConfig.kubeletConfig.evictionMinimumReclaim.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.eviction-minimum-reclaim.nodefs-inodes-free" => Some(("cluster.nodeConfig.kubeletConfig.evictionMinimumReclaim.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.eviction-minimum-reclaim.pid-available" => Some(("cluster.nodeConfig.kubeletConfig.evictionMinimumReclaim.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.eviction-soft.imagefs-available" => Some(("cluster.nodeConfig.kubeletConfig.evictionSoft.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.eviction-soft.imagefs-inodes-free" => Some(("cluster.nodeConfig.kubeletConfig.evictionSoft.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.eviction-soft.memory-available" => Some(("cluster.nodeConfig.kubeletConfig.evictionSoft.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.eviction-soft.nodefs-available" => Some(("cluster.nodeConfig.kubeletConfig.evictionSoft.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.eviction-soft.nodefs-inodes-free" => Some(("cluster.nodeConfig.kubeletConfig.evictionSoft.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.eviction-soft.pid-available" => Some(("cluster.nodeConfig.kubeletConfig.evictionSoft.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.eviction-soft-grace-period.imagefs-available" => Some(("cluster.nodeConfig.kubeletConfig.evictionSoftGracePeriod.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.eviction-soft-grace-period.imagefs-inodes-free" => Some(("cluster.nodeConfig.kubeletConfig.evictionSoftGracePeriod.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.eviction-soft-grace-period.memory-available" => Some(("cluster.nodeConfig.kubeletConfig.evictionSoftGracePeriod.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.eviction-soft-grace-period.nodefs-available" => Some(("cluster.nodeConfig.kubeletConfig.evictionSoftGracePeriod.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.eviction-soft-grace-period.nodefs-inodes-free" => Some(("cluster.nodeConfig.kubeletConfig.evictionSoftGracePeriod.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.eviction-soft-grace-period.pid-available" => Some(("cluster.nodeConfig.kubeletConfig.evictionSoftGracePeriod.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.image-gc-high-threshold-percent" => Some(("cluster.nodeConfig.kubeletConfig.imageGcHighThresholdPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.image-gc-low-threshold-percent" => Some(("cluster.nodeConfig.kubeletConfig.imageGcLowThresholdPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.image-maximum-gc-age" => Some(("cluster.nodeConfig.kubeletConfig.imageMaximumGcAge", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.image-minimum-gc-age" => Some(("cluster.nodeConfig.kubeletConfig.imageMinimumGcAge", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.insecure-kubelet-readonly-port-enabled" => Some(("cluster.nodeConfig.kubeletConfig.insecureKubeletReadonlyPortEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.max-parallel-image-pulls" => Some(("cluster.nodeConfig.kubeletConfig.maxParallelImagePulls", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.memory-manager.policy" => Some(("cluster.nodeConfig.kubeletConfig.memoryManager.policy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.pod-pids-limit" => Some(("cluster.nodeConfig.kubeletConfig.podPidsLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.single-process-oom-kill" => Some(("cluster.nodeConfig.kubeletConfig.singleProcessOomKill", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.topology-manager.policy" => Some(("cluster.nodeConfig.kubeletConfig.topologyManager.policy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.topology-manager.scope" => Some(("cluster.nodeConfig.kubeletConfig.topologyManager.scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.labels" => Some(("cluster.nodeConfig.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "cluster.node-config.linux-node-config.cgroup-mode" => Some(("cluster.nodeConfig.linuxNodeConfig.cgroupMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.linux-node-config.hugepages.hugepage-size1g" => Some(("cluster.nodeConfig.linuxNodeConfig.hugepages.hugepageSize1g", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-config.linux-node-config.hugepages.hugepage-size2m" => Some(("cluster.nodeConfig.linuxNodeConfig.hugepages.hugepageSize2m", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-config.linux-node-config.node-kernel-module-loading.policy" => Some(("cluster.nodeConfig.linuxNodeConfig.nodeKernelModuleLoading.policy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.linux-node-config.sysctls" => Some(("cluster.nodeConfig.linuxNodeConfig.sysctls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "cluster.node-config.linux-node-config.transparent-hugepage-defrag" => Some(("cluster.nodeConfig.linuxNodeConfig.transparentHugepageDefrag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.linux-node-config.transparent-hugepage-enabled" => Some(("cluster.nodeConfig.linuxNodeConfig.transparentHugepageEnabled", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.local-nvme-ssd-block-config.local-ssd-count" => Some(("cluster.nodeConfig.localNvmeSsdBlockConfig.localSsdCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-config.local-ssd-count" => Some(("cluster.nodeConfig.localSsdCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-config.local-ssd-encryption-mode" => Some(("cluster.nodeConfig.localSsdEncryptionMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.logging-config.variant-config.variant" => Some(("cluster.nodeConfig.loggingConfig.variantConfig.variant", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.machine-type" => Some(("cluster.nodeConfig.machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.max-run-duration" => Some(("cluster.nodeConfig.maxRunDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                    "cluster.node-config.sole-tenant-config.min-node-cpus" => Some(("cluster.nodeConfig.soleTenantConfig.minNodeCpus", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-config.spot" => Some(("cluster.nodeConfig.spot", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.storage-pools" => Some(("cluster.nodeConfig.storagePools", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.node-config.tags" => Some(("cluster.nodeConfig.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.node-config.windows-node-config.os-version" => Some(("cluster.nodeConfig.windowsNodeConfig.osVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.workload-metadata-config.mode" => Some(("cluster.nodeConfig.workloadMetadataConfig.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-ipv4-cidr-size" => Some(("cluster.nodeIpv4CidrSize", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.linux-node-config.cgroup-mode" => Some(("cluster.nodePoolAutoConfig.linuxNodeConfig.cgroupMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.linux-node-config.hugepages.hugepage-size1g" => Some(("cluster.nodePoolAutoConfig.linuxNodeConfig.hugepages.hugepageSize1g", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.linux-node-config.hugepages.hugepage-size2m" => Some(("cluster.nodePoolAutoConfig.linuxNodeConfig.hugepages.hugepageSize2m", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.linux-node-config.node-kernel-module-loading.policy" => Some(("cluster.nodePoolAutoConfig.linuxNodeConfig.nodeKernelModuleLoading.policy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.linux-node-config.sysctls" => Some(("cluster.nodePoolAutoConfig.linuxNodeConfig.sysctls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "cluster.node-pool-auto-config.linux-node-config.transparent-hugepage-defrag" => Some(("cluster.nodePoolAutoConfig.linuxNodeConfig.transparentHugepageDefrag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.linux-node-config.transparent-hugepage-enabled" => Some(("cluster.nodePoolAutoConfig.linuxNodeConfig.transparentHugepageEnabled", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.network-tags.tags" => Some(("cluster.nodePoolAutoConfig.networkTags.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.node-pool-auto-config.node-kubelet-config.allowed-unsafe-sysctls" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.allowedUnsafeSysctls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.node-pool-auto-config.node-kubelet-config.container-log-max-files" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.containerLogMaxFiles", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.container-log-max-size" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.containerLogMaxSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.cpu-cfs-quota" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.cpuCfsQuota", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.cpu-cfs-quota-period" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.cpuCfsQuotaPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.cpu-manager-policy" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.cpuManagerPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.eviction-max-pod-grace-period-seconds" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.evictionMaxPodGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.eviction-minimum-reclaim.imagefs-available" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.evictionMinimumReclaim.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.eviction-minimum-reclaim.imagefs-inodes-free" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.evictionMinimumReclaim.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.eviction-minimum-reclaim.memory-available" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.evictionMinimumReclaim.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.eviction-minimum-reclaim.nodefs-available" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.evictionMinimumReclaim.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.eviction-minimum-reclaim.nodefs-inodes-free" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.evictionMinimumReclaim.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.eviction-minimum-reclaim.pid-available" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.evictionMinimumReclaim.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.eviction-soft.imagefs-available" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.evictionSoft.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.eviction-soft.imagefs-inodes-free" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.evictionSoft.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.eviction-soft.memory-available" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.evictionSoft.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.eviction-soft.nodefs-available" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.evictionSoft.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.eviction-soft.nodefs-inodes-free" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.evictionSoft.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.eviction-soft.pid-available" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.evictionSoft.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.eviction-soft-grace-period.imagefs-available" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.evictionSoftGracePeriod.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.eviction-soft-grace-period.imagefs-inodes-free" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.evictionSoftGracePeriod.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.eviction-soft-grace-period.memory-available" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.evictionSoftGracePeriod.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.eviction-soft-grace-period.nodefs-available" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.evictionSoftGracePeriod.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.eviction-soft-grace-period.nodefs-inodes-free" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.evictionSoftGracePeriod.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.eviction-soft-grace-period.pid-available" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.evictionSoftGracePeriod.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.image-gc-high-threshold-percent" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.imageGcHighThresholdPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.image-gc-low-threshold-percent" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.imageGcLowThresholdPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.image-maximum-gc-age" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.imageMaximumGcAge", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.image-minimum-gc-age" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.imageMinimumGcAge", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.insecure-kubelet-readonly-port-enabled" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.insecureKubeletReadonlyPortEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.max-parallel-image-pulls" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.maxParallelImagePulls", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.memory-manager.policy" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.memoryManager.policy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.pod-pids-limit" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.podPidsLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.single-process-oom-kill" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.singleProcessOomKill", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.topology-manager.policy" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.topologyManager.policy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.topology-manager.scope" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.topologyManager.scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.resource-manager-tags.tags" => Some(("cluster.nodePoolAutoConfig.resourceManagerTags.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "cluster.node-pool-defaults.node-config-defaults.containerd-config.private-registry-access-config.enabled" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.containerdConfig.privateRegistryAccessConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.containerd-config.writable-cgroups.enabled" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.containerdConfig.writableCgroups.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.gcfs-config.enabled" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.gcfsConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.logging-config.variant-config.variant" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.loggingConfig.variantConfig.variant", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.allowed-unsafe-sysctls" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.allowedUnsafeSysctls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.container-log-max-files" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.containerLogMaxFiles", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.container-log-max-size" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.containerLogMaxSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.cpu-cfs-quota" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.cpuCfsQuota", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.cpu-cfs-quota-period" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.cpuCfsQuotaPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.cpu-manager-policy" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.cpuManagerPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.eviction-max-pod-grace-period-seconds" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.evictionMaxPodGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.eviction-minimum-reclaim.imagefs-available" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.evictionMinimumReclaim.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.eviction-minimum-reclaim.imagefs-inodes-free" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.evictionMinimumReclaim.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.eviction-minimum-reclaim.memory-available" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.evictionMinimumReclaim.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.eviction-minimum-reclaim.nodefs-available" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.evictionMinimumReclaim.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.eviction-minimum-reclaim.nodefs-inodes-free" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.evictionMinimumReclaim.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.eviction-minimum-reclaim.pid-available" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.evictionMinimumReclaim.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.eviction-soft.imagefs-available" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.evictionSoft.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.eviction-soft.imagefs-inodes-free" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.evictionSoft.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.eviction-soft.memory-available" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.evictionSoft.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.eviction-soft.nodefs-available" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.evictionSoft.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.eviction-soft.nodefs-inodes-free" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.evictionSoft.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.eviction-soft.pid-available" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.evictionSoft.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.eviction-soft-grace-period.imagefs-available" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.evictionSoftGracePeriod.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.eviction-soft-grace-period.imagefs-inodes-free" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.evictionSoftGracePeriod.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.eviction-soft-grace-period.memory-available" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.evictionSoftGracePeriod.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.eviction-soft-grace-period.nodefs-available" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.evictionSoftGracePeriod.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.eviction-soft-grace-period.nodefs-inodes-free" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.evictionSoftGracePeriod.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.eviction-soft-grace-period.pid-available" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.evictionSoftGracePeriod.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.image-gc-high-threshold-percent" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.imageGcHighThresholdPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.image-gc-low-threshold-percent" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.imageGcLowThresholdPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.image-maximum-gc-age" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.imageMaximumGcAge", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.image-minimum-gc-age" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.imageMinimumGcAge", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.insecure-kubelet-readonly-port-enabled" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.insecureKubeletReadonlyPortEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.max-parallel-image-pulls" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.maxParallelImagePulls", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.memory-manager.policy" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.memoryManager.policy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.pod-pids-limit" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.podPidsLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.single-process-oom-kill" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.singleProcessOomKill", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.topology-manager.policy" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.topologyManager.policy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.topology-manager.scope" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.topologyManager.scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.notification-config.pubsub.enabled" => Some(("cluster.notificationConfig.pubsub.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.notification-config.pubsub.filter.event-type" => Some(("cluster.notificationConfig.pubsub.filter.eventType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.notification-config.pubsub.topic" => Some(("cluster.notificationConfig.pubsub.topic", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.parent-product-config.labels" => Some(("cluster.parentProductConfig.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "cluster.parent-product-config.product-name" => Some(("cluster.parentProductConfig.productName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.pod-autoscaling.hpa-profile" => Some(("cluster.podAutoscaling.hpaProfile", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.enable-private-endpoint" => Some(("cluster.privateClusterConfig.enablePrivateEndpoint", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.enable-private-nodes" => Some(("cluster.privateClusterConfig.enablePrivateNodes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.master-global-access-config.enabled" => Some(("cluster.privateClusterConfig.masterGlobalAccessConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.master-ipv4-cidr-block" => Some(("cluster.privateClusterConfig.masterIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.peering-name" => Some(("cluster.privateClusterConfig.peeringName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.private-endpoint" => Some(("cluster.privateClusterConfig.privateEndpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.private-endpoint-subnetwork" => Some(("cluster.privateClusterConfig.privateEndpointSubnetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.public-endpoint" => Some(("cluster.privateClusterConfig.publicEndpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.rbac-binding-config.enable-insecure-binding-system-authenticated" => Some(("cluster.rbacBindingConfig.enableInsecureBindingSystemAuthenticated", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.rbac-binding-config.enable-insecure-binding-system-unauthenticated" => Some(("cluster.rbacBindingConfig.enableInsecureBindingSystemUnauthenticated", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.release-channel.channel" => Some(("cluster.releaseChannel.channel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.resource-labels" => Some(("cluster.resourceLabels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "cluster.resource-usage-export-config.bigquery-destination.dataset-id" => Some(("cluster.resourceUsageExportConfig.bigqueryDestination.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.resource-usage-export-config.consumption-metering-config.enabled" => Some(("cluster.resourceUsageExportConfig.consumptionMeteringConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.resource-usage-export-config.enable-network-egress-metering" => Some(("cluster.resourceUsageExportConfig.enableNetworkEgressMetering", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.satisfies-pzi" => Some(("cluster.satisfiesPzi", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.satisfies-pzs" => Some(("cluster.satisfiesPzs", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.secret-manager-config.enabled" => Some(("cluster.secretManagerConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.secret-manager-config.rotation-config.enabled" => Some(("cluster.secretManagerConfig.rotationConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.secret-manager-config.rotation-config.rotation-interval" => Some(("cluster.secretManagerConfig.rotationConfig.rotationInterval", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.security-posture-config.mode" => Some(("cluster.securityPostureConfig.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.security-posture-config.vulnerability-mode" => Some(("cluster.securityPostureConfig.vulnerabilityMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.self-link" => Some(("cluster.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.services-ipv4-cidr" => Some(("cluster.servicesIpv4Cidr", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.shielded-nodes.enabled" => Some(("cluster.shieldedNodes.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.status" => Some(("cluster.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.status-message" => Some(("cluster.statusMessage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.subnetwork" => Some(("cluster.subnetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.tpu-ipv4-cidr-block" => Some(("cluster.tpuIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.user-managed-keys-config.aggregation-ca" => Some(("cluster.userManagedKeysConfig.aggregationCa", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.user-managed-keys-config.cluster-ca" => Some(("cluster.userManagedKeysConfig.clusterCa", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.user-managed-keys-config.control-plane-disk-encryption-key" => Some(("cluster.userManagedKeysConfig.controlPlaneDiskEncryptionKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.user-managed-keys-config.control-plane-disk-encryption-key-versions" => Some(("cluster.userManagedKeysConfig.controlPlaneDiskEncryptionKeyVersions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.user-managed-keys-config.etcd-api-ca" => Some(("cluster.userManagedKeysConfig.etcdApiCa", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.user-managed-keys-config.etcd-peer-ca" => Some(("cluster.userManagedKeysConfig.etcdPeerCa", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.user-managed-keys-config.gkeops-etcd-backup-encryption-key" => Some(("cluster.userManagedKeysConfig.gkeopsEtcdBackupEncryptionKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.user-managed-keys-config.service-account-signing-keys" => Some(("cluster.userManagedKeysConfig.serviceAccountSigningKeys", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.user-managed-keys-config.service-account-verification-keys" => Some(("cluster.userManagedKeysConfig.serviceAccountVerificationKeys", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.vertical-pod-autoscaling.enabled" => Some(("cluster.verticalPodAutoscaling.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.workload-identity-config.workload-pool" => Some(("cluster.workloadIdentityConfig.workloadPool", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.zone" => Some(("cluster.zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent" => Some(("parent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["additional-pod-ranges-config", "additive-vpc-scope-dns-domain", "addons-config", "advanced-datapath-observability-config", "advanced-machine-features", "aggregation-ca", "allow-external-traffic", "allow-net-admin", "allowed-unsafe-sysctls", "allowlist-paths", "alpha-cluster-feature-gates", "anonymous-authentication-config", "authenticator-groups-config", "authorized-networks-config", "auto-ipam-config", "auto-monitoring-config", "auto-repair", "auto-upgrade", "auto-upgrade-start-time", "autopilot", "autopilot-compatibility-auditing-enabled", "autoprovisioning-locations", "autoprovisioning-node-pool-defaults", "autoscaled-rollout-policy", "autoscaling", "autoscaling-profile", "batch-node-count", "batch-percentage", "batch-soak-duration", "bigquery-destination", "binary-authorization", "blue-green-settings", "boot-disk", "boot-disk-kms-key", "cgroup-mode", "channel", "client-certificate", "client-certificate-config", "client-key", "cloud-run-config", "cluster", "cluster-ca", "cluster-ca-certificate", "cluster-dns", "cluster-dns-domain", "cluster-dns-scope", "cluster-ipv4-cidr", "cluster-ipv4-cidr-block", "cluster-secondary-range-name", "cluster-tier", "compliance-posture-config", "component-config", "confidential-instance-type", "confidential-nodes", "config-connector-config", "consume-reservation-type", "consumption-metering-config", "container-log-max-files", "container-log-max-size", "containerd-config", "control-plane-disk-encryption-key", "control-plane-disk-encryption-key-versions", "control-plane-endpoints-config", "cost-management-config", "cpu-cfs-quota", "cpu-cfs-quota-period", "cpu-manager-policy", "create-subnetwork", "create-time", "current-master-version", "current-node-count", "current-node-version", "current-state", "daily-maintenance-window", "data-cache-count", "database-encryption", "datapath-provider", "dataset-id", "decryption-keys", "default-compute-class-config", "default-enable-private-nodes", "default-max-pods-constraint", "default-pod-ipv4-range-utilization", "default-snat-status", "description", "desired-tier", "disable", "disable-l4-lb-firewall-reconciliation", "disabled", "disk-size-gb", "disk-type", "dns-cache-config", "dns-config", "dns-endpoint-config", "duration", "effective-cgroup-mode", "enable-certificates", "enable-cilium-clusterwide-network-policy", "enable-components", "enable-confidential-storage", "enable-fqdn-network-policy", "enable-insecure-binding-system-authenticated", "enable-insecure-binding-system-unauthenticated", "enable-integrity-monitoring", "enable-intra-node-visibility", "enable-k8s-beta-apis", "enable-k8s-certs-via-dns", "enable-k8s-tokens-via-dns", "enable-kubernetes-alpha", "enable-l4ilb-subsetting", "enable-legacy-lustre-port", "enable-metrics", "enable-multi-networking", "enable-nested-virtualization", "enable-network-egress-metering", "enable-node-autoprovisioning", "enable-private-endpoint", "enable-private-nodes", "enable-public-endpoint", "enable-relay", "enable-secure-boot", "enable-tpu", "enabled", "enabled-apis", "end-time", "end-time-behavior", "endpoint", "enterprise-config", "ephemeral-storage-local-ssd-config", "etag", "etcd-api-ca", "etcd-peer-ca", "evaluation-mode", "event-type", "eviction-max-pod-grace-period-seconds", "eviction-minimum-reclaim", "eviction-soft", "eviction-soft-grace-period", "expire-time", "fast-socket", "filter", "fleet", "flex-start", "gateway-api-config", "gce-persistent-disk-csi-driver-config", "gcfs-config", "gcp-filestore-csi-driver-config", "gcp-public-cidrs-access-enabled", "gcs-fuse-csi-driver-config", "gke-auto-upgrade-config", "gke-backup-agent-config", "gkeops-etcd-backup-encryption-key", "global-access", "gvnic", "high-scale-checkpointing-config", "horizontal-pod-autoscaling", "hpa-profile", "http-load-balancing", "hugepage-size1g", "hugepage-size2m", "hugepages", "id", "identity-service-config", "image-gc-high-threshold-percent", "image-gc-low-threshold-percent", "image-maximum-gc-age", "image-minimum-gc-age", "image-type", "imagefs-available", "imagefs-inodes-free", "in-transit-encryption-config", "initial-cluster-version", "initial-node-count", "insecure-kubelet-readonly-port-enabled", "instance-group-urls", "ip-allocation-policy", "ip-endpoints-config", "ipv6-access-type", "issue-client-certificate", "key", "key-name", "kubelet-config", "kubernetes-dashboard", "label-fingerprint", "labels", "legacy-abac", "linux-node-config", "load-balancer-type", "local-nvme-ssd-block-config", "local-ssd-count", "local-ssd-encryption-mode", "location", "locations", "logging-config", "logging-service", "lustre-csi-driver-config", "machine-type", "maintenance-exclusion-options", "maintenance-policy", "managed-opentelemetry-config", "managed-prometheus-config", "management", "master-auth", "master-authorized-networks-config", "master-global-access-config", "master-ipv4-cidr-block", "max-parallel-image-pulls", "max-pods-per-node", "max-run-duration", "max-surge", "max-unavailable", "membership", "membership-type", "memory-available", "memory-manager", "mesh-certificates", "metadata", "min-cpu-platform", "min-node-cpus", "mode", "monitoring-config", "monitoring-service", "name", "network", "network-config", "network-performance-config", "network-policy", "network-policy-config", "network-tags", "network-tier", "network-tier-config", "node-config", "node-config-defaults", "node-group", "node-ipv4-cidr", "node-ipv4-cidr-block", "node-ipv4-cidr-size", "node-kernel-module-loading", "node-kubelet-config", "node-pool-auto-config", "node-pool-defaults", "node-pool-soak-duration", "nodefs-available", "nodefs-inodes-free", "notification-config", "oauth-scopes", "os-version", "parallelstore-csi-driver-config", "parent", "parent-product-config", "password", "patch-mode", "peering-name", "performance-monitoring-unit", "pid-available", "pod-autoscaling", "pod-cidr-overprovision-config", "pod-pids-limit", "pod-range-names", "policy", "pre-registered", "preemptible", "private-cluster-config", "private-endpoint", "private-endpoint-enforcement-enabled", "private-endpoint-subnetwork", "private-ipv6-google-access", "private-registry-access-config", "privileged-admission-config", "product-name", "project", "project-id", "provider", "provisioned-iops", "provisioned-throughput", "public-endpoint", "pubsub", "ray-cluster-logging-config", "ray-cluster-monitoring-config", "ray-operator-config", "rbac-binding-config", "recurrence", "recurring-window", "relay-mode", "release-channel", "reservation-affinity", "resource-labels", "resource-manager-tags", "resource-usage-export-config", "resource-version", "rotation-config", "rotation-interval", "sandbox-config", "satisfies-pzi", "satisfies-pzs", "scope", "secret-manager-config", "security-group", "security-posture-config", "self-link", "service-account", "service-account-signing-keys", "service-account-verification-keys", "service-external-ips-config", "services-ipv4-cidr", "services-ipv4-cidr-block", "services-ipv6-cidr-block", "services-secondary-range-name", "shielded-instance-config", "shielded-nodes", "single-process-oom-kill", "size-gb", "sole-tenant-config", "spot", "stack-type", "standard-rollout-policy", "start-time", "state", "stateful-ha-config", "status", "status-message", "storage-pools", "strategy", "subnet-ipv6-cidr-block", "subnetwork", "subnetwork-name", "sysctls", "tags", "threads-per-core", "topic", "topology-manager", "total-egress-bandwidth-tier", "tpu-ipv4-cidr-block", "transparent-hugepage-defrag", "transparent-hugepage-enabled", "type", "upgrade-options", "upgrade-settings", "use-ip-aliases", "use-routes", "user-managed-keys-config", "username", "values", "variant", "variant-config", "vertical-pod-autoscaling", "vulnerability-mode", "wait-for-drain-duration", "window", "windows-node-config", "workload-identity-config", "workload-metadata-config", "workload-policy-config", "workload-pool", "writable-cgroups", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::CreateClusterRequest = serde_json::value::from_value(object).unwrap();
        let mut call = self
            .hub
            .projects()
            .locations_clusters_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_clusters_delete(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .projects()
            .locations_clusters_delete(opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "zone" => {
                    call = call.zone(value.unwrap_or(""));
                }
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
                }
                "cluster-id" => {
                    call = call.cluster_id(value.unwrap_or(""));
                }
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v.extend(["cluster-id", "project-id", "zone"].iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_clusters_fetch_cluster_upgrade_info(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .projects()
            .locations_clusters_fetch_cluster_upgrade_info(opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "version" => {
                    call = call.version(value.unwrap_or(""));
                }
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v.extend(["version"].iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_clusters_get(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .projects()
            .locations_clusters_get(opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "zone" => {
                    call = call.zone(value.unwrap_or(""));
                }
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
                }
                "cluster-id" => {
                    call = call.cluster_id(value.unwrap_or(""));
                }
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v.extend(["cluster-id", "project-id", "zone"].iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_clusters_get_jwks(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .projects()
            .locations_clusters_get_jwks(opt.value_of("parent").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
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
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_clusters_list(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .projects()
            .locations_clusters_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "zone" => {
                    call = call.zone(value.unwrap_or(""));
                }
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
                }
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v.extend(["project-id", "zone"].iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_clusters_node_pools_complete_upgrade(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec![]);
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::CompleteNodePoolUpgradeRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self
            .hub
            .projects()
            .locations_clusters_node_pools_complete_upgrade(
                request,
                opt.value_of("name").unwrap_or(""),
            );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_clusters_node_pools_create(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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
                    "node-pool.autopilot-config.enabled" => Some(("nodePool.autopilotConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
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
                    "node-pool.config.advanced-machine-features.performance-monitoring-unit" => Some(("nodePool.config.advancedMachineFeatures.performanceMonitoringUnit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.advanced-machine-features.threads-per-core" => Some(("nodePool.config.advancedMachineFeatures.threadsPerCore", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.boot-disk.disk-type" => Some(("nodePool.config.bootDisk.diskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.boot-disk.provisioned-iops" => Some(("nodePool.config.bootDisk.provisionedIops", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.boot-disk.provisioned-throughput" => Some(("nodePool.config.bootDisk.provisionedThroughput", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.boot-disk.size-gb" => Some(("nodePool.config.bootDisk.sizeGb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.boot-disk-kms-key" => Some(("nodePool.config.bootDiskKmsKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.confidential-nodes.confidential-instance-type" => Some(("nodePool.config.confidentialNodes.confidentialInstanceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.confidential-nodes.enabled" => Some(("nodePool.config.confidentialNodes.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.containerd-config.private-registry-access-config.enabled" => Some(("nodePool.config.containerdConfig.privateRegistryAccessConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.containerd-config.writable-cgroups.enabled" => Some(("nodePool.config.containerdConfig.writableCgroups.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.disk-size-gb" => Some(("nodePool.config.diskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.disk-type" => Some(("nodePool.config.diskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.effective-cgroup-mode" => Some(("nodePool.config.effectiveCgroupMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.enable-confidential-storage" => Some(("nodePool.config.enableConfidentialStorage", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.ephemeral-storage-local-ssd-config.data-cache-count" => Some(("nodePool.config.ephemeralStorageLocalSsdConfig.dataCacheCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.ephemeral-storage-local-ssd-config.local-ssd-count" => Some(("nodePool.config.ephemeralStorageLocalSsdConfig.localSsdCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.fast-socket.enabled" => Some(("nodePool.config.fastSocket.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.flex-start" => Some(("nodePool.config.flexStart", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.gcfs-config.enabled" => Some(("nodePool.config.gcfsConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.gvnic.enabled" => Some(("nodePool.config.gvnic.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.image-type" => Some(("nodePool.config.imageType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.allowed-unsafe-sysctls" => Some(("nodePool.config.kubeletConfig.allowedUnsafeSysctls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "node-pool.config.kubelet-config.container-log-max-files" => Some(("nodePool.config.kubeletConfig.containerLogMaxFiles", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.container-log-max-size" => Some(("nodePool.config.kubeletConfig.containerLogMaxSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.cpu-cfs-quota" => Some(("nodePool.config.kubeletConfig.cpuCfsQuota", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.cpu-cfs-quota-period" => Some(("nodePool.config.kubeletConfig.cpuCfsQuotaPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.cpu-manager-policy" => Some(("nodePool.config.kubeletConfig.cpuManagerPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.eviction-max-pod-grace-period-seconds" => Some(("nodePool.config.kubeletConfig.evictionMaxPodGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.eviction-minimum-reclaim.imagefs-available" => Some(("nodePool.config.kubeletConfig.evictionMinimumReclaim.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.eviction-minimum-reclaim.imagefs-inodes-free" => Some(("nodePool.config.kubeletConfig.evictionMinimumReclaim.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.eviction-minimum-reclaim.memory-available" => Some(("nodePool.config.kubeletConfig.evictionMinimumReclaim.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.eviction-minimum-reclaim.nodefs-available" => Some(("nodePool.config.kubeletConfig.evictionMinimumReclaim.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.eviction-minimum-reclaim.nodefs-inodes-free" => Some(("nodePool.config.kubeletConfig.evictionMinimumReclaim.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.eviction-minimum-reclaim.pid-available" => Some(("nodePool.config.kubeletConfig.evictionMinimumReclaim.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.eviction-soft.imagefs-available" => Some(("nodePool.config.kubeletConfig.evictionSoft.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.eviction-soft.imagefs-inodes-free" => Some(("nodePool.config.kubeletConfig.evictionSoft.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.eviction-soft.memory-available" => Some(("nodePool.config.kubeletConfig.evictionSoft.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.eviction-soft.nodefs-available" => Some(("nodePool.config.kubeletConfig.evictionSoft.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.eviction-soft.nodefs-inodes-free" => Some(("nodePool.config.kubeletConfig.evictionSoft.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.eviction-soft.pid-available" => Some(("nodePool.config.kubeletConfig.evictionSoft.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.eviction-soft-grace-period.imagefs-available" => Some(("nodePool.config.kubeletConfig.evictionSoftGracePeriod.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.eviction-soft-grace-period.imagefs-inodes-free" => Some(("nodePool.config.kubeletConfig.evictionSoftGracePeriod.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.eviction-soft-grace-period.memory-available" => Some(("nodePool.config.kubeletConfig.evictionSoftGracePeriod.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.eviction-soft-grace-period.nodefs-available" => Some(("nodePool.config.kubeletConfig.evictionSoftGracePeriod.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.eviction-soft-grace-period.nodefs-inodes-free" => Some(("nodePool.config.kubeletConfig.evictionSoftGracePeriod.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.eviction-soft-grace-period.pid-available" => Some(("nodePool.config.kubeletConfig.evictionSoftGracePeriod.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.image-gc-high-threshold-percent" => Some(("nodePool.config.kubeletConfig.imageGcHighThresholdPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.image-gc-low-threshold-percent" => Some(("nodePool.config.kubeletConfig.imageGcLowThresholdPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.image-maximum-gc-age" => Some(("nodePool.config.kubeletConfig.imageMaximumGcAge", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.image-minimum-gc-age" => Some(("nodePool.config.kubeletConfig.imageMinimumGcAge", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.insecure-kubelet-readonly-port-enabled" => Some(("nodePool.config.kubeletConfig.insecureKubeletReadonlyPortEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.max-parallel-image-pulls" => Some(("nodePool.config.kubeletConfig.maxParallelImagePulls", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.memory-manager.policy" => Some(("nodePool.config.kubeletConfig.memoryManager.policy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.pod-pids-limit" => Some(("nodePool.config.kubeletConfig.podPidsLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.single-process-oom-kill" => Some(("nodePool.config.kubeletConfig.singleProcessOomKill", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.topology-manager.policy" => Some(("nodePool.config.kubeletConfig.topologyManager.policy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.topology-manager.scope" => Some(("nodePool.config.kubeletConfig.topologyManager.scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.labels" => Some(("nodePool.config.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "node-pool.config.linux-node-config.cgroup-mode" => Some(("nodePool.config.linuxNodeConfig.cgroupMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.linux-node-config.hugepages.hugepage-size1g" => Some(("nodePool.config.linuxNodeConfig.hugepages.hugepageSize1g", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.linux-node-config.hugepages.hugepage-size2m" => Some(("nodePool.config.linuxNodeConfig.hugepages.hugepageSize2m", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.linux-node-config.node-kernel-module-loading.policy" => Some(("nodePool.config.linuxNodeConfig.nodeKernelModuleLoading.policy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.linux-node-config.sysctls" => Some(("nodePool.config.linuxNodeConfig.sysctls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "node-pool.config.linux-node-config.transparent-hugepage-defrag" => Some(("nodePool.config.linuxNodeConfig.transparentHugepageDefrag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.linux-node-config.transparent-hugepage-enabled" => Some(("nodePool.config.linuxNodeConfig.transparentHugepageEnabled", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.local-nvme-ssd-block-config.local-ssd-count" => Some(("nodePool.config.localNvmeSsdBlockConfig.localSsdCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.local-ssd-count" => Some(("nodePool.config.localSsdCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.local-ssd-encryption-mode" => Some(("nodePool.config.localSsdEncryptionMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.logging-config.variant-config.variant" => Some(("nodePool.config.loggingConfig.variantConfig.variant", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.machine-type" => Some(("nodePool.config.machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.max-run-duration" => Some(("nodePool.config.maxRunDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                    "node-pool.config.sole-tenant-config.min-node-cpus" => Some(("nodePool.config.soleTenantConfig.minNodeCpus", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.spot" => Some(("nodePool.config.spot", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.storage-pools" => Some(("nodePool.config.storagePools", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
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
                    "node-pool.network-config.network-tier-config.network-tier" => Some(("nodePool.networkConfig.networkTierConfig.networkTier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.network-config.pod-cidr-overprovision-config.disable" => Some(("nodePool.networkConfig.podCidrOverprovisionConfig.disable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.network-config.pod-ipv4-cidr-block" => Some(("nodePool.networkConfig.podIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.network-config.pod-ipv4-range-utilization" => Some(("nodePool.networkConfig.podIpv4RangeUtilization", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "node-pool.network-config.pod-range" => Some(("nodePool.networkConfig.podRange", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.network-config.subnetwork" => Some(("nodePool.networkConfig.subnetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.node-drain-config.respect-pdb-during-node-pool-deletion" => Some(("nodePool.nodeDrainConfig.respectPdbDuringNodePoolDeletion", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
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
                    "node-pool.upgrade-settings.blue-green-settings.autoscaled-rollout-policy.wait-for-drain-duration" => Some(("nodePool.upgradeSettings.blueGreenSettings.autoscaledRolloutPolicy.waitForDrainDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["advanced-machine-features", "allowed-unsafe-sysctls", "auto-repair", "auto-upgrade", "auto-upgrade-start-time", "autopilot-config", "autoprovisioned", "autoscaled-rollout-policy", "autoscaling", "batch-node-count", "batch-percentage", "batch-soak-duration", "best-effort-provisioning", "blue-green-info", "blue-green-settings", "blue-instance-group-urls", "blue-pool-deletion-start-time", "boot-disk", "boot-disk-kms-key", "cgroup-mode", "cluster-id", "confidential-instance-type", "confidential-nodes", "config", "consume-reservation-type", "container-log-max-files", "container-log-max-size", "containerd-config", "cpu-cfs-quota", "cpu-cfs-quota-period", "cpu-manager-policy", "create-pod-range", "data-cache-count", "description", "disable", "disk-size-gb", "disk-type", "effective-cgroup-mode", "enable-confidential-storage", "enable-integrity-monitoring", "enable-nested-virtualization", "enable-private-nodes", "enable-secure-boot", "enabled", "ephemeral-storage-local-ssd-config", "etag", "eviction-max-pod-grace-period-seconds", "eviction-minimum-reclaim", "eviction-soft", "eviction-soft-grace-period", "fast-socket", "flex-start", "gcfs-config", "green-instance-group-urls", "green-pool-version", "gvnic", "hugepage-size1g", "hugepage-size2m", "hugepages", "image-gc-high-threshold-percent", "image-gc-low-threshold-percent", "image-maximum-gc-age", "image-minimum-gc-age", "image-type", "imagefs-available", "imagefs-inodes-free", "initial-node-count", "insecure-kubelet-readonly-port-enabled", "instance-group-urls", "key", "kubelet-config", "labels", "linux-node-config", "local-nvme-ssd-block-config", "local-ssd-count", "local-ssd-encryption-mode", "location-policy", "locations", "logging-config", "machine-type", "management", "max-node-count", "max-parallel-image-pulls", "max-pods-constraint", "max-pods-per-node", "max-run-duration", "max-surge", "max-unavailable", "memory-available", "memory-manager", "metadata", "min-cpu-platform", "min-node-count", "min-node-cpus", "min-provision-nodes", "mode", "name", "network-config", "network-performance-config", "network-tier", "network-tier-config", "node-drain-config", "node-group", "node-kernel-module-loading", "node-pool", "node-pool-soak-duration", "nodefs-available", "nodefs-inodes-free", "oauth-scopes", "os-version", "parent", "performance-monitoring-unit", "phase", "pid-available", "placement-policy", "pod-cidr-overprovision-config", "pod-ipv4-cidr-block", "pod-ipv4-cidr-size", "pod-ipv4-range-utilization", "pod-pids-limit", "pod-range", "policy", "policy-name", "preemptible", "private-registry-access-config", "project-id", "provisioned-iops", "provisioned-throughput", "queued-provisioning", "reservation-affinity", "resource-labels", "resource-manager-tags", "respect-pdb-during-node-pool-deletion", "sandbox-config", "scope", "self-link", "service-account", "shielded-instance-config", "single-process-oom-kill", "size-gb", "sole-tenant-config", "spot", "standard-rollout-policy", "status", "status-message", "storage-pools", "strategy", "subnetwork", "sysctls", "tags", "threads-per-core", "topology-manager", "total-egress-bandwidth-tier", "total-max-node-count", "total-min-node-count", "tpu-topology", "transparent-hugepage-defrag", "transparent-hugepage-enabled", "type", "update-info", "upgrade-options", "upgrade-settings", "values", "variant", "variant-config", "version", "wait-for-drain-duration", "windows-node-config", "workload-metadata-config", "writable-cgroups", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::CreateNodePoolRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self
            .hub
            .projects()
            .locations_clusters_node_pools_create(request, opt.value_of("parent").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_clusters_node_pools_delete(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .projects()
            .locations_clusters_node_pools_delete(opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "zone" => {
                    call = call.zone(value.unwrap_or(""));
                }
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
                }
                "node-pool-id" => {
                    call = call.node_pool_id(value.unwrap_or(""));
                }
                "cluster-id" => {
                    call = call.cluster_id(value.unwrap_or(""));
                }
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v.extend(
                                    ["cluster-id", "node-pool-id", "project-id", "zone"]
                                        .iter()
                                        .map(|v| *v),
                                );
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_clusters_node_pools_fetch_node_pool_upgrade_info(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .projects()
            .locations_clusters_node_pools_fetch_node_pool_upgrade_info(
                opt.value_of("name").unwrap_or(""),
            );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "version" => {
                    call = call.version(value.unwrap_or(""));
                }
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v.extend(["version"].iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_clusters_node_pools_get(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .projects()
            .locations_clusters_node_pools_get(opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "zone" => {
                    call = call.zone(value.unwrap_or(""));
                }
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
                }
                "node-pool-id" => {
                    call = call.node_pool_id(value.unwrap_or(""));
                }
                "cluster-id" => {
                    call = call.cluster_id(value.unwrap_or(""));
                }
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v.extend(
                                    ["cluster-id", "node-pool-id", "project-id", "zone"]
                                        .iter()
                                        .map(|v| *v),
                                );
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_clusters_node_pools_list(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .projects()
            .locations_clusters_node_pools_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "zone" => {
                    call = call.zone(value.unwrap_or(""));
                }
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
                }
                "cluster-id" => {
                    call = call.cluster_id(value.unwrap_or(""));
                }
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v.extend(["cluster-id", "project-id", "zone"].iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_clusters_node_pools_rollback(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "cluster-id" => Some((
                    "clusterId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "name" => Some((
                    "name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "node-pool-id" => Some((
                    "nodePoolId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "project-id" => Some((
                    "projectId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "respect-pdb" => Some((
                    "respectPdb",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "zone" => Some((
                    "zone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "cluster-id",
                            "name",
                            "node-pool-id",
                            "project-id",
                            "respect-pdb",
                            "zone",
                        ],
                    );
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::RollbackNodePoolUpgradeRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self
            .hub
            .projects()
            .locations_clusters_node_pools_rollback(request, opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_clusters_node_pools_set_autoscaling(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "autoscaling.autoprovisioned" => Some((
                    "autoscaling.autoprovisioned",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "autoscaling.enabled" => Some((
                    "autoscaling.enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "autoscaling.location-policy" => Some((
                    "autoscaling.locationPolicy",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "autoscaling.max-node-count" => Some((
                    "autoscaling.maxNodeCount",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "autoscaling.min-node-count" => Some((
                    "autoscaling.minNodeCount",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "autoscaling.total-max-node-count" => Some((
                    "autoscaling.totalMaxNodeCount",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "autoscaling.total-min-node-count" => Some((
                    "autoscaling.totalMinNodeCount",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "cluster-id" => Some((
                    "clusterId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "name" => Some((
                    "name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "node-pool-id" => Some((
                    "nodePoolId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "project-id" => Some((
                    "projectId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "zone" => Some((
                    "zone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "autoprovisioned",
                            "autoscaling",
                            "cluster-id",
                            "enabled",
                            "location-policy",
                            "max-node-count",
                            "min-node-count",
                            "name",
                            "node-pool-id",
                            "project-id",
                            "total-max-node-count",
                            "total-min-node-count",
                            "zone",
                        ],
                    );
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::SetNodePoolAutoscalingRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self
            .hub
            .projects()
            .locations_clusters_node_pools_set_autoscaling(
                request,
                opt.value_of("name").unwrap_or(""),
            );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_clusters_node_pools_set_management(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "cluster-id" => Some((
                    "clusterId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "management.auto-repair" => Some((
                    "management.autoRepair",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "management.auto-upgrade" => Some((
                    "management.autoUpgrade",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "management.upgrade-options.auto-upgrade-start-time" => Some((
                    "management.upgradeOptions.autoUpgradeStartTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "management.upgrade-options.description" => Some((
                    "management.upgradeOptions.description",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "name" => Some((
                    "name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "node-pool-id" => Some((
                    "nodePoolId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "project-id" => Some((
                    "projectId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "zone" => Some((
                    "zone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "auto-repair",
                            "auto-upgrade",
                            "auto-upgrade-start-time",
                            "cluster-id",
                            "description",
                            "management",
                            "name",
                            "node-pool-id",
                            "project-id",
                            "upgrade-options",
                            "zone",
                        ],
                    );
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::SetNodePoolManagementRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self
            .hub
            .projects()
            .locations_clusters_node_pools_set_management(
                request,
                opt.value_of("name").unwrap_or(""),
            );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_clusters_node_pools_set_size(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "cluster-id" => Some((
                    "clusterId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "name" => Some((
                    "name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "node-count" => Some((
                    "nodeCount",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "node-pool-id" => Some((
                    "nodePoolId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "project-id" => Some((
                    "projectId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "zone" => Some((
                    "zone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "cluster-id",
                            "name",
                            "node-count",
                            "node-pool-id",
                            "project-id",
                            "zone",
                        ],
                    );
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::SetNodePoolSizeRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self
            .hub
            .projects()
            .locations_clusters_node_pools_set_size(request, opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_clusters_node_pools_update(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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
                    "boot-disk.disk-type" => Some(("bootDisk.diskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "boot-disk.provisioned-iops" => Some(("bootDisk.provisionedIops", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "boot-disk.provisioned-throughput" => Some(("bootDisk.provisionedThroughput", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "boot-disk.size-gb" => Some(("bootDisk.sizeGb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "confidential-nodes.confidential-instance-type" => Some(("confidentialNodes.confidentialInstanceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "confidential-nodes.enabled" => Some(("confidentialNodes.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "containerd-config.private-registry-access-config.enabled" => Some(("containerdConfig.privateRegistryAccessConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "containerd-config.writable-cgroups.enabled" => Some(("containerdConfig.writableCgroups.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "disk-size-gb" => Some(("diskSizeGb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "disk-type" => Some(("diskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fast-socket.enabled" => Some(("fastSocket.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "flex-start" => Some(("flexStart", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "gcfs-config.enabled" => Some(("gcfsConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "gvnic.enabled" => Some(("gvnic.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "image-type" => Some(("imageType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.allowed-unsafe-sysctls" => Some(("kubeletConfig.allowedUnsafeSysctls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "kubelet-config.container-log-max-files" => Some(("kubeletConfig.containerLogMaxFiles", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "kubelet-config.container-log-max-size" => Some(("kubeletConfig.containerLogMaxSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.cpu-cfs-quota" => Some(("kubeletConfig.cpuCfsQuota", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kubelet-config.cpu-cfs-quota-period" => Some(("kubeletConfig.cpuCfsQuotaPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.cpu-manager-policy" => Some(("kubeletConfig.cpuManagerPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.eviction-max-pod-grace-period-seconds" => Some(("kubeletConfig.evictionMaxPodGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "kubelet-config.eviction-minimum-reclaim.imagefs-available" => Some(("kubeletConfig.evictionMinimumReclaim.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.eviction-minimum-reclaim.imagefs-inodes-free" => Some(("kubeletConfig.evictionMinimumReclaim.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.eviction-minimum-reclaim.memory-available" => Some(("kubeletConfig.evictionMinimumReclaim.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.eviction-minimum-reclaim.nodefs-available" => Some(("kubeletConfig.evictionMinimumReclaim.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.eviction-minimum-reclaim.nodefs-inodes-free" => Some(("kubeletConfig.evictionMinimumReclaim.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.eviction-minimum-reclaim.pid-available" => Some(("kubeletConfig.evictionMinimumReclaim.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.eviction-soft.imagefs-available" => Some(("kubeletConfig.evictionSoft.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.eviction-soft.imagefs-inodes-free" => Some(("kubeletConfig.evictionSoft.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.eviction-soft.memory-available" => Some(("kubeletConfig.evictionSoft.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.eviction-soft.nodefs-available" => Some(("kubeletConfig.evictionSoft.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.eviction-soft.nodefs-inodes-free" => Some(("kubeletConfig.evictionSoft.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.eviction-soft.pid-available" => Some(("kubeletConfig.evictionSoft.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.eviction-soft-grace-period.imagefs-available" => Some(("kubeletConfig.evictionSoftGracePeriod.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.eviction-soft-grace-period.imagefs-inodes-free" => Some(("kubeletConfig.evictionSoftGracePeriod.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.eviction-soft-grace-period.memory-available" => Some(("kubeletConfig.evictionSoftGracePeriod.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.eviction-soft-grace-period.nodefs-available" => Some(("kubeletConfig.evictionSoftGracePeriod.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.eviction-soft-grace-period.nodefs-inodes-free" => Some(("kubeletConfig.evictionSoftGracePeriod.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.eviction-soft-grace-period.pid-available" => Some(("kubeletConfig.evictionSoftGracePeriod.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.image-gc-high-threshold-percent" => Some(("kubeletConfig.imageGcHighThresholdPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "kubelet-config.image-gc-low-threshold-percent" => Some(("kubeletConfig.imageGcLowThresholdPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "kubelet-config.image-maximum-gc-age" => Some(("kubeletConfig.imageMaximumGcAge", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.image-minimum-gc-age" => Some(("kubeletConfig.imageMinimumGcAge", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.insecure-kubelet-readonly-port-enabled" => Some(("kubeletConfig.insecureKubeletReadonlyPortEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kubelet-config.max-parallel-image-pulls" => Some(("kubeletConfig.maxParallelImagePulls", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "kubelet-config.memory-manager.policy" => Some(("kubeletConfig.memoryManager.policy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.pod-pids-limit" => Some(("kubeletConfig.podPidsLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.single-process-oom-kill" => Some(("kubeletConfig.singleProcessOomKill", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kubelet-config.topology-manager.policy" => Some(("kubeletConfig.topologyManager.policy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.topology-manager.scope" => Some(("kubeletConfig.topologyManager.scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels.labels" => Some(("labels.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "linux-node-config.cgroup-mode" => Some(("linuxNodeConfig.cgroupMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "linux-node-config.hugepages.hugepage-size1g" => Some(("linuxNodeConfig.hugepages.hugepageSize1g", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "linux-node-config.hugepages.hugepage-size2m" => Some(("linuxNodeConfig.hugepages.hugepageSize2m", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "linux-node-config.node-kernel-module-loading.policy" => Some(("linuxNodeConfig.nodeKernelModuleLoading.policy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "linux-node-config.sysctls" => Some(("linuxNodeConfig.sysctls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "linux-node-config.transparent-hugepage-defrag" => Some(("linuxNodeConfig.transparentHugepageDefrag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "linux-node-config.transparent-hugepage-enabled" => Some(("linuxNodeConfig.transparentHugepageEnabled", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "locations" => Some(("locations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "logging-config.variant-config.variant" => Some(("loggingConfig.variantConfig.variant", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "machine-type" => Some(("machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "max-run-duration" => Some(("maxRunDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-drain-config.respect-pdb-during-node-pool-deletion" => Some(("nodeDrainConfig.respectPdbDuringNodePoolDeletion", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-network-config.create-pod-range" => Some(("nodeNetworkConfig.createPodRange", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-network-config.enable-private-nodes" => Some(("nodeNetworkConfig.enablePrivateNodes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-network-config.network-performance-config.total-egress-bandwidth-tier" => Some(("nodeNetworkConfig.networkPerformanceConfig.totalEgressBandwidthTier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-network-config.network-tier-config.network-tier" => Some(("nodeNetworkConfig.networkTierConfig.networkTier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-network-config.pod-cidr-overprovision-config.disable" => Some(("nodeNetworkConfig.podCidrOverprovisionConfig.disable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-network-config.pod-ipv4-cidr-block" => Some(("nodeNetworkConfig.podIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-network-config.pod-ipv4-range-utilization" => Some(("nodeNetworkConfig.podIpv4RangeUtilization", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "node-network-config.pod-range" => Some(("nodeNetworkConfig.podRange", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-network-config.subnetwork" => Some(("nodeNetworkConfig.subnetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool-id" => Some(("nodePoolId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-version" => Some(("nodeVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "queued-provisioning.enabled" => Some(("queuedProvisioning.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "resource-labels.labels" => Some(("resourceLabels.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "resource-manager-tags.tags" => Some(("resourceManagerTags.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "storage-pools" => Some(("storagePools", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "tags.tags" => Some(("tags.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "upgrade-settings.blue-green-settings.autoscaled-rollout-policy.wait-for-drain-duration" => Some(("upgradeSettings.blueGreenSettings.autoscaledRolloutPolicy.waitForDrainDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["allowed-unsafe-sysctls", "autoscaled-rollout-policy", "batch-node-count", "batch-percentage", "batch-soak-duration", "blue-green-settings", "boot-disk", "cgroup-mode", "cluster-id", "confidential-instance-type", "confidential-nodes", "container-log-max-files", "container-log-max-size", "containerd-config", "cpu-cfs-quota", "cpu-cfs-quota-period", "cpu-manager-policy", "create-pod-range", "disable", "disk-size-gb", "disk-type", "enable-private-nodes", "enabled", "etag", "eviction-max-pod-grace-period-seconds", "eviction-minimum-reclaim", "eviction-soft", "eviction-soft-grace-period", "fast-socket", "flex-start", "gcfs-config", "gvnic", "hugepage-size1g", "hugepage-size2m", "hugepages", "image-gc-high-threshold-percent", "image-gc-low-threshold-percent", "image-maximum-gc-age", "image-minimum-gc-age", "image-type", "imagefs-available", "imagefs-inodes-free", "insecure-kubelet-readonly-port-enabled", "kubelet-config", "labels", "linux-node-config", "locations", "logging-config", "machine-type", "max-parallel-image-pulls", "max-run-duration", "max-surge", "max-unavailable", "memory-available", "memory-manager", "mode", "name", "network-performance-config", "network-tier", "network-tier-config", "node-drain-config", "node-kernel-module-loading", "node-network-config", "node-pool-id", "node-pool-soak-duration", "node-version", "nodefs-available", "nodefs-inodes-free", "os-version", "pid-available", "pod-cidr-overprovision-config", "pod-ipv4-cidr-block", "pod-ipv4-range-utilization", "pod-pids-limit", "pod-range", "policy", "private-registry-access-config", "project-id", "provisioned-iops", "provisioned-throughput", "queued-provisioning", "resource-labels", "resource-manager-tags", "respect-pdb-during-node-pool-deletion", "scope", "single-process-oom-kill", "size-gb", "standard-rollout-policy", "storage-pools", "strategy", "subnetwork", "sysctls", "tags", "topology-manager", "total-egress-bandwidth-tier", "transparent-hugepage-defrag", "transparent-hugepage-enabled", "upgrade-settings", "variant", "variant-config", "wait-for-drain-duration", "windows-node-config", "workload-metadata-config", "writable-cgroups", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::UpdateNodePoolRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self
            .hub
            .projects()
            .locations_clusters_node_pools_update(request, opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_clusters_set_addons(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "addons-config.cloud-run-config.disabled" => Some((
                    "addonsConfig.cloudRunConfig.disabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "addons-config.cloud-run-config.load-balancer-type" => Some((
                    "addonsConfig.cloudRunConfig.loadBalancerType",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "addons-config.config-connector-config.enabled" => Some((
                    "addonsConfig.configConnectorConfig.enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "addons-config.dns-cache-config.enabled" => Some((
                    "addonsConfig.dnsCacheConfig.enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "addons-config.gce-persistent-disk-csi-driver-config.enabled" => Some((
                    "addonsConfig.gcePersistentDiskCsiDriverConfig.enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "addons-config.gcp-filestore-csi-driver-config.enabled" => Some((
                    "addonsConfig.gcpFilestoreCsiDriverConfig.enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "addons-config.gcs-fuse-csi-driver-config.enabled" => Some((
                    "addonsConfig.gcsFuseCsiDriverConfig.enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "addons-config.gke-backup-agent-config.enabled" => Some((
                    "addonsConfig.gkeBackupAgentConfig.enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "addons-config.high-scale-checkpointing-config.enabled" => Some((
                    "addonsConfig.highScaleCheckpointingConfig.enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "addons-config.horizontal-pod-autoscaling.disabled" => Some((
                    "addonsConfig.horizontalPodAutoscaling.disabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "addons-config.http-load-balancing.disabled" => Some((
                    "addonsConfig.httpLoadBalancing.disabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "addons-config.kubernetes-dashboard.disabled" => Some((
                    "addonsConfig.kubernetesDashboard.disabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "addons-config.lustre-csi-driver-config.enable-legacy-lustre-port" => Some((
                    "addonsConfig.lustreCsiDriverConfig.enableLegacyLustrePort",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "addons-config.lustre-csi-driver-config.enabled" => Some((
                    "addonsConfig.lustreCsiDriverConfig.enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "addons-config.network-policy-config.disabled" => Some((
                    "addonsConfig.networkPolicyConfig.disabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "addons-config.parallelstore-csi-driver-config.enabled" => Some((
                    "addonsConfig.parallelstoreCsiDriverConfig.enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "addons-config.ray-operator-config.enabled" => Some((
                    "addonsConfig.rayOperatorConfig.enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "addons-config.ray-operator-config.ray-cluster-logging-config.enabled" => Some((
                    "addonsConfig.rayOperatorConfig.rayClusterLoggingConfig.enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "addons-config.ray-operator-config.ray-cluster-monitoring-config.enabled" => {
                    Some((
                        "addonsConfig.rayOperatorConfig.rayClusterMonitoringConfig.enabled",
                        JsonTypeInfo {
                            jtype: JsonType::Boolean,
                            ctype: ComplexType::Pod,
                        },
                    ))
                }
                "addons-config.stateful-ha-config.enabled" => Some((
                    "addonsConfig.statefulHaConfig.enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "cluster-id" => Some((
                    "clusterId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "name" => Some((
                    "name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "project-id" => Some((
                    "projectId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "zone" => Some((
                    "zone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "addons-config",
                            "cloud-run-config",
                            "cluster-id",
                            "config-connector-config",
                            "disabled",
                            "dns-cache-config",
                            "enable-legacy-lustre-port",
                            "enabled",
                            "gce-persistent-disk-csi-driver-config",
                            "gcp-filestore-csi-driver-config",
                            "gcs-fuse-csi-driver-config",
                            "gke-backup-agent-config",
                            "high-scale-checkpointing-config",
                            "horizontal-pod-autoscaling",
                            "http-load-balancing",
                            "kubernetes-dashboard",
                            "load-balancer-type",
                            "lustre-csi-driver-config",
                            "name",
                            "network-policy-config",
                            "parallelstore-csi-driver-config",
                            "project-id",
                            "ray-cluster-logging-config",
                            "ray-cluster-monitoring-config",
                            "ray-operator-config",
                            "stateful-ha-config",
                            "zone",
                        ],
                    );
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::SetAddonsConfigRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self
            .hub
            .projects()
            .locations_clusters_set_addons(request, opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_clusters_set_legacy_abac(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "cluster-id" => Some((
                    "clusterId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "enabled" => Some((
                    "enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "name" => Some((
                    "name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "project-id" => Some((
                    "projectId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "zone" => Some((
                    "zone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec!["cluster-id", "enabled", "name", "project-id", "zone"],
                    );
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::SetLegacyAbacRequest = serde_json::value::from_value(object).unwrap();
        let mut call = self
            .hub
            .projects()
            .locations_clusters_set_legacy_abac(request, opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_clusters_set_locations(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "cluster-id" => Some((
                    "clusterId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "locations" => Some((
                    "locations",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Vec,
                    },
                )),
                "name" => Some((
                    "name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "project-id" => Some((
                    "projectId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "zone" => Some((
                    "zone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec!["cluster-id", "locations", "name", "project-id", "zone"],
                    );
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::SetLocationsRequest = serde_json::value::from_value(object).unwrap();
        let mut call = self
            .hub
            .projects()
            .locations_clusters_set_locations(request, opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_clusters_set_logging(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "cluster-id" => Some((
                    "clusterId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "logging-service" => Some((
                    "loggingService",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "name" => Some((
                    "name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "project-id" => Some((
                    "projectId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "zone" => Some((
                    "zone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "cluster-id",
                            "logging-service",
                            "name",
                            "project-id",
                            "zone",
                        ],
                    );
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::SetLoggingServiceRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self
            .hub
            .projects()
            .locations_clusters_set_logging(request, opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_clusters_set_maintenance_policy(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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
                    "maintenance-policy.window.recurring-window.window.maintenance-exclusion-options.end-time-behavior" => Some(("maintenancePolicy.window.recurringWindow.window.maintenanceExclusionOptions.endTimeBehavior", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "maintenance-policy.window.recurring-window.window.maintenance-exclusion-options.scope" => Some(("maintenancePolicy.window.recurringWindow.window.maintenanceExclusionOptions.scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "maintenance-policy.window.recurring-window.window.start-time" => Some(("maintenancePolicy.window.recurringWindow.window.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cluster-id", "daily-maintenance-window", "duration", "end-time", "end-time-behavior", "maintenance-exclusion-options", "maintenance-policy", "name", "project-id", "recurrence", "recurring-window", "resource-version", "scope", "start-time", "window", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::SetMaintenancePolicyRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self
            .hub
            .projects()
            .locations_clusters_set_maintenance_policy(request, opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_clusters_set_master_auth(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "action" => Some((
                    "action",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "cluster-id" => Some((
                    "clusterId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "name" => Some((
                    "name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "project-id" => Some((
                    "projectId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "update.client-certificate" => Some((
                    "update.clientCertificate",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "update.client-certificate-config.issue-client-certificate" => Some((
                    "update.clientCertificateConfig.issueClientCertificate",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "update.client-key" => Some((
                    "update.clientKey",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "update.cluster-ca-certificate" => Some((
                    "update.clusterCaCertificate",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "update.password" => Some((
                    "update.password",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "update.username" => Some((
                    "update.username",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "zone" => Some((
                    "zone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "action",
                            "client-certificate",
                            "client-certificate-config",
                            "client-key",
                            "cluster-ca-certificate",
                            "cluster-id",
                            "issue-client-certificate",
                            "name",
                            "password",
                            "project-id",
                            "update",
                            "username",
                            "zone",
                        ],
                    );
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::SetMasterAuthRequest = serde_json::value::from_value(object).unwrap();
        let mut call = self
            .hub
            .projects()
            .locations_clusters_set_master_auth(request, opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_clusters_set_monitoring(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "cluster-id" => Some((
                    "clusterId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "monitoring-service" => Some((
                    "monitoringService",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "name" => Some((
                    "name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "project-id" => Some((
                    "projectId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "zone" => Some((
                    "zone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "cluster-id",
                            "monitoring-service",
                            "name",
                            "project-id",
                            "zone",
                        ],
                    );
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::SetMonitoringServiceRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self
            .hub
            .projects()
            .locations_clusters_set_monitoring(request, opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_clusters_set_network_policy(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "cluster-id" => Some((
                    "clusterId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "name" => Some((
                    "name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "network-policy.enabled" => Some((
                    "networkPolicy.enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "network-policy.provider" => Some((
                    "networkPolicy.provider",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "project-id" => Some((
                    "projectId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "zone" => Some((
                    "zone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "cluster-id",
                            "enabled",
                            "name",
                            "network-policy",
                            "project-id",
                            "provider",
                            "zone",
                        ],
                    );
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::SetNetworkPolicyRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self
            .hub
            .projects()
            .locations_clusters_set_network_policy(request, opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_clusters_set_resource_labels(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "cluster-id" => Some((
                    "clusterId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "label-fingerprint" => Some((
                    "labelFingerprint",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "name" => Some((
                    "name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "project-id" => Some((
                    "projectId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "resource-labels" => Some((
                    "resourceLabels",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Map,
                    },
                )),
                "zone" => Some((
                    "zone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "cluster-id",
                            "label-fingerprint",
                            "name",
                            "project-id",
                            "resource-labels",
                            "zone",
                        ],
                    );
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::SetLabelsRequest = serde_json::value::from_value(object).unwrap();
        let mut call = self
            .hub
            .projects()
            .locations_clusters_set_resource_labels(request, opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_clusters_start_ip_rotation(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "cluster-id" => Some((
                    "clusterId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "name" => Some((
                    "name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "project-id" => Some((
                    "projectId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "rotate-credentials" => Some((
                    "rotateCredentials",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "zone" => Some((
                    "zone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "cluster-id",
                            "name",
                            "project-id",
                            "rotate-credentials",
                            "zone",
                        ],
                    );
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::StartIPRotationRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self
            .hub
            .projects()
            .locations_clusters_start_ip_rotation(request, opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_clusters_update(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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
                    "update.desired-addons-config.high-scale-checkpointing-config.enabled" => Some(("update.desiredAddonsConfig.highScaleCheckpointingConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.horizontal-pod-autoscaling.disabled" => Some(("update.desiredAddonsConfig.horizontalPodAutoscaling.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.http-load-balancing.disabled" => Some(("update.desiredAddonsConfig.httpLoadBalancing.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.kubernetes-dashboard.disabled" => Some(("update.desiredAddonsConfig.kubernetesDashboard.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.lustre-csi-driver-config.enable-legacy-lustre-port" => Some(("update.desiredAddonsConfig.lustreCsiDriverConfig.enableLegacyLustrePort", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.lustre-csi-driver-config.enabled" => Some(("update.desiredAddonsConfig.lustreCsiDriverConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.network-policy-config.disabled" => Some(("update.desiredAddonsConfig.networkPolicyConfig.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.parallelstore-csi-driver-config.enabled" => Some(("update.desiredAddonsConfig.parallelstoreCsiDriverConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.ray-operator-config.enabled" => Some(("update.desiredAddonsConfig.rayOperatorConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.ray-operator-config.ray-cluster-logging-config.enabled" => Some(("update.desiredAddonsConfig.rayOperatorConfig.rayClusterLoggingConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.ray-operator-config.ray-cluster-monitoring-config.enabled" => Some(("update.desiredAddonsConfig.rayOperatorConfig.rayClusterMonitoringConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.stateful-ha-config.enabled" => Some(("update.desiredAddonsConfig.statefulHaConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-anonymous-authentication-config.mode" => Some(("update.desiredAnonymousAuthenticationConfig.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-authenticator-groups-config.enabled" => Some(("update.desiredAuthenticatorGroupsConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-authenticator-groups-config.security-group" => Some(("update.desiredAuthenticatorGroupsConfig.securityGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-auto-ipam-config.enabled" => Some(("update.desiredAutoIpamConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-autopilot-workload-policy-config.allow-net-admin" => Some(("update.desiredAutopilotWorkloadPolicyConfig.allowNetAdmin", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-autopilot-workload-policy-config.autopilot-compatibility-auditing-enabled" => Some(("update.desiredAutopilotWorkloadPolicyConfig.autopilotCompatibilityAuditingEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
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
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.blue-green-settings.autoscaled-rollout-policy.wait-for-drain-duration" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.blueGreenSettings.autoscaledRolloutPolicy.waitForDrainDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.blue-green-settings.node-pool-soak-duration" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.blueGreenSettings.nodePoolSoakDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.blue-green-settings.standard-rollout-policy.batch-node-count" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.blueGreenSettings.standardRolloutPolicy.batchNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.blue-green-settings.standard-rollout-policy.batch-percentage" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.blueGreenSettings.standardRolloutPolicy.batchPercentage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.blue-green-settings.standard-rollout-policy.batch-soak-duration" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.blueGreenSettings.standardRolloutPolicy.batchSoakDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.max-surge" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.maxSurge", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.max-unavailable" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.maxUnavailable", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.strategy" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.strategy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoscaling-profile" => Some(("update.desiredClusterAutoscaling.autoscalingProfile", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.default-compute-class-config.enabled" => Some(("update.desiredClusterAutoscaling.defaultComputeClassConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.enable-node-autoprovisioning" => Some(("update.desiredClusterAutoscaling.enableNodeAutoprovisioning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-compliance-posture-config.mode" => Some(("update.desiredCompliancePostureConfig.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-containerd-config.private-registry-access-config.enabled" => Some(("update.desiredContainerdConfig.privateRegistryAccessConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-containerd-config.writable-cgroups.enabled" => Some(("update.desiredContainerdConfig.writableCgroups.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-control-plane-endpoints-config.dns-endpoint-config.allow-external-traffic" => Some(("update.desiredControlPlaneEndpointsConfig.dnsEndpointConfig.allowExternalTraffic", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-control-plane-endpoints-config.dns-endpoint-config.enable-k8s-certs-via-dns" => Some(("update.desiredControlPlaneEndpointsConfig.dnsEndpointConfig.enableK8sCertsViaDns", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-control-plane-endpoints-config.dns-endpoint-config.enable-k8s-tokens-via-dns" => Some(("update.desiredControlPlaneEndpointsConfig.dnsEndpointConfig.enableK8sTokensViaDns", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-control-plane-endpoints-config.dns-endpoint-config.endpoint" => Some(("update.desiredControlPlaneEndpointsConfig.dnsEndpointConfig.endpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-control-plane-endpoints-config.ip-endpoints-config.authorized-networks-config.enabled" => Some(("update.desiredControlPlaneEndpointsConfig.ipEndpointsConfig.authorizedNetworksConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-control-plane-endpoints-config.ip-endpoints-config.authorized-networks-config.gcp-public-cidrs-access-enabled" => Some(("update.desiredControlPlaneEndpointsConfig.ipEndpointsConfig.authorizedNetworksConfig.gcpPublicCidrsAccessEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-control-plane-endpoints-config.ip-endpoints-config.authorized-networks-config.private-endpoint-enforcement-enabled" => Some(("update.desiredControlPlaneEndpointsConfig.ipEndpointsConfig.authorizedNetworksConfig.privateEndpointEnforcementEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-control-plane-endpoints-config.ip-endpoints-config.enable-public-endpoint" => Some(("update.desiredControlPlaneEndpointsConfig.ipEndpointsConfig.enablePublicEndpoint", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-control-plane-endpoints-config.ip-endpoints-config.enabled" => Some(("update.desiredControlPlaneEndpointsConfig.ipEndpointsConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-control-plane-endpoints-config.ip-endpoints-config.global-access" => Some(("update.desiredControlPlaneEndpointsConfig.ipEndpointsConfig.globalAccess", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-control-plane-endpoints-config.ip-endpoints-config.private-endpoint" => Some(("update.desiredControlPlaneEndpointsConfig.ipEndpointsConfig.privateEndpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-control-plane-endpoints-config.ip-endpoints-config.private-endpoint-subnetwork" => Some(("update.desiredControlPlaneEndpointsConfig.ipEndpointsConfig.privateEndpointSubnetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-control-plane-endpoints-config.ip-endpoints-config.public-endpoint" => Some(("update.desiredControlPlaneEndpointsConfig.ipEndpointsConfig.publicEndpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-cost-management-config.enabled" => Some(("update.desiredCostManagementConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-database-encryption.current-state" => Some(("update.desiredDatabaseEncryption.currentState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-database-encryption.decryption-keys" => Some(("update.desiredDatabaseEncryption.decryptionKeys", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.desired-database-encryption.key-name" => Some(("update.desiredDatabaseEncryption.keyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-database-encryption.state" => Some(("update.desiredDatabaseEncryption.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-datapath-provider" => Some(("update.desiredDatapathProvider", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-default-enable-private-nodes" => Some(("update.desiredDefaultEnablePrivateNodes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-default-snat-status.disabled" => Some(("update.desiredDefaultSnatStatus.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-disable-l4-lb-firewall-reconciliation" => Some(("update.desiredDisableL4LbFirewallReconciliation", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-dns-config.additive-vpc-scope-dns-domain" => Some(("update.desiredDnsConfig.additiveVpcScopeDnsDomain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-dns-config.cluster-dns" => Some(("update.desiredDnsConfig.clusterDns", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-dns-config.cluster-dns-domain" => Some(("update.desiredDnsConfig.clusterDnsDomain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-dns-config.cluster-dns-scope" => Some(("update.desiredDnsConfig.clusterDnsScope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-enable-cilium-clusterwide-network-policy" => Some(("update.desiredEnableCiliumClusterwideNetworkPolicy", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-enable-fqdn-network-policy" => Some(("update.desiredEnableFqdnNetworkPolicy", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-enable-multi-networking" => Some(("update.desiredEnableMultiNetworking", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-enable-private-endpoint" => Some(("update.desiredEnablePrivateEndpoint", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-enterprise-config.desired-tier" => Some(("update.desiredEnterpriseConfig.desiredTier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-fleet.membership" => Some(("update.desiredFleet.membership", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-fleet.membership-type" => Some(("update.desiredFleet.membershipType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                    "update.desired-managed-opentelemetry-config.scope" => Some(("update.desiredManagedOpentelemetryConfig.scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-master-authorized-networks-config.enabled" => Some(("update.desiredMasterAuthorizedNetworksConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-master-authorized-networks-config.gcp-public-cidrs-access-enabled" => Some(("update.desiredMasterAuthorizedNetworksConfig.gcpPublicCidrsAccessEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-master-authorized-networks-config.private-endpoint-enforcement-enabled" => Some(("update.desiredMasterAuthorizedNetworksConfig.privateEndpointEnforcementEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-master-version" => Some(("update.desiredMasterVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-mesh-certificates.enable-certificates" => Some(("update.desiredMeshCertificates.enableCertificates", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-monitoring-config.advanced-datapath-observability-config.enable-metrics" => Some(("update.desiredMonitoringConfig.advancedDatapathObservabilityConfig.enableMetrics", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-monitoring-config.advanced-datapath-observability-config.enable-relay" => Some(("update.desiredMonitoringConfig.advancedDatapathObservabilityConfig.enableRelay", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-monitoring-config.advanced-datapath-observability-config.relay-mode" => Some(("update.desiredMonitoringConfig.advancedDatapathObservabilityConfig.relayMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-monitoring-config.component-config.enable-components" => Some(("update.desiredMonitoringConfig.componentConfig.enableComponents", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.desired-monitoring-config.managed-prometheus-config.auto-monitoring-config.scope" => Some(("update.desiredMonitoringConfig.managedPrometheusConfig.autoMonitoringConfig.scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-monitoring-config.managed-prometheus-config.enabled" => Some(("update.desiredMonitoringConfig.managedPrometheusConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-monitoring-service" => Some(("update.desiredMonitoringService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-network-performance-config.total-egress-bandwidth-tier" => Some(("update.desiredNetworkPerformanceConfig.totalEgressBandwidthTier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-network-tier-config.network-tier" => Some(("update.desiredNetworkTierConfig.networkTier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.allowed-unsafe-sysctls" => Some(("update.desiredNodeKubeletConfig.allowedUnsafeSysctls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.desired-node-kubelet-config.container-log-max-files" => Some(("update.desiredNodeKubeletConfig.containerLogMaxFiles", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.container-log-max-size" => Some(("update.desiredNodeKubeletConfig.containerLogMaxSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.cpu-cfs-quota" => Some(("update.desiredNodeKubeletConfig.cpuCfsQuota", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.cpu-cfs-quota-period" => Some(("update.desiredNodeKubeletConfig.cpuCfsQuotaPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.cpu-manager-policy" => Some(("update.desiredNodeKubeletConfig.cpuManagerPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.eviction-max-pod-grace-period-seconds" => Some(("update.desiredNodeKubeletConfig.evictionMaxPodGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.eviction-minimum-reclaim.imagefs-available" => Some(("update.desiredNodeKubeletConfig.evictionMinimumReclaim.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.eviction-minimum-reclaim.imagefs-inodes-free" => Some(("update.desiredNodeKubeletConfig.evictionMinimumReclaim.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.eviction-minimum-reclaim.memory-available" => Some(("update.desiredNodeKubeletConfig.evictionMinimumReclaim.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.eviction-minimum-reclaim.nodefs-available" => Some(("update.desiredNodeKubeletConfig.evictionMinimumReclaim.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.eviction-minimum-reclaim.nodefs-inodes-free" => Some(("update.desiredNodeKubeletConfig.evictionMinimumReclaim.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.eviction-minimum-reclaim.pid-available" => Some(("update.desiredNodeKubeletConfig.evictionMinimumReclaim.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.eviction-soft.imagefs-available" => Some(("update.desiredNodeKubeletConfig.evictionSoft.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.eviction-soft.imagefs-inodes-free" => Some(("update.desiredNodeKubeletConfig.evictionSoft.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.eviction-soft.memory-available" => Some(("update.desiredNodeKubeletConfig.evictionSoft.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.eviction-soft.nodefs-available" => Some(("update.desiredNodeKubeletConfig.evictionSoft.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.eviction-soft.nodefs-inodes-free" => Some(("update.desiredNodeKubeletConfig.evictionSoft.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.eviction-soft.pid-available" => Some(("update.desiredNodeKubeletConfig.evictionSoft.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.eviction-soft-grace-period.imagefs-available" => Some(("update.desiredNodeKubeletConfig.evictionSoftGracePeriod.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.eviction-soft-grace-period.imagefs-inodes-free" => Some(("update.desiredNodeKubeletConfig.evictionSoftGracePeriod.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.eviction-soft-grace-period.memory-available" => Some(("update.desiredNodeKubeletConfig.evictionSoftGracePeriod.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.eviction-soft-grace-period.nodefs-available" => Some(("update.desiredNodeKubeletConfig.evictionSoftGracePeriod.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.eviction-soft-grace-period.nodefs-inodes-free" => Some(("update.desiredNodeKubeletConfig.evictionSoftGracePeriod.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.eviction-soft-grace-period.pid-available" => Some(("update.desiredNodeKubeletConfig.evictionSoftGracePeriod.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.image-gc-high-threshold-percent" => Some(("update.desiredNodeKubeletConfig.imageGcHighThresholdPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.image-gc-low-threshold-percent" => Some(("update.desiredNodeKubeletConfig.imageGcLowThresholdPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.image-maximum-gc-age" => Some(("update.desiredNodeKubeletConfig.imageMaximumGcAge", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.image-minimum-gc-age" => Some(("update.desiredNodeKubeletConfig.imageMinimumGcAge", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.insecure-kubelet-readonly-port-enabled" => Some(("update.desiredNodeKubeletConfig.insecureKubeletReadonlyPortEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.max-parallel-image-pulls" => Some(("update.desiredNodeKubeletConfig.maxParallelImagePulls", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.memory-manager.policy" => Some(("update.desiredNodeKubeletConfig.memoryManager.policy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.pod-pids-limit" => Some(("update.desiredNodeKubeletConfig.podPidsLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.single-process-oom-kill" => Some(("update.desiredNodeKubeletConfig.singleProcessOomKill", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.topology-manager.policy" => Some(("update.desiredNodeKubeletConfig.topologyManager.policy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.topology-manager.scope" => Some(("update.desiredNodeKubeletConfig.topologyManager.scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.allowed-unsafe-sysctls" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.allowedUnsafeSysctls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.desired-node-pool-auto-config-kubelet-config.container-log-max-files" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.containerLogMaxFiles", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.container-log-max-size" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.containerLogMaxSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.cpu-cfs-quota" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.cpuCfsQuota", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.cpu-cfs-quota-period" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.cpuCfsQuotaPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.cpu-manager-policy" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.cpuManagerPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.eviction-max-pod-grace-period-seconds" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.evictionMaxPodGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.eviction-minimum-reclaim.imagefs-available" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.evictionMinimumReclaim.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.eviction-minimum-reclaim.imagefs-inodes-free" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.evictionMinimumReclaim.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.eviction-minimum-reclaim.memory-available" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.evictionMinimumReclaim.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.eviction-minimum-reclaim.nodefs-available" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.evictionMinimumReclaim.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.eviction-minimum-reclaim.nodefs-inodes-free" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.evictionMinimumReclaim.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.eviction-minimum-reclaim.pid-available" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.evictionMinimumReclaim.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.eviction-soft.imagefs-available" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.evictionSoft.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.eviction-soft.imagefs-inodes-free" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.evictionSoft.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.eviction-soft.memory-available" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.evictionSoft.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.eviction-soft.nodefs-available" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.evictionSoft.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.eviction-soft.nodefs-inodes-free" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.evictionSoft.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.eviction-soft.pid-available" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.evictionSoft.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.eviction-soft-grace-period.imagefs-available" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.evictionSoftGracePeriod.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.eviction-soft-grace-period.imagefs-inodes-free" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.evictionSoftGracePeriod.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.eviction-soft-grace-period.memory-available" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.evictionSoftGracePeriod.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.eviction-soft-grace-period.nodefs-available" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.evictionSoftGracePeriod.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.eviction-soft-grace-period.nodefs-inodes-free" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.evictionSoftGracePeriod.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.eviction-soft-grace-period.pid-available" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.evictionSoftGracePeriod.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.image-gc-high-threshold-percent" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.imageGcHighThresholdPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.image-gc-low-threshold-percent" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.imageGcLowThresholdPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.image-maximum-gc-age" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.imageMaximumGcAge", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.image-minimum-gc-age" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.imageMinimumGcAge", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.insecure-kubelet-readonly-port-enabled" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.insecureKubeletReadonlyPortEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.max-parallel-image-pulls" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.maxParallelImagePulls", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.memory-manager.policy" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.memoryManager.policy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.pod-pids-limit" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.podPidsLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.single-process-oom-kill" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.singleProcessOomKill", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.topology-manager.policy" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.topologyManager.policy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.topology-manager.scope" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.topologyManager.scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-linux-node-config.cgroup-mode" => Some(("update.desiredNodePoolAutoConfigLinuxNodeConfig.cgroupMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-linux-node-config.hugepages.hugepage-size1g" => Some(("update.desiredNodePoolAutoConfigLinuxNodeConfig.hugepages.hugepageSize1g", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-linux-node-config.hugepages.hugepage-size2m" => Some(("update.desiredNodePoolAutoConfigLinuxNodeConfig.hugepages.hugepageSize2m", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-linux-node-config.node-kernel-module-loading.policy" => Some(("update.desiredNodePoolAutoConfigLinuxNodeConfig.nodeKernelModuleLoading.policy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-linux-node-config.sysctls" => Some(("update.desiredNodePoolAutoConfigLinuxNodeConfig.sysctls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "update.desired-node-pool-auto-config-linux-node-config.transparent-hugepage-defrag" => Some(("update.desiredNodePoolAutoConfigLinuxNodeConfig.transparentHugepageDefrag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-linux-node-config.transparent-hugepage-enabled" => Some(("update.desiredNodePoolAutoConfigLinuxNodeConfig.transparentHugepageEnabled", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                    "update.desired-pod-autoscaling.hpa-profile" => Some(("update.desiredPodAutoscaling.hpaProfile", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-private-cluster-config.enable-private-endpoint" => Some(("update.desiredPrivateClusterConfig.enablePrivateEndpoint", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-private-cluster-config.enable-private-nodes" => Some(("update.desiredPrivateClusterConfig.enablePrivateNodes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-private-cluster-config.master-global-access-config.enabled" => Some(("update.desiredPrivateClusterConfig.masterGlobalAccessConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-private-cluster-config.master-ipv4-cidr-block" => Some(("update.desiredPrivateClusterConfig.masterIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-private-cluster-config.peering-name" => Some(("update.desiredPrivateClusterConfig.peeringName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-private-cluster-config.private-endpoint" => Some(("update.desiredPrivateClusterConfig.privateEndpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-private-cluster-config.private-endpoint-subnetwork" => Some(("update.desiredPrivateClusterConfig.privateEndpointSubnetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-private-cluster-config.public-endpoint" => Some(("update.desiredPrivateClusterConfig.publicEndpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-private-ipv6-google-access" => Some(("update.desiredPrivateIpv6GoogleAccess", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-privileged-admission-config.allowlist-paths" => Some(("update.desiredPrivilegedAdmissionConfig.allowlistPaths", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.desired-rbac-binding-config.enable-insecure-binding-system-authenticated" => Some(("update.desiredRbacBindingConfig.enableInsecureBindingSystemAuthenticated", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-rbac-binding-config.enable-insecure-binding-system-unauthenticated" => Some(("update.desiredRbacBindingConfig.enableInsecureBindingSystemUnauthenticated", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-release-channel.channel" => Some(("update.desiredReleaseChannel.channel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-resource-usage-export-config.bigquery-destination.dataset-id" => Some(("update.desiredResourceUsageExportConfig.bigqueryDestination.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-resource-usage-export-config.consumption-metering-config.enabled" => Some(("update.desiredResourceUsageExportConfig.consumptionMeteringConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-resource-usage-export-config.enable-network-egress-metering" => Some(("update.desiredResourceUsageExportConfig.enableNetworkEgressMetering", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-secret-manager-config.enabled" => Some(("update.desiredSecretManagerConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-secret-manager-config.rotation-config.enabled" => Some(("update.desiredSecretManagerConfig.rotationConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-secret-manager-config.rotation-config.rotation-interval" => Some(("update.desiredSecretManagerConfig.rotationConfig.rotationInterval", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-security-posture-config.mode" => Some(("update.desiredSecurityPostureConfig.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-security-posture-config.vulnerability-mode" => Some(("update.desiredSecurityPostureConfig.vulnerabilityMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-service-external-ips-config.enabled" => Some(("update.desiredServiceExternalIpsConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-shielded-nodes.enabled" => Some(("update.desiredShieldedNodes.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-stack-type" => Some(("update.desiredStackType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-user-managed-keys-config.aggregation-ca" => Some(("update.desiredUserManagedKeysConfig.aggregationCa", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-user-managed-keys-config.cluster-ca" => Some(("update.desiredUserManagedKeysConfig.clusterCa", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-user-managed-keys-config.control-plane-disk-encryption-key" => Some(("update.desiredUserManagedKeysConfig.controlPlaneDiskEncryptionKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-user-managed-keys-config.control-plane-disk-encryption-key-versions" => Some(("update.desiredUserManagedKeysConfig.controlPlaneDiskEncryptionKeyVersions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.desired-user-managed-keys-config.etcd-api-ca" => Some(("update.desiredUserManagedKeysConfig.etcdApiCa", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-user-managed-keys-config.etcd-peer-ca" => Some(("update.desiredUserManagedKeysConfig.etcdPeerCa", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-user-managed-keys-config.gkeops-etcd-backup-encryption-key" => Some(("update.desiredUserManagedKeysConfig.gkeopsEtcdBackupEncryptionKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-user-managed-keys-config.service-account-signing-keys" => Some(("update.desiredUserManagedKeysConfig.serviceAccountSigningKeys", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.desired-user-managed-keys-config.service-account-verification-keys" => Some(("update.desiredUserManagedKeysConfig.serviceAccountVerificationKeys", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.desired-vertical-pod-autoscaling.enabled" => Some(("update.desiredVerticalPodAutoscaling.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-workload-identity-config.workload-pool" => Some(("update.desiredWorkloadIdentityConfig.workloadPool", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.enable-k8s-beta-apis.enabled-apis" => Some(("update.enableK8sBetaApis.enabledApis", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.etag" => Some(("update.etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.gke-auto-upgrade-config.patch-mode" => Some(("update.gkeAutoUpgradeConfig.patchMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.removed-additional-pod-ranges-config.pod-range-names" => Some(("update.removedAdditionalPodRangesConfig.podRangeNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.user-managed-keys-config.aggregation-ca" => Some(("update.userManagedKeysConfig.aggregationCa", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.user-managed-keys-config.cluster-ca" => Some(("update.userManagedKeysConfig.clusterCa", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.user-managed-keys-config.control-plane-disk-encryption-key" => Some(("update.userManagedKeysConfig.controlPlaneDiskEncryptionKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.user-managed-keys-config.control-plane-disk-encryption-key-versions" => Some(("update.userManagedKeysConfig.controlPlaneDiskEncryptionKeyVersions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.user-managed-keys-config.etcd-api-ca" => Some(("update.userManagedKeysConfig.etcdApiCa", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.user-managed-keys-config.etcd-peer-ca" => Some(("update.userManagedKeysConfig.etcdPeerCa", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.user-managed-keys-config.gkeops-etcd-backup-encryption-key" => Some(("update.userManagedKeysConfig.gkeopsEtcdBackupEncryptionKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.user-managed-keys-config.service-account-signing-keys" => Some(("update.userManagedKeysConfig.serviceAccountSigningKeys", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.user-managed-keys-config.service-account-verification-keys" => Some(("update.userManagedKeysConfig.serviceAccountVerificationKeys", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["additional-pod-ranges-config", "additive-vpc-scope-dns-domain", "advanced-datapath-observability-config", "aggregation-ca", "allow-external-traffic", "allow-net-admin", "allowed-unsafe-sysctls", "allowlist-paths", "authorized-networks-config", "auto-monitoring-config", "auto-repair", "auto-upgrade", "auto-upgrade-start-time", "autopilot-compatibility-auditing-enabled", "autoprovisioned", "autoprovisioning-locations", "autoprovisioning-node-pool-defaults", "autoscaled-rollout-policy", "autoscaling-profile", "batch-node-count", "batch-percentage", "batch-soak-duration", "bigquery-destination", "blue-green-settings", "boot-disk-kms-key", "cgroup-mode", "channel", "cloud-run-config", "cluster-ca", "cluster-dns", "cluster-dns-domain", "cluster-dns-scope", "cluster-id", "component-config", "config-connector-config", "consumption-metering-config", "container-log-max-files", "container-log-max-size", "control-plane-disk-encryption-key", "control-plane-disk-encryption-key-versions", "cpu-cfs-quota", "cpu-cfs-quota-period", "cpu-manager-policy", "current-state", "dataset-id", "decryption-keys", "default-compute-class-config", "description", "desired-addons-config", "desired-anonymous-authentication-config", "desired-authenticator-groups-config", "desired-auto-ipam-config", "desired-autopilot-workload-policy-config", "desired-binary-authorization", "desired-cluster-autoscaling", "desired-compliance-posture-config", "desired-containerd-config", "desired-control-plane-endpoints-config", "desired-cost-management-config", "desired-database-encryption", "desired-datapath-provider", "desired-default-enable-private-nodes", "desired-default-snat-status", "desired-disable-l4-lb-firewall-reconciliation", "desired-dns-config", "desired-enable-cilium-clusterwide-network-policy", "desired-enable-fqdn-network-policy", "desired-enable-multi-networking", "desired-enable-private-endpoint", "desired-enterprise-config", "desired-fleet", "desired-gateway-api-config", "desired-gcfs-config", "desired-identity-service-config", "desired-image-type", "desired-in-transit-encryption-config", "desired-intra-node-visibility-config", "desired-k8s-beta-apis", "desired-l4ilb-subsetting-config", "desired-locations", "desired-logging-config", "desired-logging-service", "desired-managed-opentelemetry-config", "desired-master-authorized-networks-config", "desired-master-version", "desired-mesh-certificates", "desired-monitoring-config", "desired-monitoring-service", "desired-network-performance-config", "desired-network-tier-config", "desired-node-kubelet-config", "desired-node-pool-auto-config-kubelet-config", "desired-node-pool-auto-config-linux-node-config", "desired-node-pool-auto-config-network-tags", "desired-node-pool-auto-config-resource-manager-tags", "desired-node-pool-autoscaling", "desired-node-pool-id", "desired-node-pool-logging-config", "desired-node-version", "desired-notification-config", "desired-parent-product-config", "desired-pod-autoscaling", "desired-private-cluster-config", "desired-private-ipv6-google-access", "desired-privileged-admission-config", "desired-rbac-binding-config", "desired-release-channel", "desired-resource-usage-export-config", "desired-secret-manager-config", "desired-security-posture-config", "desired-service-external-ips-config", "desired-shielded-nodes", "desired-stack-type", "desired-tier", "desired-user-managed-keys-config", "desired-vertical-pod-autoscaling", "desired-workload-identity-config", "disabled", "disk-size-gb", "disk-type", "dns-cache-config", "dns-endpoint-config", "enable-certificates", "enable-components", "enable-insecure-binding-system-authenticated", "enable-insecure-binding-system-unauthenticated", "enable-integrity-monitoring", "enable-k8s-beta-apis", "enable-k8s-certs-via-dns", "enable-k8s-tokens-via-dns", "enable-legacy-lustre-port", "enable-metrics", "enable-network-egress-metering", "enable-node-autoprovisioning", "enable-private-endpoint", "enable-private-nodes", "enable-public-endpoint", "enable-relay", "enable-secure-boot", "enabled", "enabled-apis", "endpoint", "etag", "etcd-api-ca", "etcd-peer-ca", "evaluation-mode", "event-type", "eviction-max-pod-grace-period-seconds", "eviction-minimum-reclaim", "eviction-soft", "eviction-soft-grace-period", "filter", "gce-persistent-disk-csi-driver-config", "gcp-filestore-csi-driver-config", "gcp-public-cidrs-access-enabled", "gcs-fuse-csi-driver-config", "gke-auto-upgrade-config", "gke-backup-agent-config", "gkeops-etcd-backup-encryption-key", "global-access", "high-scale-checkpointing-config", "horizontal-pod-autoscaling", "hpa-profile", "http-load-balancing", "hugepage-size1g", "hugepage-size2m", "hugepages", "image-gc-high-threshold-percent", "image-gc-low-threshold-percent", "image-maximum-gc-age", "image-minimum-gc-age", "image-type", "imagefs-available", "imagefs-inodes-free", "insecure-kubelet-readonly-port-enabled", "ip-endpoints-config", "key-name", "kubernetes-dashboard", "labels", "load-balancer-type", "location-policy", "lustre-csi-driver-config", "managed-prometheus-config", "management", "master-global-access-config", "master-ipv4-cidr-block", "max-node-count", "max-parallel-image-pulls", "max-surge", "max-unavailable", "membership", "membership-type", "memory-available", "memory-manager", "min-cpu-platform", "min-node-count", "mode", "name", "network-policy-config", "network-tier", "node-kernel-module-loading", "node-pool-soak-duration", "nodefs-available", "nodefs-inodes-free", "oauth-scopes", "parallelstore-csi-driver-config", "patch-mode", "peering-name", "pid-available", "pod-pids-limit", "pod-range-names", "policy", "pre-registered", "private-endpoint", "private-endpoint-enforcement-enabled", "private-endpoint-subnetwork", "private-registry-access-config", "product-name", "project", "project-id", "public-endpoint", "pubsub", "ray-cluster-logging-config", "ray-cluster-monitoring-config", "ray-operator-config", "relay-mode", "removed-additional-pod-ranges-config", "rotation-config", "rotation-interval", "scope", "security-group", "service-account", "service-account-signing-keys", "service-account-verification-keys", "shielded-instance-config", "single-process-oom-kill", "standard-rollout-policy", "state", "stateful-ha-config", "strategy", "sysctls", "tags", "topic", "topology-manager", "total-egress-bandwidth-tier", "total-max-node-count", "total-min-node-count", "transparent-hugepage-defrag", "transparent-hugepage-enabled", "update", "upgrade-options", "upgrade-settings", "user-managed-keys-config", "variant", "variant-config", "vulnerability-mode", "wait-for-drain-duration", "workload-pool", "writable-cgroups", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::UpdateClusterRequest = serde_json::value::from_value(object).unwrap();
        let mut call = self
            .hub
            .projects()
            .locations_clusters_update(request, opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_clusters_update_master(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "cluster-id" => Some((
                    "clusterId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "master-version" => Some((
                    "masterVersion",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "name" => Some((
                    "name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "project-id" => Some((
                    "projectId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "zone" => Some((
                    "zone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec!["cluster-id", "master-version", "name", "project-id", "zone"],
                    );
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::UpdateMasterRequest = serde_json::value::from_value(object).unwrap();
        let mut call = self
            .hub
            .projects()
            .locations_clusters_update_master(request, opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_clusters_well_known_get_openid_configuration(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .projects()
            .locations_clusters_well_known_get_openid_configuration(
                opt.value_of("parent").unwrap_or(""),
            );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
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
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_get_server_config(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .projects()
            .locations_get_server_config(opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "zone" => {
                    call = call.zone(value.unwrap_or(""));
                }
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
                }
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v.extend(["project-id", "zone"].iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_operations_cancel(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "name" => Some((
                    "name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "operation-id" => Some((
                    "operationId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "project-id" => Some((
                    "projectId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "zone" => Some((
                    "zone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec!["name", "operation-id", "project-id", "zone"],
                    );
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::CancelOperationRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self
            .hub
            .projects()
            .locations_operations_cancel(request, opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_operations_get(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .projects()
            .locations_operations_get(opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "zone" => {
                    call = call.zone(value.unwrap_or(""));
                }
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
                }
                "operation-id" => {
                    call = call.operation_id(value.unwrap_or(""));
                }
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v.extend(["operation-id", "project-id", "zone"].iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_locations_operations_list(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .projects()
            .locations_operations_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "zone" => {
                    call = call.zone(value.unwrap_or(""));
                }
                "project-id" => {
                    call = call.project_id(value.unwrap_or(""));
                }
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v.extend(["project-id", "zone"].iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_zones_clusters_addons(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "addons-config.cloud-run-config.disabled" => Some((
                    "addonsConfig.cloudRunConfig.disabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "addons-config.cloud-run-config.load-balancer-type" => Some((
                    "addonsConfig.cloudRunConfig.loadBalancerType",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "addons-config.config-connector-config.enabled" => Some((
                    "addonsConfig.configConnectorConfig.enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "addons-config.dns-cache-config.enabled" => Some((
                    "addonsConfig.dnsCacheConfig.enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "addons-config.gce-persistent-disk-csi-driver-config.enabled" => Some((
                    "addonsConfig.gcePersistentDiskCsiDriverConfig.enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "addons-config.gcp-filestore-csi-driver-config.enabled" => Some((
                    "addonsConfig.gcpFilestoreCsiDriverConfig.enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "addons-config.gcs-fuse-csi-driver-config.enabled" => Some((
                    "addonsConfig.gcsFuseCsiDriverConfig.enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "addons-config.gke-backup-agent-config.enabled" => Some((
                    "addonsConfig.gkeBackupAgentConfig.enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "addons-config.high-scale-checkpointing-config.enabled" => Some((
                    "addonsConfig.highScaleCheckpointingConfig.enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "addons-config.horizontal-pod-autoscaling.disabled" => Some((
                    "addonsConfig.horizontalPodAutoscaling.disabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "addons-config.http-load-balancing.disabled" => Some((
                    "addonsConfig.httpLoadBalancing.disabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "addons-config.kubernetes-dashboard.disabled" => Some((
                    "addonsConfig.kubernetesDashboard.disabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "addons-config.lustre-csi-driver-config.enable-legacy-lustre-port" => Some((
                    "addonsConfig.lustreCsiDriverConfig.enableLegacyLustrePort",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "addons-config.lustre-csi-driver-config.enabled" => Some((
                    "addonsConfig.lustreCsiDriverConfig.enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "addons-config.network-policy-config.disabled" => Some((
                    "addonsConfig.networkPolicyConfig.disabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "addons-config.parallelstore-csi-driver-config.enabled" => Some((
                    "addonsConfig.parallelstoreCsiDriverConfig.enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "addons-config.ray-operator-config.enabled" => Some((
                    "addonsConfig.rayOperatorConfig.enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "addons-config.ray-operator-config.ray-cluster-logging-config.enabled" => Some((
                    "addonsConfig.rayOperatorConfig.rayClusterLoggingConfig.enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "addons-config.ray-operator-config.ray-cluster-monitoring-config.enabled" => {
                    Some((
                        "addonsConfig.rayOperatorConfig.rayClusterMonitoringConfig.enabled",
                        JsonTypeInfo {
                            jtype: JsonType::Boolean,
                            ctype: ComplexType::Pod,
                        },
                    ))
                }
                "addons-config.stateful-ha-config.enabled" => Some((
                    "addonsConfig.statefulHaConfig.enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "cluster-id" => Some((
                    "clusterId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "name" => Some((
                    "name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "project-id" => Some((
                    "projectId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "zone" => Some((
                    "zone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "addons-config",
                            "cloud-run-config",
                            "cluster-id",
                            "config-connector-config",
                            "disabled",
                            "dns-cache-config",
                            "enable-legacy-lustre-port",
                            "enabled",
                            "gce-persistent-disk-csi-driver-config",
                            "gcp-filestore-csi-driver-config",
                            "gcs-fuse-csi-driver-config",
                            "gke-backup-agent-config",
                            "high-scale-checkpointing-config",
                            "horizontal-pod-autoscaling",
                            "http-load-balancing",
                            "kubernetes-dashboard",
                            "load-balancer-type",
                            "lustre-csi-driver-config",
                            "name",
                            "network-policy-config",
                            "parallelstore-csi-driver-config",
                            "project-id",
                            "ray-cluster-logging-config",
                            "ray-cluster-monitoring-config",
                            "ray-operator-config",
                            "stateful-ha-config",
                            "zone",
                        ],
                    );
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::SetAddonsConfigRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_clusters_addons(
            request,
            opt.value_of("project-id").unwrap_or(""),
            opt.value_of("zone").unwrap_or(""),
            opt.value_of("cluster-id").unwrap_or(""),
        );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_zones_clusters_complete_ip_rotation(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "cluster-id" => Some((
                    "clusterId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "name" => Some((
                    "name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "project-id" => Some((
                    "projectId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "zone" => Some((
                    "zone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec!["cluster-id", "name", "project-id", "zone"],
                    );
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::CompleteIPRotationRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_clusters_complete_ip_rotation(
            request,
            opt.value_of("project-id").unwrap_or(""),
            opt.value_of("zone").unwrap_or(""),
            opt.value_of("cluster-id").unwrap_or(""),
        );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_zones_clusters_create(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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
                    "cluster.addons-config.high-scale-checkpointing-config.enabled" => Some(("cluster.addonsConfig.highScaleCheckpointingConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.horizontal-pod-autoscaling.disabled" => Some(("cluster.addonsConfig.horizontalPodAutoscaling.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.http-load-balancing.disabled" => Some(("cluster.addonsConfig.httpLoadBalancing.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.kubernetes-dashboard.disabled" => Some(("cluster.addonsConfig.kubernetesDashboard.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.lustre-csi-driver-config.enable-legacy-lustre-port" => Some(("cluster.addonsConfig.lustreCsiDriverConfig.enableLegacyLustrePort", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.lustre-csi-driver-config.enabled" => Some(("cluster.addonsConfig.lustreCsiDriverConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.network-policy-config.disabled" => Some(("cluster.addonsConfig.networkPolicyConfig.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.parallelstore-csi-driver-config.enabled" => Some(("cluster.addonsConfig.parallelstoreCsiDriverConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.ray-operator-config.enabled" => Some(("cluster.addonsConfig.rayOperatorConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.ray-operator-config.ray-cluster-logging-config.enabled" => Some(("cluster.addonsConfig.rayOperatorConfig.rayClusterLoggingConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.ray-operator-config.ray-cluster-monitoring-config.enabled" => Some(("cluster.addonsConfig.rayOperatorConfig.rayClusterMonitoringConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.addons-config.stateful-ha-config.enabled" => Some(("cluster.addonsConfig.statefulHaConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.alpha-cluster-feature-gates" => Some(("cluster.alphaClusterFeatureGates", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.anonymous-authentication-config.mode" => Some(("cluster.anonymousAuthenticationConfig.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.authenticator-groups-config.enabled" => Some(("cluster.authenticatorGroupsConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.authenticator-groups-config.security-group" => Some(("cluster.authenticatorGroupsConfig.securityGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.autopilot.enabled" => Some(("cluster.autopilot.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.autopilot.privileged-admission-config.allowlist-paths" => Some(("cluster.autopilot.privilegedAdmissionConfig.allowlistPaths", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.autopilot.workload-policy-config.allow-net-admin" => Some(("cluster.autopilot.workloadPolicyConfig.allowNetAdmin", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.autopilot.workload-policy-config.autopilot-compatibility-auditing-enabled" => Some(("cluster.autopilot.workloadPolicyConfig.autopilotCompatibilityAuditingEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
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
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.blue-green-settings.autoscaled-rollout-policy.wait-for-drain-duration" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.blueGreenSettings.autoscaledRolloutPolicy.waitForDrainDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.blue-green-settings.node-pool-soak-duration" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.blueGreenSettings.nodePoolSoakDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.blue-green-settings.standard-rollout-policy.batch-node-count" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.blueGreenSettings.standardRolloutPolicy.batchNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.blue-green-settings.standard-rollout-policy.batch-percentage" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.blueGreenSettings.standardRolloutPolicy.batchPercentage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.blue-green-settings.standard-rollout-policy.batch-soak-duration" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.blueGreenSettings.standardRolloutPolicy.batchSoakDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.max-surge" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.maxSurge", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.max-unavailable" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.maxUnavailable", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.strategy" => Some(("cluster.autoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.strategy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.autoscaling-profile" => Some(("cluster.autoscaling.autoscalingProfile", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.default-compute-class-config.enabled" => Some(("cluster.autoscaling.defaultComputeClassConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.autoscaling.enable-node-autoprovisioning" => Some(("cluster.autoscaling.enableNodeAutoprovisioning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.binary-authorization.enabled" => Some(("cluster.binaryAuthorization.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.binary-authorization.evaluation-mode" => Some(("cluster.binaryAuthorization.evaluationMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.cluster-ipv4-cidr" => Some(("cluster.clusterIpv4Cidr", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.compliance-posture-config.mode" => Some(("cluster.compliancePostureConfig.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.confidential-nodes.confidential-instance-type" => Some(("cluster.confidentialNodes.confidentialInstanceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.confidential-nodes.enabled" => Some(("cluster.confidentialNodes.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.control-plane-endpoints-config.dns-endpoint-config.allow-external-traffic" => Some(("cluster.controlPlaneEndpointsConfig.dnsEndpointConfig.allowExternalTraffic", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.control-plane-endpoints-config.dns-endpoint-config.enable-k8s-certs-via-dns" => Some(("cluster.controlPlaneEndpointsConfig.dnsEndpointConfig.enableK8sCertsViaDns", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.control-plane-endpoints-config.dns-endpoint-config.enable-k8s-tokens-via-dns" => Some(("cluster.controlPlaneEndpointsConfig.dnsEndpointConfig.enableK8sTokensViaDns", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.control-plane-endpoints-config.dns-endpoint-config.endpoint" => Some(("cluster.controlPlaneEndpointsConfig.dnsEndpointConfig.endpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.control-plane-endpoints-config.ip-endpoints-config.authorized-networks-config.enabled" => Some(("cluster.controlPlaneEndpointsConfig.ipEndpointsConfig.authorizedNetworksConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.control-plane-endpoints-config.ip-endpoints-config.authorized-networks-config.gcp-public-cidrs-access-enabled" => Some(("cluster.controlPlaneEndpointsConfig.ipEndpointsConfig.authorizedNetworksConfig.gcpPublicCidrsAccessEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.control-plane-endpoints-config.ip-endpoints-config.authorized-networks-config.private-endpoint-enforcement-enabled" => Some(("cluster.controlPlaneEndpointsConfig.ipEndpointsConfig.authorizedNetworksConfig.privateEndpointEnforcementEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.control-plane-endpoints-config.ip-endpoints-config.enable-public-endpoint" => Some(("cluster.controlPlaneEndpointsConfig.ipEndpointsConfig.enablePublicEndpoint", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.control-plane-endpoints-config.ip-endpoints-config.enabled" => Some(("cluster.controlPlaneEndpointsConfig.ipEndpointsConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.control-plane-endpoints-config.ip-endpoints-config.global-access" => Some(("cluster.controlPlaneEndpointsConfig.ipEndpointsConfig.globalAccess", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.control-plane-endpoints-config.ip-endpoints-config.private-endpoint" => Some(("cluster.controlPlaneEndpointsConfig.ipEndpointsConfig.privateEndpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.control-plane-endpoints-config.ip-endpoints-config.private-endpoint-subnetwork" => Some(("cluster.controlPlaneEndpointsConfig.ipEndpointsConfig.privateEndpointSubnetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.control-plane-endpoints-config.ip-endpoints-config.public-endpoint" => Some(("cluster.controlPlaneEndpointsConfig.ipEndpointsConfig.publicEndpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                    "cluster.enterprise-config.desired-tier" => Some(("cluster.enterpriseConfig.desiredTier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.etag" => Some(("cluster.etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.expire-time" => Some(("cluster.expireTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.fleet.membership" => Some(("cluster.fleet.membership", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.fleet.membership-type" => Some(("cluster.fleet.membershipType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.fleet.pre-registered" => Some(("cluster.fleet.preRegistered", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.fleet.project" => Some(("cluster.fleet.project", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.gke-auto-upgrade-config.patch-mode" => Some(("cluster.gkeAutoUpgradeConfig.patchMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.id" => Some(("cluster.id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.identity-service-config.enabled" => Some(("cluster.identityServiceConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.initial-cluster-version" => Some(("cluster.initialClusterVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.initial-node-count" => Some(("cluster.initialNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.instance-group-urls" => Some(("cluster.instanceGroupUrls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.ip-allocation-policy.additional-pod-ranges-config.pod-range-names" => Some(("cluster.ipAllocationPolicy.additionalPodRangesConfig.podRangeNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.ip-allocation-policy.auto-ipam-config.enabled" => Some(("cluster.ipAllocationPolicy.autoIpamConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.cluster-ipv4-cidr" => Some(("cluster.ipAllocationPolicy.clusterIpv4Cidr", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.cluster-ipv4-cidr-block" => Some(("cluster.ipAllocationPolicy.clusterIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.cluster-secondary-range-name" => Some(("cluster.ipAllocationPolicy.clusterSecondaryRangeName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.create-subnetwork" => Some(("cluster.ipAllocationPolicy.createSubnetwork", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.default-pod-ipv4-range-utilization" => Some(("cluster.ipAllocationPolicy.defaultPodIpv4RangeUtilization", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.ipv6-access-type" => Some(("cluster.ipAllocationPolicy.ipv6AccessType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.ip-allocation-policy.network-tier-config.network-tier" => Some(("cluster.ipAllocationPolicy.networkTierConfig.networkTier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                    "cluster.maintenance-policy.window.recurring-window.window.maintenance-exclusion-options.end-time-behavior" => Some(("cluster.maintenancePolicy.window.recurringWindow.window.maintenanceExclusionOptions.endTimeBehavior", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.maintenance-policy.window.recurring-window.window.maintenance-exclusion-options.scope" => Some(("cluster.maintenancePolicy.window.recurringWindow.window.maintenanceExclusionOptions.scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.maintenance-policy.window.recurring-window.window.start-time" => Some(("cluster.maintenancePolicy.window.recurringWindow.window.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.managed-opentelemetry-config.scope" => Some(("cluster.managedOpentelemetryConfig.scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.master-auth.client-certificate" => Some(("cluster.masterAuth.clientCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.master-auth.client-certificate-config.issue-client-certificate" => Some(("cluster.masterAuth.clientCertificateConfig.issueClientCertificate", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.master-auth.client-key" => Some(("cluster.masterAuth.clientKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.master-auth.cluster-ca-certificate" => Some(("cluster.masterAuth.clusterCaCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.master-auth.password" => Some(("cluster.masterAuth.password", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.master-auth.username" => Some(("cluster.masterAuth.username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.master-authorized-networks-config.enabled" => Some(("cluster.masterAuthorizedNetworksConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.master-authorized-networks-config.gcp-public-cidrs-access-enabled" => Some(("cluster.masterAuthorizedNetworksConfig.gcpPublicCidrsAccessEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.master-authorized-networks-config.private-endpoint-enforcement-enabled" => Some(("cluster.masterAuthorizedNetworksConfig.privateEndpointEnforcementEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.mesh-certificates.enable-certificates" => Some(("cluster.meshCertificates.enableCertificates", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.monitoring-config.advanced-datapath-observability-config.enable-metrics" => Some(("cluster.monitoringConfig.advancedDatapathObservabilityConfig.enableMetrics", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.monitoring-config.advanced-datapath-observability-config.enable-relay" => Some(("cluster.monitoringConfig.advancedDatapathObservabilityConfig.enableRelay", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.monitoring-config.advanced-datapath-observability-config.relay-mode" => Some(("cluster.monitoringConfig.advancedDatapathObservabilityConfig.relayMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.monitoring-config.component-config.enable-components" => Some(("cluster.monitoringConfig.componentConfig.enableComponents", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.monitoring-config.managed-prometheus-config.auto-monitoring-config.scope" => Some(("cluster.monitoringConfig.managedPrometheusConfig.autoMonitoringConfig.scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.monitoring-config.managed-prometheus-config.enabled" => Some(("cluster.monitoringConfig.managedPrometheusConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.monitoring-service" => Some(("cluster.monitoringService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.name" => Some(("cluster.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.network" => Some(("cluster.network", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.network-config.datapath-provider" => Some(("cluster.networkConfig.datapathProvider", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.network-config.default-enable-private-nodes" => Some(("cluster.networkConfig.defaultEnablePrivateNodes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.network-config.default-snat-status.disabled" => Some(("cluster.networkConfig.defaultSnatStatus.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.network-config.disable-l4-lb-firewall-reconciliation" => Some(("cluster.networkConfig.disableL4LbFirewallReconciliation", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
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
                    "cluster.node-config.advanced-machine-features.performance-monitoring-unit" => Some(("cluster.nodeConfig.advancedMachineFeatures.performanceMonitoringUnit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.advanced-machine-features.threads-per-core" => Some(("cluster.nodeConfig.advancedMachineFeatures.threadsPerCore", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.boot-disk.disk-type" => Some(("cluster.nodeConfig.bootDisk.diskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.boot-disk.provisioned-iops" => Some(("cluster.nodeConfig.bootDisk.provisionedIops", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.boot-disk.provisioned-throughput" => Some(("cluster.nodeConfig.bootDisk.provisionedThroughput", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.boot-disk.size-gb" => Some(("cluster.nodeConfig.bootDisk.sizeGb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.boot-disk-kms-key" => Some(("cluster.nodeConfig.bootDiskKmsKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.confidential-nodes.confidential-instance-type" => Some(("cluster.nodeConfig.confidentialNodes.confidentialInstanceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.confidential-nodes.enabled" => Some(("cluster.nodeConfig.confidentialNodes.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.containerd-config.private-registry-access-config.enabled" => Some(("cluster.nodeConfig.containerdConfig.privateRegistryAccessConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.containerd-config.writable-cgroups.enabled" => Some(("cluster.nodeConfig.containerdConfig.writableCgroups.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.disk-size-gb" => Some(("cluster.nodeConfig.diskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-config.disk-type" => Some(("cluster.nodeConfig.diskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.effective-cgroup-mode" => Some(("cluster.nodeConfig.effectiveCgroupMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.enable-confidential-storage" => Some(("cluster.nodeConfig.enableConfidentialStorage", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.ephemeral-storage-local-ssd-config.data-cache-count" => Some(("cluster.nodeConfig.ephemeralStorageLocalSsdConfig.dataCacheCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-config.ephemeral-storage-local-ssd-config.local-ssd-count" => Some(("cluster.nodeConfig.ephemeralStorageLocalSsdConfig.localSsdCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-config.fast-socket.enabled" => Some(("cluster.nodeConfig.fastSocket.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.flex-start" => Some(("cluster.nodeConfig.flexStart", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.gcfs-config.enabled" => Some(("cluster.nodeConfig.gcfsConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.gvnic.enabled" => Some(("cluster.nodeConfig.gvnic.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.image-type" => Some(("cluster.nodeConfig.imageType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.allowed-unsafe-sysctls" => Some(("cluster.nodeConfig.kubeletConfig.allowedUnsafeSysctls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.node-config.kubelet-config.container-log-max-files" => Some(("cluster.nodeConfig.kubeletConfig.containerLogMaxFiles", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.container-log-max-size" => Some(("cluster.nodeConfig.kubeletConfig.containerLogMaxSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.cpu-cfs-quota" => Some(("cluster.nodeConfig.kubeletConfig.cpuCfsQuota", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.cpu-cfs-quota-period" => Some(("cluster.nodeConfig.kubeletConfig.cpuCfsQuotaPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.cpu-manager-policy" => Some(("cluster.nodeConfig.kubeletConfig.cpuManagerPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.eviction-max-pod-grace-period-seconds" => Some(("cluster.nodeConfig.kubeletConfig.evictionMaxPodGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.eviction-minimum-reclaim.imagefs-available" => Some(("cluster.nodeConfig.kubeletConfig.evictionMinimumReclaim.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.eviction-minimum-reclaim.imagefs-inodes-free" => Some(("cluster.nodeConfig.kubeletConfig.evictionMinimumReclaim.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.eviction-minimum-reclaim.memory-available" => Some(("cluster.nodeConfig.kubeletConfig.evictionMinimumReclaim.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.eviction-minimum-reclaim.nodefs-available" => Some(("cluster.nodeConfig.kubeletConfig.evictionMinimumReclaim.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.eviction-minimum-reclaim.nodefs-inodes-free" => Some(("cluster.nodeConfig.kubeletConfig.evictionMinimumReclaim.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.eviction-minimum-reclaim.pid-available" => Some(("cluster.nodeConfig.kubeletConfig.evictionMinimumReclaim.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.eviction-soft.imagefs-available" => Some(("cluster.nodeConfig.kubeletConfig.evictionSoft.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.eviction-soft.imagefs-inodes-free" => Some(("cluster.nodeConfig.kubeletConfig.evictionSoft.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.eviction-soft.memory-available" => Some(("cluster.nodeConfig.kubeletConfig.evictionSoft.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.eviction-soft.nodefs-available" => Some(("cluster.nodeConfig.kubeletConfig.evictionSoft.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.eviction-soft.nodefs-inodes-free" => Some(("cluster.nodeConfig.kubeletConfig.evictionSoft.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.eviction-soft.pid-available" => Some(("cluster.nodeConfig.kubeletConfig.evictionSoft.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.eviction-soft-grace-period.imagefs-available" => Some(("cluster.nodeConfig.kubeletConfig.evictionSoftGracePeriod.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.eviction-soft-grace-period.imagefs-inodes-free" => Some(("cluster.nodeConfig.kubeletConfig.evictionSoftGracePeriod.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.eviction-soft-grace-period.memory-available" => Some(("cluster.nodeConfig.kubeletConfig.evictionSoftGracePeriod.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.eviction-soft-grace-period.nodefs-available" => Some(("cluster.nodeConfig.kubeletConfig.evictionSoftGracePeriod.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.eviction-soft-grace-period.nodefs-inodes-free" => Some(("cluster.nodeConfig.kubeletConfig.evictionSoftGracePeriod.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.eviction-soft-grace-period.pid-available" => Some(("cluster.nodeConfig.kubeletConfig.evictionSoftGracePeriod.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.image-gc-high-threshold-percent" => Some(("cluster.nodeConfig.kubeletConfig.imageGcHighThresholdPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.image-gc-low-threshold-percent" => Some(("cluster.nodeConfig.kubeletConfig.imageGcLowThresholdPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.image-maximum-gc-age" => Some(("cluster.nodeConfig.kubeletConfig.imageMaximumGcAge", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.image-minimum-gc-age" => Some(("cluster.nodeConfig.kubeletConfig.imageMinimumGcAge", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.insecure-kubelet-readonly-port-enabled" => Some(("cluster.nodeConfig.kubeletConfig.insecureKubeletReadonlyPortEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.max-parallel-image-pulls" => Some(("cluster.nodeConfig.kubeletConfig.maxParallelImagePulls", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.memory-manager.policy" => Some(("cluster.nodeConfig.kubeletConfig.memoryManager.policy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.pod-pids-limit" => Some(("cluster.nodeConfig.kubeletConfig.podPidsLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.single-process-oom-kill" => Some(("cluster.nodeConfig.kubeletConfig.singleProcessOomKill", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.topology-manager.policy" => Some(("cluster.nodeConfig.kubeletConfig.topologyManager.policy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.kubelet-config.topology-manager.scope" => Some(("cluster.nodeConfig.kubeletConfig.topologyManager.scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.labels" => Some(("cluster.nodeConfig.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "cluster.node-config.linux-node-config.cgroup-mode" => Some(("cluster.nodeConfig.linuxNodeConfig.cgroupMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.linux-node-config.hugepages.hugepage-size1g" => Some(("cluster.nodeConfig.linuxNodeConfig.hugepages.hugepageSize1g", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-config.linux-node-config.hugepages.hugepage-size2m" => Some(("cluster.nodeConfig.linuxNodeConfig.hugepages.hugepageSize2m", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-config.linux-node-config.node-kernel-module-loading.policy" => Some(("cluster.nodeConfig.linuxNodeConfig.nodeKernelModuleLoading.policy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.linux-node-config.sysctls" => Some(("cluster.nodeConfig.linuxNodeConfig.sysctls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "cluster.node-config.linux-node-config.transparent-hugepage-defrag" => Some(("cluster.nodeConfig.linuxNodeConfig.transparentHugepageDefrag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.linux-node-config.transparent-hugepage-enabled" => Some(("cluster.nodeConfig.linuxNodeConfig.transparentHugepageEnabled", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.local-nvme-ssd-block-config.local-ssd-count" => Some(("cluster.nodeConfig.localNvmeSsdBlockConfig.localSsdCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-config.local-ssd-count" => Some(("cluster.nodeConfig.localSsdCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-config.local-ssd-encryption-mode" => Some(("cluster.nodeConfig.localSsdEncryptionMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.logging-config.variant-config.variant" => Some(("cluster.nodeConfig.loggingConfig.variantConfig.variant", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.machine-type" => Some(("cluster.nodeConfig.machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.max-run-duration" => Some(("cluster.nodeConfig.maxRunDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                    "cluster.node-config.sole-tenant-config.min-node-cpus" => Some(("cluster.nodeConfig.soleTenantConfig.minNodeCpus", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-config.spot" => Some(("cluster.nodeConfig.spot", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-config.storage-pools" => Some(("cluster.nodeConfig.storagePools", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.node-config.tags" => Some(("cluster.nodeConfig.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.node-config.windows-node-config.os-version" => Some(("cluster.nodeConfig.windowsNodeConfig.osVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-config.workload-metadata-config.mode" => Some(("cluster.nodeConfig.workloadMetadataConfig.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-ipv4-cidr-size" => Some(("cluster.nodeIpv4CidrSize", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.linux-node-config.cgroup-mode" => Some(("cluster.nodePoolAutoConfig.linuxNodeConfig.cgroupMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.linux-node-config.hugepages.hugepage-size1g" => Some(("cluster.nodePoolAutoConfig.linuxNodeConfig.hugepages.hugepageSize1g", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.linux-node-config.hugepages.hugepage-size2m" => Some(("cluster.nodePoolAutoConfig.linuxNodeConfig.hugepages.hugepageSize2m", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.linux-node-config.node-kernel-module-loading.policy" => Some(("cluster.nodePoolAutoConfig.linuxNodeConfig.nodeKernelModuleLoading.policy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.linux-node-config.sysctls" => Some(("cluster.nodePoolAutoConfig.linuxNodeConfig.sysctls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "cluster.node-pool-auto-config.linux-node-config.transparent-hugepage-defrag" => Some(("cluster.nodePoolAutoConfig.linuxNodeConfig.transparentHugepageDefrag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.linux-node-config.transparent-hugepage-enabled" => Some(("cluster.nodePoolAutoConfig.linuxNodeConfig.transparentHugepageEnabled", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.network-tags.tags" => Some(("cluster.nodePoolAutoConfig.networkTags.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.node-pool-auto-config.node-kubelet-config.allowed-unsafe-sysctls" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.allowedUnsafeSysctls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.node-pool-auto-config.node-kubelet-config.container-log-max-files" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.containerLogMaxFiles", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.container-log-max-size" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.containerLogMaxSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.cpu-cfs-quota" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.cpuCfsQuota", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.cpu-cfs-quota-period" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.cpuCfsQuotaPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.cpu-manager-policy" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.cpuManagerPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.eviction-max-pod-grace-period-seconds" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.evictionMaxPodGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.eviction-minimum-reclaim.imagefs-available" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.evictionMinimumReclaim.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.eviction-minimum-reclaim.imagefs-inodes-free" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.evictionMinimumReclaim.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.eviction-minimum-reclaim.memory-available" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.evictionMinimumReclaim.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.eviction-minimum-reclaim.nodefs-available" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.evictionMinimumReclaim.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.eviction-minimum-reclaim.nodefs-inodes-free" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.evictionMinimumReclaim.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.eviction-minimum-reclaim.pid-available" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.evictionMinimumReclaim.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.eviction-soft.imagefs-available" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.evictionSoft.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.eviction-soft.imagefs-inodes-free" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.evictionSoft.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.eviction-soft.memory-available" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.evictionSoft.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.eviction-soft.nodefs-available" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.evictionSoft.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.eviction-soft.nodefs-inodes-free" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.evictionSoft.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.eviction-soft.pid-available" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.evictionSoft.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.eviction-soft-grace-period.imagefs-available" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.evictionSoftGracePeriod.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.eviction-soft-grace-period.imagefs-inodes-free" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.evictionSoftGracePeriod.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.eviction-soft-grace-period.memory-available" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.evictionSoftGracePeriod.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.eviction-soft-grace-period.nodefs-available" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.evictionSoftGracePeriod.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.eviction-soft-grace-period.nodefs-inodes-free" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.evictionSoftGracePeriod.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.eviction-soft-grace-period.pid-available" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.evictionSoftGracePeriod.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.image-gc-high-threshold-percent" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.imageGcHighThresholdPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.image-gc-low-threshold-percent" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.imageGcLowThresholdPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.image-maximum-gc-age" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.imageMaximumGcAge", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.image-minimum-gc-age" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.imageMinimumGcAge", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.insecure-kubelet-readonly-port-enabled" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.insecureKubeletReadonlyPortEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.max-parallel-image-pulls" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.maxParallelImagePulls", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.memory-manager.policy" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.memoryManager.policy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.pod-pids-limit" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.podPidsLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.single-process-oom-kill" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.singleProcessOomKill", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.topology-manager.policy" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.topologyManager.policy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.node-kubelet-config.topology-manager.scope" => Some(("cluster.nodePoolAutoConfig.nodeKubeletConfig.topologyManager.scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-auto-config.resource-manager-tags.tags" => Some(("cluster.nodePoolAutoConfig.resourceManagerTags.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "cluster.node-pool-defaults.node-config-defaults.containerd-config.private-registry-access-config.enabled" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.containerdConfig.privateRegistryAccessConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.containerd-config.writable-cgroups.enabled" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.containerdConfig.writableCgroups.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.gcfs-config.enabled" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.gcfsConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.logging-config.variant-config.variant" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.loggingConfig.variantConfig.variant", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.allowed-unsafe-sysctls" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.allowedUnsafeSysctls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.container-log-max-files" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.containerLogMaxFiles", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.container-log-max-size" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.containerLogMaxSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.cpu-cfs-quota" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.cpuCfsQuota", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.cpu-cfs-quota-period" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.cpuCfsQuotaPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.cpu-manager-policy" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.cpuManagerPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.eviction-max-pod-grace-period-seconds" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.evictionMaxPodGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.eviction-minimum-reclaim.imagefs-available" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.evictionMinimumReclaim.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.eviction-minimum-reclaim.imagefs-inodes-free" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.evictionMinimumReclaim.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.eviction-minimum-reclaim.memory-available" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.evictionMinimumReclaim.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.eviction-minimum-reclaim.nodefs-available" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.evictionMinimumReclaim.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.eviction-minimum-reclaim.nodefs-inodes-free" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.evictionMinimumReclaim.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.eviction-minimum-reclaim.pid-available" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.evictionMinimumReclaim.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.eviction-soft.imagefs-available" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.evictionSoft.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.eviction-soft.imagefs-inodes-free" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.evictionSoft.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.eviction-soft.memory-available" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.evictionSoft.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.eviction-soft.nodefs-available" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.evictionSoft.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.eviction-soft.nodefs-inodes-free" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.evictionSoft.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.eviction-soft.pid-available" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.evictionSoft.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.eviction-soft-grace-period.imagefs-available" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.evictionSoftGracePeriod.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.eviction-soft-grace-period.imagefs-inodes-free" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.evictionSoftGracePeriod.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.eviction-soft-grace-period.memory-available" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.evictionSoftGracePeriod.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.eviction-soft-grace-period.nodefs-available" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.evictionSoftGracePeriod.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.eviction-soft-grace-period.nodefs-inodes-free" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.evictionSoftGracePeriod.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.eviction-soft-grace-period.pid-available" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.evictionSoftGracePeriod.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.image-gc-high-threshold-percent" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.imageGcHighThresholdPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.image-gc-low-threshold-percent" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.imageGcLowThresholdPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.image-maximum-gc-age" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.imageMaximumGcAge", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.image-minimum-gc-age" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.imageMinimumGcAge", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.insecure-kubelet-readonly-port-enabled" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.insecureKubeletReadonlyPortEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.max-parallel-image-pulls" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.maxParallelImagePulls", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.memory-manager.policy" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.memoryManager.policy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.pod-pids-limit" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.podPidsLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.single-process-oom-kill" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.singleProcessOomKill", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.topology-manager.policy" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.topologyManager.policy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.node-pool-defaults.node-config-defaults.node-kubelet-config.topology-manager.scope" => Some(("cluster.nodePoolDefaults.nodeConfigDefaults.nodeKubeletConfig.topologyManager.scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.notification-config.pubsub.enabled" => Some(("cluster.notificationConfig.pubsub.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.notification-config.pubsub.filter.event-type" => Some(("cluster.notificationConfig.pubsub.filter.eventType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.notification-config.pubsub.topic" => Some(("cluster.notificationConfig.pubsub.topic", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.parent-product-config.labels" => Some(("cluster.parentProductConfig.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "cluster.parent-product-config.product-name" => Some(("cluster.parentProductConfig.productName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.pod-autoscaling.hpa-profile" => Some(("cluster.podAutoscaling.hpaProfile", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.enable-private-endpoint" => Some(("cluster.privateClusterConfig.enablePrivateEndpoint", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.enable-private-nodes" => Some(("cluster.privateClusterConfig.enablePrivateNodes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.master-global-access-config.enabled" => Some(("cluster.privateClusterConfig.masterGlobalAccessConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.master-ipv4-cidr-block" => Some(("cluster.privateClusterConfig.masterIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.peering-name" => Some(("cluster.privateClusterConfig.peeringName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.private-endpoint" => Some(("cluster.privateClusterConfig.privateEndpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.private-endpoint-subnetwork" => Some(("cluster.privateClusterConfig.privateEndpointSubnetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.private-cluster-config.public-endpoint" => Some(("cluster.privateClusterConfig.publicEndpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.rbac-binding-config.enable-insecure-binding-system-authenticated" => Some(("cluster.rbacBindingConfig.enableInsecureBindingSystemAuthenticated", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.rbac-binding-config.enable-insecure-binding-system-unauthenticated" => Some(("cluster.rbacBindingConfig.enableInsecureBindingSystemUnauthenticated", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.release-channel.channel" => Some(("cluster.releaseChannel.channel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.resource-labels" => Some(("cluster.resourceLabels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "cluster.resource-usage-export-config.bigquery-destination.dataset-id" => Some(("cluster.resourceUsageExportConfig.bigqueryDestination.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.resource-usage-export-config.consumption-metering-config.enabled" => Some(("cluster.resourceUsageExportConfig.consumptionMeteringConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.resource-usage-export-config.enable-network-egress-metering" => Some(("cluster.resourceUsageExportConfig.enableNetworkEgressMetering", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.satisfies-pzi" => Some(("cluster.satisfiesPzi", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.satisfies-pzs" => Some(("cluster.satisfiesPzs", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.secret-manager-config.enabled" => Some(("cluster.secretManagerConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.secret-manager-config.rotation-config.enabled" => Some(("cluster.secretManagerConfig.rotationConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.secret-manager-config.rotation-config.rotation-interval" => Some(("cluster.secretManagerConfig.rotationConfig.rotationInterval", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.security-posture-config.mode" => Some(("cluster.securityPostureConfig.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.security-posture-config.vulnerability-mode" => Some(("cluster.securityPostureConfig.vulnerabilityMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.self-link" => Some(("cluster.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.services-ipv4-cidr" => Some(("cluster.servicesIpv4Cidr", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.shielded-nodes.enabled" => Some(("cluster.shieldedNodes.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.status" => Some(("cluster.status", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.status-message" => Some(("cluster.statusMessage", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.subnetwork" => Some(("cluster.subnetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.tpu-ipv4-cidr-block" => Some(("cluster.tpuIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.user-managed-keys-config.aggregation-ca" => Some(("cluster.userManagedKeysConfig.aggregationCa", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.user-managed-keys-config.cluster-ca" => Some(("cluster.userManagedKeysConfig.clusterCa", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.user-managed-keys-config.control-plane-disk-encryption-key" => Some(("cluster.userManagedKeysConfig.controlPlaneDiskEncryptionKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.user-managed-keys-config.control-plane-disk-encryption-key-versions" => Some(("cluster.userManagedKeysConfig.controlPlaneDiskEncryptionKeyVersions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.user-managed-keys-config.etcd-api-ca" => Some(("cluster.userManagedKeysConfig.etcdApiCa", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.user-managed-keys-config.etcd-peer-ca" => Some(("cluster.userManagedKeysConfig.etcdPeerCa", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.user-managed-keys-config.gkeops-etcd-backup-encryption-key" => Some(("cluster.userManagedKeysConfig.gkeopsEtcdBackupEncryptionKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.user-managed-keys-config.service-account-signing-keys" => Some(("cluster.userManagedKeysConfig.serviceAccountSigningKeys", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.user-managed-keys-config.service-account-verification-keys" => Some(("cluster.userManagedKeysConfig.serviceAccountVerificationKeys", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "cluster.vertical-pod-autoscaling.enabled" => Some(("cluster.verticalPodAutoscaling.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "cluster.workload-identity-config.workload-pool" => Some(("cluster.workloadIdentityConfig.workloadPool", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster.zone" => Some(("cluster.zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parent" => Some(("parent", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["additional-pod-ranges-config", "additive-vpc-scope-dns-domain", "addons-config", "advanced-datapath-observability-config", "advanced-machine-features", "aggregation-ca", "allow-external-traffic", "allow-net-admin", "allowed-unsafe-sysctls", "allowlist-paths", "alpha-cluster-feature-gates", "anonymous-authentication-config", "authenticator-groups-config", "authorized-networks-config", "auto-ipam-config", "auto-monitoring-config", "auto-repair", "auto-upgrade", "auto-upgrade-start-time", "autopilot", "autopilot-compatibility-auditing-enabled", "autoprovisioning-locations", "autoprovisioning-node-pool-defaults", "autoscaled-rollout-policy", "autoscaling", "autoscaling-profile", "batch-node-count", "batch-percentage", "batch-soak-duration", "bigquery-destination", "binary-authorization", "blue-green-settings", "boot-disk", "boot-disk-kms-key", "cgroup-mode", "channel", "client-certificate", "client-certificate-config", "client-key", "cloud-run-config", "cluster", "cluster-ca", "cluster-ca-certificate", "cluster-dns", "cluster-dns-domain", "cluster-dns-scope", "cluster-ipv4-cidr", "cluster-ipv4-cidr-block", "cluster-secondary-range-name", "cluster-tier", "compliance-posture-config", "component-config", "confidential-instance-type", "confidential-nodes", "config-connector-config", "consume-reservation-type", "consumption-metering-config", "container-log-max-files", "container-log-max-size", "containerd-config", "control-plane-disk-encryption-key", "control-plane-disk-encryption-key-versions", "control-plane-endpoints-config", "cost-management-config", "cpu-cfs-quota", "cpu-cfs-quota-period", "cpu-manager-policy", "create-subnetwork", "create-time", "current-master-version", "current-node-count", "current-node-version", "current-state", "daily-maintenance-window", "data-cache-count", "database-encryption", "datapath-provider", "dataset-id", "decryption-keys", "default-compute-class-config", "default-enable-private-nodes", "default-max-pods-constraint", "default-pod-ipv4-range-utilization", "default-snat-status", "description", "desired-tier", "disable", "disable-l4-lb-firewall-reconciliation", "disabled", "disk-size-gb", "disk-type", "dns-cache-config", "dns-config", "dns-endpoint-config", "duration", "effective-cgroup-mode", "enable-certificates", "enable-cilium-clusterwide-network-policy", "enable-components", "enable-confidential-storage", "enable-fqdn-network-policy", "enable-insecure-binding-system-authenticated", "enable-insecure-binding-system-unauthenticated", "enable-integrity-monitoring", "enable-intra-node-visibility", "enable-k8s-beta-apis", "enable-k8s-certs-via-dns", "enable-k8s-tokens-via-dns", "enable-kubernetes-alpha", "enable-l4ilb-subsetting", "enable-legacy-lustre-port", "enable-metrics", "enable-multi-networking", "enable-nested-virtualization", "enable-network-egress-metering", "enable-node-autoprovisioning", "enable-private-endpoint", "enable-private-nodes", "enable-public-endpoint", "enable-relay", "enable-secure-boot", "enable-tpu", "enabled", "enabled-apis", "end-time", "end-time-behavior", "endpoint", "enterprise-config", "ephemeral-storage-local-ssd-config", "etag", "etcd-api-ca", "etcd-peer-ca", "evaluation-mode", "event-type", "eviction-max-pod-grace-period-seconds", "eviction-minimum-reclaim", "eviction-soft", "eviction-soft-grace-period", "expire-time", "fast-socket", "filter", "fleet", "flex-start", "gateway-api-config", "gce-persistent-disk-csi-driver-config", "gcfs-config", "gcp-filestore-csi-driver-config", "gcp-public-cidrs-access-enabled", "gcs-fuse-csi-driver-config", "gke-auto-upgrade-config", "gke-backup-agent-config", "gkeops-etcd-backup-encryption-key", "global-access", "gvnic", "high-scale-checkpointing-config", "horizontal-pod-autoscaling", "hpa-profile", "http-load-balancing", "hugepage-size1g", "hugepage-size2m", "hugepages", "id", "identity-service-config", "image-gc-high-threshold-percent", "image-gc-low-threshold-percent", "image-maximum-gc-age", "image-minimum-gc-age", "image-type", "imagefs-available", "imagefs-inodes-free", "in-transit-encryption-config", "initial-cluster-version", "initial-node-count", "insecure-kubelet-readonly-port-enabled", "instance-group-urls", "ip-allocation-policy", "ip-endpoints-config", "ipv6-access-type", "issue-client-certificate", "key", "key-name", "kubelet-config", "kubernetes-dashboard", "label-fingerprint", "labels", "legacy-abac", "linux-node-config", "load-balancer-type", "local-nvme-ssd-block-config", "local-ssd-count", "local-ssd-encryption-mode", "location", "locations", "logging-config", "logging-service", "lustre-csi-driver-config", "machine-type", "maintenance-exclusion-options", "maintenance-policy", "managed-opentelemetry-config", "managed-prometheus-config", "management", "master-auth", "master-authorized-networks-config", "master-global-access-config", "master-ipv4-cidr-block", "max-parallel-image-pulls", "max-pods-per-node", "max-run-duration", "max-surge", "max-unavailable", "membership", "membership-type", "memory-available", "memory-manager", "mesh-certificates", "metadata", "min-cpu-platform", "min-node-cpus", "mode", "monitoring-config", "monitoring-service", "name", "network", "network-config", "network-performance-config", "network-policy", "network-policy-config", "network-tags", "network-tier", "network-tier-config", "node-config", "node-config-defaults", "node-group", "node-ipv4-cidr", "node-ipv4-cidr-block", "node-ipv4-cidr-size", "node-kernel-module-loading", "node-kubelet-config", "node-pool-auto-config", "node-pool-defaults", "node-pool-soak-duration", "nodefs-available", "nodefs-inodes-free", "notification-config", "oauth-scopes", "os-version", "parallelstore-csi-driver-config", "parent", "parent-product-config", "password", "patch-mode", "peering-name", "performance-monitoring-unit", "pid-available", "pod-autoscaling", "pod-cidr-overprovision-config", "pod-pids-limit", "pod-range-names", "policy", "pre-registered", "preemptible", "private-cluster-config", "private-endpoint", "private-endpoint-enforcement-enabled", "private-endpoint-subnetwork", "private-ipv6-google-access", "private-registry-access-config", "privileged-admission-config", "product-name", "project", "project-id", "provider", "provisioned-iops", "provisioned-throughput", "public-endpoint", "pubsub", "ray-cluster-logging-config", "ray-cluster-monitoring-config", "ray-operator-config", "rbac-binding-config", "recurrence", "recurring-window", "relay-mode", "release-channel", "reservation-affinity", "resource-labels", "resource-manager-tags", "resource-usage-export-config", "resource-version", "rotation-config", "rotation-interval", "sandbox-config", "satisfies-pzi", "satisfies-pzs", "scope", "secret-manager-config", "security-group", "security-posture-config", "self-link", "service-account", "service-account-signing-keys", "service-account-verification-keys", "service-external-ips-config", "services-ipv4-cidr", "services-ipv4-cidr-block", "services-ipv6-cidr-block", "services-secondary-range-name", "shielded-instance-config", "shielded-nodes", "single-process-oom-kill", "size-gb", "sole-tenant-config", "spot", "stack-type", "standard-rollout-policy", "start-time", "state", "stateful-ha-config", "status", "status-message", "storage-pools", "strategy", "subnet-ipv6-cidr-block", "subnetwork", "subnetwork-name", "sysctls", "tags", "threads-per-core", "topic", "topology-manager", "total-egress-bandwidth-tier", "tpu-ipv4-cidr-block", "transparent-hugepage-defrag", "transparent-hugepage-enabled", "type", "upgrade-options", "upgrade-settings", "use-ip-aliases", "use-routes", "user-managed-keys-config", "username", "values", "variant", "variant-config", "vertical-pod-autoscaling", "vulnerability-mode", "wait-for-drain-duration", "window", "windows-node-config", "workload-identity-config", "workload-metadata-config", "workload-policy-config", "workload-pool", "writable-cgroups", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::CreateClusterRequest = serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_clusters_create(
            request,
            opt.value_of("project-id").unwrap_or(""),
            opt.value_of("zone").unwrap_or(""),
        );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_zones_clusters_delete(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.projects().zones_clusters_delete(
            opt.value_of("project-id").unwrap_or(""),
            opt.value_of("zone").unwrap_or(""),
            opt.value_of("cluster-id").unwrap_or(""),
        );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "name" => {
                    call = call.name(value.unwrap_or(""));
                }
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v.extend(["name"].iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_zones_clusters_fetch_cluster_upgrade_info(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .projects()
            .zones_clusters_fetch_cluster_upgrade_info(opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "version" => {
                    call = call.version(value.unwrap_or(""));
                }
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v.extend(["version"].iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_zones_clusters_get(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.projects().zones_clusters_get(
            opt.value_of("project-id").unwrap_or(""),
            opt.value_of("zone").unwrap_or(""),
            opt.value_of("cluster-id").unwrap_or(""),
        );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "name" => {
                    call = call.name(value.unwrap_or(""));
                }
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v.extend(["name"].iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_zones_clusters_legacy_abac(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "cluster-id" => Some((
                    "clusterId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "enabled" => Some((
                    "enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "name" => Some((
                    "name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "project-id" => Some((
                    "projectId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "zone" => Some((
                    "zone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec!["cluster-id", "enabled", "name", "project-id", "zone"],
                    );
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::SetLegacyAbacRequest = serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_clusters_legacy_abac(
            request,
            opt.value_of("project-id").unwrap_or(""),
            opt.value_of("zone").unwrap_or(""),
            opt.value_of("cluster-id").unwrap_or(""),
        );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_zones_clusters_list(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.projects().zones_clusters_list(
            opt.value_of("project-id").unwrap_or(""),
            opt.value_of("zone").unwrap_or(""),
        );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "parent" => {
                    call = call.parent(value.unwrap_or(""));
                }
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v.extend(["parent"].iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_zones_clusters_locations(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "cluster-id" => Some((
                    "clusterId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "locations" => Some((
                    "locations",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Vec,
                    },
                )),
                "name" => Some((
                    "name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "project-id" => Some((
                    "projectId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "zone" => Some((
                    "zone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec!["cluster-id", "locations", "name", "project-id", "zone"],
                    );
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::SetLocationsRequest = serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_clusters_locations(
            request,
            opt.value_of("project-id").unwrap_or(""),
            opt.value_of("zone").unwrap_or(""),
            opt.value_of("cluster-id").unwrap_or(""),
        );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_zones_clusters_logging(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "cluster-id" => Some((
                    "clusterId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "logging-service" => Some((
                    "loggingService",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "name" => Some((
                    "name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "project-id" => Some((
                    "projectId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "zone" => Some((
                    "zone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "cluster-id",
                            "logging-service",
                            "name",
                            "project-id",
                            "zone",
                        ],
                    );
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::SetLoggingServiceRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_clusters_logging(
            request,
            opt.value_of("project-id").unwrap_or(""),
            opt.value_of("zone").unwrap_or(""),
            opt.value_of("cluster-id").unwrap_or(""),
        );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_zones_clusters_master(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "cluster-id" => Some((
                    "clusterId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "master-version" => Some((
                    "masterVersion",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "name" => Some((
                    "name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "project-id" => Some((
                    "projectId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "zone" => Some((
                    "zone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec!["cluster-id", "master-version", "name", "project-id", "zone"],
                    );
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::UpdateMasterRequest = serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_clusters_master(
            request,
            opt.value_of("project-id").unwrap_or(""),
            opt.value_of("zone").unwrap_or(""),
            opt.value_of("cluster-id").unwrap_or(""),
        );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_zones_clusters_monitoring(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "cluster-id" => Some((
                    "clusterId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "monitoring-service" => Some((
                    "monitoringService",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "name" => Some((
                    "name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "project-id" => Some((
                    "projectId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "zone" => Some((
                    "zone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "cluster-id",
                            "monitoring-service",
                            "name",
                            "project-id",
                            "zone",
                        ],
                    );
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::SetMonitoringServiceRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_clusters_monitoring(
            request,
            opt.value_of("project-id").unwrap_or(""),
            opt.value_of("zone").unwrap_or(""),
            opt.value_of("cluster-id").unwrap_or(""),
        );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_zones_clusters_node_pools_autoscaling(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "autoscaling.autoprovisioned" => Some((
                    "autoscaling.autoprovisioned",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "autoscaling.enabled" => Some((
                    "autoscaling.enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "autoscaling.location-policy" => Some((
                    "autoscaling.locationPolicy",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "autoscaling.max-node-count" => Some((
                    "autoscaling.maxNodeCount",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "autoscaling.min-node-count" => Some((
                    "autoscaling.minNodeCount",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "autoscaling.total-max-node-count" => Some((
                    "autoscaling.totalMaxNodeCount",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "autoscaling.total-min-node-count" => Some((
                    "autoscaling.totalMinNodeCount",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "cluster-id" => Some((
                    "clusterId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "name" => Some((
                    "name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "node-pool-id" => Some((
                    "nodePoolId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "project-id" => Some((
                    "projectId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "zone" => Some((
                    "zone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "autoprovisioned",
                            "autoscaling",
                            "cluster-id",
                            "enabled",
                            "location-policy",
                            "max-node-count",
                            "min-node-count",
                            "name",
                            "node-pool-id",
                            "project-id",
                            "total-max-node-count",
                            "total-min-node-count",
                            "zone",
                        ],
                    );
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::SetNodePoolAutoscalingRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_clusters_node_pools_autoscaling(
            request,
            opt.value_of("project-id").unwrap_or(""),
            opt.value_of("zone").unwrap_or(""),
            opt.value_of("cluster-id").unwrap_or(""),
            opt.value_of("node-pool-id").unwrap_or(""),
        );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_zones_clusters_node_pools_create(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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
                    "node-pool.autopilot-config.enabled" => Some(("nodePool.autopilotConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
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
                    "node-pool.config.advanced-machine-features.performance-monitoring-unit" => Some(("nodePool.config.advancedMachineFeatures.performanceMonitoringUnit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.advanced-machine-features.threads-per-core" => Some(("nodePool.config.advancedMachineFeatures.threadsPerCore", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.boot-disk.disk-type" => Some(("nodePool.config.bootDisk.diskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.boot-disk.provisioned-iops" => Some(("nodePool.config.bootDisk.provisionedIops", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.boot-disk.provisioned-throughput" => Some(("nodePool.config.bootDisk.provisionedThroughput", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.boot-disk.size-gb" => Some(("nodePool.config.bootDisk.sizeGb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.boot-disk-kms-key" => Some(("nodePool.config.bootDiskKmsKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.confidential-nodes.confidential-instance-type" => Some(("nodePool.config.confidentialNodes.confidentialInstanceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.confidential-nodes.enabled" => Some(("nodePool.config.confidentialNodes.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.containerd-config.private-registry-access-config.enabled" => Some(("nodePool.config.containerdConfig.privateRegistryAccessConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.containerd-config.writable-cgroups.enabled" => Some(("nodePool.config.containerdConfig.writableCgroups.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.disk-size-gb" => Some(("nodePool.config.diskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.disk-type" => Some(("nodePool.config.diskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.effective-cgroup-mode" => Some(("nodePool.config.effectiveCgroupMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.enable-confidential-storage" => Some(("nodePool.config.enableConfidentialStorage", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.ephemeral-storage-local-ssd-config.data-cache-count" => Some(("nodePool.config.ephemeralStorageLocalSsdConfig.dataCacheCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.ephemeral-storage-local-ssd-config.local-ssd-count" => Some(("nodePool.config.ephemeralStorageLocalSsdConfig.localSsdCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.fast-socket.enabled" => Some(("nodePool.config.fastSocket.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.flex-start" => Some(("nodePool.config.flexStart", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.gcfs-config.enabled" => Some(("nodePool.config.gcfsConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.gvnic.enabled" => Some(("nodePool.config.gvnic.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.image-type" => Some(("nodePool.config.imageType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.allowed-unsafe-sysctls" => Some(("nodePool.config.kubeletConfig.allowedUnsafeSysctls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "node-pool.config.kubelet-config.container-log-max-files" => Some(("nodePool.config.kubeletConfig.containerLogMaxFiles", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.container-log-max-size" => Some(("nodePool.config.kubeletConfig.containerLogMaxSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.cpu-cfs-quota" => Some(("nodePool.config.kubeletConfig.cpuCfsQuota", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.cpu-cfs-quota-period" => Some(("nodePool.config.kubeletConfig.cpuCfsQuotaPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.cpu-manager-policy" => Some(("nodePool.config.kubeletConfig.cpuManagerPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.eviction-max-pod-grace-period-seconds" => Some(("nodePool.config.kubeletConfig.evictionMaxPodGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.eviction-minimum-reclaim.imagefs-available" => Some(("nodePool.config.kubeletConfig.evictionMinimumReclaim.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.eviction-minimum-reclaim.imagefs-inodes-free" => Some(("nodePool.config.kubeletConfig.evictionMinimumReclaim.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.eviction-minimum-reclaim.memory-available" => Some(("nodePool.config.kubeletConfig.evictionMinimumReclaim.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.eviction-minimum-reclaim.nodefs-available" => Some(("nodePool.config.kubeletConfig.evictionMinimumReclaim.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.eviction-minimum-reclaim.nodefs-inodes-free" => Some(("nodePool.config.kubeletConfig.evictionMinimumReclaim.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.eviction-minimum-reclaim.pid-available" => Some(("nodePool.config.kubeletConfig.evictionMinimumReclaim.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.eviction-soft.imagefs-available" => Some(("nodePool.config.kubeletConfig.evictionSoft.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.eviction-soft.imagefs-inodes-free" => Some(("nodePool.config.kubeletConfig.evictionSoft.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.eviction-soft.memory-available" => Some(("nodePool.config.kubeletConfig.evictionSoft.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.eviction-soft.nodefs-available" => Some(("nodePool.config.kubeletConfig.evictionSoft.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.eviction-soft.nodefs-inodes-free" => Some(("nodePool.config.kubeletConfig.evictionSoft.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.eviction-soft.pid-available" => Some(("nodePool.config.kubeletConfig.evictionSoft.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.eviction-soft-grace-period.imagefs-available" => Some(("nodePool.config.kubeletConfig.evictionSoftGracePeriod.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.eviction-soft-grace-period.imagefs-inodes-free" => Some(("nodePool.config.kubeletConfig.evictionSoftGracePeriod.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.eviction-soft-grace-period.memory-available" => Some(("nodePool.config.kubeletConfig.evictionSoftGracePeriod.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.eviction-soft-grace-period.nodefs-available" => Some(("nodePool.config.kubeletConfig.evictionSoftGracePeriod.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.eviction-soft-grace-period.nodefs-inodes-free" => Some(("nodePool.config.kubeletConfig.evictionSoftGracePeriod.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.eviction-soft-grace-period.pid-available" => Some(("nodePool.config.kubeletConfig.evictionSoftGracePeriod.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.image-gc-high-threshold-percent" => Some(("nodePool.config.kubeletConfig.imageGcHighThresholdPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.image-gc-low-threshold-percent" => Some(("nodePool.config.kubeletConfig.imageGcLowThresholdPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.image-maximum-gc-age" => Some(("nodePool.config.kubeletConfig.imageMaximumGcAge", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.image-minimum-gc-age" => Some(("nodePool.config.kubeletConfig.imageMinimumGcAge", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.insecure-kubelet-readonly-port-enabled" => Some(("nodePool.config.kubeletConfig.insecureKubeletReadonlyPortEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.max-parallel-image-pulls" => Some(("nodePool.config.kubeletConfig.maxParallelImagePulls", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.memory-manager.policy" => Some(("nodePool.config.kubeletConfig.memoryManager.policy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.pod-pids-limit" => Some(("nodePool.config.kubeletConfig.podPidsLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.single-process-oom-kill" => Some(("nodePool.config.kubeletConfig.singleProcessOomKill", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.topology-manager.policy" => Some(("nodePool.config.kubeletConfig.topologyManager.policy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.kubelet-config.topology-manager.scope" => Some(("nodePool.config.kubeletConfig.topologyManager.scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.labels" => Some(("nodePool.config.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "node-pool.config.linux-node-config.cgroup-mode" => Some(("nodePool.config.linuxNodeConfig.cgroupMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.linux-node-config.hugepages.hugepage-size1g" => Some(("nodePool.config.linuxNodeConfig.hugepages.hugepageSize1g", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.linux-node-config.hugepages.hugepage-size2m" => Some(("nodePool.config.linuxNodeConfig.hugepages.hugepageSize2m", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.linux-node-config.node-kernel-module-loading.policy" => Some(("nodePool.config.linuxNodeConfig.nodeKernelModuleLoading.policy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.linux-node-config.sysctls" => Some(("nodePool.config.linuxNodeConfig.sysctls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "node-pool.config.linux-node-config.transparent-hugepage-defrag" => Some(("nodePool.config.linuxNodeConfig.transparentHugepageDefrag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.linux-node-config.transparent-hugepage-enabled" => Some(("nodePool.config.linuxNodeConfig.transparentHugepageEnabled", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.local-nvme-ssd-block-config.local-ssd-count" => Some(("nodePool.config.localNvmeSsdBlockConfig.localSsdCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.local-ssd-count" => Some(("nodePool.config.localSsdCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.local-ssd-encryption-mode" => Some(("nodePool.config.localSsdEncryptionMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.logging-config.variant-config.variant" => Some(("nodePool.config.loggingConfig.variantConfig.variant", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.machine-type" => Some(("nodePool.config.machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.config.max-run-duration" => Some(("nodePool.config.maxRunDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                    "node-pool.config.sole-tenant-config.min-node-cpus" => Some(("nodePool.config.soleTenantConfig.minNodeCpus", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "node-pool.config.spot" => Some(("nodePool.config.spot", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.config.storage-pools" => Some(("nodePool.config.storagePools", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
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
                    "node-pool.network-config.network-tier-config.network-tier" => Some(("nodePool.networkConfig.networkTierConfig.networkTier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.network-config.pod-cidr-overprovision-config.disable" => Some(("nodePool.networkConfig.podCidrOverprovisionConfig.disable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-pool.network-config.pod-ipv4-cidr-block" => Some(("nodePool.networkConfig.podIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.network-config.pod-ipv4-range-utilization" => Some(("nodePool.networkConfig.podIpv4RangeUtilization", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "node-pool.network-config.pod-range" => Some(("nodePool.networkConfig.podRange", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.network-config.subnetwork" => Some(("nodePool.networkConfig.subnetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool.node-drain-config.respect-pdb-during-node-pool-deletion" => Some(("nodePool.nodeDrainConfig.respectPdbDuringNodePoolDeletion", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
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
                    "node-pool.upgrade-settings.blue-green-settings.autoscaled-rollout-policy.wait-for-drain-duration" => Some(("nodePool.upgradeSettings.blueGreenSettings.autoscaledRolloutPolicy.waitForDrainDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["advanced-machine-features", "allowed-unsafe-sysctls", "auto-repair", "auto-upgrade", "auto-upgrade-start-time", "autopilot-config", "autoprovisioned", "autoscaled-rollout-policy", "autoscaling", "batch-node-count", "batch-percentage", "batch-soak-duration", "best-effort-provisioning", "blue-green-info", "blue-green-settings", "blue-instance-group-urls", "blue-pool-deletion-start-time", "boot-disk", "boot-disk-kms-key", "cgroup-mode", "cluster-id", "confidential-instance-type", "confidential-nodes", "config", "consume-reservation-type", "container-log-max-files", "container-log-max-size", "containerd-config", "cpu-cfs-quota", "cpu-cfs-quota-period", "cpu-manager-policy", "create-pod-range", "data-cache-count", "description", "disable", "disk-size-gb", "disk-type", "effective-cgroup-mode", "enable-confidential-storage", "enable-integrity-monitoring", "enable-nested-virtualization", "enable-private-nodes", "enable-secure-boot", "enabled", "ephemeral-storage-local-ssd-config", "etag", "eviction-max-pod-grace-period-seconds", "eviction-minimum-reclaim", "eviction-soft", "eviction-soft-grace-period", "fast-socket", "flex-start", "gcfs-config", "green-instance-group-urls", "green-pool-version", "gvnic", "hugepage-size1g", "hugepage-size2m", "hugepages", "image-gc-high-threshold-percent", "image-gc-low-threshold-percent", "image-maximum-gc-age", "image-minimum-gc-age", "image-type", "imagefs-available", "imagefs-inodes-free", "initial-node-count", "insecure-kubelet-readonly-port-enabled", "instance-group-urls", "key", "kubelet-config", "labels", "linux-node-config", "local-nvme-ssd-block-config", "local-ssd-count", "local-ssd-encryption-mode", "location-policy", "locations", "logging-config", "machine-type", "management", "max-node-count", "max-parallel-image-pulls", "max-pods-constraint", "max-pods-per-node", "max-run-duration", "max-surge", "max-unavailable", "memory-available", "memory-manager", "metadata", "min-cpu-platform", "min-node-count", "min-node-cpus", "min-provision-nodes", "mode", "name", "network-config", "network-performance-config", "network-tier", "network-tier-config", "node-drain-config", "node-group", "node-kernel-module-loading", "node-pool", "node-pool-soak-duration", "nodefs-available", "nodefs-inodes-free", "oauth-scopes", "os-version", "parent", "performance-monitoring-unit", "phase", "pid-available", "placement-policy", "pod-cidr-overprovision-config", "pod-ipv4-cidr-block", "pod-ipv4-cidr-size", "pod-ipv4-range-utilization", "pod-pids-limit", "pod-range", "policy", "policy-name", "preemptible", "private-registry-access-config", "project-id", "provisioned-iops", "provisioned-throughput", "queued-provisioning", "reservation-affinity", "resource-labels", "resource-manager-tags", "respect-pdb-during-node-pool-deletion", "sandbox-config", "scope", "self-link", "service-account", "shielded-instance-config", "single-process-oom-kill", "size-gb", "sole-tenant-config", "spot", "standard-rollout-policy", "status", "status-message", "storage-pools", "strategy", "subnetwork", "sysctls", "tags", "threads-per-core", "topology-manager", "total-egress-bandwidth-tier", "total-max-node-count", "total-min-node-count", "tpu-topology", "transparent-hugepage-defrag", "transparent-hugepage-enabled", "type", "update-info", "upgrade-options", "upgrade-settings", "values", "variant", "variant-config", "version", "wait-for-drain-duration", "windows-node-config", "workload-metadata-config", "writable-cgroups", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::CreateNodePoolRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_clusters_node_pools_create(
            request,
            opt.value_of("project-id").unwrap_or(""),
            opt.value_of("zone").unwrap_or(""),
            opt.value_of("cluster-id").unwrap_or(""),
        );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_zones_clusters_node_pools_delete(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.projects().zones_clusters_node_pools_delete(
            opt.value_of("project-id").unwrap_or(""),
            opt.value_of("zone").unwrap_or(""),
            opt.value_of("cluster-id").unwrap_or(""),
            opt.value_of("node-pool-id").unwrap_or(""),
        );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "name" => {
                    call = call.name(value.unwrap_or(""));
                }
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v.extend(["name"].iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_zones_clusters_node_pools_fetch_node_pool_upgrade_info(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .projects()
            .zones_clusters_node_pools_fetch_node_pool_upgrade_info(
                opt.value_of("name").unwrap_or(""),
            );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "version" => {
                    call = call.version(value.unwrap_or(""));
                }
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v.extend(["version"].iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_zones_clusters_node_pools_get(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.projects().zones_clusters_node_pools_get(
            opt.value_of("project-id").unwrap_or(""),
            opt.value_of("zone").unwrap_or(""),
            opt.value_of("cluster-id").unwrap_or(""),
            opt.value_of("node-pool-id").unwrap_or(""),
        );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "name" => {
                    call = call.name(value.unwrap_or(""));
                }
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v.extend(["name"].iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_zones_clusters_node_pools_list(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.projects().zones_clusters_node_pools_list(
            opt.value_of("project-id").unwrap_or(""),
            opt.value_of("zone").unwrap_or(""),
            opt.value_of("cluster-id").unwrap_or(""),
        );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "parent" => {
                    call = call.parent(value.unwrap_or(""));
                }
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v.extend(["parent"].iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_zones_clusters_node_pools_rollback(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "cluster-id" => Some((
                    "clusterId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "name" => Some((
                    "name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "node-pool-id" => Some((
                    "nodePoolId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "project-id" => Some((
                    "projectId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "respect-pdb" => Some((
                    "respectPdb",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "zone" => Some((
                    "zone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "cluster-id",
                            "name",
                            "node-pool-id",
                            "project-id",
                            "respect-pdb",
                            "zone",
                        ],
                    );
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::RollbackNodePoolUpgradeRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_clusters_node_pools_rollback(
            request,
            opt.value_of("project-id").unwrap_or(""),
            opt.value_of("zone").unwrap_or(""),
            opt.value_of("cluster-id").unwrap_or(""),
            opt.value_of("node-pool-id").unwrap_or(""),
        );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_zones_clusters_node_pools_set_management(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "cluster-id" => Some((
                    "clusterId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "management.auto-repair" => Some((
                    "management.autoRepair",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "management.auto-upgrade" => Some((
                    "management.autoUpgrade",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "management.upgrade-options.auto-upgrade-start-time" => Some((
                    "management.upgradeOptions.autoUpgradeStartTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "management.upgrade-options.description" => Some((
                    "management.upgradeOptions.description",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "name" => Some((
                    "name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "node-pool-id" => Some((
                    "nodePoolId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "project-id" => Some((
                    "projectId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "zone" => Some((
                    "zone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "auto-repair",
                            "auto-upgrade",
                            "auto-upgrade-start-time",
                            "cluster-id",
                            "description",
                            "management",
                            "name",
                            "node-pool-id",
                            "project-id",
                            "upgrade-options",
                            "zone",
                        ],
                    );
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::SetNodePoolManagementRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self
            .hub
            .projects()
            .zones_clusters_node_pools_set_management(
                request,
                opt.value_of("project-id").unwrap_or(""),
                opt.value_of("zone").unwrap_or(""),
                opt.value_of("cluster-id").unwrap_or(""),
                opt.value_of("node-pool-id").unwrap_or(""),
            );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_zones_clusters_node_pools_set_size(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "cluster-id" => Some((
                    "clusterId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "name" => Some((
                    "name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "node-count" => Some((
                    "nodeCount",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "node-pool-id" => Some((
                    "nodePoolId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "project-id" => Some((
                    "projectId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "zone" => Some((
                    "zone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "cluster-id",
                            "name",
                            "node-count",
                            "node-pool-id",
                            "project-id",
                            "zone",
                        ],
                    );
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::SetNodePoolSizeRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_clusters_node_pools_set_size(
            request,
            opt.value_of("project-id").unwrap_or(""),
            opt.value_of("zone").unwrap_or(""),
            opt.value_of("cluster-id").unwrap_or(""),
            opt.value_of("node-pool-id").unwrap_or(""),
        );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_zones_clusters_node_pools_update(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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
                    "boot-disk.disk-type" => Some(("bootDisk.diskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "boot-disk.provisioned-iops" => Some(("bootDisk.provisionedIops", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "boot-disk.provisioned-throughput" => Some(("bootDisk.provisionedThroughput", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "boot-disk.size-gb" => Some(("bootDisk.sizeGb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "cluster-id" => Some(("clusterId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "confidential-nodes.confidential-instance-type" => Some(("confidentialNodes.confidentialInstanceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "confidential-nodes.enabled" => Some(("confidentialNodes.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "containerd-config.private-registry-access-config.enabled" => Some(("containerdConfig.privateRegistryAccessConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "containerd-config.writable-cgroups.enabled" => Some(("containerdConfig.writableCgroups.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "disk-size-gb" => Some(("diskSizeGb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "disk-type" => Some(("diskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "etag" => Some(("etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "fast-socket.enabled" => Some(("fastSocket.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "flex-start" => Some(("flexStart", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "gcfs-config.enabled" => Some(("gcfsConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "gvnic.enabled" => Some(("gvnic.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "image-type" => Some(("imageType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.allowed-unsafe-sysctls" => Some(("kubeletConfig.allowedUnsafeSysctls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "kubelet-config.container-log-max-files" => Some(("kubeletConfig.containerLogMaxFiles", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "kubelet-config.container-log-max-size" => Some(("kubeletConfig.containerLogMaxSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.cpu-cfs-quota" => Some(("kubeletConfig.cpuCfsQuota", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kubelet-config.cpu-cfs-quota-period" => Some(("kubeletConfig.cpuCfsQuotaPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.cpu-manager-policy" => Some(("kubeletConfig.cpuManagerPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.eviction-max-pod-grace-period-seconds" => Some(("kubeletConfig.evictionMaxPodGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "kubelet-config.eviction-minimum-reclaim.imagefs-available" => Some(("kubeletConfig.evictionMinimumReclaim.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.eviction-minimum-reclaim.imagefs-inodes-free" => Some(("kubeletConfig.evictionMinimumReclaim.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.eviction-minimum-reclaim.memory-available" => Some(("kubeletConfig.evictionMinimumReclaim.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.eviction-minimum-reclaim.nodefs-available" => Some(("kubeletConfig.evictionMinimumReclaim.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.eviction-minimum-reclaim.nodefs-inodes-free" => Some(("kubeletConfig.evictionMinimumReclaim.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.eviction-minimum-reclaim.pid-available" => Some(("kubeletConfig.evictionMinimumReclaim.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.eviction-soft.imagefs-available" => Some(("kubeletConfig.evictionSoft.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.eviction-soft.imagefs-inodes-free" => Some(("kubeletConfig.evictionSoft.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.eviction-soft.memory-available" => Some(("kubeletConfig.evictionSoft.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.eviction-soft.nodefs-available" => Some(("kubeletConfig.evictionSoft.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.eviction-soft.nodefs-inodes-free" => Some(("kubeletConfig.evictionSoft.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.eviction-soft.pid-available" => Some(("kubeletConfig.evictionSoft.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.eviction-soft-grace-period.imagefs-available" => Some(("kubeletConfig.evictionSoftGracePeriod.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.eviction-soft-grace-period.imagefs-inodes-free" => Some(("kubeletConfig.evictionSoftGracePeriod.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.eviction-soft-grace-period.memory-available" => Some(("kubeletConfig.evictionSoftGracePeriod.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.eviction-soft-grace-period.nodefs-available" => Some(("kubeletConfig.evictionSoftGracePeriod.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.eviction-soft-grace-period.nodefs-inodes-free" => Some(("kubeletConfig.evictionSoftGracePeriod.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.eviction-soft-grace-period.pid-available" => Some(("kubeletConfig.evictionSoftGracePeriod.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.image-gc-high-threshold-percent" => Some(("kubeletConfig.imageGcHighThresholdPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "kubelet-config.image-gc-low-threshold-percent" => Some(("kubeletConfig.imageGcLowThresholdPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "kubelet-config.image-maximum-gc-age" => Some(("kubeletConfig.imageMaximumGcAge", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.image-minimum-gc-age" => Some(("kubeletConfig.imageMinimumGcAge", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.insecure-kubelet-readonly-port-enabled" => Some(("kubeletConfig.insecureKubeletReadonlyPortEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kubelet-config.max-parallel-image-pulls" => Some(("kubeletConfig.maxParallelImagePulls", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "kubelet-config.memory-manager.policy" => Some(("kubeletConfig.memoryManager.policy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.pod-pids-limit" => Some(("kubeletConfig.podPidsLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.single-process-oom-kill" => Some(("kubeletConfig.singleProcessOomKill", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "kubelet-config.topology-manager.policy" => Some(("kubeletConfig.topologyManager.policy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kubelet-config.topology-manager.scope" => Some(("kubeletConfig.topologyManager.scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels.labels" => Some(("labels.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "linux-node-config.cgroup-mode" => Some(("linuxNodeConfig.cgroupMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "linux-node-config.hugepages.hugepage-size1g" => Some(("linuxNodeConfig.hugepages.hugepageSize1g", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "linux-node-config.hugepages.hugepage-size2m" => Some(("linuxNodeConfig.hugepages.hugepageSize2m", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "linux-node-config.node-kernel-module-loading.policy" => Some(("linuxNodeConfig.nodeKernelModuleLoading.policy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "linux-node-config.sysctls" => Some(("linuxNodeConfig.sysctls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "linux-node-config.transparent-hugepage-defrag" => Some(("linuxNodeConfig.transparentHugepageDefrag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "linux-node-config.transparent-hugepage-enabled" => Some(("linuxNodeConfig.transparentHugepageEnabled", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "locations" => Some(("locations", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "logging-config.variant-config.variant" => Some(("loggingConfig.variantConfig.variant", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "machine-type" => Some(("machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "max-run-duration" => Some(("maxRunDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-drain-config.respect-pdb-during-node-pool-deletion" => Some(("nodeDrainConfig.respectPdbDuringNodePoolDeletion", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-network-config.create-pod-range" => Some(("nodeNetworkConfig.createPodRange", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-network-config.enable-private-nodes" => Some(("nodeNetworkConfig.enablePrivateNodes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-network-config.network-performance-config.total-egress-bandwidth-tier" => Some(("nodeNetworkConfig.networkPerformanceConfig.totalEgressBandwidthTier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-network-config.network-tier-config.network-tier" => Some(("nodeNetworkConfig.networkTierConfig.networkTier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-network-config.pod-cidr-overprovision-config.disable" => Some(("nodeNetworkConfig.podCidrOverprovisionConfig.disable", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "node-network-config.pod-ipv4-cidr-block" => Some(("nodeNetworkConfig.podIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-network-config.pod-ipv4-range-utilization" => Some(("nodeNetworkConfig.podIpv4RangeUtilization", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "node-network-config.pod-range" => Some(("nodeNetworkConfig.podRange", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-network-config.subnetwork" => Some(("nodeNetworkConfig.subnetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-pool-id" => Some(("nodePoolId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "node-version" => Some(("nodeVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "queued-provisioning.enabled" => Some(("queuedProvisioning.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "resource-labels.labels" => Some(("resourceLabels.labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "resource-manager-tags.tags" => Some(("resourceManagerTags.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "storage-pools" => Some(("storagePools", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "tags.tags" => Some(("tags.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "upgrade-settings.blue-green-settings.autoscaled-rollout-policy.wait-for-drain-duration" => Some(("upgradeSettings.blueGreenSettings.autoscaledRolloutPolicy.waitForDrainDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["allowed-unsafe-sysctls", "autoscaled-rollout-policy", "batch-node-count", "batch-percentage", "batch-soak-duration", "blue-green-settings", "boot-disk", "cgroup-mode", "cluster-id", "confidential-instance-type", "confidential-nodes", "container-log-max-files", "container-log-max-size", "containerd-config", "cpu-cfs-quota", "cpu-cfs-quota-period", "cpu-manager-policy", "create-pod-range", "disable", "disk-size-gb", "disk-type", "enable-private-nodes", "enabled", "etag", "eviction-max-pod-grace-period-seconds", "eviction-minimum-reclaim", "eviction-soft", "eviction-soft-grace-period", "fast-socket", "flex-start", "gcfs-config", "gvnic", "hugepage-size1g", "hugepage-size2m", "hugepages", "image-gc-high-threshold-percent", "image-gc-low-threshold-percent", "image-maximum-gc-age", "image-minimum-gc-age", "image-type", "imagefs-available", "imagefs-inodes-free", "insecure-kubelet-readonly-port-enabled", "kubelet-config", "labels", "linux-node-config", "locations", "logging-config", "machine-type", "max-parallel-image-pulls", "max-run-duration", "max-surge", "max-unavailable", "memory-available", "memory-manager", "mode", "name", "network-performance-config", "network-tier", "network-tier-config", "node-drain-config", "node-kernel-module-loading", "node-network-config", "node-pool-id", "node-pool-soak-duration", "node-version", "nodefs-available", "nodefs-inodes-free", "os-version", "pid-available", "pod-cidr-overprovision-config", "pod-ipv4-cidr-block", "pod-ipv4-range-utilization", "pod-pids-limit", "pod-range", "policy", "private-registry-access-config", "project-id", "provisioned-iops", "provisioned-throughput", "queued-provisioning", "resource-labels", "resource-manager-tags", "respect-pdb-during-node-pool-deletion", "scope", "single-process-oom-kill", "size-gb", "standard-rollout-policy", "storage-pools", "strategy", "subnetwork", "sysctls", "tags", "topology-manager", "total-egress-bandwidth-tier", "transparent-hugepage-defrag", "transparent-hugepage-enabled", "upgrade-settings", "variant", "variant-config", "wait-for-drain-duration", "windows-node-config", "workload-metadata-config", "writable-cgroups", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::UpdateNodePoolRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_clusters_node_pools_update(
            request,
            opt.value_of("project-id").unwrap_or(""),
            opt.value_of("zone").unwrap_or(""),
            opt.value_of("cluster-id").unwrap_or(""),
            opt.value_of("node-pool-id").unwrap_or(""),
        );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_zones_clusters_resource_labels(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "cluster-id" => Some((
                    "clusterId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "label-fingerprint" => Some((
                    "labelFingerprint",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "name" => Some((
                    "name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "project-id" => Some((
                    "projectId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "resource-labels" => Some((
                    "resourceLabels",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Map,
                    },
                )),
                "zone" => Some((
                    "zone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "cluster-id",
                            "label-fingerprint",
                            "name",
                            "project-id",
                            "resource-labels",
                            "zone",
                        ],
                    );
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::SetLabelsRequest = serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_clusters_resource_labels(
            request,
            opt.value_of("project-id").unwrap_or(""),
            opt.value_of("zone").unwrap_or(""),
            opt.value_of("cluster-id").unwrap_or(""),
        );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_zones_clusters_set_maintenance_policy(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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
                    "maintenance-policy.window.recurring-window.window.maintenance-exclusion-options.end-time-behavior" => Some(("maintenancePolicy.window.recurringWindow.window.maintenanceExclusionOptions.endTimeBehavior", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "maintenance-policy.window.recurring-window.window.maintenance-exclusion-options.scope" => Some(("maintenancePolicy.window.recurringWindow.window.maintenanceExclusionOptions.scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "maintenance-policy.window.recurring-window.window.start-time" => Some(("maintenancePolicy.window.recurringWindow.window.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "project-id" => Some(("projectId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["cluster-id", "daily-maintenance-window", "duration", "end-time", "end-time-behavior", "maintenance-exclusion-options", "maintenance-policy", "name", "project-id", "recurrence", "recurring-window", "resource-version", "scope", "start-time", "window", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::SetMaintenancePolicyRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_clusters_set_maintenance_policy(
            request,
            opt.value_of("project-id").unwrap_or(""),
            opt.value_of("zone").unwrap_or(""),
            opt.value_of("cluster-id").unwrap_or(""),
        );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_zones_clusters_set_master_auth(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "action" => Some((
                    "action",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "cluster-id" => Some((
                    "clusterId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "name" => Some((
                    "name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "project-id" => Some((
                    "projectId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "update.client-certificate" => Some((
                    "update.clientCertificate",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "update.client-certificate-config.issue-client-certificate" => Some((
                    "update.clientCertificateConfig.issueClientCertificate",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "update.client-key" => Some((
                    "update.clientKey",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "update.cluster-ca-certificate" => Some((
                    "update.clusterCaCertificate",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "update.password" => Some((
                    "update.password",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "update.username" => Some((
                    "update.username",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "zone" => Some((
                    "zone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "action",
                            "client-certificate",
                            "client-certificate-config",
                            "client-key",
                            "cluster-ca-certificate",
                            "cluster-id",
                            "issue-client-certificate",
                            "name",
                            "password",
                            "project-id",
                            "update",
                            "username",
                            "zone",
                        ],
                    );
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::SetMasterAuthRequest = serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_clusters_set_master_auth(
            request,
            opt.value_of("project-id").unwrap_or(""),
            opt.value_of("zone").unwrap_or(""),
            opt.value_of("cluster-id").unwrap_or(""),
        );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_zones_clusters_set_network_policy(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "cluster-id" => Some((
                    "clusterId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "name" => Some((
                    "name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "network-policy.enabled" => Some((
                    "networkPolicy.enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "network-policy.provider" => Some((
                    "networkPolicy.provider",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "project-id" => Some((
                    "projectId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "zone" => Some((
                    "zone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "cluster-id",
                            "enabled",
                            "name",
                            "network-policy",
                            "project-id",
                            "provider",
                            "zone",
                        ],
                    );
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::SetNetworkPolicyRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_clusters_set_network_policy(
            request,
            opt.value_of("project-id").unwrap_or(""),
            opt.value_of("zone").unwrap_or(""),
            opt.value_of("cluster-id").unwrap_or(""),
        );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_zones_clusters_start_ip_rotation(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "cluster-id" => Some((
                    "clusterId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "name" => Some((
                    "name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "project-id" => Some((
                    "projectId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "rotate-credentials" => Some((
                    "rotateCredentials",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "zone" => Some((
                    "zone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "cluster-id",
                            "name",
                            "project-id",
                            "rotate-credentials",
                            "zone",
                        ],
                    );
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::StartIPRotationRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_clusters_start_ip_rotation(
            request,
            opt.value_of("project-id").unwrap_or(""),
            opt.value_of("zone").unwrap_or(""),
            opt.value_of("cluster-id").unwrap_or(""),
        );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_zones_clusters_update(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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
                    "update.desired-addons-config.high-scale-checkpointing-config.enabled" => Some(("update.desiredAddonsConfig.highScaleCheckpointingConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.horizontal-pod-autoscaling.disabled" => Some(("update.desiredAddonsConfig.horizontalPodAutoscaling.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.http-load-balancing.disabled" => Some(("update.desiredAddonsConfig.httpLoadBalancing.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.kubernetes-dashboard.disabled" => Some(("update.desiredAddonsConfig.kubernetesDashboard.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.lustre-csi-driver-config.enable-legacy-lustre-port" => Some(("update.desiredAddonsConfig.lustreCsiDriverConfig.enableLegacyLustrePort", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.lustre-csi-driver-config.enabled" => Some(("update.desiredAddonsConfig.lustreCsiDriverConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.network-policy-config.disabled" => Some(("update.desiredAddonsConfig.networkPolicyConfig.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.parallelstore-csi-driver-config.enabled" => Some(("update.desiredAddonsConfig.parallelstoreCsiDriverConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.ray-operator-config.enabled" => Some(("update.desiredAddonsConfig.rayOperatorConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.ray-operator-config.ray-cluster-logging-config.enabled" => Some(("update.desiredAddonsConfig.rayOperatorConfig.rayClusterLoggingConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.ray-operator-config.ray-cluster-monitoring-config.enabled" => Some(("update.desiredAddonsConfig.rayOperatorConfig.rayClusterMonitoringConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-addons-config.stateful-ha-config.enabled" => Some(("update.desiredAddonsConfig.statefulHaConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-anonymous-authentication-config.mode" => Some(("update.desiredAnonymousAuthenticationConfig.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-authenticator-groups-config.enabled" => Some(("update.desiredAuthenticatorGroupsConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-authenticator-groups-config.security-group" => Some(("update.desiredAuthenticatorGroupsConfig.securityGroup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-auto-ipam-config.enabled" => Some(("update.desiredAutoIpamConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-autopilot-workload-policy-config.allow-net-admin" => Some(("update.desiredAutopilotWorkloadPolicyConfig.allowNetAdmin", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-autopilot-workload-policy-config.autopilot-compatibility-auditing-enabled" => Some(("update.desiredAutopilotWorkloadPolicyConfig.autopilotCompatibilityAuditingEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
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
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.blue-green-settings.autoscaled-rollout-policy.wait-for-drain-duration" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.blueGreenSettings.autoscaledRolloutPolicy.waitForDrainDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.blue-green-settings.node-pool-soak-duration" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.blueGreenSettings.nodePoolSoakDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.blue-green-settings.standard-rollout-policy.batch-node-count" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.blueGreenSettings.standardRolloutPolicy.batchNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.blue-green-settings.standard-rollout-policy.batch-percentage" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.blueGreenSettings.standardRolloutPolicy.batchPercentage", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.blue-green-settings.standard-rollout-policy.batch-soak-duration" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.blueGreenSettings.standardRolloutPolicy.batchSoakDuration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.max-surge" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.maxSurge", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.max-unavailable" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.maxUnavailable", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoprovisioning-node-pool-defaults.upgrade-settings.strategy" => Some(("update.desiredClusterAutoscaling.autoprovisioningNodePoolDefaults.upgradeSettings.strategy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.autoscaling-profile" => Some(("update.desiredClusterAutoscaling.autoscalingProfile", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.default-compute-class-config.enabled" => Some(("update.desiredClusterAutoscaling.defaultComputeClassConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-cluster-autoscaling.enable-node-autoprovisioning" => Some(("update.desiredClusterAutoscaling.enableNodeAutoprovisioning", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-compliance-posture-config.mode" => Some(("update.desiredCompliancePostureConfig.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-containerd-config.private-registry-access-config.enabled" => Some(("update.desiredContainerdConfig.privateRegistryAccessConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-containerd-config.writable-cgroups.enabled" => Some(("update.desiredContainerdConfig.writableCgroups.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-control-plane-endpoints-config.dns-endpoint-config.allow-external-traffic" => Some(("update.desiredControlPlaneEndpointsConfig.dnsEndpointConfig.allowExternalTraffic", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-control-plane-endpoints-config.dns-endpoint-config.enable-k8s-certs-via-dns" => Some(("update.desiredControlPlaneEndpointsConfig.dnsEndpointConfig.enableK8sCertsViaDns", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-control-plane-endpoints-config.dns-endpoint-config.enable-k8s-tokens-via-dns" => Some(("update.desiredControlPlaneEndpointsConfig.dnsEndpointConfig.enableK8sTokensViaDns", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-control-plane-endpoints-config.dns-endpoint-config.endpoint" => Some(("update.desiredControlPlaneEndpointsConfig.dnsEndpointConfig.endpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-control-plane-endpoints-config.ip-endpoints-config.authorized-networks-config.enabled" => Some(("update.desiredControlPlaneEndpointsConfig.ipEndpointsConfig.authorizedNetworksConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-control-plane-endpoints-config.ip-endpoints-config.authorized-networks-config.gcp-public-cidrs-access-enabled" => Some(("update.desiredControlPlaneEndpointsConfig.ipEndpointsConfig.authorizedNetworksConfig.gcpPublicCidrsAccessEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-control-plane-endpoints-config.ip-endpoints-config.authorized-networks-config.private-endpoint-enforcement-enabled" => Some(("update.desiredControlPlaneEndpointsConfig.ipEndpointsConfig.authorizedNetworksConfig.privateEndpointEnforcementEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-control-plane-endpoints-config.ip-endpoints-config.enable-public-endpoint" => Some(("update.desiredControlPlaneEndpointsConfig.ipEndpointsConfig.enablePublicEndpoint", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-control-plane-endpoints-config.ip-endpoints-config.enabled" => Some(("update.desiredControlPlaneEndpointsConfig.ipEndpointsConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-control-plane-endpoints-config.ip-endpoints-config.global-access" => Some(("update.desiredControlPlaneEndpointsConfig.ipEndpointsConfig.globalAccess", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-control-plane-endpoints-config.ip-endpoints-config.private-endpoint" => Some(("update.desiredControlPlaneEndpointsConfig.ipEndpointsConfig.privateEndpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-control-plane-endpoints-config.ip-endpoints-config.private-endpoint-subnetwork" => Some(("update.desiredControlPlaneEndpointsConfig.ipEndpointsConfig.privateEndpointSubnetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-control-plane-endpoints-config.ip-endpoints-config.public-endpoint" => Some(("update.desiredControlPlaneEndpointsConfig.ipEndpointsConfig.publicEndpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-cost-management-config.enabled" => Some(("update.desiredCostManagementConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-database-encryption.current-state" => Some(("update.desiredDatabaseEncryption.currentState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-database-encryption.decryption-keys" => Some(("update.desiredDatabaseEncryption.decryptionKeys", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.desired-database-encryption.key-name" => Some(("update.desiredDatabaseEncryption.keyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-database-encryption.state" => Some(("update.desiredDatabaseEncryption.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-datapath-provider" => Some(("update.desiredDatapathProvider", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-default-enable-private-nodes" => Some(("update.desiredDefaultEnablePrivateNodes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-default-snat-status.disabled" => Some(("update.desiredDefaultSnatStatus.disabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-disable-l4-lb-firewall-reconciliation" => Some(("update.desiredDisableL4LbFirewallReconciliation", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-dns-config.additive-vpc-scope-dns-domain" => Some(("update.desiredDnsConfig.additiveVpcScopeDnsDomain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-dns-config.cluster-dns" => Some(("update.desiredDnsConfig.clusterDns", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-dns-config.cluster-dns-domain" => Some(("update.desiredDnsConfig.clusterDnsDomain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-dns-config.cluster-dns-scope" => Some(("update.desiredDnsConfig.clusterDnsScope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-enable-cilium-clusterwide-network-policy" => Some(("update.desiredEnableCiliumClusterwideNetworkPolicy", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-enable-fqdn-network-policy" => Some(("update.desiredEnableFqdnNetworkPolicy", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-enable-multi-networking" => Some(("update.desiredEnableMultiNetworking", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-enable-private-endpoint" => Some(("update.desiredEnablePrivateEndpoint", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-enterprise-config.desired-tier" => Some(("update.desiredEnterpriseConfig.desiredTier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-fleet.membership" => Some(("update.desiredFleet.membership", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-fleet.membership-type" => Some(("update.desiredFleet.membershipType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                    "update.desired-managed-opentelemetry-config.scope" => Some(("update.desiredManagedOpentelemetryConfig.scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-master-authorized-networks-config.enabled" => Some(("update.desiredMasterAuthorizedNetworksConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-master-authorized-networks-config.gcp-public-cidrs-access-enabled" => Some(("update.desiredMasterAuthorizedNetworksConfig.gcpPublicCidrsAccessEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-master-authorized-networks-config.private-endpoint-enforcement-enabled" => Some(("update.desiredMasterAuthorizedNetworksConfig.privateEndpointEnforcementEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-master-version" => Some(("update.desiredMasterVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-mesh-certificates.enable-certificates" => Some(("update.desiredMeshCertificates.enableCertificates", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-monitoring-config.advanced-datapath-observability-config.enable-metrics" => Some(("update.desiredMonitoringConfig.advancedDatapathObservabilityConfig.enableMetrics", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-monitoring-config.advanced-datapath-observability-config.enable-relay" => Some(("update.desiredMonitoringConfig.advancedDatapathObservabilityConfig.enableRelay", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-monitoring-config.advanced-datapath-observability-config.relay-mode" => Some(("update.desiredMonitoringConfig.advancedDatapathObservabilityConfig.relayMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-monitoring-config.component-config.enable-components" => Some(("update.desiredMonitoringConfig.componentConfig.enableComponents", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.desired-monitoring-config.managed-prometheus-config.auto-monitoring-config.scope" => Some(("update.desiredMonitoringConfig.managedPrometheusConfig.autoMonitoringConfig.scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-monitoring-config.managed-prometheus-config.enabled" => Some(("update.desiredMonitoringConfig.managedPrometheusConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-monitoring-service" => Some(("update.desiredMonitoringService", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-network-performance-config.total-egress-bandwidth-tier" => Some(("update.desiredNetworkPerformanceConfig.totalEgressBandwidthTier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-network-tier-config.network-tier" => Some(("update.desiredNetworkTierConfig.networkTier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.allowed-unsafe-sysctls" => Some(("update.desiredNodeKubeletConfig.allowedUnsafeSysctls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.desired-node-kubelet-config.container-log-max-files" => Some(("update.desiredNodeKubeletConfig.containerLogMaxFiles", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.container-log-max-size" => Some(("update.desiredNodeKubeletConfig.containerLogMaxSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.cpu-cfs-quota" => Some(("update.desiredNodeKubeletConfig.cpuCfsQuota", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.cpu-cfs-quota-period" => Some(("update.desiredNodeKubeletConfig.cpuCfsQuotaPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.cpu-manager-policy" => Some(("update.desiredNodeKubeletConfig.cpuManagerPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.eviction-max-pod-grace-period-seconds" => Some(("update.desiredNodeKubeletConfig.evictionMaxPodGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.eviction-minimum-reclaim.imagefs-available" => Some(("update.desiredNodeKubeletConfig.evictionMinimumReclaim.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.eviction-minimum-reclaim.imagefs-inodes-free" => Some(("update.desiredNodeKubeletConfig.evictionMinimumReclaim.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.eviction-minimum-reclaim.memory-available" => Some(("update.desiredNodeKubeletConfig.evictionMinimumReclaim.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.eviction-minimum-reclaim.nodefs-available" => Some(("update.desiredNodeKubeletConfig.evictionMinimumReclaim.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.eviction-minimum-reclaim.nodefs-inodes-free" => Some(("update.desiredNodeKubeletConfig.evictionMinimumReclaim.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.eviction-minimum-reclaim.pid-available" => Some(("update.desiredNodeKubeletConfig.evictionMinimumReclaim.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.eviction-soft.imagefs-available" => Some(("update.desiredNodeKubeletConfig.evictionSoft.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.eviction-soft.imagefs-inodes-free" => Some(("update.desiredNodeKubeletConfig.evictionSoft.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.eviction-soft.memory-available" => Some(("update.desiredNodeKubeletConfig.evictionSoft.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.eviction-soft.nodefs-available" => Some(("update.desiredNodeKubeletConfig.evictionSoft.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.eviction-soft.nodefs-inodes-free" => Some(("update.desiredNodeKubeletConfig.evictionSoft.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.eviction-soft.pid-available" => Some(("update.desiredNodeKubeletConfig.evictionSoft.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.eviction-soft-grace-period.imagefs-available" => Some(("update.desiredNodeKubeletConfig.evictionSoftGracePeriod.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.eviction-soft-grace-period.imagefs-inodes-free" => Some(("update.desiredNodeKubeletConfig.evictionSoftGracePeriod.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.eviction-soft-grace-period.memory-available" => Some(("update.desiredNodeKubeletConfig.evictionSoftGracePeriod.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.eviction-soft-grace-period.nodefs-available" => Some(("update.desiredNodeKubeletConfig.evictionSoftGracePeriod.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.eviction-soft-grace-period.nodefs-inodes-free" => Some(("update.desiredNodeKubeletConfig.evictionSoftGracePeriod.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.eviction-soft-grace-period.pid-available" => Some(("update.desiredNodeKubeletConfig.evictionSoftGracePeriod.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.image-gc-high-threshold-percent" => Some(("update.desiredNodeKubeletConfig.imageGcHighThresholdPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.image-gc-low-threshold-percent" => Some(("update.desiredNodeKubeletConfig.imageGcLowThresholdPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.image-maximum-gc-age" => Some(("update.desiredNodeKubeletConfig.imageMaximumGcAge", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.image-minimum-gc-age" => Some(("update.desiredNodeKubeletConfig.imageMinimumGcAge", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.insecure-kubelet-readonly-port-enabled" => Some(("update.desiredNodeKubeletConfig.insecureKubeletReadonlyPortEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.max-parallel-image-pulls" => Some(("update.desiredNodeKubeletConfig.maxParallelImagePulls", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.memory-manager.policy" => Some(("update.desiredNodeKubeletConfig.memoryManager.policy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.pod-pids-limit" => Some(("update.desiredNodeKubeletConfig.podPidsLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.single-process-oom-kill" => Some(("update.desiredNodeKubeletConfig.singleProcessOomKill", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.topology-manager.policy" => Some(("update.desiredNodeKubeletConfig.topologyManager.policy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-kubelet-config.topology-manager.scope" => Some(("update.desiredNodeKubeletConfig.topologyManager.scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.allowed-unsafe-sysctls" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.allowedUnsafeSysctls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.desired-node-pool-auto-config-kubelet-config.container-log-max-files" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.containerLogMaxFiles", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.container-log-max-size" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.containerLogMaxSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.cpu-cfs-quota" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.cpuCfsQuota", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.cpu-cfs-quota-period" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.cpuCfsQuotaPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.cpu-manager-policy" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.cpuManagerPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.eviction-max-pod-grace-period-seconds" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.evictionMaxPodGracePeriodSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.eviction-minimum-reclaim.imagefs-available" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.evictionMinimumReclaim.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.eviction-minimum-reclaim.imagefs-inodes-free" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.evictionMinimumReclaim.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.eviction-minimum-reclaim.memory-available" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.evictionMinimumReclaim.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.eviction-minimum-reclaim.nodefs-available" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.evictionMinimumReclaim.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.eviction-minimum-reclaim.nodefs-inodes-free" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.evictionMinimumReclaim.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.eviction-minimum-reclaim.pid-available" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.evictionMinimumReclaim.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.eviction-soft.imagefs-available" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.evictionSoft.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.eviction-soft.imagefs-inodes-free" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.evictionSoft.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.eviction-soft.memory-available" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.evictionSoft.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.eviction-soft.nodefs-available" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.evictionSoft.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.eviction-soft.nodefs-inodes-free" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.evictionSoft.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.eviction-soft.pid-available" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.evictionSoft.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.eviction-soft-grace-period.imagefs-available" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.evictionSoftGracePeriod.imagefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.eviction-soft-grace-period.imagefs-inodes-free" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.evictionSoftGracePeriod.imagefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.eviction-soft-grace-period.memory-available" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.evictionSoftGracePeriod.memoryAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.eviction-soft-grace-period.nodefs-available" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.evictionSoftGracePeriod.nodefsAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.eviction-soft-grace-period.nodefs-inodes-free" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.evictionSoftGracePeriod.nodefsInodesFree", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.eviction-soft-grace-period.pid-available" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.evictionSoftGracePeriod.pidAvailable", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.image-gc-high-threshold-percent" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.imageGcHighThresholdPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.image-gc-low-threshold-percent" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.imageGcLowThresholdPercent", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.image-maximum-gc-age" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.imageMaximumGcAge", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.image-minimum-gc-age" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.imageMinimumGcAge", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.insecure-kubelet-readonly-port-enabled" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.insecureKubeletReadonlyPortEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.max-parallel-image-pulls" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.maxParallelImagePulls", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.memory-manager.policy" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.memoryManager.policy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.pod-pids-limit" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.podPidsLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.single-process-oom-kill" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.singleProcessOomKill", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.topology-manager.policy" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.topologyManager.policy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-kubelet-config.topology-manager.scope" => Some(("update.desiredNodePoolAutoConfigKubeletConfig.topologyManager.scope", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-linux-node-config.cgroup-mode" => Some(("update.desiredNodePoolAutoConfigLinuxNodeConfig.cgroupMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-linux-node-config.hugepages.hugepage-size1g" => Some(("update.desiredNodePoolAutoConfigLinuxNodeConfig.hugepages.hugepageSize1g", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-linux-node-config.hugepages.hugepage-size2m" => Some(("update.desiredNodePoolAutoConfigLinuxNodeConfig.hugepages.hugepageSize2m", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-linux-node-config.node-kernel-module-loading.policy" => Some(("update.desiredNodePoolAutoConfigLinuxNodeConfig.nodeKernelModuleLoading.policy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-linux-node-config.sysctls" => Some(("update.desiredNodePoolAutoConfigLinuxNodeConfig.sysctls", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "update.desired-node-pool-auto-config-linux-node-config.transparent-hugepage-defrag" => Some(("update.desiredNodePoolAutoConfigLinuxNodeConfig.transparentHugepageDefrag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-node-pool-auto-config-linux-node-config.transparent-hugepage-enabled" => Some(("update.desiredNodePoolAutoConfigLinuxNodeConfig.transparentHugepageEnabled", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
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
                    "update.desired-pod-autoscaling.hpa-profile" => Some(("update.desiredPodAutoscaling.hpaProfile", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-private-cluster-config.enable-private-endpoint" => Some(("update.desiredPrivateClusterConfig.enablePrivateEndpoint", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-private-cluster-config.enable-private-nodes" => Some(("update.desiredPrivateClusterConfig.enablePrivateNodes", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-private-cluster-config.master-global-access-config.enabled" => Some(("update.desiredPrivateClusterConfig.masterGlobalAccessConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-private-cluster-config.master-ipv4-cidr-block" => Some(("update.desiredPrivateClusterConfig.masterIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-private-cluster-config.peering-name" => Some(("update.desiredPrivateClusterConfig.peeringName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-private-cluster-config.private-endpoint" => Some(("update.desiredPrivateClusterConfig.privateEndpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-private-cluster-config.private-endpoint-subnetwork" => Some(("update.desiredPrivateClusterConfig.privateEndpointSubnetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-private-cluster-config.public-endpoint" => Some(("update.desiredPrivateClusterConfig.publicEndpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-private-ipv6-google-access" => Some(("update.desiredPrivateIpv6GoogleAccess", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-privileged-admission-config.allowlist-paths" => Some(("update.desiredPrivilegedAdmissionConfig.allowlistPaths", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.desired-rbac-binding-config.enable-insecure-binding-system-authenticated" => Some(("update.desiredRbacBindingConfig.enableInsecureBindingSystemAuthenticated", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-rbac-binding-config.enable-insecure-binding-system-unauthenticated" => Some(("update.desiredRbacBindingConfig.enableInsecureBindingSystemUnauthenticated", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-release-channel.channel" => Some(("update.desiredReleaseChannel.channel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-resource-usage-export-config.bigquery-destination.dataset-id" => Some(("update.desiredResourceUsageExportConfig.bigqueryDestination.datasetId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-resource-usage-export-config.consumption-metering-config.enabled" => Some(("update.desiredResourceUsageExportConfig.consumptionMeteringConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-resource-usage-export-config.enable-network-egress-metering" => Some(("update.desiredResourceUsageExportConfig.enableNetworkEgressMetering", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-secret-manager-config.enabled" => Some(("update.desiredSecretManagerConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-secret-manager-config.rotation-config.enabled" => Some(("update.desiredSecretManagerConfig.rotationConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-secret-manager-config.rotation-config.rotation-interval" => Some(("update.desiredSecretManagerConfig.rotationConfig.rotationInterval", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-security-posture-config.mode" => Some(("update.desiredSecurityPostureConfig.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-security-posture-config.vulnerability-mode" => Some(("update.desiredSecurityPostureConfig.vulnerabilityMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-service-external-ips-config.enabled" => Some(("update.desiredServiceExternalIpsConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-shielded-nodes.enabled" => Some(("update.desiredShieldedNodes.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-stack-type" => Some(("update.desiredStackType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-user-managed-keys-config.aggregation-ca" => Some(("update.desiredUserManagedKeysConfig.aggregationCa", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-user-managed-keys-config.cluster-ca" => Some(("update.desiredUserManagedKeysConfig.clusterCa", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-user-managed-keys-config.control-plane-disk-encryption-key" => Some(("update.desiredUserManagedKeysConfig.controlPlaneDiskEncryptionKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-user-managed-keys-config.control-plane-disk-encryption-key-versions" => Some(("update.desiredUserManagedKeysConfig.controlPlaneDiskEncryptionKeyVersions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.desired-user-managed-keys-config.etcd-api-ca" => Some(("update.desiredUserManagedKeysConfig.etcdApiCa", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-user-managed-keys-config.etcd-peer-ca" => Some(("update.desiredUserManagedKeysConfig.etcdPeerCa", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-user-managed-keys-config.gkeops-etcd-backup-encryption-key" => Some(("update.desiredUserManagedKeysConfig.gkeopsEtcdBackupEncryptionKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.desired-user-managed-keys-config.service-account-signing-keys" => Some(("update.desiredUserManagedKeysConfig.serviceAccountSigningKeys", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.desired-user-managed-keys-config.service-account-verification-keys" => Some(("update.desiredUserManagedKeysConfig.serviceAccountVerificationKeys", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.desired-vertical-pod-autoscaling.enabled" => Some(("update.desiredVerticalPodAutoscaling.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "update.desired-workload-identity-config.workload-pool" => Some(("update.desiredWorkloadIdentityConfig.workloadPool", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.enable-k8s-beta-apis.enabled-apis" => Some(("update.enableK8sBetaApis.enabledApis", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.etag" => Some(("update.etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.gke-auto-upgrade-config.patch-mode" => Some(("update.gkeAutoUpgradeConfig.patchMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.removed-additional-pod-ranges-config.pod-range-names" => Some(("update.removedAdditionalPodRangesConfig.podRangeNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.user-managed-keys-config.aggregation-ca" => Some(("update.userManagedKeysConfig.aggregationCa", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.user-managed-keys-config.cluster-ca" => Some(("update.userManagedKeysConfig.clusterCa", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.user-managed-keys-config.control-plane-disk-encryption-key" => Some(("update.userManagedKeysConfig.controlPlaneDiskEncryptionKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.user-managed-keys-config.control-plane-disk-encryption-key-versions" => Some(("update.userManagedKeysConfig.controlPlaneDiskEncryptionKeyVersions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.user-managed-keys-config.etcd-api-ca" => Some(("update.userManagedKeysConfig.etcdApiCa", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.user-managed-keys-config.etcd-peer-ca" => Some(("update.userManagedKeysConfig.etcdPeerCa", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.user-managed-keys-config.gkeops-etcd-backup-encryption-key" => Some(("update.userManagedKeysConfig.gkeopsEtcdBackupEncryptionKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update.user-managed-keys-config.service-account-signing-keys" => Some(("update.userManagedKeysConfig.serviceAccountSigningKeys", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "update.user-managed-keys-config.service-account-verification-keys" => Some(("update.userManagedKeysConfig.serviceAccountVerificationKeys", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "zone" => Some(("zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["additional-pod-ranges-config", "additive-vpc-scope-dns-domain", "advanced-datapath-observability-config", "aggregation-ca", "allow-external-traffic", "allow-net-admin", "allowed-unsafe-sysctls", "allowlist-paths", "authorized-networks-config", "auto-monitoring-config", "auto-repair", "auto-upgrade", "auto-upgrade-start-time", "autopilot-compatibility-auditing-enabled", "autoprovisioned", "autoprovisioning-locations", "autoprovisioning-node-pool-defaults", "autoscaled-rollout-policy", "autoscaling-profile", "batch-node-count", "batch-percentage", "batch-soak-duration", "bigquery-destination", "blue-green-settings", "boot-disk-kms-key", "cgroup-mode", "channel", "cloud-run-config", "cluster-ca", "cluster-dns", "cluster-dns-domain", "cluster-dns-scope", "cluster-id", "component-config", "config-connector-config", "consumption-metering-config", "container-log-max-files", "container-log-max-size", "control-plane-disk-encryption-key", "control-plane-disk-encryption-key-versions", "cpu-cfs-quota", "cpu-cfs-quota-period", "cpu-manager-policy", "current-state", "dataset-id", "decryption-keys", "default-compute-class-config", "description", "desired-addons-config", "desired-anonymous-authentication-config", "desired-authenticator-groups-config", "desired-auto-ipam-config", "desired-autopilot-workload-policy-config", "desired-binary-authorization", "desired-cluster-autoscaling", "desired-compliance-posture-config", "desired-containerd-config", "desired-control-plane-endpoints-config", "desired-cost-management-config", "desired-database-encryption", "desired-datapath-provider", "desired-default-enable-private-nodes", "desired-default-snat-status", "desired-disable-l4-lb-firewall-reconciliation", "desired-dns-config", "desired-enable-cilium-clusterwide-network-policy", "desired-enable-fqdn-network-policy", "desired-enable-multi-networking", "desired-enable-private-endpoint", "desired-enterprise-config", "desired-fleet", "desired-gateway-api-config", "desired-gcfs-config", "desired-identity-service-config", "desired-image-type", "desired-in-transit-encryption-config", "desired-intra-node-visibility-config", "desired-k8s-beta-apis", "desired-l4ilb-subsetting-config", "desired-locations", "desired-logging-config", "desired-logging-service", "desired-managed-opentelemetry-config", "desired-master-authorized-networks-config", "desired-master-version", "desired-mesh-certificates", "desired-monitoring-config", "desired-monitoring-service", "desired-network-performance-config", "desired-network-tier-config", "desired-node-kubelet-config", "desired-node-pool-auto-config-kubelet-config", "desired-node-pool-auto-config-linux-node-config", "desired-node-pool-auto-config-network-tags", "desired-node-pool-auto-config-resource-manager-tags", "desired-node-pool-autoscaling", "desired-node-pool-id", "desired-node-pool-logging-config", "desired-node-version", "desired-notification-config", "desired-parent-product-config", "desired-pod-autoscaling", "desired-private-cluster-config", "desired-private-ipv6-google-access", "desired-privileged-admission-config", "desired-rbac-binding-config", "desired-release-channel", "desired-resource-usage-export-config", "desired-secret-manager-config", "desired-security-posture-config", "desired-service-external-ips-config", "desired-shielded-nodes", "desired-stack-type", "desired-tier", "desired-user-managed-keys-config", "desired-vertical-pod-autoscaling", "desired-workload-identity-config", "disabled", "disk-size-gb", "disk-type", "dns-cache-config", "dns-endpoint-config", "enable-certificates", "enable-components", "enable-insecure-binding-system-authenticated", "enable-insecure-binding-system-unauthenticated", "enable-integrity-monitoring", "enable-k8s-beta-apis", "enable-k8s-certs-via-dns", "enable-k8s-tokens-via-dns", "enable-legacy-lustre-port", "enable-metrics", "enable-network-egress-metering", "enable-node-autoprovisioning", "enable-private-endpoint", "enable-private-nodes", "enable-public-endpoint", "enable-relay", "enable-secure-boot", "enabled", "enabled-apis", "endpoint", "etag", "etcd-api-ca", "etcd-peer-ca", "evaluation-mode", "event-type", "eviction-max-pod-grace-period-seconds", "eviction-minimum-reclaim", "eviction-soft", "eviction-soft-grace-period", "filter", "gce-persistent-disk-csi-driver-config", "gcp-filestore-csi-driver-config", "gcp-public-cidrs-access-enabled", "gcs-fuse-csi-driver-config", "gke-auto-upgrade-config", "gke-backup-agent-config", "gkeops-etcd-backup-encryption-key", "global-access", "high-scale-checkpointing-config", "horizontal-pod-autoscaling", "hpa-profile", "http-load-balancing", "hugepage-size1g", "hugepage-size2m", "hugepages", "image-gc-high-threshold-percent", "image-gc-low-threshold-percent", "image-maximum-gc-age", "image-minimum-gc-age", "image-type", "imagefs-available", "imagefs-inodes-free", "insecure-kubelet-readonly-port-enabled", "ip-endpoints-config", "key-name", "kubernetes-dashboard", "labels", "load-balancer-type", "location-policy", "lustre-csi-driver-config", "managed-prometheus-config", "management", "master-global-access-config", "master-ipv4-cidr-block", "max-node-count", "max-parallel-image-pulls", "max-surge", "max-unavailable", "membership", "membership-type", "memory-available", "memory-manager", "min-cpu-platform", "min-node-count", "mode", "name", "network-policy-config", "network-tier", "node-kernel-module-loading", "node-pool-soak-duration", "nodefs-available", "nodefs-inodes-free", "oauth-scopes", "parallelstore-csi-driver-config", "patch-mode", "peering-name", "pid-available", "pod-pids-limit", "pod-range-names", "policy", "pre-registered", "private-endpoint", "private-endpoint-enforcement-enabled", "private-endpoint-subnetwork", "private-registry-access-config", "product-name", "project", "project-id", "public-endpoint", "pubsub", "ray-cluster-logging-config", "ray-cluster-monitoring-config", "ray-operator-config", "relay-mode", "removed-additional-pod-ranges-config", "rotation-config", "rotation-interval", "scope", "security-group", "service-account", "service-account-signing-keys", "service-account-verification-keys", "shielded-instance-config", "single-process-oom-kill", "standard-rollout-policy", "state", "stateful-ha-config", "strategy", "sysctls", "tags", "topic", "topology-manager", "total-egress-bandwidth-tier", "total-max-node-count", "total-min-node-count", "transparent-hugepage-defrag", "transparent-hugepage-enabled", "update", "upgrade-options", "upgrade-settings", "user-managed-keys-config", "variant", "variant-config", "vulnerability-mode", "wait-for-drain-duration", "workload-pool", "writable-cgroups", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::UpdateClusterRequest = serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_clusters_update(
            request,
            opt.value_of("project-id").unwrap_or(""),
            opt.value_of("zone").unwrap_or(""),
            opt.value_of("cluster-id").unwrap_or(""),
        );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_zones_get_serverconfig(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.projects().zones_get_serverconfig(
            opt.value_of("project-id").unwrap_or(""),
            opt.value_of("zone").unwrap_or(""),
        );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "name" => {
                    call = call.name(value.unwrap_or(""));
                }
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v.extend(["name"].iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_zones_operations_cancel(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut field_cursor = FieldCursor::default();
        let mut object = serde_json::value::Value::Object(Default::default());

        for kvarg in opt
            .values_of("kv")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
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

            let type_info: Option<(&'static str, JsonTypeInfo)> = match &temp_cursor.to_string()[..]
            {
                "name" => Some((
                    "name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "operation-id" => Some((
                    "operationId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "project-id" => Some((
                    "projectId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "zone" => Some((
                    "zone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec!["name", "operation-id", "project-id", "zone"],
                    );
                    err.issues.push(CLIError::Field(FieldError::Unknown(
                        temp_cursor.to_string(),
                        suggestion,
                        value.map(|v| v.to_string()),
                    )));
                    None
                }
            };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(
                    &mut object,
                    value.unwrap(),
                    type_info,
                    err,
                    &temp_cursor,
                );
            }
        }
        let mut request: api::CancelOperationRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().zones_operations_cancel(
            request,
            opt.value_of("project-id").unwrap_or(""),
            opt.value_of("zone").unwrap_or(""),
            opt.value_of("operation-id").unwrap_or(""),
        );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_zones_operations_get(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.projects().zones_operations_get(
            opt.value_of("project-id").unwrap_or(""),
            opt.value_of("zone").unwrap_or(""),
            opt.value_of("operation-id").unwrap_or(""),
        );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "name" => {
                    call = call.name(value.unwrap_or(""));
                }
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v.extend(["name"].iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _projects_zones_operations_list(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.projects().zones_operations_list(
            opt.value_of("project-id").unwrap_or(""),
            opt.value_of("zone").unwrap_or(""),
        );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "parent" => {
                    call = call.parent(value.unwrap_or(""));
                }
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(
                                self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1,
                                value.unwrap_or("unset"),
                            );
                            break;
                        }
                    }
                    if !found {
                        err.issues
                            .push(CLIError::UnknownParameter(key.to_string(), {
                                let mut v = Vec::new();
                                v.extend(self.gp.iter().map(|v| *v));
                                v.extend(["parent"].iter().map(|v| *v));
                                v
                            }));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self
                .opt
                .values_of("url")
                .map(|i| i.collect())
                .unwrap_or(Vec::new())
                .iter()
            {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => {
                    return Err(DoitError::IoError(
                        opt.value_of("out").unwrap_or("-").to_string(),
                        io_err,
                    ))
                }
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!(),
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value =
                        serde_json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    serde_json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _doit(
        &self,
        dry_run: bool,
    ) -> Result<Result<(), DoitError>, Option<InvalidOptionsError>> {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Result<(), DoitError> = Ok(());
        let mut err_opt: Option<InvalidOptionsError> = None;
        match self.opt.subcommand() {
            ("projects", Some(opt)) => match opt.subcommand() {
                ("aggregated-usable-subnetworks-list", Some(opt)) => {
                    call_result = self
                        ._projects_aggregated_usable_subnetworks_list(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-clusters-check-autopilot-compatibility", Some(opt)) => {
                    call_result = self
                        ._projects_locations_clusters_check_autopilot_compatibility(
                            opt, dry_run, &mut err,
                        )
                        .await;
                }
                ("locations-clusters-complete-ip-rotation", Some(opt)) => {
                    call_result = self
                        ._projects_locations_clusters_complete_ip_rotation(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-clusters-create", Some(opt)) => {
                    call_result = self
                        ._projects_locations_clusters_create(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-clusters-delete", Some(opt)) => {
                    call_result = self
                        ._projects_locations_clusters_delete(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-clusters-fetch-cluster-upgrade-info", Some(opt)) => {
                    call_result = self
                        ._projects_locations_clusters_fetch_cluster_upgrade_info(
                            opt, dry_run, &mut err,
                        )
                        .await;
                }
                ("locations-clusters-get", Some(opt)) => {
                    call_result = self
                        ._projects_locations_clusters_get(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-clusters-get-jwks", Some(opt)) => {
                    call_result = self
                        ._projects_locations_clusters_get_jwks(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-clusters-list", Some(opt)) => {
                    call_result = self
                        ._projects_locations_clusters_list(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-clusters-node-pools-complete-upgrade", Some(opt)) => {
                    call_result = self
                        ._projects_locations_clusters_node_pools_complete_upgrade(
                            opt, dry_run, &mut err,
                        )
                        .await;
                }
                ("locations-clusters-node-pools-create", Some(opt)) => {
                    call_result = self
                        ._projects_locations_clusters_node_pools_create(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-clusters-node-pools-delete", Some(opt)) => {
                    call_result = self
                        ._projects_locations_clusters_node_pools_delete(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-clusters-node-pools-fetch-node-pool-upgrade-info", Some(opt)) => {
                    call_result = self
                        ._projects_locations_clusters_node_pools_fetch_node_pool_upgrade_info(
                            opt, dry_run, &mut err,
                        )
                        .await;
                }
                ("locations-clusters-node-pools-get", Some(opt)) => {
                    call_result = self
                        ._projects_locations_clusters_node_pools_get(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-clusters-node-pools-list", Some(opt)) => {
                    call_result = self
                        ._projects_locations_clusters_node_pools_list(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-clusters-node-pools-rollback", Some(opt)) => {
                    call_result = self
                        ._projects_locations_clusters_node_pools_rollback(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-clusters-node-pools-set-autoscaling", Some(opt)) => {
                    call_result = self
                        ._projects_locations_clusters_node_pools_set_autoscaling(
                            opt, dry_run, &mut err,
                        )
                        .await;
                }
                ("locations-clusters-node-pools-set-management", Some(opt)) => {
                    call_result = self
                        ._projects_locations_clusters_node_pools_set_management(
                            opt, dry_run, &mut err,
                        )
                        .await;
                }
                ("locations-clusters-node-pools-set-size", Some(opt)) => {
                    call_result = self
                        ._projects_locations_clusters_node_pools_set_size(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-clusters-node-pools-update", Some(opt)) => {
                    call_result = self
                        ._projects_locations_clusters_node_pools_update(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-clusters-set-addons", Some(opt)) => {
                    call_result = self
                        ._projects_locations_clusters_set_addons(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-clusters-set-legacy-abac", Some(opt)) => {
                    call_result = self
                        ._projects_locations_clusters_set_legacy_abac(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-clusters-set-locations", Some(opt)) => {
                    call_result = self
                        ._projects_locations_clusters_set_locations(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-clusters-set-logging", Some(opt)) => {
                    call_result = self
                        ._projects_locations_clusters_set_logging(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-clusters-set-maintenance-policy", Some(opt)) => {
                    call_result = self
                        ._projects_locations_clusters_set_maintenance_policy(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-clusters-set-master-auth", Some(opt)) => {
                    call_result = self
                        ._projects_locations_clusters_set_master_auth(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-clusters-set-monitoring", Some(opt)) => {
                    call_result = self
                        ._projects_locations_clusters_set_monitoring(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-clusters-set-network-policy", Some(opt)) => {
                    call_result = self
                        ._projects_locations_clusters_set_network_policy(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-clusters-set-resource-labels", Some(opt)) => {
                    call_result = self
                        ._projects_locations_clusters_set_resource_labels(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-clusters-start-ip-rotation", Some(opt)) => {
                    call_result = self
                        ._projects_locations_clusters_start_ip_rotation(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-clusters-update", Some(opt)) => {
                    call_result = self
                        ._projects_locations_clusters_update(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-clusters-update-master", Some(opt)) => {
                    call_result = self
                        ._projects_locations_clusters_update_master(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-clusters-well-known-get-openid-configuration", Some(opt)) => {
                    call_result = self
                        ._projects_locations_clusters_well_known_get_openid_configuration(
                            opt, dry_run, &mut err,
                        )
                        .await;
                }
                ("locations-get-server-config", Some(opt)) => {
                    call_result = self
                        ._projects_locations_get_server_config(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-operations-cancel", Some(opt)) => {
                    call_result = self
                        ._projects_locations_operations_cancel(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-operations-get", Some(opt)) => {
                    call_result = self
                        ._projects_locations_operations_get(opt, dry_run, &mut err)
                        .await;
                }
                ("locations-operations-list", Some(opt)) => {
                    call_result = self
                        ._projects_locations_operations_list(opt, dry_run, &mut err)
                        .await;
                }
                ("zones-clusters-addons", Some(opt)) => {
                    call_result = self
                        ._projects_zones_clusters_addons(opt, dry_run, &mut err)
                        .await;
                }
                ("zones-clusters-complete-ip-rotation", Some(opt)) => {
                    call_result = self
                        ._projects_zones_clusters_complete_ip_rotation(opt, dry_run, &mut err)
                        .await;
                }
                ("zones-clusters-create", Some(opt)) => {
                    call_result = self
                        ._projects_zones_clusters_create(opt, dry_run, &mut err)
                        .await;
                }
                ("zones-clusters-delete", Some(opt)) => {
                    call_result = self
                        ._projects_zones_clusters_delete(opt, dry_run, &mut err)
                        .await;
                }
                ("zones-clusters-fetch-cluster-upgrade-info", Some(opt)) => {
                    call_result = self
                        ._projects_zones_clusters_fetch_cluster_upgrade_info(opt, dry_run, &mut err)
                        .await;
                }
                ("zones-clusters-get", Some(opt)) => {
                    call_result = self
                        ._projects_zones_clusters_get(opt, dry_run, &mut err)
                        .await;
                }
                ("zones-clusters-legacy-abac", Some(opt)) => {
                    call_result = self
                        ._projects_zones_clusters_legacy_abac(opt, dry_run, &mut err)
                        .await;
                }
                ("zones-clusters-list", Some(opt)) => {
                    call_result = self
                        ._projects_zones_clusters_list(opt, dry_run, &mut err)
                        .await;
                }
                ("zones-clusters-locations", Some(opt)) => {
                    call_result = self
                        ._projects_zones_clusters_locations(opt, dry_run, &mut err)
                        .await;
                }
                ("zones-clusters-logging", Some(opt)) => {
                    call_result = self
                        ._projects_zones_clusters_logging(opt, dry_run, &mut err)
                        .await;
                }
                ("zones-clusters-master", Some(opt)) => {
                    call_result = self
                        ._projects_zones_clusters_master(opt, dry_run, &mut err)
                        .await;
                }
                ("zones-clusters-monitoring", Some(opt)) => {
                    call_result = self
                        ._projects_zones_clusters_monitoring(opt, dry_run, &mut err)
                        .await;
                }
                ("zones-clusters-node-pools-autoscaling", Some(opt)) => {
                    call_result = self
                        ._projects_zones_clusters_node_pools_autoscaling(opt, dry_run, &mut err)
                        .await;
                }
                ("zones-clusters-node-pools-create", Some(opt)) => {
                    call_result = self
                        ._projects_zones_clusters_node_pools_create(opt, dry_run, &mut err)
                        .await;
                }
                ("zones-clusters-node-pools-delete", Some(opt)) => {
                    call_result = self
                        ._projects_zones_clusters_node_pools_delete(opt, dry_run, &mut err)
                        .await;
                }
                ("zones-clusters-node-pools-fetch-node-pool-upgrade-info", Some(opt)) => {
                    call_result = self
                        ._projects_zones_clusters_node_pools_fetch_node_pool_upgrade_info(
                            opt, dry_run, &mut err,
                        )
                        .await;
                }
                ("zones-clusters-node-pools-get", Some(opt)) => {
                    call_result = self
                        ._projects_zones_clusters_node_pools_get(opt, dry_run, &mut err)
                        .await;
                }
                ("zones-clusters-node-pools-list", Some(opt)) => {
                    call_result = self
                        ._projects_zones_clusters_node_pools_list(opt, dry_run, &mut err)
                        .await;
                }
                ("zones-clusters-node-pools-rollback", Some(opt)) => {
                    call_result = self
                        ._projects_zones_clusters_node_pools_rollback(opt, dry_run, &mut err)
                        .await;
                }
                ("zones-clusters-node-pools-set-management", Some(opt)) => {
                    call_result = self
                        ._projects_zones_clusters_node_pools_set_management(opt, dry_run, &mut err)
                        .await;
                }
                ("zones-clusters-node-pools-set-size", Some(opt)) => {
                    call_result = self
                        ._projects_zones_clusters_node_pools_set_size(opt, dry_run, &mut err)
                        .await;
                }
                ("zones-clusters-node-pools-update", Some(opt)) => {
                    call_result = self
                        ._projects_zones_clusters_node_pools_update(opt, dry_run, &mut err)
                        .await;
                }
                ("zones-clusters-resource-labels", Some(opt)) => {
                    call_result = self
                        ._projects_zones_clusters_resource_labels(opt, dry_run, &mut err)
                        .await;
                }
                ("zones-clusters-set-maintenance-policy", Some(opt)) => {
                    call_result = self
                        ._projects_zones_clusters_set_maintenance_policy(opt, dry_run, &mut err)
                        .await;
                }
                ("zones-clusters-set-master-auth", Some(opt)) => {
                    call_result = self
                        ._projects_zones_clusters_set_master_auth(opt, dry_run, &mut err)
                        .await;
                }
                ("zones-clusters-set-network-policy", Some(opt)) => {
                    call_result = self
                        ._projects_zones_clusters_set_network_policy(opt, dry_run, &mut err)
                        .await;
                }
                ("zones-clusters-start-ip-rotation", Some(opt)) => {
                    call_result = self
                        ._projects_zones_clusters_start_ip_rotation(opt, dry_run, &mut err)
                        .await;
                }
                ("zones-clusters-update", Some(opt)) => {
                    call_result = self
                        ._projects_zones_clusters_update(opt, dry_run, &mut err)
                        .await;
                }
                ("zones-get-serverconfig", Some(opt)) => {
                    call_result = self
                        ._projects_zones_get_serverconfig(opt, dry_run, &mut err)
                        .await;
                }
                ("zones-operations-cancel", Some(opt)) => {
                    call_result = self
                        ._projects_zones_operations_cancel(opt, dry_run, &mut err)
                        .await;
                }
                ("zones-operations-get", Some(opt)) => {
                    call_result = self
                        ._projects_zones_operations_get(opt, dry_run, &mut err)
                        .await;
                }
                ("zones-operations-list", Some(opt)) => {
                    call_result = self
                        ._projects_zones_operations_list(opt, dry_run, &mut err)
                        .await;
                }
                _ => {
                    err.issues
                        .push(CLIError::MissingMethodError("projects".to_string()));
                    writeln!(std::io::stderr(), "{}\n", opt.usage()).ok();
                }
            },
            _ => {
                err.issues.push(CLIError::MissingCommandError);
                writeln!(std::io::stderr(), "{}\n", self.opt.usage()).ok();
            }
        }

        if dry_run {
            if !err.issues.is_empty() {
                err_opt = Some(err);
            }
            Err(err_opt)
        } else {
            Ok(call_result)
        }
    }

    // Please note that this call will fail if any part of the opt can't be handled
    async fn new(opt: ArgMatches<'n>, connector: C) -> Result<Engine<'n, C>, InvalidOptionsError> {
        let (config_dir, secret) = {
            let config_dir = match common::assure_config_dir_exists(
                opt.value_of("folder").unwrap_or("~/.google-service-cli"),
            ) {
                Err(e) => return Err(InvalidOptionsError::single(e, 3)),
                Ok(p) => p,
            };

            match common::application_secret_from_directory(&config_dir, "container1-secret.json",
                                                         "{\"installed\":{\"auth_uri\":\"https://accounts.google.com/o/oauth2/auth\",\"client_secret\":\"hCsslbCUyfehWMmbkG8vTYxG\",\"token_uri\":\"https://accounts.google.com/o/oauth2/token\",\"client_email\":\"\",\"redirect_uris\":[\"urn:ietf:wg:oauth:2.0:oob\",\"oob\"],\"client_x509_cert_url\":\"\",\"client_id\":\"620010449518-9ngf7o4dhs0dka470npqvor6dc5lqb9b.apps.googleusercontent.com\",\"auth_provider_x509_cert_url\":\"https://www.googleapis.com/oauth2/v1/certs\"}}") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let executor = hyper_util::rt::TokioExecutor::new();
        let client =
            hyper_util::client::legacy::Client::builder(executor.clone()).build(connector.clone());

        let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
            secret,
            yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
            yup_oauth2::client::CustomHyperClientBuilder::from(
                hyper_util::client::legacy::Client::builder(executor).build(connector),
            ),
        )
        .persist_tokens_to_disk(format!("{}/container1", config_dir))
        .build()
        .await
        .unwrap();

        let engine = Engine {
            opt,
            hub: api::Container::new(client, auth),
            gp: vec![
                "$-xgafv",
                "access-token",
                "alt",
                "callback",
                "fields",
                "key",
                "oauth-token",
                "pretty-print",
                "quota-user",
                "upload-type",
                "upload-protocol",
            ],
            gpm: vec![
                ("$-xgafv", "$.xgafv"),
                ("access-token", "access_token"),
                ("oauth-token", "oauth_token"),
                ("pretty-print", "prettyPrint"),
                ("quota-user", "quotaUser"),
                ("upload-type", "uploadType"),
                ("upload-protocol", "upload_protocol"),
            ],
        };

        match engine._doit(true).await {
            Err(Some(err)) => Err(err),
            Err(None) => Ok(engine),
            Ok(_) => unreachable!(),
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
        ("projects", "methods: 'aggregated-usable-subnetworks-list', 'locations-clusters-check-autopilot-compatibility', 'locations-clusters-complete-ip-rotation', 'locations-clusters-create', 'locations-clusters-delete', 'locations-clusters-fetch-cluster-upgrade-info', 'locations-clusters-get', 'locations-clusters-get-jwks', 'locations-clusters-list', 'locations-clusters-node-pools-complete-upgrade', 'locations-clusters-node-pools-create', 'locations-clusters-node-pools-delete', 'locations-clusters-node-pools-fetch-node-pool-upgrade-info', 'locations-clusters-node-pools-get', 'locations-clusters-node-pools-list', 'locations-clusters-node-pools-rollback', 'locations-clusters-node-pools-set-autoscaling', 'locations-clusters-node-pools-set-management', 'locations-clusters-node-pools-set-size', 'locations-clusters-node-pools-update', 'locations-clusters-set-addons', 'locations-clusters-set-legacy-abac', 'locations-clusters-set-locations', 'locations-clusters-set-logging', 'locations-clusters-set-maintenance-policy', 'locations-clusters-set-master-auth', 'locations-clusters-set-monitoring', 'locations-clusters-set-network-policy', 'locations-clusters-set-resource-labels', 'locations-clusters-start-ip-rotation', 'locations-clusters-update', 'locations-clusters-update-master', 'locations-clusters-well-known-get-openid-configuration', 'locations-get-server-config', 'locations-operations-cancel', 'locations-operations-get', 'locations-operations-list', 'zones-clusters-addons', 'zones-clusters-complete-ip-rotation', 'zones-clusters-create', 'zones-clusters-delete', 'zones-clusters-fetch-cluster-upgrade-info', 'zones-clusters-get', 'zones-clusters-legacy-abac', 'zones-clusters-list', 'zones-clusters-locations', 'zones-clusters-logging', 'zones-clusters-master', 'zones-clusters-monitoring', 'zones-clusters-node-pools-autoscaling', 'zones-clusters-node-pools-create', 'zones-clusters-node-pools-delete', 'zones-clusters-node-pools-fetch-node-pool-upgrade-info', 'zones-clusters-node-pools-get', 'zones-clusters-node-pools-list', 'zones-clusters-node-pools-rollback', 'zones-clusters-node-pools-set-management', 'zones-clusters-node-pools-set-size', 'zones-clusters-node-pools-update', 'zones-clusters-resource-labels', 'zones-clusters-set-maintenance-policy', 'zones-clusters-set-master-auth', 'zones-clusters-set-network-policy', 'zones-clusters-start-ip-rotation', 'zones-clusters-update', 'zones-get-serverconfig', 'zones-operations-cancel', 'zones-operations-get' and 'zones-operations-list'", vec![
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
                    Some(r##"Creates a cluster, consisting of the specified number and type of Google Compute Engine instances. By default, the cluster is created in the project's [default network](https://cloud.google.com/compute/docs/networks-and-firewalls#networks). One firewall is added for the cluster. After cluster creation, the kubelet creates routes for each node to allow the containers on that node to communicate with all other instances in the cluster. Finally, an entry is added to the project's global metadata indicating which CIDR range the cluster is using."##),
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
            ("locations-clusters-fetch-cluster-upgrade-info",
                    Some(r##"Fetch upgrade information of a specific cluster."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-fetch-cluster-upgrade-info",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name (project, location, cluster) of the cluster to get. Specified in the format `projects/*/locations/*/clusters/*` or `projects/*/zones/*/clusters/*`."##),
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
            ("locations-clusters-node-pools-fetch-node-pool-upgrade-info",
                    Some(r##"Fetch upgrade information of a specific nodepool."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_locations-clusters-node-pools-fetch-node-pool-upgrade-info",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name (project, location, cluster, nodepool) of the nodepool to get. Specified in the format `projects/*/locations/*/clusters/*/nodePools/*` or `projects/*/zones/*/clusters/*/nodePools/*`."##),
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
                    Some(r##"Creates a cluster, consisting of the specified number and type of Google Compute Engine instances. By default, the cluster is created in the project's [default network](https://cloud.google.com/compute/docs/networks-and-firewalls#networks). One firewall is added for the cluster. After cluster creation, the kubelet creates routes for each node to allow the containers on that node to communicate with all other instances in the cluster. Finally, an entry is added to the project's global metadata indicating which CIDR range the cluster is using."##),
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
            ("zones-clusters-fetch-cluster-upgrade-info",
                    Some(r##"Fetch upgrade information of a specific cluster."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-clusters-fetch-cluster-upgrade-info",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name (project, location, cluster) of the cluster to get. Specified in the format `projects/*/locations/*/clusters/*` or `projects/*/zones/*/clusters/*`."##),
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
            ("zones-clusters-node-pools-fetch-node-pool-upgrade-info",
                    Some(r##"Fetch upgrade information of a specific nodepool."##),
                    "Details at http://byron.github.io/google-apis-rs/google_container1_cli/projects_zones-clusters-node-pools-fetch-node-pool-upgrade-info",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name (project, location, cluster, nodepool) of the nodepool to get. Specified in the format `projects/*/locations/*/clusters/*/nodePools/*` or `projects/*/zones/*/clusters/*/nodePools/*`."##),
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
           .version("7.0.0+20251216")
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
                let arg_name_str = match (arg_name, flag) {
                    (&Some(an), _) => an,
                    (_, &Some(f)) => f,
                    _ => unreachable!(),
                };
                let mut arg = Arg::with_name(arg_name_str).empty_values(false);
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
    let connector = hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .unwrap()
        .https_or_http()
        .enable_http2()
        .build();

    match Engine::new(matches, connector).await {
        Err(err) => {
            exit_status = err.exit_code;
            writeln!(std::io::stderr(), "{}", err).ok();
        }
        Ok(engine) => {
            if let Err(doit_err) = engine.doit().await {
                exit_status = 1;
                match doit_err {
                    DoitError::IoError(path, err) => {
                        writeln!(
                            std::io::stderr(),
                            "Failed to open output file '{}': {}",
                            path,
                            err
                        )
                        .ok();
                    }
                    DoitError::ApiError(err) => {
                        if debug {
                            writeln!(std::io::stderr(), "{:#?}", err).ok();
                        } else {
                            writeln!(std::io::stderr(), "{}", err).ok();
                        }
                    }
                }
            }
        }
    }

    std::process::exit(exit_status);
}
