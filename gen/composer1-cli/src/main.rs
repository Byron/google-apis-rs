// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_composer1::{api, Error, oauth2, client::chrono, FieldMask};


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
    hub: api::CloudComposer<S>,
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
    async fn _projects_locations_environments_check_upgrade(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "image-version" => Some(("imageVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["image-version"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::CheckUpgradeRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_environments_check_upgrade(request, opt.value_of("environment").unwrap_or(""));
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

    async fn _projects_locations_environments_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "config.airflow-byoid-uri" => Some(("config.airflowByoidUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.airflow-uri" => Some(("config.airflowUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.dag-gcs-prefix" => Some(("config.dagGcsPrefix", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.data-retention-config.airflow-metadata-retention-config.retention-days" => Some(("config.dataRetentionConfig.airflowMetadataRetentionConfig.retentionDays", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.data-retention-config.airflow-metadata-retention-config.retention-mode" => Some(("config.dataRetentionConfig.airflowMetadataRetentionConfig.retentionMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.data-retention-config.task-logs-retention-config.storage-mode" => Some(("config.dataRetentionConfig.taskLogsRetentionConfig.storageMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.database-config.machine-type" => Some(("config.databaseConfig.machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.database-config.zone" => Some(("config.databaseConfig.zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.encryption-config.kms-key-name" => Some(("config.encryptionConfig.kmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.environment-size" => Some(("config.environmentSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.gke-cluster" => Some(("config.gkeCluster", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.maintenance-window.end-time" => Some(("config.maintenanceWindow.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.maintenance-window.recurrence" => Some(("config.maintenanceWindow.recurrence", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.maintenance-window.start-time" => Some(("config.maintenanceWindow.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.master-authorized-networks-config.enabled" => Some(("config.masterAuthorizedNetworksConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.node-config.composer-internal-ipv4-cidr-block" => Some(("config.nodeConfig.composerInternalIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.node-config.composer-network-attachment" => Some(("config.nodeConfig.composerNetworkAttachment", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.node-config.disk-size-gb" => Some(("config.nodeConfig.diskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.node-config.enable-ip-masq-agent" => Some(("config.nodeConfig.enableIpMasqAgent", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.node-config.ip-allocation-policy.cluster-ipv4-cidr-block" => Some(("config.nodeConfig.ipAllocationPolicy.clusterIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.node-config.ip-allocation-policy.cluster-secondary-range-name" => Some(("config.nodeConfig.ipAllocationPolicy.clusterSecondaryRangeName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.node-config.ip-allocation-policy.services-ipv4-cidr-block" => Some(("config.nodeConfig.ipAllocationPolicy.servicesIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.node-config.ip-allocation-policy.services-secondary-range-name" => Some(("config.nodeConfig.ipAllocationPolicy.servicesSecondaryRangeName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.node-config.ip-allocation-policy.use-ip-aliases" => Some(("config.nodeConfig.ipAllocationPolicy.useIpAliases", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.node-config.location" => Some(("config.nodeConfig.location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.node-config.machine-type" => Some(("config.nodeConfig.machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.node-config.network" => Some(("config.nodeConfig.network", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.node-config.oauth-scopes" => Some(("config.nodeConfig.oauthScopes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.node-config.service-account" => Some(("config.nodeConfig.serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.node-config.subnetwork" => Some(("config.nodeConfig.subnetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.node-config.tags" => Some(("config.nodeConfig.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.node-count" => Some(("config.nodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.private-environment-config.cloud-composer-connection-subnetwork" => Some(("config.privateEnvironmentConfig.cloudComposerConnectionSubnetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.private-environment-config.cloud-composer-network-ipv4-cidr-block" => Some(("config.privateEnvironmentConfig.cloudComposerNetworkIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.private-environment-config.cloud-composer-network-ipv4-reserved-range" => Some(("config.privateEnvironmentConfig.cloudComposerNetworkIpv4ReservedRange", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.private-environment-config.cloud-sql-ipv4-cidr-block" => Some(("config.privateEnvironmentConfig.cloudSqlIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.private-environment-config.enable-private-builds-only" => Some(("config.privateEnvironmentConfig.enablePrivateBuildsOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.private-environment-config.enable-private-environment" => Some(("config.privateEnvironmentConfig.enablePrivateEnvironment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.private-environment-config.enable-privately-used-public-ips" => Some(("config.privateEnvironmentConfig.enablePrivatelyUsedPublicIps", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.private-environment-config.networking-config.connection-type" => Some(("config.privateEnvironmentConfig.networkingConfig.connectionType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.private-environment-config.private-cluster-config.enable-private-endpoint" => Some(("config.privateEnvironmentConfig.privateClusterConfig.enablePrivateEndpoint", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.private-environment-config.private-cluster-config.master-ipv4-cidr-block" => Some(("config.privateEnvironmentConfig.privateClusterConfig.masterIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.private-environment-config.private-cluster-config.master-ipv4-reserved-range" => Some(("config.privateEnvironmentConfig.privateClusterConfig.masterIpv4ReservedRange", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.private-environment-config.web-server-ipv4-cidr-block" => Some(("config.privateEnvironmentConfig.webServerIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.private-environment-config.web-server-ipv4-reserved-range" => Some(("config.privateEnvironmentConfig.webServerIpv4ReservedRange", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.recovery-config.scheduled-snapshots-config.enabled" => Some(("config.recoveryConfig.scheduledSnapshotsConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.recovery-config.scheduled-snapshots-config.snapshot-creation-schedule" => Some(("config.recoveryConfig.scheduledSnapshotsConfig.snapshotCreationSchedule", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.recovery-config.scheduled-snapshots-config.snapshot-location" => Some(("config.recoveryConfig.scheduledSnapshotsConfig.snapshotLocation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.recovery-config.scheduled-snapshots-config.time-zone" => Some(("config.recoveryConfig.scheduledSnapshotsConfig.timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.resilience-mode" => Some(("config.resilienceMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.software-config.airflow-config-overrides" => Some(("config.softwareConfig.airflowConfigOverrides", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "config.software-config.cloud-data-lineage-integration.enabled" => Some(("config.softwareConfig.cloudDataLineageIntegration.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.software-config.env-variables" => Some(("config.softwareConfig.envVariables", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "config.software-config.image-version" => Some(("config.softwareConfig.imageVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.software-config.pypi-packages" => Some(("config.softwareConfig.pypiPackages", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "config.software-config.python-version" => Some(("config.softwareConfig.pythonVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.software-config.scheduler-count" => Some(("config.softwareConfig.schedulerCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.software-config.web-server-plugins-mode" => Some(("config.softwareConfig.webServerPluginsMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.web-server-config.machine-type" => Some(("config.webServerConfig.machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.workloads-config.dag-processor.count" => Some(("config.workloadsConfig.dagProcessor.count", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.workloads-config.dag-processor.cpu" => Some(("config.workloadsConfig.dagProcessor.cpu", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "config.workloads-config.dag-processor.memory-gb" => Some(("config.workloadsConfig.dagProcessor.memoryGb", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "config.workloads-config.dag-processor.storage-gb" => Some(("config.workloadsConfig.dagProcessor.storageGb", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "config.workloads-config.scheduler.count" => Some(("config.workloadsConfig.scheduler.count", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.workloads-config.scheduler.cpu" => Some(("config.workloadsConfig.scheduler.cpu", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "config.workloads-config.scheduler.memory-gb" => Some(("config.workloadsConfig.scheduler.memoryGb", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "config.workloads-config.scheduler.storage-gb" => Some(("config.workloadsConfig.scheduler.storageGb", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "config.workloads-config.triggerer.count" => Some(("config.workloadsConfig.triggerer.count", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.workloads-config.triggerer.cpu" => Some(("config.workloadsConfig.triggerer.cpu", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "config.workloads-config.triggerer.memory-gb" => Some(("config.workloadsConfig.triggerer.memoryGb", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "config.workloads-config.web-server.cpu" => Some(("config.workloadsConfig.webServer.cpu", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "config.workloads-config.web-server.memory-gb" => Some(("config.workloadsConfig.webServer.memoryGb", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "config.workloads-config.web-server.storage-gb" => Some(("config.workloadsConfig.webServer.storageGb", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "config.workloads-config.worker.cpu" => Some(("config.workloadsConfig.worker.cpu", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "config.workloads-config.worker.max-count" => Some(("config.workloadsConfig.worker.maxCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.workloads-config.worker.memory-gb" => Some(("config.workloadsConfig.worker.memoryGb", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "config.workloads-config.worker.min-count" => Some(("config.workloadsConfig.worker.minCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.workloads-config.worker.storage-gb" => Some(("config.workloadsConfig.worker.storageGb", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "satisfies-pzs" => Some(("satisfiesPzs", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "storage-config.bucket" => Some(("storageConfig.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uuid" => Some(("uuid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["airflow-byoid-uri", "airflow-config-overrides", "airflow-metadata-retention-config", "airflow-uri", "bucket", "cloud-composer-connection-subnetwork", "cloud-composer-network-ipv4-cidr-block", "cloud-composer-network-ipv4-reserved-range", "cloud-data-lineage-integration", "cloud-sql-ipv4-cidr-block", "cluster-ipv4-cidr-block", "cluster-secondary-range-name", "composer-internal-ipv4-cidr-block", "composer-network-attachment", "config", "connection-type", "count", "cpu", "create-time", "dag-gcs-prefix", "dag-processor", "data-retention-config", "database-config", "disk-size-gb", "enable-ip-masq-agent", "enable-private-builds-only", "enable-private-endpoint", "enable-private-environment", "enable-privately-used-public-ips", "enabled", "encryption-config", "end-time", "env-variables", "environment-size", "gke-cluster", "image-version", "ip-allocation-policy", "kms-key-name", "labels", "location", "machine-type", "maintenance-window", "master-authorized-networks-config", "master-ipv4-cidr-block", "master-ipv4-reserved-range", "max-count", "memory-gb", "min-count", "name", "network", "networking-config", "node-config", "node-count", "oauth-scopes", "private-cluster-config", "private-environment-config", "pypi-packages", "python-version", "recovery-config", "recurrence", "resilience-mode", "retention-days", "retention-mode", "satisfies-pzs", "scheduled-snapshots-config", "scheduler", "scheduler-count", "service-account", "services-ipv4-cidr-block", "services-secondary-range-name", "snapshot-creation-schedule", "snapshot-location", "software-config", "start-time", "state", "storage-config", "storage-gb", "storage-mode", "subnetwork", "tags", "task-logs-retention-config", "time-zone", "triggerer", "update-time", "use-ip-aliases", "uuid", "web-server", "web-server-config", "web-server-ipv4-cidr-block", "web-server-ipv4-reserved-range", "web-server-plugins-mode", "worker", "workloads-config", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Environment = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_environments_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_environments_database_failover(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
        let mut request: api::DatabaseFailoverRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_environments_database_failover(request, opt.value_of("environment").unwrap_or(""));
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

    async fn _projects_locations_environments_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_environments_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_environments_execute_airflow_command(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "command" => Some(("command", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "parameters" => Some(("parameters", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "subcommand" => Some(("subcommand", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["command", "parameters", "subcommand"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ExecuteAirflowCommandRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_environments_execute_airflow_command(request, opt.value_of("environment").unwrap_or(""));
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

    async fn _projects_locations_environments_fetch_database_properties(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_environments_fetch_database_properties(opt.value_of("environment").unwrap_or(""));
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

    async fn _projects_locations_environments_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_environments_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_environments_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_environments_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["page-size", "page-token"].iter().map(|v|*v));
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

    async fn _projects_locations_environments_load_snapshot(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "skip-airflow-overrides-setting" => Some(("skipAirflowOverridesSetting", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "skip-environment-variables-setting" => Some(("skipEnvironmentVariablesSetting", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "skip-gcs-data-copying" => Some(("skipGcsDataCopying", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "skip-pypi-packages-installation" => Some(("skipPypiPackagesInstallation", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "snapshot-path" => Some(("snapshotPath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["skip-airflow-overrides-setting", "skip-environment-variables-setting", "skip-gcs-data-copying", "skip-pypi-packages-installation", "snapshot-path"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::LoadSnapshotRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_environments_load_snapshot(request, opt.value_of("environment").unwrap_or(""));
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

    async fn _projects_locations_environments_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "config.airflow-byoid-uri" => Some(("config.airflowByoidUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.airflow-uri" => Some(("config.airflowUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.dag-gcs-prefix" => Some(("config.dagGcsPrefix", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.data-retention-config.airflow-metadata-retention-config.retention-days" => Some(("config.dataRetentionConfig.airflowMetadataRetentionConfig.retentionDays", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.data-retention-config.airflow-metadata-retention-config.retention-mode" => Some(("config.dataRetentionConfig.airflowMetadataRetentionConfig.retentionMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.data-retention-config.task-logs-retention-config.storage-mode" => Some(("config.dataRetentionConfig.taskLogsRetentionConfig.storageMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.database-config.machine-type" => Some(("config.databaseConfig.machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.database-config.zone" => Some(("config.databaseConfig.zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.encryption-config.kms-key-name" => Some(("config.encryptionConfig.kmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.environment-size" => Some(("config.environmentSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.gke-cluster" => Some(("config.gkeCluster", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.maintenance-window.end-time" => Some(("config.maintenanceWindow.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.maintenance-window.recurrence" => Some(("config.maintenanceWindow.recurrence", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.maintenance-window.start-time" => Some(("config.maintenanceWindow.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.master-authorized-networks-config.enabled" => Some(("config.masterAuthorizedNetworksConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.node-config.composer-internal-ipv4-cidr-block" => Some(("config.nodeConfig.composerInternalIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.node-config.composer-network-attachment" => Some(("config.nodeConfig.composerNetworkAttachment", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.node-config.disk-size-gb" => Some(("config.nodeConfig.diskSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.node-config.enable-ip-masq-agent" => Some(("config.nodeConfig.enableIpMasqAgent", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.node-config.ip-allocation-policy.cluster-ipv4-cidr-block" => Some(("config.nodeConfig.ipAllocationPolicy.clusterIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.node-config.ip-allocation-policy.cluster-secondary-range-name" => Some(("config.nodeConfig.ipAllocationPolicy.clusterSecondaryRangeName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.node-config.ip-allocation-policy.services-ipv4-cidr-block" => Some(("config.nodeConfig.ipAllocationPolicy.servicesIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.node-config.ip-allocation-policy.services-secondary-range-name" => Some(("config.nodeConfig.ipAllocationPolicy.servicesSecondaryRangeName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.node-config.ip-allocation-policy.use-ip-aliases" => Some(("config.nodeConfig.ipAllocationPolicy.useIpAliases", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.node-config.location" => Some(("config.nodeConfig.location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.node-config.machine-type" => Some(("config.nodeConfig.machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.node-config.network" => Some(("config.nodeConfig.network", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.node-config.oauth-scopes" => Some(("config.nodeConfig.oauthScopes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.node-config.service-account" => Some(("config.nodeConfig.serviceAccount", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.node-config.subnetwork" => Some(("config.nodeConfig.subnetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.node-config.tags" => Some(("config.nodeConfig.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "config.node-count" => Some(("config.nodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.private-environment-config.cloud-composer-connection-subnetwork" => Some(("config.privateEnvironmentConfig.cloudComposerConnectionSubnetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.private-environment-config.cloud-composer-network-ipv4-cidr-block" => Some(("config.privateEnvironmentConfig.cloudComposerNetworkIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.private-environment-config.cloud-composer-network-ipv4-reserved-range" => Some(("config.privateEnvironmentConfig.cloudComposerNetworkIpv4ReservedRange", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.private-environment-config.cloud-sql-ipv4-cidr-block" => Some(("config.privateEnvironmentConfig.cloudSqlIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.private-environment-config.enable-private-builds-only" => Some(("config.privateEnvironmentConfig.enablePrivateBuildsOnly", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.private-environment-config.enable-private-environment" => Some(("config.privateEnvironmentConfig.enablePrivateEnvironment", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.private-environment-config.enable-privately-used-public-ips" => Some(("config.privateEnvironmentConfig.enablePrivatelyUsedPublicIps", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.private-environment-config.networking-config.connection-type" => Some(("config.privateEnvironmentConfig.networkingConfig.connectionType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.private-environment-config.private-cluster-config.enable-private-endpoint" => Some(("config.privateEnvironmentConfig.privateClusterConfig.enablePrivateEndpoint", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.private-environment-config.private-cluster-config.master-ipv4-cidr-block" => Some(("config.privateEnvironmentConfig.privateClusterConfig.masterIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.private-environment-config.private-cluster-config.master-ipv4-reserved-range" => Some(("config.privateEnvironmentConfig.privateClusterConfig.masterIpv4ReservedRange", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.private-environment-config.web-server-ipv4-cidr-block" => Some(("config.privateEnvironmentConfig.webServerIpv4CidrBlock", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.private-environment-config.web-server-ipv4-reserved-range" => Some(("config.privateEnvironmentConfig.webServerIpv4ReservedRange", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.recovery-config.scheduled-snapshots-config.enabled" => Some(("config.recoveryConfig.scheduledSnapshotsConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.recovery-config.scheduled-snapshots-config.snapshot-creation-schedule" => Some(("config.recoveryConfig.scheduledSnapshotsConfig.snapshotCreationSchedule", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.recovery-config.scheduled-snapshots-config.snapshot-location" => Some(("config.recoveryConfig.scheduledSnapshotsConfig.snapshotLocation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.recovery-config.scheduled-snapshots-config.time-zone" => Some(("config.recoveryConfig.scheduledSnapshotsConfig.timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.resilience-mode" => Some(("config.resilienceMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.software-config.airflow-config-overrides" => Some(("config.softwareConfig.airflowConfigOverrides", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "config.software-config.cloud-data-lineage-integration.enabled" => Some(("config.softwareConfig.cloudDataLineageIntegration.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "config.software-config.env-variables" => Some(("config.softwareConfig.envVariables", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "config.software-config.image-version" => Some(("config.softwareConfig.imageVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.software-config.pypi-packages" => Some(("config.softwareConfig.pypiPackages", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "config.software-config.python-version" => Some(("config.softwareConfig.pythonVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.software-config.scheduler-count" => Some(("config.softwareConfig.schedulerCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.software-config.web-server-plugins-mode" => Some(("config.softwareConfig.webServerPluginsMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.web-server-config.machine-type" => Some(("config.webServerConfig.machineType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "config.workloads-config.dag-processor.count" => Some(("config.workloadsConfig.dagProcessor.count", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.workloads-config.dag-processor.cpu" => Some(("config.workloadsConfig.dagProcessor.cpu", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "config.workloads-config.dag-processor.memory-gb" => Some(("config.workloadsConfig.dagProcessor.memoryGb", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "config.workloads-config.dag-processor.storage-gb" => Some(("config.workloadsConfig.dagProcessor.storageGb", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "config.workloads-config.scheduler.count" => Some(("config.workloadsConfig.scheduler.count", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.workloads-config.scheduler.cpu" => Some(("config.workloadsConfig.scheduler.cpu", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "config.workloads-config.scheduler.memory-gb" => Some(("config.workloadsConfig.scheduler.memoryGb", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "config.workloads-config.scheduler.storage-gb" => Some(("config.workloadsConfig.scheduler.storageGb", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "config.workloads-config.triggerer.count" => Some(("config.workloadsConfig.triggerer.count", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.workloads-config.triggerer.cpu" => Some(("config.workloadsConfig.triggerer.cpu", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "config.workloads-config.triggerer.memory-gb" => Some(("config.workloadsConfig.triggerer.memoryGb", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "config.workloads-config.web-server.cpu" => Some(("config.workloadsConfig.webServer.cpu", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "config.workloads-config.web-server.memory-gb" => Some(("config.workloadsConfig.webServer.memoryGb", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "config.workloads-config.web-server.storage-gb" => Some(("config.workloadsConfig.webServer.storageGb", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "config.workloads-config.worker.cpu" => Some(("config.workloadsConfig.worker.cpu", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "config.workloads-config.worker.max-count" => Some(("config.workloadsConfig.worker.maxCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.workloads-config.worker.memory-gb" => Some(("config.workloadsConfig.worker.memoryGb", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "config.workloads-config.worker.min-count" => Some(("config.workloadsConfig.worker.minCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "config.workloads-config.worker.storage-gb" => Some(("config.workloadsConfig.worker.storageGb", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "labels" => Some(("labels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "satisfies-pzs" => Some(("satisfiesPzs", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "storage-config.bucket" => Some(("storageConfig.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "update-time" => Some(("updateTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "uuid" => Some(("uuid", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["airflow-byoid-uri", "airflow-config-overrides", "airflow-metadata-retention-config", "airflow-uri", "bucket", "cloud-composer-connection-subnetwork", "cloud-composer-network-ipv4-cidr-block", "cloud-composer-network-ipv4-reserved-range", "cloud-data-lineage-integration", "cloud-sql-ipv4-cidr-block", "cluster-ipv4-cidr-block", "cluster-secondary-range-name", "composer-internal-ipv4-cidr-block", "composer-network-attachment", "config", "connection-type", "count", "cpu", "create-time", "dag-gcs-prefix", "dag-processor", "data-retention-config", "database-config", "disk-size-gb", "enable-ip-masq-agent", "enable-private-builds-only", "enable-private-endpoint", "enable-private-environment", "enable-privately-used-public-ips", "enabled", "encryption-config", "end-time", "env-variables", "environment-size", "gke-cluster", "image-version", "ip-allocation-policy", "kms-key-name", "labels", "location", "machine-type", "maintenance-window", "master-authorized-networks-config", "master-ipv4-cidr-block", "master-ipv4-reserved-range", "max-count", "memory-gb", "min-count", "name", "network", "networking-config", "node-config", "node-count", "oauth-scopes", "private-cluster-config", "private-environment-config", "pypi-packages", "python-version", "recovery-config", "recurrence", "resilience-mode", "retention-days", "retention-mode", "satisfies-pzs", "scheduled-snapshots-config", "scheduler", "scheduler-count", "service-account", "services-ipv4-cidr-block", "services-secondary-range-name", "snapshot-creation-schedule", "snapshot-location", "software-config", "start-time", "state", "storage-config", "storage-gb", "storage-mode", "subnetwork", "tags", "task-logs-retention-config", "time-zone", "triggerer", "update-time", "use-ip-aliases", "uuid", "web-server", "web-server-config", "web-server-ipv4-cidr-block", "web-server-ipv4-reserved-range", "web-server-plugins-mode", "worker", "workloads-config", "zone"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Environment = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_environments_patch(request, opt.value_of("name").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(        value.map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask")).unwrap_or(FieldMask::default()));
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
                                                                           v.extend(["update-mask"].iter().map(|v|*v));
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

    async fn _projects_locations_environments_poll_airflow_command(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "execution-id" => Some(("executionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "next-line-number" => Some(("nextLineNumber", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "pod" => Some(("pod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pod-namespace" => Some(("podNamespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["execution-id", "next-line-number", "pod", "pod-namespace"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::PollAirflowCommandRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_environments_poll_airflow_command(request, opt.value_of("environment").unwrap_or(""));
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

    async fn _projects_locations_environments_save_snapshot(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "snapshot-location" => Some(("snapshotLocation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["snapshot-location"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::SaveSnapshotRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_environments_save_snapshot(request, opt.value_of("environment").unwrap_or(""));
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

    async fn _projects_locations_environments_stop_airflow_command(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "execution-id" => Some(("executionId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "force" => Some(("force", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "pod" => Some(("pod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "pod-namespace" => Some(("podNamespace", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["execution-id", "force", "pod", "pod-namespace"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::StopAirflowCommandRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_environments_stop_airflow_command(request, opt.value_of("environment").unwrap_or(""));
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

    async fn _projects_locations_environments_user_workloads_config_maps_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "data" => Some(("data", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["data", "name"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::UserWorkloadsConfigMap = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_environments_user_workloads_config_maps_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_environments_user_workloads_config_maps_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_environments_user_workloads_config_maps_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_environments_user_workloads_config_maps_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_environments_user_workloads_config_maps_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_environments_user_workloads_config_maps_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_environments_user_workloads_config_maps_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["page-size", "page-token"].iter().map(|v|*v));
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

    async fn _projects_locations_environments_user_workloads_config_maps_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "data" => Some(("data", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["data", "name"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::UserWorkloadsConfigMap = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_environments_user_workloads_config_maps_update(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_environments_user_workloads_secrets_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "data" => Some(("data", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["data", "name"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::UserWorkloadsSecret = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_environments_user_workloads_secrets_create(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_environments_user_workloads_secrets_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_environments_user_workloads_secrets_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_environments_user_workloads_secrets_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_environments_user_workloads_secrets_get(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_environments_user_workloads_secrets_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_environments_user_workloads_secrets_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
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
                                                                           v.extend(["page-size", "page-token"].iter().map(|v|*v));
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

    async fn _projects_locations_environments_user_workloads_secrets_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
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
                    "data" => Some(("data", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["data", "name"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::UserWorkloadsSecret = json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().locations_environments_user_workloads_secrets_update(request, opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_environments_workloads_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_environments_workloads_list(opt.value_of("parent").unwrap_or(""));
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

    async fn _projects_locations_image_versions_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_image_versions_list(opt.value_of("parent").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(        value.map(|v| arg_from_str(v, err, "page-size", "int32")).unwrap_or(-0));
                },
                "include-past-releases" => {
                    call = call.include_past_releases(        value.map(|v| arg_from_str(v, err, "include-past-releases", "boolean")).unwrap_or(false));
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
                                                                           v.extend(["include-past-releases", "page-size", "page-token"].iter().map(|v|*v));
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

    async fn _projects_locations_operations_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_operations_delete(opt.value_of("name").unwrap_or(""));
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

    async fn _projects_locations_operations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.projects().locations_operations_list(opt.value_of("name").unwrap_or(""));
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

    async fn _doit(&self, dry_run: bool) -> Result<Result<(), DoitError>, Option<InvalidOptionsError>> {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Result<(), DoitError> = Ok(());
        let mut err_opt: Option<InvalidOptionsError> = None;
        match self.opt.subcommand() {
            ("projects", Some(opt)) => {
                match opt.subcommand() {
                    ("locations-environments-check-upgrade", Some(opt)) => {
                        call_result = self._projects_locations_environments_check_upgrade(opt, dry_run, &mut err).await;
                    },
                    ("locations-environments-create", Some(opt)) => {
                        call_result = self._projects_locations_environments_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-environments-database-failover", Some(opt)) => {
                        call_result = self._projects_locations_environments_database_failover(opt, dry_run, &mut err).await;
                    },
                    ("locations-environments-delete", Some(opt)) => {
                        call_result = self._projects_locations_environments_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-environments-execute-airflow-command", Some(opt)) => {
                        call_result = self._projects_locations_environments_execute_airflow_command(opt, dry_run, &mut err).await;
                    },
                    ("locations-environments-fetch-database-properties", Some(opt)) => {
                        call_result = self._projects_locations_environments_fetch_database_properties(opt, dry_run, &mut err).await;
                    },
                    ("locations-environments-get", Some(opt)) => {
                        call_result = self._projects_locations_environments_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-environments-list", Some(opt)) => {
                        call_result = self._projects_locations_environments_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-environments-load-snapshot", Some(opt)) => {
                        call_result = self._projects_locations_environments_load_snapshot(opt, dry_run, &mut err).await;
                    },
                    ("locations-environments-patch", Some(opt)) => {
                        call_result = self._projects_locations_environments_patch(opt, dry_run, &mut err).await;
                    },
                    ("locations-environments-poll-airflow-command", Some(opt)) => {
                        call_result = self._projects_locations_environments_poll_airflow_command(opt, dry_run, &mut err).await;
                    },
                    ("locations-environments-save-snapshot", Some(opt)) => {
                        call_result = self._projects_locations_environments_save_snapshot(opt, dry_run, &mut err).await;
                    },
                    ("locations-environments-stop-airflow-command", Some(opt)) => {
                        call_result = self._projects_locations_environments_stop_airflow_command(opt, dry_run, &mut err).await;
                    },
                    ("locations-environments-user-workloads-config-maps-create", Some(opt)) => {
                        call_result = self._projects_locations_environments_user_workloads_config_maps_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-environments-user-workloads-config-maps-delete", Some(opt)) => {
                        call_result = self._projects_locations_environments_user_workloads_config_maps_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-environments-user-workloads-config-maps-get", Some(opt)) => {
                        call_result = self._projects_locations_environments_user_workloads_config_maps_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-environments-user-workloads-config-maps-list", Some(opt)) => {
                        call_result = self._projects_locations_environments_user_workloads_config_maps_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-environments-user-workloads-config-maps-update", Some(opt)) => {
                        call_result = self._projects_locations_environments_user_workloads_config_maps_update(opt, dry_run, &mut err).await;
                    },
                    ("locations-environments-user-workloads-secrets-create", Some(opt)) => {
                        call_result = self._projects_locations_environments_user_workloads_secrets_create(opt, dry_run, &mut err).await;
                    },
                    ("locations-environments-user-workloads-secrets-delete", Some(opt)) => {
                        call_result = self._projects_locations_environments_user_workloads_secrets_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-environments-user-workloads-secrets-get", Some(opt)) => {
                        call_result = self._projects_locations_environments_user_workloads_secrets_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-environments-user-workloads-secrets-list", Some(opt)) => {
                        call_result = self._projects_locations_environments_user_workloads_secrets_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-environments-user-workloads-secrets-update", Some(opt)) => {
                        call_result = self._projects_locations_environments_user_workloads_secrets_update(opt, dry_run, &mut err).await;
                    },
                    ("locations-environments-workloads-list", Some(opt)) => {
                        call_result = self._projects_locations_environments_workloads_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-image-versions-list", Some(opt)) => {
                        call_result = self._projects_locations_image_versions_list(opt, dry_run, &mut err).await;
                    },
                    ("locations-operations-delete", Some(opt)) => {
                        call_result = self._projects_locations_operations_delete(opt, dry_run, &mut err).await;
                    },
                    ("locations-operations-get", Some(opt)) => {
                        call_result = self._projects_locations_operations_get(opt, dry_run, &mut err).await;
                    },
                    ("locations-operations-list", Some(opt)) => {
                        call_result = self._projects_locations_operations_list(opt, dry_run, &mut err).await;
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

            match client::application_secret_from_directory(&config_dir, "composer1-secret.json",
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
        ).persist_tokens_to_disk(format!("{}/composer1", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::CloudComposer::new(client, auth),
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
        ("projects", "methods: 'locations-environments-check-upgrade', 'locations-environments-create', 'locations-environments-database-failover', 'locations-environments-delete', 'locations-environments-execute-airflow-command', 'locations-environments-fetch-database-properties', 'locations-environments-get', 'locations-environments-list', 'locations-environments-load-snapshot', 'locations-environments-patch', 'locations-environments-poll-airflow-command', 'locations-environments-save-snapshot', 'locations-environments-stop-airflow-command', 'locations-environments-user-workloads-config-maps-create', 'locations-environments-user-workloads-config-maps-delete', 'locations-environments-user-workloads-config-maps-get', 'locations-environments-user-workloads-config-maps-list', 'locations-environments-user-workloads-config-maps-update', 'locations-environments-user-workloads-secrets-create', 'locations-environments-user-workloads-secrets-delete', 'locations-environments-user-workloads-secrets-get', 'locations-environments-user-workloads-secrets-list', 'locations-environments-user-workloads-secrets-update', 'locations-environments-workloads-list', 'locations-image-versions-list', 'locations-operations-delete', 'locations-operations-get' and 'locations-operations-list'", vec![
            ("locations-environments-check-upgrade",
                    Some(r##"Check if an upgrade operation on the environment will succeed. In case of problems detailed info can be found in the returned Operation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_composer1_cli/projects_locations-environments-check-upgrade",
                  vec![
                    (Some(r##"environment"##),
                     None,
                     Some(r##"Required. The resource name of the environment to check upgrade for, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}""##),
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
            ("locations-environments-create",
                    Some(r##"Create a new environment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_composer1_cli/projects_locations-environments-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"The parent must be of the form "projects/{projectId}/locations/{locationId}"."##),
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
            ("locations-environments-database-failover",
                    Some(r##"Triggers database failover (only for highly resilient environments)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_composer1_cli/projects_locations-environments-database-failover",
                  vec![
                    (Some(r##"environment"##),
                     None,
                     Some(r##"Target environment: "projects/{projectId}/locations/{locationId}/environments/{environmentId}""##),
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
            ("locations-environments-delete",
                    Some(r##"Delete an environment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_composer1_cli/projects_locations-environments-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The environment to delete, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}""##),
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
            ("locations-environments-execute-airflow-command",
                    Some(r##"Executes Airflow CLI command."##),
                    "Details at http://byron.github.io/google-apis-rs/google_composer1_cli/projects_locations-environments-execute-airflow-command",
                  vec![
                    (Some(r##"environment"##),
                     None,
                     Some(r##"The resource name of the environment in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}"."##),
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
            ("locations-environments-fetch-database-properties",
                    Some(r##"Fetches database properties."##),
                    "Details at http://byron.github.io/google-apis-rs/google_composer1_cli/projects_locations-environments-fetch-database-properties",
                  vec![
                    (Some(r##"environment"##),
                     None,
                     Some(r##"Required. The resource name of the environment, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}""##),
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
            ("locations-environments-get",
                    Some(r##"Get an existing environment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_composer1_cli/projects_locations-environments-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The resource name of the environment to get, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}""##),
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
            ("locations-environments-list",
                    Some(r##"List environments."##),
                    "Details at http://byron.github.io/google-apis-rs/google_composer1_cli/projects_locations-environments-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"List environments in the given project and location, in the form: "projects/{projectId}/locations/{locationId}""##),
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
            ("locations-environments-load-snapshot",
                    Some(r##"Loads a snapshot of a Cloud Composer environment. As a result of this operation, a snapshot of environment's specified in LoadSnapshotRequest is loaded into the environment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_composer1_cli/projects_locations-environments-load-snapshot",
                  vec![
                    (Some(r##"environment"##),
                     None,
                     Some(r##"The resource name of the target environment in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}""##),
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
            ("locations-environments-patch",
                    Some(r##"Update an environment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_composer1_cli/projects_locations-environments-patch",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The relative resource name of the environment to update, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}""##),
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
            ("locations-environments-poll-airflow-command",
                    Some(r##"Polls Airflow CLI command execution and fetches logs."##),
                    "Details at http://byron.github.io/google-apis-rs/google_composer1_cli/projects_locations-environments-poll-airflow-command",
                  vec![
                    (Some(r##"environment"##),
                     None,
                     Some(r##"The resource name of the environment in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}""##),
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
            ("locations-environments-save-snapshot",
                    Some(r##"Creates a snapshots of a Cloud Composer environment. As a result of this operation, snapshot of environment's state is stored in a location specified in the SaveSnapshotRequest."##),
                    "Details at http://byron.github.io/google-apis-rs/google_composer1_cli/projects_locations-environments-save-snapshot",
                  vec![
                    (Some(r##"environment"##),
                     None,
                     Some(r##"The resource name of the source environment in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}""##),
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
            ("locations-environments-stop-airflow-command",
                    Some(r##"Stops Airflow CLI command execution."##),
                    "Details at http://byron.github.io/google-apis-rs/google_composer1_cli/projects_locations-environments-stop-airflow-command",
                  vec![
                    (Some(r##"environment"##),
                     None,
                     Some(r##"The resource name of the environment in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}"."##),
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
            ("locations-environments-user-workloads-config-maps-create",
                    Some(r##"Creates a user workloads ConfigMap. This method is supported for Cloud Composer environments in versions composer-3.*.*-airflow-*.*.* and newer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_composer1_cli/projects_locations-environments-user-workloads-config-maps-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The environment name to create a ConfigMap for, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}""##),
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
            ("locations-environments-user-workloads-config-maps-delete",
                    Some(r##"Deletes a user workloads ConfigMap. This method is supported for Cloud Composer environments in versions composer-3.*.*-airflow-*.*.* and newer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_composer1_cli/projects_locations-environments-user-workloads-config-maps-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The ConfigMap to delete, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}/userWorkloadsConfigMaps/{userWorkloadsConfigMapId}""##),
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
            ("locations-environments-user-workloads-config-maps-get",
                    Some(r##"Gets an existing user workloads ConfigMap. This method is supported for Cloud Composer environments in versions composer-3.*.*-airflow-*.*.* and newer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_composer1_cli/projects_locations-environments-user-workloads-config-maps-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name of the ConfigMap to get, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}/userWorkloadsConfigMaps/{userWorkloadsConfigMapId}""##),
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
            ("locations-environments-user-workloads-config-maps-list",
                    Some(r##"Lists user workloads ConfigMaps. This method is supported for Cloud Composer environments in versions composer-3.*.*-airflow-*.*.* and newer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_composer1_cli/projects_locations-environments-user-workloads-config-maps-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. List ConfigMaps in the given environment, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}""##),
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
            ("locations-environments-user-workloads-config-maps-update",
                    Some(r##"Updates a user workloads ConfigMap. This method is supported for Cloud Composer environments in versions composer-3.*.*-airflow-*.*.* and newer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_composer1_cli/projects_locations-environments-user-workloads-config-maps-update",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Identifier. The resource name of the ConfigMap, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}/userWorkloadsConfigMaps/{userWorkloadsConfigMapId}""##),
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
            ("locations-environments-user-workloads-secrets-create",
                    Some(r##"Creates a user workloads Secret. This method is supported for Cloud Composer environments in versions composer-3.*.*-airflow-*.*.* and newer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_composer1_cli/projects_locations-environments-user-workloads-secrets-create",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The environment name to create a Secret for, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}""##),
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
            ("locations-environments-user-workloads-secrets-delete",
                    Some(r##"Deletes a user workloads Secret. This method is supported for Cloud Composer environments in versions composer-3.*.*-airflow-*.*.* and newer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_composer1_cli/projects_locations-environments-user-workloads-secrets-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The Secret to delete, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}/userWorkloadsSecrets/{userWorkloadsSecretId}""##),
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
            ("locations-environments-user-workloads-secrets-get",
                    Some(r##"Gets an existing user workloads Secret. Values of the "data" field in the response are cleared. This method is supported for Cloud Composer environments in versions composer-3.*.*-airflow-*.*.* and newer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_composer1_cli/projects_locations-environments-user-workloads-secrets-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The resource name of the Secret to get, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}/userWorkloadsSecrets/{userWorkloadsSecretId}""##),
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
            ("locations-environments-user-workloads-secrets-list",
                    Some(r##"Lists user workloads Secrets. This method is supported for Cloud Composer environments in versions composer-3.*.*-airflow-*.*.* and newer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_composer1_cli/projects_locations-environments-user-workloads-secrets-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. List Secrets in the given environment, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}""##),
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
            ("locations-environments-user-workloads-secrets-update",
                    Some(r##"Updates a user workloads Secret. This method is supported for Cloud Composer environments in versions composer-3.*.*-airflow-*.*.* and newer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_composer1_cli/projects_locations-environments-user-workloads-secrets-update",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Identifier. The resource name of the Secret, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}/userWorkloadsSecrets/{userWorkloadsSecretId}""##),
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
            ("locations-environments-workloads-list",
                    Some(r##"Lists workloads in a Cloud Composer environment. Workload is a unit that runs a single Composer component. This method is supported for Cloud Composer environments in versions composer-3.*.*-airflow-*.*.* and newer."##),
                    "Details at http://byron.github.io/google-apis-rs/google_composer1_cli/projects_locations-environments-workloads-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The environment name to get workloads for, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}""##),
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
            ("locations-image-versions-list",
                    Some(r##"List ImageVersions for provided location."##),
                    "Details at http://byron.github.io/google-apis-rs/google_composer1_cli/projects_locations-image-versions-list",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"List ImageVersions in the given project and location, in the form: "projects/{projectId}/locations/{locationId}""##),
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
            ("locations-operations-delete",
                    Some(r##"Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_composer1_cli/projects_locations-operations-delete",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the operation resource to be deleted."##),
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
            ("locations-operations-get",
                    Some(r##"Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_composer1_cli/projects_locations-operations-get",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the operation resource."##),
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
                    Some(r##"Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`."##),
                    "Details at http://byron.github.io/google-apis-rs/google_composer1_cli/projects_locations-operations-list",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"The name of the operation's parent resource."##),
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
    
    let mut app = App::new("composer1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.5+20240618")
           .about("Manages Apache Airflow environments on Google Cloud Platform.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_composer1_cli")
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
